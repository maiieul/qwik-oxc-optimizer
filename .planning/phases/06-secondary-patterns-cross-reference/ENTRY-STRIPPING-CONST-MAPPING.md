# Entry Strategy, Code Stripping, and Const Folding: OXC API Mapping (APIM-10)

**Date:** 2026-02-11
**OXC Version:** 0.113.0
**Purpose:** Map all entry strategy variants, code stripping, const folding, input binding, sync$ serialization, and dev mode patterns to concrete OXC AstBuilder API call sequences. A developer reading this document can implement any of these transformation patterns without consulting SWC source code.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-10 (Entry Strategy) | Sections 1-6 | 7 entry strategy variants with OXC API construction |
| CONV-12 (Input Binding) | Section 7 | bind:value, bind:checked, bind:* transformation rules |
| CONV-13 (Sync$ Serialization) | Section 8 | _qrlSync construction and stringification approach |
| CONV-09 (Code Stripping) | Sections 10-14 | _noopQrl, nested preservation, side effect analysis, export stripping |
| CONV-10 (Const Folding) | Sections 15-16 | isServer replacement, dead branch elimination, static expression evaluation |

---

## Section 1: Entry Strategy Overview

The Qwik optimizer supports 7 entry strategies that determine how extracted `$()` segments are referenced in the main module. The critical insight is that **the OXC API calls are identical to Phase 4 API-MAPPING.md Pattern 2** -- the strategy only determines WHICH function to call (`qrl` vs `inlinedQrl`) and WHERE to place the segment body (separate file vs same file).

### Strategy Table

| Strategy | QRL Function | Import Pattern | Segment Output | Use Case |
|----------|-------------|----------------|----------------|----------|
| **Segment** (default) | `qrl()` | `const i_HASH = () => import("./file_HASH")` | Separate `.js` file | Production default |
| **Inline** | `inlinedQrl()` | None (body stays in module) | Inlined in main | Testing, SSR-only |
| **Hoist** | `inlinedQrl()` | None (body hoisted to top-level const) | Top-level function | Testing, Hoist variant |
| **Single** | `qrl()` | Single shared chunk import | Grouped into one file | Single-bundle builds |
| **Component** | `qrl()` | Per-component chunk import | Grouped by component | Component-level splitting |
| **Smart** | `qrl()` | Automatic grouping | Grouped by heuristics | Bundler-hint-driven |
| **Hook** | `qrl()` | Per-hook chunk import | Grouped by hook type | Hook-level splitting |

### Configuration-Driven Branching

