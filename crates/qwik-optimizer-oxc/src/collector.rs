//! First-pass AST analysis.
//!
//! Walk the parsed AST once (before transformation) to collect information
//! needed by the transform pass: which imports come from `@qwik.dev/core`,
//! which of those are `$`-suffixed, where `$()` call sites appear, and what
//! the module exports. This is a read-only pass -- it does not mutate the AST.

use std::collections::HashSet;

use oxc::ast::ast::*;
use oxc::semantic::Scoping;

use crate::types::{CollectResult, DollarCallSite, ExportInfo, ImportInfo};
use crate::words;

/// Context for tracking nesting depth during recursive AST walk.
struct CollectContext {
    /// Set of dollar-suffixed imports from @qwik.dev/core.
    dollar_imports: HashSet<String>,
    /// All located dollar call sites.
    dollar_calls: Vec<DollarCallSite>,
    /// All import declarations.
    module_imports: Vec<ImportInfo>,
    /// All export declarations.
    module_exports: Vec<ExportInfo>,
    /// Current nesting depth inside $-calls (0 = top level).
    nesting_depth: u32,
    /// Parent call site's display name when nested.
    parent_display_name: Option<String>,
    /// Current variable name context (set when walking a variable declarator).
    current_var_name: Option<String>,
}

impl CollectContext {
    fn new() -> Self {
        Self {
            dollar_imports: HashSet::new(),
            dollar_calls: Vec::new(),
            module_imports: Vec::new(),
            module_exports: Vec::new(),
            nesting_depth: 0,
            parent_display_name: None,
            current_var_name: None,
        }
    }
}

/// Perform first-pass analysis of the parsed module.
///
/// Walks the program body to collect:
/// - Dollar-suffixed imports from `@qwik.dev/core`
/// - All import and export declarations
/// - All `$()` call sites with display names and nesting info
///
/// The `scoping` parameter is passed through for future capture analysis
/// (Phase 9). It is unused in the collector but kept in the signature for
/// API stability.
pub(crate) fn collect<'a>(
    program: &Program<'a>,
    _scoping: &Scoping,
) -> CollectResult {
    let mut ctx = CollectContext::new();

    // First pass: collect all imports (need dollar_imports before finding call sites)
    for stmt in &program.body {
        if let Statement::ImportDeclaration(import) = stmt {
            collect_import(&mut ctx, import);
        }
    }

    // Second pass: collect exports and dollar call sites
    for stmt in &program.body {
        match stmt {
            Statement::ImportDeclaration(_) => {
                // Already processed
            }
            Statement::ExportNamedDeclaration(export) => {
                collect_named_export(&mut ctx, export);
            }
            Statement::ExportDefaultDeclaration(export) => {
                collect_default_export(&mut ctx, export);
            }
            _ => {
                walk_statement_for_calls(&mut ctx, stmt);
            }
        }
    }

    CollectResult {
        dollar_imports: ctx.dollar_imports,
        dollar_calls: ctx.dollar_calls,
        module_imports: ctx.module_imports,
        module_exports: ctx.module_exports,
    }
}

/// Collect information from an import declaration.
fn collect_import(ctx: &mut CollectContext, import: &ImportDeclaration<'_>) {
    let source = import.source.value.as_str();
    let is_qwik_core = words::is_qwik_core_import(source);

    let mut specifiers_vec = Vec::new();

    if let Some(specifiers) = &import.specifiers {
        for spec in specifiers {
            match spec {
                ImportDeclarationSpecifier::ImportSpecifier(s) => {
                    let imported_name = match &s.imported {
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::StringLiteral(s) => s.value.as_str(),
                    };
                    // Use local name (what is actually used in the code)
                    let local_name = s.local.name.as_str();
                    specifiers_vec.push(local_name.to_string());

                    // Track dollar imports from Qwik core
                    if is_qwik_core
                        && (imported_name == "$" || imported_name.ends_with('$'))
                    {
                        // Insert the local name since that is what call sites will use
                        ctx.dollar_imports.insert(local_name.to_string());
                    }
                }
                ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                    specifiers_vec.push(s.local.name.as_str().to_string());
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                    specifiers_vec.push(s.local.name.as_str().to_string());
                }
            }
        }
    }

    ctx.module_imports.push(ImportInfo {
        source: source.to_string(),
        specifiers: specifiers_vec,
        is_qwik_core,
        span: (import.span.start, import.span.end),
    });
}

