# JSX Transformation: OXC API Mapping (APIM-04)

**Date:** 2026-02-10
**Purpose:** A standalone, implementable specification for JSX transformation using OXC AstBuilder APIs. A developer reading this document can implement the complete JSX transformation pass -- including `_jsxSorted`, `_jsxSplit`, prop classification, Fragment handling, event handler transformation, dev mode, and children encoding -- without referencing the SWC source code.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-04 | Entire document | JSX transformation patterns mapped to OXC expression replacement APIs |
| CONV-03 | Sections 1-7 | `_jsxSorted`/`_jsxSplit` construction with all argument variants |
| CONV-04 (partial) | Section 3 | Prop classification logic for signal helper generation (full signal docs in PROPS-SIGNALS-MAPPING.md) |

---

## 1. _jsxSorted Construction

### 1.1 Signature

```
_jsxSorted(tag, varProps, constProps, children, flags, key)
_jsxSorted(tag, varProps, constProps, children, flags, key, devInfo)  // Dev mode only
```

**Arguments:**

| Position | Name | Type | Description |
|----------|------|------|-------------|
| 1 | tag | StringLiteral or Identifier | `"div"` for native elements, `Cmp` for components, `_Fragment` for fragments |
| 2 | varProps | ObjectExpression or `null` | Mutable/variable props (globals, function calls, mixed expressions) |
| 3 | constProps | ObjectExpression or `null` | Constant/trackable props (static values, signals, store accesses, imports) |
| 4 | children | Expression, ArrayExpression, or `null` | Single child, array of children, or null |
| 5 | flags | NumericLiteral | Values 0, 1, 2, or 3 encoding child type information |
| 6 | key | StringLiteral or `null` | Element key (e.g., `"u6_0"`, `"u6_1"`) or null |
| 7 | devInfo | ObjectExpression (Dev only) | `{fileName, lineNumber, columnNumber}` source location |

### 1.2 Before/After Examples

**Native element with static props (`example_jsx.md` output line 1160):**

```tsx
// Input:
<div class="class">12</div>

// Output:
_jsxSorted("div", null, { class: "class" }, "12", 3, null)
```

**Component with var/const split (`example_derived_signals_cmp.md` output lines 80-103):**

```tsx
// Input:
<Cmp staticText="text" global={globalThing} signalValue={signal.value} />

// Output:
_jsxSorted(Cmp, {
    global: globalThing,          // varProps
}, {
    staticText: "text",           // constProps
    signalValue: _wrapProp(signal),
}, null, 3, "u6_0")
```

**Fragment with children (`example_getter_generation.md` output lines 86-95):**

```tsx
// Input:
<>
    <p data-value={props.count}>{props.nested.count}</p>
    <p>Value {props.count}<span></span></p>
</>

// Output:
_jsxSorted(_Fragment, null, null, [
    _jsxSorted("p", { "data-value": _wrapProp(props, "count") }, null,
        _fnSignal(_hf0, [props], _hf0_str), 1, null),
    _jsxSorted("p", null, null, [
        "Value ", _wrapProp(props, "count"),
        _jsxSorted("span", null, null, null, 3, null)
    ], 1, null)
], 1, "u6_1")
```

### 1.3 Complete Rust Function: `build_jsx_sorted_call()`

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a _jsxSorted() call expression:
///   _jsxSorted(tag, varProps, constProps, children, flags, key)
///
/// Arguments:
/// - `tag_expr`: String literal for native elements, identifier for components/fragments
/// - `var_props`: Object expression with variable props, or None for null
/// - `const_props`: Object expression with constant props, or None for null
/// - `children_expr`: Children expression (single, array, or None for null)
/// - `flags`: Numeric flags value (0, 1, 2, or 3)
/// - `key_value`: Optional string literal key (e.g., "u6_0")
/// - `dev_info`: Optional dev mode source location object
/// - `ctx`: TraverseCtx for AstBuilder access
///
/// Returns an Expression representing the _jsxSorted(...) call.
pub fn build_jsx_sorted_call<'a>(
    tag_expr: Expression<'a>,
    var_props: Option<Expression<'a>>,
    const_props: Option<Expression<'a>>,
    children_expr: Option<Expression<'a>>,
    flags: u32,
    key_value: Option<&str>,
    dev_info: Option<Expression<'a>>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Determine argument count: 6 standard + optional dev info
    let capacity = if dev_info.is_some() { 7 } else { 6 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);

    // Arg 1: tag
    // Native: ast.expression_string_literal(SPAN, "div")
    // Component: identifier reference (e.g., Cmp)
    // Fragment: _Fragment identifier
    arguments.push(Argument::from(tag_expr));

    // Arg 2: varProps (mutable props object or null)
    arguments.push(Argument::from(
        var_props.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
    ));

    // Arg 3: constProps (constant/trackable props object or null)
    arguments.push(Argument::from(
        const_props.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
    ));

    // Arg 4: children (single expression, array, or null)
    arguments.push(Argument::from(
        children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
    ));

    // Arg 5: flags (numeric literal)
    arguments.push(Argument::from(
        ctx.ast.expression_numeric_literal(
            SPAN,
            flags as f64,
            None,
            NumberBase::Decimal,
        ),
    ));

    // Arg 6: key (string literal or null)
    arguments.push(Argument::from(match key_value {
        Some(key) => ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(key),
            None,
        ),
        None => ctx.ast.expression_null_literal(SPAN),
    }));

    // Arg 7 (optional): dev info object
    if let Some(dev) = dev_info {
        arguments.push(Argument::from(dev));
    }

    // Build callee: _jsxSorted identifier
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_jsxSorted"),
    );

    // Build: _jsxSorted(tag, varProps, constProps, children, flags, key)
    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}
