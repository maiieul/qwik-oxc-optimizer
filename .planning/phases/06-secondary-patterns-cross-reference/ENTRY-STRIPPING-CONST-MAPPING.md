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

## Section 10: Code Stripping Overview (CONV-09)

Code stripping removes server-only code from client-side builds. When `strip_ctx_name` is configured (e.g., `["server"]`), any `$`-suffixed call whose context name matches a stripped name has its QRL body replaced with a noop placeholder.

### Configuration

```rust
pub struct TransformModulesOptions {
    /// Context names to strip. If a $-suffixed call's context name
    /// contains any of these strings, the QRL body is replaced with _noopQrl.
    /// Example: ["server"] strips server$(), serverLoader$(), serverStuff$()
    pub strip_ctx_name: Vec<String>,
    // ...
}
```

### Detection

During `enter_call_expression`, after identifying a `$`-call site, check if its context name matches the strip list:

```rust
/// Check if a $-call should be stripped based on its context name.
///
/// Rules:
/// - The context name is the $-suffixed function name without the '$'
///   e.g., "server$" -> ctx_name = "server"
///        "serverStuff$" -> ctx_name = "serverStuff"
///        "serverLoader$" -> ctx_name = "serverLoader"
/// - If ctx_name contains ANY of the strip_ctx_name entries, it is stripped
///   e.g., strip_ctx_name = ["server"] matches "server", "serverStuff", "serverLoader"
fn should_strip_call(
    callee_name: &str,
    strip_ctx_names: &[String],
) -> bool {
    if !callee_name.ends_with('$') {
        return false;
    }
    let ctx_name = &callee_name[..callee_name.len() - 1]; // Remove '$'
    strip_ctx_names.iter().any(|strip_name| {
        ctx_name.to_lowercase().contains(&strip_name.to_lowercase())
    })
}
```

### Output

The stripped call's QRL body is replaced, but the Qrl-suffixed wrapper is preserved:

```javascript
// Prod mode:
serverStuffQrl(/*#__PURE__*/ _noopQrl("s_r1qAHX7Opp0"));
serverLoaderQrl(/*#__PURE__*/ _noopQrl("s_k1L0DiPQV1I"));

// Dev mode:
serverQrl(/*#__PURE__*/ _noopQrlDEV("api_server_JonPp043gH0", {
    file: "/user/qwik/src/test.tsx",
    lo: 0,
    hi: 0,
    displayName: "test.tsx_api_server"
}));
```

**Evidence:** `example_strip_server_code.md` (Prod mode, lines 201-202), `example_drop_side_effects.md` (Dev mode, lines 73-78)

---

## Section 11: Noop QRL Construction

### Prod Mode: _noopQrl(hash_string)

```rust
/// Build _noopQrl("s_HASH") for Prod mode stripping.
///
/// Input:  serverStuff$(() => { ... })
/// Output: serverStuffQrl(_noopQrl("s_r1qAHX7Opp0"))
pub fn build_noop_qrl<'a>(
    hash: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let callee = ctx.ast.expression_identifier_reference(
        SPAN, ctx.ast.atom("_noopQrl"),
    );
    let hash_lit = ctx.ast.expression_string_literal(
        SPAN, ctx.ast.atom(hash), None,
    );
    let mut args = ctx.ast.vec_with_capacity(1);
    args.push(Argument::from(hash_lit));
    ctx.ast.expression_call(SPAN, callee, NONE, args, false)
}
```

### Dev Mode: _noopQrlDEV(name_string, debug_object)

```rust
/// Build _noopQrlDEV("name_HASH", { file, lo, hi, displayName }) for Dev mode stripping.
///
/// See Section 9.2 for the complete build_noop_qrl_dev() implementation.
```

### Complete build_noop_qrl with dev/prod branching

```rust
/// Build the noop QRL replacement, branching on dev vs prod mode.
pub fn build_noop_qrl_replacement<'a>(
    is_dev: bool,
    hash: &str,
    display_name: &str,
    debug_info: Option<&DebugInfo>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    if is_dev {
        let info = debug_info.expect("Dev mode requires debug info");
        build_noop_qrl_dev(display_name, info, ctx)
    } else {
        build_noop_qrl(hash, ctx)
    }
}
```

### Import Requirements

- Prod: `import { _noopQrl } from "@qwik.dev/core"`
- Dev: `import { _noopQrlDEV } from "@qwik.dev/core"`