/// Collect information from a named export declaration.
fn collect_named_export(ctx: &mut CollectContext, export: &ExportNamedDeclaration<'_>) {
    // If export has a declaration (e.g., `export const Foo = ...`), get the names
    if let Some(decl) = &export.declaration {
        match decl {
            Declaration::VariableDeclaration(var_decl) => {
                for declarator in &var_decl.declarations {
                    if let Some(name) = binding_pattern_name(&declarator.id) {
                        ctx.module_exports.push(ExportInfo {
                            name: name.clone(),
                            is_reexport: false,
                            span: (export.span.start, export.span.end),
                        });

                        // Also walk the init for dollar call sites
                        ctx.current_var_name = Some(name);
                        if let Some(init) = &declarator.init {
                            walk_expression_for_calls(ctx, init);
                        }
                        ctx.current_var_name = None;
                    }
                }
            }
            Declaration::FunctionDeclaration(func) => {
                if let Some(id) = &func.id {
                    ctx.module_exports.push(ExportInfo {
                        name: id.name.as_str().to_string(),
                        is_reexport: false,
                        span: (export.span.start, export.span.end),
                    });
                }
            }
            Declaration::ClassDeclaration(class) => {
                if let Some(id) = &class.id {
                    ctx.module_exports.push(ExportInfo {
                        name: id.name.as_str().to_string(),
                        is_reexport: false,
                        span: (export.span.start, export.span.end),
                    });
                }
            }
            _ => {}
        }
    }

    // Handle re-exports: `export { Foo } from './module'`
    if export.source.is_some() {
        for spec in &export.specifiers {
            let name = match &spec.exported {
                ModuleExportName::IdentifierName(id) => id.name.as_str().to_string(),
                ModuleExportName::IdentifierReference(id) => id.name.as_str().to_string(),
                ModuleExportName::StringLiteral(s) => s.value.as_str().to_string(),
            };
            ctx.module_exports.push(ExportInfo {
                name,
                is_reexport: true,
                span: (export.span.start, export.span.end),
            });
        }
    }
}

/// Collect information from a default export declaration.
fn collect_default_export(ctx: &mut CollectContext, export: &ExportDefaultDeclaration<'_>) {
    ctx.module_exports.push(ExportInfo {
        name: "default".to_string(),
        is_reexport: false,
        span: (export.span.start, export.span.end),
    });

    // Walk the expression or declaration for dollar call sites
    match &export.declaration {
        ExportDefaultDeclarationKind::FunctionDeclaration(_)
        | ExportDefaultDeclarationKind::ClassDeclaration(_) => {}
        _ => {
            // For expressions, walk to find dollar calls
            if let Some(expr) = export.declaration.as_expression() {
                ctx.current_var_name = Some("default".to_string());
                walk_expression_for_calls(ctx, expr);
                ctx.current_var_name = None;
            }
        }
    }
}

/// Extract the first identifier name from a binding pattern.
fn binding_pattern_name(pattern: &BindingPattern<'_>) -> Option<String> {
    match pattern {
        BindingPattern::BindingIdentifier(id) => Some(id.name.as_str().to_string()),
        _ => None,
    }
}

