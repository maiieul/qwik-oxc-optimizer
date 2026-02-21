//! Main QwikTransform traverse implementation.
//!
//! The core of the optimizer. Implements the `Traverse` trait to walk the AST
//! and apply all Qwik transformations: replace `$()` calls with `qrl()` wrappers,
//! record segments for extraction, rewrite imports, and handle special patterns
//! (component$, useTask$, etc.).

use std::collections::{HashMap, HashSet};

use oxc::ast::ast::*;
use oxc::ast::Comment;
use oxc::span::SPAN;
use oxc_traverse::{Traverse, TraverseCtx};

use crate::collector;
use crate::entry_strategy;
use crate::hash;
use crate::import_rewrite;
use crate::jsx_transform::{
    get_jsx_lambda_span, transform_jsx_element_inner,
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

    /// Custom JSX import source for `import { jsx as _jsx }` from `{source}/jsx-runtime`.
    /// When Some, `_jsx` is used instead of `_jsxSorted` for JSX transform output.
    pub custom_jsx_source: Option<String>,

    /// Monotonic counter for generating unique JSX key suffixes like "u6_0", "u6_1".
    pub jsx_key_counter: u32,

    /// Monotonic counter for hoisted function names (_hf0, _hf1, ...).
    pub hoisted_fn_counter: u32,

    /// Set of identifier names that are considered "immutable" component tags.
    /// Using these as JSX element tags does NOT set jsx_mutable = true.
    /// Built from imports: Fragment, RenderOnce, Link, and any import from ?jsx or .md sources.
    /// Mirrors SWC's immutable_function_cmp.
    pub immutable_function_cmp: HashSet<String>,

    /// Tracks whether the current JSX subtree has been marked as mutable.
    /// Set to true when mutable children or non-immutable component tags are encountered.
    /// Saved/restored around children processing to avoid leaking between siblings.
    /// Mirrors SWC's jsx_mutable.
    pub jsx_mutable: bool,

    /// Set of identifier names known to be const-bound (imports + const declarations).
    /// Used for scope-aware JSX prop/children immutability classification.
    /// Mirrors SWC's ConstCollector which tracks imports and const bindings.
    pub const_bindings: HashSet<String>,

    /// In dev mode, the fileName for JSX dev location metadata.
    /// When Some, `_jsxSorted` calls get an extra `{ fileName, lineNumber, columnNumber }` argument.
    pub jsx_dev_file_name: Option<String>,

    /// Source code for computing line/column from byte offsets (dev mode only).
    /// Stored as a String reference to avoid lifetime issues.
    pub jsx_dev_source_code: Option<String>,
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

    /// Stack of "invalid declaration" names (function/class declarations) per $()-body.
    /// In SWC, these go to `invalid_decl` and are NOT captured -- instead C02 diagnostics
    /// are emitted. Matches SWC's partition of decl_stack into (decl_collect, invalid_decl).
    invalid_decl_stack: Vec<HashSet<String>>,

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

    /// Custom JSX import source module path (e.g., "react" from `@jsxImportSource react`).
    /// When Some, JSX event handler `$`-attributes are NOT extracted as segments
    /// because the JSX is not Qwik JSX.
    custom_jsx_import_source: Option<String>,

    /// Original source code, used for extracting JSX lambda body code by span.
    source_code: String,

    /// Source comments from the original parse, used for comment-preserving codegen.
    /// Stored as standard Vec (Comment is Copy) since arena Vec can't outlive the allocator.
    source_comments: Vec<Comment>,

    /// JSX event handler replacement info, keyed by lambda expression span start.
    /// When the JSX transform encounters an event handler attribute whose value
    /// expression has a span start in this map, it replaces the value with
    /// a `qrl()` or `inlinedQrl()` call instead of keeping the raw lambda.
    jsx_event_replacements: HashMap<u32, JsxEventReplacement>,

    /// Scope context stack (mirrors SWC's stack_ctxt).
    /// Accumulates ALL scope names (variable names, function names, JSX element tags,
    /// attribute names, callee names) as the AST is traversed. Display names are
    /// built by joining this stack with "_" and running through `escape_sym()`.
    stack_ctxt: Vec<String>,

    /// Segment name stack for parent field (mirrors SWC's segment_stack).
    /// Stores the segment_name (display_name + hash, e.g., "renderHeader_XXXXXXXXXXXX")
    /// of each enclosing segment. Used to set the `parent` field on child segments.
    segment_stack: Vec<String>,

    /// Deduplication counter for display names (mirrors SWC's segment_names).
    /// When the same display name occurs multiple times, appends "_1", "_2", etc.
    segment_names: HashMap<String, u32>,

    /// Depth markers for variable declarator stack_ctxt push/pop.
    /// Each entry records the stack_ctxt length before the declarator was entered.
    var_decl_ctxt_depths: Vec<usize>,

    /// Depth markers for function declaration stack_ctxt push/pop.
    fn_decl_ctxt_depths: Vec<usize>,

    /// Depth markers for call expression stack_ctxt push/pop.
    call_expr_ctxt_depths: Vec<usize>,

    /// Depth markers for export default declaration stack_ctxt push/pop.
    export_default_ctxt_depths: Vec<usize>,

    /// Stack tracking whether current JSX element is a native HTML element.
    /// Mirrors SWC's `jsx_element_is_native` stack.
    jsx_element_is_native: Vec<bool>,

    /// Depth counter for nested loops. When > 0, QRL calls are inside a loop context.
    loop_depth: u32,

    /// Stack of iteration variables for each loop scope.
    /// Each entry contains the iteration variable names for that loop level.
    iteration_var_stack: Vec<Vec<String>>,

    /// Depth counter for iteration method callbacks (.map/.filter/etc).
    /// Increment on entering an iteration method call, decrement on exit.
    /// When > 0, the current scope is inside an iteration method callback.
    /// Uses a depth counter (not bool) to handle nested iteration methods
    /// correctly: e.g. arr.map(x => x.items.filter(y => ...)) — exiting
    /// the inner .filter() decrements to 1 (still inside .map()), not 0.
    in_callback_depth: u32,

    /// Whether the next JSX element processed is the "root" element in its scope.
    /// When true, the element gets a generated key; when false, native elements get null.
    /// Set to true on entering function/arrow bodies and statement-level scopes
    /// (for/while/if/block/return), set to false after first JSX element processes.
    /// Mirrors SWC's root_jsx_mode.
    root_jsx_mode: bool,

    /// Stack for saving/restoring root_jsx_mode across nested scopes.
    root_jsx_mode_stack: Vec<bool>,

    /// Precomputed JSX key prefix string (e.g., "u6" for test.tsx).
    /// Computed from base64url(DefaultHasher(scope?, rel_path).to_le_bytes())[0..2]
    /// with '-' and '_' chars replaced by '0'.
    jsx_key_prefix: String,

    /// QRL declarations to hoist to the top of the enclosing function body.
    /// Each entry: (lazy_import_ident_name, segment_export_name, capture_names).
    /// Populated when replace_jsx_element_handlers processes a segment-strategy
    /// QRL replacement while loop_depth > 0.
    /// Flushed in exit_function / exit_arrow_function_expression when loop_depth == 0.
    pending_loop_qrl_hoists: Vec<(String, String, Vec<String>)>,

    /// Iteration variables used by each event handler, keyed by lambda span start.
    /// Populated during enter_call_expression when processing JSX event handler
    /// lambdas inside loops. Consumed during replace_jsx_element_handlers for
    /// q:p/q:ps attribute injection at the correct element level.
    iter_var_usage_by_handler: HashMap<u32, Vec<String>>,
}

/// Info needed to build a qrl()/inlinedQrl() call for a JSX event handler.
#[derive(Debug, Clone)]
pub(crate) struct JsxEventReplacement {
    /// The segment name (e.g., "test_div_q_e_click_abc123")
    pub segment_name: String,
    /// The segment hash (e.g., "abc123")
    pub hash: String,
    /// Captured variable names
    pub capture_names: Vec<String>,
    /// Whether to use inlinedQrl (true) or qrl (false)
    pub is_inline: bool,
    /// The full display name for dev mode metadata (e.g., "test.tsx_App_component_Cmp_p_q_e_click")
    pub display_name: String,
    /// The body span (lo, hi) for dev mode metadata
    pub body_span: (u32, u32),
}

