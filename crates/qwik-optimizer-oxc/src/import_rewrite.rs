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

use crate::types::ImportInfo;

/// Describes what imports to add/remove/keep after transformation.
pub(crate) struct ImportChanges {
    /// Import specifiers to add (e.g., "componentQrl", "qrl").
    pub to_add: Vec<String>,

    /// Import specifiers to remove (e.g., "component$").
    pub to_remove: Vec<String>,

    /// Import specifiers to keep unchanged.
    pub to_keep: Vec<String>,
}

/// Compute what import changes are needed after transformation.
pub(crate) fn compute_import_changes(
    original_imports: &[ImportInfo],
    needed_qrl_names: &[String],
    needs_qrl: bool,
    needs_inlined_qrl: bool,
) -> ImportChanges {
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();
    let mut to_keep = Vec::new();

    // Collect all dollar-suffixed specifiers from qwik core imports
    for import in original_imports {
        if import.is_qwik_core {
            for spec in &import.specifiers {
                if spec == "$" || spec.ends_with('$') {
                    to_remove.push(spec.clone());
                } else {
                    to_keep.push(spec.clone());
                }
            }
        }
    }

    // Add needed Qrl-suffixed imports
    for qrl_name in needed_qrl_names {
        to_add.push(qrl_name.clone());
    }

    // Add qrl/inlinedQrl
    if needs_qrl {
        to_add.push("qrl".to_string());
    }
    if needs_inlined_qrl {
        to_add.push("inlinedQrl".to_string());
    }

    ImportChanges {
        to_add,
        to_remove,
        to_keep,
    }
}

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
    // Allocate strings into the arena so they live for 'a
    let import_atom = ctx.ast.atom(import_ident_name);
    let name_atom = ctx.ast.atom(segment_export_name);

    // Argument 1: identifier reference to the lazy import function
    let import_ref = ctx.ast.expression_identifier(SPAN, import_atom);

    // Argument 2: string literal with the segment export name
    let name_literal = ctx
        .ast
        .expression_string_literal(SPAN, name_atom, None);

    // Capacity: import_ref + name + optional captures array
    let capacity = if captures.is_empty() { 2 } else { 3 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);
    arguments.push(Argument::from(import_ref));
    arguments.push(Argument::from(name_literal));

    // Argument 3 (optional): captures array
    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            let cap_atom = ctx.ast.atom(capture_name.as_str());
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier(SPAN, cap_atom),
            ));
        }
        arguments.push(Argument::from(
            ctx.ast.expression_array(SPAN, elements),
        ));
    }

    // Build callee: identifier "qrl"
    let callee = ctx.ast.expression_identifier(SPAN, "qrl");

    // Build: /*#__PURE__*/ qrl(i_hash, "name", [captures])
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
    // Allocate segment name into the arena
    let name_atom = ctx.ast.atom(segment_name);

    // Capacity: body + name + optional captures array
    let capacity = if captures.is_empty() { 2 } else { 3 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);

    // Argument 1: the expression (arrow function, string literal, etc.)
    arguments.push(Argument::from(body_expr));

    // Argument 2: string literal segment name
    arguments.push(Argument::from(
        ctx.ast
            .expression_string_literal(SPAN, name_atom, None),
    ));

    // Argument 3 (optional): captures array
    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            let cap_atom = ctx.ast.atom(capture_name.as_str());
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier(SPAN, cap_atom),
            ));
        }
        arguments.push(Argument::from(
            ctx.ast.expression_array(SPAN, elements),
        ));
    }

    // Build callee: identifier "inlinedQrl"
    let callee = ctx.ast.expression_identifier(SPAN, "inlinedQrl");

    // Build: /*#__PURE__*/ inlinedQrl(body, "name", [captures])
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
    // Allocate strings into the arena
    let name_atom = ctx.ast.atom(name);
    let source_atom = ctx.ast.atom(source);

    // Build the local binding identifier
    let local = ctx.ast.binding_identifier(SPAN, name_atom.clone());

    // Build the imported name (same as local for non-aliased imports)
    let imported = ctx
        .ast
        .module_export_name_identifier_name(SPAN, name_atom);

    // Build the import specifier: { name }
    let specifier = ctx
        .ast
        .import_specifier(SPAN, imported, local, ImportOrExportKind::Value);

    // Wrap in specifiers vec
    let specifiers = ctx
        .ast
        .vec1(ImportDeclarationSpecifier::ImportSpecifier(
            ctx.ast.alloc(specifier),
        ));

    // Build the source string literal: "@qwik.dev/core"
    let source_lit = ctx.ast.string_literal(SPAN, source_atom, None);

    // Build the import declaration
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

    // Build the local binding identifier (the alias)
    let local = ctx.ast.binding_identifier(SPAN, local_atom);

    // Build the imported name (the original export name)
    let imported = ctx
        .ast
        .module_export_name_identifier_name(SPAN, imported_atom);

    // Build the import specifier: { imported_name as local_name }
    let specifier = ctx
        .ast
        .import_specifier(SPAN, imported, local, ImportOrExportKind::Value);

    // Wrap in specifiers vec
    let specifiers = ctx
        .ast
        .vec1(ImportDeclarationSpecifier::ImportSpecifier(
            ctx.ast.alloc(specifier),
        ));

    // Build the source string literal
    let source_lit = ctx.ast.string_literal(SPAN, source_atom, None);

    // Build the import declaration
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
    // Allocate strings into the arena
    let ident_atom = ctx.ast.atom(&ident_name);
    let path_atom = ctx.ast.atom(import_path);

    // Build the import expression: import("./path_segment_hash")
    let import_source = ctx
        .ast
        .expression_string_literal(SPAN, path_atom, None);
    let import_expr = ctx.ast.expression_import(
        SPAN,
        import_source,
        None, // no options
        None, // no phase
    );

    // Build the arrow function: () => import(...)
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

    // Build: const i_hash = () => import(...)
    let binding = ctx
        .ast
        .binding_pattern_binding_identifier(SPAN, ident_atom);
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

    Statement::from(Declaration::VariableDeclaration(
        ctx.ast.alloc(declaration),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_import_changes_basic() {
        let imports = vec![ImportInfo {
            source: "@qwik.dev/core".to_string(),
            specifiers: vec![
                "$".to_string(),
                "component$".to_string(),
                "useStore".to_string(),
            ],
            is_qwik_core: true,
            span: (0, 50),
        }];

        let changes = compute_import_changes(
            &imports,
            &["componentQrl".to_string()],
            true,
            false,
        );

        assert!(changes.to_remove.contains(&"$".to_string()));
        assert!(changes.to_remove.contains(&"component$".to_string()));
        assert!(changes.to_keep.contains(&"useStore".to_string()));
        assert!(changes.to_add.contains(&"componentQrl".to_string()));
        assert!(changes.to_add.contains(&"qrl".to_string()));
    }

    #[test]
    fn test_compute_import_changes_inline() {
        let imports = vec![ImportInfo {
            source: "@qwik.dev/core".to_string(),
            specifiers: vec!["$".to_string()],
            is_qwik_core: true,
            span: (0, 30),
        }];

        let changes = compute_import_changes(
            &imports,
            &[],
            false,
            true,
        );

        assert!(changes.to_add.contains(&"inlinedQrl".to_string()));
        assert!(!changes.to_add.contains(&"qrl".to_string()));
    }
}
