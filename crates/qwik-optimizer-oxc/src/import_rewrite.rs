//! Import mutation logic.
//!
//! Transform import declarations after the main traversal. Removes `$`-suffixed
//! imports that were consumed by the transform, adds new Qrl-suffixed imports,
//! and adds `qrl` or `inlinedQrl` imports as needed.
//!
//! Also provides AST builder functions for constructing QRL call expressions,
//! named imports, and lazy import declarations.

use oxc::ast::ast::*;
use oxc::span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a segment-strategy QRL call expression:
///   `qrl(i_hashValue, "SegmentName_hash")`                        -- no captures
///   `qrl(i_hashValue, "SegmentName_hash", [captured_vars])`       -- with captures
///
/// The string parameters are allocated into the arena via `ctx.ast.atom()`.
pub(crate) fn build_qrl_call<'a>(
    import_ident_name: &str,
    segment_export_name: &str,
    captures: &[String],
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let import_atom = ctx.ast.atom(import_ident_name);
    let name_atom = ctx.ast.atom(segment_export_name);

    let import_ref = ctx.ast.expression_identifier(SPAN, import_atom);

    let name_literal = ctx.ast.expression_string_literal(SPAN, name_atom, None);

    let capacity = if captures.is_empty() { 2 } else { 3 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);
    arguments.push(Argument::from(import_ref));
    arguments.push(Argument::from(name_literal));

    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            let cap_atom = ctx.ast.atom(capture_name.as_str());
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier(SPAN, cap_atom),
            ));
        }
        arguments.push(Argument::from(ctx.ast.expression_array(SPAN, elements)));
    }

    let callee = ctx.ast.expression_identifier(SPAN, "qrl");

    ctx.ast.expression_call_with_pure(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
        true, // pure annotation
    )
}

/// Build an inline-strategy QRL call expression:
///   `inlinedQrl(() => { body }, "Name_hash")`                    -- no captures
///   `inlinedQrl(() => { body }, "Name_hash", [captured_vars])`   -- with captures
pub(crate) fn build_inlined_qrl_call<'a>(
    body_expr: Expression<'a>,
    segment_name: &str,
    captures: &[String],
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let name_atom = ctx.ast.atom(segment_name);

    let capacity = if captures.is_empty() { 2 } else { 3 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);

    arguments.push(Argument::from(body_expr));

    arguments.push(Argument::from(
        ctx.ast.expression_string_literal(SPAN, name_atom, None),
    ));

    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            let cap_atom = ctx.ast.atom(capture_name.as_str());
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier(SPAN, cap_atom),
            ));
        }
        arguments.push(Argument::from(ctx.ast.expression_array(SPAN, elements)));
    }

    let callee = ctx.ast.expression_identifier(SPAN, "inlinedQrl");

    ctx.ast.expression_call_with_pure(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
        true, // pure annotation
    )
}

/// Build a named import declaration:
///   `import { name } from "source"`
pub(crate) fn build_named_import<'a>(
    name: &str,
    source: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Statement<'a> {
    let name_atom = ctx.ast.atom(name);
    let source_atom = ctx.ast.atom(source);

    let local = ctx.ast.binding_identifier(SPAN, name_atom.clone());

    let imported = ctx.ast.module_export_name_identifier_name(SPAN, name_atom);

    let specifier = ctx
        .ast
        .import_specifier(SPAN, imported, local, ImportOrExportKind::Value);

    let specifiers = ctx.ast.vec1(ImportDeclarationSpecifier::ImportSpecifier(
        ctx.ast.alloc(specifier),
    ));

    let source_lit = ctx.ast.string_literal(SPAN, source_atom, None);

    let import_decl = ctx.ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        None,
        None::<oxc::allocator::Box<'a, WithClause<'a>>>,
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}

/// Build an aliased import declaration:
///   `import { imported_name as local_name } from "source"`
///
/// Used for `import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime"`.
pub(crate) fn build_aliased_import<'a>(
    imported_name: &str,
    local_name: &str,
    source: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Statement<'a> {
    let imported_atom = ctx.ast.atom(imported_name);
    let local_atom = ctx.ast.atom(local_name);
    let source_atom = ctx.ast.atom(source);

    let local = ctx.ast.binding_identifier(SPAN, local_atom);

    let imported = ctx
        .ast
        .module_export_name_identifier_name(SPAN, imported_atom);

    let specifier = ctx
        .ast
        .import_specifier(SPAN, imported, local, ImportOrExportKind::Value);

    let specifiers = ctx.ast.vec1(ImportDeclarationSpecifier::ImportSpecifier(
        ctx.ast.alloc(specifier),
    ));

    let source_lit = ctx.ast.string_literal(SPAN, source_atom, None);

    let import_decl = ctx.ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        None,
        None::<oxc::allocator::Box<'a, WithClause<'a>>>,
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}