```

### 1.4 Tag Argument Construction

The tag argument varies by element type:

```rust
/// Build the tag expression for a JSX element.
fn build_tag_expression<'a>(
    element_name: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Check if the element name starts with a lowercase letter (native HTML element)
    if element_name.chars().next().map_or(false, |c| c.is_lowercase()) {
        // Native element: string literal
        // "div", "button", "input", "span", "p"
        ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(element_name),
            None,
        )
    } else {
        // Component: identifier reference
        // Cmp, Lightweight, _Fragment
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom(element_name),
        )
    }
}
```

**Evidence:**
- Native elements as string literals: `"div"`, `"button"`, `"span"`, `"p"`, `"input"` -- seen in all spec files
- Components as identifiers: `Cmp` (`example_derived_signals_cmp.md` line 80), `Lightweight` (`example_jsx.md` line 1162), `_Fragment` (`example_getter_generation.md` line 86)

---

## 2. _jsxSplit Construction

### 2.1 When to Use _jsxSplit

`_jsxSplit` is used instead of `_jsxSorted` when a **spread operator** (`{...props}`) is present on the JSX element. The spread props are decomposed into `_getVarProps()` and `_getConstProps()` helper calls.

### 2.2 Pattern

```javascript
// Input:
<button {...props}/>

// Output:
_jsxSplit("button", { ..._getVarProps(props) }, _getConstProps(props), null, 0, null)
```

The varProps object uses `..._getVarProps(source)` as a spread element inside the object. The constProps argument is the result of `_getConstProps(source)` directly.

**Evidence:** `example_jsx.md` output lines 1162-1164, `should_destructure_args.md` output lines 88-90

### 2.3 Complete Example From `should_destructure_args.md`

```tsx
// Input:
<span {...rest}>
    {message} {c}
</span>

// Output (lines 88-94):
_jsxSplit("span", {
    ..._getVarProps(rest)
}, _getConstProps(rest), [
    _wrapProp(_rawProps, "message"),
    " ",
    _wrapProp(_rawProps, "count")
], 0, null)
```

Key observations:
- `_jsxSplit` replaces `_jsxSorted` when spread is present
- The spread source (`rest`) is passed to both `_getVarProps` and `_getConstProps`
- Other (non-spread) props can coexist alongside the spread
- flags = `0` for spread elements

### 2.4 Rust Function: `build_jsx_split_call()`

```rust
/// Build a _jsxSplit() call expression for elements with spread props.
///
/// _jsxSplit has the same signature as _jsxSorted, but the varProps object
/// contains { ..._getVarProps(source) } and constProps is _getConstProps(source).
///
/// Arguments:
/// - `tag_expr`: Tag expression (string literal or identifier)
/// - `spread_source_name`: The identifier being spread (e.g., "props", "rest")
/// - `additional_var_props`: Extra variable props beyond the spread (Vec of property pairs)
/// - `children_expr`: Children expression or None
/// - `flags`: Flags value
/// - `key_value`: Optional key string
/// - `ctx`: TraverseCtx
pub fn build_jsx_split_call<'a>(
    tag_expr: Expression<'a>,
    spread_source_name: &str,
    children_expr: Option<Expression<'a>>,
    flags: u32,
    key_value: Option<&str>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let mut arguments = ctx.ast.vec_with_capacity(6);

    // Arg 1: tag
    arguments.push(Argument::from(tag_expr));

    // Arg 2: varProps = { ..._getVarProps(source) }
    let get_var_props_call = build_helper_call(
        "_getVarProps",
        spread_source_name,
        ctx,
    );
    let spread = ctx.ast.spread_element(SPAN, get_var_props_call);
    let mut properties = ctx.ast.vec_with_capacity(1);
    properties.push(ObjectPropertyKind::SpreadProperty(
        ctx.ast.alloc(spread),
    ));
    let var_props_obj = ctx.ast.expression_object(SPAN, properties, None);
    arguments.push(Argument::from(var_props_obj));

    // Arg 3: constProps = _getConstProps(source)
    let get_const_props_call = build_helper_call(
        "_getConstProps",
        spread_source_name,
        ctx,
    );
    arguments.push(Argument::from(get_const_props_call));

    // Arg 4: children
    arguments.push(Argument::from(
        children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN)),
    ));

    // Arg 5: flags
    arguments.push(Argument::from(
        ctx.ast.expression_numeric_literal(
            SPAN,
            flags as f64,
            None,
            NumberBase::Decimal,
        ),
    ));

    // Arg 6: key
    arguments.push(Argument::from(match key_value {
        Some(key) => ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(key),
            None,
        ),
        None => ctx.ast.expression_null_literal(SPAN),
    }));

    // Build callee: _jsxSplit identifier
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_jsxSplit"),
    );

    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}

