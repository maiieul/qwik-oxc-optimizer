# Props Destructuring & Signal Optimization: OXC API Mapping (APIM-09)

**Date:** 2026-02-10
**Purpose:** A standalone, implementable specification for props destructuring and signal optimization using OXC AstBuilder APIs. A developer reading this document can implement the complete props destructuring transformation (`_rawProps`, `_restProps`) and all signal optimization patterns (`_wrapProp`, `_fnSignal`, hoisted functions) without referencing the SWC source code.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-09 | Entire document | Props destructuring and signal optimization mapped to OXC APIs |
| CONV-11 | Sections 1, 6 | Props destructuring detection, transformation, and ordering |
| CONV-04 | Sections 2, 3, 5 | Signal helpers: `_wrapProp`, `_fnSignal`, non-wrapping rules |
| CONV-14 | Section 4 | Hoisted function placement and construction |

---

## 1. Props Destructuring Transform

### 1.1 Overview

When a `component$()` callback argument uses object destructuring, the optimizer replaces the destructured parameter with `_rawProps` and transforms all property accesses to use `_wrapProp(_rawProps, "propName")` for reactive tracking.

### 1.2 Before/After: `should_destructure_args.md`

**Input:**
```tsx
export default component$(({ message, id, count: c, ...rest }: Record<string, any>) => {
    const renders = useStore({ renders: 0 }, { reactive: false });
    renders.renders++;
    const rerenders = renders.renders + 0;
    return (
        <div id={id}>
            <span {...rest}>
                {message} {c}
            </span>
            <div class="renders">{rerenders}</div>
        </div>
    );
});
```

**Output (segment, lines 72-99):**
```javascript
export const test_component_LUXeXe0DQrg = (_rawProps)=>{
    const rest = _restProps(_rawProps, [
        "message",
        "id",
        "count"
    ]);
    const renders = useStore({
        renders: 0
    }, {
        reactive: false
    });
    renders.renders++;
    const rerenders = renders.renders + 0;
    return _jsxSorted("div", {
        id: _wrapProp(_rawProps, "id")
    }, null, [
        _jsxSplit("span", {
            ..._getVarProps(rest)
        }, _getConstProps(rest), [
            _wrapProp(_rawProps, "message"),
            " ",
            _wrapProp(_rawProps, "count")
        ], 0, null),
        _jsxSorted("div", null, {
            class: "renders"
        }, rerenders, 1, null)
    ], 1, "u6_0");
};
```

**Segment metadata (`should_destructure_args.md` lines 111-126):**
```json
{
    "paramNames": ["_rawProps"]
}
```

### 1.3 Transformation Algorithm

The props destructuring transform follows these steps:

1. **Detection:** In `enter_arrow_function_expression`, check if:
   - The parent is a `component$()` call expression
   - The first parameter is an `ObjectPattern` (destructuring)

2. **Collection:** Iterate the ObjectPattern properties to collect:
   - Named properties: `message`, `id` (simple bindings)
   - Renamed properties: `count: c` (original name `"count"`, local binding `c`)
   - Rest element: `...rest` (if present)

3. **Parameter replacement:** Replace the ObjectPattern with a single identifier `_rawProps`

4. **Reference replacement:** For each reference to a destructured binding in the function body:
   - `message` -> `_wrapProp(_rawProps, "message")`
   - `id` -> `_wrapProp(_rawProps, "id")`
   - `c` (renamed from `count`) -> `_wrapProp(_rawProps, "count")` (uses the ORIGINAL property name)

5. **Rest element insertion:** If `...rest` was present, insert at function body start:
   ```javascript
   const rest = _restProps(_rawProps, ["message", "id", "count"])
   ```
   The array lists all NAMED destructured properties (not the rest itself).

### 1.4 OXC Detection Code

