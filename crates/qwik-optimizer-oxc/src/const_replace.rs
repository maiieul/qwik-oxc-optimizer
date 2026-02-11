//! Build constant replacement and dead branch elimination.
//!
//! Replace references to compile-time constants (`isServer`, `isBrowser`, `isDev`)
//! with boolean literals based on build configuration, then eliminate dead branches.
//! This runs as a pre-pass before the main traverse so that segment body serialization
//! sees the replaced values.

use std::collections::HashMap;

use oxc::ast::ast::*;
use oxc::ast::AstBuilder;
use oxc::span::SPAN;

use crate::types::{EmitMode, TransformOptions};

/// Sources that can export build constants.
const BUILD_CONSTANT_SOURCES: &[&str] = &["@qwik.dev/core", "@qwik.dev/core/build"];

/// Known build constant names and what they represent.
const BUILD_CONSTANTS: &[&str] = &["isServer", "isBrowser", "isDev"];

/// Replace build constants and eliminate dead branches in the program AST.
///
/// Scans import declarations for known build constant imports from `@qwik.dev/core`
/// and `@qwik.dev/core/build`, replaces references with boolean literals based on
/// build configuration, then eliminates dead branches.
pub(crate) fn replace_build_constants<'a>(
    program: &mut Program<'a>,
    options: &TransformOptions,
    allocator: &'a oxc::allocator::Allocator,
) {
    let replacements = build_replacement_map(program, options);
    if replacements.is_empty() {
        return;
    }

    let ast = AstBuilder::new(allocator);

    replace_identifiers_in_statements(&mut program.body, &replacements, &ast);

    eliminate_dead_branches(&mut program.body, &ast);

    strip_build_constant_imports(&mut program.body, &replacements);
}

/// Scan import declarations for build constant imports and build a map of
/// local_name -> replacement boolean value.
fn build_replacement_map(
    program: &Program<'_>,
    options: &TransformOptions,
) -> HashMap<String, bool> {
    let mut map = HashMap::new();

    for stmt in &program.body {
        if let Statement::ImportDeclaration(import) = stmt {
            let source = import.source.value.as_str();
            if !BUILD_CONSTANT_SOURCES.iter().any(|s| *s == source) {
                continue;
            }

            if let Some(specifiers) = &import.specifiers {
                for spec in specifiers {
                    if let ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                        let imported_name = match &s.imported {
                            ModuleExportName::IdentifierName(id) => id.name.as_str(),
                            ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                            ModuleExportName::StringLiteral(sl) => sl.value.as_str(),
                        };

                        if !BUILD_CONSTANTS.contains(&imported_name) {
                            continue;
                        }

                        let local_name = s.local.name.as_str().to_string();
                        let value = match imported_name {
                            "isServer" => options.is_server,
                            "isBrowser" => !options.is_server,
                            "isDev" => matches!(options.mode, EmitMode::Dev),
                            _ => continue,
                        };

                        map.insert(local_name, value);
                    }
                }
            }
        }
    }

    map
}

// ---------------------------------------------------------------------------
// Identifier Replacement
// ---------------------------------------------------------------------------

/// Replace build constant identifiers with boolean literals in a list of statements.
fn replace_identifiers_in_statements<'a>(
    stmts: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    for stmt in stmts.iter_mut() {
        replace_identifiers_in_statement(stmt, replacements, ast);
    }
}