/// Build a helper function call: _getVarProps(source) or _getConstProps(source)
fn build_helper_call<'a>(
    helper_name: &str,
    source_name: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom(helper_name),
    );
    let mut args = ctx.ast.vec_with_capacity(1);
    args.push(Argument::from(
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom(source_name),
        ),
    ));
    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

---

## 3. Prop Classification Algorithm

### 3.1 Overview

The optimizer classifies each JSX prop value as "variable" (goes into varProps, argument 2) or "constant" (goes into constProps, argument 3). The rules differ for **component** targets vs **native element** targets.

### 3.2 Component Prop Classification

For components (elements whose tag name starts with an uppercase letter, e.g., `<Cmp prop={value}/>`):

| Prop Value Type | Classification | Output | Example |
|-----------------|---------------|--------|---------|
| Static string literal | const | `staticText: "text"` | `staticText="text"` |
| Template literal (no dynamic parts) | const | `staticText2: \`text\`` | `staticText2={\`text\`}` |
| Static number literal | const | `staticNumber: 1` | `staticNumber={1}` |
| Static boolean literal | const | `staticBoolean: true` | `staticBoolean={true}` |
| Static expression (compile-time evaluable) | const | `staticExpr: \`text${12}\`` | `staticExpr={\`text${12}\`}` |
| Static ternary (compile-time evaluable) | const | `staticExpr2: typeof ...` | `staticExpr2={typeof ... ? 12 : 43}` |
| Signal identifier (no `.value`) | const | `signal: signal` | `signal={signal}` |
| Signal `.value` access | const (wrapped) | `signalValue: _wrapProp(signal)` | `signalValue={signal.value}` |
| Computed signal expression | const (fnSignal) | `signalComputedValue: _fnSignal(...)` | `signalComputedValue={12 + signal.value}` |
| Store property access | const (fnSignal) | `store: _fnSignal(...)` | `store={store.address.city.name}` |
| Store computed expression | const (fnSignal) | `storeComputed: _fnSignal(...)` | `storeComputed={store.x ? 'true' : 'false'}` |
| Imported dep identifier | const | `dep: dep` | `dep={dep}` |
| Imported dep property access | const | `depAccess: dep.thing` | `depAccess={dep.thing}` |
| Imported dep computed | const | `depComputed: dep.thing + 'stuff'` | `depComputed={dep.thing + 'stuff'}` |
| Global variable | var | `global: globalThing` | `global={globalThing}` |
| Global property access | var | `globalAccess: globalThing.thing` | `globalAccess={globalThing.thing}` |
| Global computed | var | `globalComputed: globalThing.thing + 'stuff'` | `globalComputed={...}` |
| Function call result | var | `noInline: signal.value()` | `noInline={signal.value()}` |
| Mixed signal + non-reactive call | var | `noInline2: signal.value + unknown()` | `noInline2={signal.value + unknown()}` |
| Mutable wrapper | var | `noInline3: mutable(signal)` | `noInline3={mutable(signal)}` |
| Mixed signal + dep | var | `noInline4: signal.value + dep` | `noInline4={signal.value + dep}` |

**Evidence:** `example_derived_signals_cmp.md` -- comprehensive prop classification example showing all categories in lines 80-103.

### 3.3 Native Element Prop Classification

For native HTML elements (elements whose tag name starts with a lowercase letter, e.g., `<div class="foo">`):

| Prop Value Type | Classification | Output | Example |
|-----------------|---------------|--------|---------|
| Static attribute value | const | `class: "renders"` | `class="renders"` |
| QRL event handler | const | `"q-e:click": qrl(...)` | `onClick$={() => ...}` |
| Signal-wrapped value (from _rawProps) | var | `id: _wrapProp(_rawProps, "id")` | `id={id}` (destructured prop) |

**Evidence:** `should_destructure_args.md` output lines 85-97 -- `_jsxSorted("div", {id: _wrapProp(_rawProps, "id")}, null, [...])` shows signal-wrapped prop in varProps for native element.

### 3.4 Rust Pseudocode: `classify_prop_value()`