**Evidence:** `example_strip_server_code.md` (Prod, component entry):
```javascript
import { _noopQrl } from "@qwik.dev/core";
serverStuffQrl(/*#__PURE__*/ _noopQrl("s_r1qAHX7Opp0"));
serverLoaderQrl(/*#__PURE__*/ _noopQrl("s_k1L0DiPQV1I"));
```

`example_drop_side_effects.md` (Dev, main module):
```javascript
import { _noopQrlDEV } from "@qwik.dev/core";
export const api = serverQrl(/*#__PURE__*/ _noopQrlDEV("api_server_JonPp043gH0", { ... }));
```

---

## Section 12: Nested $() Preservation

When a `$`-call is stripped, `$()` and `client$()` calls **inside** the stripped callback body are NOT stripped. They produce their own segments normally.

### Rule

Only the outer stripped call gets the noop replacement. Inner `$()` calls are extracted as if the outer code were not stripped.

### Implementation

During the stripping pass, when entering a stripped `$`-call body, do NOT suppress inner `$()` processing:

```rust
/// Stripping state tracked during traversal.
struct StripState {
    /// Whether we are currently inside a stripped $-call body.
    /// This flag does NOT suppress inner $() detection.
    inside_stripped_body: bool,
}

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        if let Some(kind) = self.is_dollar_call(call) {
            if self.should_strip(&kind) {
                // Mark the OUTER call for noop replacement
                self.mark_for_stripping(call);
                self.strip_state.inside_stripped_body = true;
                // DO NOT return -- continue processing to detect inner $() calls
            } else {
                // Process normally (extract segment, build QRL)
                self.record_segment(call, &kind, ctx);
            }
        }
    }
}
```

### Evidence

`example_strip_server_code.md` shows this clearly:

**Input:**
```tsx
serverStuff$(async () => {
    const a = $(() => {
        // from $(), should not be removed
    });
    const b = client$(() => {
        // from client$(), should not be removed
    });
    return [a, b];
})
```

**Output:** The `serverStuff$` call becomes `serverStuffQrl(_noopQrl(...))`, but two segments are STILL extracted:
```javascript
// Segment: s_2ca3HLDC7yc (from nested $())
export const s_2ca3HLDC7yc = ()=>{ /* from $(), should not be removed */ };

// Segment: s_v9qawr2Inkk (from nested client$())
export const s_v9qawr2Inkk = ()=>{ /* from client$(), should not be removed */ };
```

Both nested `$()` and `client$()` produce entry point segments. The parent `serverStuff$` body is stripped, but the children are preserved.

---

## Section 13: Side Effect Analysis for Dead Declaration Removal

After stripping server-only code, the optimizer performs dead code elimination on associated declarations. Imports and module-level expressions used ONLY by stripped code are removed.

### Algorithm Overview

1. **Identify stripped segment bodies** -- collect all variable/import bindings referenced exclusively within stripped callback bodies
2. **For each declaration in the module**, check if ALL references are inside stripped bodies
3. **If yes**, remove the declaration
4. **For imports**, remove the import specifier (and the entire import statement if no specifiers remain)
5. **PRESERVE side-effect expressions** -- IIFEs, standalone function calls, arrow IIFEs

### Reference Analysis with oxc_semantic

```rust
use oxc_semantic::{Scoping, ScopeId, SymbolId};

/// Remove declarations that have zero live references after stripping.
///
/// "Live" means a reference that is NOT inside a stripped callback body.
/// A declaration with zero live references is dead code and can be removed.
///
/// Arguments:
/// - scoping: The Scoping from SemanticBuilder (post-stripping analysis)
/// - stripped_scope_ids: Set of ScopeIds for stripped $-call bodies
///
/// Returns: Set of SymbolIds to remove (dead declarations)
pub fn find_dead_declarations(
    scoping: &Scoping,
    stripped_scope_ids: &HashSet<ScopeId>,
) -> HashSet<SymbolId> {
    let mut dead_symbols = HashSet::new();

    for symbol_id in scoping.symbol_ids() {
        let symbol_scope = scoping.symbol_scope_id(symbol_id);

        // Skip symbols declared inside stripped bodies
        // (they're already gone with the stripped code)
        if is_in_stripped_scope(scoping, symbol_scope, stripped_scope_ids) {
            continue;
        }

        // Check if ALL references to this symbol are in stripped scopes
        let all_refs_stripped = scoping
            .get_resolved_references(symbol_id)
            .all(|ref_id| {
                let ref_scope = get_reference_scope(scoping, ref_id);
                is_in_stripped_scope(scoping, ref_scope, stripped_scope_ids)
            });

        // Also check: symbol must have at least one reference
        // (unreferenced symbols may be intentional exports/side-effects)
        let has_any_refs = scoping
            .get_resolved_references(symbol_id)
            .next()
            .is_some();

        if has_any_refs && all_refs_stripped {
            dead_symbols.insert(symbol_id);
        }
    }

    dead_symbols
}

/// Check if a scope is inside any stripped scope.
fn is_in_stripped_scope(
    scoping: &Scoping,
    scope_id: ScopeId,
    stripped_scope_ids: &HashSet<ScopeId>,
) -> bool {
    scoping.scope_ancestors(scope_id)
        .any(|ancestor| stripped_scope_ids.contains(&ancestor))
}
```

