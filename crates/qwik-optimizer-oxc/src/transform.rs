//! Main QwikTransform traverse implementation.
//!
//! The core of the optimizer. Implements the `Traverse` trait to walk the AST
//! and apply all Qwik transformations: replace `$()` calls with `qrl()` wrappers,
//! record segments for extraction, rewrite imports, and handle special patterns
//! (component$, useTask$, etc.).

use std::collections::HashSet;

use oxc::ast::ast::*;
use oxc::span::SPAN;
use oxc_traverse::{Traverse, TraverseCtx};

use crate::collector;
use crate::entry_strategy;
use crate::hash;
use crate::import_rewrite;
use crate::jsx_transform::{
    get_jsx_lambda_span, transform_attr_name_for_display, transform_jsx_element_inner,
    transform_jsx_fragment_inner,
};
use crate::props_destructuring::{self, PropsDestructuringInfo};
use crate::types::{CollectResult, Diagnostic, SegmentData, TransformOptions};
use crate::words;

/// Identifies the kind of $-call detected.
#[derive(Debug, Clone)]
pub(crate) enum DollarCallKind {
    /// Raw $() call: $(() => { ... })
    RawDollar,
    /// Named $-suffixed call: component$(() => { ... })
    Named(String),
}

/// Tracks which imports are needed for the transformed module.
#[derive(Debug, Default)]
pub(crate) struct ImportTracker {
    /// Qrl-suffixed imports needed: "componentQrl", "useStylesQrl", etc.
    pub qrl_imports: Vec<String>,

    /// Whether the module needs `import { qrl }` (segment strategy).
    pub needs_qrl: bool,

    /// Whether the module needs `import { inlinedQrl }` (inline strategy).
    pub needs_inlined_qrl: bool,

    /// Whether the module needs `import { _captures }` (inline + captures).
    pub needs_captures: bool,

    /// Whether the module needs `import { _restProps }` (props destructuring with rest).
    pub needs_rest_props: bool,

    /// Lazy import constants to insert (segment strategy).
    /// Each entry: (hash, import_path).
    pub lazy_imports: Vec<(String, String)>,

    /// Whether the module needs `import { _jsxSorted }` from core.
    pub needs_jsx_sorted: bool,

    /// Whether the module needs `import { _jsxSplit }` from core.
    pub needs_jsx_split: bool,

    /// Whether the module needs `import { _getVarProps }` from core.
    pub needs_get_var_props: bool,

    /// Whether the module needs `import { _getConstProps }` from core.
    pub needs_get_const_props: bool,

    /// Whether the module needs `import { Fragment as _Fragment }` from jsx-runtime.
    pub needs_fragment: bool,

    /// Whether the module needs `import { _wrapProp }` from core.
    pub needs_wrap_prop: bool,

    /// Whether the module needs `import { _fnSignal }` from core.
    pub needs_fn_signal: bool,

    /// Whether the module needs `import { _val }` from core (bind:value).
    pub needs_val: bool,

    /// Whether the module needs `import { _chk }` from core (bind:checked).
    pub needs_chk: bool,

    /// Whether the module needs `import { _noopQrl }` from core (stripped ctx calls).
    pub needs_noop_qrl: bool,

    /// Whether the module needs `import { _qrlSync }` from core (sync$ calls).
    pub needs_qrl_sync: bool,

    /// Monotonic counter for generating unique JSX key suffixes like "u6_0", "u6_1".
    pub jsx_key_counter: u32,

    /// Monotonic counter for hoisted function names (_hf0, _hf1, ...).
    pub hoisted_fn_counter: u32,
}

/// The core Qwik transform traversal state.
pub(crate) struct QwikTransform {
    options: TransformOptions,
    collected: CollectResult,
    filename: String,
    segments: Vec<SegmentData>,
    diagnostics: Vec<Diagnostic>,
    import_tracker: ImportTracker,
    segment_counter: u32,
    dollar_call_stack: Vec<String>,
    /// Set of span starts for dollar calls that we've already recorded,
    /// so exit_expression can identify them.
    pending_dollar_calls: HashSet<u32>,

    /// Active props destructuring info for the current component$ call.
    /// Set in enter_call_expression, consumed in exit_expression.
    active_props_info: Option<PropsDestructuringInfo>,

    /// Stack of capture tracking state for nested $()-bodies.
    /// Each entry is (body_ident_refs, body_local_decls) for one $()-body.
    /// Pushed on entering a $()-call, popped on exiting.
    capture_stack: Vec<(Vec<String>, HashSet<String>)>,

    /// Serialized body code for each segment (segment strategy only).
    /// Keyed by call span.start for matching to SegmentData.
    segment_body_codes: Vec<(u32, String)>,

    /// Hoisted function declarations to insert at module top level.
    /// Each entry is (fn_declaration_code, str_declaration_code).
    /// e.g., ("const _hf0 = (p0)=>p0.value;", "const _hf0_str = \"p0.value\";")
    hoisted_function_stmts: Vec<(String, String)>,

    /// Set of span starts for segments that are stripped (matching strip_ctx_name).
    stripped_segments: HashSet<u32>,

    /// Set of span starts for sync$() calls.
    pending_sync_calls: HashSet<u32>,

    /// Pending Qrl-suffixed imports for parent segments (nested $-calls).
    /// Each entry: (parent_display_name, qrl_name).
    pending_segment_qrl_imports: Vec<(String, String)>,

    /// Whether this module has a custom JSX import source (e.g., `@jsxImportSource react`).
    /// When true, JSX event handler `$`-attributes are NOT extracted as segments
    /// because the JSX is not Qwik JSX.
    has_custom_jsx_import_source: bool,
}