/// Replace build constant identifiers in a single statement.
fn replace_identifiers_in_statement<'a>(
    stmt: &mut Statement<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match stmt {
        Statement::ExpressionStatement(expr_stmt) => {
            replace_identifiers_in_expression(&mut expr_stmt.expression, replacements, ast);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(ref mut arg) = ret.argument {
                replace_identifiers_in_expression(arg, replacements, ast);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for declarator in decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    replace_identifiers_in_expression(init, replacements, ast);
                }
            }
        }
        Statement::IfStatement(if_stmt) => {
            replace_identifiers_in_expression(&mut if_stmt.test, replacements, ast);
            replace_identifiers_in_statement(&mut if_stmt.consequent, replacements, ast);
            if let Some(ref mut alt) = if_stmt.alternate {
                replace_identifiers_in_statement(alt, replacements, ast);
            }
        }
        Statement::BlockStatement(block) => {
            replace_identifiers_in_statements(&mut block.body, replacements, ast);
        }
        Statement::ForStatement(for_stmt) => {
            if let Some(ref mut test) = for_stmt.test {
                replace_identifiers_in_expression(test, replacements, ast);
            }
            if let Some(ref mut update) = for_stmt.update {
                replace_identifiers_in_expression(update, replacements, ast);
            }
            replace_identifiers_in_statement(&mut for_stmt.body, replacements, ast);
        }
        Statement::WhileStatement(while_stmt) => {
            replace_identifiers_in_expression(&mut while_stmt.test, replacements, ast);
            replace_identifiers_in_statement(&mut while_stmt.body, replacements, ast);
        }
        Statement::ThrowStatement(throw_stmt) => {
            replace_identifiers_in_expression(&mut throw_stmt.argument, replacements, ast);
        }
        Statement::SwitchStatement(switch_stmt) => {
            replace_identifiers_in_expression(
                &mut switch_stmt.discriminant,
                replacements,
                ast,
            );
            for case in switch_stmt.cases.iter_mut() {
                if let Some(ref mut test) = case.test {
                    replace_identifiers_in_expression(test, replacements, ast);
                }
                replace_identifiers_in_statements(&mut case.consequent, replacements, ast);
            }
        }
        Statement::ExportNamedDeclaration(export) => {
            if let Some(ref mut decl) = export.declaration {
                replace_identifiers_in_declaration(decl, replacements, ast);
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            match &mut export.declaration {
                ExportDefaultDeclarationKind::ArrowFunctionExpression(arrow) => {
                    replace_identifiers_in_statements(
                        &mut arrow.body.statements,
                        replacements,
                        ast,
                    );
                }
                ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    if let Some(ref mut body) = func.body {
                        replace_identifiers_in_statements(
                            &mut body.statements,
                            replacements,
                            ast,
                        );
                    }
                }
                _ => {}
            }
        }
        Statement::FunctionDeclaration(func) => {
            if let Some(ref mut body) = func.body {
                replace_identifiers_in_statements(&mut body.statements, replacements, ast);
            }
        }
        _ => {}
    }
}

/// Replace build constant identifiers in a declaration (inside export statements).
fn replace_identifiers_in_declaration<'a>(
    decl: &mut Declaration<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match decl {
        Declaration::VariableDeclaration(var_decl) => {
            for declarator in var_decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    replace_identifiers_in_expression(init, replacements, ast);
                }
            }
        }
        Declaration::FunctionDeclaration(func) => {
            if let Some(ref mut body) = func.body {
                replace_identifiers_in_statements(&mut body.statements, replacements, ast);
            }
        }
        Declaration::ClassDeclaration(class) => {
            for element in class.body.body.iter_mut() {
                if let ClassElement::MethodDefinition(method) = element {
                    if let Some(ref mut body) = method.value.body {
                        replace_identifiers_in_statements(
                            &mut body.statements,
                            replacements,
                            ast,
                        );
                    }
                }
            }
        }
        _ => {}
    }
}