```rust
use oxc_ast::ast::*;

/// Classification result for a single prop value
#[derive(Debug)]
pub enum PropClassification<'a> {
    /// Goes into constProps object
    Const(Expression<'a>),
    /// Goes into constProps, wrapped with _wrapProp
    ConstWrapped { source: Expression<'a>, prop_name: Option<String> },
    /// Goes into constProps, wrapped with _fnSignal
    ConstFnSignal { body_expr: Expression<'a>, deps: Vec<String> },
    /// Goes into varProps object (as-is)
    Var(Expression<'a>),
}

/// Classify a prop value expression for a COMPONENT target.
///
/// The classification determines whether the prop goes into varProps or constProps,
/// and whether it needs signal wrapping (_wrapProp or _fnSignal).
///
/// Key rules:
/// - Static literals -> Const
/// - Signal.value access -> ConstWrapped (_wrapProp)
/// - Computed signal/store expressions -> ConstFnSignal (_fnSignal)
/// - Imported dep access -> Const (as-is)
/// - Global variables -> Var
/// - Function call results -> Var
/// - Mixed signal + non-reactive -> Var
pub fn classify_component_prop<'a>(
    expr: &Expression<'a>,
    is_import: impl Fn(&str) -> bool,
    is_global: impl Fn(&str) -> bool,
) -> PropClassification<'a> {
    match expr {
        // Static literals -> const
        Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::TemplateLiteral(_) => {
            PropClassification::Const(/* pass through */)
        }

        // signal.value access -> const wrapped
        // Check: MemberExpression with property "value"
        Expression::StaticMemberExpression(member)
            if member.property.name == "value"
               && !has_call_parent(member) =>
        {
            // Strip .value, wrap with _wrapProp(signal)
            PropClassification::ConstWrapped {
                source: /* member.object */,
                prop_name: None,
            }
        }

        // Identifier reference
        Expression::Identifier(ident) => {
            let name = ident.name.as_str();
            if is_global(name) {
                PropClassification::Var(/* pass through */)
            } else {
                // Import or local signal/variable -> const
                PropClassification::Const(/* pass through */)
            }
        }

        // Function call -> var (always, including signal.value())
        Expression::CallExpression(_) => {
            PropClassification::Var(/* pass through */)
        }

        // Binary/conditional expressions: check if contains reactive sources
        Expression::BinaryExpression(_)
        | Expression::ConditionalExpression(_) => {
            let reactive_sources = find_reactive_sources(expr);
            let non_reactive_deps = find_non_reactive_deps(expr, &is_import);

            if reactive_sources.is_empty() {
                // Pure static or import-based expression -> const
                PropClassification::Const(/* pass through */)
            } else if !non_reactive_deps.is_empty() {
                // Mixed reactive + non-reactive -> var
                PropClassification::Var(/* pass through */)
            } else {
                // Pure reactive expression -> const fnSignal
                PropClassification::ConstFnSignal {
                    body_expr: /* the expression */,
                    deps: reactive_sources,
                }
            }
        }

        // Member expression on global -> var
        Expression::StaticMemberExpression(member) => {
            let root = get_root_identifier(member);
            if is_global(&root) {
                PropClassification::Var(/* pass through */)
            } else if is_import(&root) {
                PropClassification::Const(/* pass through */)
            } else {
                // Store property chain -> const fnSignal
                PropClassification::ConstFnSignal {
                    body_expr: /* the expression */,
                    deps: vec![root],
                }
            }
        }

        // Default: var (safe fallback)
        _ => PropClassification::Var(/* pass through */)
    }
}
```

### 3.5 Prop Object Construction

After classifying all props, build the varProps and constProps objects:

```rust
/// Build varProps and constProps objects from classified props.
///
/// Returns (Option<varProps_expr>, Option<constProps_expr>).
/// Returns None for empty objects (replaced with null in _jsxSorted call).
fn build_prop_objects<'a>(
    classified_props: Vec<(String, PropClassification<'a>)>,
    ctx: &mut TraverseCtx<'a>,
) -> (Option<Expression<'a>>, Option<Expression<'a>>) {
    let mut var_properties = ctx.ast.vec();
    let mut const_properties = ctx.ast.vec();

    for (prop_name, classification) in classified_props {
        let key = ctx.ast.property_key_identifier_name(
            SPAN,
            ctx.ast.atom(&prop_name),
        );

        match classification {
            PropClassification::Const(value) => {
                let prop = ctx.ast.object_property_kind_object_property(
                    SPAN, PropertyKind::Init, key, value,
                    false, false, false,
                );
                const_properties.push(prop);
            }
            PropClassification::ConstWrapped { source, prop_name: opt_name } => {
                let wrapped = build_wrap_prop_call(source, opt_name.as_deref(), ctx);
                let prop = ctx.ast.object_property_kind_object_property(
                    SPAN, PropertyKind::Init, key, wrapped,
                    false, false, false,
                );
                const_properties.push(prop);
            }
            PropClassification::ConstFnSignal { body_expr, deps } => {
                let fn_signal = build_fn_signal_call(body_expr, &deps, ctx);
                let prop = ctx.ast.object_property_kind_object_property(
                    SPAN, PropertyKind::Init, key, fn_signal,
                    false, false, false,
                );
                const_properties.push(prop);
            }
            PropClassification::Var(value) => {
                let prop = ctx.ast.object_property_kind_object_property(
                    SPAN, PropertyKind::Init, key, value,
                    false, false, false,
                );
                var_properties.push(prop);
            }
        }
    }

    let var_obj = if var_properties.is_empty() {
        None
    } else {
        Some(ctx.ast.expression_object(SPAN, var_properties, None))
    };

    let const_obj = if const_properties.is_empty() {
        None
    } else {
        Some(ctx.ast.expression_object(SPAN, const_properties, None))
    };

    (var_obj, const_obj)
}
```