The entry strategy is a **runtime configuration** (`TransformModulesOptions.entry_strategy`), not a different set of API calls. The implementation branches on strategy at the point of QRL construction:

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build the QRL replacement expression for a $()-call based on entry strategy.
///
/// This is the central dispatch point: entry_strategy determines which
/// build function to call. The rest of the optimizer is strategy-agnostic.
pub fn build_strategy_qrl<'a>(
    strategy: &EntryStrategy,
    segment_info: &SegmentInfo,
    body_expr: Option<Expression<'a>>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    match strategy {
        // Segment, Smart, Component, Hook, Single all use qrl() + lazy import
        EntryStrategy::Segment
        | EntryStrategy::Smart
        | EntryStrategy::Component
        | EntryStrategy::Hook
        | EntryStrategy::Single => {
            // Build: qrl(i_HASH, "name_HASH")
            // or:    qrl(i_HASH, "name_HASH", [captures])
            // See Phase 4 API-MAPPING.md build_qrl_call()
            build_qrl_call(
                &segment_info.import_ident,
                &segment_info.export_name,
                &segment_info.captures,
                ctx,
            )
        }
        // Inline and Hoist use inlinedQrl() with body kept in same file
        EntryStrategy::Inline | EntryStrategy::Hoist => {
            // Build: inlinedQrl(body, "name_HASH")
            // or:    inlinedQrl(body, "name_HASH", [captures])
            // See Phase 4 API-MAPPING.md build_inlined_qrl_call()
            let body = body_expr.expect("Inline/Hoist strategy requires body expression");
            build_inlined_qrl_call(
                body,
                &segment_info.export_name,
                &segment_info.captures,
                ctx,
            )
        }
    }
}
```

**Evidence:** All spec files demonstrate this branching. `example_inlined_entry_strategy.md` uses Inline, `example_derived_signals_cmp.md` uses Hoist, `example_strip_server_code.md` uses Segment, `example_manual_chunks.md` uses Smart.

---

## Section 2: Segment Strategy (Default)

The Segment strategy is the default production configuration. Each `$()` extraction becomes a separate lazy-loaded module with a dynamic `import()`.

### Output Structure

**Main module:**
```javascript
const i_HASH = () => import("./test.tsx_NAME_HASH");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HASH, "NAME_HASH"));
```

**Segment module (separate file):**
```javascript
export const NAME_HASH = () => { ... };
```

### OXC Construction

The main module requires two constructions:

**1. Lazy import declaration** (see Phase 4 API-MAPPING.md `build_lazy_import_declaration()`):
```rust
// const i_HASH = () => import("./path_HASH")
// Construction: ast.declaration_variable(Const, i_ident,
//   ast.expression_arrow_function([], import_call))
let lazy_import = build_lazy_import_declaration(hash, import_path, ctx);
```

**2. QRL call expression** (see Phase 4 API-MAPPING.md `build_qrl_call()`):
```rust
// qrl(i_HASH, "NAME_HASH")
// Construction: ast.expression_call(qrl_ident, vec![import_ref, name_str])
let qrl_expr = build_qrl_call(import_ident, export_name, &captures, ctx);
```

With captures (3-arg form):
```rust
// qrl(i_HASH, "NAME_HASH", [state])
let qrl_expr = build_qrl_call(import_ident, export_name, &["state".to_string()], ctx);
```

**Evidence:** `example_strip_server_code.md` (Prod mode, Segment strategy):
```javascript
const i_0TaiDayHrlo = ()=>import("./test.tsx_Parent_component_0TaiDayHrlo");
export const Parent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0TaiDayHrlo, "s_0TaiDayHrlo"));
```

`example_manual_chunks.md` (Smart strategy, same pattern as Segment):
```javascript
const i_1TaiDayHrlo = ()=>import("./test.tsx_Parent_component_1TaiDayHrlo");
export const Parent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_1TaiDayHrlo, "Parent_component_1TaiDayHrlo"));
```

---

## Section 3: Inline Strategy

The Inline strategy keeps the segment body in the **same file** using `inlinedQrl()`. No separate segment file is generated.

### Output Structure

```javascript
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    // segment body stays inline
    useStylesQrl(/*#__PURE__*/ inlinedQrl('somestring', "Child_component_useStyles_qBZTuFM0160"));
    const state = useStore({ count: 0 });
    useBrowserVisibleTaskQrl(/*#__PURE__*/ inlinedQrl(()=>{
        const state = _captures[0];
        state.count = thing.doStuff() + import("./sibling");
    }, "Child_component_useBrowserVisibleTask_0IGFPOyJmQA", [state]));
    return <div q-e:click={/*#__PURE__*/ inlinedQrl(()=>console.log(mongodb),
        "Child_component_div_q_e_click_cROa4sult1s")}>
    </div>;
}, "Child_component_9GyF01GDKqw"));
```

### OXC Construction

```rust
// inlinedQrl(body_fn, "NAME_HASH")           -- no captures
// inlinedQrl(body_fn, "NAME_HASH", [state])  -- with captures
//
// Construction: ast.expression_call(inlinedQrl_ident,
//   vec![body_expr, name_str, captures_array?])
let inlined_expr = build_inlined_qrl_call(body_expr, export_name, &captures, ctx);
```

Key differences from Segment:
- **No lazy import declaration** is generated
- **No separate segment file** is emitted
- The `body_expr` is the original `$()` callback, kept in place
- Nested `$()` calls within the body are also inlined (recursively)
- The `_captures` import is needed when any nested segment has captures

**Evidence:** `example_inlined_entry_strategy.md` (complete inline strategy example)

---

## Section 4: Hoist Strategy

The Hoist strategy is similar to Inline but the function body is **hoisted to a top-level const** in the same file, and `inlinedQrl()` references the hoisted function by name.

### Output Structure

```javascript
// Hoisted function body (top-level const)
const App_component_ckEPmXZlub0 = () => {
    const signal = useSignal(0);
    const store = useStore({});
    return /*#__PURE__*/ _jsxSorted(Cmp, { ... }, { ... }, null, 3, "u6_0");
};