impl QwikTransform {
    /// Create a new QwikTransform instance.
    pub fn new(
        options: &TransformOptions,
        collected: CollectResult,
        filename: &str,
        source_code: &str,
    ) -> Self {
        // Compute JSX key prefix from file hash (matches SWC transform.ts lines 163-183)
        let jsx_key_prefix = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;

            // Normalize Windows backslashes before hashing to match SWC
            let normalized_filename = filename.replace('\\', "/");
            let mut hasher = DefaultHasher::new();
            if let Some(ref scope) = options.scope {
                hasher.write(scope.as_bytes());
            }
            hasher.write(normalized_filename.as_bytes());
            let file_hash = hasher.finish();

            // Base64url encode first 2 chars of LE bytes
            // MUST use URL_SAFE alphabet (matches SWC), NOT standard base64 (+/)
            const CHARS: &[u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
            let bytes = file_hash.to_le_bytes();
            let b0 = bytes[0] as usize;
            let b1 = bytes[1] as usize;
            let c0 = CHARS[b0 >> 2] as char;
            let c1 = CHARS[((b0 & 0x03) << 4) | (b1 >> 4)] as char;
            // Replace - and _ with 0 (matching SWC)
            let c0 = if c0 == '-' || c0 == '_' { '0' } else { c0 };
            let c1 = if c1 == '-' || c1 == '_' { '0' } else { c1 };
            format!("{}{}", c0, c1)
        };

        // Build immutable_function_cmp set from imports (mirrors SWC lines 205-232).
        // These are component tags that don't set jsx_mutable = true.
        let mut immutable_function_cmp = HashSet::new();
        // Always include _Fragment (the transform-generated import name)
        immutable_function_cmp.insert("_Fragment".to_string());
        for import in &collected.module_imports {
            let source = &import.source;

            // Fragment from jsx-runtime or jsx-dev-runtime
            if source.contains("jsx-runtime") || source.contains("jsx-dev-runtime") {
                for local_name in &import.specifiers {
                    let imported_name = import
                        .specifier_aliases
                        .get(local_name)
                        .map(|s| s.as_str())
                        .unwrap_or(local_name.as_str());
                    if imported_name == "Fragment" {
                        immutable_function_cmp.insert(local_name.clone());
                    }
                }
            }

            // Fragment, RenderOnce from @qwik.dev/core or @builder.io/qwik
            if source == "@qwik.dev/core" || source == "@builder.io/qwik" {
                for local_name in &import.specifiers {
                    let imported_name = import
                        .specifier_aliases
                        .get(local_name)
                        .map(|s| s.as_str())
                        .unwrap_or(local_name.as_str());
                    if imported_name == "Fragment" || imported_name == "RenderOnce" {
                        immutable_function_cmp.insert(local_name.clone());
                    }
                }
            }

            // Link from @qwik.dev/router or @builder.io/qwik-city
            if source == "@qwik.dev/router" || source == "@builder.io/qwik-city" {
                for local_name in &import.specifiers {
                    let imported_name = import
                        .specifier_aliases
                        .get(local_name)
                        .map(|s| s.as_str())
                        .unwrap_or(local_name.as_str());
                    if imported_name == "Link" {
                        immutable_function_cmp.insert(local_name.clone());
                    }
                }
            }

            // ALL names from ?jsx or .md sources
            if source.ends_with("?jsx") || source.ends_with(".md") {
                for local_name in &import.specifiers {
                    immutable_function_cmp.insert(local_name.clone());
                }
            }
        }

        // Build const_bindings from all import specifier names.
        // Imports are always const in JavaScript -- they cannot be reassigned.
        // This mirrors SWC's ConstCollector which includes all imports.
        let mut const_bindings = HashSet::new();
        for imp in &collected.module_imports {
            for spec in &imp.specifiers {
                const_bindings.insert(spec.clone());
            }
        }

        Self {
            options: options.clone(),
            collected,
            filename: filename.to_string(),
            segments: Vec::new(),
            diagnostics: Vec::new(),
            import_tracker: ImportTracker {
                immutable_function_cmp,
                const_bindings,
                jsx_dev_file_name: if matches!(options.mode, crate::types::EmitMode::Dev) {
                    Some(filename.to_string())
                } else {
                    None
                },
                jsx_dev_source_code: if matches!(options.mode, crate::types::EmitMode::Dev) {
                    Some(source_code.to_string())
                } else {
                    None
                },
                ..ImportTracker::default()
            },
            segment_counter: 0,
            dollar_call_stack: Vec::new(),
            pending_dollar_calls: HashSet::new(),
            active_props_info: None,
            capture_stack: Vec::new(),
            invalid_decl_stack: Vec::new(),
            segment_body_codes: Vec::new(),
            hoisted_function_stmts: Vec::new(),
            stripped_segments: HashSet::new(),
            pending_sync_calls: HashSet::new(),
            pending_segment_qrl_imports: Vec::new(),
            custom_jsx_import_source: None,
            source_code: source_code.to_string(),
            source_comments: Vec::new(),
            jsx_event_replacements: HashMap::new(),
            stack_ctxt: Vec::new(),
            segment_stack: Vec::new(),
            segment_names: HashMap::new(),
            var_decl_ctxt_depths: Vec::new(),
            fn_decl_ctxt_depths: Vec::new(),
            call_expr_ctxt_depths: Vec::new(),
            export_default_ctxt_depths: Vec::new(),
            jsx_element_is_native: Vec::new(),
            loop_depth: 0,
            iteration_var_stack: Vec::new(),
            in_callback_depth: 0,
            root_jsx_mode: true,
            root_jsx_mode_stack: Vec::new(),
            jsx_key_prefix,
            pending_loop_qrl_hoists: Vec::new(),
            iter_var_usage_by_handler: HashMap::new(),
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

    /// Set the custom JSX import source module path (e.g., "react" from `@jsxImportSource react`).
    /// When Some, JSX event handler `$`-attributes are NOT extracted as segments,
    /// and JSX is transformed to `_jsx()` from `{source}/jsx-runtime` instead of `_jsxSorted`.
    pub fn set_custom_jsx_import_source(&mut self, source: Option<String>) {
        self.import_tracker.custom_jsx_source = source.clone();
        self.custom_jsx_import_source = source;
    }

    /// Get the custom JSX import source module path, if any.
    pub fn custom_jsx_import_source(&self) -> Option<&str> {
        self.custom_jsx_import_source.as_deref()
    }

    /// Get all current iteration variables, flattened from all loop scopes.
    pub(crate) fn current_iteration_vars(&self) -> Vec<String> {
        self.iteration_var_stack.iter().flatten().cloned().collect()
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
        let self_import_source = self.self_import_source();

        for seg in self.segments.iter_mut() {
            // Parent field now stores segment name with hash (e.g., "renderHeader_XXXXXXXXXXXX"),
            // so match against seg.name (which is also segment name with hash).
            let children: Vec<_> = child_info
                .iter()
                .filter(|(parent, _, _)| parent == &seg.name)
                .collect();

            if !children.is_empty() {
                seg.needs_qrl_import = true;
                for (_, hash, path) in children {
                    seg.child_lazy_imports.push((hash.clone(), path.clone()));
                }
            }

            // Assign Qrl-suffixed imports from nested $-calls to their parent segment.
            // If the Qrl name is a locally-defined export (not a framework import),
            // add it as a self-import in needed_imports instead of segment_qrl_names.
            // This matches SWC's behavior where local_idents found in global.exports
            // become self-imports (import from "./filename") rather than core imports.
            for (parent_name, qrl_name) in &pending_qrl_imports {
                if parent_name == &seg.display_name && !seg.segment_qrl_names.contains(qrl_name) {
                    if self.collected.module_level_decls.contains(qrl_name.as_str()) {
                        // Locally-defined Qrl function: import from self module
                        let already_imported = seg.needed_imports.iter().any(|imp| {
                            imp.specifiers.contains(qrl_name)
                        });
                        if !already_imported {
                            seg.needed_imports.push(crate::types::ImportInfo {
                                source: self_import_source.clone(),
                                specifiers: vec![qrl_name.clone()],
                                specifier_kinds: vec![crate::types::ImportKind::Named],
                                specifier_aliases: std::collections::HashMap::new(),
                                is_qwik_core: false,
                                span: (0, 0),
                            });
                        }
                    } else {
                        seg.segment_qrl_names.push(qrl_name.clone());
                    }
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

    /// Extract the basename with extension (file_name) from the filename path.
    ///
    /// E.g., "src/routes/_repl/[id]/[[...slug]].tsx" -> "[[...slug]].tsx"
    /// E.g., "test.tsx" -> "test.tsx"
    /// E.g., "components\\\\apps\\\\apps.tsx" -> "apps.tsx"
    ///
    /// Handles both Unix `/` and Windows `\\` path separators.
    /// Matches SWC's `path_data.file_name`.
    fn file_name(&self) -> &str {
        // Find the last path separator (either / or \)
        let last_sep = self
            .filename
            .rfind(|c: char| c == '/' || c == '\\')
            .map(|pos| pos + 1)
            .unwrap_or(0);
        &self.filename[last_sep..]
    }

    /// Extract the file stem (basename without extension) from the filename path.
    ///
    /// Uses Rust's Path::file_stem() logic: strips only the LAST extension.
    /// E.g., "[[...slug]].tsx" -> "[[...slug]]"
    /// E.g., "test.tsx" -> "test"
    /// E.g., "404.tsx" -> "404"
    ///
    /// Matches SWC's `path_data.file_stem`.
    fn file_stem(&self) -> String {
        let basename = self.file_name();
        // Strip only the last extension (after the last dot, if the dot is not at position 0)
        if let Some(dot_pos) = basename.rfind('.') {
            if dot_pos > 0 {
                return basename[..dot_pos].to_string();
            }
        }
        basename.to_string()
    }

    /// Build the canonical filename for a segment.
    ///
    /// Uses file_name (basename with extension), not the full path.
    /// Matches SWC's `get_canonical_filename(display_name, symbol_name)`.
    fn build_canonical_filename(&self, display_name: &str, hash: &str) -> String {
        format!("{}_{display_name}_{hash}", self.file_name())
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

    /// Whether the current emit mode is Dev.
    fn is_dev_mode(&self) -> bool {
        matches!(self.options.mode, crate::types::EmitMode::Dev)
    }

    /// Compute the absolute file path for dev mode metadata.
    /// Matches SWC's `path_data.abs_path.to_slash_lossy()`: `normalize_path(src_dir.join(filename))`
    fn dev_abs_path(&self) -> String {
        let src_dir = &self.options.src_dir;
        let filename = &self.filename;
        if src_dir == "." || src_dir.is_empty() {
            format!("./{}", filename)
        } else {
            let src = src_dir.trim_end_matches('/');
            format!("{}/{}", src, filename)
        }
    }

    /// Build dev metadata for a QRL call, or None if not in dev mode.
    /// - `lo`/`hi`: byte offsets of the $()-call body (0-based, from OXC spans)
    /// - `display_name`: the full display name of the segment
    ///
    /// SWC uses 1-based byte positions (BytePos), so we add 1 to match golden output.
    fn make_qrl_dev_meta(
        &self,
        lo: u32,
        hi: u32,
        display_name: &str,
    ) -> Option<import_rewrite::QrlDevMetadata> {
        if !self.is_dev_mode() {
            return None;
        }
        Some(import_rewrite::QrlDevMetadata {
            file: self.dev_abs_path(),
            lo: lo + 1,
            hi: hi + 1,
            display_name: display_name.to_string(),
        })
    }

    /// Build dev metadata for a _noopQrl call (lo and hi are always 0).
    fn make_noop_dev_meta(
        &self,
        display_name: &str,
    ) -> Option<import_rewrite::QrlDevMetadata> {
        if !self.is_dev_mode() {
            return None;
        }
        Some(import_rewrite::QrlDevMetadata {
            file: self.dev_abs_path(),
            lo: 0,
            hi: 0,
            display_name: display_name.to_string(),
        })
    }

    /// Compute the self-import source path for module-level declaration re-imports.
    ///
    /// When a nested segment references a module-level declaration (const, function,
    /// class), the SWC optimizer re-imports it from the parent module rather than
    /// capturing it. This method returns the import source path (e.g., "./test"
    /// for a file named "test.tsx").
    fn self_import_source(&self) -> String {
        let stem = self
            .filename
            .rsplit('/')
            .next()
            .unwrap_or(&self.filename)
            .rsplit('.')
            .last()
            .unwrap_or(&self.filename);
        format!("./{}", stem)
    }

    /// Post-process a capture analysis result to convert module-level declarations
    /// from captures into needed_imports (self-imports from the parent module).
    ///
    /// The SWC optimizer handles module-level declarations (const, function, class)
    /// by re-importing them in the segment module rather than serializing/restoring
    /// them via `_captures[]`. This post-processing step implements that behavior.
    fn reclassify_module_level_decl_captures(
        &self,
        mut capture_result: collector::CaptureAnalysisResult,
    ) -> (collector::CaptureAnalysisResult, Vec<crate::types::ImportInfo>) {
        let self_import_source = self.self_import_source();
        let mut extra_imports: Vec<crate::types::ImportInfo> = Vec::new();

        // Partition capture_names: keep non-module-level-decl names as true captures,
        // convert module-level-decl names to needed_imports (self-imports).
        let mut true_captures = Vec::new();
        for name in capture_result.capture_names.drain(..) {
            if self.collected.module_level_decls.contains(&name) {
                extra_imports.push(crate::types::ImportInfo {
                    source: self_import_source.clone(),
                    specifiers: vec![name],
                    specifier_kinds: vec![crate::types::ImportKind::Named],
                    specifier_aliases: std::collections::HashMap::new(),
                    is_qwik_core: false,
                    span: (0, 0),
                });
            } else {
                true_captures.push(name);
            }
        }

        capture_result.capture_names = true_captures;
        (capture_result, extra_imports)
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
            BindingPattern::AssignmentPattern(assign) => {
                self.collect_binding_pattern_names(&assign.left);
            }
        }
    }

    /// Build display name from the stack_ctxt, applying escape_sym, digit prefix,
    /// and deduplication -- mirrors SWC's `register_context_name`.
    ///
    /// Returns `(display_name, full_display_name, segment_hash, segment_name)`.
    fn register_context_name(&mut self) -> (String, String, String, String) {
        // 1. Join stack context
        let mut display_name = if self.stack_ctxt.is_empty() {
            "s_".to_string()
        } else {
            self.stack_ctxt.join("_")
        };

        // 2. Escape non-alphanumeric chars
        display_name = escape_sym(&display_name);

        // 3. Ensure valid identifier start
        if display_name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            display_name = format!("_{}", display_name);
        }

        // 4. Deduplicate (mirrors SWC's segment_names logic)
        let index = match self.segment_names.get_mut(&display_name) {
            Some(count) => {
                *count += 1;
                *count
            }
            None => 0,
        };
        if index == 0 {
            self.segment_names.insert(display_name.clone(), 0);
        } else {
            display_name = format!("{}_{}", display_name, index);
        }

        // 5. Hash -- computed on display_name (WITHOUT filename prefix), matching SWC.
        // SWC hashes: scope? + rel_path + display_name (no file_name prefix).
        // Then AFTER hashing, prepends file_name to display_name for the full display name.
        let segment_hash = hash::compute_segment_hash(
            self.options.scope.as_deref(),
            &self.filename,
            &display_name,
        );

        // 6. Prepend file_name (basename with extension) to display_name
        let full_display_name = format!("{}_{}", self.file_name(), display_name);

        // 7. Build segment_name -- in Dev/Lib modes use "display_name_hash",
        // in Prod mode use "s_hash" (mirrors SWC's register_context_name).
        // SWC uses Dev|Test for full names and Lib|Prod for s_HASH, but since
        // OXC tests default to Lib mode (matching SWC's Test mode behavior),
        // we only use s_HASH for Prod mode.
        let segment_name = if matches!(self.options.mode, crate::types::EmitMode::Prod) {
            format!("s_{}", segment_hash)
        } else {
            hash::format_segment_name(&display_name, &segment_hash)
        };

        (display_name, full_display_name, segment_hash, segment_name)
    }

    /// Record a segment and track imports. Returns the segment data.
    fn record_segment(&mut self, call: &CallExpression<'_>, kind: &DollarCallKind) -> SegmentData {
        let (display_name, full_display_name, segment_hash, segment_name) =
            self.register_context_name();

        let canonical_filename = self.build_canonical_filename(&display_name, &segment_hash);
        let import_path = self.build_segment_import_path(&canonical_filename);

        let ctx_name = match kind {
            DollarCallKind::RawDollar => "$".to_string(),
            DollarCallKind::Named(name) => name.clone(),
        };
        let ctx_kind = words::classify_ctx_kind(&ctx_name);

        // Parent uses segment_stack (segment name WITH hash), matching SWC
        let parent = self.segment_stack.last().cloned();

        let segment = SegmentData {
            display_name: full_display_name.clone(),
            hash: segment_hash.clone(),
            name: segment_name.clone(),
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
            body_span: if let Some(first_arg) = call.arguments.first() {
                use oxc::span::GetSpan;
                let s = first_arg.span();
                (s.start, s.end)
            } else {
                (call.span.start, call.span.end)
            },
            param_names: if let Some(first_arg) = call.arguments.first() {
                extract_param_names_from_argument(first_arg)
            } else {
                vec![]
            },
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

        // Push to segment_stack (for parent field of nested segments)
        self.segment_stack.push(segment_name);
        // Push to dollar_call_stack (for finalize_segments parent matching)
        self.dollar_call_stack.push(full_display_name);

        self.segments.push(segment.clone());
        segment
    }

    /// Record a segment for a JSX event handler attribute (e.g., onClick$).
    ///
    /// Unlike `record_segment`, this doesn't require a CallExpression -- it takes
    /// the span and ctx_name directly from JSX attribute info.
    /// The display name is derived from `stack_ctxt` (which should already have
    /// the JSX element name and attribute name pushed).
    pub(crate) fn record_jsx_event_segment(
        &mut self,
        span: (u32, u32),
        ctx_name: &str,
        param_names: Vec<String>,
    ) -> SegmentData {
        let (display_name, full_display_name, segment_hash, segment_name) =
            self.register_context_name();

        let canonical_filename = self.build_canonical_filename(&display_name, &segment_hash);
        let import_path = self.build_segment_import_path(&canonical_filename);

        // attribute name pattern. This includes onClick$, onInput$, custom$, etc.
        let ctx_kind = crate::types::CtxKind::EventHandler;
        // Parent uses segment_stack (segment name WITH hash), matching SWC
        let parent = self.segment_stack.last().cloned();

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
            param_names,
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
        if self.custom_jsx_import_source.is_some() {
            return;
        }

        // Push element name to stack_ctxt (mirrors SWC's fold_jsx_element)
        let (element_name, is_native_element) = match &element.opening_element.name {
            JSXElementName::Identifier(ident) => {
                let name = ident.name.as_str().to_string();
                let is_native = name.chars().next().is_some_and(|c| c.is_lowercase());
                (Some(name), is_native)
            }
            JSXElementName::IdentifierReference(ident) => {
                let name = ident.name.as_str().to_string();
                (Some(name), false) // Component elements are not native
            }
            JSXElementName::NamespacedName(ns) => {
                let name = format!("{}:{}", ns.namespace.name, ns.name.name);
                (Some(name), false)
            }
            _ => (None, false),
        };

        if let Some(ref name) = element_name {
            self.stack_ctxt.push(name.clone());
        }

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
                                // Push the attribute name to stack_ctxt (mirrors SWC's fold_jsx_attr)
                                let attr_ctx_name = if let Some(ref prefix) = namespace_prefix {
                                    // Namespaced: push "ns-name$" format
                                    let full_attr = format!("{}:{}", prefix, attr_name_str);
                                    if is_native_element {
                                        if let Some(html_attr) = jsx_event_to_html_attribute(&full_attr) {
                                            html_attr
                                        } else {
                                            format!("{}-{}", prefix, attr_name_str)
                                        }
                                    } else {
                                        format!("{}-{}", prefix, attr_name_str)
                                    }
                                } else if is_native_element {
                                    // Native element: transform event name
                                    if let Some(html_attr) = jsx_event_to_html_attribute(&attr_name_str) {
                                        html_attr
                                    } else {
                                        attr_name_str.clone()
                                    }
                                } else {
                                    // Component element: push original name
                                    attr_name_str.clone()
                                };
                                self.stack_ctxt.push(attr_ctx_name);

                                let ctx_name = if namespace_prefix.is_some() {
                                    format!(
                                        "{}:{}",
                                        namespace_prefix.as_ref().unwrap(),
                                        attr_name_str
                                    )
                                } else {
                                    attr_name_str.clone()
                                };
                                let mut param_names =
                                    extract_param_names_from_jsx_expr(&container.expression);

                                // Run capture analysis on the JSX event handler lambda.
                                // This determines which variables from the enclosing scope
                                // need to be serialized and restored in the segment module.
                                // We do this BEFORE segment recording so we can use the
                                // body_ident_refs to check iteration variable usage.
                                let (body_ident_refs, body_local_decls) =
                                    analyze_lambda_captures(&self.source_code, span);

                                // When inside a loop, check if the handler uses iteration
                                // variables and transform param_names accordingly
                                // (mirrors SWC's transform_event_handler_with_iter_var).
                                if self.loop_depth > 0 {
                                    let iter_vars = self.current_iteration_vars();
                                    // Use deep scan for iteration variable detection.
                                    // body_ident_refs doesn't descend into nested functions,
                                    // but iteration variables can be captured by nested closures.
                                    // SWC's body_contains_ident does a full deep scan.
                                    let deep_refs = analyze_lambda_deep_ident_refs(
                                        &self.source_code, span,
                                    );
                                    let used_iter_vars: Vec<String> = iter_vars
                                        .iter()
                                        .filter(|v| deep_refs.contains(*v))
                                        .cloned()
                                        .collect();
                                    if !used_iter_vars.is_empty() {
                                        // Ensure at least 2 params (event, element)
                                        // SWC uses "_" for both placeholders
                                        while param_names.len() < 2 {
                                            param_names.push("_".to_string());
                                        }
                                        // Append used iteration variables
                                        for var_name in &used_iter_vars {
                                            param_names.push(var_name.clone());
                                        }
                                        // Record used iteration variables keyed by lambda span
                                        // start for q:p injection at the correct element level
                                        // during replace_jsx_element_handlers.
                                        self.iter_var_usage_by_handler
                                            .insert(span.0, used_iter_vars.clone());
                                    }
                                }

                                let seg =
                                    self.record_jsx_event_segment(span, &ctx_name, param_names);
                                let seg_span_0 = seg.span.0;
                                let capture_result = collector::compute_captures(
                                    &body_ident_refs,
                                    &body_local_decls,
                                    &self.collected,
                                );

                                // Filter captures to only include variables that actually
                                // exist in an enclosing scope. When inside $()-bodies,
                                // filter against ALL capture_stack frames' local declarations
                                // (not just the innermost). For nested $() like:
                                //   component$(() => { const state = ...; return $(() => {
                                //     return <div onClick$={() => state.count++} />
                                //   }) })
                                // `state` is in the outer frame, not the inner one.
                                // When NOT inside any $()-body (bare function like
                                // `export default ({data}) => <div onClick$={...}/>`),
                                // keep all captures since compute_captures() already
                                // filtered against module-level declarations/imports.
                                // Collect parent scope's invalid (fn/class) declarations
                                let all_invalid_decls: HashSet<String> = self
                                    .invalid_decl_stack
                                    .iter()
                                    .flat_map(|s| s.iter().cloned())
                                    .collect();

                                let capture_result = if !self.capture_stack.is_empty() {
                                    // Merge all local declarations from all capture stack frames
                                    let all_parent_decls: HashSet<String> = self
                                        .capture_stack
                                        .iter()
                                        .flat_map(|(_, decls)| decls.iter().cloned())
                                        .collect();
                                    let filtered_names: Vec<String> = capture_result
                                        .capture_names
                                        .into_iter()
                                        .filter(|name| {
                                            !all_invalid_decls.contains(name)
                                                && (all_parent_decls.contains(name)
                                                    || self.collected.module_level_decls.contains(name))
                                        })
                                        .collect();
                                    collector::CaptureAnalysisResult {
                                        capture_names: filtered_names,
                                        reemitted_imports: capture_result.reemitted_imports,
                                        diagnostics: capture_result.diagnostics,
                                    }
                                } else {
                                    // No parent $()-body scope -- keep all captures
                                    capture_result
                                };

                                // Reclassify module-level declarations from captures
                                // to needed_imports (self-imports from the parent module).
                                let (capture_result, module_decl_imports) =
                                    self.reclassify_module_level_decl_captures(capture_result);

                                // Convert reemitted imports to ImportInfo for needed_imports
                                let mut needed_imports: Vec<crate::types::ImportInfo> =
                                    capture_result
                                        .reemitted_imports
                                        .iter()
                                        .map(|ri| {
                                            let mut aliases = std::collections::HashMap::new();
                                            if let Some(ref imported) = ri.imported_name {
                                                aliases.insert(
                                                    ri.local_name.clone(),
                                                    imported.clone(),
                                                );
                                            }
                                            crate::types::ImportInfo {
                                                source: ri.source.clone(),
                                                specifiers: vec![ri.local_name.clone()],
                                                specifier_kinds: vec![ri.kind.clone()],
                                                specifier_aliases: aliases,
                                                is_qwik_core: false,
                                                span: (0, 0),
                                            }
                                        })
                                        .collect();
                                needed_imports.extend(module_decl_imports);

                                // Update the segment with capture info
                                if let Some(seg_mut) = self
                                    .segments
                                    .iter_mut()
                                    .find(|s| s.span.0 == seg_span_0)
                                {
                                    seg_mut.captures =
                                        !capture_result.capture_names.is_empty();
                                    seg_mut.capture_names =
                                        capture_result.capture_names.clone();
                                    seg_mut.needed_imports = needed_imports;
                                }

                                // Serialize the lambda body code for the segment module.
                                // This is the JSX event handler equivalent of what
                                // exit_expression does for regular $() calls.
                                let is_inline = entry_strategy::should_inline(
                                    &self.options.entry_strategy,
                                ) || matches!(
                                    self.options.entry_strategy,
                                    crate::types::EntryStrategy::Hoist
                                );
                                if !is_inline {
                                    let body_code = serialize_jsx_lambda_from_source(
                                        &self.source_code,
                                        span,
                                    );
                                    if !body_code.is_empty() {
                                        self.segment_body_codes
                                            .push((seg_span_0, body_code));
                                    }
                                }

                                // Record replacement info so the JSX transform can
                                // replace the raw lambda with qrl()/inlinedQrl().
                                let seg_info = self.segments.iter()
                                    .find(|s| s.span.0 == seg_span_0)
                                    .cloned();
                                if let Some(seg_info) = seg_info {
                                    self.jsx_event_replacements.insert(
                                        span.0,
                                        JsxEventReplacement {
                                            segment_name: seg_info.name.clone(),
                                            hash: seg_info.hash.clone(),
                                            capture_names: seg_info.capture_names.clone(),
                                            is_inline,
                                            display_name: seg_info.display_name.clone(),
                                            body_span: seg_info.body_span,
                                        },
                                    );
                                }

                                // Pop the attribute name from stack_ctxt
                                self.stack_ctxt.pop();
                            }
                        }
                    }
                }
            }
        }

        self.create_jsx_event_segments_in_children(&element.children);

        // Pop element name from stack_ctxt
        if element_name.is_some() {
            self.stack_ctxt.pop();
        }
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
                    if self.options.transpile_jsx {
                        self.stack_ctxt.push("Fragment".to_string());
                    }
                    self.create_jsx_event_segments_in_children(&frag.children);
                    if self.options.transpile_jsx {
                        self.stack_ctxt.pop();
                    }
                }
                _ => {}
            }
        }
    }

    /// Replace JSX event handler lambda expressions with qrl()/inlinedQrl() calls.
    ///
    /// Walks a JSXElement (recursively including children) and replaces any
    /// `$`-suffixed attribute values whose lambda span matches a registered
    /// segment replacement with the appropriate `qrl(i_hash, "name", [caps])`
    /// or `inlinedQrl(fn, "name", [caps])` call.
    fn replace_jsx_event_handler_values<'a>(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        match expr {
            Expression::JSXElement(el) => {
                self.replace_jsx_element_handlers(el, ctx);
            }
            Expression::JSXFragment(frag) => {
                self.replace_jsx_children_handlers(&mut frag.children, ctx);
            }
            _ => {}
        }
    }

    /// Replace event handler lambdas in a JSXElement and recurse into children.
    /// Also injects q:p/q:ps attributes for iteration variables used by handlers.
    fn replace_jsx_element_handlers<'a>(
        &mut self,
        el: &mut JSXElement<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Collect iteration variables used by handlers on THIS element.
        // We accumulate these during attribute processing and inject q:p/q:ps after.
        let mut element_used_iter_vars: Vec<String> = Vec::new();

        // Process this element's attributes
        for attr_item in &mut el.opening_element.attributes {
            if let JSXAttributeItem::Attribute(attr) = attr_item {
                let attr_name = match &attr.name {
                    JSXAttributeName::Identifier(ident) => ident.name.as_str().to_string(),
                    JSXAttributeName::NamespacedName(ns) => {
                        format!("{}:{}", ns.namespace.name, ns.name.name)
                    }
                };

                if !attr_name.ends_with('$') {
                    continue;
                }

                // Check if the value expression has a registered replacement
                let lambda_span_start = attr.value.as_ref().and_then(|val| {
                    if let JSXAttributeValue::ExpressionContainer(container) = val {
                        match &container.expression {
                            JSXExpression::ArrowFunctionExpression(arrow) => {
                                Some(arrow.span.start)
                            }
                            JSXExpression::FunctionExpression(func) => Some(func.span.start),
                            _ => None,
                        }
                    } else {
                        None
                    }
                });

                if let Some(span_start) = lambda_span_start {
                    // Check if this handler uses iteration variables (for q:p injection)
                    if let Some(used_vars) = self.iter_var_usage_by_handler.get(&span_start) {
                        for var_name in used_vars {
                            if !element_used_iter_vars.contains(var_name) {
                                element_used_iter_vars.push(var_name.clone());
                            }
                        }
                    }

                    if let Some(info) = self.jsx_event_replacements.get(&span_start).cloned() {
                        let dev_meta = self.make_qrl_dev_meta(
                            info.body_span.0,
                            info.body_span.1,
                            &info.display_name,
                        );
                        let replacement = if info.is_inline {
                            // inlinedQrl(handler_expr, "name", [captures])
                            let handler_expr =
                                if let Some(val) = std::mem::take(&mut attr.value) {
                                    match val {
                                        JSXAttributeValue::ExpressionContainer(container) => {
                                            let unboxed = container.unbox();
                                            match unboxed.expression {
                                                JSXExpression::ArrowFunctionExpression(arrow) => {
                                                    Expression::ArrowFunctionExpression(arrow)
                                                }
                                                JSXExpression::FunctionExpression(func) => {
                                                    Expression::FunctionExpression(func)
                                                }
                                                _ => ctx
                                                    .ast
                                                    .expression_identifier(SPAN, "undefined"),
                                            }
                                        }
                                        _ => ctx.ast.expression_identifier(SPAN, "undefined"),
                                    }
                                } else {
                                    ctx.ast.expression_identifier(SPAN, "undefined")
                                };
                            import_rewrite::build_inlined_qrl_call(
                                handler_expr,
                                &info.segment_name,
                                &info.capture_names,
                                dev_meta.as_ref(),
                                ctx,
                            )
                        } else if self.loop_depth > 0 {
                            // Inside a loop: hoist QRL to enclosing function body.
                            // Buffer the QRL components and replace inline with
                            // an identifier reference to the segment name.
                            let import_ident = format!("i_{}", info.hash);
                            // Drop the original lambda value
                            let _ = std::mem::take(&mut attr.value);
                            self.pending_loop_qrl_hoists.push((
                                import_ident,
                                info.segment_name.clone(),
                                info.capture_names.clone(),
                            ));
                            // Replace with identifier reference to the hoisted const
                            let seg_atom = ctx.ast.atom(&info.segment_name);
                            ctx.ast.expression_identifier(SPAN, seg_atom)
                        } else {
                            // qrl(i_hash, "name", [captures]) -- segment strategy
                            let import_ident = format!("i_{}", info.hash);
                            // Drop the original lambda value
                            let _ = std::mem::take(&mut attr.value);
                            import_rewrite::build_qrl_call(
                                &import_ident,
                                &info.segment_name,
                                &info.capture_names,
                                dev_meta.as_ref(),
                                ctx,
                            )
                        };

                        // Wrap the replacement in a JSXExpressionContainer
                        let container = ctx.ast.jsx_expression_container(
                            SPAN,
                            JSXExpression::from(replacement),
                        );
                        attr.value = Some(JSXAttributeValue::ExpressionContainer(
                            ctx.ast.alloc(container),
                        ));
                    }
                }
            }
        }

        // Inject q:p/q:ps attributes for iteration variables used by event handlers
        // on this element. This must happen here (before JSX transform) because by the
        // time transform_jsx_element_inner runs, the handler lambdas are already replaced
        // by QRL identifiers and we can't scan them for iteration variable usage.
        if !element_used_iter_vars.is_empty() {
            if element_used_iter_vars.len() == 1 {
                // q:p={iterVar}
                let var_name = &element_used_iter_vars[0];
                let ident_expr = ctx.ast.expression_identifier(SPAN, ctx.ast.atom(var_name));
                let container = ctx.ast.jsx_expression_container(
                    SPAN,
                    JSXExpression::from(ident_expr),
                );
                let ns_name = ctx.ast.jsx_namespaced_name(
                    SPAN,
                    ctx.ast.jsx_identifier(SPAN, "q"),
                    ctx.ast.jsx_identifier(SPAN, "p"),
                );
                let qp_attr = ctx.ast.jsx_attribute(
                    SPAN,
                    JSXAttributeName::NamespacedName(ctx.ast.alloc(ns_name)),
                    Some(JSXAttributeValue::ExpressionContainer(ctx.ast.alloc(container))),
                );
                el.opening_element.attributes.push(
                    JSXAttributeItem::Attribute(ctx.ast.alloc(qp_attr)),
                );
            } else {
                // q:ps={[var1, var2, ...]}
                let mut elements = ctx.ast.vec();
                for var_name in &element_used_iter_vars {
                    let ident = ctx.ast.expression_identifier(SPAN, ctx.ast.atom(var_name));
                    elements.push(ArrayExpressionElement::from(ident));
                }
                let arr = ctx.ast.expression_array(SPAN, elements);
                let container = ctx.ast.jsx_expression_container(
                    SPAN,
                    JSXExpression::from(arr),
                );
                let ns_name = ctx.ast.jsx_namespaced_name(
                    SPAN,
                    ctx.ast.jsx_identifier(SPAN, "q"),
                    ctx.ast.jsx_identifier(SPAN, "ps"),
                );
                let qps_attr = ctx.ast.jsx_attribute(
                    SPAN,
                    JSXAttributeName::NamespacedName(ctx.ast.alloc(ns_name)),
                    Some(JSXAttributeValue::ExpressionContainer(ctx.ast.alloc(container))),
                );
                el.opening_element.attributes.push(
                    JSXAttributeItem::Attribute(ctx.ast.alloc(qps_attr)),
                );
            }
        }

        // Recurse into children (JSXChild elements are part of the parent AST node,
        // not separate expressions, so they don't get their own exit_expression)
        self.replace_jsx_children_handlers(&mut el.children, ctx);
    }

    /// Recurse into JSX children to replace event handler lambdas.
    fn replace_jsx_children_handlers<'a>(
        &mut self,
        children: &mut oxc::allocator::Vec<'a, JSXChild<'a>>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        for child in children.iter_mut() {
            match child {
                JSXChild::Element(child_el) => {
                    self.replace_jsx_element_handlers(child_el, ctx);
                }
                JSXChild::Fragment(frag) => {
                    self.replace_jsx_children_handlers(&mut frag.children, ctx);
                }
                _ => {}
            }
        }
    }

    /// When transpile_jsx is false, rename $-suffixed event handler attributes
    /// on native JSX elements to their HTML form (e.g., onClick$ -> q-e:click).
    /// This runs AFTER segment extraction and QRL value replacement, so it
    /// doesn't interfere with those processes.
    fn rename_jsx_event_attrs<'a>(
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        match expr {
            Expression::JSXElement(el) => {
                Self::rename_jsx_element_event_attrs(el, ctx);
            }
            Expression::JSXFragment(frag) => {
                Self::rename_jsx_children_event_attrs(&mut frag.children, ctx);
            }
            _ => {}
        }
    }

    /// Rename event handler attributes on a single JSXElement and recurse
    /// into its children.
    fn rename_jsx_element_event_attrs<'a>(
        el: &mut JSXElement<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Determine if this is a native element (lowercase first char)
        let is_native = match &el.opening_element.name {
            JSXElementName::Identifier(ident) => {
                ident.name.as_str().chars().next().is_some_and(|c| c.is_lowercase())
            }
            JSXElementName::IdentifierReference(ident) => {
                ident.name.as_str().chars().next().is_some_and(|c| c.is_lowercase())
            }
            _ => false,
        };

        if is_native {
            for attr_item in &mut el.opening_element.attributes {
                if let JSXAttributeItem::Attribute(attr) = attr_item {
                    if let JSXAttributeName::Identifier(ident) = &attr.name {
                        if let Some(html_attr) = jsx_event_to_html_attribute(ident.name.as_str()) {
                            if let Some(colon_pos) = html_attr.find(':') {
                                let ns_part = &html_attr[..colon_pos];
                                let name_part = &html_attr[colon_pos + 1..];
                                let ns_atom = ctx.ast.atom(ns_part);
                                let name_atom = ctx.ast.atom(name_part);
                                let ns_ident = JSXIdentifier { span: SPAN, name: ns_atom };
                                let name_ident = JSXIdentifier { span: SPAN, name: name_atom };
                                let ns_name = JSXNamespacedName {
                                    span: SPAN,
                                    namespace: ns_ident,
                                    name: name_ident,
                                };
                                attr.name = JSXAttributeName::NamespacedName(
                                    ctx.ast.alloc(ns_name),
                                );
                            }
                        }
                    }
                }
            }
        }

        // Recurse into children
        Self::rename_jsx_children_event_attrs(&mut el.children, ctx);
    }

    /// Recurse into JSX children to rename event handler attributes.
    fn rename_jsx_children_event_attrs<'a>(
        children: &mut oxc::allocator::Vec<'a, JSXChild<'a>>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        for child in children.iter_mut() {
            match child {
                JSXChild::Element(child_el) => {
                    Self::rename_jsx_element_event_attrs(child_el, ctx);
                }
                JSXChild::Fragment(frag) => {
                    Self::rename_jsx_children_event_attrs(&mut frag.children, ctx);
                }
                _ => {}
            }
        }
    }
}

