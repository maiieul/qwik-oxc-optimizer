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
            captures: false,       // Phase 9 will compute captures
            capture_names: vec![], // Phase 9
            needed_imports: vec![], // Phase 10 (code_move)
            body_span: (call.span.start, call.span.end),
            param_names: vec![],   // Set by props destructuring if needed
        };

        // Track imports based on strategy
        let is_inline = entry_strategy::should_inline(&self.options.entry_strategy);

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

            // Record the segment
            let segment = self.record_segment(call, &kind);

            // Mark this call span so exit_expression can find it
            self.pending_dollar_calls.insert(call.span.start);

            // Push to nesting stack
            self.dollar_call_stack.push(segment.display_name.clone());
        }
    }

    fn exit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
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

            // Apply props destructuring if active for this component$ call
            let props_info = self.active_props_info.take();
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
                    }
                }
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

            let is_inline = entry_strategy::should_inline(&self.options.entry_strategy);

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
                // Segment strategy: qrl(i_hash, "name_hash")
                let import_ident = format!("i_{}", segment_info.hash);
                import_rewrite::build_qrl_call(&import_ident, &segment_info.name, ctx)
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

        // 4. Add lazy import constants (segment strategy)
        for (hash, import_path) in &self.import_tracker.lazy_imports {
            let stmt = import_rewrite::build_lazy_import_declaration(hash, import_path, ctx);
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

/// Convert an Argument to an Expression.
///
/// In OXC 0.113, Argument uses inherit_variants! from Expression, meaning
/// all Expression variants are directly on Argument. We need to match and convert.
fn argument_to_expression<'a>(
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