// QRL reference (using the hoisted function identifier)
export const App = /*#__PURE__*/ componentQrl(
    /*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0")
);
```

### OXC Construction

**1. Hoist the function body to a top-level const:**
```rust
// const App_component_HASH = () => { ... };
// Construction: ast.declaration_variable(Const, binding_ident, arrow_fn)
let binding = ctx.ast.binding_pattern_kind_binding_identifier(
    SPAN, ctx.ast.atom(&segment_name),
);
let binding_pattern = ctx.ast.binding_pattern(binding, None, false);
let declarator = ctx.ast.variable_declarator(
    SPAN, VariableDeclarationKind::Const, binding_pattern, Some(arrow_fn), false,
);
let decl = ctx.ast.variable_declaration(
    SPAN, VariableDeclarationKind::Const, ctx.ast.vec1(declarator), false,
);
// Insert at module top-level before the export
```

**2. Reference the hoisted function in inlinedQrl (2-arg form):**
```rust
// inlinedQrl(App_component_HASH, "App_component_HASH")
// The first argument is an identifier reference, not the function body
let fn_ref = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom(&segment_name));
let name_lit = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(&segment_name), None);
let mut args = ctx.ast.vec_with_capacity(2);
args.push(Argument::from(fn_ref));
args.push(Argument::from(name_lit));
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("inlinedQrl"));
let inlined_call = ctx.ast.expression_call(SPAN, callee, NONE, args, false);
```

**Hoist vs Inline distinction:**
- **Inline**: `inlinedQrl(() => { body }, "NAME_HASH")` -- body is an anonymous arrow inline
- **Hoist**: `inlinedQrl(Named_HASH, "Named_HASH")` -- body is extracted to a named top-level const

In simple cases (no captures), the Hoist strategy uses the 2-arg form. With captures, the 3-arg form is used:
```javascript
inlinedQrl(fn_ref, "NAME_HASH", [captures])
```

**Evidence:** `example_derived_signals_cmp.md` (Entry Strategy: Hoist):
```javascript
const App_component_ckEPmXZlub0 = ()=>{ ... };
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

---

## Section 5: Smart/Component/Hook/Single Strategies

These four strategies are **variants of the Segment strategy**. They all use the same `qrl()` call pattern with lazy imports. The difference is in **segment grouping and file naming** (affects bundler hints), not the OXC API calls.

### Grouping Behavior

| Strategy | Grouping Rule | Impact on API Calls |
|----------|--------------|---------------------|
| **Smart** | Automatic heuristic grouping | None -- same `qrl()` pattern |
| **Component** | Group by parent component | None -- same `qrl()` pattern |
| **Hook** | Group by hook type (useTask, onClick, etc.) | None -- same `qrl()` pattern |
| **Single** | All segments in one chunk | None -- same `qrl()` pattern |

### OXC Construction

Identical to Section 2 (Segment strategy). The grouping affects:
1. The `import_path` argument to `build_lazy_import_declaration()` -- segment files may have different paths based on grouping
2. The `manualChunks` metadata passed to the bundler -- not part of the AST

```rust
// All four strategies produce the same AST:
// const i_HASH = () => import("./grouped_path_HASH");
// componentQrl(qrl(i_HASH, "NAME_HASH"))
```

**Evidence:** `example_manual_chunks.md` (Entry Strategy: Smart) -- output shows the same `qrl()` + lazy import pattern as Segment strategy, just with different file grouping in the segment metadata.

---

## Section 6: Naming Patterns

### Dev vs Prod Naming

| Mode | Segment Export Name | Example |
|------|---------------------|---------|
| **Dev/Test** | `DisplayName_HASH` (descriptive) | `App_component_ckEPmXZlub0`, `Parent_component_useTask_gDH1EtUWqBU` |
| **Prod** | `s_HASH` (short prefix) | `s_ckEPmXZlub0`, `s_gDH1EtUWqBU` |

### DisplayName Construction

The display name follows the pattern: `{parent}_{ctxName}_{suffix}_HASH` where:
- `{parent}` is the enclosing component or function name (e.g., `App`, `Parent`, `Child`)
- `{ctxName}` is derived from the `$`-call context (e.g., `component`, `useTask`, `div_q_e_click`)
- `{suffix}` is a disambiguator for multiple calls of the same type (e.g., `_1` for the second `useTask$`)
- `HASH` is an 11-character base64url hash of the segment content

### OXC API

The naming logic is NOT in the AstBuilder calls -- it is in the segment hash + display name generation logic. The AstBuilder always receives the computed name as a string:

```rust
// Same API for both dev and prod -- different input string
let name = if is_prod_mode {
    format!("s_{}", hash)           // "s_ckEPmXZlub0"
} else {
    format!("{}_{}", display, hash) // "App_component_ckEPmXZlub0"
};
let name_literal = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(&name), None);
```

**Evidence:**
- Dev mode: `example_derived_signals_cmp.md` -- `App_component_ckEPmXZlub0`
- Prod mode: `example_strip_server_code.md` -- `s_0TaiDayHrlo`, `s_gDH1EtUWqBU`

---

## Section 7: Input Binding (CONV-12)

The `bind:` directive on input elements is syntactic sugar for two-way binding. The optimizer detects `bind:` prefixed JSX attributes and expands them into value prop + event handler pairs.

### Detection

During JSX attribute processing, check if the attribute name starts with `bind:`:
```rust
// In JSX attribute visitor:
if let Some(attr_name) = get_jsx_attribute_name(attr) {
    if attr_name.starts_with("bind:") {
        let binding_name = &attr_name["bind:".len()..]; // "value", "checked", "stuff"
        handle_bind_directive(binding_name, attr_value, props, ctx);
    }
}
```

### Transformation Rules

| Directive | Output constProps | Handler | Handler Import |
|-----------|------------------|---------|----------------|
| `bind:value` | `"value": signal` + `"q-e:input": inlinedQrl(_val, "_val", [signal])` | `_val` | `@qwik.dev/core` |
| `bind:checked` | `"checked": signal` + `"q-e:input": inlinedQrl(_chk, "_chk", [signal])` | `_chk` | `@qwik.dev/core` |
| `bind:*` (other) | `"bind:stuff": signal` | None (passthrough) | N/A |

### OXC Construction for bind:value

```rust
/// Build the constProps entries for a bind:value directive.
///
/// Input:  <input bind:value={value} />
/// Output constProps: { "value": value, "q-e:input": inlinedQrl(_val, "_val", [value]) }
pub fn build_bind_value_props<'a>(
    signal_expr: Expression<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Vec<ObjectPropertyKind<'a>> {
    let mut props = Vec::new();

    // Prop 1: "value": signal
    let value_key = ctx.ast.property_key_expression(
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom("value"), None),
    );
    let value_prop = ctx.ast.object_property_kind_object_property(
        SPAN, value_key, signal_expr.clone_in(ctx.ast), None, false, false, false,
    );
    props.push(value_prop);

    // Prop 2: "q-e:input": inlinedQrl(_val, "_val", [signal])
    let event_key = ctx.ast.property_key_expression(
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom("q-e:input"), None),
    );

    // Build: inlinedQrl(_val, "_val", [signal])
    let _val_ref = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_val"));
    let _val_str = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom("_val"), None);
    let mut captures = ctx.ast.vec_with_capacity(1);
    captures.push(ArrayExpressionElement::from(signal_expr));
    let captures_array = ctx.ast.expression_array(SPAN, captures, None);

    let mut handler_args = ctx.ast.vec_with_capacity(3);
    handler_args.push(Argument::from(_val_ref));
    handler_args.push(Argument::from(_val_str));
    handler_args.push(Argument::from(captures_array));

    let inlinedQrl_callee = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom("inlinedQrl"),
    );
    let handler_call = ctx.ast.expression_call(
        SPAN, inlinedQrl_callee, NONE, handler_args, false,
    );

    let event_prop = ctx.ast.object_property_kind_object_property(
        SPAN, event_key, handler_call, None, false, false, false,
    );
    props.push(event_prop);

    props
}
```

### OXC Construction for bind:checked

Identical to `bind_value` but with:
- Key: `"checked"` instead of `"value"`
- Handler: `_chk` instead of `_val`

```rust
// Prop 1: "checked": signal
// Prop 2: "q-e:input": inlinedQrl(_chk, "_chk", [signal])
```

### OXC Construction for bind:* (generic passthrough)

```rust
// Prop: "bind:stuff": signal
// No event handler generated
let key = ctx.ast.property_key_expression(
    ctx.ast.expression_string_literal(SPAN, ctx.ast.atom("bind:stuff"), None),
);
let prop = ctx.ast.object_property_kind_object_property(
    SPAN, key, signal_expr, None, false, false, false,
);
```

### Import Requirements

When `bind:value` is present: add `import { _val } from "@qwik.dev/core"`
When `bind:checked` is present: add `import { _chk } from "@qwik.dev/core"`
Both also require `import { inlinedQrl } from "@qwik.dev/core"` for the handler wrapper.

**Evidence:** `example_input_bind.md` (output lines 694-709):
```javascript
/*#__PURE__*/ _jsxSorted("input", null, {
    "value": value,
    "q-e:input": inlinedQrl(_val, "_val", [value])
}, null, 3, null),
/*#__PURE__*/ _jsxSorted("input", null, {
    "checked": checked,
    "q-e:input": inlinedQrl(_chk, "_chk", [checked])
}, null, 3, null),
/*#__PURE__*/ _jsxSorted("input", null, {
    "bind:stuff": stuff
}, null, 3, null),
```

---

## Section 8: Sync$ Serialization (CONV-13)

`sync$()` wraps synchronous event handlers that need to be serialized as strings for SSR hydration. Unlike regular `$()` calls, `sync$()` callbacks are **NOT extracted to segments**. They stay inline and are wrapped with `_qrlSync()`.

### Detection

During `enter_call_expression`, check if the callee is `sync$`:
```rust
if let Expression::Identifier(ident) = &call.callee {
    if ident.name.as_str() == "sync$" && self.dollar_imports.contains("sync$") {
        // This is a sync$ call -- do NOT extract to segment
        // Instead, build _qrlSync(fn_expr, stringified)
        self.handle_sync_dollar(call, ctx);
    }
}
```

### Transformation Rules

1. `sync$(fn_expr)` becomes `_qrlSync(fn_expr, stringified_string)`
2. The function body is kept **inline** (not extracted)
3. A **stringified representation** of the function is added as the second argument
4. Comments are **removed** from the stringified version
5. Whitespace is **minified** in the string representation
6. `_qrlSync` results go into **varProps** (not constProps) of the JSX call
7. The `sync$` callee is removed (not renamed to `syncQrl`)

### Stringification Approach

Use `oxc_codegen::Codegen` to serialize the function body to a minified string:

```rust
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenOptions};