/// Replace build constant identifiers in an expression.
fn replace_identifiers_in_expression<'a>(
    expr: &mut Expression<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match expr {
        Expression::Identifier(ident) => {
            if let Some(&value) = replacements.get(ident.name.as_str()) {
                *expr = ast.expression_boolean_literal(SPAN, value);
            }
        }
        Expression::LogicalExpression(logical) => {
            replace_identifiers_in_expression(&mut logical.left, replacements, ast);
            replace_identifiers_in_expression(&mut logical.right, replacements, ast);
        }
        Expression::BinaryExpression(bin) => {
            replace_identifiers_in_expression(&mut bin.left, replacements, ast);
            replace_identifiers_in_expression(&mut bin.right, replacements, ast);
        }
        Expression::UnaryExpression(unary) => {
            replace_identifiers_in_expression(&mut unary.argument, replacements, ast);
        }
        Expression::ConditionalExpression(cond) => {
            replace_identifiers_in_expression(&mut cond.test, replacements, ast);
            replace_identifiers_in_expression(&mut cond.consequent, replacements, ast);
            replace_identifiers_in_expression(&mut cond.alternate, replacements, ast);
        }
        Expression::CallExpression(call) => {
            replace_identifiers_in_expression(&mut call.callee, replacements, ast);
            for arg in call.arguments.iter_mut() {
                replace_identifiers_in_argument(arg, replacements, ast);
            }
        }
        Expression::ArrowFunctionExpression(arrow) => {
            replace_identifiers_in_statements(&mut arrow.body.statements, replacements, ast);
        }
        Expression::AssignmentExpression(assign) => {
            replace_identifiers_in_expression(&mut assign.right, replacements, ast);
        }
        Expression::SequenceExpression(seq) => {
            for item in seq.expressions.iter_mut() {
                replace_identifiers_in_expression(item, replacements, ast);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            replace_identifiers_in_expression(&mut paren.expression, replacements, ast);
        }
        Expression::ArrayExpression(arr) => {
            for elem in arr.elements.iter_mut() {
                replace_identifiers_in_array_element(elem, replacements, ast);
            }
        }
        Expression::ObjectExpression(obj) => {
            for prop in obj.properties.iter_mut() {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        replace_identifiers_in_expression(&mut p.value, replacements, ast);
                    }
                    ObjectPropertyKind::SpreadProperty(s) => {
                        replace_identifiers_in_expression(&mut s.argument, replacements, ast);
                    }
                }
            }
        }
        Expression::TemplateLiteral(tmpl) => {
            for inner_expr in tmpl.expressions.iter_mut() {
                replace_identifiers_in_expression(inner_expr, replacements, ast);
            }
        }
        Expression::NewExpression(new_expr) => {
            replace_identifiers_in_expression(&mut new_expr.callee, replacements, ast);
            for arg in new_expr.arguments.iter_mut() {
                replace_identifiers_in_argument(arg, replacements, ast);
            }
        }
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => {}
        Expression::JSXElement(jsx) => {
            replace_identifiers_in_jsx_element(jsx, replacements, ast);
        }
        Expression::JSXFragment(jsx) => {
            replace_identifiers_in_jsx_children(&mut jsx.children, replacements, ast);
        }
        _ => {}
    }
}

/// Replace identifiers in an Argument (inherit_variants pattern from Expression).
fn replace_identifiers_in_argument<'a>(
    arg: &mut Argument<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match arg {
        Argument::Identifier(ident) => {
            if let Some(&value) = replacements.get(ident.name.as_str()) {
                *arg = Argument::BooleanLiteral(ast.alloc_boolean_literal(SPAN, value));
            }
        }
        Argument::LogicalExpression(logical) => {
            replace_identifiers_in_expression(&mut logical.left, replacements, ast);
            replace_identifiers_in_expression(&mut logical.right, replacements, ast);
        }
        Argument::CallExpression(call) => {
            replace_identifiers_in_expression(&mut call.callee, replacements, ast);
            for inner_arg in call.arguments.iter_mut() {
                replace_identifiers_in_argument(inner_arg, replacements, ast);
            }
        }
        Argument::ArrowFunctionExpression(arrow) => {
            replace_identifiers_in_statements(&mut arrow.body.statements, replacements, ast);
        }
        Argument::ConditionalExpression(cond) => {
            replace_identifiers_in_expression(&mut cond.test, replacements, ast);
            replace_identifiers_in_expression(&mut cond.consequent, replacements, ast);
            replace_identifiers_in_expression(&mut cond.alternate, replacements, ast);
        }
        Argument::UnaryExpression(unary) => {
            replace_identifiers_in_expression(&mut unary.argument, replacements, ast);
        }
        Argument::BinaryExpression(bin) => {
            replace_identifiers_in_expression(&mut bin.left, replacements, ast);
            replace_identifiers_in_expression(&mut bin.right, replacements, ast);
        }
        Argument::ArrayExpression(arr) => {
            for elem in arr.elements.iter_mut() {
                replace_identifiers_in_array_element(elem, replacements, ast);
            }
        }
        Argument::ObjectExpression(obj) => {
            for prop in obj.properties.iter_mut() {
                match prop {
                    ObjectPropertyKind::ObjectProperty(p) => {
                        replace_identifiers_in_expression(&mut p.value, replacements, ast);
                    }
                    ObjectPropertyKind::SpreadProperty(s) => {
                        replace_identifiers_in_expression(&mut s.argument, replacements, ast);
                    }
                }
            }
        }
        Argument::ParenthesizedExpression(paren) => {
            replace_identifiers_in_expression(&mut paren.expression, replacements, ast);
        }
        _ => {}
    }
}