impl<'a> Traverse<'a, ()> for QwikTransform {
    fn enter_program(&mut self, program: &mut Program<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        // Store comments from the parsed program for use in comment-preserving codegen.
        // Comment is Copy, so we clone each one into a standard Vec.
        self.source_comments = program.comments.iter().copied().collect();
    }

    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Check if this is an array iteration method call (.map, .filter, etc.)
        if let Expression::StaticMemberExpression(member) = &call.callee {
            let method_name = member.property.name.as_str();
            if matches!(
                method_name,
                "map"
                    | "filter"
                    | "forEach"
                    | "flatMap"
                    | "some"
                    | "every"
                    | "find"
                    | "findIndex"
                    | "reduce"
                    | "reduceRight"
            ) {
                self.loop_depth += 1;
                self.in_callback_depth += 1;
                // Extract callback parameters as iteration variables
                if let Some(first_arg) = call.arguments.first() {
                    let iteration_vars = extract_callback_params(first_arg);
                    self.iteration_var_stack.push(iteration_vars);
                } else {
                    self.iteration_var_stack.push(Vec::new());
                }
            }
        }

        // Push callee name to stack_ctxt, mirroring SWC's fold_call_expr.
        // SWC pushes the callee ident.sym for marker functions and all other
        // ident callees, but NOT for:
        // - Raw $() calls (handle_qsegment returns early without push)
        // - sync$(), inlinedQrl, _fnSignal, jsx functions
        self.call_expr_ctxt_depths.push(self.stack_ctxt.len());

        let kind = self.is_dollar_call(call);

        // Only push callee name when it's NOT a raw $() call.
        // SWC's handle_qsegment returns before the push at line 3186/3226.
        if let Expression::Identifier(ident) = &call.callee {
            let is_raw_dollar = matches!(kind, Some(DollarCallKind::RawDollar));
            if !is_raw_dollar {
                self.stack_ctxt.push(ident.name.as_str().to_string());
            }
        }