---

## 4. Fragment Handling

### 4.1 Fragment Import

JSX fragments (`<>...</>`) are transformed to `_jsxSorted(_Fragment, null, null, children, flags, key)`. The `_Fragment` identifier must be imported with an alias from a different module than the standard `@qwik.dev/core`:

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
```

Note the import source: `@qwik.dev/core/jsx-runtime` (NOT `@qwik.dev/core`).

**Evidence:** `example_jsx.md` output line 1158, `example_getter_generation.md` output line 79

### 4.2 Fragment Transformation

```tsx
// Input:
<>
    <div/>
    <button {...props}/>
</>

// Output (example_jsx.md lines 1160-1165):
_jsxSorted(_Fragment, null, null, [
    _jsxSorted("div", null, null, null, 3, null),
    _jsxSplit("button", { ..._getVarProps(props) }, _getConstProps(props), null, 0, null)
], 1, "u6_0")
```

Key observations:
- Fragment tag is the `_Fragment` identifier (not a string literal)
- Fragments always have null for varProps and constProps
- Children follow the standard array encoding
- Fragments can have keys (e.g., `"u6_0"`) assigned by the optimizer

### 4.3 Rust Code: Fragment Import Construction

```rust
/// Build the Fragment aliased import:
///   import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime"
pub fn build_fragment_import<'a>(
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    // Build imported name: Fragment
    let imported = ctx.ast.module_export_name_identifier_name(
        SPAN,
        ctx.ast.atom("Fragment"),
    );

    // Build local binding: _Fragment (the alias)
    let local = ctx.ast.binding_identifier(
        SPAN,
        ctx.ast.atom("_Fragment"),
    );

    // Build import specifier with alias: Fragment as _Fragment
    let specifier = ctx.ast.import_specifier(
        SPAN,
        imported,
        local,
        ImportOrExportKind::Value,
    );

    let specifiers = ctx.ast.vec1(
        ImportDeclarationSpecifier::ImportSpecifier(specifier),
    );

    // Build source: "@qwik.dev/core/jsx-runtime" (different from @qwik.dev/core)
    let source = ctx.ast.string_literal(
        SPAN,
        ctx.ast.atom("@qwik.dev/core/jsx-runtime"),
        None,
    );

    let import_decl = ctx.ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source,
        NONE,
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}
```

---

## 5. Event Handler Transformation

### 5.1 Name Transformation Rule

JSX event attributes with `$` suffix are transformed to QRL attribute props with a `q-e:` prefix:

| Input Attribute | Output Key | Prefix Rule |
|----------------|-----------|-------------|
| `onClick$` | `"q-e:click"` | Strip `$`, add `q-e:`, lowercase event name |
| `onInput$` | `"q-e:input"` | Strip `$`, add `q-e:`, lowercase event name |
| `onMouseOver$` | `"q-e:mouseover"` | Strip `$`, add `q-e:`, lowercase event name |

### 5.2 Async Event Handlers (QRL)

Async event handlers (standard `$()` pattern) produce `qrl()` calls that go into **constProps**:

```tsx
// Input:
<div onClick$={() => console.log('parent')}>

// Output:
_jsxSorted("div", null, {
    "q-e:click": qrl(i_HASH, "s_HASH")
}, ...)
```

The event handler body is extracted as a segment (following the standard `$()` extraction pattern from Phase 4 API-MAPPING.md Pattern 1 and 2).

**Evidence:** `example_jsx.md` output (implicit through segment extraction), Phase 4 API-MAPPING.md

### 5.3 Sync Event Handlers (_qrlSync)

Sync handlers (`sync$()`) produce `_qrlSync()` calls that go into **varProps**:

```tsx
// Input:
<input onClick$={sync$(function(event, target) {
    event.preventDefault();
})}/>