impl QwikTransform {
    /// Create a new QwikTransform instance.
    pub fn new(options: &TransformOptions, collected: CollectResult, filename: &str) -> Self {
        Self {
            options: options.clone(),
            collected,
            filename: filename.to_string(),
            segments: Vec::new(),
            diagnostics: Vec::new(),
            import_tracker: ImportTracker::default(),
            segment_counter: 0,
            dollar_call_stack: Vec::new(),
            pending_dollar_calls: HashSet::new(),
            active_props_info: None,
            capture_stack: Vec::new(),
            segment_body_codes: Vec::new(),
            hoisted_function_stmts: Vec::new(),
            stripped_segments: HashSet::new(),
            pending_sync_calls: HashSet::new(),
            pending_segment_qrl_imports: Vec::new(),
            has_custom_jsx_import_source: false,
        }
    }

    /// Get the segments extracted during traversal.
    pub fn extracted_segments(&self) -> &[SegmentData] {
        &self.segments
    }

    /// Get any diagnostics generated during traversal.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Take the serialized body codes for segment strategy.
    /// Each entry is (span_start, body_code_string).
    pub fn take_segment_body_codes(&mut self) -> Vec<(u32, String)> {
        std::mem::take(&mut self.segment_body_codes)
    }

    /// Get the hoisted function declarations for _fnSignal.
    /// Each entry is (fn_declaration_code, str_declaration_code).
    /// E.g., ("const _hf0 = (p0)=>p0.value;", "const _hf0_str = \"p0.value\";")
    pub fn hoisted_function_stmts(&self) -> &[(String, String)] {
        &self.hoisted_function_stmts
    }

    /// Get the set of span starts for stripped segments.
    pub fn stripped_segments(&self) -> &HashSet<u32> {
        &self.stripped_segments
    }

    /// Mark this module as having a custom JSX import source (e.g., `@jsxImportSource react`).
    /// When set, JSX event handler `$`-attributes are NOT extracted as segments.
    pub fn set_custom_jsx_import_source(&mut self, has: bool) {
        self.has_custom_jsx_import_source = has;
    }

    /// Check if a ctx name should be stripped based on strip_ctx_name config.
    fn should_strip_ctx_name(&self, ctx_name: &str) -> bool {
        self.options.strip_ctx_name.iter().any(|stripped| {
            let name_without_dollar = ctx_name.trim_end_matches('$');
            name_without_dollar
                .to_lowercase()
                .contains(&stripped.to_lowercase())
        })
    }

    /// Post-transform processing: populate child segment metadata.
    /// Must be called after traverse_mut completes.
    pub fn finalize_segments(&mut self) {
        let child_info: Vec<(String, String, String)> = self
            .segments
            .iter()
            .filter_map(|seg| {
                if self.stripped_segments.contains(&seg.span.0) {
                    return None;
                }
                seg.parent.as_ref().map(|parent_name| {
                    let canonical = format!("{}_{}", seg.display_name, seg.hash);
                    let import_path = if self.options.explicit_extensions {
                        let ext = self.compute_output_extension();
                        format!("./{}.{}", canonical, ext)
                    } else {
                        format!("./{}", canonical)
                    };
                    (parent_name.clone(), seg.hash.clone(), import_path)
                })
            })
            .collect();

        // Transfer pending Qrl-suffixed imports to parent segments
        let pending_qrl_imports = std::mem::take(&mut self.pending_segment_qrl_imports);

        for seg in self.segments.iter_mut() {
            let children: Vec<_> = child_info
                .iter()
                .filter(|(parent, _, _)| parent == &seg.display_name)
                .collect();

            if !children.is_empty() {
                seg.needs_qrl_import = true;
                for (_, hash, path) in children {
                    seg.child_lazy_imports.push((hash.clone(), path.clone()));
                }
            }

            // Assign Qrl-suffixed imports from nested $-calls to their parent segment
            for (parent_name, qrl_name) in &pending_qrl_imports {
                if parent_name == &seg.display_name && !seg.segment_qrl_names.contains(qrl_name) {
                    seg.segment_qrl_names.push(qrl_name.clone());
                }
            }
        }
    }