### Preserved Patterns (NEVER Removed)

These expression types have observable side effects and must be preserved even when declarations they reference are stripped:

| Pattern | Example | Reason |
|---------|---------|--------|
| IIFE (function) | `(function() { console.log('run'); })()` | Observable side effect |
| IIFE (arrow) | `(() => { console.log('run'); })()` | Observable side effect |
| Standalone call | `sideEffect()` | Observable function call |
| Import with live refs | `import { sideEffect } from './secret'` | Still called elsewhere |

### Declaration Removal via AST Mutation

```rust
/// Remove a dead variable declaration from the program body.
///
/// In exit_program, filter out dead declarations:
fn remove_dead_declarations<'a>(
    program: &mut Program<'a>,
    dead_symbols: &HashSet<SymbolId>,
    scoping: &Scoping,
) {
    program.body.retain(|stmt| {
        match stmt {
            // Remove variable declarations where ALL declarators are dead
            Statement::VariableDeclaration(decl) => {
                !decl.declarations.iter().all(|declarator| {
                    if let Some(binding_ident) = declarator.id.get_binding_identifier() {
                        if let Some(symbol_id) = scoping.find_binding(
                            ScopeId::new(0), // module scope
                            &binding_ident.name,
                        ) {
                            return dead_symbols.contains(&symbol_id);
                        }
                    }
                    false
                })
            }
            // Remove import declarations where ALL specifiers are dead
            Statement::ImportDeclaration(import_decl) => {
                if let Some(specifiers) = &import_decl.specifiers {
                    let all_dead = specifiers.iter().all(|spec| {
                        let local_name = match spec {
                            ImportDeclarationSpecifier::ImportSpecifier(s) =>
                                &s.local.name,
                            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) =>
                                &s.local.name,
                            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) =>
                                &s.local.name,
                        };
                        if let Some(symbol_id) = scoping.find_binding(
                            ScopeId::new(0),
                            local_name,
                        ) {
                            dead_symbols.contains(&symbol_id)
                        } else {
                            false
                        }
                    });
                    !all_dead // Keep if NOT all dead
                } else {
                    true // Side-effect import (no specifiers), keep
                }
            }
            // Keep expression statements -- they may be side effects
            // The is_preserved_side_effect check happens here
            Statement::ExpressionStatement(expr_stmt) => {
                is_preserved_side_effect(&expr_stmt.expression)
                    || !is_dead_expression(&expr_stmt.expression, dead_symbols, scoping)
            }
            // Keep everything else
            _ => true,
        }
    });
}

/// Check if an expression is a preserved side-effect pattern.
fn is_preserved_side_effect(expr: &Expression<'_>) -> bool {
    match expr {
        // IIFE: (function() { ... })()
        Expression::CallExpression(call) => {
            matches!(&call.callee,
                Expression::ParenthesizedExpression(paren) if matches!(
                    &paren.expression,
                    Expression::FunctionExpression(_) | Expression::ArrowFunctionExpression(_)
                )
            )
            // Also: standalone function calls like sideEffect()
            || matches!(&call.callee, Expression::Identifier(_))
        }
        _ => false,
    }
}
```

**Evidence:** `example_drop_side_effects.md` demonstrates all cases:

**Removed (dead code):**
- `import { clientSupabase } from 'supabase'` -- only used by stripped `server$` callback
- `import { Client } from 'openai'` -- only used by stripped `server$` callback
- `import { secret } from './secret'` -- only used by stripped `server$` callback
- `const supabase = clientSupabase()` -- only feeds stripped code
- `const dfd = new Client(secret)` -- only feeds stripped code

**Preserved:**
- `(function() { console.log('run'); })()` -- IIFE side effect
- `(() => { console.log('run'); })()` -- arrow IIFE side effect
- `sideEffect()` -- standalone function call
- `import { sideEffect } from './secret'` -- still has live references

---

## Section 14: Export Stripping

When entire exported functions or classes are server-only, their body is replaced with a `throw` statement instead of being completely removed:

```javascript
export function handler() {
    throw "Symbol removed by Qwik Optimizer...";
}
```

### OXC Construction

```rust
/// Build a throw statement to replace a stripped export body.
///
/// export function handler() { throw "Symbol removed by Qwik Optimizer..."; }
pub fn build_stripped_export_body<'a>(
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let message = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom("Symbol removed by Qwik Optimizer..."),
        None,
    );
    ctx.ast.statement_throw(SPAN, message)
}
```

This pattern applies when the export itself is referenced elsewhere (e.g., another module imports `handler`). The export must exist but cannot contain server-only logic.

**Evidence:** `example_strip_server_code.md` describes this pattern in the conventions section (CONV-09).

---

## Section 15: Const Replacement / isServer (CONV-10)

The optimizer evaluates `isServer` at compile time, replacing it with a boolean literal. This enables dead branch elimination for server-only code paths.

### Detection

During `enter_identifier_reference`, check if the identifier is `isServer` imported from `@qwik.dev/core`:

```rust
impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_identifier_reference(
        &mut self,
        ident: &mut IdentifierReference<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        if ident.name.as_str() == "isServer" {
            // Verify this is the @qwik.dev/core import, not a local variable
            if self.is_qwik_import("isServer") {
                // Replace with boolean literal based on build target
                // is_server_value comes from TransformModulesOptions
            }
        }
    }
}
```

### Replacement

```rust
/// Replace isServer identifier with a boolean literal.
///
/// Input:  if (!isServer) return;
/// Output: if (!true) return;   (server build)
/// Output: if (!false) return;  (client build)
fn replace_is_server<'a>(
    is_server_value: bool,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    ctx.ast.expression_boolean_literal(SPAN, is_server_value)
}
```

### Dead Branch Elimination

After `isServer` replacement, the optimizer simplifies known-value conditionals:

```rust
/// Simplify if-statements with known boolean conditions.
///
/// Rules:
///   if (false) { ... }           -> remove entire if statement
///   if (true) { ... }            -> keep only the consequent block
///   if (true) { ... } else { ... } -> keep only the consequent
///   if (false) { ... } else { ... } -> keep only the alternate
///   !false -> true
///   !true  -> false
pub fn simplify_dead_branches<'a>(
    program: &mut Program<'a>,
    ctx: &mut TraverseCtx<'a>,
) {
    // This runs as a post-pass after isServer replacement.
    // For each IfStatement in the program:
    //   1. Check if test is BooleanLiteral
    //   2. If true: replace IfStatement with consequent block statements
    //   3. If false: remove IfStatement (or keep alternate if present)
}

/// Detect and simplify boolean negation: !true -> false, !false -> true
fn simplify_unary_not<'a>(
    expr: &mut Expression<'a>,
    ctx: &mut TraverseCtx<'a>,
) {
    if let Expression::UnaryExpression(unary) = expr {
        if unary.operator == UnaryOperator::LogicalNot {
            if let Expression::BooleanLiteral(bool_lit) = &unary.argument {
                *expr = ctx.ast.expression_boolean_literal(SPAN, !bool_lit.value);
            }
        }
    }
}
```

### OXC API for Dead Branch Removal