        let Some(kind) = kind else {
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
                self.invalid_decl_stack.push(HashSet::new());
                return;
            }
            if name == "component$" || name == "useResource$" {
                if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first() {
                    // Build set of import names for const-checking default values
                    let import_names: HashSet<String> = self
                        .collected
                        .module_imports
                        .iter()
                        .flat_map(|imp| imp.specifiers.iter().cloned())
                        .collect();
                    let mut info = props_destructuring::analyze_props_destructuring(
                        &arrow.params,
                        &import_names,
                    );
                    if info.needs_transform {
                        if info.rest_name.is_some() {
                            self.import_tracker.needs_rest_props = true;
                        }
                        self.active_props_info = Some(info);
                    } else if name == "component$" {
                        // Non-destructured props param body destructuring detection
                        // only applies to component$ (not useResource$ or other hooks).
                        if let Some(ref param_name) = info.props_param_name {
                            let body_destr = props_destructuring::detect_body_destructuring(
                                &arrow.body.statements,
                                param_name,
                            );
                            if let Some(ref body_info) = body_destr {
                                info.prop_keys = body_info.prop_keys.clone();
                                info.rest_name = body_info.rest_name.clone();
                            }
                            self.active_props_info = Some(info);
                        }
                    }
                }
            }
        }

        self.capture_stack.push((Vec::new(), HashSet::new()));
        self.invalid_decl_stack.push(HashSet::new());

        if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first() {
            for param in &arrow.params.items {
                self.collect_binding_pattern_names(&param.pattern);
            }
            if let Some(rest) = &arrow.params.rest {
                self.collect_binding_pattern_names(&rest.rest.argument);
            }
        }

        // record_segment reads stack_ctxt and pushes to segment_stack + dollar_call_stack
        let _segment = self.record_segment(call, &kind);

        if let DollarCallKind::Named(ref name) = kind {
            if self.should_strip_ctx_name(name) {
                self.stripped_segments.insert(call.span.start);
            }
        }

        self.pending_dollar_calls.insert(call.span.start);
    }

    fn exit_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Clean up iteration tracking for array methods
        if let Expression::StaticMemberExpression(member) = &call.callee {
            let method_name = member.property.name.as_str();
            if matches!(
                method_name,
                "map"
                    | "filter"
                    | "forEach"
                    | "flatMap"
                    | "some"
                    | "every"
                    | "find"
                    | "findIndex"
                    | "reduce"
                    | "reduceRight"
            ) {
                self.iteration_var_stack.pop();
                self.loop_depth -= 1;
                self.in_callback_depth = self.in_callback_depth.saturating_sub(1);
            }
        }

        // Pop callee name from stack_ctxt
        if let Some(depth) = self.call_expr_ctxt_depths.pop() {
            self.stack_ctxt.truncate(depth);
        }
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

    fn enter_variable_declaration(
        &mut self,
        decl: &mut VariableDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Populate const_bindings from `const` declarations.
        // This mirrors SWC's ConstCollector which tracks const bindings for
        // scope-aware JSX prop/children immutability classification.
        if decl.kind == VariableDeclarationKind::Const {
            for declarator in &decl.declarations {
                collect_const_binding_names(
                    &declarator.id,
                    &mut self.import_tracker.const_bindings,
                );
            }
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

        // Push variable name to stack_ctxt (mirrors SWC's fold_var_declarator)
        self.var_decl_ctxt_depths.push(self.stack_ctxt.len());
        if let BindingPattern::BindingIdentifier(ref ident) = declarator.id {
            self.stack_ctxt.push(ident.name.as_str().to_string());
        }
    }

    fn exit_variable_declarator(
        &mut self,
        _declarator: &mut VariableDeclarator<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(depth) = self.var_decl_ctxt_depths.pop() {
            self.stack_ctxt.truncate(depth);
        }
    }

    fn enter_function(&mut self, func: &mut Function<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        // Push function name to stack_ctxt for named function declarations
        // (mirrors SWC's fold_fn_decl)
        self.fn_decl_ctxt_depths.push(self.stack_ctxt.len());
        if let Some(ref id) = func.id {
            let name = id.name.as_str().to_string();
            self.stack_ctxt.push(name.clone());
            // Track function declarations as "invalid" for capture purposes.
            // SWC treats IdentType::Fn as invalid_decl: they're NOT captured
            // but emit C02 diagnostics instead.
            if let Some(frame) = self.invalid_decl_stack.last_mut() {
                frame.insert(name);
            }
        }
        // Save and set root_jsx_mode for function bodies (mirrors SWC fold_fn_expr)
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_function(&mut self, func: &mut Function<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        if let Some(depth) = self.fn_decl_ctxt_depths.pop() {
            self.stack_ctxt.truncate(depth);
        }
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
        // Flush pending QRL hoists when exiting a function that's not inside a loop.
        // This prepends hoisted const declarations to the function body.
        if self.loop_depth == 0 && !self.pending_loop_qrl_hoists.is_empty() {
            if let Some(ref mut body) = func.body {
                let hoists = std::mem::take(&mut self.pending_loop_qrl_hoists);
                flush_qrl_hoists_to_body(&mut body.statements, &hoists, ctx);
            }
        }
    }

    fn enter_class(&mut self, class: &mut Class<'a>, _ctx: &mut TraverseCtx<'a, ()>) {
        // Track class declarations as "invalid" for capture purposes.
        // SWC treats IdentType::Class as invalid_decl: they're NOT captured
        // but emit C02 diagnostics instead (matching function declarations).
        if let Some(ref id) = class.id {
            if let Some(frame) = self.invalid_decl_stack.last_mut() {
                frame.insert(id.name.as_str().to_string());
            }
        }
    }

    fn enter_arrow_function_expression(
        &mut self,
        _arrow: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_arrow_function_expression(
        &mut self,
        arrow: &mut ArrowFunctionExpression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Flush pending QRL hoists when exiting an arrow that's not inside a loop.
        if self.loop_depth == 0 && !self.pending_loop_qrl_hoists.is_empty() {
            let hoists = std::mem::take(&mut self.pending_loop_qrl_hoists);
            flush_qrl_hoists_to_body(&mut arrow.body.statements, &hoists, ctx);
        }
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_export_default_declaration(
        &mut self,
        _decl: &mut ExportDefaultDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Push file stem (or folder name for index files) to stack_ctxt
        // (mirrors SWC's fold_export_default_expr)
        self.export_default_ctxt_depths
            .push(self.stack_ctxt.len());

        let mut file_stem = self.file_stem();

        if file_stem == "index" {
            // Use folder name instead (mirrors SWC's rel_dir.file_name())
            // Handle both / and \ path separators
            let dir_part = {
                let last_sep = self
                    .filename
                    .rfind(|c: char| c == '/' || c == '\\')
                    .unwrap_or(0);
                if last_sep > 0 {
                    &self.filename[..last_sep]
                } else {
                    ""
                }
            };
            if !dir_part.is_empty() {
                let folder = dir_part
                    .rsplit(|c: char| c == '/' || c == '\\')
                    .next()
                    .unwrap_or(dir_part);
                if !folder.is_empty() {
                    file_stem = folder.to_string();
                }
            }
        }

        self.stack_ctxt.push(file_stem);
    }

    fn exit_export_default_declaration(
        &mut self,
        _decl: &mut ExportDefaultDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(depth) = self.export_default_ctxt_depths.pop() {
            self.stack_ctxt.truncate(depth);
        }
    }

    fn enter_jsx_element(
        &mut self,
        el: &mut JSXElement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Push element tag name to stack_ctxt (mirrors SWC's fold_jsx_element)
        match &el.opening_element.name {
            JSXElementName::Identifier(ident) => {
                let is_native = ident.name.as_str().chars().next().is_some_and(|c| c.is_lowercase());
                self.stack_ctxt.push(ident.name.as_str().to_string());
                self.jsx_element_is_native.push(is_native);
            }
            JSXElementName::IdentifierReference(ident) => {
                // Component JSX elements (capital first letter) are IdentifierReference in OXC
                self.stack_ctxt.push(ident.name.as_str().to_string());
                self.jsx_element_is_native.push(false);
            }
            _ => {
                // For member expressions, namespaced names, etc.
                self.jsx_element_is_native.push(false);
            }
        }
    }

    fn exit_jsx_element(
        &mut self,
        el: &mut JSXElement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if matches!(
            el.opening_element.name,
            JSXElementName::Identifier(_) | JSXElementName::IdentifierReference(_)
        ) {
            self.stack_ctxt.pop();
        }
        self.jsx_element_is_native.pop();
    }

    fn enter_jsx_fragment(
        &mut self,
        _frag: &mut JSXFragment<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Push "Fragment" to stack_ctxt only when JSX is being transpiled.
        // In SWC, fragments become _jsxQ(Fragment, ...) calls after JSX transform,
        // and handle_jsx pushes the first arg "Fragment" to stack_ctxt.
        // When JSX is NOT transpiled, raw <> fragments don't push anything.
        if self.options.transpile_jsx {
            self.stack_ctxt.push("Fragment".to_string());
        }
    }

    fn exit_jsx_fragment(
        &mut self,
        _frag: &mut JSXFragment<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.options.transpile_jsx {
            self.stack_ctxt.pop();
        }
    }

    fn enter_jsx_attribute(
        &mut self,
        attr: &mut JSXAttribute<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // Push attribute name to stack_ctxt (mirrors SWC's fold_jsx_attr)
        let is_native = self.jsx_element_is_native.last().copied().unwrap_or(false);
        match &attr.name {
            JSXAttributeName::Identifier(ident) => {
                if is_native {
                    if let Some(html_attr) = jsx_event_to_html_attribute(ident.name.as_str()) {
                        self.stack_ctxt.push(html_attr);
                    } else {
                        self.stack_ctxt.push(ident.name.as_str().to_string());
                    }
                } else {
                    self.stack_ctxt.push(ident.name.as_str().to_string());
                }
            }
            JSXAttributeName::NamespacedName(ns) => {
                // Push "ns-name" format (e.g., "host-onClick$")
                self.stack_ctxt
                    .push(format!("{}-{}", ns.namespace.name, ns.name.name));
            }
        }
    }

    fn exit_jsx_attribute(
        &mut self,
        _attr: &mut JSXAttribute<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.stack_ctxt.pop();
    }

    // -----------------------------------------------------------------------
    // Loop tracking: enter/exit hooks for for/for-in/for-of/while statements
    // -----------------------------------------------------------------------

    fn enter_for_statement(
        &mut self,
        node: &mut ForStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
        self.loop_depth += 1;
        // Extract iteration variable from init: for (let i = ...) or for (var i = ...)
        let iteration_vars =
            if let Some(ForStatementInit::VariableDeclaration(ref decl)) = node.init {
                decl.declarations
                    .first()
                    .and_then(|d| {
                        if let BindingPattern::BindingIdentifier(ref ident) = d.id {
                            Some(vec![ident.name.to_string()])
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
        self.iteration_var_stack.push(iteration_vars);
    }

    fn exit_for_statement(
        &mut self,
        _node: &mut ForStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.iteration_var_stack.pop();
        self.loop_depth -= 1;
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_for_in_statement(
        &mut self,
        node: &mut ForInStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
        self.loop_depth += 1;
        let iteration_vars = match &node.left {
            ForStatementLeft::VariableDeclaration(decl) => decl
                .declarations
                .first()
                .and_then(|d| {
                    if let BindingPattern::BindingIdentifier(ref ident) = d.id {
                        Some(vec![ident.name.to_string()])
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
            ForStatementLeft::AssignmentTargetIdentifier(ident) => {
                vec![ident.name.to_string()]
            }
            _ => Vec::new(),
        };
        self.iteration_var_stack.push(iteration_vars);
    }

    fn exit_for_in_statement(
        &mut self,
        _node: &mut ForInStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.iteration_var_stack.pop();
        self.loop_depth -= 1;
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_for_of_statement(
        &mut self,
        node: &mut ForOfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
        self.loop_depth += 1;
        let iteration_vars = match &node.left {
            ForStatementLeft::VariableDeclaration(decl) => decl
                .declarations
                .first()
                .and_then(|d| {
                    if let BindingPattern::BindingIdentifier(ref ident) = d.id {
                        Some(vec![ident.name.to_string()])
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
            ForStatementLeft::AssignmentTargetIdentifier(ident) => {
                vec![ident.name.to_string()]
            }
            _ => Vec::new(),
        };
        self.iteration_var_stack.push(iteration_vars);
    }

    fn exit_for_of_statement(
        &mut self,
        _node: &mut ForOfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.iteration_var_stack.pop();
        self.loop_depth -= 1;
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_while_statement(
        &mut self,
        node: &mut WhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
        self.loop_depth += 1;
        // Extract iteration variable from test: while (i < ...) => extract "i"
        let iteration_vars = match &node.test {
            Expression::BinaryExpression(bin) => {
                if let Expression::Identifier(ref ident) = bin.left {
                    vec![ident.name.to_string()]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        };
        self.iteration_var_stack.push(iteration_vars);
    }

    fn exit_while_statement(
        &mut self,
        _node: &mut WhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.iteration_var_stack.pop();
        self.loop_depth -= 1;
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_do_while_statement(
        &mut self,
        _stmt: &mut DoWhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_do_while_statement(
        &mut self,
        _stmt: &mut DoWhileStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_if_statement(
        &mut self,
        _stmt: &mut IfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_if_statement(
        &mut self,
        _stmt: &mut IfStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_block_statement(
        &mut self,
        _stmt: &mut BlockStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_block_statement(
        &mut self,
        _stmt: &mut BlockStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
        }
    }

    fn enter_return_statement(
        &mut self,
        _stmt: &mut ReturnStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        self.root_jsx_mode_stack.push(self.root_jsx_mode);
        self.root_jsx_mode = true;
    }

    fn exit_return_statement(
        &mut self,
        _stmt: &mut ReturnStatement<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Some(prev) = self.root_jsx_mode_stack.pop() {
            self.root_jsx_mode = prev;
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
                    if self.options.transpile_jsx {
                        self.stack_ctxt.push("Fragment".to_string());
                    }
                    self.create_jsx_event_segments_in_children(&frag.children);
                    if self.options.transpile_jsx {
                        self.stack_ctxt.pop();
                    }
                }
            }
            _ => {}
        }

        // Replace JSX event handler lambda expressions with qrl()/inlinedQrl() calls
        // BEFORE the JSX transform runs, so the _jsxSorted output has the correct values.
        if !self.jsx_event_replacements.is_empty() {
            self.replace_jsx_event_handler_values(expr, ctx);
        }

        // When transpile_jsx is false, rename $-suffixed event handler attributes
        // to their HTML form (e.g., onClick$ -> q-e:click as NamespacedName).
        // This MUST run AFTER create_jsx_event_segments_recursive and
        // replace_jsx_event_handler_values, which rely on the $ suffix to identify
        // event handler attributes for segment extraction and QRL wrapping.
        // When transpile_jsx is true, the JSX transform module handles this
        // during _jsxSorted() call construction.
        if !self.options.transpile_jsx {
            Self::rename_jsx_event_attrs(expr, ctx);
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
            let props_param_name: Option<String> = self.active_props_info.as_ref()
                .and_then(|info| info.props_param_name.clone());
            let props_param_ref = props_param_name.as_deref();

            // Take hoisted_function_stmts out to avoid borrow conflict with &mut self
            let mut hoisted_stmts = std::mem::take(&mut self.hoisted_function_stmts);
            let module_imports = &self.collected.module_imports;
            let loop_depth = self.loop_depth;
            let iteration_vars = self.current_iteration_vars();

            let root_mode = self.root_jsx_mode;
            let key_prefix = self.jsx_key_prefix.clone();

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
                            loop_depth,
                            &iteration_vars,
                            props_param_ref,
                            root_mode,
                            &key_prefix,
                        );
                        *expr = result;
                    }
                    self.root_jsx_mode = false;
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
                            loop_depth,
                            &iteration_vars,
                            props_param_ref,
                            root_mode,
                            &key_prefix,
                        );
                        *expr = result;
                    }
                    self.root_jsx_mode = false;
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
                self.invalid_decl_stack.pop();

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

            // Pop segment_stack and dollar_call_stack (pushed in record_segment)
            self.segment_stack.pop();
            self.dollar_call_stack.pop();

            let is_component_exit =
                matches!(&kind, DollarCallKind::Named(name) if name == "component$");
            let is_props_rewrite_exit =
                matches!(&kind, DollarCallKind::Named(name) if name == "component$" || name == "useResource$");
            let props_info = if is_props_rewrite_exit {
                self.active_props_info.take()
            } else {
                None
            };
            if let Some(ref info) = props_info {
                if info.needs_transform {
                    // Standard destructured props: replace parameter, rewrite references
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
                                &info.prop_defaults,
                                ctx,
                            );
                        }
                    }
                } else if is_component_exit {
                    if let Some(ref param_name) = info.props_param_name {
                    // Non-destructured props param (e.g., `(props) =>`).
                    // Check for body destructuring: `const { "bind:value": bindValue } = props;`
                    // This only applies to component$ (not useResource$ or other hooks).
                    if let Some(Argument::ArrowFunctionExpression(arrow)) = call.arguments.first_mut() {
                        let body_destr = props_destructuring::detect_body_destructuring(
                            &arrow.body.statements,
                            param_name,
                        );

                        if let Some(body_info) = body_destr {
                            // Remove the destructuring statement
                            let removed_stmt = arrow.body.statements.remove(body_info.stmt_index);
                            let _ = removed_stmt;

                            // If rest pattern: insert `const rest = _restProps(props, [...])`
                            if let Some(ref rest_name) = body_info.rest_name {
                                self.import_tracker.needs_rest_props = true;
                                let excluded_keys: Vec<String> =
                                    body_info.prop_keys.iter().map(|(key, _)| key.clone()).collect();
                                let rest_stmt = props_destructuring::build_rest_props_declaration(
                                    rest_name,
                                    param_name,
                                    &excluded_keys,
                                    ctx,
                                );
                                arrow.body.statements.insert(body_info.stmt_index, rest_stmt);
                            }

                            // Build a prop_map for rewriting: (local_alias -> original_key)
                            // In non-JSX contexts, replace alias with props["key"] (computed member)
                            // The JSX contexts are handled by detect_signal_wrap via props_param_name
                            let prop_map: Vec<(String, String)> = body_info
                                .prop_keys
                                .iter()
                                .map(|(key, local)| (local.clone(), key.clone()))
                                .collect();

                            if !prop_map.is_empty() {
                                // Rewrite local alias references in non-JSX body statements.
                                // For body destructuring, replace `bindValue` with `props["bind:value"]`
                                // in non-JSX positions (like useSignal(bindValue) -> useSignal(props["bind:value"]))
                                // JSX positions are handled by _wrapProp via detect_signal_wrap.
                                rewrite_body_destr_references(
                                    &mut arrow.body.statements,
                                    &prop_map,
                                    param_name,
                                    ctx,
                                );
                            }

                            // Handle `const test = useSignal(...)` where `test` is a destructured prop key:
                            // Strip the `const test =` to just `useSignal(...)` as expression statement.
                            // This is for destructure_args_colon_props3 where `test` was a prop key.
                            let non_rest_aliases: std::collections::HashSet<String> = body_info
                                .prop_keys
                                .iter()
                                .map(|(_, local)| local.clone())
                                .collect();
                            strip_prop_alias_bindings(&mut arrow.body.statements, &non_rest_aliases, ctx);
                        }
                    }
                }
                }

                if let Some(seg) = self
                    .segments
                    .iter_mut()
                    .find(|s| s.span.0 == call.span.start && s.span.1 == call.span.end)
                {
                    seg.param_names = vec![info.raw_props_name.clone()];
                }

                // Reclassify child segment captures: replace prop alias captures
                // with _rawProps. Only for component$ -- useResource$ and other hooks
                // don't have user props that child segments would capture.
                if is_component_exit {
                    let local_aliases: HashSet<String> = info
                        .prop_keys
                        .iter()
                        .map(|(_, local)| local.clone())
                        .collect();

                    let component_span = (call.span.start, call.span.end);
                    // Use segment name (with hash) for parent matching since
                    // seg.parent now stores segment_name, not display_name.
                    let component_segment_name = self
                        .segments
                        .iter()
                        .find(|s| s.span == component_span)
                        .map(|s| s.name.clone());

                    if let Some(parent_name) = component_segment_name {
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
            }

            let (body_ident_refs, body_local_decls) = self.capture_stack.pop().unwrap_or_default();
            self.invalid_decl_stack.pop();

            // enclosing function scope.
            let is_top_level_dollar_call = self.capture_stack.is_empty();

            let capture_result =
                collector::compute_captures(&body_ident_refs, &body_local_decls, &self.collected);

            // Emit C02 diagnostics for function/class references captured inside $() scope.
            // SWC partitions declarations into (decl_collect, invalid_decl) where invalid_decl
            // contains function/class declarations. These are NOT captured but emit errors.
            // We check parent scope's invalid_decl_stack to find references to fn/class decls.
            let parent_invalid_decls: HashSet<String> = self.invalid_decl_stack
                .iter()
                .flat_map(|s| s.iter().cloned())
                .collect();
            let mut filtered_captures = Vec::new();
            for name in &capture_result.capture_names {
                if parent_invalid_decls.contains(name) {
                    // Emit C02 diagnostic matching SWC's "FunctionReference" error
                    self.diagnostics.push(crate::types::Diagnostic {
                        scope: "optimizer".to_string(),
                        category: crate::types::DiagnosticCategory::Error,
                        code: Some("C02".to_string()),
                        file: self.filename.clone(),
                        message: format!(
                            "Reference to identifier '{}' can not be used inside a Qrl($) scope because it's a function",
                            name
                        ),
                        highlights: None,
                        suggestions: None,
                    });
                } else {
                    filtered_captures.push(name.clone());
                }
            }
            let capture_result = collector::CaptureAnalysisResult {
                capture_names: filtered_captures,
                reemitted_imports: capture_result.reemitted_imports,
                diagnostics: capture_result.diagnostics,
            };

            // Reclassify module-level declarations from captures to needed_imports
            // (self-imports from the parent module). This matches SWC behavior where
            // module-level declarations are re-imported rather than captured.
            let (capture_result, module_decl_imports) =
                self.reclassify_module_level_decl_captures(capture_result);

            // Convert reemitted_imports into ImportInfo entries for the segment's needed_imports.
            // These imports will be emitted in the segment module by code_move.rs.
            let mut needed_imports: Vec<crate::types::ImportInfo> = capture_result
                .reemitted_imports
                .iter()
                .map(|ri| {
                    let mut aliases = std::collections::HashMap::new();
                    if let Some(ref imported) = ri.imported_name {
                        aliases.insert(ri.local_name.clone(), imported.clone());
                    }
                    crate::types::ImportInfo {
                        source: ri.source.clone(),
                        specifiers: vec![ri.local_name.clone()],
                        specifier_kinds: vec![ri.kind.clone()],
                        specifier_aliases: aliases,
                        is_qwik_core: false,
                        span: (0, 0),
                    }
                })
                .collect();
            needed_imports.extend(module_decl_imports);

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
                // Store needed imports for the segment module (applies to all segments,
                // both top-level and nested)
                seg.needed_imports = needed_imports;
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

                    if let Some(expr_val) = body_expr {
                        let body_code = codegen_expression_with_comments(
                            expr_val,
                            &self.source_code,
                            &self.source_comments,
                            ctx,
                        );
                        self.segment_body_codes.push((call.span.start, body_code));
                    }
                }
            }

            let replacement = if is_stripped {
                self.import_tracker.needs_noop_qrl = true;
                let noop_meta = self.make_noop_dev_meta(&segment_info.display_name);
                import_rewrite::build_noop_qrl_call(
                    &segment_info.name,
                    &capture_result.capture_names,
                    noop_meta.as_ref(),
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

                let dev_meta = self.make_qrl_dev_meta(
                    segment_info.body_span.0,
                    segment_info.body_span.1,
                    &segment_info.display_name,
                );
                import_rewrite::build_inlined_qrl_call(
                    body_as_expr,
                    &segment_info.name,
                    &segment_info.capture_names,
                    dev_meta.as_ref(),
                    ctx,
                )
            } else {
                let import_ident = format!("i_{}", segment_info.hash);
                let dev_meta = self.make_qrl_dev_meta(
                    segment_info.body_span.0,
                    segment_info.body_span.1,
                    &segment_info.display_name,
                );
                import_rewrite::build_qrl_call(
                    &import_ident,
                    &segment_info.name,
                    &segment_info.capture_names,
                    dev_meta.as_ref(),
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
                    let mut args = ctx.ast.vec_with_capacity(call.arguments.len().max(1));
                    args.push(Argument::from(replacement));
                    // Pass through additional arguments (e.g., { tagName: "my-foo" } for component$)
                    for i in 1..call.arguments.len() {
                        let placeholder = Argument::from(
                            ctx.ast.expression_identifier(SPAN, "undefined"),
                        );
                        let extra_arg = std::mem::replace(&mut call.arguments[i], placeholder);
                        args.push(extra_arg);
                    }
                    // Only component$ is tree-shakeable and gets PURE annotation.
                    // Side-effectful wrappers (useStylesQrl, useTaskQrl, etc.) must NOT
                    // have PURE because bundlers would incorrectly remove them.
                    if is_tree_shakeable_dollar_call(name) {
                        ctx.ast.expression_call_with_pure(
                            SPAN,
                            qrl_callee,
                            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
                            args,
                            false,
                            true,
                        )
                    } else {
                        ctx.ast.expression_call(
                            SPAN,
                            qrl_callee,
                            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
                            args,
                            false,
                        )
                    }
                }
            };

            *expr = final_expr;
        }
    }

    fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a, ()>) {
        let core_module = &self.options.core_module;

        // Determine if we're in inline/hoist mode (segments stay in entry module)
        let is_inline = entry_strategy::should_inline(&self.options.entry_strategy)
            || matches!(
                self.options.entry_strategy,
                crate::types::EntryStrategy::Hoist
            );

        // ---------------------------------------------------------------
        // Phase 1: Swap out old body and separate into categories
        // ---------------------------------------------------------------
        let mut old_body = ctx.ast.vec();
        std::mem::swap(&mut program.body, &mut old_body);

        // Separate old_body into: Qwik-core imports (to strip), non-Qwik imports,
        // and non-import stmts
        let mut non_qwik_imports: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();
        let mut non_import_stmts: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();

        for stmt in old_body {
            if let Statement::ImportDeclaration(ref import_decl) = stmt {
                let source = import_decl.source.value.as_str();
                let is_qwik_core = self
                    .collected
                    .module_imports
                    .iter()
                    .any(|i| i.source == source && i.is_qwik_core);
                if is_qwik_core {
                    continue; // Skip -- Qwik core imports are re-emitted as synthetic imports
                }
                non_qwik_imports.push(stmt);
            } else {
                non_import_stmts.push(stmt);
            }
        }

        // ---------------------------------------------------------------
        // Phase 1b: Simplify unused pure-annotated variable declarations
        // ---------------------------------------------------------------
        // When MinifyMode::Simplify, convert non-exported variable declarations
        // whose init is a PURE-annotated call expression AND whose binding is
        // unreferenced in the module body into expression statements.
        // This matches SWC's tree-shaker/DCE behavior:
        //   `const App = /* @__PURE__ */ componentQrl(...)` -> `componentQrl(...);`
        //   `const Header = /* @__PURE__ */ qrl(...)` -> `qrl(...);`
        // But non-pure calls are kept:
        //   `const renderHeader = component(qrl(...))` -> preserved as-is
        if matches!(self.options.minify, crate::types::MinifyMode::Simplify) {
            simplify_unused_pure_var_decls(&mut non_import_stmts, ctx);
        }

        // ---------------------------------------------------------------
        // Phase 2: Collect referenced identifiers from the entry module body
        // ---------------------------------------------------------------
        // Scan non-import statements to find which identifiers are actually
        // referenced in the entry module. This is used to filter synthetic
        // framework imports and non-Qwik user imports that are only needed
        // by segment bodies (which become separate files).
        let referenced_idents = collect_referenced_idents(&non_import_stmts);

        // ---------------------------------------------------------------
        // Phase 3: Build synthetic framework import statements
        // ---------------------------------------------------------------
        // These are framework imports from import_tracker (componentQrl, qrl,
        // _jsxSorted, etc.) Each entry is (local_name, statement).

        // 3a: Qrl-suffixed imports (componentQrl, useStylesQrl, etc.)
        // Skip locally-defined Qrl functions (they're module-level exports, not framework imports).
        let mut synthetic_imports: std::vec::Vec<(&str, Statement<'a>)> = std::vec::Vec::new();
        for qrl_name in &self.import_tracker.qrl_imports {
            if self.collected.module_level_decls.contains(qrl_name.as_str()) {
                continue; // Locally-defined Qrl function, not a framework import
            }
            let stmt = import_rewrite::build_named_import(qrl_name, core_module, ctx);
            synthetic_imports.push((qrl_name.as_str(), stmt));
        }

        // For inline strategy, add segment-level Qrl-suffixed imports too
        if is_inline {
            let mut emitted_qrl_names: std::collections::HashSet<String> =
                self.import_tracker.qrl_imports.iter().cloned().collect();
            for (_parent_name, qrl_name) in &self.pending_segment_qrl_imports {
                if self.collected.module_level_decls.contains(qrl_name.as_str()) {
                    continue; // Locally-defined Qrl function
                }
                if emitted_qrl_names.insert(qrl_name.clone()) {
                    let stmt =
                        import_rewrite::build_named_import(qrl_name, core_module, ctx);
                    synthetic_imports.push(("_segment_qrl", stmt));
                }
            }
        }

        // 3b: Other framework imports (qrl/qrlDEV, inlinedQrl/inlinedQrlDEV, _captures, etc.)
        let is_dev = self.is_dev_mode();
        if self.import_tracker.needs_qrl {
            let name = if is_dev { "qrlDEV" } else { "qrl" };
            let stmt = import_rewrite::build_named_import(name, core_module, ctx);
            synthetic_imports.push((name, stmt));
        }
        if self.import_tracker.needs_inlined_qrl {
            let name = if is_dev { "inlinedQrlDEV" } else { "inlinedQrl" };
            let stmt = import_rewrite::build_named_import(name, core_module, ctx);
            synthetic_imports.push((name, stmt));
        }
        if self.import_tracker.needs_captures {
            let stmt = import_rewrite::build_named_import("_captures", core_module, ctx);
            synthetic_imports.push(("_captures", stmt));
        }
        if self.import_tracker.needs_rest_props {
            let stmt = import_rewrite::build_named_import("_restProps", core_module, ctx);
            synthetic_imports.push(("_restProps", stmt));
        }
        if self.import_tracker.needs_jsx_sorted {
            if let Some(ref source) = self.import_tracker.custom_jsx_source {
                let jsx_runtime_source = format!("{}/jsx-runtime", source);
                let stmt = import_rewrite::build_aliased_import(
                    "jsx",
                    "_jsx",
                    &jsx_runtime_source,
                    ctx,
                );
                synthetic_imports.push(("_jsx", stmt));
            } else {
                let stmt = import_rewrite::build_named_import("_jsxSorted", core_module, ctx);
                synthetic_imports.push(("_jsxSorted", stmt));
            }
        }
        if self.import_tracker.needs_get_var_props {
            let stmt = import_rewrite::build_named_import("_getVarProps", core_module, ctx);
            synthetic_imports.push(("_getVarProps", stmt));
        }
        if self.import_tracker.needs_get_const_props {
            let stmt = import_rewrite::build_named_import("_getConstProps", core_module, ctx);
            synthetic_imports.push(("_getConstProps", stmt));
        }
        if self.import_tracker.needs_jsx_split {
            let stmt = import_rewrite::build_named_import("_jsxSplit", core_module, ctx);
            synthetic_imports.push(("_jsxSplit", stmt));
        }
        if self.import_tracker.needs_wrap_prop {
            let stmt = import_rewrite::build_named_import("_wrapProp", core_module, ctx);
            synthetic_imports.push(("_wrapProp", stmt));
        }
        if self.import_tracker.needs_fn_signal {
            let stmt = import_rewrite::build_named_import("_fnSignal", core_module, ctx);
            synthetic_imports.push(("_fnSignal", stmt));
        }
        if self.import_tracker.needs_val {
            let stmt = import_rewrite::build_named_import("_val", core_module, ctx);
            synthetic_imports.push(("_val", stmt));
        }
        if self.import_tracker.needs_chk {
            let stmt = import_rewrite::build_named_import("_chk", core_module, ctx);
            synthetic_imports.push(("_chk", stmt));
        }
        if self.import_tracker.needs_noop_qrl {
            let name = if is_dev { "_noopQrlDEV" } else { "_noopQrl" };
            let stmt = import_rewrite::build_named_import(name, core_module, ctx);
            synthetic_imports.push((name, stmt));
        }
        if self.import_tracker.needs_qrl_sync {
            let stmt = import_rewrite::build_named_import("_qrlSync", core_module, ctx);
            synthetic_imports.push(("_qrlSync", stmt));
        }
        if self.import_tracker.needs_fragment {
            let stmt = import_rewrite::build_aliased_import(
                "Fragment",
                "_Fragment",
                "@qwik.dev/core/jsx-runtime",
                ctx,
            );
            synthetic_imports.push(("_Fragment", stmt));
        }

        // 3c: Build lazy import declarations (const i_XXX = () => import(...))
        // Sort by import path to match SWC's BTreeMap ordering (alphabetical by key).
        // Filter to only include lazy imports whose identifier is actually referenced
        // in the entry module body. With QRL hoisting, event handler lazy imports
        // may only be referenced inside segment bodies, not the entry module.
        // Sort lazy imports by hash (first element) to match SWC's BTreeMap<Id> ordering
        // where keys are i_{hash} identifiers. BTreeMap sorts alphabetically by key.
        self.import_tracker.lazy_imports.sort_by(|a, b| a.0.cmp(&b.0));
        let mut lazy_imports: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();
        for (hash, import_path) in &self.import_tracker.lazy_imports {
            let ident_name = format!("i_{}", hash);
            if !referenced_idents.contains(ident_name.as_str()) {
                continue; // Only used in segment bodies -- don't emit in entry module
            }
            let stmt = import_rewrite::build_lazy_import_declaration(hash, import_path, ctx);
            lazy_imports.push(stmt);
        }

        // ---------------------------------------------------------------
        // Phase 4: Filter synthetic imports -- keep only those referenced
        // ---------------------------------------------------------------
        // For segment strategy, many framework imports (e.g., _jsxSorted,
        // _wrapProp, _fnSignal) are only used inside segment bodies that
        // become separate files. Remove them from the entry module.
        let filtered_synthetic: std::vec::Vec<Statement<'a>> = synthetic_imports
            .into_iter()
            .filter(|(name, _stmt)| referenced_idents.contains(*name))
            .map(|(_name, stmt)| stmt)
            .collect();

        // ---------------------------------------------------------------
        // Phase 5: Collect and filter non-dollar Qwik core specifiers
        // ---------------------------------------------------------------
        // These are specifiers like `useStore`, `mutable` that were imported
        // alongside $-suffixed ones from @qwik.dev/core.
        // Only emit those that are actually referenced in entry module code.
        // Merge specifiers from the same source into single import statements.
        const BUILD_CONSTANTS: &[&str] = &["isServer", "isBrowser", "isDev"];

        // Group kept specifiers by source: { source => [(imported, local)] }
        let mut grouped_specifiers:
            std::collections::BTreeMap<String, std::vec::Vec<(String, String)>> =
            std::collections::BTreeMap::new();

        for import_info in &self.collected.module_imports {
            if import_info.is_qwik_core {
                for spec_name in &import_info.specifiers {
                    if self.collected.dollar_imports.contains(spec_name) {
                        continue; // Dollar import: stripped
                    }
                    let imported_name = import_info
                        .specifier_aliases
                        .get(spec_name)
                        .map(|s| s.as_str())
                        .unwrap_or(spec_name.as_str());
                    if BUILD_CONSTANTS.contains(&imported_name) {
                        continue; // Build constant: handled by const_replace
                    }

                    // Only emit if referenced in entry module body
                    if !referenced_idents.contains(spec_name.as_str()) {
                        continue; // Only used in segments -- don't emit in entry module
                    }

                    // Group by source for merging
                    grouped_specifiers
                        .entry(import_info.source.clone())
                        .or_default()
                        .push((imported_name.to_string(), spec_name.clone()));
                }
            }
        }

        // Build merged import statements from grouped specifiers.
        // Specifiers are in original source order (order of import_info.specifiers)
        // which matches SWC's preserved order.
        let mut non_dollar_imports: std::vec::Vec<Statement<'a>> = std::vec::Vec::new();
        for (source, specifiers) in grouped_specifiers {
            let stmt =
                import_rewrite::build_multi_specifier_import(&specifiers, &source, ctx);
            non_dollar_imports.push(stmt);
        }

        // ---------------------------------------------------------------
        // Phase 6: Filter non-Qwik user imports from old_body
        // ---------------------------------------------------------------
        // User imports like `import { mongodb } from "mondodb"` should only
        // appear in the entry module if they're actually referenced in the
        // entry module's non-import code.
        let filtered_non_qwik_imports: std::vec::Vec<Statement<'a>> = non_qwik_imports
            .into_iter()
            .filter(|stmt| {
                if let Statement::ImportDeclaration(import_decl) = stmt {
                    // Check if ANY specifier from this import is referenced
                    if let Some(specifiers) = &import_decl.specifiers {
                        for spec in specifiers {
                            let local_name = match spec {
                                ImportDeclarationSpecifier::ImportSpecifier(s) => {
                                    s.local.name.as_str()
                                }
                                ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                                    s.local.name.as_str()
                                }
                                ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                                    s.local.name.as_str()
                                }
                            };
                            if referenced_idents.contains(local_name) {
                                return true;
                            }
                        }
                        return false; // No specifiers referenced
                    }
                    // Side-effect import (no specifiers): always keep
                    true
                } else {
                    true // Not an import -- keep
                }
            })
            .collect();

        // ---------------------------------------------------------------
        // Phase 7: Assemble body in SWC order
        // ---------------------------------------------------------------
        // SWC order:
        // 1. Synthetic framework imports (componentQrl, qrl, etc.) - filtered
        // 2. Lazy import declarations (const i_XXX = ...)
        // 3. Non-dollar Qwik core specifiers (useStore, mutable, etc.) - merged
        // 4. Original non-Qwik imports (filtered)
        // 5. Original non-import code (exports, declarations, etc.)

        let total_capacity = filtered_synthetic.len()
            + lazy_imports.len()
            + non_dollar_imports.len()
            + filtered_non_qwik_imports.len()
            + non_import_stmts.len();
        let mut new_body = ctx.ast.vec_with_capacity(total_capacity);

        // 1: Filtered synthetic framework imports
        for stmt in filtered_synthetic {
            new_body.push(stmt);
        }

        // 2: Lazy import declarations
        for stmt in lazy_imports {
            new_body.push(stmt);
        }

        // 3: Non-dollar Qwik core specifiers (merged by source)
        for stmt in non_dollar_imports {
            new_body.push(stmt);
        }

        // 4: Filtered non-Qwik user imports
        for stmt in filtered_non_qwik_imports {
            new_body.push(stmt);
        }

        // 5: Non-import code (exports, declarations, expressions)
        for stmt in non_import_stmts {
            new_body.push(stmt);
        }

        program.body = new_body;
    }
}

/// Collect all binding names from a binding pattern into a HashSet.
///
/// Used to populate `const_bindings` from `const` declarations.
/// Handles simple identifiers, object/array destructuring patterns,
/// and assignment patterns (defaults).
fn collect_const_binding_names(pattern: &BindingPattern<'_>, set: &mut HashSet<String>) {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            set.insert(ident.name.as_str().to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_const_binding_names(&prop.value, set);
            }
            if let Some(rest) = &obj.rest {
                collect_const_binding_names(&rest.argument, set);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_const_binding_names(elem, set);
            }
            if let Some(rest) = &arr.rest {
                collect_const_binding_names(&rest.argument, set);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_const_binding_names(&assign.left, set);
        }
    }
}

/// Flush pending QRL hoists to a function body.
///
/// Prepends `const seg_name = /* @__PURE__ */ qrl(import_ident, "seg_name", [captures]);`
/// declarations to the beginning of `statements`. This implements SWC's behavior of
/// hoisting QRL calls from inside loops to the enclosing function body.
fn flush_qrl_hoists_to_body<'a>(
    statements: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    hoists: &[(String, String, Vec<String>)],
    ctx: &mut TraverseCtx<'a, ()>,
) {
    // Find the insertion point: after variable declarations at the top of the body.
    // SWC places hoisted QRL consts after local variable declarations (useStore, useSignal, etc.)
    // but before function declarations, loops, and return statements.
    let mut insert_idx = 0;
    for (i, stmt) in statements.iter().enumerate() {
        match stmt {
            Statement::VariableDeclaration(_) => {
                insert_idx = i + 1;
            }
            _ => break,
        }
    }

    // Build const declarations and insert in forward order.
    // We increment insert_idx after each insertion so the hoists appear in
    // the same order they were pushed (which matches SWC's source-order hoisting).
    let mut idx = insert_idx;
    for (import_ident, seg_name, captures) in hoists.iter() {
        let qrl_call = import_rewrite::build_qrl_call(import_ident, seg_name, captures, None, ctx);

        let binding = ctx
            .ast
            .binding_pattern_binding_identifier(SPAN, ctx.ast.atom(seg_name.as_str()));
        let declarator = ctx.ast.variable_declarator(
            SPAN,
            VariableDeclarationKind::Const,
            binding,
            None::<oxc::allocator::Box<'a, TSTypeAnnotation<'a>>>,
            Some(qrl_call),
            false,
        );
        let declaration = ctx.ast.variable_declaration(
            SPAN,
            VariableDeclarationKind::Const,
            ctx.ast.vec1(declarator),
            false,
        );
        let stmt = Statement::from(Declaration::VariableDeclaration(ctx.ast.alloc(declaration)));
        statements.insert(idx, stmt);
        idx += 1;
    }
}

/// Normalize a symbol name by replacing non-alphanumeric chars with `_`,
/// squashing consecutive underscores, and trimming leading/trailing underscores.
///
/// Port of SWC's `escape_sym` from `crates/swc-optimizer/core/src/transform.rs:3320`.
fn escape_sym(str: &str) -> String {
    str.chars()
        .flat_map(|x| match x {
            'A'..='Z' | 'a'..='z' | '0'..='9' => Some(x),
            _ => Some('_'),
        })
        .fold((String::new(), None), |(mut acc, prev), x| {
            if x == '_' {
                if prev.is_none() {
                    (acc, None)
                } else {
                    (acc, Some('_'))
                }
            } else {
                if prev == Some('_') {
                    acc.push('_');
                }
                acc.push(x);
                (acc, Some(x))
            }
        })
        .0
}

/// Simplify unused variable declarations in the module body.
///
/// Simplified tree-shaker: drop unused pure-annotated variable declarations.
///
/// When a non-exported VariableDeclaration has a single declarator whose
/// init is a PURE-annotated CallExpression (`/* @__PURE__ */`), and the
/// declared name is not referenced elsewhere in the module body, convert
/// the VariableDeclaration to an ExpressionStatement (dropping the binding).
///
/// This matches SWC's tree-shaker/DCE behavior for `MinifyMode::Simplify`:
/// - `const App = /* @__PURE__ */ componentQrl(...)` -> `componentQrl(...);`
/// - `const Header = /* @__PURE__ */ qrl(...)` -> `qrl(...);`
///
/// Non-pure calls are NOT affected:
/// - `const renderHeader = component(qrl(...))` -> preserved
fn simplify_unused_pure_var_decls<'a>(
    stmts: &mut std::vec::Vec<Statement<'a>>,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    // 1. Collect all referenced identifiers across all statements.
    let all_refs = collect_referenced_idents(stmts);

    // 2. For each bare VariableDeclaration (not wrapped in export),
    //    check if it qualifies for removal.
    let mut i = 0;
    while i < stmts.len() {
        let should_unwrap = if let Statement::VariableDeclaration(ref var_decl) = stmts[i] {
            if var_decl.declarations.len() == 1 {
                if let BindingPattern::BindingIdentifier(ref ident) = var_decl.declarations[0].id {
                    let name = ident.name.as_str();
                    // Init must be a PURE-annotated call expression
                    let has_pure_call_init = var_decl.declarations[0]
                        .init
                        .as_ref()
                        .is_some_and(|init| {
                            matches!(init, Expression::CallExpression(call) if call.pure)
                        });
                    // Name must not be referenced elsewhere in the module
                    let is_referenced = all_refs.contains(name);
                    has_pure_call_init && !is_referenced
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if should_unwrap {
            let stmt = std::mem::replace(
                &mut stmts[i],
                Statement::EmptyStatement(ctx.ast.alloc(oxc::ast::ast::EmptyStatement { span: SPAN })),
            );
            if let Statement::VariableDeclaration(mut var_decl) = stmt {
                if let Some(init) = var_decl.declarations[0].init.take() {
                    let expr_stmt = ctx.ast.alloc(ExpressionStatement {
                        span: SPAN,
                        expression: init,
                    });
                    stmts[i] = Statement::ExpressionStatement(expr_stmt);
                }
            }
        }
        i += 1;
    }
}

/// Convert a JSX event attribute name to its HTML attribute equivalent.
///
/// Only applies when the attribute name ends with `$` and starts with `on`.
/// Returns `None` if the attribute doesn't match the pattern.
///
/// Port of SWC's `jsx_event_to_html_attribute` from
/// `crates/swc-optimizer/core/src/transform.rs:3347`.
///
/// Examples:
/// - `onClick$` -> `Some("q-e:click")`
/// - `onInput$` -> `Some("q-e:input")`
/// - `window:onClick$` is handled separately (prefix already stripped)
/// - `onClick` (no $) -> `None`
fn jsx_event_to_html_attribute(jsx_event: &str) -> Option<String> {
    if !jsx_event.ends_with('$') {
        return None;
    }

    let (prefix, idx) = get_event_scope_data_from_jsx_event(jsx_event);

    if idx == usize::MAX {
        return None;
    }

    let name = &jsx_event[idx..jsx_event.len() - 1];

    if name == "DOMContentLoaded" {
        return Some(format!("{}-d-o-m-content-loaded", prefix));
    }

    let processed_name = if let Some(stripped) = name.strip_prefix('-') {
        // marker for case sensitive event name
        stripped.to_string()
    } else {
        name.to_lowercase()
    };

    Some(create_event_name(&processed_name, prefix))
}

/// Get the event scope prefix and starting index from a JSX event name.
///
/// Port of SWC's `get_event_scope_data_from_jsx_event`.
fn get_event_scope_data_from_jsx_event(jsx_event: &str) -> (&str, usize) {
    if jsx_event.starts_with("window:on") {
        ("q-w:", 9)
    } else if jsx_event.starts_with("document:on") {
        ("q-d:", 11)
    } else if jsx_event.starts_with("on") {
        ("q-e:", 2)
    } else {
        ("", usize::MAX)
    }
}

/// Create an event name by converting from camelCase to kebab-case.
///
/// Port of SWC's `create_event_name`.
fn create_event_name(name: &str, prefix: &str) -> String {
    let mut result = String::from(prefix);

    for c in name.chars() {
        if c.is_ascii_uppercase() || c == '-' {
            result.push('-');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c);
        }
    }

    result
}

/// Collect all identifier names referenced in a list of statements.
///
/// Unlike `walk_statement_for_captures`, this function:
/// - DOES descend into nested function/arrow bodies (needed for inline strategy)
/// - Returns a HashSet of unique identifier names (not a Vec)
/// - Does NOT track local declarations (only collects references)
/// - Skips import declarations (we only want to see what non-import code references)
///
/// Used by `exit_program` to determine which synthetic framework imports and
/// non-Qwik user imports are actually needed in the entry module.
fn collect_referenced_idents(stmts: &[Statement<'_>]) -> HashSet<String> {
    let mut idents = HashSet::new();
    for stmt in stmts {
        if matches!(stmt, Statement::ImportDeclaration(_)) {
            continue; // Skip import declarations themselves
        }
        collect_idents_from_statement(stmt, &mut idents);
    }
    idents
}

/// Walk a statement collecting all identifier reference names (deep traversal).
fn collect_idents_from_statement(stmt: &Statement<'_>, idents: &mut HashSet<String>) {
    match stmt {
        Statement::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let Some(init) = &declarator.init {
                    collect_idents_from_expression(init, idents);
                }
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            collect_idents_from_expression(&expr_stmt.expression, idents);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                collect_idents_from_expression(arg, idents);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                collect_idents_from_statement(s, idents);
            }
        }
        Statement::IfStatement(if_stmt) => {
            collect_idents_from_expression(&if_stmt.test, idents);
            collect_idents_from_statement(&if_stmt.consequent, idents);
            if let Some(alt) = &if_stmt.alternate {
                collect_idents_from_statement(alt, idents);
            }
        }
        Statement::ForStatement(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                match init {
                    ForStatementInit::VariableDeclaration(var_decl) => {
                        for declarator in &var_decl.declarations {
                            if let Some(init_expr) = &declarator.init {
                                collect_idents_from_expression(init_expr, idents);
                            }
                        }
                    }
                    _ => {
                        if let Some(expr) = init.as_expression() {
                            collect_idents_from_expression(expr, idents);
                        }
                    }
                }
            }
            if let Some(test) = &for_stmt.test {
                collect_idents_from_expression(test, idents);
            }
            if let Some(update) = &for_stmt.update {
                collect_idents_from_expression(update, idents);
            }
            collect_idents_from_statement(&for_stmt.body, idents);
        }
        Statement::ForInStatement(for_in) => {
            collect_idents_from_expression(&for_in.right, idents);
            collect_idents_from_statement(&for_in.body, idents);
        }
        Statement::ForOfStatement(for_of) => {
            collect_idents_from_expression(&for_of.right, idents);
            collect_idents_from_statement(&for_of.body, idents);
        }
        Statement::WhileStatement(while_stmt) => {
            collect_idents_from_expression(&while_stmt.test, idents);
            collect_idents_from_statement(&while_stmt.body, idents);
        }
        Statement::DoWhileStatement(do_while) => {
            collect_idents_from_statement(&do_while.body, idents);
            collect_idents_from_expression(&do_while.test, idents);
        }
        Statement::FunctionDeclaration(func) => {
            // Descend into function body (unlike capture walker)
            for param in &func.params.items {
                collect_idents_from_binding_pattern(&param.pattern, idents);
            }
            if let Some(body) = &func.body {
                for s in &body.statements {
                    collect_idents_from_statement(s, idents);
                }
            }
        }
        Statement::SwitchStatement(switch) => {
            collect_idents_from_expression(&switch.discriminant, idents);
            for case in &switch.cases {
                if let Some(test) = &case.test {
                    collect_idents_from_expression(test, idents);
                }
                for s in &case.consequent {
                    collect_idents_from_statement(s, idents);
                }
            }
        }
        Statement::ThrowStatement(throw) => {
            collect_idents_from_expression(&throw.argument, idents);
        }
        Statement::TryStatement(try_stmt) => {
            for s in &try_stmt.block.body {
                collect_idents_from_statement(s, idents);
            }
            if let Some(handler) = &try_stmt.handler {
                for s in &handler.body.body {
                    collect_idents_from_statement(s, idents);
                }
            }
            if let Some(finalizer) = &try_stmt.finalizer {
                for s in &finalizer.body {
                    collect_idents_from_statement(s, idents);
                }
            }
        }
        Statement::LabeledStatement(labeled) => {
            collect_idents_from_statement(&labeled.body, idents);
        }
        // ExportNamedDeclaration and ExportDefaultDeclaration
        Statement::ExportNamedDeclaration(export) => {
            // Collect references from export specifiers: `export { X, Y as Z }`
            // The `local` name of each specifier references a module-level binding.
            for spec in &export.specifiers {
                let local_name = spec.local.name().as_str();
                idents.insert(local_name.to_string());
            }
            if let Some(decl) = &export.declaration {
                match decl {
                    Declaration::VariableDeclaration(var_decl) => {
                        for declarator in &var_decl.declarations {
                            if let Some(init) = &declarator.init {
                                collect_idents_from_expression(init, idents);
                            }
                        }
                    }
                    Declaration::FunctionDeclaration(func) => {
                        for param in &func.params.items {
                            collect_idents_from_binding_pattern(&param.pattern, idents);
                        }
                        if let Some(body) = &func.body {
                            for s in &body.statements {
                                collect_idents_from_statement(s, idents);
                            }
                        }
                    }
                    Declaration::ClassDeclaration(class) => {
                        if let Some(super_class) = &class.super_class {
                            collect_idents_from_expression(super_class, idents);
                        }
                        for elem in &class.body.body {
                            collect_idents_from_class_element(elem, idents);
                        }
                    }
                    _ => {}
                }
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            match &export.declaration {
                ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    for param in &func.params.items {
                        collect_idents_from_binding_pattern(&param.pattern, idents);
                    }
                    if let Some(body) = &func.body {
                        for s in &body.statements {
                            collect_idents_from_statement(s, idents);
                        }
                    }
                }
                ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                    if let Some(super_class) = &class.super_class {
                        collect_idents_from_expression(super_class, idents);
                    }
                    for elem in &class.body.body {
                        collect_idents_from_class_element(elem, idents);
                    }
                }
                _ => {
                    if let Some(expr) = export.declaration.as_expression() {
                        collect_idents_from_expression(expr, idents);
                    }
                }
            }
        }
        _ => {}
    }
}

/// Walk an expression collecting all identifier reference names (deep traversal).
/// Unlike `walk_expression_for_captures`, this DOES descend into nested functions.
fn collect_idents_from_expression(expr: &Expression<'_>, idents: &mut HashSet<String>) {
    match expr {
        Expression::Identifier(ident) => {
            idents.insert(ident.name.as_str().to_string());
        }
        Expression::CallExpression(call) => {
            collect_idents_from_expression(&call.callee, idents);
            for arg in &call.arguments {
                match arg {
                    Argument::SpreadElement(spread) => {
                        collect_idents_from_expression(&spread.argument, idents);
                    }
                    _ => {
                        if let Some(expr) = arg.as_expression() {
                            collect_idents_from_expression(expr, idents);
                        }
                    }
                }
            }
        }
        Expression::StaticMemberExpression(member) => {
            collect_idents_from_expression(&member.object, idents);
        }
        Expression::ComputedMemberExpression(member) => {
            collect_idents_from_expression(&member.object, idents);
            collect_idents_from_expression(&member.expression, idents);
        }
        Expression::PrivateFieldExpression(member) => {
            collect_idents_from_expression(&member.object, idents);
        }
        Expression::BinaryExpression(binary) => {
            collect_idents_from_expression(&binary.left, idents);
            collect_idents_from_expression(&binary.right, idents);
        }
        Expression::LogicalExpression(logical) => {
            collect_idents_from_expression(&logical.left, idents);
            collect_idents_from_expression(&logical.right, idents);
        }
        Expression::AssignmentExpression(assign) => {
            collect_idents_from_assignment_target(&assign.left, idents);
            collect_idents_from_expression(&assign.right, idents);
        }
        Expression::UnaryExpression(unary) => {
            collect_idents_from_expression(&unary.argument, idents);
        }
        Expression::UpdateExpression(update) => {
            match &update.argument {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    idents.insert(ident.name.as_str().to_string());
                }
                SimpleAssignmentTarget::StaticMemberExpression(member) => {
                    collect_idents_from_expression(&member.object, idents);
                }
                SimpleAssignmentTarget::ComputedMemberExpression(member) => {
                    collect_idents_from_expression(&member.object, idents);
                    collect_idents_from_expression(&member.expression, idents);
                }
                SimpleAssignmentTarget::PrivateFieldExpression(member) => {
                    collect_idents_from_expression(&member.object, idents);
                }
                _ => {}
            }
        }
        Expression::ConditionalExpression(cond) => {
            collect_idents_from_expression(&cond.test, idents);
            collect_idents_from_expression(&cond.consequent, idents);
            collect_idents_from_expression(&cond.alternate, idents);
        }
        Expression::TemplateLiteral(tmpl) => {
            for expr in &tmpl.expressions {
                collect_idents_from_expression(expr, idents);
            }
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    ArrayExpressionElement::SpreadElement(spread) => {
                        collect_idents_from_expression(&spread.argument, idents);
                    }
                    ArrayExpressionElement::Elision(_) => {}
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            collect_idents_from_expression(expr, idents);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        if p.computed {
                            if let Some(key_expr) = p.key.as_expression() {
                                collect_idents_from_expression(key_expr, idents);
                            }
                        }
                        collect_idents_from_expression(&p.value, idents);
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        collect_idents_from_expression(&spread.argument, idents);
                    }
                }
            }
        }
        // IMPORTANT: Unlike capture walker, we DO descend into nested functions
        // because in inline/hoist mode, the segment code is inside arrow functions.
        Expression::ArrowFunctionExpression(arrow) => {
            for param in &arrow.params.items {
                collect_idents_from_binding_pattern(&param.pattern, idents);
            }
            for s in &arrow.body.statements {
                collect_idents_from_statement(s, idents);
            }
        }
        Expression::FunctionExpression(func) => {
            for param in &func.params.items {
                collect_idents_from_binding_pattern(&param.pattern, idents);
            }
            if let Some(body) = &func.body {
                for s in &body.statements {
                    collect_idents_from_statement(s, idents);
                }
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            collect_idents_from_expression(&paren.expression, idents);
        }
        Expression::SequenceExpression(seq) => {
            for expr in &seq.expressions {
                collect_idents_from_expression(expr, idents);
            }
        }
        Expression::AwaitExpression(await_expr) => {
            collect_idents_from_expression(&await_expr.argument, idents);
        }
        Expression::TaggedTemplateExpression(tagged) => {
            collect_idents_from_expression(&tagged.tag, idents);
            for expr in &tagged.quasi.expressions {
                collect_idents_from_expression(expr, idents);
            }
        }
        Expression::NewExpression(new_expr) => {
            collect_idents_from_expression(&new_expr.callee, idents);
            for arg in &new_expr.arguments {
                match arg {
                    Argument::SpreadElement(spread) => {
                        collect_idents_from_expression(&spread.argument, idents);
                    }
                    _ => {
                        if let Some(expr) = arg.as_expression() {
                            collect_idents_from_expression(expr, idents);
                        }
                    }
                }
            }
        }
        Expression::YieldExpression(yield_expr) => {
            if let Some(arg) = &yield_expr.argument {
                collect_idents_from_expression(arg, idents);
            }
        }
        Expression::ImportExpression(import_expr) => {
            collect_idents_from_expression(&import_expr.source, idents);
        }
        Expression::ClassExpression(class) => {
            if let Some(super_class) = &class.super_class {
                collect_idents_from_expression(super_class, idents);
            }
            for elem in &class.body.body {
                collect_idents_from_class_element(elem, idents);
            }
        }
        // JSX expressions reference identifiers in element names and attribute values
        Expression::JSXElement(jsx) => {
            collect_idents_from_jsx_element(jsx, idents);
        }
        Expression::JSXFragment(frag) => {
            for child in &frag.children {
                collect_idents_from_jsx_child(child, idents);
            }
        }
        _ => {}
    }
}

/// Walk a JSX element collecting all identifier references.
fn collect_idents_from_jsx_element(elem: &JSXElement<'_>, idents: &mut HashSet<String>) {
    // Element name: <Component ...> references "Component"
    match &elem.opening_element.name {
        JSXElementName::Identifier(id) => {
            // Lowercase = HTML element (div, span), uppercase = component reference
            let name = id.name.as_str();
            if name.starts_with(|c: char| c.is_uppercase()) {
                idents.insert(name.to_string());
            }
        }
        JSXElementName::IdentifierReference(id) => {
            idents.insert(id.name.as_str().to_string());
        }
        JSXElementName::MemberExpression(member) => {
            collect_idents_from_jsx_member_expr(member, idents);
        }
        JSXElementName::NamespacedName(_) => {}
        JSXElementName::ThisExpression(_) => {}
    }

    // Attributes
    for attr in &elem.opening_element.attributes {
        match attr {
            JSXAttributeItem::Attribute(a) => {
                if let Some(value) = &a.value {
                    match value {
                        JSXAttributeValue::ExpressionContainer(expr) => {
                            if let Some(inner) = expr.expression.as_expression() {
                                collect_idents_from_expression(inner, idents);
                            }
                        }
                        _ => {}
                    }
                }
            }
            JSXAttributeItem::SpreadAttribute(spread) => {
                collect_idents_from_expression(&spread.argument, idents);
            }
        }
    }

    // Children
    for child in &elem.children {
        collect_idents_from_jsx_child(child, idents);
    }
}

/// Walk a JSX child collecting identifier references.
fn collect_idents_from_jsx_child(child: &JSXChild<'_>, idents: &mut HashSet<String>) {
    match child {
        JSXChild::Element(elem) => {
            collect_idents_from_jsx_element(elem, idents);
        }
        JSXChild::Fragment(frag) => {
            for c in &frag.children {
                collect_idents_from_jsx_child(c, idents);
            }
        }
        JSXChild::ExpressionContainer(expr) => {
            if let Some(inner) = expr.expression.as_expression() {
                collect_idents_from_expression(inner, idents);
            }
        }
        JSXChild::Spread(spread) => {
            collect_idents_from_expression(&spread.expression, idents);
        }
        JSXChild::Text(_) => {}
    }
}

/// Walk a JSX member expression to collect the root identifier.
fn collect_idents_from_jsx_member_expr(
    member: &JSXMemberExpression<'_>,
    idents: &mut HashSet<String>,
) {
    match &member.object {
        JSXMemberExpressionObject::IdentifierReference(id) => {
            idents.insert(id.name.as_str().to_string());
        }
        JSXMemberExpressionObject::MemberExpression(inner) => {
            collect_idents_from_jsx_member_expr(inner, idents);
        }
        JSXMemberExpressionObject::ThisExpression(_) => {}
    }
}

/// Walk a binding pattern collecting identifier references from default values.
fn collect_idents_from_binding_pattern(
    pattern: &BindingPattern<'_>,
    idents: &mut HashSet<String>,
) {
    match pattern {
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_idents_from_binding_pattern(&prop.value, idents);
            }
            if let Some(rest) = &obj.rest {
                collect_idents_from_binding_pattern(&rest.argument, idents);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_idents_from_binding_pattern(elem, idents);
            }
            if let Some(rest) = &arr.rest {
                collect_idents_from_binding_pattern(&rest.argument, idents);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_idents_from_binding_pattern(&assign.left, idents);
            collect_idents_from_expression(&assign.right, idents);
        }
        _ => {}
    }
}

/// Walk a class element collecting identifier references.
fn collect_idents_from_class_element(elem: &ClassElement<'_>, idents: &mut HashSet<String>) {
    match elem {
        ClassElement::MethodDefinition(method) => {
            if let Some(body) = &method.value.body {
                for s in &body.statements {
                    collect_idents_from_statement(s, idents);
                }
            }
        }
        ClassElement::PropertyDefinition(prop) => {
            if let Some(value) = &prop.value {
                collect_idents_from_expression(value, idents);
            }
        }
        ClassElement::StaticBlock(block) => {
            for s in &block.body {
                collect_idents_from_statement(s, idents);
            }
        }
        _ => {}
    }
}

/// Walk an assignment target collecting identifier references.
fn collect_idents_from_assignment_target(
    target: &AssignmentTarget<'_>,
    idents: &mut HashSet<String>,
) {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(ident) => {
            idents.insert(ident.name.as_str().to_string());
        }
        AssignmentTarget::StaticMemberExpression(member) => {
            collect_idents_from_expression(&member.object, idents);
        }
        AssignmentTarget::ComputedMemberExpression(member) => {
            collect_idents_from_expression(&member.object, idents);
            collect_idents_from_expression(&member.expression, idents);
        }
        _ => {}
    }
}

/// Deep scan a lambda's source code for ALL identifier references, including inside
/// nested arrow/function expressions. Used for iteration variable detection where
/// variables can be captured by nested closures.
/// Unlike `analyze_lambda_captures` which respects function scoping (doesn't descend
/// into nested functions), this descends everywhere to match SWC's `body_contains_ident`.
fn analyze_lambda_deep_ident_refs(source_code: &str, span: (u32, u32)) -> HashSet<String> {
    let start = span.0 as usize;
    let end = span.1 as usize;
    if start >= source_code.len() || end > source_code.len() || start >= end {
        return HashSet::new();
    }
    let lambda_source = &source_code[start..end];

    let parse_source = format!("var x = {}", lambda_source);
    let alloc = oxc::allocator::Allocator::default();
    let source_ref = alloc.alloc_str(&parse_source);

    let parser = oxc::parser::Parser::new(&alloc, source_ref, oxc::span::SourceType::tsx());
    let parse_result = parser.parse();

    if parse_result.program.body.is_empty() {
        return HashSet::new();
    }

    let mut ident_refs = HashSet::new();

    if let Some(Statement::VariableDeclaration(decl)) = parse_result.program.body.first() {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(ref init) = declarator.init {
                walk_expression_deep_idents(init, &mut ident_refs);
            }
        }
    }

    ident_refs
}

/// Walk an expression tree collecting ALL identifier references, descending into
/// nested arrow/function expressions (unlike `walk_expression_for_captures`).
fn walk_expression_deep_idents(expr: &Expression<'_>, idents: &mut HashSet<String>) {
    match expr {
        Expression::Identifier(ident) => {
            idents.insert(ident.name.as_str().to_string());
        }
        Expression::CallExpression(call) => {
            walk_expression_deep_idents(&call.callee, idents);
            for arg in &call.arguments {
                match arg {
                    Argument::SpreadElement(spread) => {
                        walk_expression_deep_idents(&spread.argument, idents);
                    }
                    _ => {
                        if let Some(e) = arg.as_expression() {
                            walk_expression_deep_idents(e, idents);
                        }
                    }
                }
            }
        }
        Expression::StaticMemberExpression(member) => {
            walk_expression_deep_idents(&member.object, idents);
        }
        Expression::ComputedMemberExpression(member) => {
            walk_expression_deep_idents(&member.object, idents);
            walk_expression_deep_idents(&member.expression, idents);
        }
        Expression::BinaryExpression(binary) => {
            walk_expression_deep_idents(&binary.left, idents);
            walk_expression_deep_idents(&binary.right, idents);
        }
        Expression::LogicalExpression(logical) => {
            walk_expression_deep_idents(&logical.left, idents);
            walk_expression_deep_idents(&logical.right, idents);
        }
        Expression::AssignmentExpression(assign) => {
            walk_assignment_target_deep_idents(&assign.left, idents);
            walk_expression_deep_idents(&assign.right, idents);
        }
        Expression::UnaryExpression(unary) => {
            walk_expression_deep_idents(&unary.argument, idents);
        }
        Expression::UpdateExpression(update) => {
            match &update.argument {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    idents.insert(ident.name.as_str().to_string());
                }
                SimpleAssignmentTarget::StaticMemberExpression(member) => {
                    walk_expression_deep_idents(&member.object, idents);
                }
                SimpleAssignmentTarget::ComputedMemberExpression(member) => {
                    walk_expression_deep_idents(&member.object, idents);
                    walk_expression_deep_idents(&member.expression, idents);
                }
                _ => {}
            }
        }
        Expression::ConditionalExpression(cond) => {
            walk_expression_deep_idents(&cond.test, idents);
            walk_expression_deep_idents(&cond.consequent, idents);
            walk_expression_deep_idents(&cond.alternate, idents);
        }
        Expression::TemplateLiteral(tmpl) => {
            for e in &tmpl.expressions {
                walk_expression_deep_idents(e, idents);
            }
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    ArrayExpressionElement::SpreadElement(spread) => {
                        walk_expression_deep_idents(&spread.argument, idents);
                    }
                    ArrayExpressionElement::Elision(_) => {}
                    _ => {
                        if let Some(e) = elem.as_expression() {
                            walk_expression_deep_idents(e, idents);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        walk_expression_deep_idents(&p.value, idents);
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        walk_expression_deep_idents(&spread.argument, idents);
                    }
                }
            }
        }
        // Descend into nested functions (unlike walk_expression_for_captures)
        Expression::ArrowFunctionExpression(arrow) => {
            for stmt in &arrow.body.statements {
                walk_statement_deep_idents(stmt, idents);
            }
        }
        Expression::FunctionExpression(func) => {
            if let Some(body) = &func.body {
                for stmt in &body.statements {
                    walk_statement_deep_idents(stmt, idents);
                }
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            walk_expression_deep_idents(&paren.expression, idents);
        }
        Expression::SequenceExpression(seq) => {
            for e in &seq.expressions {
                walk_expression_deep_idents(e, idents);
            }
        }
        Expression::AwaitExpression(await_expr) => {
            walk_expression_deep_idents(&await_expr.argument, idents);
        }
        _ => {}
    }
}

/// Walk a statement collecting all identifier references (deep -- descends into nested functions).
fn walk_statement_deep_idents(stmt: &Statement<'_>, idents: &mut HashSet<String>) {
    match stmt {
        Statement::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                if let Some(init) = &declarator.init {
                    walk_expression_deep_idents(init, idents);
                }
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            walk_expression_deep_idents(&expr_stmt.expression, idents);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                walk_expression_deep_idents(arg, idents);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                walk_statement_deep_idents(s, idents);
            }
        }
        Statement::IfStatement(if_stmt) => {
            walk_expression_deep_idents(&if_stmt.test, idents);
            walk_statement_deep_idents(&if_stmt.consequent, idents);
            if let Some(alt) = &if_stmt.alternate {
                walk_statement_deep_idents(alt, idents);
            }
        }
        Statement::ForStatement(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                if let Some(expr) = init.as_expression() {
                    walk_expression_deep_idents(expr, idents);
                }
            }
            if let Some(test) = &for_stmt.test {
                walk_expression_deep_idents(test, idents);
            }
            if let Some(update) = &for_stmt.update {
                walk_expression_deep_idents(update, idents);
            }
            walk_statement_deep_idents(&for_stmt.body, idents);
        }
        _ => {}
    }
}