/// Replace identifiers in an ArrayExpressionElement (inherit_variants from Expression).
fn replace_identifiers_in_array_element<'a>(
    elem: &mut ArrayExpressionElement<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match elem {
        ArrayExpressionElement::Identifier(ident) => {
            if let Some(&value) = replacements.get(ident.name.as_str()) {
                *elem = ArrayExpressionElement::BooleanLiteral(
                    ast.alloc_boolean_literal(SPAN, value),
                );
            }
        }
        ArrayExpressionElement::LogicalExpression(logical) => {
            replace_identifiers_in_expression(&mut logical.left, replacements, ast);
            replace_identifiers_in_expression(&mut logical.right, replacements, ast);
        }
        ArrayExpressionElement::SpreadElement(spread) => {
            replace_identifiers_in_expression(&mut spread.argument, replacements, ast);
        }
        _ => {}
    }
}

/// Replace identifiers in JSX element children and attributes.
fn replace_identifiers_in_jsx_element<'a>(
    jsx: &mut JSXElement<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    for attr in jsx.opening_element.attributes.iter_mut() {
        if let JSXAttributeItem::Attribute(a) = attr {
            if let Some(JSXAttributeValue::ExpressionContainer(container)) = &mut a.value {
                replace_identifiers_in_jsx_expression(
                    &mut container.expression,
                    replacements,
                    ast,
                );
            }
        }
    }
    replace_identifiers_in_jsx_children(&mut jsx.children, replacements, ast);
}

/// Replace identifiers in JSX children.
fn replace_identifiers_in_jsx_children<'a>(
    children: &mut oxc::allocator::Vec<'a, JSXChild<'a>>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    for child in children.iter_mut() {
        match child {
            JSXChild::ExpressionContainer(container) => {
                replace_identifiers_in_jsx_expression(
                    &mut container.expression,
                    replacements,
                    ast,
                );
            }
            JSXChild::Element(elem) => {
                replace_identifiers_in_jsx_element(elem, replacements, ast);
            }
            JSXChild::Fragment(frag) => {
                replace_identifiers_in_jsx_children(&mut frag.children, replacements, ast);
            }
            _ => {}
        }
    }
}

