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

    /// Post-transform processing: populate child segment metadata.
    /// Must be called after traverse_mut completes.
    pub fn finalize_segments(&mut self) {
        // Build a list of child segment info: (parent_display_name, child_hash, child_import_path)
        let child_info: Vec<(String, String, String)> = self
            .segments
            .iter()
            .filter_map(|seg| {
                seg.parent.as_ref().map(|parent_name| {
                    let canonical = format!("{}_{}", seg.display_name, seg.hash);
                    let import_path = format!("./{}", canonical);
                    (parent_name.clone(), seg.hash.clone(), import_path)
                })
            })
            .collect();

        // For each parent segment, add child lazy imports
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
        }
    }

    /// Check if a CallExpression is a Qwik $-call.
    fn is_dollar_call(&self, call: &CallExpression<'_>) -> Option<DollarCallKind> {
        match &call.callee {
            Expression::Identifier(ident) => {
                let name = ident.name.as_str();
                if self.collected.dollar_imports.contains(name) {
                    if name == "$" {
                        Some(DollarCallKind::RawDollar)
                    } else {
                        Some(DollarCallKind::Named(name.to_string()))
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
        // Look up the call site in the collector data by matching span
        let call_start = call.span.start;
        let call_end = call.span.end;
        for site in &self.collected.dollar_calls {
            if site.span.0 == call_start && site.span.1 == call_end {
                return site.display_name.clone();
            }
        }
        // Fallback: use a counter-based name
        format!("s_{}", self.segment_counter)
    }

    /// Build the canonical filename for a segment.
    fn build_canonical_filename(&self, display_name: &str, hash: &str) -> String {
        // Strip extension from filename for the canonical name
        let base = self
            .filename
            .rfind('.')
            .map(|i| &self.filename[..i])
            .unwrap_or(&self.filename);
        format!("{base}_{display_name}_{hash}")
    }

    /// Build the segment import path for segment strategy.
    fn build_segment_import_path(&self, canonical_filename: &str) -> String {
        format!("./{canonical_filename}")
    }

    /// Detect file extension from filename.
    fn file_extension(&self) -> String {
        self.filename
            .rsplit('.')
            .next()
            .unwrap_or("js")
            .to_string()
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
    fn record_segment(
        &mut self,
        call: &CallExpression<'_>,
        kind: &DollarCallKind,
    ) -> SegmentData {
        let display_name = self.derive_display_name_for_call(call);

        // Compute the full display name with filename prefix (for hashing)
        let full_display_name = format!("{}_{}", self.filename, display_name);

        let segment_hash = hash::compute_segment_hash(
            self.options.scope.as_deref(),
            &self.filename,
            &full_display_name,
        );

        let segment_name = hash::format_segment_name(&display_name, &segment_hash);
        let canonical_filename =
            self.build_canonical_filename(&display_name, &segment_hash);
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
            captures: false,       // Computed in exit_expression
            capture_names: vec![], // Computed in exit_expression
            needed_imports: vec![], // Populated by finalize_segments
            body_span: (call.span.start, call.span.end),
            param_names: vec![],   // Set by props destructuring if needed
            body_code: String::new(), // Populated in exit_expression for segment strategy
            child_lazy_imports: vec![], // Populated by finalize_segments
            needs_qrl_import: false,   // Populated by finalize_segments
        };

        // Track imports based on strategy
        let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
                || matches!(self.options.entry_strategy, crate::types::EntryStrategy::Hoist);

        if is_inline {
            self.import_tracker.needs_inlined_qrl = true;
        } else {
            self.import_tracker.needs_qrl = true;
            // Add lazy import for segment strategy
            self.import_tracker
                .lazy_imports
                .push((segment_hash.clone(), import_path));
        }

        // For named calls, track the Qrl-suffixed import
        if let DollarCallKind::Named(name) = kind {
            let qrl_name = words::dollar_to_qrl_name(name);
            if !self.import_tracker.qrl_imports.contains(&qrl_name) {
                self.import_tracker.qrl_imports.push(qrl_name);
            }
        }

        self.segment_counter += 1;
        self.segments.push(segment.clone());
        segment
    }
}

impl<'a> Traverse<'a, ()> for QwikTransform {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(kind) = self.is_dollar_call(call) {
            // For component$ calls, check for props destructuring
            if let DollarCallKind::Named(ref name) = kind {
                if name == "component$" {
                    // Check if first argument is an ArrowFunctionExpression
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

            // Push a new capture tracking frame for this $()-body.
            // Each $()-body gets its own set of identifier refs and local declarations.
            self.capture_stack.push((Vec::new(), HashSet::new()));

            // Collect parameter names of the arrow function body as body-local declarations
            if let Some(arg) = call.arguments.first() {
                if let Argument::ArrowFunctionExpression(arrow) = arg {
                    for param in &arrow.params.items {
                        self.collect_binding_pattern_names(&param.pattern);
                    }
                    // Also collect rest parameter
                    if let Some(rest) = &arrow.params.rest {
                        self.collect_binding_pattern_names(&rest.rest.argument);
                    }
                }
            }

            // Record the segment
            let segment = self.record_segment(call, &kind);

            // Mark this call span so exit_expression can find it
            self.pending_dollar_calls.insert(call.span.start);

            // Push to nesting stack
            self.dollar_call_stack.push(segment.display_name.clone());
        }
    }

    fn enter_identifier_reference(
        &mut self,
        ident: &mut IdentifierReference<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // If we're inside a $()-body, collect the identifier name for capture analysis.
        // Add to the TOPMOST (current) capture stack frame.
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

    fn exit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // JSX transformation: replace JSXElement/JSXFragment with _jsxSorted/_jsxSplit calls
        if self.options.transpile_jsx {
            // Build destructured props map for signal wrapping detection
            let destr_props: Option<Vec<(String, String)>> = self.active_props_info.as_ref().map(|info| {
                info.prop_keys.iter().map(|(key, local)| (local.clone(), key.clone())).collect()
            });
            let destr_props_ref = destr_props.as_deref();

            match expr {
                Expression::JSXElement(_) => {
                    // Take the JSXElement out to process it
                    let placeholder = ctx.ast.expression_null_literal(SPAN);
                    let old_expr = std::mem::replace(expr, placeholder);
                    if let Expression::JSXElement(el) = old_expr {
                        let result = transform_jsx_element_inner(
                            el.unbox(),
                            &mut self.import_tracker,
                            ctx,
                            destr_props_ref,
                        );
                        *expr = result;
                    }
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
                        );
                        *expr = result;
                    }
                    return;
                }
                _ => {}
            }
        }

        if let Expression::CallExpression(call) = expr {
            // Check if this was a detected dollar call
            if !self.pending_dollar_calls.remove(&call.span.start) {
                return;
            }

            let kind = match self.is_dollar_call(call) {
                Some(k) => k,
                None => return,
            };

            // Pop from nesting stack
            self.dollar_call_stack.pop();

            // Apply props destructuring if active for this component$ call.
            // Only take() the active_props_info for the component$ call itself,
            // not for nested $() calls inside it.
            let is_component_exit = matches!(&kind, DollarCallKind::Named(name) if name == "component$");
            let props_info = if is_component_exit {
                self.active_props_info.take()
            } else {
                None
            };
            if let Some(ref info) = props_info {
                if let DollarCallKind::Named(ref name) = kind {
                    if name == "component$" {
                        // Apply the transformation to the arrow function argument
                        if let Some(Argument::ArrowFunctionExpression(arrow)) =
                            call.arguments.first_mut()
                        {
                            // 1. Replace the ObjectPattern parameter with _rawProps BindingIdentifier
                            if !arrow.params.items.is_empty() {
                                let new_pattern = ctx.ast.binding_pattern_binding_identifier(
                                    SPAN,
                                    ctx.ast.atom(&info.raw_props_name),
                                );
                                let new_param = ctx.ast.formal_parameter(
                                    SPAN,
                                    ctx.ast.vec(),  // no decorators
                                    new_pattern,
                                    None::<oxc::allocator::Box<'a, TSTypeAnnotation<'a>>>,
                                    None::<oxc::allocator::Box<'a, Expression<'a>>>,
                                    false,          // not optional
                                    None,           // no accessibility
                                    false,          // not readonly
                                    false,          // no override
                                );
                                arrow.params.items[0] = new_param;
                                // Remove rest param from FormalParameters if present
                                // (the rest is handled via _restProps call)
                                arrow.params.rest = None;
                            }

                            // 2. If rest_name is present, prepend _restProps declaration to body
                            if let Some(ref rest_name) = info.rest_name {
                                let excluded_keys: Vec<String> = info
                                    .prop_keys
                                    .iter()
                                    .map(|(key, _)| key.clone())
                                    .collect();
                                let rest_stmt = props_destructuring::build_rest_props_declaration(
                                    rest_name,
                                    &info.raw_props_name,
                                    &excluded_keys,
                                    ctx,
                                );

                                // Prepend to body statements
                                let mut old_stmts = ctx.ast.vec();
                                std::mem::swap(&mut arrow.body.statements, &mut old_stmts);
                                let mut new_stmts =
                                    ctx.ast.vec_with_capacity(1 + old_stmts.len());
                                new_stmts.push(rest_stmt);
                                for s in old_stmts {
                                    new_stmts.push(s);
                                }
                                arrow.body.statements = new_stmts;
                            }

                            // 3. Rewrite identifier references in the body
                            // Build (local_alias, original_key) pairs for the rewriter
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

                        // Update the segment's param_names
                        if let Some(seg) = self.segments.iter_mut().find(|s| {
                            s.span.0 == call.span.start && s.span.1 == call.span.end
                        }) {
                            seg.param_names = vec![info.raw_props_name.clone()];
                        }

                        // Post-process captures for child segments: replace individual
                        // destructured prop names with _rawProps.
                        // Because capture analysis runs during inner $() exit (before
                        // props destructuring rewrites the AST), child segments may have
                        // captured individual prop aliases (e.g., "foo") instead of "_rawProps".
                        // We need to replace those with the raw props name.
                        let local_aliases: HashSet<String> = info
                            .prop_keys
                            .iter()
                            .map(|(_, local)| local.clone())
                            .collect();

                        // Find all child segments (segments whose parent matches this component's display_name).
                        // The child segment's `parent` field stores the parent segment's `display_name`.
                        let component_span = (call.span.start, call.span.end);
                        let component_display_name = self
                            .segments
                            .iter()
                            .find(|s| s.span == component_span)
                            .map(|s| s.display_name.clone());

                        if let Some(parent_name) = component_display_name {
                            for seg in self.segments.iter_mut() {
                                if seg.parent.as_ref() == Some(&parent_name) && !seg.capture_names.is_empty() {
                                    let mut needs_rawprops = false;
                                    seg.capture_names.retain(|name| {
                                        if local_aliases.contains(name) {
                                            needs_rawprops = true;
                                            false // remove the individual prop name
                                        } else {
                                            true
                                        }
                                    });
                                    if needs_rawprops {
                                        // Add _rawProps if not already present
                                        if !seg.capture_names.contains(&info.raw_props_name) {
                                            seg.capture_names.insert(0, info.raw_props_name.clone());
                                        }
                                    }
                                    seg.captures = !seg.capture_names.is_empty();
                                }
                            }
                        }
                    }
                }
            }

            // Pop the capture tracking frame for this $()-body
            let (body_ident_refs, body_local_decls) = self
                .capture_stack
                .pop()
                .unwrap_or_default();

            // Compute captures using the collected identifier references and local declarations
            let capture_result = collector::compute_captures(
                &body_ident_refs,
                &body_local_decls,
                &self.collected,
            );

            // Update the segment's capture metadata
            if let Some(seg) = self.segments.iter_mut().find(|s| {
                s.span.0 == call.span.start && s.span.1 == call.span.end
            }) {
                seg.captures = !capture_result.capture_names.is_empty();
                seg.capture_names = capture_result.capture_names.clone();
            }

            // If captures are non-empty and strategy is inline, track _captures import
            let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
                || matches!(self.options.entry_strategy, crate::types::EntryStrategy::Hoist);
            if !capture_result.capture_names.is_empty() && is_inline {
                self.import_tracker.needs_captures = true;
            }

            // Find the segment info we recorded for this call
            let segment_info = self
                .segments
                .iter()
                .find(|s| s.span.0 == call.span.start && s.span.1 == call.span.end)
                .cloned();

            let segment_info = match segment_info {
                Some(s) => s,
                None => return,
            };

            // For segment strategy, serialize the body expression to a string
            // before building the replacement. This captures the transformed body
            // (after props destructuring + capture analysis have modified it).
            // The body argument is extracted and serialized, then discarded --
            // it won't be used again because segment strategy replaces the entire
            // call expression with qrl(i_hash, "name", captures).
            if !is_inline && !call.arguments.is_empty() {
                let placeholder =
                    Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"));
                let body_arg =
                    std::mem::replace(&mut call.arguments[0], placeholder);

                // Convert Argument to Expression for serialization
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
                // body_expr is dropped here -- the original call.arguments[0] is
                // now a placeholder, but the entire call expression will be replaced
                // by *expr = final_expr below, so this is fine.
            }

            // Build the replacement expression
            let replacement = if is_inline {
                // Inline strategy: inlinedQrl(body, "name_hash")
                // Extract the first argument (the arrow/expression body)
                let body_expr = if !call.arguments.is_empty() {
                    // Take the first argument's expression
                    let arg = &mut call.arguments[0];
                    // Move the argument expression out
                    std::mem::replace(
                        arg,
                        Argument::from(ctx.ast.expression_identifier(SPAN, "undefined")),
                    )
                } else {
                    Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"))
                };

                // Convert Argument to Expression
                let body_as_expr = match body_expr {
                    Argument::SpreadElement(_) => {
                        ctx.ast.expression_identifier(SPAN, "undefined")
                    }
                    _ => {
                        // Argument inherits from Expression via inherit_variants!
                        // We need to extract the expression
                        argument_to_expression(body_expr, ctx)
                    }
                };

                import_rewrite::build_inlined_qrl_call(
                    body_as_expr,
                    &segment_info.name,
                    &segment_info.capture_names,
                    ctx,
                )
            } else {
                // Segment strategy: qrl(i_hash, "name_hash", captures)
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

            // Replace the expression in-place
            *expr = final_expr;
        }
    }

    fn exit_program(
        &mut self,
        program: &mut Program<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        let core_module = &self.options.core_module;

        let mut new_stmts: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();

        // 1. Add Qrl-suffixed imports
        for qrl_name in &self.import_tracker.qrl_imports {
            let stmt = import_rewrite::build_named_import(qrl_name, core_module, ctx);
            new_stmts.push(stmt);
        }

        // 2. Add qrl or inlinedQrl import
        if self.import_tracker.needs_qrl {
            let stmt = import_rewrite::build_named_import("qrl", core_module, ctx);
            new_stmts.push(stmt);
        }
        if self.import_tracker.needs_inlined_qrl {
            let stmt = import_rewrite::build_named_import("inlinedQrl", core_module, ctx);
            new_stmts.push(stmt);
        }

        // 3. Add _captures import if needed
        if self.import_tracker.needs_captures {
            let stmt = import_rewrite::build_named_import("_captures", core_module, ctx);
            new_stmts.push(stmt);
        }

        // 3b. Add _restProps import if needed (props destructuring with rest patterns)
        if self.import_tracker.needs_rest_props {
            let stmt = import_rewrite::build_named_import("_restProps", core_module, ctx);
            new_stmts.push(stmt);
        }

        // 4. Add JSX-related imports
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

        // 4b. Add signal/binding-related imports
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

        // 5. Add lazy import constants (segment strategy)
        for (hash, import_path) in &self.import_tracker.lazy_imports {
            let stmt = import_rewrite::build_lazy_import_declaration(hash, import_path, ctx);
            new_stmts.push(stmt);
        }

        // 5b. Add Fragment aliased import from jsx-runtime (after lazy imports)
        if self.import_tracker.needs_fragment {
            let stmt = import_rewrite::build_aliased_import(
                "Fragment",
                "_Fragment",
                "@qwik.dev/core/jsx-runtime",
                ctx,
            );
            new_stmts.push(stmt);
        }

        // 5. Prepend new statements before existing program body
        if !new_stmts.is_empty() {
            // Build a new arena vec with capacity for both new and existing statements
            let existing_len = program.body.len();
            let mut new_body = ctx.ast.vec_with_capacity(new_stmts.len() + existing_len);

            // Add new statements first
            for stmt in new_stmts {
                new_body.push(stmt);
            }

            // Move existing statements (drain the arena vec)
            // We need to use a swap approach since OXC Vec doesn't implement Default
            let mut old_body = ctx.ast.vec();
            std::mem::swap(&mut program.body, &mut old_body);
            for stmt in old_body {
                new_body.push(stmt);
            }

            program.body = new_body;
        }
    }
}

// ---------------------------------------------------------------------------
// JSX Transformation Helpers
// ---------------------------------------------------------------------------

/// Normalize JSX text: collapse whitespace, strip leading/trailing newlines.
/// Returns empty string for whitespace-only text.
fn normalize_jsx_text(raw: &str) -> String {
    // Split into lines
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut parts: Vec<String> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = if i == 0 {
            line.trim_end()
        } else if i == lines.len() - 1 {
            line.trim_start()
        } else {
            line.trim()
        };
        if !trimmed.is_empty() {
            parts.push(trimmed.to_string());
        }
    }

    parts.join(" ")
}

/// Transform a JSX event attribute name to its output form.
///
/// Handles patterns:
/// - onClick$ -> q-e:click
/// - onDocumentScroll$ -> q-e:documentscroll
/// - on-cLick$ -> q-e:c-lick
/// - onDocument-sCroll$ -> q-e:document--scroll
/// - document:onFocus$ -> q-d:focus
/// - window:onClick$ -> q-w:click
/// - onKeyup$ -> q-e:keyup
/// - onDocument:keyup$ -> q-e:document:keyup
/// - onWindow:keyup$ -> q-e:window:keyup
/// - host:onClick$ -> host:onClick$ (kept as-is, deprecated)
///
/// Returns None if the attribute is not a transformable event handler.
fn transform_event_attr_name(attr_name: &str) -> Option<String> {
    // Handle scope prefixes: document:, window:
    if let Some(rest) = attr_name.strip_prefix("document:") {
        if rest.starts_with("on") && rest.ends_with('$') {
            let event_part = &rest[2..rest.len() - 1];
            return Some(format!("q-d:{}", event_part.to_lowercase()));
        }
        return None;
    }
    if let Some(rest) = attr_name.strip_prefix("window:") {
        if rest.starts_with("on") && rest.ends_with('$') {
            let event_part = &rest[2..rest.len() - 1];
            return Some(format!("q-w:{}", event_part.to_lowercase()));
        }
        return None;
    }
    if attr_name.starts_with("host:") {
        // host: prefix is kept as-is (deprecated)
        return None;
    }

    // Standard event: onClick$, onDocumentScroll$, on-cLick$
    if attr_name.starts_with("on") && attr_name.ends_with('$') {
        let event_part = &attr_name[2..attr_name.len() - 1];

        // Handle colon-separated scope: onDocument:keyup$ -> q-e:document:keyup
        if let Some(colon_pos) = event_part.find(':') {
            let scope = &event_part[..colon_pos];
            let event = &event_part[colon_pos + 1..];
            return Some(format!(
                "q-e:{}:{}",
                scope.to_lowercase(),
                event.to_lowercase()
            ));
        }

        let event_lower = event_part.to_lowercase();
        return Some(format!("q-e:{}", event_lower));
    }

    None
}

/// Check if a JSX attribute value is a compile-time constant for prop classification.
fn is_const_jsx_value(value: &Expression<'_>) -> bool {
    crate::is_const::is_const_expression(value)
}

/// Result of analyzing a JSX prop value for signal wrapping.
enum SignalWrapResult {
    /// Expression should be wrapped with _wrapProp(signal) -- Form 1.
    /// The String is the signal identifier name (object of .value).
    WrapPropSignal,
    /// Expression should be wrapped with _wrapProp(source, "propName") -- Form 2.
    /// The Strings are (source_name, prop_name).
    WrapPropNamed(String),
    /// No signal wrapping needed; use normal var/const classification.
    None,
}

/// Detect if a JSX prop value expression needs signal wrapping.
///
/// Rules:
/// - `X.value` where X is a simple identifier -> WrapPropSignal
///   BUT NOT `X.value()` (call on .value)
/// - `_rawProps.propName` where _rawProps is the props parameter -> WrapPropNamed
/// - Identifier matching a destructured prop key -> WrapPropNamed (with original key)
///
/// `destructured_props` is an optional map of (local_alias, original_key) pairs
/// from active props destructuring. If an identifier matches a local alias,
/// it will be treated as _rawProps.originalKey.
fn detect_signal_wrap(
    value: &Expression<'_>,
    destructured_props: Option<&[(String, String)]>,
) -> SignalWrapResult {
    match value {
        Expression::StaticMemberExpression(member) => {
            let prop_name = member.property.name.as_str();

            // Check if it's X.value where X is a simple identifier
            if prop_name == "value" {
                if let Expression::Identifier(ident) = &member.object {
                    let _name = ident.name.as_str();
                    return SignalWrapResult::WrapPropSignal;
                }
            }

            // Check if it's _rawProps.propName (props parameter member access)
            if let Expression::Identifier(ident) = &member.object {
                if ident.name.as_str() == "_rawProps" && prop_name != "value" {
                    return SignalWrapResult::WrapPropNamed(prop_name.to_string());
                }
            }

            SignalWrapResult::None
        }
        // Check for destructured prop identifier: if `fromProps` matches a
        // destructured prop alias, treat as _wrapProp(_rawProps, "fromProps")
        Expression::Identifier(ident) => {
            if let Some(props) = destructured_props {
                let name = ident.name.as_str();
                for (local_alias, original_key) in props {
                    if local_alias == name {
                        return SignalWrapResult::WrapPropNamed(original_key.clone());
                    }
                }
            }
            SignalWrapResult::None
        }
        _ => SignalWrapResult::None,
    }
}

/// Check if an expression is a call expression on a .value member.
/// E.g., `signal.value()` -- this should NOT be wrapped.
fn is_call_on_value(value: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = value {
        if let Expression::StaticMemberExpression(member) = &call.callee {
            return member.property.name.as_str() == "value";
        }
    }
    false
}

/// Build the tag expression for a JSX element name.
fn build_tag_expression<'a>(
    name: &JSXElementName<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    match name {
        JSXElementName::Identifier(ident) => {
            let tag_name = ident.name.as_str();
            if tag_name
                .chars()
                .next()
                .map_or(false, |c| c.is_lowercase())
            {
                let atom = ctx.ast.atom(tag_name);
                ctx.ast.expression_string_literal(SPAN, atom, None)
            } else {
                let atom = ctx.ast.atom(tag_name);
                ctx.ast.expression_identifier(SPAN, atom)
            }
        }
        JSXElementName::IdentifierReference(ident_ref) => {
            let atom = ctx.ast.atom(ident_ref.name.as_str());
            ctx.ast.expression_identifier(SPAN, atom)
        }
        JSXElementName::MemberExpression(member) => build_jsx_member_expr(member, ctx),
        JSXElementName::NamespacedName(ns) => {
            let name = format!("{}:{}", ns.namespace.name, ns.name.name);
            let atom = ctx.ast.atom(&name);
            ctx.ast.expression_string_literal(SPAN, atom, None)
        }
        JSXElementName::ThisExpression(_) => ctx.ast.expression_this(SPAN),
    }
}

/// Build a member expression from a JSXMemberExpression (e.g., Foo.Bar.Baz).
fn build_jsx_member_expr<'a>(
    member: &JSXMemberExpression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let object = match &member.object {
        JSXMemberExpressionObject::IdentifierReference(ident) => {
            let atom = ctx.ast.atom(ident.name.as_str());
            ctx.ast.expression_identifier(SPAN, atom)
        }
        JSXMemberExpressionObject::MemberExpression(inner) => build_jsx_member_expr(inner, ctx),
        JSXMemberExpressionObject::ThisExpression(_) => ctx.ast.expression_this(SPAN),
    };
    let property_atom = ctx.ast.atom(member.property.name.as_str());
    let property = ctx.ast.identifier_name(SPAN, property_atom);
    Expression::StaticMemberExpression(ctx.ast.alloc_static_member_expression(
        SPAN, object, property, false,
    ))
}

/// Convert a JSXExpression to an Expression. JSXExpression uses inherit_variants!
/// from Expression, so all Expression variants appear directly on JSXExpression.
fn jsx_expression_to_expression<'a>(
    jsx_expr: JSXExpression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    match jsx_expr {
        JSXExpression::EmptyExpression(_) => ctx.ast.expression_identifier(SPAN, "undefined"),
        // Inherited Expression variants - literals
        JSXExpression::BooleanLiteral(e) => Expression::BooleanLiteral(e),
        JSXExpression::NullLiteral(e) => Expression::NullLiteral(e),
        JSXExpression::NumericLiteral(e) => Expression::NumericLiteral(e),
        JSXExpression::BigIntLiteral(e) => Expression::BigIntLiteral(e),
        JSXExpression::RegExpLiteral(e) => Expression::RegExpLiteral(e),
        JSXExpression::StringLiteral(e) => Expression::StringLiteral(e),
        JSXExpression::TemplateLiteral(e) => Expression::TemplateLiteral(e),
        // Identifiers and special
        JSXExpression::Identifier(e) => Expression::Identifier(e),
        JSXExpression::MetaProperty(e) => Expression::MetaProperty(e),
        JSXExpression::Super(e) => Expression::Super(e),
        // Compound expressions
        JSXExpression::ArrayExpression(e) => Expression::ArrayExpression(e),
        JSXExpression::ArrowFunctionExpression(e) => Expression::ArrowFunctionExpression(e),
        JSXExpression::AssignmentExpression(e) => Expression::AssignmentExpression(e),
        JSXExpression::AwaitExpression(e) => Expression::AwaitExpression(e),
        JSXExpression::BinaryExpression(e) => Expression::BinaryExpression(e),
        JSXExpression::CallExpression(e) => Expression::CallExpression(e),
        JSXExpression::ChainExpression(e) => Expression::ChainExpression(e),
        JSXExpression::ClassExpression(e) => Expression::ClassExpression(e),
        JSXExpression::ConditionalExpression(e) => Expression::ConditionalExpression(e),
        JSXExpression::FunctionExpression(e) => Expression::FunctionExpression(e),
        JSXExpression::ImportExpression(e) => Expression::ImportExpression(e),
        JSXExpression::LogicalExpression(e) => Expression::LogicalExpression(e),
        JSXExpression::NewExpression(e) => Expression::NewExpression(e),
        JSXExpression::ObjectExpression(e) => Expression::ObjectExpression(e),
        JSXExpression::ParenthesizedExpression(e) => Expression::ParenthesizedExpression(e),
        JSXExpression::SequenceExpression(e) => Expression::SequenceExpression(e),
        JSXExpression::TaggedTemplateExpression(e) => Expression::TaggedTemplateExpression(e),
        JSXExpression::ThisExpression(e) => Expression::ThisExpression(e),
        JSXExpression::UnaryExpression(e) => Expression::UnaryExpression(e),
        JSXExpression::UpdateExpression(e) => Expression::UpdateExpression(e),
        JSXExpression::YieldExpression(e) => Expression::YieldExpression(e),
        JSXExpression::PrivateInExpression(e) => Expression::PrivateInExpression(e),
        // Member expressions (inherited from MemberExpression)
        JSXExpression::ComputedMemberExpression(e) => Expression::ComputedMemberExpression(e),
        JSXExpression::StaticMemberExpression(e) => Expression::StaticMemberExpression(e),
        JSXExpression::PrivateFieldExpression(e) => Expression::PrivateFieldExpression(e),
        // JSX
        JSXExpression::JSXElement(e) => Expression::JSXElement(e),
        JSXExpression::JSXFragment(e) => Expression::JSXFragment(e),
        // TypeScript expressions
        JSXExpression::TSAsExpression(e) => Expression::TSAsExpression(e),
        JSXExpression::TSSatisfiesExpression(e) => Expression::TSSatisfiesExpression(e),
        JSXExpression::TSTypeAssertion(e) => Expression::TSTypeAssertion(e),
        JSXExpression::TSNonNullExpression(e) => Expression::TSNonNullExpression(e),
        JSXExpression::TSInstantiationExpression(e) => Expression::TSInstantiationExpression(e),
        JSXExpression::V8IntrinsicExpression(e) => Expression::V8IntrinsicExpression(e),
    }
}