/// Walk an assignment target collecting identifier references (deep scan).
fn walk_assignment_target_deep_idents(target: &AssignmentTarget<'_>, idents: &mut HashSet<String>) {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(ident) => {
            idents.insert(ident.name.as_str().to_string());
        }
        AssignmentTarget::StaticMemberExpression(member) => {
            walk_expression_deep_idents(&member.object, idents);
        }
        AssignmentTarget::ComputedMemberExpression(member) => {
            walk_expression_deep_idents(&member.object, idents);
            walk_expression_deep_idents(&member.expression, idents);
        }
        _ => {}
    }
}

/// Analyze a JSX lambda's source code to extract identifier references and local declarations.
///
/// Parses the lambda source, walks the resulting AST to collect:
/// - All IdentifierReference names (potential captures)
/// - All locally-declared names (parameters, let/const/var declarations)
///
/// Returns (body_ident_refs, body_local_decls) suitable for passing to `compute_captures()`.
fn analyze_lambda_captures(source_code: &str, span: (u32, u32)) -> (Vec<String>, HashSet<String>) {
    let start = span.0 as usize;
    let end = span.1 as usize;
    if start >= source_code.len() || end > source_code.len() || start >= end {
        return (Vec::new(), HashSet::new());
    }
    let lambda_source = &source_code[start..end];

    // Wrap as variable declaration so OXC can parse it
    let parse_source = format!("var x = {}", lambda_source);
    let alloc = oxc::allocator::Allocator::default();
    let source_ref = alloc.alloc_str(&parse_source);

    let parser = oxc::parser::Parser::new(&alloc, source_ref, oxc::span::SourceType::tsx());
    let parse_result = parser.parse();

    // Only bail on empty program body -- not on parse errors.
    // Some errors are semantic (e.g., `await` in non-async function) but the AST
    // is still well-formed and we can still extract identifier references for
    // capture analysis. Bailing on errors causes missing imports in segments.
    if parse_result.program.body.is_empty() {
        return (Vec::new(), HashSet::new());
    }

    // Extract the arrow/function expression from `var x = <expr>`
    if let Some(Statement::VariableDeclaration(decl)) = parse_result.program.body.first() {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(ref init) = declarator.init {
                let mut ident_refs = Vec::new();
                let mut local_decls = HashSet::new();

                // Collect parameter names as local declarations
                match init {
                    Expression::ArrowFunctionExpression(arrow) => {
                        for param in &arrow.params.items {
                            collect_binding_names_from_pattern(&param.pattern, &mut local_decls);
                        }
                        if let Some(rest) = &arrow.params.rest {
                            collect_binding_names_from_pattern(
                                &rest.rest.argument,
                                &mut local_decls,
                            );
                        }
                        // Walk the body for identifier references and local declarations
                        for stmt in &arrow.body.statements {
                            walk_statement_for_captures(stmt, &mut ident_refs, &mut local_decls);
                        }
                        // For expression bodies (single statement with implicit return),
                        // the AST wraps it as a return statement in the body, so we've
                        // already handled it above.
                    }
                    Expression::FunctionExpression(func) => {
                        for param in &func.params.items {
                            collect_binding_names_from_pattern(&param.pattern, &mut local_decls);
                        }
                        if let Some(body) = &func.body {
                            for stmt in &body.statements {
                                walk_statement_for_captures(
                                    stmt,
                                    &mut ident_refs,
                                    &mut local_decls,
                                );
                            }
                        }
                    }
                    _ => {}
                }

                return (ident_refs, local_decls);
            }
        }
    }

    (Vec::new(), HashSet::new())
}