/// Walk a statement to find dollar call sites.
fn walk_statement_for_calls(ctx: &mut CollectContext, stmt: &Statement<'_>) {
    match stmt {
        Statement::VariableDeclaration(var_decl) => {
            for declarator in &var_decl.declarations {
                let var_name = binding_pattern_name(&declarator.id);
                ctx.current_var_name = var_name;
                if let Some(init) = &declarator.init {
                    walk_expression_for_calls(ctx, init);
                }
                ctx.current_var_name = None;
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            walk_expression_for_calls(ctx, &expr_stmt.expression);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(arg) = &ret.argument {
                walk_expression_for_calls(ctx, arg);
            }
        }
        Statement::BlockStatement(block) => {
            for s in &block.body {
                walk_statement_for_calls(ctx, s);
            }
        }
        Statement::IfStatement(if_stmt) => {
            walk_expression_for_calls(ctx, &if_stmt.test);
            walk_statement_for_calls(ctx, &if_stmt.consequent);
            if let Some(alt) = &if_stmt.alternate {
                walk_statement_for_calls(ctx, alt);
            }
        }
        Statement::ExportNamedDeclaration(export) => {
            collect_named_export(ctx, export);
        }
        Statement::ExportDefaultDeclaration(export) => {
            collect_default_export(ctx, export);
        }
        _ => {}
    }
}

/// Walk an expression to find dollar call sites.
fn walk_expression_for_calls(ctx: &mut CollectContext, expr: &Expression<'_>) {
    match expr {
        Expression::CallExpression(call) => {
            // Check if this is a dollar call
            if let Expression::Identifier(ident) = &call.callee {
                let name = ident.name.as_str();
                if ctx.dollar_imports.contains(name) {
                    let display_name = derive_display_name(ctx, name);
                    let is_nested = ctx.nesting_depth > 0;
                    let parent_name = ctx.parent_display_name.clone();

                    ctx.dollar_calls.push(DollarCallSite {
                        callee_name: name.to_string(),
                        span: (call.span.start, call.span.end),
                        display_name: display_name.clone(),
                        is_nested,
                        parent_name,
                    });

                    // Walk arguments with increased nesting depth
                    let prev_parent = ctx.parent_display_name.take();
                    ctx.parent_display_name = Some(display_name);
                    ctx.nesting_depth += 1;

                    for arg in &call.arguments {
                        walk_argument_for_calls(ctx, arg);
                    }

                    ctx.nesting_depth -= 1;
                    ctx.parent_display_name = prev_parent;
                    return;
                }
            }

            // Not a dollar call -- walk callee and arguments normally
            walk_expression_for_calls(ctx, &call.callee);
            for arg in &call.arguments {
                walk_argument_for_calls(ctx, arg);
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            // Walk the arrow body
            for stmt in &arrow.body.statements {
                walk_statement_for_calls(ctx, stmt);
            }
        }
        Expression::FunctionExpression(func) => {
            if let Some(body) = &func.body {
                for stmt in &body.statements {
                    walk_statement_for_calls(ctx, stmt);
                }
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            walk_expression_for_calls(ctx, &paren.expression);
        }
        Expression::SequenceExpression(seq) => {
            for expr in &seq.expressions {
                walk_expression_for_calls(ctx, expr);
            }
        }
        Expression::ConditionalExpression(cond) => {
            walk_expression_for_calls(ctx, &cond.test);
            walk_expression_for_calls(ctx, &cond.consequent);
            walk_expression_for_calls(ctx, &cond.alternate);
        }
        Expression::AssignmentExpression(assign) => {
            walk_expression_for_calls(ctx, &assign.right);
        }
        Expression::LogicalExpression(logical) => {
            walk_expression_for_calls(ctx, &logical.left);
            walk_expression_for_calls(ctx, &logical.right);
        }
        Expression::BinaryExpression(binary) => {
            walk_expression_for_calls(ctx, &binary.left);
            walk_expression_for_calls(ctx, &binary.right);
        }
        Expression::ArrayExpression(arr) => {
            for elem in &arr.elements {
                match elem {
                    ArrayExpressionElement::SpreadElement(spread) => {
                        walk_expression_for_calls(ctx, &spread.argument);
                    }
                    ArrayExpressionElement::Elision(_) => {}
                    _ => {
                        if let Some(expr) = elem.as_expression() {
                            walk_expression_for_calls(ctx, expr);
                        }
                    }
                }
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        walk_expression_for_calls(ctx, &p.value);
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        walk_expression_for_calls(ctx, &spread.argument);
                    }
                }
            }
        }
        Expression::TemplateLiteral(tmpl) => {
            for expr in &tmpl.expressions {
                walk_expression_for_calls(ctx, expr);
            }
        }
        Expression::TaggedTemplateExpression(tagged) => {
            walk_expression_for_calls(ctx, &tagged.tag);
        }
        Expression::JSXElement(jsx) => {
            walk_jsx_element_for_calls(ctx, jsx);
        }
        Expression::JSXFragment(jsx) => {
            walk_jsx_children_for_calls(ctx, &jsx.children);
        }
        _ => {}
    }
}

/// Walk a call argument for dollar call sites.
fn walk_argument_for_calls(ctx: &mut CollectContext, arg: &Argument<'_>) {
    match arg {
        Argument::SpreadElement(spread) => {
            walk_expression_for_calls(ctx, &spread.argument);
        }
        _ => {
            if let Some(expr) = arg.as_expression() {
                walk_expression_for_calls(ctx, expr);
            }
        }
    }
}

/// Walk a JSXExpression to find dollar calls.
///
/// JSXExpression uses `inherit_variants!` from Expression in OXC 0.113,
/// meaning all Expression variants are directly on JSXExpression. We handle
/// call expressions specifically and delegate the rest.
fn walk_jsx_expression_for_calls(ctx: &mut CollectContext, jsx_expr: &JSXExpression<'_>) {
    match jsx_expr {
        JSXExpression::EmptyExpression(_) => {}
        // JSXExpression inherits all Expression variants via inherit_variants! macro.
        // CallExpression is the one we need to check for dollar calls.
        JSXExpression::CallExpression(call) => {
            if let Expression::Identifier(ident) = &call.callee {
                let name = ident.name.as_str();
                if ctx.dollar_imports.contains(name) {
                    let display_name = derive_display_name(ctx, name);
                    let is_nested = ctx.nesting_depth > 0;
                    let parent_name = ctx.parent_display_name.clone();

                    ctx.dollar_calls.push(DollarCallSite {
                        callee_name: name.to_string(),
                        span: (call.span.start, call.span.end),
                        display_name: display_name.clone(),
                        is_nested,
                        parent_name,
                    });

                    let prev_parent = ctx.parent_display_name.take();
                    ctx.parent_display_name = Some(display_name);
                    ctx.nesting_depth += 1;
                    for arg in &call.arguments {
                        walk_argument_for_calls(ctx, arg);
                    }
                    ctx.nesting_depth -= 1;
                    ctx.parent_display_name = prev_parent;
                    return;
                }
            }
            // Not a dollar call -- walk normally
            walk_expression_for_calls(ctx, &call.callee);
            for arg in &call.arguments {
                walk_argument_for_calls(ctx, arg);
            }
        }
        JSXExpression::ArrowFunctionExpression(arrow) => {
            for stmt in &arrow.body.statements {
                walk_statement_for_calls(ctx, stmt);
            }
        }
        JSXExpression::JSXElement(el) => {
            walk_jsx_element_for_calls(ctx, el);
        }
        JSXExpression::JSXFragment(frag) => {
            walk_jsx_children_for_calls(ctx, &frag.children);
        }
        // For all other inherited expression variants, we skip deep walking
        // since they rarely contain dollar calls in JSX attribute positions.
        _ => {}
    }
}

/// Walk JSX element and its children for dollar calls.
fn walk_jsx_element_for_calls(ctx: &mut CollectContext, element: &JSXElement<'_>) {
    // Walk attributes for dollar calls in attribute values
    for attr in &element.opening_element.attributes {
        if let JSXAttributeItem::Attribute(attr) = attr {
            if let Some(value) = &attr.value {
                if let JSXAttributeValue::ExpressionContainer(container) = value {
                    walk_jsx_expression_for_calls(ctx, &container.expression);
                }
            }
        }
    }
    // Walk children
    walk_jsx_children_for_calls(ctx, &element.children);
}

/// Walk JSX children for dollar calls.
fn walk_jsx_children_for_calls<'a>(ctx: &mut CollectContext, children: &oxc::allocator::Vec<'a, JSXChild<'a>>) {
    for child in children {
        match child {
            JSXChild::Element(el) => {
                walk_jsx_element_for_calls(ctx, el);
            }
            JSXChild::Fragment(frag) => {
                walk_jsx_children_for_calls(ctx, &frag.children);
            }
            JSXChild::ExpressionContainer(container) => {
                walk_jsx_expression_for_calls(ctx, &container.expression);
            }
            _ => {}
        }
    }
}