/// Stringify a function expression for _qrlSync's second argument.
///
/// Produces minified output with comments removed:
///   function(event,target){event.preventDefault();}
///   (event,target)=>{event.preventDefault();}
///   (event,target)=>event.preventDefault()
pub fn stringify_sync_function<'a>(
    fn_expr: &Expression<'a>,
    allocator: &Allocator,
) -> String {
    // Create a minimal program wrapping the expression
    // and use Codegen with minification to produce the string.
    //
    // Key: OXC's Codegen with minify=true will:
    //   - Remove comments
    //   - Minimize whitespace
    //   - Preserve function/arrow syntax
    let codegen = Codegen::new()
        .with_options(CodegenOptions {
            minify: true,
            ..Default::default()
        });

    // Generate the expression as a string
    // The exact API for expression-only codegen may require
    // wrapping in an ExpressionStatement inside a Program.
    // The result is then trimmed of any trailing semicolons.
    let result = codegen.build_expression(fn_expr);
    result.code
}
```

### OXC Construction

```rust
/// Build a _qrlSync(fn_expr, stringified) call expression.
///
/// Input:  sync$(function(event, target) { event.preventDefault(); })
/// Output: _qrlSync(function(event, target) { event.preventDefault(); },
///                   "function(event,target){event.preventDefault();}")
pub fn build_qrl_sync_call<'a>(
    fn_expr: Expression<'a>,
    stringified: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom("_qrlSync"),
    );

    let string_literal = ctx.ast.expression_string_literal(
        SPAN, ctx.ast.atom(stringified), None,
    );

    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(fn_expr));
    args.push(Argument::from(string_literal));

    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

