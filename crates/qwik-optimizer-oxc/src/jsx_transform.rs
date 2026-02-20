//! JSX transformation: convert JSX elements/fragments into _jsxSorted/_jsxSplit function calls.
//!
//! This module contains all the pure JSX transformation logic extracted from transform.rs.
//! Functions here operate on AST nodes and ImportTracker without needing QwikTransform state.

use oxc::ast::ast::*;
use oxc::span::SPAN;
use oxc_traverse::TraverseCtx;

use crate::import_rewrite;
use crate::transform::ImportTracker;

/// Get the span of a JSXExpression's inner expression, but ONLY for arrow/function
/// expressions that represent inline lambda bodies needing segment extraction.
///
/// Identifier references (e.g., `onClick$={handler}`) are skipped because the
/// referenced binding is already extracted elsewhere. Call expressions (e.g.,
/// `onClick$={sync$(...)}`) are skipped because they're handled by enter_call_expression.
pub(crate) fn get_jsx_lambda_span(expr: &JSXExpression<'_>) -> Option<(u32, u32)> {
    match expr {
        JSXExpression::ArrowFunctionExpression(arrow) => Some((arrow.span.start, arrow.span.end)),
        JSXExpression::FunctionExpression(func) => Some((func.span.start, func.span.end)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// JSX Transformation Helpers
// ---------------------------------------------------------------------------

/// Normalize JSX text: collapse whitespace, strip leading/trailing newlines.
/// Returns empty string for whitespace-only text.
fn normalize_jsx_text(raw: &str) -> String {
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

    if attr_name.starts_with("on") && attr_name.ends_with('$') {
        let event_part = &attr_name[2..attr_name.len() - 1];

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

            if prop_name == "value" {
                if let Expression::Identifier(ident) = &member.object {
                    let _name = ident.name.as_str();
                    return SignalWrapResult::WrapPropSignal;
                }
            }

            if let Expression::Identifier(ident) = &member.object {
                if ident.name.as_str() == "_rawProps" && prop_name != "value" {
                    return SignalWrapResult::WrapPropNamed(prop_name.to_string());
                }
            }

            SignalWrapResult::None
        }
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

/// A reactive dependency found in an expression.
#[derive(Debug, Clone)]
struct ReactiveDep {
    /// The root identifier name (e.g., "signal", "store", "_rawProps").
    root_name: String,
    /// The parameter name assigned (e.g., "p0", "p1").
    param_name: String,
}

/// Check if an expression contains any function calls (makes it non-wrappable).
fn contains_function_call(expr: &Expression<'_>) -> bool {
    match expr {
        Expression::CallExpression(_) => true,
        Expression::BinaryExpression(bin) => {
            contains_function_call(&bin.left) || contains_function_call(&bin.right)
        }
        Expression::ConditionalExpression(cond) => {
            contains_function_call(&cond.test)
                || contains_function_call(&cond.consequent)
                || contains_function_call(&cond.alternate)
        }
        Expression::UnaryExpression(unary) => contains_function_call(&unary.argument),
        Expression::ObjectExpression(obj) => obj.properties.iter().any(|prop| match prop {
            ObjectPropertyKind::ObjectProperty(p) => contains_function_call(&p.value),
            ObjectPropertyKind::SpreadProperty(s) => contains_function_call(&s.argument),
        }),
        Expression::ArrayExpression(arr) => arr.elements.iter().any(|elem| match elem {
            ArrayExpressionElement::SpreadElement(s) => contains_function_call(&s.argument),
            ArrayExpressionElement::Elision(_) => false,
            _ => false, // array element literals can't contain calls
        }),
        Expression::ParenthesizedExpression(paren) => contains_function_call(&paren.expression),
        Expression::StaticMemberExpression(mem) => contains_function_call(&mem.object),
        Expression::ComputedMemberExpression(mem) => {
            contains_function_call(&mem.object) || contains_function_call(&mem.expression)
        }
        _ => false,
    }
}

/// Collect reactive dependency root identifiers from an expression.
///
/// A reactive source is:
/// - An identifier whose `.value` is accessed (signal pattern: `signal.value`)
/// - An identifier that is the root of a multi-level property chain (store pattern: `store.address.city`)
/// - `_rawProps` identifier in a member expression (`_rawProps.propName`)
/// - A destructured prop identifier
///
/// Returns the list of unique reactive deps and whether the expression
/// contains any non-reactive non-const sub-expressions (which would prevent wrapping).
fn collect_reactive_deps(
    expr: &Expression<'_>,
    destructured_props: Option<&[(String, String)]>,
    collected_imports: &[crate::types::ImportInfo],
) -> (Vec<ReactiveDep>, bool) {
    let mut deps: Vec<ReactiveDep> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut has_non_reactive_non_const = false;

    collect_reactive_deps_inner(
        expr,
        destructured_props,
        collected_imports,
        &mut deps,
        &mut seen,
        &mut has_non_reactive_non_const,
    );

    (deps, has_non_reactive_non_const)
}

fn collect_reactive_deps_inner(
    expr: &Expression<'_>,
    destructured_props: Option<&[(String, String)]>,
    collected_imports: &[crate::types::ImportInfo],
    deps: &mut Vec<ReactiveDep>,
    seen: &mut std::collections::HashSet<String>,
    has_non_reactive_non_const: &mut bool,
) {
    match expr {
        // signal.value -> signal is a reactive dep
        Expression::StaticMemberExpression(member) => {
            let prop = member.property.name.as_str();

            let root = get_root_identifier(&member.object);

            if prop == "value" {
                if let Some(root_name) = &root {
                    if !seen.contains(root_name.as_str()) {
                        let param = format!("p{}", deps.len());
                        seen.insert(root_name.clone());
                        deps.push(ReactiveDep {
                            root_name: root_name.clone(),
                            param_name: param,
                        });
                    }
                    return; // Don't recurse further
                }
            }

            if let Some(root_name) = &root {
                if root_name == "_rawProps" {
                    if !seen.contains(root_name.as_str()) {
                        let param = format!("p{}", deps.len());
                        seen.insert(root_name.clone());
                        deps.push(ReactiveDep {
                            root_name: root_name.clone(),
                            param_name: param,
                        });
                    }
                    return;
                }

                if is_imported_identifier(root_name, collected_imports) {
                    // Imports act like "side effects" in SWC: they prevent
                    // _fnSignal wrapping when mixed with reactive deps.
                    *has_non_reactive_non_const = true;
                    return;
                }

                if crate::collector::KNOWN_GLOBALS.contains(root_name.as_str()) {
                    *has_non_reactive_non_const = true;
                    return;
                }

                if has_chain_depth(expr, 2) {
                    if !seen.contains(root_name.as_str()) {
                        let param = format!("p{}", deps.len());
                        seen.insert(root_name.clone());
                        deps.push(ReactiveDep {
                            root_name: root_name.clone(),
                            param_name: param,
                        });
                    }
                    return;
                }
            }

            collect_reactive_deps_inner(
                &member.object,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
        }

        Expression::Identifier(ident) => {
            let name = ident.name.as_str();

            if let Some(props) = destructured_props {
                for (local_alias, _original_key) in props {
                    if local_alias == name {
                        if !seen.contains("_rawProps") {
                            let param = format!("p{}", deps.len());
                            seen.insert("_rawProps".to_string());
                            deps.push(ReactiveDep {
                                root_name: "_rawProps".to_string(),
                                param_name: param,
                            });
                        }
                        return;
                    }
                }
            }

            if is_imported_identifier(name, collected_imports) {
                // Imports act like "side effects" in SWC: they prevent
                // _fnSignal wrapping when mixed with reactive deps.
                *has_non_reactive_non_const = true;
                return;
            }

            if crate::collector::KNOWN_GLOBALS.contains(name) {
                *has_non_reactive_non_const = true;
                return;
            }

            // Unknown identifier (not a prop, import, or global) -- prevents
            // _fnSignal wrapping when mixed with reactive deps (SWC aborts
            // inlining for unknown identifiers).
            *has_non_reactive_non_const = true;
        }

        Expression::BinaryExpression(bin) => {
            collect_reactive_deps_inner(
                &bin.left,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
            collect_reactive_deps_inner(
                &bin.right,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
        }
        Expression::ConditionalExpression(cond) => {
            collect_reactive_deps_inner(
                &cond.test,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
            collect_reactive_deps_inner(
                &cond.consequent,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
            collect_reactive_deps_inner(
                &cond.alternate,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
        }
        Expression::UnaryExpression(unary) => {
            collect_reactive_deps_inner(
                &unary.argument,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
        }
        Expression::ObjectExpression(obj) => {
            for prop in &obj.properties {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        collect_reactive_deps_inner(
                            &p.value,
                            destructured_props,
                            collected_imports,
                            deps,
                            seen,
                            has_non_reactive_non_const,
                        );
                    }
                    ObjectPropertyKind::SpreadProperty(s) => {
                        collect_reactive_deps_inner(
                            &s.argument,
                            destructured_props,
                            collected_imports,
                            deps,
                            seen,
                            has_non_reactive_non_const,
                        );
                    }
                }
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            collect_reactive_deps_inner(
                &paren.expression,
                destructured_props,
                collected_imports,
                deps,
                seen,
                has_non_reactive_non_const,
            );
        }

        _ => {}
    }
}

/// Get the root identifier name from a (possibly chained) member expression.
pub(crate) fn get_root_identifier(expr: &Expression<'_>) -> Option<String> {
    match expr {
        Expression::Identifier(ident) => Some(ident.name.as_str().to_string()),
        Expression::StaticMemberExpression(member) => get_root_identifier(&member.object),
        Expression::ComputedMemberExpression(member) => get_root_identifier(&member.object),
        _ => None,
    }
}

/// Check if a member expression chain has at least `min_depth` levels.
fn has_chain_depth(expr: &Expression<'_>, min_depth: usize) -> bool {
    fn depth(expr: &Expression<'_>) -> usize {
        match expr {
            Expression::StaticMemberExpression(member) => 1 + depth(&member.object),
            Expression::ComputedMemberExpression(member) => 1 + depth(&member.object),
            _ => 0,
        }
    }
    depth(expr) >= min_depth
}

/// Check if an identifier is an imported name.
fn is_imported_identifier(name: &str, imports: &[crate::types::ImportInfo]) -> bool {
    imports
        .iter()
        .any(|imp| imp.specifiers.iter().any(|spec| spec == name))
}

/// Build an _fnSignal call and hoisted function declarations.
///
/// Returns (replacement_expression, fn_code, str_code) where fn_code and str_code
/// are string representations of the hoisted const declarations to insert at module level.
fn build_fn_signal_wrapping<'a>(
    expr: Expression<'a>,
    deps: &[ReactiveDep],
    destructured_props: Option<&[(String, String)]>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
) -> (Expression<'a>, String, String) {
    let hf_index = tracker.hoisted_fn_counter;
    tracker.hoisted_fn_counter += 1;

    let hf_name = format!("_hf{}", hf_index);
    let hf_str_name = format!("_hf{}_str", hf_index);

    let mut codegen = oxc::codegen::Codegen::new();
    codegen.print_expression(&expr);
    let mut body_str = codegen.into_source_text();

    for dep in deps {
        if dep.root_name == "_rawProps" {
            if let Some(props) = destructured_props {
                for (local_alias, original_key) in props {
                    body_str = replace_identifier_in_code(
                        &body_str,
                        local_alias,
                        &format!("{}.{}", dep.param_name, original_key),
                    );
                }
            }
            body_str = replace_identifier_in_code(&body_str, "_rawProps", &dep.param_name);
        } else {
            body_str = replace_identifier_in_code(&body_str, &dep.root_name, &dep.param_name);
        }
    }

    let params_str = deps
        .iter()
        .map(|d| d.param_name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let body_for_fn = if body_str.starts_with('{') {
        format!("({})", body_str)
    } else {
        body_str.clone()
    };

    let fn_code = format!("const {} = ({}) => {};", hf_name, params_str, body_for_fn);

    let minified = minify_expression_string(&body_str);
    let str_code = format!(
        "const {} = \"{}\";",
        hf_str_name,
        escape_string_literal(&minified)
    );

    let callee = ctx.ast.expression_identifier(SPAN, "_fnSignal");
    let mut arguments = ctx.ast.vec_with_capacity(3);

    let hf_atom = ctx.ast.atom(&hf_name);
    arguments.push(Argument::from(ctx.ast.expression_identifier(SPAN, hf_atom)));

    let mut dep_elements = ctx.ast.vec_with_capacity(deps.len());
    for dep in deps {
        let dep_atom = ctx.ast.atom(&dep.root_name);
        dep_elements.push(ArrayExpressionElement::from(
            ctx.ast.expression_identifier(SPAN, dep_atom),
        ));
    }
    arguments.push(Argument::from(ctx.ast.expression_array(SPAN, dep_elements)));

    let str_atom = ctx.ast.atom(&hf_str_name);
    arguments.push(Argument::from(
        ctx.ast.expression_identifier(SPAN, str_atom),
    ));

    let fn_signal_call = ctx.ast.expression_call(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
    );

    (fn_signal_call, fn_code, str_code)
}

/// Replace an identifier in code with a replacement string, being aware of word boundaries.
fn replace_identifier_in_code(code: &str, old_name: &str, new_name: &str) -> String {
    let mut result = String::with_capacity(code.len());
    let chars: Vec<char> = code.chars().collect();
    let old_chars: Vec<char> = old_name.chars().collect();
    let old_len = old_chars.len();
    let mut i = 0;

    while i < chars.len() {
        if i + old_len <= chars.len() && &chars[i..i + old_len] == old_chars.as_slice() {
            // Check word boundaries
            let before_ok = i == 0 || !is_ident_char(chars[i - 1]);
            let after_ok = i + old_len >= chars.len() || !is_ident_char(chars[i + old_len]);

            if before_ok && after_ok {
                result.push_str(new_name);
                i += old_len;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }

    result
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

/// Simple minification: remove unnecessary whitespace.
pub(crate) fn minify_expression_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut in_string = false;
    let mut string_char = '"';
    let mut prev_was_space = false;

    for c in s.chars() {
        if in_string {
            result.push(c);
            if c == string_char {
                in_string = false;
            }
            continue;
        }

        if c == '"' || c == '\'' || c == '`' {
            in_string = true;
            string_char = c;
            prev_was_space = false;
            result.push(c);
            continue;
        }

        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            if !prev_was_space && !result.is_empty() {
                let last = result.chars().last().unwrap_or(' ');
                if is_ident_char(last) {
                    prev_was_space = true;
                    // Defer the space -- only add if next char is also ident-like
                }
            }
            continue;
        }

        if prev_was_space && is_ident_char(c) {
            result.push(' ');
        }
        prev_was_space = false;
        result.push(c);
    }

    result
}

/// Escape a string for use inside a double-quoted JS string literal.
pub(crate) fn escape_string_literal(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Extract the identifier name from an expression (if it's a simple identifier).
pub(crate) fn extract_identifier_name(expr: &Expression<'_>) -> String {
    match expr {
        Expression::Identifier(ident) => ident.name.as_str().to_string(),
        _ => {
            let mut codegen = oxc::codegen::Codegen::new();
            codegen.print_expression(expr);
            codegen.into_source_text()
        }
    }
}

/// Build an inlinedQrl event handler for bind: directives.
///
/// Produces: `inlinedQrl(_handler, "_handler", [signal])`
/// where _handler is _val or _chk, and signal is the bound signal identifier.
pub(crate) fn build_bind_event_handler<'a>(
    handler_name: &str,
    signal_name: &str,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier(SPAN, "inlinedQrl");

    let handler_atom = ctx.ast.atom(handler_name);
    let handler_str_atom = ctx.ast.atom(handler_name);
    let signal_atom = ctx.ast.atom(signal_name);

    let mut arguments = ctx.ast.vec_with_capacity(3);

    arguments.push(Argument::from(
        ctx.ast.expression_identifier(SPAN, handler_atom),
    ));

    arguments.push(Argument::from(ctx.ast.expression_string_literal(
        SPAN,
        handler_str_atom,
        None,
    )));

    let mut elements = ctx.ast.vec_with_capacity(1);
    elements.push(ArrayExpressionElement::from(
        ctx.ast.expression_identifier(SPAN, signal_atom),
    ));
    arguments.push(Argument::from(ctx.ast.expression_array(SPAN, elements)));

    ctx.ast.expression_call(
        SPAN,
        callee,
        None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
        arguments,
        false,
    )
}

/// Build the tag expression for a JSX element name.
pub(crate) fn build_tag_expression<'a>(
    name: &JSXElementName<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) -> Expression<'a> {
    match name {
        JSXElementName::Identifier(ident) => {
            let tag_name = ident.name.as_str();
            if tag_name.chars().next().map_or(false, |c| c.is_lowercase()) {
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
pub(crate) fn build_jsx_member_expr<'a>(
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
    Expression::StaticMemberExpression(
        ctx.ast
            .alloc_static_member_expression(SPAN, object, property, false),
    )
}

/// Convert a JSXExpression to an Expression. JSXExpression uses inherit_variants!
/// from Expression, so all Expression variants appear directly on JSXExpression.
pub(crate) fn jsx_expression_to_expression<'a>(
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
    module_imports: &[crate::types::ImportInfo],
    hoisted_stmts: &mut Vec<(String, String)>,
) -> Expression<'a> {
    match value {
        JSXAttributeValue::StringLiteral(lit) => Expression::StringLiteral(lit),
        JSXAttributeValue::ExpressionContainer(container) => {
            jsx_expression_to_expression(container.unbox().expression, ctx)
        }
        JSXAttributeValue::Element(el) => {
            // JSX element as attribute value: transform it
            transform_jsx_element_inner(
                el.unbox(),
                tracker,
                ctx,
                destructured_props,
                module_imports,
                hoisted_stmts,
            )
        }
        JSXAttributeValue::Fragment(frag) => transform_jsx_fragment_inner(
            frag.unbox(),
            tracker,
            ctx,
            destructured_props,
            module_imports,
            hoisted_stmts,
        ),
    }
}

/// Recursively transform a JSXElement into a _jsxSorted/_jsxSplit call expression.
/// When `tracker.custom_jsx_source` is Some, uses the simpler React-style `_jsx("tag", {props})`
/// form instead of Qwik's `_jsxSorted`.
pub(crate) fn transform_jsx_element_inner<'a>(
    mut element: JSXElement<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
    module_imports: &[crate::types::ImportInfo],
    hoisted_stmts: &mut Vec<(String, String)>,
) -> Expression<'a> {
    // When a custom JSX import source is set (e.g., React), use the standard
    // JSX runtime transform: _jsx("tag", {props}) instead of Qwik's _jsxSorted.
    if tracker.custom_jsx_source.is_some() {
        return transform_jsx_element_custom_source(
            element, tracker, ctx, hoisted_stmts, module_imports, destructured_props,
        );
    }

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
                        key_value = Some(jsx_attr_value_to_expression(
                            val,
                            tracker,
                            ctx,
                            destructured_props,
                            module_imports,
                            hoisted_stmts,
                        ));
                    }
                    continue;
                }

                // Check for event handler attributes
                if let Some(event_name) = transform_event_attr_name(&attr_name) {
                    // Event handler: value goes into const props with renamed key
                    let value = if let Some(val) = attr.value {
                        jsx_attr_value_to_expression(
                            val,
                            tracker,
                            ctx,
                            destructured_props,
                            module_imports,
                            hoisted_stmts,
                        )
                    } else {
                        ctx.ast.expression_boolean_literal(SPAN, true)
                    };
                    const_props.push((event_name, value));
                    continue;
                }

                // Check for host: prefix (kept as-is) or custom$ (kept as-is)
                if attr_name.starts_with("host:")
                    || (attr_name.ends_with('$') && !attr_name.starts_with("on"))
                {
                    let value = if let Some(val) = attr.value {
                        jsx_attr_value_to_expression(
                            val,
                            tracker,
                            ctx,
                            destructured_props,
                            module_imports,
                            hoisted_stmts,
                        )
                    } else {
                        ctx.ast.expression_boolean_literal(SPAN, true)
                    };
                    const_props.push((attr_name, value));
                    continue;
                }

                // Check for bind: directive (CONV-12)
                if let Some(bind_prop) = attr_name.strip_prefix("bind:") {
                    let signal_value = if let Some(val) = attr.value {
                        jsx_attr_value_to_expression(
                            val,
                            tracker,
                            ctx,
                            destructured_props,
                            module_imports,
                            hoisted_stmts,
                        )
                    } else {
                        ctx.ast.expression_boolean_literal(SPAN, true)
                    };

                    match bind_prop {
                        "value" => {
                            // bind:value={signal} ->
                            //   "value": signal (const prop)
                            //   "q-e:input": inlinedQrl(_val, "_val", [signal]) (const prop)
                            tracker.needs_val = true;
                            tracker.needs_inlined_qrl = true;

                            // Build inlinedQrl(_val, "_val", [signal])
                            // We need to clone the signal expression for the captures array.
                            // Since we can't clone AST nodes, we serialize and re-identify.
                            let signal_name = extract_identifier_name(&signal_value);
                            let event_handler = build_bind_event_handler("_val", &signal_name, ctx);
                            const_props.push(("value".to_string(), signal_value));
                            const_props.push(("q-e:input".to_string(), event_handler));
                            continue;
                        }
                        "checked" => {
                            // bind:checked={signal} ->
                            //   "checked": signal (const prop)
                            //   "q-e:input": inlinedQrl(_chk, "_chk", [signal]) (const prop)
                            tracker.needs_chk = true;
                            tracker.needs_inlined_qrl = true;

                            let signal_name = extract_identifier_name(&signal_value);
                            let event_handler = build_bind_event_handler("_chk", &signal_name, ctx);
                            const_props.push(("checked".to_string(), signal_value));
                            const_props.push(("q-e:input".to_string(), event_handler));
                            continue;
                        }
                        _ => {
                            // bind:other -> pass through as-is in const props
                            const_props.push((attr_name, signal_value));
                            continue;
                        }
                    }
                }

                // Regular attribute
                has_any_visible_prop = true;
                _has_only_events = false;
                let value = if let Some(val) = attr.value {
                    jsx_attr_value_to_expression(
                        val,
                        tracker,
                        ctx,
                        destructured_props,
                        module_imports,
                        hoisted_stmts,
                    )
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
                            let source_obj =
                                if let Expression::StaticMemberExpression(member) = value {
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
                } else if !contains_function_call(&value) {
                    // Reactive deps without non-reactive refs -> _fnSignal wrapping
                    let (deps, has_non_reactive) =
                        collect_reactive_deps(&value, destructured_props, module_imports);
                    if !deps.is_empty() && !has_non_reactive {
                        // Wrap with _fnSignal
                        let (wrapped, fn_code, str_code) = build_fn_signal_wrapping(
                            value,
                            &deps,
                            destructured_props,
                            tracker,
                            ctx,
                        );
                        tracker.needs_fn_signal = true;
                        hoisted_stmts.push((fn_code, str_code));
                        const_props.push((attr_name, wrapped));
                    } else {
                        var_props.push((attr_name, value));
                    }
                } else {
                    var_props.push((attr_name, value));
                }
            }
        }
    }

    // Build children
    let (children_expr, children_count) = transform_jsx_children(
        &mut element.children,
        tracker,
        ctx,
        destructured_props,
        module_imports,
        hoisted_stmts,
    );

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
    } else if children_count > 0
        || has_any_visible_prop
        || !const_props.is_empty()
        || !var_props.is_empty()
    {
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
        let spread_source_clone = ctx.ast.expression_identifier(
            SPAN,
            // Try to extract name from spread_source
            if let Expression::Identifier(ref ident) = spread_source {
                ctx.ast.atom(ident.name.as_str())
            } else {
                ctx.ast.atom("props")
            },
        );
        get_var_args.push(Argument::from(spread_source_clone));
        let get_var_call = ctx.ast.expression_call(
            SPAN,
            get_var_callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            get_var_args,
            false,
        );
        var_obj_props.push(
            ctx.ast
                .object_property_kind_spread_property(SPAN, get_var_call),
        );

        // Add non-spread var props
        for (name, value) in var_props {
            let key = ctx
                .ast
                .property_key_static_identifier(SPAN, ctx.ast.atom(&name));
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
                let key = ctx
                    .ast
                    .property_key_static_identifier(SPAN, ctx.ast.atom(&name));
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
                    ctx.ast
                        .property_key_static_identifier(SPAN, ctx.ast.atom(&name))
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
pub(crate) fn transform_jsx_fragment_inner<'a>(
    mut fragment: JSXFragment<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
    module_imports: &[crate::types::ImportInfo],
    hoisted_stmts: &mut Vec<(String, String)>,
) -> Expression<'a> {
    tracker.needs_jsx_sorted = true;
    tracker.needs_fragment = true;

    let tag = ctx.ast.expression_identifier(SPAN, "_Fragment");

    // Build children
    let (children_expr, children_count) = transform_jsx_children(
        &mut fragment.children,
        tracker,
        ctx,
        destructured_props,
        module_imports,
        hoisted_stmts,
    );

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
pub(crate) fn transform_jsx_children<'a>(
    children: &mut oxc::allocator::Vec<'a, JSXChild<'a>>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    destructured_props: Option<&[(String, String)]>,
    module_imports: &[crate::types::ImportInfo],
    hoisted_stmts: &mut Vec<(String, String)>,
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
                let transformed = transform_jsx_element_inner(
                    el.unbox(),
                    tracker,
                    ctx,
                    destructured_props,
                    module_imports,
                    hoisted_stmts,
                );
                child_exprs.push(transformed);
            }
            JSXChild::Fragment(frag) => {
                // Recursively transform child JSXFragment
                let transformed = transform_jsx_fragment_inner(
                    frag.unbox(),
                    tracker,
                    ctx,
                    destructured_props,
                    module_imports,
                    hoisted_stmts,
                );
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
                                let result = transform_jsx_element_inner(
                                    el.unbox(),
                                    tracker,
                                    ctx,
                                    destructured_props,
                                    module_imports,
                                    hoisted_stmts,
                                );
                                child_exprs.push(result);
                            }
                            Expression::JSXFragment(frag) => {
                                let result = transform_jsx_fragment_inner(
                                    frag.unbox(),
                                    tracker,
                                    ctx,
                                    destructured_props,
                                    module_imports,
                                    hoisted_stmts,
                                );
                                child_exprs.push(result);
                            }
                            other => {
                                // Check for signal wrapping in children
                                // (mirrors attribute-level logic at lines 1031-1092)
                                if !is_call_on_value(&other) {
                                    match detect_signal_wrap(&other, destructured_props) {
                                        SignalWrapResult::WrapPropSignal => {
                                            // signal.value -> _wrapProp(signal) [EXISTING]
                                            if let Expression::StaticMemberExpression(member) =
                                                other
                                            {
                                                let signal_obj = member.unbox().object;
                                                let wrapped =
                                                    import_rewrite::build_wrap_prop_call(
                                                        signal_obj, ctx,
                                                    );
                                                tracker.needs_wrap_prop = true;
                                                child_exprs.push(wrapped);
                                                continue;
                                            }
                                        }
                                        SignalWrapResult::WrapPropNamed(prop_name) => {
                                            // _rawProps.propName or destructured prop ->
                                            // _wrapProp(_rawProps, "propName") [NEW]
                                            let source_obj = if let Expression::StaticMemberExpression(member) = other {
                                                member.unbox().object
                                            } else {
                                                ctx.ast.expression_identifier(SPAN, "_rawProps")
                                            };
                                            let wrapped =
                                                import_rewrite::build_wrap_prop_call_named(
                                                    source_obj, &prop_name, ctx,
                                                );
                                            tracker.needs_wrap_prop = true;
                                            child_exprs.push(wrapped);
                                            continue;
                                        }
                                        SignalWrapResult::None => {}
                                    }
                                }
                                // Check for _fnSignal wrapping (complex reactive
                                // expressions) [NEW]
                                if !is_call_on_value(&other)
                                    && !contains_function_call(&other)
                                {
                                    let (deps, has_non_reactive) = collect_reactive_deps(
                                        &other,
                                        destructured_props,
                                        module_imports,
                                    );
                                    if !deps.is_empty() && !has_non_reactive {
                                        let (wrapped, fn_code, str_code) =
                                            build_fn_signal_wrapping(
                                                other,
                                                &deps,
                                                destructured_props,
                                                tracker,
                                                ctx,
                                            );
                                        tracker.needs_fn_signal = true;
                                        hoisted_stmts.push((fn_code, str_code));
                                        child_exprs.push(wrapped);
                                        continue;
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

// ---------------------------------------------------------------------------
// Custom JSX import source transform (React-style _jsx)
// ---------------------------------------------------------------------------

/// Transform a JSXElement using the standard React JSX runtime form: `_jsx("tag", {props})`.
///
/// Unlike Qwik's `_jsxSorted`, this:
/// - Merges all props into a single object (no var/const split)
/// - Does NOT rename event handlers (onClick$ stays as onClick$)
/// - Uses 2-arg form: `_jsx(tag, props)`
/// - Children go into the props object as `children` key
fn transform_jsx_element_custom_source<'a>(
    mut element: JSXElement<'a>,
    tracker: &mut ImportTracker,
    ctx: &mut TraverseCtx<'a, ()>,
    hoisted_stmts: &mut Vec<(String, String)>,
    module_imports: &[crate::types::ImportInfo],
    destructured_props: Option<&[(String, String)]>,
) -> Expression<'a> {
    // Signal that we need the _jsx import (reuses needs_jsx_sorted flag --
    // the import emission in transform.rs will emit _jsx instead of _jsxSorted
    // when custom_jsx_source is set).
    tracker.needs_jsx_sorted = true;

    let tag = build_tag_expression(&element.opening_element.name, ctx);

    // Collect all props into a single object
    let mut props: Vec<ObjectPropertyKind<'a>> = Vec::new();

    // Take attributes out of the opening element
    let mut attrs = ctx.ast.vec();
    std::mem::swap(&mut element.opening_element.attributes, &mut attrs);

    for attr_item in attrs {
        match attr_item {
            JSXAttributeItem::SpreadAttribute(spread) => {
                props.push(
                    ctx.ast
                        .object_property_kind_spread_property(SPAN, spread.unbox().argument),
                );
            }
            JSXAttributeItem::Attribute(attr) => {
                let attr = attr.unbox();
                let attr_name = match &attr.name {
                    JSXAttributeName::Identifier(ident) => ident.name.as_str().to_string(),
                    JSXAttributeName::NamespacedName(ns) => {
                        format!("{}:{}", ns.namespace.name, ns.name.name)
                    }
                };

                let value = if let Some(val) = attr.value {
                    jsx_attr_value_to_expression(
                        val,
                        tracker,
                        ctx,
                        destructured_props,
                        module_imports,
                        hoisted_stmts,
                    )
                } else {
                    // Boolean attribute: <input disabled /> -> disabled: true
                    ctx.ast.expression_boolean_literal(SPAN, true)
                };

                // Use string literal key for names with special chars
                let key = if attr_name.contains(':') || attr_name.contains('-') || attr_name.contains('$') {
                    let atom = ctx.ast.atom(&attr_name);
                    PropertyKey::from(ctx.ast.expression_string_literal(SPAN, atom, None))
                } else {
                    ctx.ast
                        .property_key_static_identifier(SPAN, ctx.ast.atom(&attr_name))
                };

                props.push(ctx.ast.object_property_kind_object_property(
                    SPAN,
                    PropertyKind::Init,
                    key,
                    value,
                    false,
                    false,
                    false,
                ));
            }
        }
    }

    // Process children: add as `children` prop if present
    let (children_expr, _children_count) = transform_jsx_children(
        &mut element.children,
        tracker,
        ctx,
        destructured_props,
        module_imports,
        hoisted_stmts,
    );

    if let Some(children) = children_expr {
        let key = ctx
            .ast
            .property_key_static_identifier(SPAN, ctx.ast.atom("children"));
        props.push(ctx.ast.object_property_kind_object_property(
            SPAN,
            PropertyKind::Init,
            key,
            children,
            false,
            false,
            false,
        ));
    }

    // Build: _jsx(tag, {props}) or _jsx(tag) if no props
    let callee = ctx.ast.expression_identifier(SPAN, "_jsx");

    if props.is_empty() {
        // _jsx(tag) -- no props
        let mut arguments = ctx.ast.vec_with_capacity(1);
        arguments.push(Argument::from(tag));
        ctx.ast.expression_call_with_pure(
            SPAN,
            callee,
            None::<oxc::allocator::Box<'a, TSTypeParameterInstantiation<'a>>>,
            arguments,
            false,
            true,
        )
    } else {
        // _jsx(tag, {props})
        let mut props_vec = ctx.ast.vec_with_capacity(props.len());
        for prop in props {
            props_vec.push(prop);
        }
        let props_obj = ctx.ast.expression_object(SPAN, props_vec);

        let mut arguments = ctx.ast.vec_with_capacity(2);
        arguments.push(Argument::from(tag));
        arguments.push(Argument::from(props_obj));
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