/// Build a lazy import declaration:
///   `const i_hash = () => import("./path_segment_hash")`
pub(crate) fn build_lazy_import_declaration<'a>(
    hash: &str,
    import_path: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Statement<'a> {
    let ident_name = format!("i_{}", hash);
    let ident_atom = ctx.ast.atom(&ident_name);
    let path_atom = ctx.ast.atom(import_path);

    let import_source = ctx.ast.expression_string_literal(SPAN, path_atom, None);
    let import_expr = ctx.ast.expression_import(
        SPAN,
        import_source,
        None, // no options
        None, // no phase
    );

    let params = ctx.ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ctx.ast.vec(), // no parameters
        None::<oxc::allocator::Box<'a, FormalParameterRest<'a>>>,
    );

    let expr_stmt = ctx.ast.statement_expression(SPAN, import_expr);
    let body = ctx.ast.function_body(
        SPAN,
        ctx.ast.vec(),           // no directives
        ctx.ast.vec1(expr_stmt), // single expression statement
    );

    let arrow = ctx.ast.expression_arrow_function(
        SPAN,
        true,  // expression body
        false, // not async
        None::<oxc::allocator::Box<'a, TSTypeParameterDeclaration<'a>>>,
        params,
        None::<oxc::allocator::Box<'a, TSTypeAnnotation<'a>>>,
        body,
    );

    let binding = ctx.ast.binding_pattern_binding_identifier(SPAN, ident_atom);
    let declarator = ctx.ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        binding,
        None::<oxc::allocator::Box<'a, TSTypeAnnotation<'a>>>,
        Some(arrow),
        false,
    );
    let declaration = ctx.ast.variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ctx.ast.vec1(declarator),
        false,
    );

    Statement::from(Declaration::VariableDeclaration(ctx.ast.alloc(declaration)))
}

/// Build a _wrapProp(signal) call expression (Form 1: signal.value access).
///
/// Strips the `.value` access and passes just the signal identifier.
/// Used when a JSX prop value is `signal.value`.
pub(crate) fn build_wrap_prop_call<'a>(
    signal_expr: Expression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier(SPAN, "_wrapProp");
    let mut args = ctx.ast.vec_with_capacity(1);
    args.push(Argument::from(signal_expr));

    ctx.ast.expression_call(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        args,
        false,
    )
}

/// Build a _wrapProp(source, "propName") call expression (Form 2: named property wrapping).
///
/// Used when a JSX prop value is `_rawProps.propName` or `store.propName`.
pub(crate) fn build_wrap_prop_call_named<'a>(
    source_expr: Expression<'a>,
    prop_name: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier(SPAN, "_wrapProp");
    let prop_atom = ctx.ast.atom(prop_name);
    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(source_expr));
    args.push(Argument::from(
        ctx.ast.expression_string_literal(SPAN, prop_atom, None),
    ));

    ctx.ast.expression_call(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        args,
        false,
    )
}

/// Build a _noopQrl call expression for stripped segments:
///   `/*#__PURE__*/ _noopQrl("s_HASH")`                     -- no captures
///   `/*#__PURE__*/ _noopQrl("s_HASH", [captured_vars])`    -- with captures
pub(crate) fn build_noop_qrl_call<'a>(
    segment_name: &str,
    captures: &[String],
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let name_atom = ctx.ast.atom(segment_name);
    let name_literal = ctx.ast.expression_string_literal(SPAN, name_atom, None);

    let capacity = if captures.is_empty() { 1 } else { 2 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);
    arguments.push(Argument::from(name_literal));

    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for cap in captures {
            let cap_atom = ctx.ast.atom(cap);
            let cap_ref = ctx.ast.expression_identifier(SPAN, cap_atom);
            elements.push(ArrayExpressionElement::from(cap_ref));
        }
        let captures_array = ctx.ast.expression_array(SPAN, elements);
        arguments.push(Argument::from(captures_array));
    }

    let noop_atom = ctx.ast.atom("_noopQrl");
    let noop_callee = ctx.ast.expression_identifier(SPAN, noop_atom);
    ctx.ast.expression_call_with_pure(
        SPAN,
        noop_callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
        true, // PURE annotation
    )
}

/// Build a _qrlSync call expression for sync$ serialization:
///   `_qrlSync(fn, "stringified_fn")`
///
/// Note: _qrlSync does NOT get a PURE annotation (sync handlers are side-effectful).
pub(crate) fn build_qrl_sync_call<'a>(
    fn_expr: Expression<'a>,
    minified_string: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let str_atom = ctx.ast.atom(minified_string);
    let str_literal = ctx.ast.expression_string_literal(SPAN, str_atom, None);

    let mut arguments = ctx.ast.vec_with_capacity(2);
    arguments.push(Argument::from(fn_expr));
    arguments.push(Argument::from(str_literal));

    let sync_atom = ctx.ast.atom("_qrlSync");
    let sync_callee = ctx.ast.expression_identifier(SPAN, sync_atom);
    ctx.ast.expression_call(
        SPAN,
        sync_callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
    )
}