// Output:
_jsxSorted("input", {
    "q-e:click": _qrlSync(function(event, target) {
        event.preventDefault();
    }, "function(event,target){event.preventDefault();}")
}, null, null, 2, null)
```

Key differences from async:
- `_qrlSync` goes into varProps (not constProps)
- The function body is kept inline (not extracted to a segment)
- A stringified representation is added as the second argument
- flags = `2` for event-only elements

**Evidence:** `example_of_synchronous_qrl.md` output lines 791-804 (referenced in 06-RESEARCH.md Section 8)

### 5.4 Rust Pseudocode: Event Name Transformation

```rust
/// Transform a JSX event attribute name to its output form.
///
/// Examples:
///   "onClick$"     -> "q-e:click"
///   "onInput$"     -> "q-e:input"
///   "onMouseOver$" -> "q-e:mouseover"
///
/// Returns None if the attribute is not an event handler.
pub fn transform_event_name(attr_name: &str) -> Option<String> {
    if !attr_name.starts_with("on") || !attr_name.ends_with('$') {
        return None;
    }

    // Strip "on" prefix and "$" suffix
    let event_part = &attr_name[2..attr_name.len() - 1];

    // Lowercase the event name
    let event_lower = event_part.to_lowercase();

    Some(format!("q-e:{}", event_lower))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_name_transformation() {
        assert_eq!(transform_event_name("onClick$"), Some("q-e:click".into()));
        assert_eq!(transform_event_name("onInput$"), Some("q-e:input".into()));
        assert_eq!(transform_event_name("onMouseOver$"), Some("q-e:mouseover".into()));
        assert_eq!(transform_event_name("class"), None);
        assert_eq!(transform_event_name("onClick"), None); // No $ suffix
    }
}
```

---

## 6. Dev Mode JSX

### 6.1 Dev Mode 7th Argument

In Dev mode, `_jsxSorted` receives a 7th argument with the source location of the JSX element in the original file:

```javascript
_jsxSorted("button", null, {
    "q-e:click": qrlDEV(...)
}, null, 3, "u6_0", {
    fileName: "test.tsx",
    lineNumber: 27,
    columnNumber: 3
});
```

**Evidence:** `example_drop_side_effects.md` output lines 134-145 (referenced in 06-RESEARCH.md Section 1.8)

### 6.2 Rust Code: Build Dev Info Object

```rust
/// Build the dev mode source location object:
///   { fileName: "test.tsx", lineNumber: 27, columnNumber: 3 }
///
/// Arguments:
/// - `file_name`: The source file name
/// - `line`: 1-based line number of the JSX element
/// - `column`: 0-based column number of the JSX element
/// - `ctx`: TraverseCtx
pub fn build_dev_info<'a>(
    file_name: &str,
    line: u32,
    column: u32,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let mut properties = ctx.ast.vec_with_capacity(3);

    // Property 1: fileName: "test.tsx"
    let file_key = ctx.ast.property_key_identifier_name(
        SPAN,
        ctx.ast.atom("fileName"),
    );
    let file_value = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(file_name),
        None,
    );
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN, PropertyKind::Init, file_key, file_value,
        false, false, false,
    ));

    // Property 2: lineNumber: N
    let line_key = ctx.ast.property_key_identifier_name(
        SPAN,
        ctx.ast.atom("lineNumber"),
    );
    let line_value = ctx.ast.expression_numeric_literal(
        SPAN,
        line as f64,
        None,
        NumberBase::Decimal,
    );
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN, PropertyKind::Init, line_key, line_value,
        false, false, false,
    ));

    // Property 3: columnNumber: N
    let col_key = ctx.ast.property_key_identifier_name(
        SPAN,
        ctx.ast.atom("columnNumber"),
    );
    let col_value = ctx.ast.expression_numeric_literal(
        SPAN,
        column as f64,
        None,
        NumberBase::Decimal,
    );
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN, PropertyKind::Init, col_key, col_value,
        false, false, false,
    ));

    ctx.ast.expression_object(SPAN, properties, None)
}
```

### 6.3 When to Include Dev Info

The dev info argument is included when the transform options indicate Dev mode is enabled. In Prod mode, `_jsxSorted` is called with only 6 arguments. The calling code checks a `dev_mode: bool` flag:

```rust
let dev_info = if options.is_dev_mode {
    let (line, col) = source_text_line_col(jsx_element.span.start);
    Some(build_dev_info(&options.filename, line, col, ctx))
} else {
    None
};

build_jsx_sorted_call(tag, var_props, const_props, children, flags, key, dev_info, ctx)
```

---

## 7. Children Encoding Rules

### 7.1 Encoding Table

| Children Pattern | Output | Example |
|-----------------|--------|---------|
| No children | `null` (4th argument) | `<div/>` -> `..., null, 3, null` |
| Single child expression | Direct expression | `<div>{value}</div>` -> `..., value, 1, null` |
| Single text content | String literal | `<div>12</div>` -> `..., "12", 3, null` |
| Multiple children | Array expression | `<div>{a}{b}</div>` -> `..., [a, b], 1, null` |
| Mixed text + expression | Array with string literals | `<p>Value {count}</p>` -> `..., ["Value ", count], 1, null` |

### 7.2 Evidence From Spec Files

**No children** (`example_jsx.md` line 1161):
```javascript
_jsxSorted("div", null, null, null, 3, null)
```

**Single text child** (`example_jsx.md` line 2119):
```javascript
_jsxSorted("div", null, { class: "class" }, "12", 3, null)
```

**Single expression child** (`example_getter_generation.md` line 89):
```javascript
_jsxSorted("p", { "data-value": _wrapProp(props, "count") }, null,
    _fnSignal(_hf0, [props], _hf0_str), 1, null)
```

**Multiple children** (`example_getter_generation.md` lines 90-94):
```javascript
_jsxSorted("p", null, null, [
    "Value ",
    _wrapProp(props, "count"),
    _jsxSorted("span", null, null, null, 3, null)
], 1, null)
```

### 7.3 Rust Code: Children Encoding

```rust
/// Encode JSX children into the appropriate expression format.
///
/// - 0 children -> None (will become null)
/// - 1 child -> Some(child_expression)
/// - 2+ children -> Some(array_expression)
fn encode_children<'a>(
    children: Vec<Expression<'a>>,
    ctx: &mut TraverseCtx<'a>,
) -> Option<Expression<'a>> {
    match children.len() {
        0 => None,
        1 => Some(children.into_iter().next().unwrap()),
        _ => {
            let mut elements = ctx.ast.vec_with_capacity(children.len());
            for child in children {
                elements.push(ArrayExpressionElement::from(child));
            }
            Some(ctx.ast.expression_array(SPAN, elements, None))
        }
    }
}