```rust
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

/// Information collected from a destructured props pattern.
#[derive(Debug)]
pub struct DestructuredProps {
    /// Named properties: Vec<(original_name, local_binding_name)>
    /// For simple bindings: ("message", "message")
    /// For renamed: ("count", "c")
    pub named_props: Vec<(String, String)>,

    /// Rest element local name, if present (e.g., "rest")
    pub rest_name: Option<String>,
}

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_arrow_function_expression(
        &mut self,
        arrow: &mut ArrowFunctionExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // Check if this arrow is the direct callback of a component$() call
        if !self.is_component_dollar_callback(ctx) {
            return;
        }

        // Check if first parameter is an ObjectPattern (destructuring)
        let params = &arrow.params;
        if params.items.is_empty() {
            return;
        }

        let first_param = &params.items[0];
        match &first_param.pattern.kind {
            BindingPatternKind::ObjectPattern(obj_pattern) => {
                // Collect destructured property info
                let destructured = collect_destructured_props(obj_pattern);
                self.pending_props_destructuring = Some(destructured);
            }
            _ => {
                // Not destructured -- no transformation needed.
                // If the param is a simple identifier (e.g., `(props)`),
                // signal wrapping uses _wrapProp(props, "propName") directly
                // (see Section 2.2 Form 2).
            }
        }
    }
}

/// Collect property names and rest element from an ObjectPattern.
fn collect_destructured_props(
    obj_pattern: &ObjectPattern<'_>,
) -> DestructuredProps {
    let mut named_props = Vec::new();
    let mut rest_name = None;

    for prop in &obj_pattern.properties {
        match prop {
            BindingProperty::Property(prop) => {
                // Get the original property name (from the key)
                let original_name = match &prop.key {
                    PropertyKey::StaticIdentifier(ident) => {
                        ident.name.to_string()
                    }
                    PropertyKey::StringLiteral(s) => s.value.to_string(),
                    _ => continue,
                };

                // Get the local binding name (from the value pattern)
                let local_name = match &prop.value.kind {
                    BindingPatternKind::BindingIdentifier(ident) => {
                        ident.name.to_string()
                    }
                    _ => continue, // skip complex nested patterns
                };

                named_props.push((original_name, local_name));
            }
            _ => {}
        }
    }

    // Check for rest element: ...rest
    if let Some(rest) = &obj_pattern.rest {
        if let BindingPatternKind::BindingIdentifier(ident) = &rest.argument.kind {
            rest_name = Some(ident.name.to_string());
        }
    }

    DestructuredProps {
        named_props,
        rest_name,
    }
}
```

### 1.5 Parameter Replacement

Replace the ObjectPattern parameter with a single `_rawProps` identifier:

```rust
/// Replace a destructured parameter with _rawProps.
///
/// Before: ({ message, id, count: c, ...rest })
/// After:  (_rawProps)
fn replace_param_with_raw_props<'a>(
    arrow: &mut ArrowFunctionExpression<'a>,
    ctx: &mut TraverseCtx<'a>,
) {
    // Build new parameter: _rawProps
    let binding = ctx.ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ctx.ast.atom("_rawProps"),
    );
    let pattern = ctx.ast.binding_pattern(binding, None, false);
    let param = ctx.ast.formal_parameter(
        SPAN,
        ctx.ast.vec(),  // no decorators
        pattern,
        None,           // no accessibility
        false,          // not readonly
        false,          // not override
    );

    // Replace the params list with just _rawProps
    let new_params = ctx.ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ctx.ast.vec1(param),
        None, // no rest element (rest is handled separately in body)
    );
    arrow.params = new_params;
}
```

### 1.6 Rest Element Insertion

Insert `const rest = _restProps(_rawProps, ["message", "id", "count"])` at the function body start:

```rust
/// Build the _restProps declaration and insert at body start.
///
/// Produces: const rest = _restProps(_rawProps, ["message", "id", "count"])
fn build_rest_props_declaration<'a>(
    rest_name: &str,
    named_prop_names: &[String],
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    // Build the property names array: ["message", "id", "count"]
    let mut elements = ctx.ast.vec_with_capacity(named_prop_names.len());
    for name in named_prop_names {
        elements.push(ArrayExpressionElement::from(
            ctx.ast.expression_string_literal(
                SPAN,
                ctx.ast.atom(name),
                None,
            ),
        ));
    }
    let names_array = ctx.ast.expression_array(SPAN, elements, None);

    // Build the _restProps call: _restProps(_rawProps, ["message", ...])
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_restProps"),
    );
    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom("_rawProps"),
        ),
    ));
    args.push(Argument::from(names_array));
    let rest_call = ctx.ast.expression_call(SPAN, callee, NONE, args, false);

    // Build: const rest = _restProps(...)
    let binding = ctx.ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ctx.ast.atom(rest_name),
    );
    let pattern = ctx.ast.binding_pattern(binding, None, false);
    let declarator = ctx.ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        pattern,
        Some(rest_call),
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
```

### 1.7 Reference Replacement

After parameter replacement, every reference to a destructured binding in the function body must be replaced with `_wrapProp(_rawProps, "originalName")`:

```rust
/// Replace references to destructured bindings with _wrapProp calls.
///
/// In the Traverse implementation, when visiting IdentifierReference nodes
/// inside the component body:
///
/// 1. Check if the identifier name matches a destructured prop local name
/// 2. If yes, replace the identifier with _wrapProp(_rawProps, "originalName")
///
/// For renamed props (count: c), the local name "c" maps to
/// the original property name "count".
fn replace_destructured_reference<'a>(
    expr: &mut Expression<'a>,
    local_name: &str,
    original_name: &str,
    ctx: &mut TraverseCtx<'a>,
) {
    // Build: _wrapProp(_rawProps, "originalName")
    let wrap_prop_callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_wrapProp"),
    );
    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom("_rawProps"),
        ),
    ));
    args.push(Argument::from(
        ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(original_name),
            None,
        ),
    ));

    *expr = ctx.ast.expression_call(
        SPAN,
        wrap_prop_callee,
        NONE,
        args,
        false,
    );
}
```

### 1.8 Non-Destructured Props

When the parameter is NOT destructured (e.g., `component$((props) => ...)`), no `_rawProps` transformation occurs. Instead, prop access within JSX uses `_wrapProp(props, "propName")` directly on the original parameter name.

**Evidence:** `example_getter_generation.md` lines 85-96:
```javascript
export const Cmp_component_4ryKJTOKjWE = (props)=>{
    return _jsxSorted(_Fragment, null, null, [
        _jsxSorted("p", { "data-value": _wrapProp(props, "count") }, null,
            _fnSignal(_hf0, [props], _hf0_str), 1, null),
        _jsxSorted("p", null, null, [
            "Value ", _wrapProp(props, "count"),
            _jsxSorted("span", null, null, null, 3, null)
        ], 1, null)
    ], 1, "u6_1");
};
```

The parameter name `props` is used directly in `_wrapProp(props, "count")` -- no renaming to `_rawProps`.

---

## 2. _wrapProp Patterns

### 2.1 Overview

`_wrapProp` creates a reactive wrapper around a property access so the Qwik runtime can efficiently track and update only the affected DOM nodes. Two distinct forms exist.

### 2.2 Form 1: Direct Signal Wrapping (No Property Name)

```javascript
_wrapProp(signal)
```

**Used when:** A component prop receives `signal.value` -- the `.value` access is stripped and just the signal identifier is passed.

**Input -> Output:**
```tsx
// Input:
<Cmp signalValue={signal.value} />

// Output (example_derived_signals_cmp.md line 96):
signalValue: _wrapProp(signal)
```

**OXC Construction:**
```rust
/// Build _wrapProp(signal) -- Form 1, direct signal wrapping.
///
/// The .value access is stripped; only the signal identifier is passed.
fn build_wrap_prop_signal<'a>(
    signal_expr: Expression<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_wrapProp"),
    );
    let mut args = ctx.ast.vec_with_capacity(1);
    args.push(Argument::from(signal_expr));

    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

### 2.3 Form 2: Named Property Wrapping

```javascript
_wrapProp(source, "propertyName")
```

**Used when:** Accessing a named property reactively on a store, props object, or `_rawProps`.

**Input -> Output examples:**

| Input | Output | Spec File |
|-------|--------|-----------|
| `store.count` on component | `_wrapProp(store, "count")` | `example_getter_generation.md` line 142 |
| `props.count` on component | `_wrapProp(props, "count")` | `example_getter_generation.md` line 88, 92 |
| `id` (destructured from props) | `_wrapProp(_rawProps, "id")` | `should_destructure_args.md` line 86 |
| `message` (destructured) | `_wrapProp(_rawProps, "message")` | `should_destructure_args.md` line 91 |
| `c` (renamed from `count`) | `_wrapProp(_rawProps, "count")` | `should_destructure_args.md` line 93 |

**OXC Construction:**
```rust
/// Build _wrapProp(source, "propertyName") -- Form 2, named property wrapping.
fn build_wrap_prop_named<'a>(
    source_expr: Expression<'a>,
    prop_name: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_wrapProp"),
    );
    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(source_expr));
    args.push(Argument::from(
        ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(prop_name),
            None,
        ),
    ));

    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

### 2.4 Detection: When to Use Each Form