/// Convert a BindingPattern to a human-readable string for paramNames metadata.
/// Matches SWC's `pat_to_string` (transform.rs:2312-2368).
fn binding_pattern_to_string(pattern: &BindingPattern<'_>) -> Option<String> {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => Some(ident.name.as_str().to_string()),
        BindingPattern::ObjectPattern(obj) => {
            let mut parts = Vec::new();
            for prop in &obj.properties {
                if prop.shorthand {
                    // Shorthand {a} -- equivalent to SWC's ObjectPatProp::Assign
                    if let BindingPattern::BindingIdentifier(ident) = &prop.value {
                        parts.push(ident.name.as_str().to_string());
                    }
                } else {
                    // KeyValue {key: value} -- equivalent to SWC's ObjectPatProp::KeyValue
                    let key_str = match &prop.key {
                        PropertyKey::StaticIdentifier(ident) => ident.name.as_str().to_string(),
                        PropertyKey::StringLiteral(s) => s.value.as_str().to_string(),
                        PropertyKey::NumericLiteral(n) => n.value.to_string(),
                        PropertyKey::BigIntLiteral(b) => {
                            b.raw.as_ref().map_or_else(String::new, |r| r.as_str().to_string())
                        }
                        _ => continue, // Computed keys: skip
                    };
                    if let Some(value) = binding_pattern_to_string(&prop.value) {
                        parts.push(format!("{}: {}", key_str, value));
                    }
                }
            }
            // Skip rest properties in object patterns (SWC skips ObjectPatProp::Rest)
            if parts.is_empty() {
                None
            } else {
                Some(format!("{{{}}}", parts.join(", ")))
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            let mut parts = Vec::new();
            for elem in &arr.elements {
                match elem {
                    Some(pat) => {
                        if let Some(name) = binding_pattern_to_string(pat) {
                            parts.push(name);
                        }
                    }
                    None => parts.push(String::new()),
                }
            }
            if parts.is_empty() {
                None
            } else {
                Some(format!("[{}]", parts.join(", ")))
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            // Assignment pattern with default: use the left-hand side
            binding_pattern_to_string(&assign.left)
        }
    }
}

/// Extract parameter names from FormalParameters.
/// Matches SWC's `extract_param_names` inner logic (transform.rs:2370-2399).
fn extract_param_names_from_params(params: &FormalParameters<'_>) -> Vec<String> {
    let mut names = Vec::new();
    for param in &params.items {
        if let Some(name) = binding_pattern_to_string(&param.pattern) {
            names.push(name);
        }
    }
    // Handle rest parameter: ...args
    if let Some(rest) = &params.rest {
        if let Some(name) = binding_pattern_to_string(&rest.rest.argument) {
            names.push(format!("...{}", name));
        }
    }
    names
}

/// Extract parameter names from a $() call argument.
/// Handles ArrowFunctionExpression and FunctionExpression arguments.
fn extract_param_names_from_argument(arg: &Argument<'_>) -> Vec<String> {
    match arg {
        Argument::ArrowFunctionExpression(arrow) => extract_param_names_from_params(&arrow.params),
        Argument::FunctionExpression(func) => extract_param_names_from_params(&func.params),
        _ => Vec::new(),
    }
}

/// Extract parameter names from a JSX attribute expression (event handler).
/// Handles ArrowFunctionExpression and FunctionExpression in JSX expression containers.
fn extract_param_names_from_jsx_expr(expr: &JSXExpression<'_>) -> Vec<String> {
    match expr {
        JSXExpression::ArrowFunctionExpression(arrow) => {
            extract_param_names_from_params(&arrow.params)
        }
        JSXExpression::FunctionExpression(func) => extract_param_names_from_params(&func.params),
        _ => Vec::new(),
    }
}

/// Collect binding names from a BindingPattern into a set.
fn collect_binding_names_from_pattern(pattern: &BindingPattern<'_>, names: &mut HashSet<String>) {
    match pattern {
        BindingPattern::BindingIdentifier(ident) => {
            names.insert(ident.name.as_str().to_string());
        }
        BindingPattern::ObjectPattern(obj) => {
            for prop in &obj.properties {
                collect_binding_names_from_pattern(&prop.value, names);
            }
            if let Some(rest) = &obj.rest {
                collect_binding_names_from_pattern(&rest.argument, names);
            }
        }
        BindingPattern::ArrayPattern(arr) => {
            for elem in arr.elements.iter().flatten() {
                collect_binding_names_from_pattern(elem, names);
            }
            if let Some(rest) = &arr.rest {
                collect_binding_names_from_pattern(&rest.argument, names);
            }
        }
        BindingPattern::AssignmentPattern(assign) => {
            collect_binding_names_from_pattern(&assign.left, names);
        }
    }
}

/// Walk a statement to collect identifier references and local declarations for capture analysis.
fn walk_statement_for_captures(
    stmt: &Statement<'_>,
    ident_refs: &mut Vec<String>,
    local_decls: &mut HashSet<String>,
) {
    match stmt {
        Statement::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                collect_binding_names_from_pattern(&declarator.id, local_decls);
                if let Some(init) = &declarator.init {
                    walk_expression_for_captures(init, ident_refs);
                }
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            walk_expression_for_captures(&expr_stmt.expression, ident_refs);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                walk_expression_for_captures(arg, ident_refs);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                walk_statement_for_captures(s, ident_refs, local_decls);
            }
        }
        Statement::IfStatement(if_stmt) => {
            walk_expression_for_captures(&if_stmt.test, ident_refs);
            walk_statement_for_captures(&if_stmt.consequent, ident_refs, local_decls);
            if let Some(alt) = &if_stmt.alternate {
                walk_statement_for_captures(alt, ident_refs, local_decls);
            }
        }
        Statement::ForStatement(for_stmt) => {
            if let Some(init) = &for_stmt.init {
                match init {
                    ForStatementInit::VariableDeclaration(var_decl) => {
                        for declarator in &var_decl.declarations {
                            collect_binding_names_from_pattern(&declarator.id, local_decls);
                            if let Some(init_expr) = &declarator.init {
                                walk_expression_for_captures(init_expr, ident_refs);
                            }
                        }
                    }
                    _ => {
                        if let Some(expr) = init.as_expression() {
                            walk_expression_for_captures(expr, ident_refs);
                        }
                    }
                }
            }
            if let Some(test) = &for_stmt.test {
                walk_expression_for_captures(test, ident_refs);
            }
            if let Some(update) = &for_stmt.update {
                walk_expression_for_captures(update, ident_refs);
            }
            walk_statement_for_captures(&for_stmt.body, ident_refs, local_decls);
        }
        Statement::FunctionDeclaration(func) => {
            if let Some(id) = &func.id {
                local_decls.insert(id.name.as_str().to_string());
            }
            // Don't walk inside function body -- inner functions create their own scope
        }
        _ => {}
    }
}

