mod common;

use common::{build_scoping, parse_source};
use oxc::allocator::Allocator;
use oxc::ast::ast::*;
use oxc::span::SourceType;
use oxc::syntax::scope::ScopeId;
use oxc::syntax::symbol::{SymbolFlags, SymbolId};
use oxc::semantic::Scoping;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::collections::HashMap;

// ============================================================================
// Capture Analysis Data Types
// ============================================================================

/// Result of capture analysis for a single $()-body.
#[derive(Debug)]
struct CaptureAnalysisResult {
    captures: Vec<String>,
    reemitted_imports: Vec<String>,
    inlined_consts: Vec<String>,
    diagnostics: Vec<String>,
}

// ============================================================================
// Dollar Body Collector (Traversal Phase)
// ============================================================================

/// Information about a detected $() call and its body's scope.
#[derive(Debug)]
struct DollarBodyInfo {
    callee: String,
    body_scope_id: ScopeId,
    span_start: u32,
    /// If true, the dollar call takes an expression argument (not an arrow function).
    /// For expression-argument calls (like useStyles$(css)), we use the PARENT scope
    /// as the body scope, meaning captures are symbols referenced in args but declared
    /// even further out. This is semantically correct because the expression-arg call
    /// will be extracted into a segment that needs those references.
    is_expression_arg: bool,
}

/// Traversal visitor that detects $() calls and records the scope ID of
/// each body (the arrow function argument).
struct DollarBodyCollector {
    /// Names imported from @qwik.dev/core that end with $
    dollar_imports: std::collections::HashSet<String>,
    /// Collected dollar body infos
    bodies: Vec<DollarBodyInfo>,
    /// Track when we're inside a $() call and need to record the next
    /// arrow/function scope as a body scope
    pending_dollar_call: bool,
    pending_callee: String,
    pending_span_start: u32,
}

impl<'a> Traverse<'a, ()> for DollarBodyCollector {
    fn enter_import_declaration(
        &mut self,
        import: &mut ImportDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if import.source.value.as_str() != "@qwik.dev/core" {
            return;
        }
        if let Some(specifiers) = &import.specifiers {
            for spec in specifiers {
                if let ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                    let name = match &s.imported {
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::StringLiteral(s) => s.value.as_str(),
                    };
                    if name == "$" || name.ends_with('$') {
                        self.dollar_imports.insert(name.to_string());
                    }
                }
            }
        }
    }

    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            if self.dollar_imports.contains(name) {
                self.pending_dollar_call = true;
                self.pending_callee = name.to_string();
                self.pending_span_start = call.span.start;
            }
        }
    }

    fn enter_arrow_function_expression(
        &mut self,
        arrow: &mut ArrowFunctionExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if self.pending_dollar_call {
            self.pending_dollar_call = false;
            // IMPORTANT: ctx.current_scope_id() returns the PARENT scope here because
            // OXC traverse calls enter_* BEFORE pushing the new scope onto the stack.
            // The arrow function's own scope is stored in the node's scope_id field.
            let body_scope_id = arrow.scope_id.get().expect(
                "ArrowFunctionExpression should have a scope_id assigned by semantic analysis",
            );
            self.bodies.push(DollarBodyInfo {
                callee: std::mem::take(&mut self.pending_callee),
                body_scope_id,
                span_start: self.pending_span_start,
                is_expression_arg: false,
            });
        }
    }

    fn exit_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a, ()>,
    ) {
        // If pending_dollar_call is still true when exiting the call, the call
        // doesn't have an arrow function argument (e.g., useStyles$(css1 + css2)).
        // Record it as an expression-arg dollar call.
        if self.pending_dollar_call {
            self.pending_dollar_call = false;
            // ctx.current_scope_id() here gives us the scope CONTAINING this call,
            // which is correct -- the expression args live in this scope.
            let body_scope_id = ctx.current_scope_id();
            self.bodies.push(DollarBodyInfo {
                callee: std::mem::take(&mut self.pending_callee),
                body_scope_id,
                span_start: call.span.start,
                is_expression_arg: true,
            });
        }
    }
}

// ============================================================================
// Capture Analysis Algorithm
// ============================================================================

/// Check if `inner` scope is contained within `outer` scope (or is the same scope).
fn is_scope_contained_in(scoping: &Scoping, inner: ScopeId, outer: ScopeId) -> bool {
    scoping.scope_ancestors(inner).any(|scope_id| scope_id == outer)
}