### Observed Stringified Forms

| Input Syntax | Stringified Output |
|-------------|-------------------|
| `function(event, target) { event.preventDefault(); }` | `"function(event,target){event.preventDefault();}"` |
| `(event, target) => { event.preventDefault(); }` | `"(event,target)=>{event.preventDefault();}"` |
| `(event, target) => event.preventDefault()` | `"(event,target)=>event.preventDefault()"` |

Note: Comments are stripped. `// comment should be removed` does not appear in the stringified output.

### Import Requirements

When `sync$` is present: add `import { _qrlSync } from "@qwik.dev/core"`

**Evidence:** `example_of_synchronous_qrl.md` (output lines 789-806):
```javascript
/*#__PURE__*/ _jsxSorted("input", {
    "q-e:click": _qrlSync(function(event, target) {
        // comment should be removed
        event.preventDefault();
    }, "function(event,target){event.preventDefault();}")
}, null, null, 2, null),
/*#__PURE__*/ _jsxSorted("input", {
    "q-e:click": _qrlSync((event, target)=>{
        event.preventDefault();
    }, "(event,target)=>{event.preventDefault();}")
}, null, null, 2, null),
/*#__PURE__*/ _jsxSorted("input", {
    "q-e:click": _qrlSync((event, target)=>event.preventDefault(),
        "(event,target)=>event.preventDefault()")
}, null, null, 2, null)
```