    /// Check if a CallExpression is a Qwik $-call.
    ///
    /// Resolves aliases: if the callee is an aliased import (e.g., `Component`
    /// for `component$`), returns the DollarCallKind using the ORIGINAL
    /// imported name, not the alias.
    fn is_dollar_call(&self, call: &CallExpression<'_>) -> Option<DollarCallKind> {
        match &call.callee {
            Expression::Identifier(ident) => {
                let name = ident.name.as_str();
                if self.collected.dollar_imports.contains(name) {
                    let original_name = self
                        .collected
                        .alias_map
                        .get(name)
                        .map(|s| s.as_str())
                        .unwrap_or(name);

                    if original_name == "$" {
                        Some(DollarCallKind::RawDollar)
                    } else {
                        Some(DollarCallKind::Named(original_name.to_string()))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Derive the display name for a dollar call from collector data or fallback.
    fn derive_display_name_for_call(&self, call: &CallExpression<'_>) -> String {
        let call_start = call.span.start;
        let call_end = call.span.end;
        for site in &self.collected.dollar_calls {
            if site.span.0 == call_start && site.span.1 == call_end {
                return site.display_name.clone();
            }
        }
        format!("s_{}", self.segment_counter)
    }

    /// Build the canonical filename for a segment.
    fn build_canonical_filename(&self, display_name: &str, hash: &str) -> String {
        format!("{}_{display_name}_{hash}", self.filename)
    }

    /// Compute the output file extension based on transpile options.
    ///
    /// Logic: strip what you transpile.
    /// - transpile_ts removes TypeScript: tsx->jsx, ts->js
    /// - transpile_jsx removes JSX: tsx->ts, jsx->js
    /// - both: tsx->js, ts->js
    fn compute_output_extension(&self) -> &str {
        let ext = self.filename.rsplit('.').next().unwrap_or("js");
        match (self.options.transpile_ts, self.options.transpile_jsx, ext) {
            (true, true, "tsx") => "js",
            (true, true, "ts") => "js",
            (true, false, "tsx") => "jsx",
            (true, false, "ts") => "js",
            (false, true, "tsx") => "ts",
            (false, true, "jsx") => "js",
            _ => ext,
        }
    }

    /// Build the segment import path for segment strategy.
    fn build_segment_import_path(&self, canonical_filename: &str) -> String {
        if self.options.explicit_extensions {
            let ext = self.compute_output_extension();
            format!("./{canonical_filename}.{ext}")
        } else {
            format!("./{canonical_filename}")
        }
    }

    /// Detect file extension from filename.
    fn file_extension(&self) -> String {
        self.filename.rsplit('.').next().unwrap_or("js").to_string()
    }

    /// Collect all binding names from a BindingPattern into the current
    /// capture stack frame's body_local_decls.
    /// Handles BindingIdentifier, ObjectPattern, and ArrayPattern recursively.
    fn collect_binding_pattern_names(&mut self, pattern: &BindingPattern<'_>) {
        match pattern {
            BindingPattern::BindingIdentifier(ident) => {
                if let Some(frame) = self.capture_stack.last_mut() {
                    frame.1.insert(ident.name.as_str().to_string());
                }
            }
            BindingPattern::ObjectPattern(obj) => {
                for prop in &obj.properties {
                    self.collect_binding_pattern_names(&prop.value);
                }
                if let Some(rest) = &obj.rest {
                    self.collect_binding_pattern_names(&rest.argument);
                }
            }
            BindingPattern::ArrayPattern(arr) => {
                for elem in arr.elements.iter().flatten() {
                    self.collect_binding_pattern_names(elem);
                }
                if let Some(rest) = &arr.rest {
                    self.collect_binding_pattern_names(&rest.argument);
                }
            }
            _ => {}
        }
    }

    /// Record a segment and track imports. Returns the segment data.
    fn record_segment(&mut self, call: &CallExpression<'_>, kind: &DollarCallKind) -> SegmentData {
        let display_name = self.derive_display_name_for_call(call);

        let full_display_name = format!("{}_{}", self.filename, display_name);

        let segment_hash = hash::compute_segment_hash(
            self.options.scope.as_deref(),
            &self.filename,
            &full_display_name,
        );

        let segment_name = hash::format_segment_name(&display_name, &segment_hash);
        let canonical_filename = self.build_canonical_filename(&display_name, &segment_hash);
        let import_path = self.build_segment_import_path(&canonical_filename);

        let ctx_name = match kind {
            DollarCallKind::RawDollar => "$".to_string(),
            DollarCallKind::Named(name) => name.clone(),
        };
        let ctx_kind = words::classify_ctx_kind(&ctx_name);

        let parent = self.dollar_call_stack.last().cloned();

        let segment = SegmentData {
            display_name: full_display_name.clone(),
            hash: segment_hash.clone(),
            name: segment_name,
            ctx_name: ctx_name.clone(),
            ctx_kind,
            origin: self.filename.clone(),
            extension: self.file_extension(),
            span: (call.span.start, call.span.end),
            parent,
            captures: false,        // Computed in exit_expression
            capture_names: vec![],  // Computed in exit_expression
            needed_imports: vec![], // Populated by finalize_segments
            segment_qrl_names: vec![],
            body_span: (call.span.start, call.span.end),
            param_names: vec![],        // Set by props destructuring if needed
            body_code: String::new(),   // Populated in exit_expression for segment strategy
            child_lazy_imports: vec![], // Populated by finalize_segments
            needs_qrl_import: false,    // Populated by finalize_segments
        };

        let will_be_stripped = match kind {
            DollarCallKind::Named(name) => self.should_strip_ctx_name(name),
            _ => false,
        };

        if !will_be_stripped {
            let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
                || matches!(
                    self.options.entry_strategy,
                    crate::types::EntryStrategy::Hoist
                );

            if is_inline {
                self.import_tracker.needs_inlined_qrl = true;
            } else {
                self.import_tracker.needs_qrl = true;
                self.import_tracker
                    .lazy_imports
                    .push((segment_hash.clone(), import_path));
            }
        }

        if let DollarCallKind::Named(name) = kind {
            let qrl_name = words::dollar_to_qrl_name(name);
            if self.dollar_call_stack.is_empty() {
                // Top-level $-call: Qrl import goes to main module
                if !self.import_tracker.qrl_imports.contains(&qrl_name) {
                    self.import_tracker.qrl_imports.push(qrl_name);
                }
            } else {
                // Nested $-call: Qrl import goes to parent segment, not main module
                let parent_display_name = self.dollar_call_stack.last().unwrap().clone();
                self.pending_segment_qrl_imports
                    .push((parent_display_name, qrl_name));
            }
        }

        self.segment_counter += 1;
        self.segments.push(segment.clone());
        segment
    }

    /// Record a segment for a JSX event handler attribute (e.g., onClick$).
    ///
    /// Unlike `record_segment`, this doesn't require a CallExpression -- it takes
    /// the display name, span, and ctx_name directly from JSX attribute info.
    pub(crate) fn record_jsx_event_segment(
        &mut self,
        display_name: &str,
        span: (u32, u32),
        ctx_name: &str,
    ) -> SegmentData {
        let full_display_name = format!("{}_{}", self.filename, display_name);

        let segment_hash = hash::compute_segment_hash(
            self.options.scope.as_deref(),
            &self.filename,
            &full_display_name,
        );

        let segment_name = hash::format_segment_name(display_name, &segment_hash);
        let canonical_filename = self.build_canonical_filename(display_name, &segment_hash);
        let import_path = self.build_segment_import_path(&canonical_filename);

        // attribute name pattern. This includes onClick$, onInput$, custom$, etc.
        let ctx_kind = crate::types::CtxKind::EventHandler;
        let parent = self.dollar_call_stack.last().cloned();

        let segment = SegmentData {
            display_name: full_display_name.clone(),
            hash: segment_hash.clone(),
            name: segment_name,
            ctx_name: ctx_name.to_string(),
            ctx_kind,
            origin: self.filename.clone(),
            extension: self.file_extension(),
            span,
            parent,
            captures: false,
            capture_names: vec![],
            needed_imports: vec![],
            segment_qrl_names: vec![],
            body_span: span,
            param_names: vec![],
            body_code: String::new(),
            child_lazy_imports: vec![],
            needs_qrl_import: false,
        };

        let will_be_stripped = self.should_strip_ctx_name(ctx_name);

        if !will_be_stripped {
            let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
                || matches!(
                    self.options.entry_strategy,
                    crate::types::EntryStrategy::Hoist
                );

            if is_inline {
                self.import_tracker.needs_inlined_qrl = true;
            } else {
                self.import_tracker.needs_qrl = true;
                self.import_tracker
                    .lazy_imports
                    .push((segment_hash.clone(), import_path));
            }
        }

        self.segment_counter += 1;
        self.segments.push(segment.clone());
        segment
    }

    /// Pre-scan a JSXElement (recursively) for $-suffixed attributes and create
    /// segments for each. This ensures segment modules are produced for JSX event
    /// handlers like onClick$, onInput$, render$, etc.
    ///
    /// When `strip_event_handlers` is true, no JSX event segments are created.
    /// When the module has a custom `@jsxImportSource`, JSX events are also skipped
    /// since they are not Qwik JSX attributes.
    fn create_jsx_event_segments_recursive(&mut self, element: &JSXElement<'_>) {
        // When strip_event_handlers is enabled, skip all JSX event handler extraction
        if self.options.strip_event_handlers {
            return;
        }
        // When a custom JSX import source is set (e.g., React), JSX $-attributes
        // are not Qwik event handlers -- do not extract them.
        if self.has_custom_jsx_import_source {
            return;
        }
        let element_name = match &element.opening_element.name {
            JSXElementName::Identifier(ident) => ident.name.as_str().to_string(),
            JSXElementName::NamespacedName(ns) => {
                format!("{}_{}", ns.namespace.name, ns.name.name)
            }
            _ => "_".to_string(),
        };

        // Scan attributes for $-suffixed names (including namespaced like document:onFocus$)
        for attr_item in &element.opening_element.attributes {
            if let JSXAttributeItem::Attribute(attr) = attr_item {
                let (attr_name_str, namespace_prefix) = match &attr.name {
                    JSXAttributeName::Identifier(ident) => (ident.name.as_str().to_string(), None),
                    JSXAttributeName::NamespacedName(ns) => {
                        let name = ns.name.name.as_str();
                        let prefix = ns.namespace.name.as_str();
                        (name.to_string(), Some(prefix.to_string()))
                    }
                };

                if attr_name_str.ends_with('$') {
                    if let Some(value) = &attr.value {
                        if let JSXAttributeValue::ExpressionContainer(container) = value {
                            let expr_span = get_jsx_lambda_span(&container.expression);
                            if let Some(span) = expr_span {
                                // the event suffix (e.g., document:onFocus$ -> q_d_focus).
                                let event_suffix = if let Some(ref prefix) = namespace_prefix {
                                    let base = transform_attr_name_for_display(&attr_name_str);
                                    let prefix_code = match prefix.as_str() {
                                        "document" => "q_d",
                                        "window" => "q_w",
                                        _ => "q_e",
                                    };
                                    format!("{}_{}", prefix_code, base.trim_start_matches("q_e_"))
                                } else {
                                    transform_attr_name_for_display(&attr_name_str)
                                };
                                let display_name = self
                                    .derive_jsx_event_display_name(&element_name, &event_suffix);
                                let ctx_name = if namespace_prefix.is_some() {
                                    format!(
                                        "{}:{}",
                                        namespace_prefix.as_ref().unwrap(),
                                        attr_name_str
                                    )
                                } else {
                                    attr_name_str.clone()
                                };
                                self.record_jsx_event_segment(&display_name, span, &ctx_name);
                            }
                        }
                    }
                }
            }
        }

        self.create_jsx_event_segments_in_children(&element.children);
    }

    /// Scan JSX children for elements with $-suffixed attributes.
    fn create_jsx_event_segments_in_children<'b>(
        &mut self,
        children: &oxc::allocator::Vec<'b, JSXChild<'b>>,
    ) {
        for child in children {
            match child {
                JSXChild::Element(el) => {
                    self.create_jsx_event_segments_recursive(el);
                }
                JSXChild::Fragment(frag) => {
                    self.create_jsx_event_segments_in_children(&frag.children);
                }
                _ => {}
            }
        }
    }

    /// Build display name for a JSX event handler segment.
    fn derive_jsx_event_display_name(&self, element_name: &str, event_suffix: &str) -> String {
        let parent_ctx = self
            .dollar_call_stack
            .last()
            .map(|s| s.as_str())
            .unwrap_or("");

        if parent_ctx.is_empty() {
            format!("{}_{}", element_name, event_suffix)
        } else {
            // Strip filename prefix from parent context if present
            let parent = parent_ctx
                .strip_prefix(&format!("{}_", self.filename))
                .unwrap_or(parent_ctx);
            format!("{}_{}_{}", parent, element_name, event_suffix)
        }
    }
}

impl<'a> Traverse<'a, ()> for QwikTransform {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let Some(kind) = self.is_dollar_call(call) else {
            return;
        };

        if call.arguments.is_empty() {
            if let DollarCallKind::Named(ref name) = kind {
                let qrl_name = crate::words::dollar_to_qrl_name(name);
                if self.dollar_call_stack.is_empty() {
                    // Top-level: Qrl import goes to main module
                    if !self.import_tracker.qrl_imports.contains(&qrl_name) {
                        self.import_tracker.qrl_imports.push(qrl_name);
                    }
                } else {
                    // Nested: Qrl import goes to parent segment
                    let parent_display_name = self.dollar_call_stack.last().unwrap().clone();
                    self.pending_segment_qrl_imports
                        .push((parent_display_name, qrl_name));
                }
            }
            return;
        }

        if let DollarCallKind::Named(ref name) = kind {
            if name == "sync$" {
                self.pending_sync_calls.insert(call.span.start);
                self.capture_stack.push((Vec::new(), HashSet::new()));
                return;
            }
            if name == "component$" {
                if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first() {
                    let info = props_destructuring::analyze_props_destructuring(&arrow.params);
                    if info.needs_transform {
                        if info.rest_name.is_some() {
                            self.import_tracker.needs_rest_props = true;
                        }
                        self.active_props_info = Some(info);
                    }
                }
            }
        }

        self.capture_stack.push((Vec::new(), HashSet::new()));

        if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first() {
            for param in &arrow.params.items {
                self.collect_binding_pattern_names(&param.pattern);
            }
            if let Some(rest) = &arrow.params.rest {
                self.collect_binding_pattern_names(&rest.rest.argument);
            }
        }

        let segment = self.record_segment(call, &kind);

        if let DollarCallKind::Named(ref name) = kind {
            if self.should_strip_ctx_name(name) {
                self.stripped_segments.insert(call.span.start);
            }
        }

        self.pending_dollar_calls.insert(call.span.start);
        self.dollar_call_stack.push(segment.display_name.clone());
    }

    fn enter_identifier_reference(
        &mut self,
        ident: &mut IdentifierReference<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // If we're inside a $()-body, collect the identifier name for capture analysis.
        if let Some(frame) = self.capture_stack.last_mut() {
            frame.0.push(ident.name.as_str().to_string());
        }
    }

    fn enter_variable_declarator(
        &mut self,
        declarator: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // If we're inside a $()-body, record variable declarations as body-local.
        if !self.capture_stack.is_empty() {
            self.collect_binding_pattern_names(&declarator.id);
        }
    }

    fn exit_expression(&mut self, expr: &mut Expression<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        // Pre-scan JSX elements for $-suffixed event handler attributes.
        match expr {
            Expression::JSXElement(_) => {
                if let Expression::JSXElement(el) = &*expr {
                    self.create_jsx_event_segments_recursive(el);
                }
            }
            Expression::JSXFragment(_) => {
                if let Expression::JSXFragment(frag) = &*expr {
                    self.create_jsx_event_segments_in_children(&frag.children);
                }
            }
            _ => {}
        }

        if self.options.transpile_jsx {
            let destr_props: Option<Vec<(String, String)>> =
                self.active_props_info.as_ref().map(|info| {
                    info.prop_keys
                        .iter()
                        .map(|(key, local)| (local.clone(), key.clone()))
                        .collect()
                });
            let destr_props_ref = destr_props.as_deref();

            // Take hoisted_function_stmts out to avoid borrow conflict with &mut self
            let mut hoisted_stmts = std::mem::take(&mut self.hoisted_function_stmts);
            let module_imports = &self.collected.module_imports;

            match expr {
                Expression::JSXElement(_) => {
                    let placeholder = ctx.ast.expression_null_literal(SPAN);
                    let old_expr = std::mem::replace(expr, placeholder);
                    if let Expression::JSXElement(el) = old_expr {
                        let result = transform_jsx_element_inner(
                            el.unbox(),
                            &mut self.import_tracker,
                            ctx,
                            destr_props_ref,
                            module_imports,
                            &mut hoisted_stmts,
                        );
                        *expr = result;
                    }
                    self.hoisted_function_stmts = hoisted_stmts;
                    return;
                }
                Expression::JSXFragment(_) => {
                    let placeholder = ctx.ast.expression_null_literal(SPAN);
                    let old_expr = std::mem::replace(expr, placeholder);
                    if let Expression::JSXFragment(frag) = old_expr {
                        let result = transform_jsx_fragment_inner(
                            frag.unbox(),
                            &mut self.import_tracker,
                            ctx,
                            destr_props_ref,
                            module_imports,
                            &mut hoisted_stmts,
                        );
                        *expr = result;
                    }
                    self.hoisted_function_stmts = hoisted_stmts;
                    return;
                }
                _ => {}
            }
            self.hoisted_function_stmts = hoisted_stmts;
        }

        if let Expression::CallExpression(call) = expr {
            // Handle sync$() calls -- replace with _qrlSync(fn, "stringified_fn")
            if self.pending_sync_calls.remove(&call.span.start) {
                self.capture_stack.pop();

                let body_expr = if !call.arguments.is_empty() {
                    let placeholder =
                        Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"));
                    let body_arg = std::mem::replace(&mut call.arguments[0], placeholder);
                    Some(argument_to_expression(body_arg, ctx))
                } else {
                    None
                };

                if let Some(fn_expr) = body_expr {
                    let mut codegen = oxc::codegen::Codegen::new();
                    codegen.print_expression(&fn_expr);
                    let fn_string = codegen.into_source_text();
                    let minified = minify_fn_string(&fn_string);

                    let replacement = import_rewrite::build_qrl_sync_call(fn_expr, &minified, ctx);

                    // the top level (not inside a $-body that will be extracted).
                    let is_segment_strategy =
                        !entry_strategy::should_inline(&self.options.entry_strategy)
                            && !matches!(
                                self.options.entry_strategy,
                                crate::types::EntryStrategy::Hoist
                            );
                    let inside_dollar_body = !self.capture_stack.is_empty();
                    if !(is_segment_strategy && inside_dollar_body) {
                        self.import_tracker.needs_qrl_sync = true;
                    }

                    *expr = replacement;
                }
                return;
            }

            if !self.pending_dollar_calls.remove(&call.span.start) {
                return;
            }

            let kind = match self.is_dollar_call(call) {
                Some(k) => k,
                None => return,
            };

            self.dollar_call_stack.pop();

            let is_component_exit =
                matches!(&kind, DollarCallKind::Named(name) if name == "component$");
            let props_info = if is_component_exit {
                self.active_props_info.take()
            } else {
                None
            };
            // props_info is only Some when is_component_exit is true, so the
            // inner kind/name checks are unnecessary -- flatten to one level.
            if let Some(ref info) = props_info {
                if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first_mut() {
                    if !arrow.params.items.is_empty() {
                        let new_pattern = ctx.ast.binding_pattern_binding_identifier(
                            SPAN,
                            ctx.ast.atom(&info.raw_props_name),
                        );
                        let new_param = ctx.ast.formal_parameter(
                            SPAN,
                            ctx.ast.vec(),
                            new_pattern,
                            None::<oxc::allocator::Box<'a, TSTypeAnnotation<'a>>>,
                            None::<oxc::allocator::Box<'a, Expression<'a>>>,
                            false,
                            None,
                            false,
                            false,
                        );
                        arrow.params.items[0] = new_param;
                        arrow.params.rest = None;
                    }

                    if let Some(ref rest_name) = info.rest_name {
                        let excluded_keys: Vec<String> =
                            info.prop_keys.iter().map(|(key, _)| key.clone()).collect();
                        let rest_stmt = props_destructuring::build_rest_props_declaration(
                            rest_name,
                            &info.raw_props_name,
                            &excluded_keys,
                            ctx,
                        );

                        let mut old_stmts = ctx.ast.vec();
                        std::mem::swap(&mut arrow.body.statements, &mut old_stmts);
                        let mut new_stmts = ctx.ast.vec_with_capacity(1 + old_stmts.len());
                        new_stmts.push(rest_stmt);
                        for s in old_stmts {
                            new_stmts.push(s);
                        }
                        arrow.body.statements = new_stmts;
                    }

                    let prop_map: Vec<(String, String)> = info
                        .prop_keys
                        .iter()
                        .map(|(key, local)| (local.clone(), key.clone()))
                        .collect();

                    if !prop_map.is_empty() {
                        props_destructuring::rewrite_body_statements(
                            &mut arrow.body.statements,
                            &prop_map,
                            &info.raw_props_name,
                            ctx,
                        );
                    }
                }

                if let Some(seg) = self
                    .segments
                    .iter_mut()
                    .find(|s| s.span.0 == call.span.start && s.span.1 == call.span.end)
                {
                    seg.param_names = vec![info.raw_props_name.clone()];
                }

                let local_aliases: HashSet<String> = info
                    .prop_keys
                    .iter()
                    .map(|(_, local)| local.clone())
                    .collect();

                let component_span = (call.span.start, call.span.end);
                let component_display_name = self
                    .segments
                    .iter()
                    .find(|s| s.span == component_span)
                    .map(|s| s.display_name.clone());

                if let Some(parent_name) = component_display_name {
                    for seg in self.segments.iter_mut() {
                        if seg.parent.as_ref() != Some(&parent_name) || seg.capture_names.is_empty()
                        {
                            continue;
                        }
                        let mut needs_rawprops = false;
                        seg.capture_names.retain(|name| {
                            if local_aliases.contains(name) {
                                needs_rawprops = true;
                                false
                            } else {
                                true
                            }
                        });
                        if needs_rawprops && !seg.capture_names.contains(&info.raw_props_name) {
                            seg.capture_names.insert(0, info.raw_props_name.clone());
                        }
                        seg.captures = !seg.capture_names.is_empty();
                    }
                }
            }

            let (body_ident_refs, body_local_decls) = self.capture_stack.pop().unwrap_or_default();

            // enclosing function scope.
            let is_top_level_dollar_call = self.capture_stack.is_empty();

            let capture_result =
                collector::compute_captures(&body_ident_refs, &body_local_decls, &self.collected);

            if let Some(seg) = self
                .segments
                .iter_mut()
                .find(|s| s.span.0 == call.span.start && s.span.1 == call.span.end)
            {
                if is_top_level_dollar_call {
                    // Top-level $()-calls never have captures
                    seg.captures = false;
                    seg.capture_names = vec![];
                } else {
                    seg.captures = !capture_result.capture_names.is_empty();
                    seg.capture_names = capture_result.capture_names.clone();
                }
            }

            let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
                || matches!(
                    self.options.entry_strategy,
                    crate::types::EntryStrategy::Hoist
                );
            if !is_top_level_dollar_call && !capture_result.capture_names.is_empty() && is_inline {
                self.import_tracker.needs_captures = true;
            }

            let segment_info = self
                .segments
                .iter()
                .find(|s| s.span.0 == call.span.start && s.span.1 == call.span.end)
                .cloned();

            let segment_info = match segment_info {
                Some(s) => s,
                None => return,
            };

            let is_stripped = self.stripped_segments.contains(&call.span.start);

            if !is_stripped {
                if !is_inline && !call.arguments.is_empty() {
                    let placeholder =
                        Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"));
                    let body_arg = std::mem::replace(&mut call.arguments[0], placeholder);

                    let body_expr = match body_arg {
                        Argument::SpreadElement(_) => None,
                        _ => Some(argument_to_expression(body_arg, ctx)),
                    };

                    if let Some(ref expr_val) = body_expr {
                        let mut codegen = oxc::codegen::Codegen::new();
                        codegen.print_expression(expr_val);
                        let body_code = codegen.into_source_text();
                        self.segment_body_codes.push((call.span.start, body_code));
                    }
                }
            }

            let replacement = if is_stripped {
                self.import_tracker.needs_noop_qrl = true;
                import_rewrite::build_noop_qrl_call(
                    &segment_info.name,
                    &capture_result.capture_names,
                    ctx,
                )
            } else if is_inline {
                let body_expr = if !call.arguments.is_empty() {
                    let arg = &mut call.arguments[0];
                    std::mem::replace(
                        arg,
                        Argument::from(ctx.ast.expression_identifier(SPAN, "undefined")),
                    )
                } else {
                    Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"))
                };

                let body_as_expr = match body_expr {
                    Argument::SpreadElement(_) => ctx.ast.expression_identifier(SPAN, "undefined"),
                    _ => argument_to_expression(body_expr, ctx),
                };

                import_rewrite::build_inlined_qrl_call(
                    body_as_expr,
                    &segment_info.name,
                    &segment_info.capture_names,
                    ctx,
                )
            } else {
                let import_ident = format!("i_{}", segment_info.hash);
                import_rewrite::build_qrl_call(
                    &import_ident,
                    &segment_info.name,
                    &segment_info.capture_names,
                    ctx,
                )
            };

            // For named $-suffixed calls, wrap with the Qrl-suffixed version
            let final_expr = match &kind {
                DollarCallKind::RawDollar => replacement,
                DollarCallKind::Named(name) => {
                    let qrl_name = words::dollar_to_qrl_name(name);
                    let qrl_atom = ctx.ast.atom(&qrl_name);
                    let qrl_callee = ctx.ast.expression_identifier(SPAN, qrl_atom);
                    let mut args = ctx.ast.vec_with_capacity(1);
                    args.push(Argument::from(replacement));
                    ctx.ast.expression_call_with_pure(
                        SPAN,
                        qrl_callee,
                        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
                        args,
                        false,
                        true,
                    )
                }
            };

            *expr = final_expr;
        }
    }

    fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        let core_module = &self.options.core_module;

        let mut new_stmts: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();

        for qrl_name in &self.import_tracker.qrl_imports {
            let stmt = import_rewrite::build_named_import(qrl_name, core_module, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_qrl {
            let stmt = import_rewrite::build_named_import("qrl", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_inlined_qrl {
            let stmt = import_rewrite::build_named_import("inlinedQrl", core_module, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_captures {
            let stmt = import_rewrite::build_named_import("_captures", core_module, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_rest_props {
            let stmt = import_rewrite::build_named_import("_restProps", core_module, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_jsx_sorted {
            let stmt = import_rewrite::build_named_import("_jsxSorted", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_get_var_props {
            let stmt = import_rewrite::build_named_import("_getVarProps", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_get_const_props {
            let stmt = import_rewrite::build_named_import("_getConstProps", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_jsx_split {
            let stmt = import_rewrite::build_named_import("_jsxSplit", core_module, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_wrap_prop {
            let stmt = import_rewrite::build_named_import("_wrapProp", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_fn_signal {
            let stmt = import_rewrite::build_named_import("_fnSignal", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_val {
            let stmt = import_rewrite::build_named_import("_val", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_chk {
            let stmt = import_rewrite::build_named_import("_chk", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_noop_qrl {
            let stmt = import_rewrite::build_named_import("_noopQrl", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_qrl_sync {
            let stmt = import_rewrite::build_named_import("_qrlSync", core_module, ctx);
            new_stmts.push(stmt);
        }

        for (hash, import_path) in &self.import_tracker.lazy_imports {
            let stmt = import_rewrite::build_lazy_import_declaration(hash, import_path, ctx);
            new_stmts.push(stmt);
        }

        if self.import_tracker.needs_fragment {
            let stmt = import_rewrite::build_aliased_import(
                "Fragment",
                "_Fragment",
                "@qwik.dev/core/jsx-runtime",
                ctx,
            );
            new_stmts.push(stmt);
        }

        // Collect non-dollar specifiers from Qwik core imports that need to be preserved.
        // These are specifiers like `useStore` that were imported alongside $-suffixed ones.
        // Build constants (isServer, isBrowser, isDev) are NOT re-emitted because they are
        // handled by const_replace which strips them from the AST.
        // Aliased specifiers (e.g., `isServer as myServer`) are re-emitted with the alias.
        const BUILD_CONSTANTS: &[&str] = &["isServer", "isBrowser", "isDev"];
        for import_info in &self.collected.module_imports {
            if import_info.is_qwik_core {
                for spec_name in &import_info.specifiers {
                    if self.collected.dollar_imports.contains(spec_name) {
                        continue; // Dollar import: stripped
                    }
                    // Check if this specifier is a build constant (by imported name)
                    let imported_name = import_info
                        .specifier_aliases
                        .get(spec_name)
                        .map(|s| s.as_str())
                        .unwrap_or(spec_name.as_str());
                    if BUILD_CONSTANTS.contains(&imported_name) {
                        continue; // Build constant: handled by const_replace
                    }

                    if import_info.specifier_aliases.contains_key(spec_name) {
                        // Aliased import: emit `import { imported as local } from "..."`
                        let stmt = import_rewrite::build_aliased_import(
                            imported_name,
                            spec_name,
                            &import_info.source,
                            ctx,
                        );
                        new_stmts.push(stmt);
                    } else {
                        // Non-aliased import: emit `import { name } from "..."`
                        let stmt =
                            import_rewrite::build_named_import(spec_name, &import_info.source, ctx);
                        new_stmts.push(stmt);
                    }
                }
            }
        }

        // Always rebuild body: new imports first, then filtered old statements
        let existing_len = program.body.len();
        let mut new_body = ctx.ast.vec_with_capacity(new_stmts.len() + existing_len);

        for stmt in new_stmts {
            new_body.push(stmt);
        }

        let mut old_body = ctx.ast.vec();
        std::mem::swap(&mut program.body, &mut old_body);
        for stmt in old_body {
            // Skip original Qwik core import declarations (they've been replaced
            // by the new imports above: Qrl-suffixed + non-dollar specifiers)
            if let Statement::ImportDeclaration(ref import_decl) = stmt {
                let source = import_decl.source.value.as_str();
                let is_qwik_core = self
                    .collected
                    .module_imports
                    .iter()
                    .any(|i| i.source == source && i.is_qwik_core);
                if is_qwik_core {
                    continue; // Skip -- already re-emitted above
                }
            }
            new_body.push(stmt);
        }

        program.body = new_body;
    }
}

/// Minify a function string for sync$ serialization.
/// Removes comments and normalizes whitespace via parse+codegen roundtrip.
fn minify_fn_string(source: &str) -> String {
    let parse_source = format!("var x = {}", source);
    let parse_alloc = oxc::allocator::Allocator::default();
    let source_for_parse = parse_alloc.alloc_str(&parse_source);

    let parser =
        oxc::parser::Parser::new(&parse_alloc, source_for_parse, oxc::span::SourceType::mjs());
    let parse_result = parser.parse();

    if !parse_result.errors.is_empty() || parse_result.program.body.is_empty() {
        return source.to_string();
    }

    if let Some(Statement::VariableDeclaration(decl)) = parse_result.program.body.first() {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(ref init) = declarator.init {
                let mut codegen = oxc::codegen::Codegen::new();
                codegen.print_expression(init);
                return codegen.into_source_text();
            }
        }
    }

    source.to_string()
}

/// Convert an Argument to an Expression.
///
/// In OXC 0.113, Argument uses inherit_variants! from Expression, meaning
/// all Expression variants are directly on Argument. We need to match and convert.
pub(crate) fn argument_to_expression<'a>(
    arg: Argument<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    match arg {
        Argument::SpreadElement(_) => ctx.ast.expression_identifier(SPAN, "undefined"),
        // Argument inherits all Expression variants
        Argument::ArrayExpression(e) => Expression::ArrayExpression(e),
        Argument::ArrowFunctionExpression(e) => Expression::ArrowFunctionExpression(e),
        Argument::AssignmentExpression(e) => Expression::AssignmentExpression(e),
        Argument::AwaitExpression(e) => Expression::AwaitExpression(e),
        Argument::BinaryExpression(e) => Expression::BinaryExpression(e),
        Argument::CallExpression(e) => Expression::CallExpression(e),
        Argument::ChainExpression(e) => Expression::ChainExpression(e),
        Argument::ClassExpression(e) => Expression::ClassExpression(e),
        Argument::ConditionalExpression(e) => Expression::ConditionalExpression(e),
        Argument::FunctionExpression(e) => Expression::FunctionExpression(e),
        Argument::Identifier(e) => Expression::Identifier(e),
        Argument::ImportExpression(e) => Expression::ImportExpression(e),
        Argument::LogicalExpression(e) => Expression::LogicalExpression(e),
        Argument::MetaProperty(e) => Expression::MetaProperty(e),
        Argument::NewExpression(e) => Expression::NewExpression(e),
        Argument::ObjectExpression(e) => Expression::ObjectExpression(e),
        Argument::ParenthesizedExpression(e) => Expression::ParenthesizedExpression(e),
        Argument::SequenceExpression(e) => Expression::SequenceExpression(e),
        Argument::TaggedTemplateExpression(e) => Expression::TaggedTemplateExpression(e),
        Argument::TemplateLiteral(e) => Expression::TemplateLiteral(e),
        Argument::ThisExpression(e) => Expression::ThisExpression(e),
        Argument::UnaryExpression(e) => Expression::UnaryExpression(e),
        Argument::UpdateExpression(e) => Expression::UpdateExpression(e),
        Argument::YieldExpression(e) => Expression::YieldExpression(e),
        Argument::BooleanLiteral(e) => Expression::BooleanLiteral(e),
        Argument::NullLiteral(e) => Expression::NullLiteral(e),
        Argument::NumericLiteral(e) => Expression::NumericLiteral(e),
        Argument::BigIntLiteral(e) => Expression::BigIntLiteral(e),
        Argument::RegExpLiteral(e) => Expression::RegExpLiteral(e),
        Argument::StringLiteral(e) => Expression::StringLiteral(e),
        // Catch-all for any other inherited variants
        _ => ctx.ast.expression_identifier(SPAN, "undefined"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsx_transform::minify_expression_string;

    #[test]
    fn test_dollar_call_kind_debug() {
        let kind = DollarCallKind::RawDollar;
        assert!(format!("{:?}", kind).contains("RawDollar"));

        let kind = DollarCallKind::Named("component$".to_string());
        assert!(format!("{:?}", kind).contains("component$"));
    }

    #[test]
    fn test_import_tracker_default() {
        let tracker = ImportTracker::default();
        assert!(tracker.qrl_imports.is_empty());
        assert!(!tracker.needs_qrl);
        assert!(!tracker.needs_inlined_qrl);
        assert!(!tracker.needs_captures);
        assert!(tracker.lazy_imports.is_empty());
    }

    #[test]
    fn test_minify_expression_string_preserves_ident_space() {
        // BUG-01: spaces between identifier characters must be preserved
        assert_eq!(minify_expression_string("a b"), "a b");
        assert_eq!(minify_expression_string("foo bar"), "foo bar");
        assert_eq!(minify_expression_string("return value"), "return value");
    }

    #[test]
    fn test_minify_expression_string_removes_extra_whitespace() {
        // Multiple spaces collapse to one
        assert_eq!(minify_expression_string("a    b"), "a b");
        // Leading/trailing whitespace removed
        assert_eq!(minify_expression_string("  a  "), "a");
        // Spaces around operators removed
        assert_eq!(minify_expression_string("a + b"), "a+b");
        assert_eq!(minify_expression_string("a  +  b"), "a+b");
    }

    #[test]
    fn test_minify_expression_string_preserves_strings() {
        assert_eq!(minify_expression_string("'hello world'"), "'hello world'");
        assert_eq!(
            minify_expression_string("\"hello world\""),
            "\"hello world\""
        );
    }
}