/// Convert a JSX text node to a string literal expression.
/// JSX text whitespace is normalized (leading/trailing newlines stripped,
/// internal whitespace collapsed).
fn jsx_text_to_expression<'a>(
    text: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Option<Expression<'a>> {
    let trimmed = normalize_jsx_text(text);
    if trimmed.is_empty() {
        None
    } else {
        Some(ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(&trimmed),
            None,
        ))
    }
}
```

---

## 8. Flags Semantics

### 8.1 Flags Value Table

The `flags` argument to `_jsxSorted` / `_jsxSplit` encodes information about the element's children and properties:

| flags | When Used | Observed Context |
|-------|-----------|------------------|
| `0` | Element has spread props | `_jsxSplit` calls: `<button {...props}/>` |
| `1` | Element has multiple children or Fragment with content | `_jsxSorted` with array children, Fragment containers |
| `2` | Element has only sync event handlers | `_qrlSync` event-only elements |
| `3` | Self-closing element, single-child element, or leaf element | Most native elements: `<div/>`, `<span>text</span>` |

**Evidence:**
- `0` with spread: `example_jsx.md` line 1164 (`_jsxSplit("button", ..., 0, null)`)
- `1` with array children: `example_getter_generation.md` line 89, 94 (`_jsxSorted("p", ..., 1, null)`)
- `1` with Fragment: `example_getter_generation.md` line 86 (`_jsxSorted(_Fragment, ..., 1, "u6_1")`)
- `2` with sync event: `example_of_synchronous_qrl.md` (referenced in 06-RESEARCH.md Section 8)
- `3` self-closing: `example_jsx.md` line 1161 (`_jsxSorted("div", null, null, null, 3, null)`)
- `3` with single child: `example_derived_signals_cmp.md` line 103 (`_jsxSorted(Cmp, ..., 3, "u6_0")`)
- `3` with children array: `example_jsx.md` line 2120 (`_jsxSorted(_Fragment, ..., 3, "u6_2")`) -- Fragment with static children

### 8.2 Flags Determination Algorithm

```rust
/// Determine the flags value for a JSX element.
fn determine_flags(
    has_spread: bool,
    has_sync_event_only: bool,
    children_count: usize,
    is_fragment_with_static_children: bool,
) -> u32 {
    if has_spread {
        0
    } else if has_sync_event_only {
        2
    } else if children_count > 1 && !is_fragment_with_static_children {
        1
    } else {
        3
    }
}
```

Note: The exact flags semantics have some nuance (see 06-RESEARCH.md Section 13.2). The values `3` and `1` appear to overlap in some cases. The pattern-matched rules above cover all observed spec file outputs. The runtime behavior is: `3` = leaf/simple, `1` = container with dynamic content, `0` = spread, `2` = event-only.

---

## 9. Import Management

### 9.1 Required Imports

The JSX transformation pass may generate the following imports, each added conditionally based on usage:

| Import | Source | When Needed |
|--------|--------|-------------|
| `_jsxSorted` | `@qwik.dev/core` | Always (any JSX element present) |
| `_jsxSplit` | `@qwik.dev/core` | When any element has spread props |
| `Fragment as _Fragment` | `@qwik.dev/core/jsx-runtime` | When `<>...</>` fragments are used |
| `_getVarProps` | `@qwik.dev/core` | When spread props are present |
| `_getConstProps` | `@qwik.dev/core` | When spread props are present |
| `_wrapProp` | `@qwik.dev/core` | When signal/store prop wrapping occurs |
| `_fnSignal` | `@qwik.dev/core` | When computed signal expressions are hoisted |

### 9.2 Import Tracking During Traversal

Following the collect-during-traversal + build-in-exit_program pattern established in Phase 4 API-MAPPING.md:

```rust
/// Tracks JSX-specific imports needed during transformation.
#[derive(Default)]
pub struct JsxImportTracker {
    pub needs_jsx_sorted: bool,
    pub needs_jsx_split: bool,
    pub needs_fragment: bool,
    pub needs_get_var_props: bool,
    pub needs_get_const_props: bool,
    pub needs_wrap_prop: bool,
    pub needs_fn_signal: bool,
}

impl JsxImportTracker {
    /// Record that a _jsxSorted call was generated
    pub fn record_jsx_sorted(&mut self) {
        self.needs_jsx_sorted = true;
    }

    /// Record that a _jsxSplit call was generated (spread props present)
    pub fn record_jsx_split(&mut self) {
        self.needs_jsx_split = true;
        self.needs_get_var_props = true;
        self.needs_get_const_props = true;
    }