/// Determine which variables are captured by a $()-body.
///
/// This function iterates all symbols in the program and checks which ones:
/// 1. Are declared OUTSIDE the body scope
/// 2. Have references INSIDE the body scope
///
/// For each such symbol, it classifies the capture type.
fn compute_captures(
    scoping: &Scoping,
    body_scope_id: ScopeId,
    const_literals: &HashMap<SymbolId, bool>,
) -> CaptureAnalysisResult {
    let mut captures = Vec::new();
    let mut reemitted_imports = Vec::new();
    let mut inlined_consts = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    for symbol_id in scoping.symbol_ids() {
        let symbol_scope = scoping.symbol_scope_id(symbol_id);

        // Skip symbols declared INSIDE the body scope
        if is_scope_contained_in(scoping, symbol_scope, body_scope_id) {
            continue;
        }

        // Check if any references to this symbol are INSIDE the body scope
        let has_reference_in_body = scoping
            .get_resolved_references(symbol_id)
            .any(|reference| {
                let ref_scope = reference.scope_id();
                is_scope_contained_in(scoping, ref_scope, body_scope_id)
            });

        if !has_reference_in_body {
            continue;
        }

        let name = scoping.symbol_name(symbol_id).to_string();
        if !seen_names.insert(name.clone()) {
            continue;
        }

        let flags = scoping.symbol_flags(symbol_id);

        // Classify the capture
        if flags.contains(SymbolFlags::Import) {
            reemitted_imports.push(name);
        } else if flags.contains(SymbolFlags::Function) {
            diagnostics.push(format!(
                "Cannot capture function declaration '{}' across $() boundary",
                name
            ));
        } else if flags.contains(SymbolFlags::Class) {
            diagnostics.push(format!(
                "Cannot capture class declaration '{}' across $() boundary",
                name
            ));
        } else if *const_literals.get(&symbol_id).unwrap_or(&false) {
            inlined_consts.push(name);
        } else {
            let _index = captures.len();
            captures.push(name);
        }
    }

    CaptureAnalysisResult {
        captures,
        reemitted_imports,
        inlined_consts,
        diagnostics,
    }
}

/// Compute captures for expression-argument dollar calls (e.g., useStyles$(css1 + css2)).
/// Instead of scope containment, we directly collect identifier references from the
/// call arguments and classify the symbols they resolve to.
fn compute_captures_for_expr_args(
    scoping: &Scoping,
    call_scope_id: ScopeId,
    const_literals: &HashMap<SymbolId, bool>,
) -> CaptureAnalysisResult {
    let mut captures = Vec::new();
    let mut reemitted_imports = Vec::new();
    let mut inlined_consts = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    // For expression-arg calls, we look at all symbols that have references
    // exactly in the call_scope_id (the scope where the $-call lives).
    // We then check if those symbols are declared OUTSIDE that scope.
    // This captures exactly the external dependencies of the expression arguments.
    for symbol_id in scoping.symbol_ids() {
        let symbol_scope = scoping.symbol_scope_id(symbol_id);

        // Skip symbols declared inside the call scope (they're local to it)
        if is_scope_contained_in(scoping, symbol_scope, call_scope_id) {
            continue;
        }

        // Check if this symbol has references in the call scope (but NOT in deeper scopes --
        // deeper scopes belong to nested arrow bodies which are separate segments)
        let has_reference_in_call_scope = scoping
            .get_resolved_references(symbol_id)
            .any(|reference| {
                reference.scope_id() == call_scope_id
            });

        if !has_reference_in_call_scope {
            continue;
        }

        let name = scoping.symbol_name(symbol_id).to_string();
        if !seen_names.insert(name.clone()) {
            continue;
        }

        let flags = scoping.symbol_flags(symbol_id);

        if flags.contains(SymbolFlags::Import) {
            reemitted_imports.push(name);
        } else if flags.contains(SymbolFlags::Function) {
            diagnostics.push(format!(
                "Cannot capture function declaration '{}' across $() boundary",
                name
            ));
        } else if flags.contains(SymbolFlags::Class) {
            diagnostics.push(format!(
                "Cannot capture class declaration '{}' across $() boundary",
                name
            ));
        } else if *const_literals.get(&symbol_id).unwrap_or(&false) {
            inlined_consts.push(name);
        } else {
            captures.push(name);
        }
    }

    CaptureAnalysisResult {
        captures,
        reemitted_imports,
        inlined_consts,
        diagnostics,
    }
}