/// Walk an expression to collect identifier references for capture analysis.
fn walk_expression_for_captures(expr: &Expression<'_>, ident_refs: &mut Vec<String>) {
    match expr {
        Expression::Identifier(ident) => {
            ident_refs.push(ident.name.as_str().to_string());
        }
        Expression::CallExpression(call) => {
            walk_expression_for_captures(&call.callee, ident_refs);
            for arg in &call.arguments {
                match arg {
                    Argument::SpreadElement(spread) => {
                        walk_expression_for_captures(&spread.argument, ident_refs);
                    }
                    _ => {
                        if let Some(expr) = arg.as_expression() {
                            walk_expression_for_captures(expr, ident_refs);
                        }
                    }
                }
            }
        }
        Expression::StaticMemberExpression(member) => {
            walk_expression_for_captures(&member.object, ident_refs);
            // Don't add the property name as an identifier reference
        }
        Expression::ComputedMemberExpression(member) => {
            walk_expression_for_captures(&member.object, ident_refs);
            walk_expression_for_captures(&member.expression, ident_refs);
        }
        Expression::PrivateFieldExpression(member) => {
            walk_expression_for_captures(&member.object, ident_refs);
        }
        Expression::BinaryExpression(binary) => {
            walk_expression_for_captures(&binary.left, ident_refs);
            walk_expression_for_captures(&binary.right, ident_refs);
        }
        Expression::LogicalExpression(logical) => {
            walk_expression_for_captures(&logical.left, ident_refs);
            walk_expression_for_captures(&logical.right, ident_refs);
        }
        Expression::AssignmentExpression(assign) => {
            // For assignment targets, walk to collect identifiers
            walk_assignment_target_for_captures(&assign.left, ident_refs);
            walk_expression_for_captures(&assign.right, ident_refs);
        }
        Expression::UnaryExpression(unary) => {
            walk_expression_for_captures(&unary.argument, ident_refs);
        }
        Expression::UpdateExpression(update) => {
            match &update.argument {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(ident) => {
                    ident_refs.push(ident.name.as_str().to_string());
                }
                SimpleAssignmentTarget::StaticMemberExpression(member) => {
                    walk_expression_for_captures(&member.object, ident_refs);
                }
                SimpleAssignmentTarget::ComputedMemberExpression(member) => {
                    walk_expression_for_captures(&member.object, ident_refs);
                    walk_expression_for_captures(&member.expression, ident_refs);
                }
                SimpleAssignmentTarget::PrivateFieldExpression(member) => {
                    walk_expression_for_captures(&member.object, ident_refs);
                }
                _ => {}
            }
        }
        Expression::ConditionalExpression(cond) => {
            walk_expression_for_captures(&cond.test, ident_refs);
            walk_expression_for_captures(&cond.consequent, ident_refs);
            walk_expression_for_captures(&cond.alternate, ident_refs);
        }
        Expression::TemplateLiteral(tmpl) => {
            for expr in &tmpl.expressions {
                walk_expression_for_captures(expr, ident_refs);
            }
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    ArrayExpressionElement::SpreadElement(spread) => {
                        walk_expression_for_captures(&spread.argument, ident_refs);
                    }
                    ArrayExpressionElement::Elision(_) => {}
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            walk_expression_for_captures(expr, ident_refs);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        walk_expression_for_captures(&p.value, ident_refs);
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        walk_expression_for_captures(&spread.argument, ident_refs);
                    }
                }
            }
        }
        Expression::ArrowFunctionExpression(_) | Expression::FunctionExpression(_) => {
            // Don't descend into nested functions -- they create their own scope
        }
        Expression::ParenthesizedExpression(paren) => {
            walk_expression_for_captures(&paren.expression, ident_refs);
        }
        Expression::SequenceExpression(seq) => {
            for expr in &seq.expressions {
                walk_expression_for_captures(expr, ident_refs);
            }
        }
        Expression::AwaitExpression(await_expr) => {
            walk_expression_for_captures(&await_expr.argument, ident_refs);
        }
        Expression::TaggedTemplateExpression(tagged) => {
            walk_expression_for_captures(&tagged.tag, ident_refs);
            for expr in &tagged.quasi.expressions {
                walk_expression_for_captures(expr, ident_refs);
            }
        }
        Expression::NewExpression(new_expr) => {
            walk_expression_for_captures(&new_expr.callee, ident_refs);
            for arg in &new_expr.arguments {
                match arg {
                    Argument::SpreadElement(spread) => {
                        walk_expression_for_captures(&spread.argument, ident_refs);
                    }
                    _ => {
                        if let Some(expr) = arg.as_expression() {
                            walk_expression_for_captures(expr, ident_refs);
                        }
                    }
                }
            }
        }
        Expression::YieldExpression(yield_expr) => {
            if let Some(arg) = &yield_expr.argument {
                walk_expression_for_captures(arg, ident_refs);
            }
        }
        Expression::ImportExpression(import_expr) => {
            walk_expression_for_captures(&import_expr.source, ident_refs);
        }
        _ => {}
    }
}