---

## Section 9: Dev Mode Patterns

In Dev mode, the optimizer replaces production QRL functions with dev-instrumented variants that include source location metadata. Three constructs are affected.

### 9.1 qrlDEV (replaces qrl)

In Dev mode, `qrl()` becomes `qrlDEV()` with a 3rd argument containing debug metadata:

```javascript
qrlDEV(i_HASH, "segment_name_HASH", {
    file: "/user/qwik/src/test.tsx",
    lo: 503,
    hi: 575,
    displayName: "test.tsx_test_component"
})
```

### OXC Construction for qrlDEV

```rust
/// Build a qrlDEV(import_fn, name, debug_obj) call expression.
pub fn build_qrl_dev_call<'a>(
    import_ident_name: &str,
    segment_export_name: &str,
    debug_info: &DebugInfo,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let import_ref = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom(import_ident_name),
    );
    let name_literal = ctx.ast.expression_string_literal(
        SPAN, ctx.ast.atom(segment_export_name), None,
    );
    let debug_obj = build_debug_object(debug_info, ctx);

    let mut args = ctx.ast.vec_with_capacity(3);
    args.push(Argument::from(import_ref));
    args.push(Argument::from(name_literal));
    args.push(Argument::from(debug_obj));

    let callee = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom("qrlDEV"),
    );
    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}

/// Build the debug metadata object: { file, lo, hi, displayName }
fn build_debug_object<'a>(
    info: &DebugInfo,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let mut properties = ctx.ast.vec_with_capacity(4);

    // file: "/user/qwik/src/test.tsx"
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("file")),
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(&info.file), None),
        None, false, false, false,
    ));

    // lo: 503
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("lo")),
        ctx.ast.expression_numeric_literal(SPAN, info.lo as f64, None, NumberBase::Decimal),
        None, false, false, false,
    ));

    // hi: 575
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("hi")),
        ctx.ast.expression_numeric_literal(SPAN, info.hi as f64, None, NumberBase::Decimal),
        None, false, false, false,
    ));

    // displayName: "test.tsx_test_component"
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("displayName")),
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(&info.display_name), None),
        None, false, false, false,
    ));

    ctx.ast.expression_object(SPAN, properties, None)
}
```