    /// Record that a Fragment was used
    pub fn record_fragment(&mut self) {
        self.needs_fragment = true;
    }

    /// Build all import statements in exit_program
    pub fn build_imports<'a>(
        &self,
        ctx: &mut TraverseCtx<'a>,
    ) -> Vec<Statement<'a>> {
        let mut imports = Vec::new();

        if self.needs_jsx_sorted {
            imports.push(build_named_import("_jsxSorted", "@qwik.dev/core", ctx));
        }
        if self.needs_jsx_split {
            imports.push(build_named_import("_jsxSplit", "@qwik.dev/core", ctx));
        }
        if self.needs_get_var_props {
            imports.push(build_named_import("_getVarProps", "@qwik.dev/core", ctx));
        }
        if self.needs_get_const_props {
            imports.push(build_named_import("_getConstProps", "@qwik.dev/core", ctx));
        }
        if self.needs_wrap_prop {
            imports.push(build_named_import("_wrapProp", "@qwik.dev/core", ctx));
        }
        if self.needs_fn_signal {
            imports.push(build_named_import("_fnSignal", "@qwik.dev/core", ctx));
        }
        if self.needs_fragment {
            imports.push(build_fragment_import(ctx));
        }

        imports
    }
}
```

### 9.3 Import Evidence From Spec Files

**`example_jsx.md` main module imports (lines 1151-1158):**
```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
```

**`example_derived_signals_cmp.md` imports (lines 63-67):**
```javascript
import { componentQrl } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
```

**`should_destructure_args.md` segment imports (lines 66-71):**
```javascript
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { _restProps } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
```

---

## 10. Complete JSX Transformation Flow

### 10.1 Detection

In the Traverse implementation, JSX elements are detected via `enter_jsx_element` and `enter_jsx_fragment`:

```rust
impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_jsx_element(
        &mut self,
        jsx: &mut JSXElement<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // Record the JSX element for transformation in exit_expression
        // Collect element name, attributes, children
    }

    fn enter_jsx_fragment(
        &mut self,
        jsx: &mut JSXFragment<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // Record the fragment for transformation
        self.jsx_import_tracker.record_fragment();
    }
}
```

### 10.2 Replacement

The actual replacement happens by replacing the JSX expression with the constructed `_jsxSorted` / `_jsxSplit` call:

```rust
impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn exit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        match expr {
            Expression::JSXElement(jsx) => {
                // 1. Resolve tag
                let tag = build_tag_expression(&jsx.opening_element.name, ctx);

                // 2. Process attributes -> classify into var/const props
                let (var_props, const_props) = process_jsx_attributes(
                    &jsx.opening_element.attributes,
                    &tag,
                    ctx,
                );

                // 3. Encode children
                let children = encode_jsx_children(&jsx.children, ctx);

                // 4. Determine flags
                let flags = determine_flags(has_spread, has_sync_only, children_count, false);

                // 5. Check for spread -> _jsxSplit vs _jsxSorted
                if has_spread {
                    *expr = build_jsx_split_call(tag, spread_source, children, flags, key, ctx);
                    self.jsx_import_tracker.record_jsx_split();
                } else {
                    let dev_info = if self.is_dev_mode {
                        Some(build_dev_info(&self.filename, line, col, ctx))
                    } else {
                        None
                    };
                    *expr = build_jsx_sorted_call(
                        tag, var_props, const_props, children, flags, key, dev_info, ctx
                    );
                    self.jsx_import_tracker.record_jsx_sorted();
                }
            }
            Expression::JSXFragment(jsx) => {
                // Fragment: _jsxSorted(_Fragment, null, null, children, flags, key)
                let tag = ctx.ast.expression_identifier_reference(
                    SPAN, ctx.ast.atom("_Fragment"),
                );
                let children = encode_jsx_children(&jsx.children, ctx);
                let flags = if children_count > 1 { 1 } else { 3 };
                *expr = build_jsx_sorted_call(
                    tag, None, None, children, flags, key, None, ctx
                );
                self.jsx_import_tracker.record_jsx_sorted();
                self.jsx_import_tracker.record_fragment();
            }
            _ => {}
        }
    }
}
```

### 10.3 Cross-References

- **Signal helpers** (`_wrapProp`, `_fnSignal`) generated during prop classification are documented in detail in **PROPS-SIGNALS-MAPPING.md** (APIM-09)
- **QRL construction** for event handlers follows Phase 4 **API-MAPPING.md** Pattern 2
- **Hoisted functions** (`_hfN`, `_hfN_str`) placement documented in **PROPS-SIGNALS-MAPPING.md** Section 4
- **PURE annotations** (`/*#__PURE__*/`) are applied to all `_jsxSorted` and `_jsxSplit` calls (see Phase 4 API-MAPPING.md PURE Annotation Strategy)

---

*This document satisfies APIM-04: JSX transformation patterns mapped to OXC expression replacement APIs. It covers CONV-03 (JSX Transforms) with complete OXC AstBuilder Rust code examples for _jsxSorted, _jsxSplit, prop classification, Fragment handling, event handler transformation, dev mode, children encoding, and import management.*