```rust
/// Remove or replace an if-statement based on its known boolean test.
///
/// Match on IfStatement.test being a BooleanLiteral:
///   - true:  replace IfStatement with consequent.body statements
///   - false: remove IfStatement entirely (or use alternate if present)
fn handle_dead_branch<'a>(
    if_stmt: &IfStatement<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Option<Vec<Statement<'a>>> {
    if let Expression::BooleanLiteral(bool_lit) = &if_stmt.test {
        if bool_lit.value {
            // if (true) { BODY } -> keep BODY
            Some(if_stmt.consequent.body.clone_in(ctx.ast))
        } else {
            // if (false) { ... } -> remove (or keep alternate)
            if let Some(alternate) = &if_stmt.alternate {
                match alternate {
                    Statement::BlockStatement(block) =>
                        Some(block.body.clone_in(ctx.ast)),
                    other => Some(vec![other.clone_in(ctx.ast)]),
                }
            } else {
                Some(vec![]) // Remove entirely
            }
        }
    } else {
        None // Test is not a boolean literal, no simplification
    }
}
```

**Evidence:** `example_strip_server_code.md` (Prod mode, server build):

**Input:**
```tsx
useTask$(async () => {
    if (!isServer) return;
    state.text = await mongo.users();
    redis.set(state.text);
});
```

**Output (the guard is eliminated):**
```javascript
export const s_gDH1EtUWqBU = async ()=>{
    const state = _captures[0];
    state.text = await mongo.users();
    redis.set(state.text);
};
```

The `if (!isServer) return;` guard is completely removed because:
1. `isServer` evaluates to `true` (server build)
2. `!true` simplifies to `false`
3. `if (false) return;` is dead code, removed

---

## Section 16: Static Expression Evaluation

The optimizer includes a mini constant folder that evaluates compile-time-known expressions. This is used for prop value simplification and const folding contexts.

### Evaluable Expression Types

| Expression | Example Input | Evaluated Output |
|-----------|---------------|-----------------|
| String concatenation | `'true' + 1` | `"true1"` |
| Ternary with known condition | `"true1" ? 'true' : ''` | `'true'` (truthy string) |
| Template literal | `` `text${12}` `` | `"text12"` |
| typeof | `typeof "text12"` | `"string"` |
| Boolean operations | `!false` | `true` |
| Numeric arithmetic | `1 + 2` | `3` |

### Rust Implementation: try_eval_const_expr()