/// Convert a JSXAttributeValue to an Expression, taking ownership.
fn jsx_attr_value_to_expression<'a>(
    value: JSXAttributeValue<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
) -> Expression<'a> {
    match value {
        JSXAttributeValue::StringLiteral(lit) => Expression::StringLiteral(lit),
        JSXAttributeValue::ExpressionContainer(container) => {
            jsx_expression_to_expression(container.unbox().expression, ctx)
        }
        JSXAttributeValue::Element(el) => {
            // JSX element as attribute value: transform it
            transform_jsx_element_inner(el.unbox(), tracker, ctx, destructured_props)
        }
        JSXAttributeValue::Fragment(frag) => {
            transform_jsx_fragment_inner(frag.unbox(), tracker, ctx, destructured_props)
        }
    }
}

/// Recursively transform a JSXElement into a _jsxSorted/_jsxSplit call expression.
fn transform_jsx_element_inner<'a>(
    mut element: JSXElement<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
) -> Expression<'a> {
    let tag = build_tag_expression(&element.opening_element.name, ctx);

    // Classify attributes: detect spreads, separate key, classify var/const props
    let mut has_spread = false;
    let mut key_value: Option<Expression<'a>> = None;
    let mut var_props: Vec<(String, Expression<'a>)> = Vec::new();
    let mut const_props: Vec<(String, Expression<'a>)> = Vec::new();
    let mut spread_args: Vec<Expression<'a>> = Vec::new();
    let mut _has_only_events = true;
    let mut has_any_visible_prop = false;

    // Take attributes out of the opening element
    let mut attrs = ctx.ast.vec();
    std::mem::swap(&mut element.opening_element.attributes, &mut attrs);

    for attr_item in attrs {
        match attr_item {
            JSXAttributeItem::SpreadAttribute(spread) => {
                has_spread = true;
                spread_args.push(spread.unbox().argument);
            }
            JSXAttributeItem::Attribute(attr) => {
                let attr = attr.unbox();
                let attr_name = match &attr.name {
                    JSXAttributeName::Identifier(ident) => ident.name.as_str().to_string(),
                    JSXAttributeName::NamespacedName(ns) => {
                        format!("{}:{}", ns.namespace.name, ns.name.name)
                    }
                };

                // Handle key attribute
                if attr_name == "key" {
                    if let Some(val) = attr.value {
                        key_value = Some(jsx_attr_value_to_expression(val, tracker, ctx, destructured_props));
                    }
                    continue;
                }

                // Check for event handler attributes
                if let Some(event_name) = transform_event_attr_name(&attr_name) {
                    // Event handler: value goes into const props with renamed key
                    let value = if let Some(val) = attr.value {
                        jsx_attr_value_to_expression(val, tracker, ctx, destructured_props)
                    } else {
                        ctx.ast.expression_boolean_literal(SPAN, true)
                    };
                    const_props.push((event_name, value));
                    continue;
                }

                // Check for host: prefix (kept as-is) or custom$ (kept as-is)
                if attr_name.starts_with("host:") || (attr_name.ends_with('$') && !attr_name.starts_with("on")) {
                    let value = if let Some(val) = attr.value {
                        jsx_attr_value_to_expression(val, tracker, ctx, destructured_props)
                    } else {
                        ctx.ast.expression_boolean_literal(SPAN, true)
                    };
                    const_props.push((attr_name, value));
                    continue;
                }

                // Regular attribute
                has_any_visible_prop = true;
                _has_only_events = false;
                let value = if let Some(val) = attr.value {
                    jsx_attr_value_to_expression(val, tracker, ctx, destructured_props)
                } else {
                    // Boolean attribute: <input disabled /> -> disabled: true
                    ctx.ast.expression_boolean_literal(SPAN, true)
                };

                // Check for signal wrapping BEFORE const/var classification.
                // signal.value -> _wrapProp(signal) in const props
                // _rawProps.propName -> _wrapProp(_rawProps, "propName") in const props
                // But NOT signal.value() (function call on .value)
                if !is_call_on_value(&value) {
                    match detect_signal_wrap(&value, destructured_props) {
                        SignalWrapResult::WrapPropSignal => {
                            // Extract the signal identifier from X.value
                            if let Expression::StaticMemberExpression(member) = value {
                                let signal_obj = member.unbox().object;
                                let wrapped = import_rewrite::build_wrap_prop_call(signal_obj, ctx);
                                tracker.needs_wrap_prop = true;
                                const_props.push((attr_name, wrapped));
                                continue;
                            }
                        }
                        SignalWrapResult::WrapPropNamed(prop_name) => {
                            // Build _wrapProp(source, "propName") in const props.
                            // Source is either _rawProps (for destructured props) or extracted from
                            // a StaticMemberExpression (for _rawProps.propName).
                            let source_obj = if let Expression::StaticMemberExpression(member) = value {
                                member.unbox().object
                            } else {
                                // For destructured prop identifiers, build _rawProps reference
                                ctx.ast.expression_identifier(SPAN, "_rawProps")
                            };
                            let wrapped = import_rewrite::build_wrap_prop_call_named(
                                source_obj, &prop_name, ctx,
                            );
                            tracker.needs_wrap_prop = true;
                            const_props.push((attr_name, wrapped));
                            continue;
                        }
                        SignalWrapResult::None => {}
                    }
                }

                if is_const_jsx_value(&value) {
                    const_props.push((attr_name, value));
                } else {
                    var_props.push((attr_name, value));
                }
            }
        }
    }

    // Build children
    let (children_expr, children_count) =
        transform_jsx_children(&mut element.children, tracker, ctx, destructured_props);

    // Compute flags
    let flags = if has_spread {
        0
    } else if children_count > 1 {
        1
    } else {
        3
    };

    // Generate key
    let key_expr = if let Some(key) = key_value {
        key
    } else if children_count > 0 || has_any_visible_prop || !const_props.is_empty() || !var_props.is_empty() {
        // Generate auto-key for elements with content (they may be siblings)
        let key_str = format!("u6_{}", tracker.jsx_key_counter);
        tracker.jsx_key_counter += 1;
        let atom = ctx.ast.atom(&key_str);
        ctx.ast.expression_string_literal(SPAN, atom, None)
    } else {
        ctx.ast.expression_null_literal(SPAN)
    };

    if has_spread {
        // Use _jsxSplit for elements with spread attributes
        tracker.needs_jsx_split = true;
        tracker.needs_get_var_props = true;
        tracker.needs_get_const_props = true;

        // Build: _jsxSplit(tag, { ..._getVarProps(source) }, _getConstProps(source), children, flags, key)
        // For multiple spreads, use the first one (simplification)
        let spread_source = if !spread_args.is_empty() {
            spread_args.remove(0)
        } else {
            ctx.ast.expression_identifier(SPAN, "undefined")
        };

        // Build varProps = { ..._getVarProps(source), ...otherVarProps }
        let mut var_obj_props = ctx.ast.vec_with_capacity(1 + var_props.len());

        // _getVarProps(source) spread
        let get_var_callee = ctx.ast.expression_identifier(SPAN, "_getVarProps");
        let mut get_var_args = ctx.ast.vec_with_capacity(1);
        // Clone the spread source expression for _getVarProps
        let spread_source_clone = ctx.ast.expression_identifier(SPAN,
            // Try to extract name from spread_source
            if let Expression::Identifier(ref ident) = spread_source {
                ctx.ast.atom(ident.name.as_str())
            } else {
                ctx.ast.atom("props")
            }
        );
        get_var_args.push(Argument::from(spread_source_clone));
        let get_var_call = ctx.ast.expression_call(
            SPAN,
            get_var_callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            get_var_args,
            false,
        );
        var_obj_props.push(ctx.ast.object_property_kind_spread_property(SPAN, get_var_call));

        // Add non-spread var props
        for (name, value) in var_props {
            let key = ctx.ast.property_key_static_identifier(SPAN, ctx.ast.atom(&name));
            var_obj_props.push(ctx.ast.object_property_kind_object_property(
                SPAN,
                PropertyKind::Init,
                key,
                value,
                false,
                false,
                false,
            ));
        }

        let var_props_expr = ctx.ast.expression_object(SPAN, var_obj_props);

        // Build _getConstProps(source) for constProps argument
        let get_const_callee = ctx.ast.expression_identifier(SPAN, "_getConstProps");
        let mut get_const_args = ctx.ast.vec_with_capacity(1);
        get_const_args.push(Argument::from(spread_source));
        let const_props_expr = ctx.ast.expression_call(
            SPAN,
            get_const_callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            get_const_args,
            false,
        );

        // Build the _jsxSplit call
        let callee = ctx.ast.expression_identifier(SPAN, "_jsxSplit");
        let mut arguments = ctx.ast.vec_with_capacity(6);
        arguments.push(Argument::from(tag));
        arguments.push(Argument::from(var_props_expr));
        arguments.push(Argument::from(const_props_expr));
        arguments.push(Argument::from(
            children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
        ));
        arguments.push(Argument::from(ctx.ast.expression_numeric_literal(
            SPAN,
            flags as f64,
            None,
            NumberBase::Decimal,
        )));
        arguments.push(Argument::from(key_expr));

        ctx.ast.expression_call_with_pure(
            SPAN,
            callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            arguments,
            false,
            true,
        )
    } else {
        // Use _jsxSorted for normal elements
        tracker.needs_jsx_sorted = true;

        // Build var_props object or null
        let var_props_arg = if var_props.is_empty() {
            ctx.ast.expression_null_literal(SPAN)
        } else {
            let mut props_vec = ctx.ast.vec_with_capacity(var_props.len());
            for (name, value) in var_props {
                let key = ctx.ast.property_key_static_identifier(SPAN, ctx.ast.atom(&name));
                props_vec.push(ctx.ast.object_property_kind_object_property(
                    SPAN,
                    PropertyKind::Init,
                    key,
                    value,
                    false,
                    false,
                    false,
                ));
            }
            ctx.ast.expression_object(SPAN, props_vec)
        };

        // Build const_props object or null
        let const_props_arg = if const_props.is_empty() {
            ctx.ast.expression_null_literal(SPAN)
        } else {
            let mut props_vec = ctx.ast.vec_with_capacity(const_props.len());
            for (name, value) in const_props {
                // Use string literal key for names with special chars (q-e:, etc.)
                let key = if name.contains(':') || name.contains('-') || name.contains('$') {
                    let atom = ctx.ast.atom(&name);
                    PropertyKey::from(ctx.ast.expression_string_literal(SPAN, atom, None))
                } else {
                    ctx.ast.property_key_static_identifier(SPAN, ctx.ast.atom(&name))
                };
                props_vec.push(ctx.ast.object_property_kind_object_property(
                    SPAN,
                    PropertyKind::Init,
                    key,
                    value,
                    false,
                    false,
                    false,
                ));
            }
            ctx.ast.expression_object(SPAN, props_vec)
        };

        // Build the _jsxSorted call
        let callee = ctx.ast.expression_identifier(SPAN, "_jsxSorted");
        let mut arguments = ctx.ast.vec_with_capacity(6);
        arguments.push(Argument::from(tag));
        arguments.push(Argument::from(var_props_arg));
        arguments.push(Argument::from(const_props_arg));
        arguments.push(Argument::from(
            children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
        ));
        arguments.push(Argument::from(ctx.ast.expression_numeric_literal(
            SPAN,
            flags as f64,
            None,
            NumberBase::Decimal,
        )));
        arguments.push(Argument::from(key_expr));

        ctx.ast.expression_call_with_pure(
            SPAN,
            callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            arguments,
            false,
            true,
        )
    }
}

/// Recursively transform a JSXFragment into a _jsxSorted call with _Fragment tag.
fn transform_jsx_fragment_inner<'a>(
    mut fragment: JSXFragment<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
) -> Expression<'a> {
    tracker.needs_jsx_sorted = true;
    tracker.needs_fragment = true;

    let tag = ctx.ast.expression_identifier(SPAN, "_Fragment");

    // Build children
    let (children_expr, children_count) =
        transform_jsx_children(&mut fragment.children, tracker, ctx, destructured_props);

    // Flags: 1 for multiple children, 3 for single/no children
    let flags: u32 = if children_count > 1 { 1 } else { 3 };

    // Generate auto-key
    let key_str = format!("u6_{}", tracker.jsx_key_counter);
    tracker.jsx_key_counter += 1;
    let key_atom = ctx.ast.atom(&key_str);
    let key_expr = ctx.ast.expression_string_literal(SPAN, key_atom, None);

    let callee = ctx.ast.expression_identifier(SPAN, "_jsxSorted");
    let mut arguments = ctx.ast.vec_with_capacity(6);
    arguments.push(Argument::from(tag));
    arguments.push(Argument::from(ctx.ast.expression_null_literal(SPAN)));
    arguments.push(Argument::from(ctx.ast.expression_null_literal(SPAN)));
    arguments.push(Argument::from(
        children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
    ));
    arguments.push(Argument::from(ctx.ast.expression_numeric_literal(
        SPAN,
        flags as f64,
        None,
        NumberBase::Decimal,
    )));
    arguments.push(Argument::from(key_expr));

    ctx.ast.expression_call_with_pure(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
        true,
    )
}