**Evidence:** `example_drop_side_effects.md` (Dev mode output lines 79-84):
```javascript
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrlDEV(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg", {
    file: "/user/qwik/src/test.tsx",
    lo: 503,
    hi: 575,
    displayName: "test.tsx_test_component"
}));
```

### 9.2 _noopQrlDEV (replaces _noopQrl)

In Dev mode, stripped `$()` calls use `_noopQrlDEV()` instead of `_noopQrl()`:

```javascript
_noopQrlDEV("api_server_HASH", {
    file: "/user/qwik/src/test.tsx",
    lo: 0,
    hi: 0,
    displayName: "test.tsx_api_server"
})
```

Note the 2-arg form: name string + debug object. In Prod mode, `_noopQrl` uses only a 1-arg form: hash string.

### OXC Construction for _noopQrlDEV

```rust
/// Build _noopQrlDEV(name, debug_obj)
pub fn build_noop_qrl_dev<'a>(
    segment_name: &str,
    debug_info: &DebugInfo,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let name_lit = ctx.ast.expression_string_literal(
        SPAN, ctx.ast.atom(segment_name), None,
    );
    let debug_obj = build_debug_object(debug_info, ctx);

    let mut args = ctx.ast.vec_with_capacity(2);
    args.push(Argument::from(name_lit));
    args.push(Argument::from(debug_obj));

    let callee = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom("_noopQrlDEV"),
    );
    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

**Evidence:** `example_drop_side_effects.md` (Dev mode output lines 73-78):
```javascript
export const api = serverQrl(/*#__PURE__*/ _noopQrlDEV("api_server_JonPp043gH0", {
    file: "/user/qwik/src/test.tsx",
    lo: 0,
    hi: 0,
    displayName: "test.tsx_api_server"
}));
```

### 9.3 JSX devInfo (7th argument to _jsxSorted)

In Dev mode, `_jsxSorted` receives a 7th argument with source location:

```javascript
_jsxSorted("button", null, {
    "q-e:click": /*#__PURE__*/ qrlDEV(...)
}, null, 3, "u6_0", {
    fileName: "test.tsx",
    lineNumber: 27,
    columnNumber: 3
});
```

### OXC Construction for JSX devInfo

```rust
/// Build the JSX devInfo object: { fileName, lineNumber, columnNumber }
fn build_jsx_dev_info<'a>(
    file_name: &str,
    line: u32,
    column: u32,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let mut properties = ctx.ast.vec_with_capacity(3);

    // fileName: "test.tsx"
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("fileName")),
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(file_name), None),
        None, false, false, false,
    ));

    // lineNumber: 27
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("lineNumber")),
        ctx.ast.expression_numeric_literal(SPAN, line as f64, None, NumberBase::Decimal),
        None, false, false, false,
    ));

    // columnNumber: 3
    properties.push(ctx.ast.object_property_kind_object_property(
        SPAN,
        ctx.ast.property_key_identifier_name(SPAN, ctx.ast.atom("columnNumber")),
        ctx.ast.expression_numeric_literal(SPAN, column as f64, None, NumberBase::Decimal),
        None, false, false, false,
    ));

    ctx.ast.expression_object(SPAN, properties, None)
}
```

**Evidence:** `example_drop_side_effects.md` (output lines 134-145):
```javascript
return /*#__PURE__*/ _jsxSorted("button", null, {
    "q-e:click": /*#__PURE__*/ qrlDEV(i_qwSL5gM03T4, "test_component_button_q_e_click_qwSL5gM03T4", {
        file: "/user/qwik/src/test.tsx",
        lo: 541,
        hi: 558,
        displayName: "test.tsx_test_component_button_q_e_click"
    })
}, null, 3, "u6_0", {
    fileName: "test.tsx",
    lineNumber: 27,
    columnNumber: 3
});
```

### Dev Mode Import Requirements

When dev mode is active:
- `qrl` -> `qrlDEV` from `@qwik.dev/core`
- `inlinedQrl` -> `inlinedQrlDEV` from `@qwik.dev/core` (if inline strategy)
- `_noopQrl` -> `_noopQrlDEV` from `@qwik.dev/core` (when stripping)

---

*Sections 1-9 complete. Sections 10-18 (code stripping, const folding, import management) follow below.*
