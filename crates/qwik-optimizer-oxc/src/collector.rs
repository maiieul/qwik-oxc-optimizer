//! First-pass AST analysis.
//!
//! Walk the parsed AST once (before transformation) to collect information
//! needed by the transform pass: which imports come from `@qwik.dev/core`,
//! which of those are `$`-suffixed, where `$()` call sites appear, and what
//! the module exports. This is a read-only pass -- it does not mutate the AST.
//!
//! Also provides `compute_captures()` for capture analysis: given a set of
//! identifier names referenced inside a $()-body and a set of names declared
//! locally in that body, classify each outer reference as LocalCapture,
//! ImportReemit, or skipped (global / framework import).

use std::collections::{HashMap, HashSet};

use oxc::ast::ast::*;
use oxc::semantic::Scoping;

use crate::types::{CollectResult, DollarCallSite, ExportInfo, ImportInfo};
use crate::words;

// ---------------------------------------------------------------------------
// Capture Analysis
// ---------------------------------------------------------------------------

/// Result of capture analysis for a single $()-body.
#[derive(Debug, Clone)]
pub(crate) struct CaptureAnalysisResult {
    /// Variable names that will be passed via _captures[] at runtime.
    /// Order matches encounter order from the body traversal.
    pub capture_names: Vec<String>,

    /// Import bindings referenced in the body that should be re-emitted
    /// in the segment module (NOT captured). Each entry is (local_name, source, is_default).
    pub reemitted_imports: Vec<(String, String, bool)>,

    /// Diagnostic messages for invalid captures (function/class declarations).
    pub diagnostics: Vec<String>,
}

/// A well-known global identifier that should never be treated as a capture.
pub(crate) const KNOWN_GLOBALS: &[&str] = &[
    "console",
    "undefined",
    "NaN",
    "Infinity",
    "window",
    "document",
    "globalThis",
    "self",
    "navigator",
    "location",
    "history",
    "localStorage",
    "sessionStorage",
    "fetch",
    "setTimeout",
    "setInterval",
    "clearTimeout",
    "clearInterval",
    "requestAnimationFrame",
    "cancelAnimationFrame",
    "queueMicrotask",
    "Promise",
    "Array",
    "Object",
    "String",
    "Number",
    "Boolean",
    "Symbol",
    "Map",
    "Set",
    "WeakMap",
    "WeakSet",
    "Date",
    "RegExp",
    "Error",
    "TypeError",
    "RangeError",
    "JSON",
    "Math",
    "parseInt",
    "parseFloat",
    "isNaN",
    "isFinite",
    "encodeURIComponent",
    "decodeURIComponent",
    "encodeURI",
    "decodeURI",
    "atob",
    "btoa",
    "structuredClone",
    "URL",
    "URLSearchParams",
    "Headers",
    "Request",
    "Response",
    "FormData",
    "Blob",
    "File",
    "TextEncoder",
    "TextDecoder",
    "AbortController",
    "AbortSignal",
    "Event",
    "CustomEvent",
    "EventTarget",
    "crypto",
    "performance",
    "alert",
    "confirm",
    "prompt",
    "import",
    "require",
    "module",
    "exports",
    "__dirname",
    "__filename",
    "process",
    "Buffer",
    "global",
    "arguments",
    "this",
    "super",
    "new",
    "true",
    "false",
    "null",
];

