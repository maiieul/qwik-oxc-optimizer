//! Export stripping.
//!
//! Strip exports from the module based on the `strip_exports` option.
//! Used for server/client mode where certain exports should be removed.
//!
//! When an export name is in the `strip_exports` list, its function/arrow
//! body is replaced with a single `throw "Symbol removed ..."` statement.
//! The export declaration itself is preserved so that downstream consumers
//! see the binding but get a runtime error if they call it on the wrong
//! platform.

use oxc::ast::ast::*;
use oxc::ast::AstBuilder;
use oxc::span::SPAN;

/// The error message injected into stripped export bodies.
const STRIP_MESSAGE: &str =
    "Symbol removed by Qwik Optimizer, it can not be called from current platform";

/// Filter (strip) exports from the program AST.
///
/// For each export whose binding name appears in `strip_exports`, the
/// function/arrow body is replaced with `{ throw "Symbol removed ..." }`.
///
/// This must run BEFORE the main traverse so that the transform pass does
/// not attempt to extract $-calls from stripped export bodies.
pub(crate) fn filter_exports<'a>(
    program: &mut Program<'a>,
    strip_exports: &[String],
    _strip_ctx_name: &[String],
    allocator: &'a oxc::allocator::Allocator,
) {
    if strip_exports.is_empty() {
        return;
    }

    let ast = AstBuilder::new(allocator);

    for stmt in program.body.iter_mut() {
        match stmt {
            Statement::ExportNamedDeclaration(export_decl) => {
                if let Some(ref mut decl) = export_decl.declaration {
                    match decl {
                        Declaration::VariableDeclaration(var_decl) => {
                            for declarator in var_decl.declarations.iter_mut() {
                                if let Some(name) = binding_pattern_name(&declarator.id) {
                                    if strip_exports.iter().any(|s| s == name) {
                                        replace_init_body(
                                            &mut declarator.init,
                                            &ast,
                                        );
                                    }
                                }
                            }
                        }
                        Declaration::FunctionDeclaration(func_decl) => {
                            if let Some(ref id) = func_decl.id {
                                let name = id.name.as_str();
                                if strip_exports.iter().any(|s| s == name) {
                                    // Replace function body with throw statement
                                    replace_function_body(&mut func_decl.body, &ast);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            Statement::ExportDefaultDeclaration(export_default) => {
                if strip_exports.iter().any(|s| s == "default") {
                    replace_default_export_body(
                        &mut export_default.declaration,
                        &ast,
                    );
                }
            }
            _ => {}
        }
    }
}

/// Replace the body of a function/arrow initializer with the throw stub.
///
/// Handles:
///   - ArrowFunctionExpression: replace body statements
///   - FunctionExpression: replace body statements
///   - Other expressions: leave as-is (not a function)
fn replace_init_body<'a>(
    init: &mut Option<Expression<'a>>,
    ast: &AstBuilder<'a>,
) {
    if let Some(expr) = init {
        match expr {
            Expression::ArrowFunctionExpression(arrow) => {
                arrow.expression = false;
                let throw_body = build_throw_body(ast);
                arrow.body = ast.alloc(throw_body);
            }
            Expression::FunctionExpression(func) => {
                let throw_body = build_throw_body(ast);
                func.body = Some(ast.alloc(throw_body));
            }
            _ => {
            }
        }
    }
}

/// Replace a function declaration's body with the throw stub.
fn replace_function_body<'a>(
    body: &mut Option<oxc::allocator::Box<'a, FunctionBody<'a>>>,
    ast: &AstBuilder<'a>,
) {
    let throw_body = build_throw_body(ast);
    *body = Some(ast.alloc(throw_body));
}

/// Replace a default export declaration body with the throw stub.
fn replace_default_export_body<'a>(
    decl: &mut ExportDefaultDeclarationKind<'a>,
    ast: &AstBuilder<'a>,
) {
    match decl {
        ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
            let throw_body = build_throw_body(ast);
            func.body = Some(ast.alloc(throw_body));
        }
        _ => {
        }
    }
}

/// Build a FunctionBody containing a single: `throw "Symbol removed ..."`
fn build_throw_body<'a>(ast: &AstBuilder<'a>) -> FunctionBody<'a> {
    let message = ast.expression_string_literal(SPAN, STRIP_MESSAGE, None);
    let throw_stmt = ast.statement_throw(SPAN, message);

    ast.function_body(SPAN, ast.vec(), ast.vec1(throw_stmt))
}

/// Extract the binding name from a BindingPattern, if it's a simple identifier.
fn binding_pattern_name<'a>(pattern: &BindingPattern<'a>) -> Option<&'a str> {
    match pattern {
        BindingPattern::BindingIdentifier(id) => Some(id.name.as_str()),
        _ => None,
    }
}