/// Walk an assignment target to collect identifier references.
fn walk_assignment_target_for_captures(
    target: &AssignmentTarget<'_>,
    ident_refs: &mut Vec<String>,
) {
    match target {
        AssignmentTarget::AssignmentTargetIdentifier(ident) => {
            ident_refs.push(ident.name.as_str().to_string());
        }
        AssignmentTarget::StaticMemberExpression(member) => {
            walk_expression_for_captures(&member.object, ident_refs);
        }
        AssignmentTarget::ComputedMemberExpression(member) => {
            walk_expression_for_captures(&member.object, ident_refs);
            walk_expression_for_captures(&member.expression, ident_refs);
        }
        _ => {}
    }
}

/// Serialize a JSX lambda expression to a code string for segment body generation.
///
/// Uses a parse-and-codegen roundtrip: extracts the lambda source code from the
/// original source text using the span, wraps it as `var x = <lambda>`, parses it,
/// and then runs codegen on the expression to produce clean output.
///
/// This avoids unsafe pointer casts between JSXExpression and Expression types
/// (which use different enum layouts despite sharing variant types via inherit_variants!).
fn serialize_jsx_lambda_from_source(source_code: &str, span: (u32, u32)) -> String {
    let start = span.0 as usize;
    let end = span.1 as usize;
    if start >= source_code.len() || end > source_code.len() || start >= end {
        return String::new();
    }
    let lambda_source = &source_code[start..end];

    // Wrap in a variable declaration so OXC can parse it as a complete expression
    let parse_source = format!("var x = {}", lambda_source);
    let alloc = oxc::allocator::Allocator::default();
    let source_ref = alloc.alloc_str(&parse_source);

    let parser = oxc::parser::Parser::new(&alloc, source_ref, oxc::span::SourceType::tsx());
    let parse_result = parser.parse();

    if !parse_result.errors.is_empty() || parse_result.program.body.is_empty() {
        // Fallback: return the raw source text
        return lambda_source.to_string();
    }

    // Extract the expression from `var x = <expr>`
    if let Some(Statement::VariableDeclaration(decl)) = parse_result.program.body.first() {
        if let Some(declarator) = decl.declarations.first() {
            if let Some(ref init) = declarator.init {
                let mut codegen = oxc::codegen::Codegen::new();
                codegen.print_expression(init);
                return codegen.into_source_text();
            }
        }
    }

    lambda_source.to_string()
}

/// Check if a dollar-suffixed call name produces a tree-shakeable wrapper.
///
/// Only `component$` produces a side-effect-free wrapper (`componentQrl`).
/// All other wrappers (useStylesQrl, useTaskQrl, useVisibleTaskQrl,
/// serverStuffQrl, serverLoaderQrl, useResourceQrl, etc.) are side-effectful
/// runtime calls that must NOT be annotated with `/*#__PURE__*/`.
fn is_tree_shakeable_dollar_call(name: &str) -> bool {
    name == "component$"
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

/// Serialize an expression to a code string, preserving source comments.
///
/// Creates a temporary Program containing the expression as an ExpressionStatement,
/// includes source comments whose `attached_to` positions fall within the expression's
/// span range, and uses `Codegen::build()` to emit the code with comments.
///
/// The result is the expression code without the trailing semicolon/newline that
/// `build()` adds for the ExpressionStatement.
fn codegen_expression_with_comments<'a>(
    expr: Expression<'a>,
    source_text: &str,
    source_comments: &[Comment],
    ctx: &mut TraverseCtx<'a, ()>,
) -> String {
    use oxc::span::{GetSpan, SourceType};

    let expr_span = expr.span();

    // Filter comments to those attached to nodes within the expression's span range.
    let mut comments = ctx.ast.vec_with_capacity(source_comments.len());
    for comment in source_comments {
        if comment.attached_to >= expr_span.start && comment.attached_to < expr_span.end {
            comments.push(*comment);
        }
    }

    // Build a temporary program containing just this expression as an ExpressionStatement.
    let stmt = ctx.ast.statement_expression(expr_span, expr);
    let mut body = ctx.ast.vec_with_capacity(1);
    body.push(stmt);

    let source_in_arena = ctx.ast.allocator.alloc_str(source_text);
    let program = ctx.ast.program(
        expr_span,
        SourceType::mjs(),
        source_in_arena,
        comments,
        None,
        ctx.ast.vec(),
        body,
    );

    let codegen_result = oxc::codegen::Codegen::new().build(&program);
    let code = codegen_result.code;

    // build() emits "expression;\n" for an ExpressionStatement.
    // Strip the trailing ";\n" to get just the expression code.
    code.trim_end().strip_suffix(';').unwrap_or(code.trim_end()).to_string()
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
        // MemberExpression variants (inherited via inherit_variants!)
        Argument::ComputedMemberExpression(e) => Expression::ComputedMemberExpression(e),
        Argument::StaticMemberExpression(e) => Expression::StaticMemberExpression(e),
        Argument::PrivateFieldExpression(e) => Expression::PrivateFieldExpression(e),
        Argument::Super(e) => Expression::Super(e),
        Argument::V8IntrinsicExpression(e) => Expression::V8IntrinsicExpression(e),
        Argument::PrivateInExpression(e) => Expression::PrivateInExpression(e),
        // TypeScript expressions
        Argument::TSAsExpression(e) => Expression::TSAsExpression(e),
        Argument::TSSatisfiesExpression(e) => Expression::TSSatisfiesExpression(e),
        Argument::TSTypeAssertion(e) => Expression::TSTypeAssertion(e),
        Argument::TSNonNullExpression(e) => Expression::TSNonNullExpression(e),
        Argument::TSInstantiationExpression(e) => Expression::TSInstantiationExpression(e),
        // Catch-all for any other inherited variants
        _ => ctx.ast.expression_identifier(SPAN, "undefined"),
    }
}

/// Extract callback parameter names from the first argument of an iteration method.
/// E.g., for `.map((item, index) => ...)`, returns `["item", "index"]`.
fn extract_callback_params(arg: &Argument<'_>) -> Vec<String> {
    match arg {
        Argument::ArrowFunctionExpression(arrow) => arrow
            .params
            .items
            .iter()
            .filter_map(|p| {
                if let BindingPattern::BindingIdentifier(ref ident) = p.pattern {
                    Some(ident.name.to_string())
                } else {
                    None
                }
            })
            .collect(),
        Argument::FunctionExpression(func) => func
            .params
            .items
            .iter()
            .filter_map(|p| {
                if let BindingPattern::BindingIdentifier(ref ident) = p.pattern {
                    Some(ident.name.to_string())
                } else {
                    None
                }
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// Rewrite body destructuring alias references in non-JSX statements.
///
/// Replaces occurrences of `alias` with `props["key"]` (computed member access)
/// in variable declarations and expression statements. Does NOT touch return
/// statements since those contain JSX which is handled by detect_signal_wrap.
fn rewrite_body_destr_references<'a>(
    stmts: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    prop_map: &[(String, String)], // (local_alias, original_key)
    props_param_name: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    for stmt in stmts.iter_mut() {
        match stmt {
            Statement::VariableDeclaration(decl) => {
                for declarator in decl.declarations.iter_mut() {
                    if let Some(ref mut init) = declarator.init {
                        rewrite_expr_body_destr(init, prop_map, props_param_name, ctx);
                    }
                }
            }
            Statement::ExpressionStatement(expr_stmt) => {
                rewrite_expr_body_destr(&mut expr_stmt.expression, prop_map, props_param_name, ctx);
            }
            // Don't rewrite return statements -- JSX children are handled by detect_signal_wrap
            _ => {}
        }
    }
}

/// Recursively rewrite identifier references matching body destructuring aliases
/// to `props["key"]` computed member expressions.
fn rewrite_expr_body_destr<'a>(
    expr: &mut Expression<'a>,
    prop_map: &[(String, String)],
    props_param_name: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    match expr {
        Expression::Identifier(ident) => {
            let name = ident.name.as_str();
            for (local_alias, original_key) in prop_map {
                if local_alias == name {
                    // Replace with props["original_key"]
                    let obj = ctx.ast.expression_identifier(SPAN, ctx.ast.atom(props_param_name));
                    let key_atom = ctx.ast.atom(original_key.as_str());
                    let key_expr = ctx.ast.expression_string_literal(SPAN, key_atom, None);
                    let member = ctx.ast.computed_member_expression(SPAN, obj, key_expr, false);
                    *expr = Expression::ComputedMemberExpression(ctx.ast.alloc(member));
                    return;
                }
            }
        }
        Expression::CallExpression(call) => {
            rewrite_expr_body_destr(&mut call.callee, prop_map, props_param_name, ctx);
            for i in 0..call.arguments.len() {
                let placeholder = Argument::from(ctx.ast.expression_identifier(SPAN, "undefined"));
                let old = std::mem::replace(&mut call.arguments[i], placeholder);
                let mut arg_expr = argument_to_expression(old, ctx);
                rewrite_expr_body_destr(&mut arg_expr, prop_map, props_param_name, ctx);
                call.arguments[i] = Argument::from(arg_expr);
            }
        }
        Expression::BinaryExpression(bin) => {
            rewrite_expr_body_destr(&mut bin.left, prop_map, props_param_name, ctx);
            rewrite_expr_body_destr(&mut bin.right, prop_map, props_param_name, ctx);
        }
        Expression::StaticMemberExpression(mem) => {
            rewrite_expr_body_destr(&mut mem.object, prop_map, props_param_name, ctx);
        }
        Expression::ComputedMemberExpression(mem) => {
            rewrite_expr_body_destr(&mut mem.object, prop_map, props_param_name, ctx);
            rewrite_expr_body_destr(&mut mem.expression, prop_map, props_param_name, ctx);
        }
        Expression::ConditionalExpression(cond) => {
            rewrite_expr_body_destr(&mut cond.test, prop_map, props_param_name, ctx);
            rewrite_expr_body_destr(&mut cond.consequent, prop_map, props_param_name, ctx);
            rewrite_expr_body_destr(&mut cond.alternate, prop_map, props_param_name, ctx);
        }
        Expression::LogicalExpression(log) => {
            rewrite_expr_body_destr(&mut log.left, prop_map, props_param_name, ctx);
            rewrite_expr_body_destr(&mut log.right, prop_map, props_param_name, ctx);
        }
        Expression::UnaryExpression(unary) => {
            rewrite_expr_body_destr(&mut unary.argument, prop_map, props_param_name, ctx);
        }
        Expression::ParenthesizedExpression(paren) => {
            rewrite_expr_body_destr(&mut paren.expression, prop_map, props_param_name, ctx);
        }
        Expression::TemplateLiteral(tmpl) => {
            for e in tmpl.expressions.iter_mut() {
                rewrite_expr_body_destr(e, prop_map, props_param_name, ctx);
            }
        }
        _ => {}
    }
}

/// Strip `const <alias> = <expr>` bindings where `<alias>` matches a destructured
/// prop key, converting them to expression statements containing just `<expr>`.
///
/// This handles the `destructure_args_colon_props3` pattern where:
/// `const test = useSignal(rest["bind:value"])` -> `useSignal(rest["bind:value"])`
/// because `test` was originally a destructured prop from `const { test, ...rest } = props`.
fn strip_prop_alias_bindings<'a>(
    stmts: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    non_rest_aliases: &std::collections::HashSet<String>,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    let mut indices_to_replace: Vec<(usize, Expression<'a>)> = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        if let Statement::VariableDeclaration(decl) = stmt {
            if decl.declarations.len() == 1 {
                let declarator = &mut decl.declarations[0];
                if let BindingPattern::BindingIdentifier(ident) = &declarator.id {
                    if non_rest_aliases.contains(ident.name.as_str()) {
                        if let Some(init) = declarator.init.take() {
                            indices_to_replace.push((i, init));
                        }
                    }
                }
            }
        }
    }

    // Replace in reverse order to preserve indices
    for (i, init_expr) in indices_to_replace.into_iter().rev() {
        let expr_stmt = ctx.ast.statement_expression(SPAN, init_expr);
        stmts[i] = expr_stmt;
    }
}