/// Determine which variables are captured by a $()-body.
///
/// This is the simplified/pragmatic approach for Phase 9: rather than using the
/// full OXC Scoping API (which is consumed by `traverse_mut`), we receive:
///
/// - `body_ident_refs`: All IdentifierReference names seen inside the $()-body
///   (collected during Traverse via `enter_identifier_reference`)
/// - `body_local_decls`: Names declared locally inside the $()-body (parameters,
///   let/const/var declarations) -- these are NOT captures
/// - `collect_result`: The collector result with import data
///
/// For each unique name in `body_ident_refs` that is NOT in `body_local_decls`:
/// - If it's a known global -> skip
/// - If it's in the module's imports -> classify as ImportReemit (NOT captured)
/// - If it's a $-suffixed import from @qwik.dev/core -> skip (framework, handled by QRL rewriting)
/// - Otherwise -> LocalCapture (add to capture_names)
///
/// Encounter order from the body traversal is preserved in `body_ident_refs`.
pub(crate) fn compute_captures(
    body_ident_refs: &[String],
    body_local_decls: &HashSet<String>,
    collect_result: &CollectResult,
) -> CaptureAnalysisResult {
    let mut capture_names = Vec::new();
    let mut reemitted_imports: Vec<(String, String, bool)> = Vec::new();
    let diagnostics = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for name in body_ident_refs {
        // Skip if already processed
        if !seen.insert(name.clone()) {
            continue;
        }

        // Skip if declared locally in the body
        if body_local_decls.contains(name) {
            continue;
        }

        // Skip known globals
        if KNOWN_GLOBALS.contains(&name.as_str()) {
            continue;
        }

        // Skip $-suffixed framework imports (handled by QRL rewriting)
        if collect_result.dollar_imports.contains(name) {
            continue;
        }

        // Check if it's a module import
        let mut is_import = false;
        for import_info in &collect_result.module_imports {
            if import_info.specifiers.contains(name) {
                // It's an import -- classify as re-emitted import, NOT captured
                let is_default = import_info.specifiers.len() == 1
                    && import_info.specifiers[0] == *name
                    && !import_info.is_qwik_core;
                reemitted_imports.push((
                    name.clone(),
                    import_info.source.clone(),
                    is_default,
                ));
                is_import = true;
                break;
            }
        }
        if is_import {
            continue;
        }

        // Not a global, not an import, not body-local -> it's a capture
        capture_names.push(name.clone());
    }

    CaptureAnalysisResult {
        capture_names,
        reemitted_imports,
        diagnostics,
    }
}

/// Context for tracking nesting depth during recursive AST walk.
struct CollectContext {
    /// Set of dollar-suffixed imports from @qwik.dev/core (or custom core_module).
    /// Contains LOCAL names (which may be aliases).
    dollar_imports: HashSet<String>,
    /// Alias map: local_name -> original_imported_name for $-suffixed imports.
    /// Only populated when the local name differs from the imported name.
    alias_map: HashMap<String, String>,
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
    /// The core module import path(s) to recognize as Qwik imports.
    /// Always includes "@qwik.dev/core"; may also include a custom core_module.
    core_modules: Vec<String>,
}

impl CollectContext {
    fn new(core_module: Option<&str>) -> Self {
        let mut core_modules = vec!["@qwik.dev/core".to_string()];
        if let Some(cm) = core_module {
            if cm != "@qwik.dev/core" {
                core_modules.push(cm.to_string());
            }
        }
        // Also recognize legacy @builder.io/qwik
        if !core_modules.iter().any(|m| m == "@builder.io/qwik") {
            core_modules.push("@builder.io/qwik".to_string());
        }
        Self {
            dollar_imports: HashSet::new(),
            alias_map: HashMap::new(),
            dollar_calls: Vec::new(),
            module_imports: Vec::new(),
            module_exports: Vec::new(),
            nesting_depth: 0,
            parent_display_name: None,
            current_var_name: None,
            core_modules,
        }
    }