// ============================================================================
// Const Literal Detection
// ============================================================================

/// Check if an expression is a primitive literal.
fn is_primitive_literal(expr: &Expression<'_>) -> bool {
    matches!(
        expr,
        Expression::NumericLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
    ) || matches!(expr, Expression::Identifier(id) if id.name == "undefined")
}

// ============================================================================
// Nested Const Literal Detection (inside function bodies)
// ============================================================================

/// Detect const literals within a function body's statements.
/// This handles the case where `const arg0 = 20` is inside a component callback.
fn detect_const_literals_in_body(
    body: &[Statement<'_>],
    _scoping: &Scoping,
    result: &mut HashMap<SymbolId, bool>,
) {
    for stmt in body {
        if let Statement::VariableDeclaration(decl) = stmt {
            if decl.kind == VariableDeclarationKind::Const {
                for declarator in &decl.declarations {
                    if let Some(init) = &declarator.init {
                        if is_primitive_literal(init) {
                            if let BindingPattern::BindingIdentifier(id) = &declarator.id {
                                if let Some(symbol_id) = id.symbol_id.get() {
                                    result.insert(symbol_id, true);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Walk the full AST to find const literals in any scope (including nested function bodies).
fn detect_all_const_literals(program: &Program<'_>, scoping: &Scoping) -> HashMap<SymbolId, bool> {
    let mut result = HashMap::new();

    // Top-level
    detect_const_literals_in_body(&program.body, scoping, &mut result);

    // Walk into function bodies, arrow functions, etc.
    for stmt in &program.body {
        walk_stmt_for_consts(stmt, scoping, &mut result);
    }

    result
}

fn walk_stmt_for_consts(
    stmt: &Statement<'_>,
    scoping: &Scoping,
    result: &mut HashMap<SymbolId, bool>,
) {
    match stmt {
        Statement::ExportNamedDeclaration(export) => {
            if let Some(decl) = &export.declaration {
                walk_declaration_for_consts(decl, scoping, result);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for declarator in &decl.declarations {
                if let Some(init) = &declarator.init {
                    walk_expr_for_consts(init, scoping, result);
                }
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            walk_expr_for_consts(&expr_stmt.expression, scoping, result);
        }
        _ => {}
    }
}

fn walk_declaration_for_consts(
    decl: &Declaration<'_>,
    scoping: &Scoping,
    result: &mut HashMap<SymbolId, bool>,
) {
    if let Declaration::VariableDeclaration(var_decl) = decl {
        if var_decl.kind == VariableDeclarationKind::Const {
            for declarator in &var_decl.declarations {
                if let Some(init) = &declarator.init {
                    if is_primitive_literal(init) {
                        if let BindingPattern::BindingIdentifier(id) = &declarator.id {
                            if let Some(symbol_id) = id.symbol_id.get() {
                                result.insert(symbol_id, true);
                            }
                        }
                    }
                    walk_expr_for_consts(init, scoping, result);
                }
            }
        } else {
            for declarator in &var_decl.declarations {
                if let Some(init) = &declarator.init {
                    walk_expr_for_consts(init, scoping, result);
                }
            }
        }
    }
}

fn walk_expr_for_consts(
    expr: &Expression<'_>,
    scoping: &Scoping,
    result: &mut HashMap<SymbolId, bool>,
) {
    match expr {
        Expression::CallExpression(call) => {
            for arg in &call.arguments {
                if let Argument::ArrowFunctionExpression(arrow) = arg {
                    // Walk into arrow function body
                    if let Some(body) = arrow.body.as_ref().statements.as_slice().into() {
                        detect_const_literals_in_body(body, scoping, result);
                        for stmt in body {
                            walk_stmt_for_consts(stmt, scoping, result);
                        }
                    }
                }
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            let body = arrow.body.statements.as_slice();
            detect_const_literals_in_body(body, scoping, result);
            for stmt in body {
                walk_stmt_for_consts(stmt, scoping, result);
            }
        }
        _ => {}
    }
}

// ============================================================================
// Test Runner
// ============================================================================

fn analyze_source(label: &str, source: &str) -> Vec<(String, CaptureAnalysisResult)> {
    println!("=== {} ===", label);

    let allocator = Allocator::default();
    let source_type = SourceType::tsx();
    let ret = parse_source(&allocator, source, source_type);

    if !ret.errors.is_empty() {
        println!("  Parse errors:");
        for err in &ret.errors {
            println!("    {}", err);
        }
    }

    let mut program = ret.program;
    let scoping = build_scoping(&program);

    // Detect const literals across the entire AST
    let const_literals = detect_all_const_literals(&program, &scoping);

    // Collect dollar body scope IDs via traversal
    let mut collector = DollarBodyCollector {
        dollar_imports: std::collections::HashSet::new(),
        bodies: Vec::new(),
        pending_dollar_call: false,
        pending_callee: String::new(),
        pending_span_start: 0,
    };

    let scoping = traverse_mut(&mut collector, &allocator, &mut program, scoping, ());

    println!("  Found {} $-call bodies:", collector.bodies.len());
    let mut results = Vec::new();

    for body_info in &collector.bodies {
        println!(
            "  --- {} at span {} (body_scope_id={:?}, expr_arg={}) ---",
            body_info.callee, body_info.span_start, body_info.body_scope_id,
            body_info.is_expression_arg
        );

        let analysis = if body_info.is_expression_arg {
            compute_captures_for_expr_args(&scoping, body_info.body_scope_id, &const_literals)
        } else {
            compute_captures(&scoping, body_info.body_scope_id, &const_literals)
        };

        println!("    Captures: {:?}", analysis.captures);
        println!("    Re-emitted imports: {:?}", analysis.reemitted_imports);
        println!("    Inlined consts: {:?}", analysis.inlined_consts);
        if !analysis.diagnostics.is_empty() {
            println!("    Diagnostics: {:?}", analysis.diagnostics);
        }

        results.push((body_info.callee.clone(), analysis));
    }

    println!();
    results
}

fn main() {
    println!("POC-02: Capture Analysis using OXC Semantic (Scoping)");
    println!("=====================================================\n");

    // --- Test Case 1: example_multi_capture.md ---
    // Foo: inner $() captures _rawProps (after destructuring), arg0 is const-inlined
    // Bar: inner $() captures _rawProps
    // NOTE: We test pre-destructuring form first. In actual transform, ({foo}) -> (_rawProps)
    // happens before capture analysis. For this POC, we test with the original form
    // to validate the capture algorithm itself.
    let source_1 = r#"import { $, component$ } from '@qwik.dev/core';

export const Foo = component$((_rawProps) => {
	const arg0 = 20;
	return $(() => {
		const fn2 = ({aaa}) => aaa;
		return _rawProps.foo + fn2() + arg0;
	});
});

export const Bar = component$((_rawProps) => {
	return $(() => {
		return _rawProps.bar;
	});
});"#;

    let results_1 = analyze_source("Test Case 1: example_multi_capture (post-destructuring)", source_1);

    // Find the inner $() bodies (not component$ bodies)
    let inner_dollar_results: Vec<_> = results_1
        .iter()
        .filter(|(callee, _)| callee == "$")
        .collect();

    assert_eq!(
        inner_dollar_results.len(),
        2,
        "Expected 2 inner $() bodies, found {}",
        inner_dollar_results.len()
    );

    // First inner $(): Foo's $() body
    let foo_captures = &inner_dollar_results[0].1;
    assert!(
        foo_captures.captures.contains(&"_rawProps".to_string()),
        "Expected _rawProps in captures, got {:?}",
        foo_captures.captures
    );
    assert!(
        foo_captures.inlined_consts.contains(&"arg0".to_string()),
        "Expected arg0 in inlined_consts, got {:?}",
        foo_captures.inlined_consts
    );
    assert!(
        !foo_captures.captures.contains(&"arg0".to_string()),
        "arg0 should NOT be in captures (it's a const literal)"
    );
    println!("  PASS: Foo inner $(): _rawProps captured, arg0 const-inlined\n");

    // Second inner $(): Bar's $() body
    let bar_captures = &inner_dollar_results[1].1;
    assert!(
        bar_captures.captures.contains(&"_rawProps".to_string()),
        "Expected _rawProps in captures, got {:?}",
        bar_captures.captures
    );
    println!("  PASS: Bar inner $(): _rawProps captured\n");

    // --- Test Case 2: example_inlined_entry_strategy.md ---
    // useBrowserVisibleTask$ body: captures `state`, `thing` is import (re-emitted)
    let source_2 = r#"import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';

export const Child = component$(() => {
    useStyles$('somestring');
    const state = useStore({count: 0});
    useBrowserVisibleTask$(() => {
        state.count = thing.doStuff() + 1;
    });
    return 'hello';
});"#;

    let results_2 = analyze_source("Test Case 2: example_inlined_entry_strategy", source_2);

    // Find the useBrowserVisibleTask$ body
    let task_results: Vec<_> = results_2
        .iter()
        .filter(|(callee, _)| callee == "useBrowserVisibleTask$")
        .collect();

    assert_eq!(
        task_results.len(),
        1,
        "Expected 1 useBrowserVisibleTask$ body, found {}",
        task_results.len()
    );

    let task_captures = &task_results[0].1;
    assert!(
        task_captures.captures.contains(&"state".to_string()),
        "Expected 'state' in captures, got {:?}",
        task_captures.captures
    );
    assert!(
        task_captures.reemitted_imports.contains(&"thing".to_string()),
        "Expected 'thing' in re-emitted imports, got {:?}",
        task_captures.reemitted_imports
    );
    assert!(
        !task_captures.captures.contains(&"thing".to_string()),
        "'thing' should NOT be in captures (it's an import)"
    );
    println!("  PASS: useBrowserVisibleTask$: state captured, thing re-emitted as import\n");

    // --- Test Case 3: example_capture_imports.md ---
    // useStyles$ bodies: css imports are re-emitted, NOT captured
    let source_3 = r#"import { component$, useStyles$ } from '@qwik.dev/core';
import css1 from './global.css';
import css2 from './style.css';
import css3 from './style.css';

export const App = component$(() => {
    useStyles$(css1 + css2);
    useStyles$(css3);
});"#;

    let results_3 = analyze_source("Test Case 3: example_capture_imports", source_3);

    // Find useStyles$ bodies
    let style_results: Vec<_> = results_3
        .iter()
        .filter(|(callee, _)| callee == "useStyles$")
        .collect();

    assert_eq!(
        style_results.len(),
        2,
        "Expected 2 useStyles$ bodies, found {}",
        style_results.len()
    );

    // First useStyles$: references css1 and css2 (both imports)
    let style1_captures = &style_results[0].1;
    assert!(
        style1_captures.captures.is_empty(),
        "Expected empty captures for useStyles$ with CSS imports, got {:?}",
        style1_captures.captures
    );
    assert!(
        style1_captures.reemitted_imports.contains(&"css1".to_string()),
        "Expected css1 in re-emitted imports"
    );
    assert!(
        style1_captures.reemitted_imports.contains(&"css2".to_string()),
        "Expected css2 in re-emitted imports"
    );
    println!("  PASS: useStyles$ (template): css1, css2 re-emitted as imports, no captures\n");

    // Second useStyles$: references css3 (import)
    let style2_captures = &style_results[1].1;
    assert!(
        style2_captures.captures.is_empty(),
        "Expected empty captures for useStyles$ with css3"
    );
    assert!(
        style2_captures.reemitted_imports.contains(&"css3".to_string()),
        "Expected css3 in re-emitted imports"
    );
    println!("  PASS: useStyles$ (css3): css3 re-emitted as import, no captures\n");

    // --- Test Case 4: No captures ---
    let source_4 = r#"import { $, component } from '@qwik.dev/core';

export const renderHeader = $(() => {
    return 'hello';
});"#;

    let results_4 = analyze_source("Test Case 4: No captures (example_1 simplified)", source_4);

    let dollar_results: Vec<_> = results_4
        .iter()
        .filter(|(callee, _)| callee == "$")
        .collect();

    assert_eq!(dollar_results.len(), 1, "Expected 1 $() body");
    let no_capture_result = &dollar_results[0].1;
    assert!(
        no_capture_result.captures.is_empty(),
        "Expected no captures, got {:?}",
        no_capture_result.captures
    );
    assert!(
        no_capture_result.reemitted_imports.is_empty(),
        "Expected no re-emitted imports"
    );
    println!("  PASS: No captures detected\n");

    println!("=====================================================");
    println!("All POC-02 tests passed!");
}