/// Derive the display name for a dollar call site from the lexical context.
///
/// The display name follows the pattern:
/// - For `const Foo = component$(() => ...)` -> `"Foo_component"`
/// - For `const bar = $(() => ...)` -> `"bar"`
/// - For nested `$()` inside another `$()` body -> uses parent context
/// - The filename prefix is NOT included here -- it's added during segment data construction.
fn derive_display_name(ctx: &CollectContext, callee_name: &str) -> String {
    let var_name = ctx.current_var_name.as_deref().unwrap_or("");

    // Strip the '$' from the callee for the suffix
    let callee_suffix = callee_name.strip_suffix('$').unwrap_or("");

    if var_name.is_empty() {
        // No variable context -- use callee as display name
        if callee_suffix.is_empty() {
            // Bare `$()` call without variable context
            "s_".to_string()
        } else {
            callee_suffix.to_string()
        }
    } else if callee_suffix.is_empty() {
        // Bare `$()` with a variable name
        var_name.to_string()
    } else {
        // Named callee like `component$`, `useTask$`
        format!("{var_name}_{callee_suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parse_module;
    use oxc::allocator::Allocator;

    /// Helper: parse source and run collector
    fn parse_and_collect(source: &str) -> CollectResult {
        let allocator = Allocator::default();
        let result = parse_module(&allocator, source, "test.tsx").expect("parse failed");
        collect(&result.program, &result.scoping)
    }

    #[test]
    fn test_collect_dollar_imports() {
        let result = parse_and_collect(
            r#"import { $, component$, useStore } from '@qwik.dev/core';"#,
        );

        assert!(result.dollar_imports.contains("$"));
        assert!(result.dollar_imports.contains("component$"));
        // useStore is not $-suffixed
        assert!(!result.dollar_imports.contains("useStore"));
        assert_eq!(result.dollar_imports.len(), 2);
    }

    #[test]
    fn test_collect_non_qwik_imports() {
        let result = parse_and_collect(
            r#"import { something$ } from './not-qwik';
import { $ } from '@qwik.dev/core';"#,
        );

        // Only $ from @qwik.dev/core should be in dollar_imports
        assert_eq!(result.dollar_imports.len(), 1);
        assert!(result.dollar_imports.contains("$"));

        // Both imports should be in module_imports
        assert_eq!(result.module_imports.len(), 2);
    }

    #[test]
    fn test_collect_call_sites_component() {
        let result = parse_and_collect(
            r#"import { component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return <div>Hello</div>;
});"#,
        );

        assert_eq!(result.dollar_calls.len(), 1);
        assert_eq!(result.dollar_calls[0].callee_name, "component$");
        assert_eq!(result.dollar_calls[0].display_name, "App_component");
        assert!(!result.dollar_calls[0].is_nested);
    }

    #[test]
    fn test_collect_call_sites_bare_dollar() {
        let result = parse_and_collect(
            r#"import { $ } from '@qwik.dev/core';
const handler = $(() => {
    console.log("hi");
});"#,
        );

        assert_eq!(result.dollar_calls.len(), 1);
        assert_eq!(result.dollar_calls[0].callee_name, "$");
        assert_eq!(result.dollar_calls[0].display_name, "handler");
    }

    #[test]
    fn test_collect_nested_dollar_calls() {
        let result = parse_and_collect(
            r#"import { $, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return $(() => {
        console.log("inner");
    });
});"#,
        );

        assert_eq!(result.dollar_calls.len(), 2);

        // The component$ call should be first (outer)
        let outer = &result.dollar_calls[0];
        assert_eq!(outer.callee_name, "component$");
        assert_eq!(outer.display_name, "App_component");
        assert!(!outer.is_nested);

        // The inner $() call should be nested
        let inner = &result.dollar_calls[1];
        assert_eq!(inner.callee_name, "$");
        assert!(inner.is_nested);
        assert_eq!(inner.parent_name.as_deref(), Some("App_component"));
    }

    #[test]
    fn test_collect_exports() {
        let result = parse_and_collect(
            r#"import { component$ } from '@qwik.dev/core';
export const App = component$(() => <div/>);
export function helper() { return 1; }"#,
        );

        assert_eq!(result.module_exports.len(), 2);
        let names: Vec<&str> = result.module_exports.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"App"));
        assert!(names.contains(&"helper"));
    }

    #[test]
    fn test_collect_module_imports_info() {
        let result = parse_and_collect(
            r#"import { $, component$, useStore } from '@qwik.dev/core';
import { thing } from './utils';"#,
        );

        assert_eq!(result.module_imports.len(), 2);

        let qwik_import = result.module_imports.iter().find(|i| i.is_qwik_core).unwrap();
        assert_eq!(qwik_import.source, "@qwik.dev/core");
        assert_eq!(qwik_import.specifiers.len(), 3);
        assert!(qwik_import.specifiers.contains(&"$".to_string()));
        assert!(qwik_import.specifiers.contains(&"component$".to_string()));
        assert!(qwik_import.specifiers.contains(&"useStore".to_string()));

        let other_import = result.module_imports.iter().find(|i| !i.is_qwik_core).unwrap();
        assert_eq!(other_import.source, "./utils");
    }

    #[test]
    fn test_collect_display_name_multiple_dollar_apis() {
        let result = parse_and_collect(
            r#"import { component$, useTask$, useStyles$ } from '@qwik.dev/core';
export const App = component$(() => {
    useTask$(() => {
        console.log("task");
    });
    useStyles$('div { color: red }');
    return <div/>;
});"#,
        );

        assert_eq!(result.dollar_calls.len(), 3);

        let component_call = result.dollar_calls.iter().find(|c| c.callee_name == "component$").unwrap();
        assert_eq!(component_call.display_name, "App_component");

        let task_call = result.dollar_calls.iter().find(|c| c.callee_name == "useTask$").unwrap();
        assert!(task_call.is_nested);

        let styles_call = result.dollar_calls.iter().find(|c| c.callee_name == "useStyles$").unwrap();
        assert!(styles_call.is_nested);
    }

    #[test]
    fn test_collect_example_1_pattern() {
        // From spec example_1.md: has $() and component() wrapping $()
        let result = parse_and_collect(
            r#"import { $, component, onRender } from '@qwik.dev/core';

export const renderHeader = $(() => {
	return (
		<div onClick={$((ctx) => console.log(ctx))}/>
	);
});
const renderHeader2 = component($(() => {
	console.log("mount");
	return render;
}));"#,
        );

        // Only '$' is dollar-suffixed (component and onRender don't end with $)
        assert_eq!(result.dollar_imports.len(), 1);
        assert!(result.dollar_imports.contains("$"));

        // Should find 3 $() call sites
        assert_eq!(
            result.dollar_calls.len(),
            3,
            "Expected 3 $-call sites, found {}: {:?}",
            result.dollar_calls.len(),
            result.dollar_calls.iter().map(|c| &c.callee_name).collect::<Vec<_>>()
        );

        // All should be '$' callee
        for call in &result.dollar_calls {
            assert_eq!(call.callee_name, "$");
        }
    }

    #[test]
    fn test_collect_spans() {
        let result = parse_and_collect(
            r#"import { $ } from '@qwik.dev/core';
const x = $(() => 1);"#,
        );

        assert_eq!(result.dollar_calls.len(), 1);
        let (start, end) = result.dollar_calls[0].span;
        assert!(start < end, "Span start ({}) should be less than end ({})", start, end);
        assert!(start > 0, "Span start should be > 0 (not at beginning of file)");
    }
}