    /// Check if an import source is a recognized Qwik module.
    ///
    /// Returns true for:
    /// - `@qwik.dev/core` (default)
    /// - `@qwik.dev/*` packages (e.g., `@qwik.dev/react`)
    /// - `@builder.io/qwik` and `@builder.io/qwik-*` (legacy)
    /// - Any custom core_module specified in config
    ///
    /// Sub-paths like `@qwik.dev/core/build` or `@qwik.dev/core/jsx-runtime`
    /// are NOT treated as core imports (they don't re-export $-APIs).
    fn is_qwik_core_import(&self, source: &str) -> bool {
        // Check exact matches from config (core_module, @qwik.dev/core, @builder.io/qwik)
        if self.core_modules.iter().any(|m| m == source) {
            return true;
        }
        // Check @qwik.dev/* packages (excluding sub-paths like /core/build)
        // e.g., "@qwik.dev/react" matches but "@qwik.dev/core/jsx-runtime" does not
        if source.starts_with("@qwik.dev/") {
            let after_scope = &source["@qwik.dev/".len()..];
            // Must be a simple package name (no slash)
            return !after_scope.contains('/');
        }
        // Check @builder.io/qwik-* packages (legacy)
        // e.g., "@builder.io/qwik-react" matches but "@builder.io/qwik/build" does not
        if source.starts_with("@builder.io/qwik-") {
            return true;
        }
        false
    }
}

/// Perform first-pass analysis of the parsed module.
///
/// Walks the program body to collect:
/// - Dollar-suffixed imports from `@qwik.dev/core` (or custom core_module)
/// - All import and export declarations
/// - All `$()` call sites with display names and nesting info
/// - Alias mappings for renamed $-suffixed imports
///
/// The `scoping` parameter is passed through for future capture analysis
/// (Phase 9). It is unused in the collector but kept in the signature for
/// API stability.
///
/// The `core_module` parameter allows recognizing imports from a custom
/// module (e.g., `@qwik.dev/react`, `@builder.io/qwik`) as Qwik core
/// imports in addition to the default `@qwik.dev/core`.
pub(crate) fn collect<'a>(
    program: &Program<'a>,
    _scoping: &Scoping,
    core_module: Option<&str>,
) -> CollectResult {
    let mut ctx = CollectContext::new(core_module);

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
        alias_map: ctx.alias_map,
        dollar_calls: ctx.dollar_calls,
        module_imports: ctx.module_imports,
        module_exports: ctx.module_exports,
    }
}