/// Transform JSX children to expression(s).
/// Returns (children_expression, significant_children_count).
fn transform_jsx_children<'a>(
    children: &mut oxc::allocator::Vec<'a, JSXChild<'a>>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
) -> (Option<Expression<'a>>, usize) {
    let mut child_exprs: Vec<Expression<'a>> = Vec::new();

    // Take children out to process them
    let mut old_children = ctx.ast.vec();
    std::mem::swap(children, &mut old_children);

    for child in old_children {
        match child {
            JSXChild::Text(text) => {
                let trimmed = normalize_jsx_text(text.value.as_str());
                if !trimmed.is_empty() {
                    let atom = ctx.ast.atom(&trimmed);
                    child_exprs.push(ctx.ast.expression_string_literal(SPAN, atom, None));
                }
            }
            JSXChild::Element(el) => {
                // Recursively transform child JSXElement
                let transformed = transform_jsx_element_inner(el.unbox(), tracker, ctx, destructured_props);
                child_exprs.push(transformed);
            }
            JSXChild::Fragment(frag) => {
                // Recursively transform child JSXFragment
                let transformed = transform_jsx_fragment_inner(frag.unbox(), tracker, ctx, destructured_props);
                child_exprs.push(transformed);
            }
            JSXChild::ExpressionContainer(container) => {
                let container = container.unbox();
                match container.expression {
                    JSXExpression::EmptyExpression(_) => {
                        // Skip empty expressions {}
                    }
                    expr => {
                        // The expression inside the container may already have been
                        // transformed by exit_expression (e.g., $() calls, or inner
                        // JSXElement expressions). Convert to Expression.
                        let transformed = jsx_expression_to_expression(expr, ctx);
                        // If it's a JSXElement or JSXFragment, transform it
                        match transformed {
                            Expression::JSXElement(el) => {
                                let result =
                                    transform_jsx_element_inner(el.unbox(), tracker, ctx, destructured_props);
                                child_exprs.push(result);
                            }
                            Expression::JSXFragment(frag) => {
                                let result =
                                    transform_jsx_fragment_inner(frag.unbox(), tracker, ctx, destructured_props);
                                child_exprs.push(result);
                            }
                            other => {
                                // Check if child expression is signal.value -> _wrapProp(signal)
                                if !is_call_on_value(&other) {
                                    if let SignalWrapResult::WrapPropSignal = detect_signal_wrap(&other, destructured_props) {
                                        if let Expression::StaticMemberExpression(member) = other {
                                            let signal_obj = member.unbox().object;
                                            let wrapped = import_rewrite::build_wrap_prop_call(signal_obj, ctx);
                                            tracker.needs_wrap_prop = true;
                                            child_exprs.push(wrapped);
                                            continue;
                                        }
                                    }
                                }
                                child_exprs.push(other);
                            }
                        }
                    }
                }
            }
            JSXChild::Spread(spread) => {
                let spread = spread.unbox();
                child_exprs.push(spread.expression);
            }
        }
    }

    let count = child_exprs.len();
    match count {
        0 => (None, 0),
        1 => (Some(child_exprs.into_iter().next().unwrap()), 1),
        _ => {
            let mut elements = ctx.ast.vec_with_capacity(count);
            for child in child_exprs {
                elements.push(ArrayExpressionElement::from(child));
            }
            (Some(ctx.ast.expression_array(SPAN, elements)), count)
        }
    }
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
}