```rust
/// Determine which _wrapProp form to use for a prop value expression.
///
/// Form 1: signal.value -> _wrapProp(signal)
///   Detects MemberExpression with property "value" on a signal identifier.
///
/// Form 2: store.prop / props.prop / _rawProps.prop -> _wrapProp(source, "prop")
///   Detects direct property access (not chained) on a store/props identifier.
fn detect_wrap_prop_form<'a>(
    expr: &Expression<'a>,
) -> Option<WrapPropForm> {
    match expr {
        // signal.value -> Form 1
        Expression::StaticMemberExpression(member)
            if member.property.name == "value" =>
        {
            // Check: is the object a simple identifier (not a call)?
            if let Expression::Identifier(ident) = &member.object {
                return Some(WrapPropForm::Signal {
                    signal_name: ident.name.to_string(),
                });
            }
            None
        }

        // For direct single-level prop access on store/props:
        // store.count -> Form 2
        // props.count -> Form 2
        // But store.address.city.name -> _fnSignal (not _wrapProp)
        Expression::StaticMemberExpression(member) => {
            if let Expression::Identifier(ident) = &member.object {
                // Single-level property access
                Some(WrapPropForm::Named {
                    source_name: ident.name.to_string(),
                    prop_name: member.property.name.to_string(),
                })
            } else {
                // Multi-level chain -> _fnSignal territory
                None
            }
        }

        _ => None,
    }
}

#[derive(Debug)]
enum WrapPropForm {
    /// _wrapProp(signal) -- signal.value access
    Signal { signal_name: String },
    /// _wrapProp(source, "prop") -- direct property access
    Named { source_name: String, prop_name: String },
}
```

---

## 3. _fnSignal Pattern

### 3.1 Overview

For computed expressions involving reactive values (signals, stores), the optimizer hoists a getter function to the top of the module and replaces the expression with `_fnSignal(_hfN, [deps], _hfN_str)`.

### 3.2 Algorithm

1. **Identify reactive sources** in the expression (identifiers whose `.value` is accessed, or store property chains)
2. **Replace** each reactive source with a parameter `p0`, `p1`, etc.
3. **Create hoisted getter function**: `const _hfN = (p0) => EXPR_WITH_PARAMS;`
4. **Create minified string**: `const _hfN_str = "MINIFIED_EXPR";`
5. **Build the call**: `_fnSignal(_hfN, [dep0, dep1, ...], _hfN_str)`

### 3.3 Observed Pattern Table

From `example_derived_signals_cmp.md` and `example_getter_generation.md`:

| Input Expression | Hoisted Function | String Representation | Deps Array |
|------------------|------------------|-----------------------|------------|
| `12 + signal.value` | `(p0)=>12 + p0.value` | `"12+p0.value"` | `[signal]` |
| `store.address.city.name` | `(p0)=>p0.address.city.name` | `"p0.address.city.name"` | `[store]` |
| `store.address.city.name ? 'true' : 'false'` | `(p0)=>p0.address.city.name ? 'true' : 'false'` | `'p0.address.city.name?"true":"false"'` | `[store]` |
| `store.nested.count` | `(p0)=>p0.nested.count` | `"p0.nested.count"` | `[store]` |
| `store.stuff + 12` | `(p0)=>p0.stuff + 12` | `"p0.stuff+12"` | `[store]` |
| `signal.formData?.get('username')` | `(p0)=>p0.formData?.get('username')` | `'p0.formData?.get("username")'` | `[signal]` |
| `props.nested.count` | `(p0)=>p0.nested.count` | `"p0.nested.count"` | `[props]` |

**Evidence:** `example_derived_signals_cmp.md` lines 68-73, 97-99; `example_getter_generation.md` lines 83-84, 127-132

### 3.4 Reactive Source Identification

A reactive source is the **root identifier** of a property chain that involves `.value` access or a store/props property chain deeper than one level:

```rust
/// Identify reactive sources in an expression.
///
/// A reactive source is:
/// - An identifier whose .value is accessed (signal)
/// - An identifier that is the root of a multi-level property chain (store)
///
/// Returns: Vec<(root_identifier_name, parameter_name)>
fn identify_reactive_sources(
    expr: &Expression<'_>,
) -> Vec<(String, String)> {
    let mut sources = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let mut param_index = 0;

    walk_expression(expr, &mut |sub_expr| {
        let root = match sub_expr {
            // signal.value -> root is signal
            Expression::StaticMemberExpression(member)
                if member.property.name == "value" =>
            {
                get_root_identifier(&member.object)
            }
            // store.address.city.name -> root is store
            Expression::StaticMemberExpression(member) => {
                if has_chain_depth(&member, 2) {
                    get_root_identifier(sub_expr)
                } else {
                    None
                }
            }
            _ => None,
        };

        if let Some(root_name) = root {
            if seen.insert(root_name.clone()) {
                let param = format!("p{}", param_index);
                param_index += 1;
                sources.push((root_name, param));
            }
        }
    });

    sources
}
```