/// Replace identifiers in a JSX expression.
fn replace_identifiers_in_jsx_expression<'a>(
    expr: &mut JSXExpression<'a>,
    replacements: &HashMap<String, bool>,
    ast: &AstBuilder<'a>,
) {
    match expr {
        JSXExpression::Identifier(ident) => {
            if let Some(&value) = replacements.get(ident.name.as_str()) {
                *expr = JSXExpression::BooleanLiteral(ast.alloc_boolean_literal(SPAN, value));
            }
        }
        JSXExpression::LogicalExpression(logical) => {
            replace_identifiers_in_expression(&mut logical.left, replacements, ast);
            replace_identifiers_in_expression(&mut logical.right, replacements, ast);
        }
        JSXExpression::ConditionalExpression(cond) => {
            replace_identifiers_in_expression(&mut cond.test, replacements, ast);
            replace_identifiers_in_expression(&mut cond.consequent, replacements, ast);
            replace_identifiers_in_expression(&mut cond.alternate, replacements, ast);
        }
        JSXExpression::CallExpression(call) => {
            replace_identifiers_in_expression(&mut call.callee, replacements, ast);
            for arg in call.arguments.iter_mut() {
                replace_identifiers_in_argument(arg, replacements, ast);
            }
        }
        JSXExpression::UnaryExpression(unary) => {
            replace_identifiers_in_expression(&mut unary.argument, replacements, ast);
        }
        JSXExpression::ParenthesizedExpression(paren) => {
            replace_identifiers_in_expression(&mut paren.expression, replacements, ast);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Logical Expression Simplification
// ---------------------------------------------------------------------------

/// Returns `Some(bool)` if the expression can be reduced to a boolean literal.
fn eval_boolean_value(expr: &Expression<'_>) -> Option<bool> {
    match expr {
        Expression::BooleanLiteral(lit) => Some(lit.value),
        Expression::UnaryExpression(unary) => {
            if matches!(
                unary.operator,
                oxc::syntax::operator::UnaryOperator::LogicalNot
            ) {
                eval_boolean_value(&unary.argument).map(|v| !v)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Simplify a logical expression in-place.
/// `false && x` -> `false`, `true && x` -> `x`
/// `true || x` -> `true`, `false || x` -> `x`
fn simplify_logical_expression<'a>(expr: &mut Expression<'a>, ast: &AstBuilder<'a>) {
    if let Expression::LogicalExpression(logical) = expr {
        simplify_logical_expression(&mut logical.left, ast);
        simplify_logical_expression(&mut logical.right, ast);
    }
    if let Expression::UnaryExpression(unary) = expr {
        simplify_logical_expression(&mut unary.argument, ast);
    }

    if let Expression::LogicalExpression(logical) = expr {
        if let Some(left_val) = eval_boolean_value(&logical.left) {
            match logical.operator {
                oxc::syntax::operator::LogicalOperator::And => {
                    if !left_val {
                        // false && x -> false
                        *expr = ast.expression_boolean_literal(SPAN, false);
                    } else {
                        // true && x -> x
                        let right = std::mem::replace(
                            &mut logical.right,
                            ast.expression_boolean_literal(SPAN, false),
                        );
                        *expr = right;
                    }
                }
                oxc::syntax::operator::LogicalOperator::Or => {
                    if left_val {
                        // true || x -> true
                        *expr = ast.expression_boolean_literal(SPAN, true);
                    } else {
                        // false || x -> x
                        let right = std::mem::replace(
                            &mut logical.right,
                            ast.expression_boolean_literal(SPAN, false),
                        );
                        *expr = right;
                    }
                }
                _ => {}
            }
        }
    }

    if let Expression::UnaryExpression(unary) = expr {
        if matches!(
            unary.operator,
            oxc::syntax::operator::UnaryOperator::LogicalNot
        ) {
            if let Some(val) = eval_boolean_value(&unary.argument) {
                *expr = ast.expression_boolean_literal(SPAN, !val);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Dead Branch Elimination
// ---------------------------------------------------------------------------

/// Eliminate dead branches in a list of statements.
///
/// - `if (false) { ... }` with no alternate -> remove entirely
/// - `if (true) { ... }` -> inline the consequent block body
/// - `if (false) { ... } else { ... }` -> keep only the else body
/// - `if (!false) { ... }` same as `if (true) { ... }`
fn eliminate_dead_branches<'a>(
    stmts: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    ast: &AstBuilder<'a>,
) {
    for stmt in stmts.iter_mut() {
        simplify_expressions_in_statement(stmt, ast);
    }

    let mut actions: Vec<(usize, StmtAction<'a>)> = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        if let Statement::IfStatement(if_stmt) = stmt {
            if let Some(test_val) = eval_boolean_value(&if_stmt.test) {
                if test_val {
                    // if (true) { consequent } -> inline consequent
                    let placeholder =
                        Statement::EmptyStatement(ast.alloc_empty_statement(SPAN));
                    let consequent =
                        std::mem::replace(&mut if_stmt.consequent, placeholder);
                    if let Statement::BlockStatement(block) = consequent {
                        // Unbox the block and drain the body into a std vec
                        let block_inner = block.unbox();
                        let body_stmts: Vec<Statement<'a>> =
                            block_inner.body.into_iter().collect();
                        actions.push((i, StmtAction::ReplaceWith(body_stmts)));
                    } else {
                        actions.push((i, StmtAction::ReplaceWith(vec![consequent])));
                    }
                } else if if_stmt.alternate.is_some() {
                    // if (false) { ... } else { alternate } -> inline alternate
                    let alternate = if_stmt.alternate.take().unwrap();
                    if let Statement::BlockStatement(block) = alternate {
                        let block_inner = block.unbox();
                        let body_stmts: Vec<Statement<'a>> =
                            block_inner.body.into_iter().collect();
                        actions.push((i, StmtAction::ReplaceWith(body_stmts)));
                    } else {
                        actions.push((i, StmtAction::ReplaceWith(vec![alternate])));
                    }
                } else {
                    // if (false) { ... } -> remove
                    actions.push((i, StmtAction::Remove));
                }
            }
        }
    }

    if !actions.is_empty() {
        let mut action_map: HashMap<usize, StmtAction<'a>> =
            actions.into_iter().collect();
        let all_stmts: Vec<Statement<'a>> = stmts.drain(..).collect();
        let mut result: Vec<Statement<'a>> = Vec::new();

        for (i, stmt) in all_stmts.into_iter().enumerate() {
            if let Some(action) = action_map.remove(&i) {
                match action {
                    StmtAction::Remove => { /* drop */ }
                    StmtAction::ReplaceWith(replacement) => {
                        result.extend(replacement);
                    }
                }
            } else {
                result.push(stmt);
            }
        }

        for stmt in result {
            stmts.push(stmt);
        }
    }

    for stmt in stmts.iter_mut() {
        recurse_dead_branches_in_statement(stmt, ast);
    }
}

/// Action to take for a statement during dead branch elimination.
enum StmtAction<'a> {
    Remove,
    ReplaceWith(Vec<Statement<'a>>),
}

/// Simplify logical and unary expressions within a statement.
fn simplify_expressions_in_statement<'a>(stmt: &mut Statement<'a>, ast: &AstBuilder<'a>) {
    match stmt {
        Statement::ExpressionStatement(expr_stmt) => {
            simplify_logical_expression(&mut expr_stmt.expression, ast);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(ref mut arg) = ret.argument {
                simplify_logical_expression(arg, ast);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for declarator in decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    simplify_logical_expression(init, ast);
                }
            }
        }
        Statement::IfStatement(if_stmt) => {
            simplify_logical_expression(&mut if_stmt.test, ast);
        }
        Statement::ExportNamedDeclaration(export) => {
            if let Some(ref mut decl) = export.declaration {
                simplify_expressions_in_declaration(decl, ast);
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            match &mut export.declaration {
                ExportDefaultDeclarationKind::ArrowFunctionExpression(arrow) => {
                    for inner_stmt in arrow.body.statements.iter_mut() {
                        simplify_expressions_in_statement(inner_stmt, ast);
                    }
                }
                ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    if let Some(ref mut body) = func.body {
                        for inner_stmt in body.statements.iter_mut() {
                            simplify_expressions_in_statement(inner_stmt, ast);
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
}

/// Simplify logical expressions in a declaration.
fn simplify_expressions_in_declaration<'a>(
    decl: &mut Declaration<'a>,
    ast: &AstBuilder<'a>,
) {
    match decl {
        Declaration::VariableDeclaration(var_decl) => {
            for declarator in var_decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    simplify_logical_expression(init, ast);
                }
            }
        }
        Declaration::FunctionDeclaration(func) => {
            if let Some(ref mut body) = func.body {
                for inner_stmt in body.statements.iter_mut() {
                    simplify_expressions_in_statement(inner_stmt, ast);
                }
            }
        }
        _ => {}
    }
}

/// Recurse into nested statements for dead branch elimination.
fn recurse_dead_branches_in_statement<'a>(stmt: &mut Statement<'a>, ast: &AstBuilder<'a>) {
    match stmt {
        Statement::BlockStatement(block) => {
            eliminate_dead_branches(&mut block.body, ast);
        }
        Statement::IfStatement(if_stmt) => {
            if let Statement::BlockStatement(block) = &mut if_stmt.consequent {
                eliminate_dead_branches(&mut block.body, ast);
            }
            if let Some(ref mut alt) = if_stmt.alternate {
                if let Statement::BlockStatement(block) = alt {
                    eliminate_dead_branches(&mut block.body, ast);
                }
            }
        }
        Statement::ExpressionStatement(expr_stmt) => {
            recurse_dead_branches_in_expression(&mut expr_stmt.expression, ast);
        }
        Statement::ReturnStatement(ret) => {
            if let Some(ref mut arg) = ret.argument {
                recurse_dead_branches_in_expression(arg, ast);
            }
        }
        Statement::VariableDeclaration(decl) => {
            for declarator in decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    recurse_dead_branches_in_expression(init, ast);
                }
            }
        }
        Statement::ForStatement(for_stmt) => {
            recurse_dead_branches_in_statement(&mut for_stmt.body, ast);
        }
        Statement::WhileStatement(while_stmt) => {
            recurse_dead_branches_in_statement(&mut while_stmt.body, ast);
        }
        Statement::ExportNamedDeclaration(export) => {
            if let Some(ref mut decl) = export.declaration {
                recurse_dead_branches_in_declaration(decl, ast);
            }
        }
        Statement::ExportDefaultDeclaration(export) => {
            match &mut export.declaration {
                ExportDefaultDeclarationKind::ArrowFunctionExpression(arrow) => {
                    eliminate_dead_branches(&mut arrow.body.statements, ast);
                }
                ExportDefaultDeclarationKind::FunctionDeclaration(func) => {
                    if let Some(ref mut body) = func.body {
                        eliminate_dead_branches(&mut body.statements, ast);
                    }
                }
                _ => {}
            }
        }
        Statement::FunctionDeclaration(func) => {
            if let Some(ref mut body) = func.body {
                eliminate_dead_branches(&mut body.statements, ast);
            }
        }
        _ => {}
    }
}

/// Recurse into a declaration for dead branch elimination.
fn recurse_dead_branches_in_declaration<'a>(
    decl: &mut Declaration<'a>,
    ast: &AstBuilder<'a>,
) {
    match decl {
        Declaration::VariableDeclaration(var_decl) => {
            for declarator in var_decl.declarations.iter_mut() {
                if let Some(ref mut init) = declarator.init {
                    recurse_dead_branches_in_expression(init, ast);
                }
            }
        }
        Declaration::FunctionDeclaration(func) => {
            if let Some(ref mut body) = func.body {
                eliminate_dead_branches(&mut body.statements, ast);
            }
        }
        _ => {}
    }
}

/// Recurse into expressions for dead branch elimination in nested arrow functions.
fn recurse_dead_branches_in_expression<'a>(
    expr: &mut Expression<'a>,
    ast: &AstBuilder<'a>,
) {
    match expr {
        Expression::ArrowFunctionExpression(arrow) => {
            eliminate_dead_branches(&mut arrow.body.statements, ast);
        }
        Expression::CallExpression(call) => {
            for arg in call.arguments.iter_mut() {
                recurse_dead_branches_in_argument(arg, ast);
            }
        }
        Expression::ParenthesizedExpression(paren) => {
            recurse_dead_branches_in_expression(&mut paren.expression, ast);
        }
        _ => {}
    }
}

/// Recurse into arguments for dead branch elimination.
fn recurse_dead_branches_in_argument<'a>(arg: &mut Argument<'a>, ast: &AstBuilder<'a>) {
    match arg {
        Argument::ArrowFunctionExpression(arrow) => {
            eliminate_dead_branches(&mut arrow.body.statements, ast);
        }
        Argument::CallExpression(call) => {
            for inner_arg in call.arguments.iter_mut() {
                recurse_dead_branches_in_argument(inner_arg, ast);
            }
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Import Stripping
// ---------------------------------------------------------------------------

/// Remove build constant specifiers from import declarations.
/// If an import declaration only contains build constant specifiers, remove it entirely.
/// If it also contains non-build-constant specifiers, only remove the build constant ones.
fn strip_build_constant_imports<'a>(
    stmts: &mut oxc::allocator::Vec<'a, Statement<'a>>,
    replacements: &HashMap<String, bool>,
) {
    let mut remove_indices: Vec<usize> = Vec::new();

    for (i, stmt) in stmts.iter_mut().enumerate() {
        if let Statement::ImportDeclaration(import) = stmt {
            let source = import.source.value.as_str();
            if !BUILD_CONSTANT_SOURCES.iter().any(|s| *s == source) {
                continue;
            }

            if let Some(ref mut specifiers) = import.specifiers {
                specifiers.retain(|spec| {
                    if let ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                        !replacements.contains_key(s.local.name.as_str())
                    } else {
                        true
                    }
                });

                // If no specifiers remain, mark the entire import for removal
                if specifiers.is_empty() {
                    remove_indices.push(i);
                }
            }
        }
    }

    // Remove marked imports (iterate in reverse to preserve indices)
    if !remove_indices.is_empty() {
        let all_stmts: Vec<Statement<'a>> = stmts.drain(..).collect();
        for (i, stmt) in all_stmts.into_iter().enumerate() {
            if !remove_indices.contains(&i) {
                stmts.push(stmt);
            }
        }
    }
}