/// Collect information from an import declaration.
fn collect_import(ctx: &mut CollectContext, import: &ImportDeclaration<'_>) {
    let source = import.source.value.as_str();
    let is_qwik_core = ctx.is_qwik_core_import(source);

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

                    // Track dollar imports: recognize $-suffixed functions from any module.
                    // The SWC optimizer treats all $-suffixed imports as segment
                    // boundaries, including from non-core modules like @auth/qwik.
                    if imported_name == "$" || imported_name.ends_with('$') {
                        // Insert the local name since that is what call sites will use
                        ctx.dollar_imports.insert(local_name.to_string());

                        // If the local name differs from the imported name, record the alias
                        if local_name != imported_name {
                            ctx.alias_map
                                .insert(local_name.to_string(), imported_name.to_string());
                        }
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

                        // Track locally-defined $-suffixed exports as dollar imports
                        // ONLY when defined via wrap() or implicit$FirstArg() calls.
                        if name.ends_with('$') && !ctx.dollar_imports.contains(&name) {
                            if let Some(init) = &declarator.init {
                                if is_wrap_call(init) {
                                    ctx.dollar_imports.insert(name.clone());
                                }
                            }
                        }

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

/// Check if an expression is a `wrap(...)` or `implicit$FirstArg(...)` call.
/// These are the Qwik conventions for creating custom $-APIs from Qrl variants.
fn is_wrap_call(expr: &Expression<'_>) -> bool {
    if let Expression::CallExpression(call) = expr {
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            return name == "wrap" || name == "implicit$FirstArg";
        }
    }
    false
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
                // Track locally-defined $-suffixed variables as dollar imports
                // ONLY when defined via wrap() or implicit$FirstArg() calls.
                // These are the Qwik conventions for creating custom $-APIs.
                if let Some(ref name) = var_name {
                    if name.ends_with('$') && !ctx.dollar_imports.contains(name) {
                        if let Some(init) = &declarator.init {
                            if is_wrap_call(init) {
                                ctx.dollar_imports.insert(name.clone());
                            }
                        }
                    }
                }
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
                    // Resolve alias: use the original imported name for callee_name and display_name
                    let original_name = ctx
                        .alias_map
                        .get(name)
                        .map(|s| s.as_str())
                        .unwrap_or(name);
                    let display_name = derive_display_name(ctx, original_name);
                    let is_nested = ctx.nesting_depth > 0;
                    let parent_name = ctx.parent_display_name.clone();

                    ctx.dollar_calls.push(DollarCallSite {
                        callee_name: original_name.to_string(),
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
                    // Resolve alias for callee_name and display_name
                    let original_name = ctx
                        .alias_map
                        .get(name)
                        .map(|s| s.as_str())
                        .unwrap_or(name);
                    let display_name = derive_display_name(ctx, original_name);
                    let is_nested = ctx.nesting_depth > 0;
                    let parent_name = ctx.parent_display_name.clone();

                    ctx.dollar_calls.push(DollarCallSite {
                        callee_name: original_name.to_string(),
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
    // Determine the element/component name for display name context
    let element_name = match &element.opening_element.name {
        JSXElementName::Identifier(ident) => Some(ident.name.as_str().to_string()),
        JSXElementName::NamespacedName(ns) => {
            Some(format!("{}_{}", ns.namespace.name, ns.name.name))
        }
        JSXElementName::MemberExpression(_) => None,
        _ => None,
    };

    // Walk attributes for dollar calls in attribute values
    for attr in &element.opening_element.attributes {
        if let JSXAttributeItem::Attribute(attr) = attr {
            let attr_name = match &attr.name {
                JSXAttributeName::Identifier(ident) => ident.name.as_str(),
                JSXAttributeName::NamespacedName(ns) => {
                    // For namespace:name pattern, we don't treat as $-call
                    // Walk the value for nested calls
                    if let Some(value) = &attr.value {
                        if let JSXAttributeValue::ExpressionContainer(container) = value {
                            walk_jsx_expression_for_calls(ctx, &container.expression);
                        }
                    }
                    continue;
                }
            };

            // Check if this is a $-suffixed attribute (event handler or custom $ prop)
            if attr_name.ends_with('$') {
                if let Some(value) = &attr.value {
                    if let JSXAttributeValue::ExpressionContainer(container) = value {
                        // Get the expression span for the DollarCallSite
                        let expr_span = match &container.expression {
                            JSXExpression::EmptyExpression(_) => None,
                            _ => {
                                // Get span from the expression inside the container
                                get_jsx_expression_span(&container.expression)
                            }
                        };

                        if let Some((start, end)) = expr_span {
                            // Build display name: parent_context + element_name + attr_name_transformed
                            let event_suffix = transform_attr_name_for_display(attr_name);
                            let display_name = derive_jsx_event_display_name(
                                ctx, element_name.as_deref(), &event_suffix,
                            );
                            let is_nested = ctx.nesting_depth > 0;
                            let parent_name = ctx.parent_display_name.clone();

                            ctx.dollar_calls.push(DollarCallSite {
                                callee_name: attr_name.to_string(),
                                span: (start, end),
                                display_name: display_name.clone(),
                                is_nested,
                                parent_name,
                            });

                            // Walk the value expression for nested dollar calls
                            let prev_parent = ctx.parent_display_name.take();
                            ctx.parent_display_name = Some(display_name);
                            ctx.nesting_depth += 1;
                            walk_jsx_expression_for_calls(ctx, &container.expression);
                            ctx.nesting_depth -= 1;
                            ctx.parent_display_name = prev_parent;
                            continue;
                        }
                    }
                }
            }

            // Non-$ attribute: walk value for nested dollar calls normally
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

/// Get the span of a JSXExpression.
fn get_jsx_expression_span(expr: &JSXExpression<'_>) -> Option<(u32, u32)> {
    match expr {
        JSXExpression::EmptyExpression(_) => None,
        JSXExpression::ArrowFunctionExpression(arrow) => Some((arrow.span.start, arrow.span.end)),
        JSXExpression::FunctionExpression(func) => Some((func.span.start, func.span.end)),
        JSXExpression::CallExpression(call) => Some((call.span.start, call.span.end)),
        JSXExpression::Identifier(ident) => Some((ident.span.start, ident.span.end)),
        _ => {
            // For other inherited expression variants, try to extract span
            // from the expression type. Many expression variants have a span field.
            None
        }
    }
}

/// Transform a JSX attribute name to a display name suffix.
///
/// - `onClick$` -> `q_e_click`
/// - `onInput$` -> `q_e_input`
/// - `render$` -> `render`
/// - `shouldRemove$` -> `shouldRemove`
fn transform_attr_name_for_display(attr_name: &str) -> String {
    // Strip trailing $
    let base = attr_name.strip_suffix('$').unwrap_or(attr_name);

    // Handle onX -> q_e_x pattern
    if base.starts_with("on") && base.len() > 2 {
        let event_part = &base[2..]; // Everything after "on"
        // Convert camelCase to lowercase
        format!("q_e_{}", event_part.to_lowercase())
    } else {
        base.to_string()
    }
}

/// Derive display name for a JSX event handler.
fn derive_jsx_event_display_name(
    ctx: &CollectContext,
    element_name: Option<&str>,
    event_suffix: &str,
) -> String {
    let parent_ctx = ctx.parent_display_name.as_deref()
        .or(ctx.current_var_name.as_deref())
        .unwrap_or("");

    let elem = element_name.unwrap_or("_");

    if parent_ctx.is_empty() {
        format!("{}_{}", elem, event_suffix)
    } else {
        format!("{}_{}_{}", parent_ctx, elem, event_suffix)
    }
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
        let (result, _diags) = parse_module(&allocator, source, "test.tsx").expect("parse failed");
        collect(&result.program, &result.scoping, None)
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

        // All $-suffixed imports are recognized as dollar imports,
        // regardless of source module (SWC behavior: any $-suffixed
        // function call creates a segment boundary).
        assert_eq!(result.dollar_imports.len(), 2);
        assert!(result.dollar_imports.contains("$"));
        assert!(result.dollar_imports.contains("something$"));

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

    // -----------------------------------------------------------------------
    // Capture Analysis Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_compute_captures_no_captures() {
        // $() body with only local vars and globals -> empty captures
        let collect_result = parse_and_collect(
            r#"import { $ } from '@qwik.dev/core';
const handler = $(() => {
    const x = 1;
    console.log(x);
});"#,
        );

        let body_ident_refs = vec![
            "x".to_string(),
            "console".to_string(),
        ];
        let body_local_decls: HashSet<String> = ["x".to_string()].into_iter().collect();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert!(result.capture_names.is_empty(), "Expected no captures, got {:?}", result.capture_names);
        assert!(result.reemitted_imports.is_empty());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_compute_captures_state_variable() {
        // $() body referencing outer `state` -> captures=["state"]
        let collect_result = parse_and_collect(
            r#"import { $, component$, useStore } from '@qwik.dev/core';
export const App = component$(() => {
    const state = useStore({count: 0});
    return $(() => state.count);
});"#,
        );

        // Simulating the inner $() body that references "state"
        let body_ident_refs = vec!["state".to_string()];
        let body_local_decls: HashSet<String> = HashSet::new();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert_eq!(result.capture_names, vec!["state".to_string()]);
        assert!(result.reemitted_imports.is_empty());
    }

    #[test]
    fn test_compute_captures_import_not_captured() {
        // $() body referencing imported `useStore` -> not in captures, is in reemitted_imports
        let collect_result = parse_and_collect(
            r#"import { $, component$, useStore } from '@qwik.dev/core';
import { thing } from './utils';
export const App = component$(() => {
    return $(() => {
        thing.doStuff();
        useStore({});
    });
});"#,
        );

        // The inner $() body references "thing" (import) and "useStore" (qwik core import)
        let body_ident_refs = vec![
            "thing".to_string(),
            "useStore".to_string(),
        ];
        let body_local_decls: HashSet<String> = HashSet::new();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        // "thing" is an import -> reemitted, not captured
        // "useStore" is a qwik core import (specifier on a qwik import) -> reemitted, not captured
        assert!(result.capture_names.is_empty(), "Expected no captures, got {:?}", result.capture_names);

        // "thing" should be in reemitted_imports
        assert!(
            result.reemitted_imports.iter().any(|(name, _, _)| name == "thing"),
            "Expected 'thing' in reemitted_imports, got {:?}",
            result.reemitted_imports
        );
        // "useStore" should also be in reemitted_imports (it's in the qwik core import specifiers)
        assert!(
            result.reemitted_imports.iter().any(|(name, _, _)| name == "useStore"),
            "Expected 'useStore' in reemitted_imports, got {:?}",
            result.reemitted_imports
        );
    }

    #[test]
    fn test_compute_captures_dollar_import_skipped() {
        // $-suffixed imports (framework) should be skipped entirely
        let collect_result = parse_and_collect(
            r#"import { $, component$ } from '@qwik.dev/core';"#,
        );

        let body_ident_refs = vec![
            "$".to_string(),
            "component$".to_string(),
        ];
        let body_local_decls: HashSet<String> = HashSet::new();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert!(result.capture_names.is_empty());
        assert!(result.reemitted_imports.is_empty());
    }

    #[test]
    fn test_compute_captures_mixed() {
        // Mix of captures, imports, and body-locals
        let collect_result = parse_and_collect(
            r#"import { $, component$ } from '@qwik.dev/core';
import { thing } from './sibling';"#,
        );

        let body_ident_refs = vec![
            "state".to_string(),       // outer variable -> capture
            "count".to_string(),       // outer variable -> capture
            "thing".to_string(),       // import -> reemitted
            "localVar".to_string(),    // body-local -> skip
            "console".to_string(),     // global -> skip
            "$".to_string(),           // dollar import -> skip
        ];
        let body_local_decls: HashSet<String> = ["localVar".to_string()].into_iter().collect();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert_eq!(result.capture_names, vec!["state".to_string(), "count".to_string()]);
        assert_eq!(result.reemitted_imports.len(), 1);
        assert_eq!(result.reemitted_imports[0].0, "thing");
    }

    #[test]
    fn test_compute_captures_deduplication() {
        // Same name referenced multiple times -> only counted once
        let collect_result = parse_and_collect(
            r#"import { $ } from '@qwik.dev/core';"#,
        );

        let body_ident_refs = vec![
            "state".to_string(),
            "state".to_string(),
            "state".to_string(),
        ];
        let body_local_decls: HashSet<String> = HashSet::new();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert_eq!(result.capture_names.len(), 1);
        assert_eq!(result.capture_names[0], "state");
    }

    #[test]
    fn test_compute_captures_rawprops() {
        // After props destructuring, _rawProps is referenced in inner $() -> captured
        let collect_result = parse_and_collect(
            r#"import { $, component$ } from '@qwik.dev/core';"#,
        );

        let body_ident_refs = vec!["_rawProps".to_string()];
        let body_local_decls: HashSet<String> = HashSet::new();

        let result = compute_captures(&body_ident_refs, &body_local_decls, &collect_result);
        assert_eq!(result.capture_names, vec!["_rawProps".to_string()]);
    }

    // -----------------------------------------------------------------------
    // Import Alias Detection Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_collect_alias_detection() {
        // component$ as Component, $ as onRender
        let result = parse_and_collect(
            r#"import { component$ as Component, $ as onRender, useStore } from '@qwik.dev/core';"#,
        );

        // Both aliased imports should be in dollar_imports (under local names)
        assert!(result.dollar_imports.contains("Component"), "Component should be in dollar_imports");
        assert!(result.dollar_imports.contains("onRender"), "onRender should be in dollar_imports");
        assert!(!result.dollar_imports.contains("component$"), "original name should NOT be in dollar_imports");
        assert!(!result.dollar_imports.contains("$"), "original '$' should NOT be in dollar_imports");
        assert_eq!(result.dollar_imports.len(), 2);

        // Alias map should record the mapping
        assert_eq!(result.alias_map.get("Component").map(|s| s.as_str()), Some("component$"));
        assert_eq!(result.alias_map.get("onRender").map(|s| s.as_str()), Some("$"));
        assert_eq!(result.alias_map.len(), 2);
    }

    #[test]
    fn test_collect_alias_call_sites() {
        // Aliased imports used in call sites should have original names as callee_name
        let result = parse_and_collect(
            r#"import { component$ as Component, $ as onRender } from '@qwik.dev/core';
export const App = Component(() => {
    return onRender(() => "hello");
});"#,
        );

        assert_eq!(result.dollar_calls.len(), 2);

        // Component call should resolve to component$
        let component_call = result.dollar_calls.iter().find(|c| c.display_name.contains("component")).unwrap();
        assert_eq!(component_call.callee_name, "component$");

        // onRender call should resolve to $
        let dollar_call = result.dollar_calls.iter().find(|c| c.callee_name == "$").unwrap();
        assert_eq!(dollar_call.callee_name, "$");
    }

    #[test]
    fn test_collect_no_alias_when_same_name() {
        // No alias when local == imported
        let result = parse_and_collect(
            r#"import { component$, $ } from '@qwik.dev/core';"#,
        );

        assert!(result.alias_map.is_empty(), "No aliases when names match");
        assert!(result.dollar_imports.contains("$"));
        assert!(result.dollar_imports.contains("component$"));
    }

    // -----------------------------------------------------------------------
    // Custom Core Module Tests
    // -----------------------------------------------------------------------

    fn parse_and_collect_with_core_module(source: &str, core_module: Option<&str>) -> CollectResult {
        let allocator = Allocator::default();
        let (result, _diags) = parse_module(&allocator, source, "test.tsx").expect("parse failed");
        collect(&result.program, &result.scoping, core_module)
    }

    #[test]
    fn test_collect_builder_io_qwik_legacy() {
        // Legacy @builder.io/qwik imports should be recognized
        let result = parse_and_collect(
            r#"import { $, component$ } from '@builder.io/qwik';"#,
        );

        assert!(result.dollar_imports.contains("$"));
        assert!(result.dollar_imports.contains("component$"));
        assert_eq!(result.dollar_imports.len(), 2);
    }

    #[test]
    fn test_collect_builder_io_qwik_react() {
        // @builder.io/qwik-react should be recognized for $-suffixed imports
        let result = parse_and_collect(
            r#"import { qwikify$ } from '@builder.io/qwik-react';
import { component$ } from '@builder.io/qwik';"#,
        );

        assert!(result.dollar_imports.contains("qwikify$"));
        assert!(result.dollar_imports.contains("component$"));
        assert_eq!(result.dollar_imports.len(), 2);
    }

    #[test]
    fn test_collect_qwik_dev_react() {
        // @qwik.dev/react should be recognized for $-suffixed imports
        let result = parse_and_collect(
            r#"import { qwikify$ } from '@qwik.dev/react';"#,
        );

        assert!(result.dollar_imports.contains("qwikify$"));
        assert_eq!(result.dollar_imports.len(), 1);
    }

    #[test]
    fn test_collect_custom_core_module() {
        // Custom core_module should be recognized
        let result = parse_and_collect_with_core_module(
            r#"import { myThing$ } from '@my/custom-framework';"#,
            Some("@my/custom-framework"),
        );

        assert!(result.dollar_imports.contains("myThing$"));
        assert_eq!(result.dollar_imports.len(), 1);
    }

    #[test]
    fn test_collect_qwik_core_subpath_not_recognized() {
        // Sub-paths like @qwik.dev/core/build should NOT be recognized
        // (they export build constants, not $-APIs)
        let result = parse_and_collect(
            r#"import { isDev } from '@qwik.dev/core/build';"#,
        );

        assert!(result.dollar_imports.is_empty());
    }
}