### 3.5 Hoisted Function Construction

```rust
/// Build a hoisted getter function declaration:
///   const _hfN = (p0) => EXPR;
///
/// Arguments:
/// - `index`: The hoisted function index (0, 1, 2, ...)
/// - `param_count`: Number of parameters (typically 1)
/// - `body_expr`: The getter expression with p0/p1 parameters substituted
/// - `ctx`: TraverseCtx
fn build_hoisted_function<'a>(
    index: usize,
    param_count: usize,
    body_expr: Expression<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let fn_name = format!("_hf{}", index);

    // Build parameters: (p0) or (p0, p1)
    let mut params_vec = ctx.ast.vec_with_capacity(param_count);
    for i in 0..param_count {
        let param_name = format!("p{}", i);
        let binding = ctx.ast.binding_pattern_kind_binding_identifier(
            SPAN,
            ctx.ast.atom(&param_name),
        );
        let pattern = ctx.ast.binding_pattern(binding, None, false);
        let param = ctx.ast.formal_parameter(
            SPAN,
            ctx.ast.vec(),
            pattern,
            None,
            false,
            false,
        );
        params_vec.push(param);
    }
    let params = ctx.ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        params_vec,
        None,
    );

    // Build arrow body: expression body (single expression, not block)
    let expr_stmt = ctx.ast.statement_expression(SPAN, body_expr);
    let body = ctx.ast.function_body(
        SPAN,
        ctx.ast.vec(),
        ctx.ast.vec1(expr_stmt),
    );

    let arrow = ctx.ast.expression_arrow_function(
        SPAN,
        true,   // expression body
        false,  // not async
        false,  // not generator
        NONE,   // no type params
        params,
        NONE,   // no return type
        body,
    );

    // Build: const _hfN = (p0) => EXPR
    let binding = ctx.ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ctx.ast.atom(&fn_name),
    );
    let pattern = ctx.ast.binding_pattern(binding, None, false);
    let declarator = ctx.ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        pattern,
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
```

### 3.6 String Representation Construction

The string representation is a minified version of the getter body. It is generated using `oxc_codegen::Codegen` in minify mode:

```rust
use oxc_codegen::{Codegen, CodegenOptions};

/// Build the string constant for a hoisted function:
///   const _hfN_str = "MINIFIED_EXPR";
///
/// The string is generated by running Codegen in minify mode on the getter
/// body expression. This produces a compact representation like:
///   "12+p0.value"
///   "p0.address.city.name"
///   'p0.formData?.get("username")'
fn build_hoisted_string<'a>(
    index: usize,
    minified_body: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let str_name = format!("_hf{}_str", index);

    let string_value = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(minified_body),
        None,
    );

    let binding = ctx.ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ctx.ast.atom(&str_name),
    );
    let pattern = ctx.ast.binding_pattern(binding, None, false);
    let declarator = ctx.ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        pattern,
        Some(string_value),
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

/// Generate the minified string for a getter body expression.
///
/// Uses oxc_codegen in minify mode (whitespace-only minification).
/// The body expression is wrapped in a temporary Program for codegen.
///
/// Example outputs:
///   12 + p0.value      -> "12+p0.value"
///   p0.address.city.name -> "p0.address.city.name"
///   p0.stuff + 12      -> "p0.stuff+12"
fn minify_expression_to_string(
    body_expr: &Expression<'_>,
    allocator: &Allocator,
) -> String {
    // Create a minimal program containing just the expression
    // Run Codegen with minify options
    // Strip trailing semicolons/newlines
    // Return the minified string
    let codegen = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            ..Default::default()
        });

    // The exact implementation depends on how to serialize a single expression.
    // In practice: wrap in ExpressionStatement, codegen, strip trailing ";".
    todo!("Wrap expression in minimal Program, codegen, extract string")
}
```

### 3.7 _fnSignal Call Construction

```rust
/// Build the _fnSignal call expression:
///   _fnSignal(_hfN, [dep0, dep1], _hfN_str)
///
/// Arguments:
/// - `index`: The hoisted function index
/// - `deps`: Names of reactive source identifiers
/// - `ctx`: TraverseCtx
fn build_fn_signal_call<'a>(
    index: usize,
    deps: &[String],
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("_fnSignal"),
    );

    let mut arguments = ctx.ast.vec_with_capacity(3);

    // Arg 1: _hfN identifier reference
    let hf_name = format!("_hf{}", index);
    arguments.push(Argument::from(
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom(&hf_name),
        ),
    ));

    // Arg 2: deps array [signal, store, ...]
    let mut dep_elements = ctx.ast.vec_with_capacity(deps.len());
    for dep_name in deps {
        dep_elements.push(ArrayExpressionElement::from(
            ctx.ast.expression_identifier_reference(
                SPAN,
                ctx.ast.atom(dep_name),
            ),
        ));
    }
    arguments.push(Argument::from(
        ctx.ast.expression_array(SPAN, dep_elements, None),
    ));

    // Arg 3: _hfN_str identifier reference
    let str_name = format!("_hf{}_str", index);
    arguments.push(Argument::from(
        ctx.ast.expression_identifier_reference(
            SPAN,
            ctx.ast.atom(&str_name),
        ),
    ));

    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}
```