```rust
/// Result of attempting to evaluate a constant expression.
#[derive(Debug, Clone)]
pub enum ConstValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Undefined,
}

impl ConstValue {
    /// JavaScript truthiness check.
    pub fn is_truthy(&self) -> bool {
        match self {
            ConstValue::String(s) => !s.is_empty(),
            ConstValue::Number(n) => *n != 0.0 && !n.is_nan(),
            ConstValue::Boolean(b) => *b,
            ConstValue::Null | ConstValue::Undefined => false,
        }
    }
}

/// Attempt to evaluate a constant expression at compile time.
///
/// Returns Some(ConstValue) if the expression can be fully evaluated,
/// None if it contains runtime-only values (function calls, identifiers, etc.)
///
/// Scope: Only handles literals and simple operations.
/// Does NOT evaluate function calls or complex expressions.
pub fn try_eval_const_expr(expr: &Expression<'_>) -> Option<ConstValue> {
    match expr {
        // Literal values
        Expression::StringLiteral(s) => Some(ConstValue::String(s.value.to_string())),
        Expression::NumericLiteral(n) => Some(ConstValue::Number(n.value)),
        Expression::BooleanLiteral(b) => Some(ConstValue::Boolean(b.value)),
        Expression::NullLiteral(_) => Some(ConstValue::Null),

        // Template literal: `text${12}` -> "text12"
        Expression::TemplateLiteral(tpl) => {
            let mut result = String::new();
            for (i, quasi) in tpl.quasis.iter().enumerate() {
                result.push_str(quasi.value.raw.as_str());
                if i < tpl.expressions.len() {
                    let expr_val = try_eval_const_expr(&tpl.expressions[i])?;
                    match expr_val {
                        ConstValue::String(s) => result.push_str(&s),
                        ConstValue::Number(n) => result.push_str(&n.to_string()),
                        ConstValue::Boolean(b) => result.push_str(&b.to_string()),
                        ConstValue::Null => result.push_str("null"),
                        ConstValue::Undefined => result.push_str("undefined"),
                    }
                }
            }
            Some(ConstValue::String(result))
        }

        // Binary expressions: string concatenation, arithmetic
        Expression::BinaryExpression(bin) => {
            let left = try_eval_const_expr(&bin.left)?;
            let right = try_eval_const_expr(&bin.right)?;
            match bin.operator {
                BinaryOperator::Addition => {
                    // String concatenation takes precedence
                    match (&left, &right) {
                        (ConstValue::String(l), _) => {
                            let r_str = match &right {
                                ConstValue::String(s) => s.clone(),
                                ConstValue::Number(n) => n.to_string(),
                                ConstValue::Boolean(b) => b.to_string(),
                                ConstValue::Null => "null".to_string(),
                                ConstValue::Undefined => "undefined".to_string(),
                            };
                            Some(ConstValue::String(format!("{}{}", l, r_str)))
                        }
                        (_, ConstValue::String(r)) => {
                            let l_str = match &left {
                                ConstValue::Number(n) => n.to_string(),
                                ConstValue::Boolean(b) => b.to_string(),
                                ConstValue::Null => "null".to_string(),
                                ConstValue::Undefined => "undefined".to_string(),
                                _ => return None,
                            };
                            Some(ConstValue::String(format!("{}{}", l_str, r)))
                        }
                        (ConstValue::Number(l), ConstValue::Number(r)) => {
                            Some(ConstValue::Number(l + r))
                        }
                        _ => None,
                    }
                }
                BinaryOperator::StrictEquality => {
                    match (&left, &right) {
                        (ConstValue::String(l), ConstValue::String(r)) =>
                            Some(ConstValue::Boolean(l == r)),
                        (ConstValue::Number(l), ConstValue::Number(r)) =>
                            Some(ConstValue::Boolean(l == r)),
                        (ConstValue::Boolean(l), ConstValue::Boolean(r)) =>
                            Some(ConstValue::Boolean(l == r)),
                        _ => None,
                    }
                }
                _ => None, // Other operators not handled
            }
        }

        // Unary expressions: !value, typeof
        Expression::UnaryExpression(unary) => {
            match unary.operator {
                UnaryOperator::LogicalNot => {
                    let val = try_eval_const_expr(&unary.argument)?;
                    Some(ConstValue::Boolean(!val.is_truthy()))
                }
                UnaryOperator::Typeof => {
                    let val = try_eval_const_expr(&unary.argument)?;
                    let type_str = match val {
                        ConstValue::String(_) => "string",
                        ConstValue::Number(_) => "number",
                        ConstValue::Boolean(_) => "boolean",
                        ConstValue::Null => "object", // typeof null === "object"
                        ConstValue::Undefined => "undefined",
                    };
                    Some(ConstValue::String(type_str.to_string()))
                }
                _ => None,
            }
        }

        // Conditional (ternary): cond ? consequent : alternate
        Expression::ConditionalExpression(cond) => {
            let test_val = try_eval_const_expr(&cond.test)?;
            if test_val.is_truthy() {
                try_eval_const_expr(&cond.consequent)
            } else {
                try_eval_const_expr(&cond.alternate)
            }
        }

        // Parenthesized expressions: (expr)
        Expression::ParenthesizedExpression(paren) => {
            try_eval_const_expr(&paren.expression)
        }

        _ => None, // Cannot evaluate
    }
}
```

### Example: Chained Evaluation

**Input expression:** `typeof \`text${12}\` === 'string' ? 12 : 43`

**Evaluation chain:**
1. `` `text${12}` `` -> `ConstValue::String("text12")`
2. `typeof "text12"` -> `ConstValue::String("string")`
3. `"string" === 'string'` -> `ConstValue::Boolean(true)`
4. `true ? 12 : 43` -> `ConstValue::Number(12)`

**Result:** The expression is replaced with literal `12`.

**Evidence:** `example_derived_signals_cmp.md` shows this evaluation in the constProps:
```javascript
staticExpr2: typeof `text${12}` === 'string' ? 12 : 43,
```
This evaluates to `12` at the prop classification stage (the expression is const-evaluable so it goes into constProps).

---

## Section 17: CONV Dependency Note

CONV-10 (const replacement / isServer) should run **BEFORE** CONV-09 (code stripping) because `isServer` evaluation can create additional dead code paths that stripping can then eliminate.

### Ordering Rationale

```
CONV-10 (isServer -> true/false)
    |
    v
Dead branch elimination (if (false) { ... } -> removed)
    |
    v
CONV-09 (strip server$ calls -> _noopQrl)
    |
    v
Side effect analysis (remove dead imports/declarations)
```

If stripping runs first, the `if (!isServer) return;` guard inside a non-stripped `useTask$` would not be eliminated. By running const replacement first:

1. `isServer` becomes `true` (server build)
2. `!true` becomes `false`
3. `if (false) return;` is removed as dead code
4. The `useTask$` body now only contains the server-side logic
5. If stripping then also applies (because the useTask itself is NOT stripped -- only `server$` calls are), the clean body is preserved

### Implementation

```rust
pub fn transform_module(program: &mut Program, options: &TransformOptions) {
    // Phase 1: Const replacement (CONV-10)
    replace_const_values(program, options);
    simplify_dead_branches(program);

    // Phase 2: $() detection and stripping (CONV-09)
    detect_dollar_calls(program);
    strip_matching_calls(program, &options.strip_ctx_name);

    // Phase 3: Side effect analysis (post-stripping)
    remove_dead_declarations(program);
}
```

**Evidence:** `example_strip_server_code.md` confirms this ordering -- the `isServer` guard is gone from the first `useTask$` segment, while the `serverStuff$` and `serverLoader$` calls are separately stripped to noops.

---

## Section 18: Import Management

These patterns require specific imports from `@qwik.dev/core`. All imports follow the same one-specifier-per-declaration pattern documented in Phase 4 API-MAPPING.md Pattern 3.

### Complete Import Requirements by Feature

| Feature | Imports Required | Source Module |
|---------|-----------------|---------------|
| Code stripping (Prod) | `_noopQrl` | `@qwik.dev/core` |
| Code stripping (Dev) | `_noopQrlDEV` | `@qwik.dev/core` |
| Sync$ | `_qrlSync` | `@qwik.dev/core` |
| Input bind:value | `_val`, `inlinedQrl` | `@qwik.dev/core` |
| Input bind:checked | `_chk`, `inlinedQrl` | `@qwik.dev/core` |
| Dev mode QRL | `qrlDEV` | `@qwik.dev/core` |
| Dev mode Inline QRL | `inlinedQrlDEV` | `@qwik.dev/core` |
| Entry Strategy: Segment/Smart/etc. | `qrl` | `@qwik.dev/core` |
| Entry Strategy: Inline/Hoist | `inlinedQrl` | `@qwik.dev/core` |

### Import Construction Pattern

All imports use the same `build_named_import()` function from Phase 4 API-MAPPING.md:

```rust
// Each import is a separate declaration (matches spec output):
// import { _noopQrl } from "@qwik.dev/core";
// import { _qrlSync } from "@qwik.dev/core";
// import { _val } from "@qwik.dev/core";
// import { _chk } from "@qwik.dev/core";
// import { qrlDEV } from "@qwik.dev/core";

for import_name in &needed_imports {
    let stmt = build_named_import(import_name, "@qwik.dev/core", ctx);
    new_stmts.push(stmt);
}
```

### Import Tracker Extension

The `ImportTracker` from Phase 4 is extended to track these additional imports:

```rust
struct ImportTracker {
    // ... existing fields from Phase 4 ...

    /// Whether the module needs _noopQrl or _noopQrlDEV (stripping)
    needs_noop_qrl: bool,

    /// Whether the module needs _qrlSync (sync$ present)
    needs_qrl_sync: bool,

    /// Whether the module needs _val (bind:value present)
    needs_val: bool,

    /// Whether the module needs _chk (bind:checked present)
    needs_chk: bool,

    /// Whether to use dev variants (qrlDEV, _noopQrlDEV, inlinedQrlDEV)
    is_dev_mode: bool,
}
```

**Evidence:** All spec file outputs show these imports following the one-per-declaration pattern:
- `example_strip_server_code.md`: `import { _noopQrl } from "@qwik.dev/core"`
- `example_drop_side_effects.md`: `import { _noopQrlDEV } from "@qwik.dev/core"`, `import { qrlDEV } from "@qwik.dev/core"`
- `example_input_bind.md`: `import { _val } from "@qwik.dev/core"`, `import { _chk } from "@qwik.dev/core"`
- `example_of_synchronous_qrl.md`: `import { _qrlSync } from "@qwik.dev/core"`

---

*This document satisfies APIM-10: Entry strategy, code stripping, const folding, input binding, sync$ serialization, and dev mode patterns mapped to OXC AstBuilder API call sequences. It covers CONV-09 (Code Stripping), CONV-10 (Const Replacement), CONV-12 (Input Binding), and CONV-13 (Sync$ Serialization).*