---

## 4. Hoisted Function Placement

### 4.1 Placement Rules (CONV-14)

Hoisted functions (`_hfN` and `_hfN_str`) are placed at the **top of the module**, after imports but before export declarations:

```javascript
// Imports come first
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";

// Hoisted functions next
const _hf0 = (p0)=>p0.nested.count;
const _hf0_str = "p0.nested.count";
const _hf1 = (p0)=>p0.stuff + 12;
const _hf1_str = "p0.stuff+12";
const _hf2 = (p0)=>p0.formData?.get('username');
const _hf2_str = 'p0.formData?.get("username")';

// Exported segment function last
export const App_component_ckEPmXZlub0 = ()=>{ ... };
```

**Evidence:** `example_getter_generation.md` lines 127-132, `example_derived_signals_cmp.md` lines 68-73

### 4.2 Numbering

- Each module (main or segment) has its own counter starting at `_hf0`
- Each `_hfN` is paired with a matching `_hfN_str` string constant
- The same `_hf0` name can appear in different segment modules (each is a separate file)

### 4.3 OXC API: Insertion

```rust
/// Insert hoisted function declarations into a program body.
///
/// Uses program.body.insert() to place declarations after imports
/// but before export declarations.
fn insert_hoisted_functions<'a>(
    program: &mut Program<'a>,
    hoisted: &[(Statement<'a>, Statement<'a>)], // pairs of (_hfN, _hfN_str)
) {
    // Find insertion index: after last import declaration
    let insertion_index = program.body.iter()
        .position(|stmt| !matches!(stmt, Statement::ImportDeclaration(_)))
        .unwrap_or(0);

    // Insert each pair at the insertion index
    // Insert in reverse order to maintain _hf0, _hf1, _hf2 ordering
    for (i, (fn_decl, str_decl)) in hoisted.iter().enumerate().rev() {
        let idx = insertion_index;
        program.body.insert(idx, str_decl.clone());
        program.body.insert(idx, fn_decl.clone());
    }
}
```

### 4.4 Per-Segment Numbering Example

In `example_getter_generation.md`, both the `App` and `Cmp` segments have their own `_hf0`:

**App segment (lines 127-132):**
```javascript
const _hf0 = (p0)=>p0.nested.count;     // store.nested.count
const _hf0_str = "p0.nested.count";
const _hf1 = (p0)=>p0.stuff + 12;       // store.stuff + 12
const _hf1_str = "p0.stuff+12";
const _hf2 = (p0)=>p0.formData?.get('username');
const _hf2_str = 'p0.formData?.get("username")';
```

**Cmp segment (lines 83-84):**
```javascript
const _hf0 = (p0)=>p0.nested.count;     // props.nested.count
const _hf0_str = "p0.nested.count";
```

Both segments independently number from `_hf0`.

---

## 5. Non-Wrapping Rules

### 5.1 Complete Decision Tree

Not all reactive-looking expressions get wrapped. The following cases produce **plain values** without `_wrapProp` or `_fnSignal`:

| Expression Type | Classification | Location | Reason |
|-----------------|---------------|----------|--------|
| Global variable | var (as-is) | varProps | Global reference, not trackable |
| Global property access | var (as-is) | varProps | Root is global, not reactive |
| Global computed expression | var (as-is) | varProps | Contains global reference |
| Function call result | var (as-is) | varProps | Call result not trackable |
| Mixed signal + non-reactive call | var (as-is) | varProps | Contains non-trackable sub-expression |
| Mutable wrapper | var (as-is) | varProps | Explicitly marked mutable |
| Mixed signal + dep | var (as-is) | varProps | Contains non-reactive import |
| Signal identifier (no `.value`) | const (as-is) | constProps | Signal passed by reference, no wrapping needed |
| Imported dep identifier | const (as-is) | constProps | Static import, not reactive |
| Imported dep property access | const (as-is) | constProps | Static access on import |
| Imported dep computed | const (as-is) | constProps | Computed from static imports only |

**Evidence:** `example_derived_signals_cmp.md` lines 81-88 (varProps) and lines 100-101 (constProps, unwrapped)

### 5.2 Decision Flowchart

```
Is the prop value expression:

1. A literal (string, number, boolean, null, template)?
   YES -> constProps, as-is
   NO  -> continue

2. Contains a function call (signal.value(), unknown())?
   YES -> varProps, as-is (function calls are not trackable)
   NO  -> continue

3. Contains a global variable (not import, not local)?
   YES -> varProps, as-is (globals are not trackable)
   NO  -> continue

4. Contains mutable() wrapper?
   YES -> varProps, as-is
   NO  -> continue

5. Is it signal.value (direct MemberExpression, property "value")?
   YES -> constProps, _wrapProp(signal) [Form 1]
   NO  -> continue

6. Is it a direct single-level store/props property access?
   (e.g., store.count, props.count)
   YES -> constProps, _wrapProp(source, "prop") [Form 2]
   NO  -> continue

7. Is it a multi-level property chain or computed expression
   involving only reactive sources?
   (e.g., store.address.city.name, 12 + signal.value)
   YES -> constProps, _fnSignal(_hfN, [deps], _hfN_str)
   NO  -> continue

8. Is it a mixed expression (reactive + non-reactive imports/deps)?
   Contains signal.value + dep -> varProps, as-is
   Contains only imports/deps  -> constProps, as-is

9. Is it a signal identifier without .value?
   YES -> constProps, as-is (signal={signal})
   NO  -> varProps, as-is (safe fallback)
```

### 5.3 Key Distinction: signal.value vs signal

```tsx
// signal.value -> _wrapProp(signal) -- const, wrapped
<Cmp signalValue={signal.value} />
// Output: signalValue: _wrapProp(signal)

// signal (no .value) -> as-is -- const, no wrapping
<Cmp signal={signal} />
// Output: signal: signal
```

**Evidence:** `example_derived_signals_cmp.md` line 96 (`_wrapProp(signal)`) vs line 95 (`signal: signal`)

### 5.4 Key Distinction: dep.thing vs globalThing.thing

```tsx
// Import dep access -> const, as-is (imports are stable)
<Cmp depAccess={dep.thing} />
// Output: depAccess: dep.thing  (in constProps)

// Global access -> var, as-is (globals are not trackable)
<Cmp globalAccess={globalThing.thing} />
// Output: globalAccess: globalThing.thing  (in varProps)
```

**Evidence:** `example_derived_signals_cmp.md` lines 101 (`dep.thing` in constProps) vs 82 (`globalThing.thing` in varProps)

---

## 6. Ordering Constraint

### 6.1 Props Destructuring MUST Run Before Capture Analysis

Props destructuring changes variable references that capture analysis reads. If destructuring runs after capture analysis, the captures will reference the original destructured names (`message`, `id`, `c`) instead of `_rawProps`.

**Correct order:**
1. Props destructuring: `({message, id, count: c}) -> (_rawProps)`, all references updated
2. Capture analysis: sees `_rawProps` as the parameter, captures `_rawProps` if referenced in nested `$()` body

**Incorrect order (would produce wrong captures):**
1. Capture analysis: sees `message`, `id`, `c` as separate captures
2. Props destructuring: renames parameter, but capture list is already wrong

### 6.2 Implementation Strategy

From Phase 5 decision (05-01): Props destructuring runs as a **pre-pass** before the main transformation traverse, or as an early step within the same traverse (before capture analysis processing).

**Two-pass approach:**
```rust
// Pass 1: Pre-transform (props destructuring)
traverse_mut(&mut props_destructuring_pass, &allocator, &mut program, symbols, scopes);

// Re-run semantic analysis (scoping data is stale after AST mutations)
let semantic_ret = SemanticBuilder::new()
    .with_excess_capacity(2.0)
    .build(&program);
let (symbols, scopes) = semantic_ret.semantic.into_symbol_table_and_scope_tree();

// Pass 2: Main transform (capture analysis, QRL wrapping, JSX, etc.)
traverse_mut(&mut main_transform, &allocator, &mut program, symbols, scopes);
```

**Alternative single-pass approach:**
```rust
// During enter_arrow_function_expression: detect and record destructuring
// During exit_arrow_function_expression: apply the parameter replacement
// This ensures all body references are visited AFTER the replacement
// is recorded but BEFORE capture analysis runs in a subsequent pass
```

**Evidence:** Phase 5 decision from 05-01 SUMMARY and CAPTURE-ANALYSIS-MAPPING.md Section 5.8.

### 6.3 CONV Dependency Chain

```
CONV-11 (Props Destructuring) -> BEFORE -> CONV-05 (Capture Analysis)
  Reason: Destructuring changes variable references that capture analysis reads

CONV-03 (JSX Transforms) -> CONCURRENT WITH -> CONV-04 (Signal Helpers)
  Reason: Signal helpers (_wrapProp, _fnSignal) are generated during JSX prop analysis

CONV-04 (Signal Helpers) -> BEFORE -> CONV-14 (Hoisted Functions)
  Reason: _fnSignal calls reference _hfN identifiers that must be declared
```

---

## 7. Import Management

### 7.1 Required Imports

The props destructuring and signal optimization patterns may generate these imports:

| Import | Source | When Needed |
|--------|--------|-------------|
| `_wrapProp` | `@qwik.dev/core` | When signal/store prop wrapping occurs |
| `_fnSignal` | `@qwik.dev/core` | When computed signal expressions are hoisted |
| `_restProps` | `@qwik.dev/core` | When destructured props have a rest pattern (`...rest`) |
| `_getVarProps` | `@qwik.dev/core` | When spread in JSX (`{...rest}`) after destructuring |
| `_getConstProps` | `@qwik.dev/core` | When spread in JSX (`{...rest}`) after destructuring |

### 7.2 Import Evidence

**`should_destructure_args.md` segment imports (lines 66-71):**
```javascript
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { _restProps } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
```

**`example_derived_signals_cmp.md` imports (lines 64-66):**
```javascript
import { _wrapProp } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
```

**`example_getter_generation.md` segment imports (lines 80-82):**
```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
```

### 7.3 Import Tracking

Following the same collect-during-traversal + build-in-exit_program pattern:

```rust
#[derive(Default)]
pub struct PropsSignalImportTracker {
    pub needs_wrap_prop: bool,
    pub needs_fn_signal: bool,
    pub needs_rest_props: bool,
    pub needs_get_var_props: bool,
    pub needs_get_const_props: bool,
}

impl PropsSignalImportTracker {
    pub fn record_wrap_prop(&mut self) {
        self.needs_wrap_prop = true;
    }

    pub fn record_fn_signal(&mut self) {
        self.needs_fn_signal = true;
    }

    pub fn record_rest_props(&mut self) {
        self.needs_rest_props = true;
    }

    pub fn record_spread_in_jsx(&mut self) {
        self.needs_get_var_props = true;
        self.needs_get_const_props = true;
    }

    pub fn build_imports<'a>(
        &self,
        ctx: &mut TraverseCtx<'a>,
    ) -> Vec<Statement<'a>> {
        let mut imports = Vec::new();

        if self.needs_wrap_prop {
            imports.push(build_named_import("_wrapProp", "@qwik.dev/core", ctx));
        }
        if self.needs_fn_signal {
            imports.push(build_named_import("_fnSignal", "@qwik.dev/core", ctx));
        }
        if self.needs_rest_props {
            imports.push(build_named_import("_restProps", "@qwik.dev/core", ctx));
        }
        if self.needs_get_var_props {
            imports.push(build_named_import("_getVarProps", "@qwik.dev/core", ctx));
        }
        if self.needs_get_const_props {
            imports.push(build_named_import("_getConstProps", "@qwik.dev/core", ctx));
        }

        imports
    }
}
```

---

## 8. Cross-References

- **JSX prop analysis** that triggers signal wrapping is documented in **JSX-TRANSFORMS-MAPPING.md** Section 3 (Prop Classification Algorithm)
- **Capture analysis** that runs AFTER props destructuring is documented in Phase 5 **CAPTURE-ANALYSIS-MAPPING.md**
- **Segment extraction** that produces the per-module context for hoisted functions is documented in Phase 5 **MULTI-MODULE-OUTPUT-MAPPING.md**
- **Import management** overall strategy (collect-during-traversal + build-in-exit_program) is documented in Phase 4 **API-MAPPING.md** Pattern 3

---

*This document satisfies APIM-09: Props destructuring and signal optimization patterns mapped to OXC APIs. It covers CONV-11 (Props Destructuring), CONV-04 (Signal Helpers), and CONV-14 (Hoisted Functions) with complete OXC AstBuilder Rust code examples for _wrapProp, _fnSignal, _restProps, props destructuring detection and transformation, hoisted function placement, and the non-wrapping decision tree.*
