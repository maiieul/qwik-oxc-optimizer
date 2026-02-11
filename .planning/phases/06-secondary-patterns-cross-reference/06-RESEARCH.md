# Phase 6: Secondary Patterns & Cross-Reference -- Research

**Date:** 2026-02-10
**Phase:** 06-secondary-patterns-cross-reference
**Goal:** Map remaining transformation patterns (JSX, props destructuring, signals, entry strategy, code stripping, const folding) to OXC APIs and produce the complete 14 CONV type cross-reference table

**Requirements:**
- APIM-04: JSX transformation patterns mapped to OXC expression replacement APIs
- APIM-07: Source map generation mapped to oxc_codegen + oxc_sourcemap APIs
- APIM-08: All 14 CONV types cross-referenced to specific OXC API patterns
- APIM-09: Props destructuring and signal optimization patterns mapped to OXC APIs
- APIM-10: Entry strategy, code stripping, and const folding patterns mapped to OXC APIs

---

## 1. JSX Transformation Patterns (APIM-04)

### 1.1 Overview

The Qwik optimizer replaces JSX syntax with function calls. There are two primary output functions:

- **`_jsxSorted(tag, varProps, constProps, children, flags, key)`** -- standard JSX replacement
- **`_jsxSplit(tag, varProps, constProps, children, flags, key)`** -- used when spread props are present

Additionally, the `<></>` fragment syntax produces a `_Fragment` import from `@qwik.dev/core/jsx-runtime`.

**Evidence:** `example_jsx.md` (output lines 1150-1169), `example_derived_signals_cmp.md` (output lines 62-106), `example_getter_generation.md` (output lines 59-97), `should_destructure_args.md` (output lines 66-99)

### 1.2 _jsxSorted Signature

```
_jsxSorted(tag, varProps, constProps, children, flags, key)
_jsxSorted(tag, varProps, constProps, children, flags, key, devInfo)  // Dev mode only
```

**Arguments:**
1. **tag**: String literal for native elements (`"div"`, `"button"`, `"input"`, `"span"`, `"p"`) or identifier reference for components (`Cmp`, `Lightweight`, `_Fragment`)
2. **varProps**: Object expression with mutable/variable props, or `null` if none
3. **constProps**: Object expression with constant/trackable props, or `null` if none
4. **children**: Single child expression, array of children, or `null` if no children
5. **flags**: Numeric literal (observed values: `0`, `1`, `2`, `3`)
6. **key**: String literal key (e.g., `"u6_0"`, `"u6_1"`) or `null`
7. **devInfo** (Dev mode only): Object with `{fileName, lineNumber, columnNumber}`

### 1.3 Prop Classification Rules

The optimizer classifies each prop value into "variable" (varProps, arg 2) or "constant" (constProps, arg 3). The rules differ for **component** vs **native element** targets.

**For components (e.g., `<Cmp prop={value}/>`):**

| Prop Value Type | Classification | Example |
|-----------------|---------------|---------|
| Static literals | const | `staticText="text"`, `staticNumber={1}`, `staticBoolean={true}` |
| Template literals (no dynamic parts) | const | `staticText2={\`text\`}` |
| Static expressions (compile-time evaluable) | const | `staticExpr={\`text${12}\`}` |
| Signal identifiers | const | `signal={signal}` |
| Signal `.value` access | const (wrapped) | `signalValue={signal.value}` -> `_wrapProp(signal)` |
| Computed signal expressions | const (fnSignal) | `signalComputedValue={12 + signal.value}` -> `_fnSignal(_hf0, [signal], _hf0_str)` |
| Store property access | const (fnSignal) | `store={store.address.city.name}` -> `_fnSignal(_hf1, [store], _hf1_str)` |
| Store computed expressions | const (fnSignal) | `storeComputed={...}` -> `_fnSignal(_hf2, [store], _hf2_str)` |
| Imported dep identifiers | const | `dep={dep}` |
| Imported dep property access | const | `depAccess={dep.thing}` |
| Imported dep computed | const | `depComputed={dep.thing + 'stuff'}` |
| Global identifiers | var | `global={globalThing}` |
| Global property access | var | `globalAccess={globalThing.thing}` |
| Global computed | var | `globalComputed={globalThing.thing + 'stuff'}` |
| Function calls | var | `noInline={signal.value()}` |
| Mixed (signal + unknown call) | var | `noInline2={signal.value + unknown()}` |
| Mutable wrappers | var | `noInline3={mutable(signal)}` |
| Mixed (signal + dep) | var | `noInline4={signal.value + dep}` |

**Evidence:** `example_derived_signals_cmp.md` -- comprehensive prop classification example showing all categories.

**For native elements (e.g., `<div class="foo">`)**:

| Prop Value Type | Classification | Example |
|-----------------|---------------|---------|
| Static attributes | const | `class="renders"` |
| Event handlers (QRL) | const | `"q-e:click": qrl(...)` |
| Signal-wrapped values | var | `id: _wrapProp(_rawProps, "id")` |

**Evidence:** `should_destructure_args.md` -- `_jsxSorted("div", {id: _wrapProp(...)}, null, [...])` shows signal-wrapped prop in varProps for native element.

### 1.4 _jsxSplit Pattern

`_jsxSplit` is used instead of `_jsxSorted` when a **spread operator** (`{...props}`) is present on the element.

```javascript
// Input:  <button {...props}/>
// Output: _jsxSplit("button", { ..._getVarProps(props) }, _getConstProps(props), null, 0, null)
```

The spread is decomposed into:
- `_getVarProps(props)` -- extracts mutable props (goes into varProps object via spread)
- `_getConstProps(props)` -- extracts constant props (goes into constProps)

**Evidence:** `example_jsx.md` (output line 1162-1164), `should_destructure_args.md` (output line 88-90)

### 1.5 Fragment Handling

JSX fragments (`<>...</>`) are transformed to `_jsxSorted(_Fragment, null, null, children, flags, key)`.

The `_Fragment` identifier is imported as:
```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
```

Note the different import source: `@qwik.dev/core/jsx-runtime` (not `@qwik.dev/core`).

**Evidence:** `example_getter_generation.md` (output line 79-86), `example_jsx.md` (output line 1158, 1160)

### 1.6 Children Encoding

- **No children**: `null` (4th argument)
- **Single child**: Direct expression (e.g., `_wrapProp(state, "text")`, `rerenders`, `value`)
- **Multiple children**: Array expression `[child1, child2, ...]`
- **Text content**: String literal (e.g., `"Value "`, `" "`)

**Evidence:** Across all JSX spec files.

### 1.7 Event Handler Transformation

JSX event attributes like `onClick$={handler}` are transformed to QRL attribute props:

```javascript
// Input:  <div onClick$={() => console.log('parent')}>
// Output: _jsxSorted("div", null, {"q-e:click": qrl(i_HASH, "s_HASH")}, ...)

// Input:  <button onClick$={sync$(handler)}>
// Output: _jsxSorted("button", {"q-e:click": _qrlSync(handler, stringified)}, ...)
```

The `onClick$` attribute name becomes `"q-e:click"` in the output. The `$` suffix is removed and a `q-e:` prefix is added. Sync handlers go into varProps; async QRL handlers go into constProps.

**Evidence:** `example_strip_server_code.md` (output line 204-206), `example_of_synchronous_qrl.md` (output lines 791-804)

### 1.8 Dev Mode JSX

In Dev mode, `_jsxSorted` receives a 7th argument with source location:

```javascript
_jsxSorted("button", null, {
    "q-e:click": qrlDEV(...)
}, null, 3, "u6_0", {
    fileName: "test.tsx",
    lineNumber: 27,
    columnNumber: 3
});
```

**Evidence:** `example_drop_side_effects.md` (output lines 134-145)

### 1.9 OXC API Mapping for JSX

The JSX transform requires these OXC operations:

1. **Detection**: In `enter_jsx_element` / `enter_jsx_fragment`, capture the JSX node
2. **Tag resolution**:
   - Native: `ast.expression_string_literal("div")`
   - Component: keep existing identifier reference
   - Fragment: create `_Fragment` identifier + add import
3. **Prop splitting**: Traverse JSX attributes, classify each into var/const
4. **Expression construction**: `ast.expression_call(callee, args)` to build `_jsxSorted(...)` or `_jsxSplit(...)`
5. **Replacement**: Replace the JSXElement/JSXFragment with the constructed CallExpression

**Key AstBuilder calls:**
- `ast.expression_call(callee_expr, arguments_vec)` -- build `_jsxSorted(tag, varProps, constProps, children, flags, key)`
- `ast.expression_object(properties_vec)` -- build var/const prop objects
- `ast.expression_array(elements_vec)` -- build children array
- `ast.expression_numeric_literal(flags)` -- build flags argument
- `ast.expression_string_literal(key)` -- build key argument
- `ast.expression_null_literal()` -- for null arguments

### 1.10 Flags Analysis

The `flags` argument appears to encode child type information:

| flags | Observed in | Context |
|-------|-------------|---------|
| `0` | `_jsxSplit` calls | Elements with spread props |
| `1` | `_jsxSorted` with array children or Fragment | Multiple children |
| `2` | sync$ event handlers | Event-only elements |
| `3` | Most native elements | Self-closing or single-child elements |

---

## 2. Signal Optimization Patterns (APIM-09 partial)

### 2.1 Overview

Signal optimization converts reactive property accesses into trackable wrappers so the Qwik runtime can efficiently update only the affected DOM nodes. Three helper functions are involved:

- **`_wrapProp(source, propName?)`** -- wraps direct signal/store property access
- **`_fnSignal(_hfN, [deps], _hfN_str)`** -- wraps computed signal expressions
- **Hoisted functions `_hfN` / `_hfN_str`** -- extracted getter functions + their string representations

### 2.2 _wrapProp Patterns

Two forms exist:

```javascript
// Form 1: Direct signal wrapping (no property name)
_wrapProp(signal)
// Used when: signalValue={signal.value} on a component

// Form 2: Named property wrapping
_wrapProp(store, "count")
_wrapProp(props, "count")
_wrapProp(_rawProps, "id")
_wrapProp(value)  // signal used directly (input binding)
// Used when: accessing a named property reactively
```

**Evidence:**
- Form 1: `example_derived_signals_cmp.md` line 96 (`_wrapProp(signal)`)
- Form 2: `example_getter_generation.md` line 142 (`_wrapProp(store, "count")`), line 88 (`_wrapProp(props, "count")`), `should_destructure_args.md` line 86 (`_wrapProp(_rawProps, "id")`)

### 2.3 _fnSignal Pattern

For computed expressions involving reactive values, the optimizer hoists a getter function and passes its string representation:

```javascript
// Input:  signalComputedValue={12 + signal.value}
// Output:
const _hf0 = (p0) => 12 + p0.value;
const _hf0_str = "12+p0.value";
// ... in JSX call:
signalComputedValue: _fnSignal(_hf0, [signal], _hf0_str)
```

**Key rules:**
1. The getter function replaces the original reactive source with parameter `p0` (or `p0`, `p1` for multiple sources)
2. The string representation is a minified version of the getter body
3. The deps array `[signal]` / `[store]` contains the original reactive sources
4. Hoisted functions are placed at module top-level, BEFORE the segment export
5. Each segment has its own `_hf0`, `_hf1`, ... numbering (can reset per segment)

**Observed _fnSignal patterns:**

| Input Expression | Hoisted Function | String | Deps |
|------------------|-----------------|--------|------|
| `12 + signal.value` | `(p0)=>12 + p0.value` | `"12+p0.value"` | `[signal]` |
| `store.address.city.name` | `(p0)=>p0.address.city.name` | `"p0.address.city.name"` | `[store]` |
| `store.address.city.name ? 'true' : 'false'` | `(p0)=>p0.address.city.name ? 'true' : 'false'` | `'p0.address.city.name?"true":"false"'` | `[store]` |
| `store.nested.count` | `(p0)=>p0.nested.count` | `"p0.nested.count"` | `[store]` |
| `store.stuff + 12` | `(p0)=>p0.stuff + 12` | `"p0.stuff+12"` | `[store]` |
| `signal.formData?.get('username')` | `(p0)=>p0.formData?.get('username')` | `'p0.formData?.get("username")'` | `[signal]` |

**Evidence:** `example_derived_signals_cmp.md` (lines 68-73, 97-99), `example_getter_generation.md` (lines 83-84, 127-132)

### 2.4 When NOT to use signal wrappers

The following cases produce **plain values** (no wrapping):

- **Global variable access**: `globalThing.thing` stays as-is (goes into varProps)
- **Function call results**: `signal.value()` stays as-is (goes into varProps)
- **Mixed signal+non-reactive**: `signal.value + dep` stays as-is (goes into varProps)
- **Mutable wrappers**: `mutable(signal)` stays as-is (goes into varProps)
- **Signal identifiers (no .value)**: `signal={signal}` passes the signal directly (constProps, no wrapping)
- **Imported dep access**: `dep.thing` stays as-is (constProps for component)

**Evidence:** `example_derived_signals_cmp.md` lines 81-88 (varProps) and 100-101 (constProps unwrapped)

### 2.5 OXC API Mapping for Signals

**Detection phase (in Traverse):**
1. When processing JSX prop values, analyze each expression:
   - Is it `identifier.value`? -> `_wrapProp(identifier)`
   - Is it a computed expression containing `.value` or store property access? -> hoist + `_fnSignal`
   - Is it a plain value? -> keep as-is, classify into var or const

**Construction phase:**
1. `ast.expression_call(wrapProp_ident, vec![source_expr])` -- build `_wrapProp(signal)`
2. `ast.expression_call(wrapProp_ident, vec![source_expr, prop_name])` -- build `_wrapProp(store, "count")`
3. `ast.expression_call(fnSignal_ident, vec![hf_ident, deps_array, hf_str])` -- build `_fnSignal`
4. For hoisted functions: `ast.declaration_variable(VariableDeclarationKind::Const, ...)` at module top level

---

## 3. Props Destructuring (APIM-09 partial)

### 3.1 Overview

When a component's callback argument uses destructuring, the optimizer replaces it with `_rawProps` and transforms all property accesses.

### 3.2 Transformation Rules

**Input pattern:**
```tsx
component$(({ message, id, count: c, ...rest }) => {
    return <div id={id}>{message} {c}</div>;
})
```

**Output pattern:**
```javascript
export const SEGMENT = (_rawProps) => {
    const rest = _restProps(_rawProps, ["message", "id", "count"]);
    return _jsxSorted("div", {
        id: _wrapProp(_rawProps, "id")
    }, null, [
        _wrapProp(_rawProps, "message"),
        " ",
        _wrapProp(_rawProps, "count")  // "c" renamed prop uses original name
    ], 1, "u6_0");
};
```

**Key rules:**
1. The destructured parameter `({message, id, count: c, ...rest})` becomes `(_rawProps)`
2. Each named property access becomes `_wrapProp(_rawProps, "propertyName")`
3. **Renamed props**: `count: c` -- the `c` references are replaced with `_wrapProp(_rawProps, "count")` using the ORIGINAL property name
4. **Rest patterns**: `...rest` becomes `const rest = _restProps(_rawProps, ["message", "id", "count"])` listing all named destructured properties
5. **Spread in JSX**: `{...rest}` on elements triggers `_jsxSplit` with `_getVarProps(rest)` and `_getConstProps(rest)`
6. The segment metadata includes `"paramNames": ["_rawProps"]`
7. This transform must run BEFORE capture analysis (since it changes variable references)

**Evidence:** `should_destructure_args.md` (complete example with all patterns)

### 3.3 Non-destructured Props

When the parameter is NOT destructured (e.g., `component$((props) => ...)`), no `_rawProps` transformation occurs. Instead, prop access uses `_wrapProp(props, "propName")` directly:

```javascript
export const Cmp_component_HASH = (props) => {
    return _jsxSorted(_Fragment, null, null, [
        _jsxSorted("p", {"data-value": _wrapProp(props, "count")}, null,
            _fnSignal(_hf0, [props], _hf0_str), 1, null),
    ], 1, "u6_1");
};
```

**Evidence:** `example_getter_generation.md` (lines 85-96) -- `Cmp` receives `(props)` without destructuring

### 3.4 OXC API Mapping for Props Destructuring

**Detection (in enter_arrow_function_expression or enter_function):**
1. Check if the function is a direct callback to a `component$()` call
2. Check if the first parameter is an ObjectPattern (destructuring)
3. If yes, collect all property names and rest element

**Transformation:**
1. Replace ObjectPattern param with `ast.binding_pattern_identifier("_rawProps")`
2. For rest element: insert `const rest = _restProps(_rawProps, [names...])` at function body start
3. For each reference to a destructured property: replace with `_wrapProp(_rawProps, "originalName")`
4. Add `_restProps`, `_wrapProp`, `_getVarProps`, `_getConstProps` to import list as needed

---

## 4. Entry Strategy Patterns (APIM-10 partial)

### 4.1 Overview

The entry strategy determines how extracted segments are referenced in the main module. Seven strategies exist:

| Strategy | QRL Function | Import Pattern | Segment Output |
|----------|-------------|----------------|----------------|
| **Segment** (default) | `qrl()` | `const i_HASH = () => import("./file_HASH")` | Separate `.js` file |
| **Inline** | `inlinedQrl()` | None (body stays in module) | Inlined in main |
| **Hoist** | `inlinedQrl()` | None (body hoisted to top-level const) | Top-level function |
| **Single** | `qrl()` | Single shared chunk import | Grouped into one file |
| **Component** | `qrl()` | Per-component chunk import | Grouped by component |
| **Smart** | `qrl()` | Automatic grouping | Grouped by heuristics |
| **Hook** | `qrl()` | Per-hook chunk import | Grouped by hook type |

### 4.2 Segment Strategy (Default)

Each `$()` extraction becomes a separate lazy-loaded module with a dynamic `import()`.

```javascript
// Main module
const i_HASH = () => import("./test.tsx_NAME_HASH");
export const App = componentQrl(qrl(i_HASH, "NAME_HASH"));

// Segment module (separate file)
export const NAME_HASH = () => { ... };
```

**Evidence:** `example_getter_generation.md`, `example_strip_server_code.md`, `example_of_synchronous_qrl.md`, `example_manual_chunks.md` -- all use Segment strategy

### 4.3 Inline Strategy

The segment body is kept inline using `inlinedQrl()` instead of being extracted:

```javascript
// Input:
export const Child = component$(() => {
    useStyles$('somestring');
    const state = useStore({ count: 0 });
    ...
});

// Output (Inline strategy):
export const Child = componentQrl(inlinedQrl(Child_component_HASH, "Child_component_HASH", [state]));
// Where Child_component_HASH is defined as a top-level const in the same module
```

With inline strategy, the segment function body is extracted to a **top-level const** in the same file, and `inlinedQrl()` references it by name. Captures are passed as the third argument array.

**Evidence:** `example_inlined_entry_strategy.md`, `example_input_bind.md`

### 4.4 Hoist Strategy

Similar to Inline but the function body is hoisted and referenced directly:

```javascript
const App_component_HASH = () => { ... };
export const App = componentQrl(inlinedQrl(App_component_HASH, "App_component_HASH"));
```

**Evidence:** `example_derived_signals_cmp.md` (Entry Strategy: Hoist)

### 4.5 Smart Strategy (Manual Chunks)

Smart strategy groups related segments. The output structure is the same as Segment (separate files, `qrl()` calls) but the bundler receives hints about which segments should be co-located.

**Evidence:** `example_manual_chunks.md` (Entry Strategy: Smart) -- segments still output as separate files

### 4.6 Naming Patterns

| Mode | Naming | Example |
|------|--------|---------|
| **Dev/Test** | Descriptive | `App_component_ckEPmXZlub0`, `Parent_component_useTask_gDH1EtUWqBU` |
| **Prod** | `s_` prefix | `s_ckEPmXZlub0`, `s_gDH1EtUWqBU` |

In Prod mode, all segment export names use the short `s_HASH` form. In Dev/Test mode, they use the descriptive `DisplayName_HASH` form.

**Evidence:** `example_strip_server_code.md` (Prod mode, `s_` prefix), `example_derived_signals_cmp.md` (Hoist/default mode, descriptive names)

### 4.7 Dev Mode: qrlDEV

In Dev mode, `qrl()` is replaced with `qrlDEV()` which adds debug metadata:

```javascript
qrlDEV(i_HASH, "segment_name_HASH", {
    file: "/user/qwik/src/test.tsx",
    lo: 503,
    hi: 575,
    displayName: "test.tsx_test_component"
})
```

And `_noopQrl()` becomes `_noopQrlDEV()`:

```javascript
_noopQrlDEV("api_server_HASH", {
    file: "/user/qwik/src/test.tsx",
    lo: 0,
    hi: 0,
    displayName: "test.tsx_api_server"
})
```

**Evidence:** `example_drop_side_effects.md` (Dev mode output lines 63-84)

### 4.8 OXC API Mapping for Entry Strategy

The entry strategy is configured via `TransformModulesOptions.entry_strategy` and affects the emit phase:

1. **Segment/Smart/Component/Hook/Single**: Build `qrl()` / `qrlDEV()` call + lazy import declaration
2. **Inline/Hoist**: Build `inlinedQrl()` call, keep segment body in same module as top-level const

The OXC API calls are the same as documented in Phase 4's API-MAPPING.md (Pattern 2: QRL Wrapping). The strategy selection is a configuration-driven branch, not a different set of APIs.

---

## 5. Code Stripping (APIM-10 partial)

### 5.1 Overview

When `strip_ctx_name` is configured (e.g., `["server"]`), any `$`-suffixed call whose `ctxName` matches a stripped name has its QRL replaced with `_noopQrl()` (Prod) or `_noopQrlDEV()` (Dev).

### 5.2 Stripping Rules

1. **Matched names**: If the `$`-suffixed function name contains a stripped context name (e.g., `serverStuff$`, `serverLoader$`, `server$`), its QRL becomes a noop
2. **Nested `$()` preserved**: `$()` and `client$()` calls INSIDE a stripped callback body are NOT stripped -- they produce their own segments normally
3. **Side effect dropping**: Imports and module-level expressions used ONLY by stripped code are removed
4. **IIFE preservation**: Self-executing functions and standalone side-effect calls are preserved

**Stripping output:**
```javascript
// Prod mode:
serverStuffQrl(_noopQrl("s_r1qAHX7Opp0"));
serverLoaderQrl(_noopQrl("s_k1L0DiPQV1I"));

// Dev mode:
serverQrl(_noopQrlDEV("api_server_JonPp043gH0", {
    file: "/user/qwik/src/test.tsx", lo: 0, hi: 0,
    displayName: "test.tsx_api_server"
}));
```

### 5.3 Side Effect Analysis

When code is stripped, the optimizer performs dead code elimination on associated declarations:

| Code Type | Stripped? | Reason |
|-----------|----------|--------|
| `import { secret } from './secret'` | YES (if only used by stripped code) | No remaining references |
| `const supabase = clientSupabase()` | YES (if only feeds stripped code) | Result only used in stripped callback |
| `(function() { console.log('run'); })()` | NO | Observable IIFE side effect |
| `(() => { console.log('run'); })()` | NO | Observable arrow IIFE side effect |
| `sideEffect()` | NO | Observable function call |
| `import { sideEffect } from './secret'` | NO (if sideEffect still called) | Still has live references |

**Evidence:** `example_drop_side_effects.md` (lines 59-84, lines 185-189)

### 5.4 Export Stripping

When entire exports are stripped, their body is replaced with a throw:

```javascript
export function handler() {
    throw "Symbol removed by Qwik Optimizer...";
}
```

This pattern applies to exported functions/classes whose bodies are server-only.

### 5.5 OXC API Mapping for Stripping

1. **Detection**: Check `ctx_name` against `strip_ctx_name` list during `$()` detection pass
2. **Noop construction**: `ast.expression_call(noopQrl_ident, vec![ast.expression_string_literal(hash)])` for Prod
3. **Dev noop**: `ast.expression_call(noopQrlDEV_ident, vec![name_literal, debug_obj])` for Dev
4. **Side effect analysis**: Use `oxc_semantic` reference tracking to identify declarations with no remaining live references after stripping
5. **Import cleanup**: Remove import specifiers whose local bindings have no remaining references

---

## 6. Const Folding / Replacement (APIM-10 partial)

### 6.1 Overview

The optimizer evaluates certain constant expressions at compile time:

1. **`isServer` evaluation**: In Prod mode, `isServer` is replaced with `true` (server build) or `false` (client build), enabling dead branch elimination
2. **Static expression evaluation**: Expressions like `'true' + 1 ? 'true' : ''` are evaluated to `'true'` at compile time

### 6.2 isServer Dead Branch Elimination

```typescript
// Input:
useTask$(async () => {
    if (!isServer) return;
    state.text = await mongo.users();
});

// Output (Prod mode, server build):
// The `if (!isServer) return;` guard is eliminated entirely
// because isServer evaluates to true, making `!isServer` false,
// so the early return is dead code
export const s_gDH1EtUWqBU = async () => {
    const state = _captures[0];
    state.text = await mongo.users();
    redis.set(state.text);
};
```

**Evidence:** `example_strip_server_code.md` lines 91-95 (guard removed), conventions section (line 299: CONV-10)

### 6.3 Static Prop Evaluation

```typescript
// Input:  prop={'true' + 1 ? 'true' : ''}
// Output: prop: 'true'
```

The expression `'true' + 1` evaluates to `"true1"` (truthy), so the ternary resolves to `'true'`.

**Evidence:** `example_getter_generation.md` lines 140-141 (`prop: 'true'`)

### 6.4 OXC API Mapping for Const Folding

1. **isServer replacement**: During traverse, when encountering `Identifier("isServer")` imported from `@qwik.dev/core`:
   - Replace with `ast.expression_boolean_literal(is_server_value)`
   - Then simplify `if (false) { ... }` -> remove block, `if (true) { ... }` -> keep body only
2. **Static evaluation**: Implement a small constant folder for:
   - String concatenation
   - Numeric arithmetic
   - Boolean operations
   - Ternary with known condition
3. **Dead branch removal**: After const replacement, use `if (false_literal) { ... }` detection to remove dead branches

---

## 7. Input Binding (CONV-12)

### 7.1 Overview

The `bind:` directive on input elements is a sugar for two-way binding:

```tsx
// Input:
<input bind:value={value} />
<input bind:checked={checked} />
<input bind:stuff={stuff} />
```

### 7.2 Transformation Rules

| Directive | Output Props | Handler | Handler Import |
|-----------|-------------|---------|----------------|
| `bind:value` | `"value": signal, "q-e:input": inlinedQrl(_val, "_val", [signal])` | `_val` | `@qwik.dev/core` |
| `bind:checked` | `"checked": signal, "q-e:input": inlinedQrl(_chk, "_chk", [signal])` | `_chk` | `@qwik.dev/core` |
| `bind:*` (other) | `"bind:stuff": signal` | None (passed through) | N/A |

**Key observations:**
- `bind:value` and `bind:checked` are special-cased with built-in handlers (`_val`, `_chk`)
- Other `bind:*` directives pass through as-is
- The handler is wrapped with `inlinedQrl()` capturing the signal
- Both the value prop and event handler go into `constProps`

**Evidence:** `example_input_bind.md` (output lines 694-709)

### 7.3 OXC API Mapping

1. **Detection**: In JSX attribute processing, check for `bind:` prefix on attribute name
2. **Special cases**: If attribute is `bind:value` or `bind:checked`, generate the corresponding handler
3. **Construction**: Build `inlinedQrl(_val_ident, "_val", [signal_expr])` call expression
4. **Import**: Add `_val` and/or `_chk` to import list from `@qwik.dev/core`

---

## 8. Sync$ Serialization (CONV-13)

### 8.1 Overview

`sync$()` wraps synchronous event handlers that need to be serialized as strings for SSR:

```tsx
// Input:
<input onClick$={sync$(function(event, target) {
    // comment should be removed
    event.preventDefault();
})}/>

// Output:
_jsxSorted("input", {
    "q-e:click": _qrlSync(function(event, target) {
        event.preventDefault();
    }, "function(event,target){event.preventDefault();}")
}, null, null, 2, null)
```

### 8.2 Transformation Rules

1. `sync$()` is replaced with `_qrlSync()` (not `qrl` or `inlinedQrl`)
2. The function body is kept inline (not extracted to a segment)
3. A **stringified representation** of the function is added as the second argument
4. Comments are removed from the stringified version
5. Whitespace is minified in the string representation
6. The string preserves function keyword vs arrow function syntax
7. `_qrlSync` results go into **varProps** (not constProps) of the JSX call

**Observed string forms:**
- `"function(event,target){event.preventDefault();}"` -- function expression
- `"(event,target)=>{event.preventDefault();}"` -- arrow with block body
- `"(event,target)=>event.preventDefault()"` -- arrow with expression body

**Evidence:** `example_of_synchronous_qrl.md` (output lines 789-806)

### 8.3 OXC API Mapping

1. **Detection**: When processing `$()` calls, check if the callee is `sync$`
2. **No segment extraction**: `sync$` callbacks are NOT extracted to segments
3. **Stringification**: Use `oxc_codegen::Codegen` to print the function body to a string (minified, no comments)
4. **Construction**: `ast.expression_call(_qrlSync_ident, vec![function_expr, string_literal])`

---

## 9. Hoisted Functions (CONV-14)

### 9.1 Overview

Hoisted functions (`_hfN`, `_hfN_str`) are generated for computed signal expressions. They are documented in Section 2.3 above but have their own CONV type because they appear as top-level declarations in segment modules.

### 9.2 Placement Rules

1. Hoisted functions are placed at the **top of the module**, before any export declarations
2. Each hoisted function has a matching string constant
3. Numbering is per-module: `_hf0`, `_hf1`, `_hf2`, etc.
4. The same `_hf0` name can appear in different segments (each segment is a separate module)

### 9.3 OXC API Mapping

1. Collect all `_fnSignal` candidates during JSX prop analysis
2. For each candidate, create:
   - `const _hfN = (p0) => EXPR;` via `ast.declaration_variable(...)` + `ast.expression_arrow_function(...)`
   - `const _hfN_str = "MINIFIED_EXPR";` via `ast.declaration_variable(...)` + `ast.expression_string_literal(...)`
3. Insert these declarations at program body index 0 (after imports, before exports) via `program.body.insert()`

---

## 10. Source Map Integration (APIM-07)

### 10.1 Prior Research

Phase 5 (POC-04) validated the source map generation approach. Key findings:

1. `oxc_codegen::Codegen::new().with_options(CodegenOptions { source_map_path: Some(path) }).with_source_text(original_source).build(&program)` generates source maps
2. **Span preservation strategy**: Original spans on extracted AST nodes, `SPAN` (zero span) on constructed nodes
3. **Program span**: Must encompass all child spans for the source map builder to work correctly
4. Source maps are generated per-module (main module + each segment module)

### 10.2 Phase 6 Additions

The JSX, signal, and props transformations create new nodes that need span decisions:

| Node Type | Span Strategy | Rationale |
|-----------|--------------|-----------|
| `_jsxSorted(...)` call | SPAN (zero) | Constructed replacement for JSX |
| Props objects (var/const) | SPAN (zero) | Newly constructed |
| Original prop values | Preserve original span | Extracted from input JSX attributes |
| `_wrapProp(...)` call | SPAN (zero) | Constructed wrapper |
| `_fnSignal(...)` call | SPAN (zero) | Constructed wrapper |
| Hoisted function body expr | Preserve original span | Extracted from prop expression |
| Hoisted function string | SPAN (zero) | Constructed |
| `_noopQrl(...)` call | SPAN (zero) | Constructed replacement |
| `_qrlSync(...)` call | SPAN (zero) | Constructed replacement |
| Stringified function arg | SPAN (zero) | Constructed |

**Principle**: Preserve spans on AST nodes that were extracted/moved from the original source; use zero span on nodes that are purely constructed by the optimizer.

### 10.3 OXC API Reference

```rust
use oxc_codegen::{Codegen, CodegenOptions};
use oxc_sourcemap::SourceMap;

let codegen = Codegen::new()
    .with_options(CodegenOptions {
        source_map_path: Some(source_map_path.into()),
        ..Default::default()
    })
    .with_source_text(original_source);

let codegen_result = codegen.build(&program);
// codegen_result.code: String
// codegen_result.source_map: Option<SourceMap>
```

---

## 11. Complete 14 CONV Type Cross-Reference (APIM-08)

### 11.1 Cross-Reference Table

| CONV | Name | OXC Detection API | OXC Construction API | Primary Spec Files | Phase Mapped |
|------|------|-------------------|---------------------|-------------------|--------------|
| CONV-01 | QRL Calls | `enter_call_expression` -> check callee is `$`-suffixed | `ast.expression_call(qrl_ident, vec![import_fn, name_str])` or `ast.expression_call(inlinedQrl_ident, vec![fn_ref, name_str, captures])` | example_1.md, example_functional_component.md | Phase 4 (API-MAPPING.md Pattern 2) |
| CONV-02 | Dollar-to-QRL Suffix | `enter_call_expression` -> check callee ends with `$` | `ast.expression_identifier_reference("componentQrl")` replacing `component$` callee | example_functional_component.md, example_derived_signals_cmp.md | Phase 4 (API-MAPPING.md Pattern 1) |
| CONV-03 | JSX Transforms | `enter_jsx_element` / `enter_jsx_fragment` | `ast.expression_call(_jsxSorted_ident, vec![tag, var, const, children, flags, key])` | example_jsx.md, example_derived_signals_cmp.md, should_destructure_args.md | Phase 6 (this document, Section 1) |
| CONV-04 | Signal Helpers | JSX prop value analysis (member expr `.value`, store property chains) | `ast.expression_call(_wrapProp_ident, ...)`, `ast.expression_call(_fnSignal_ident, ...)` | example_derived_signals_cmp.md, example_getter_generation.md | Phase 6 (this document, Section 2) |
| CONV-05 | Capture Patterns | `oxc_semantic` scoping (Phase 5 `compute_captures()`) | `ast.expression_member(captures_ident, numeric_index)` -> `_captures[N]` | example_multi_capture.md, example_strip_server_code.md | Phase 5 (CAPTURE-ANALYSIS-MAPPING.md) |
| CONV-06 | Lazy Imports | Generated during emit phase for Segment/Smart/Component/Hook strategies | `ast.declaration_variable(Const, ...)` with `ast.expression_arrow_function([], import_call)` | example_1.md, example_getter_generation.md | Phase 4 (API-MAPPING.md Pattern 2) |
| CONV-07 | PURE Annotations | Generated for all framework call replacements | `oxc_codegen` annotation support or manual `/*#__PURE__*/` comment insertion | All spec files | Phase 4/6 (cross-cutting) |
| CONV-08 | Segment Extraction | `enter_call_expression` -> `$()` detection collects segment boundaries | `build_segment_program()` (Phase 5 MULTI-MODULE-OUTPUT-MAPPING.md) | All spec files with segments | Phase 5 (MULTI-MODULE-OUTPUT-MAPPING.md) |
| CONV-09 | Code Stripping | `enter_call_expression` -> check `ctx_name` against `strip_ctx_name` | `ast.expression_call(_noopQrl_ident, vec![hash_str])` | example_strip_server_code.md, example_drop_side_effects.md | Phase 6 (this document, Section 5) |
| CONV-10 | Const Replacement | `enter_identifier_reference` -> check for `isServer` | `ast.expression_boolean_literal(value)` + dead branch removal | example_strip_server_code.md | Phase 6 (this document, Section 6) |
| CONV-11 | Props Destructuring | `enter_arrow_function_expression` -> check param is ObjectPattern in `component$()` | Replace param, insert `_restProps()`, replace references with `_wrapProp(_rawProps, name)` | should_destructure_args.md | Phase 6 (this document, Section 3) |
| CONV-12 | Input Binding | JSX attribute processing -> check for `bind:` prefix | Build `inlinedQrl(_val, "_val", [signal])` or `inlinedQrl(_chk, "_chk", [signal])` | example_input_bind.md | Phase 6 (this document, Section 7) |
| CONV-13 | Sync$ Serialization | `enter_call_expression` -> check callee is `sync$` | `ast.expression_call(_qrlSync_ident, vec![fn_expr, stringified])` | example_of_synchronous_qrl.md | Phase 6 (this document, Section 8) |
| CONV-14 | Hoisted Functions | Generated during signal optimization for computed expressions | `ast.declaration_variable(Const, _hfN, arrow_fn)` + `ast.declaration_variable(Const, _hfN_str, string)` | example_derived_signals_cmp.md, example_getter_generation.md | Phase 6 (this document, Section 9) |

### 11.2 CONV Type Frequency Across 162 Spec Files

| CONV | Occurrence Count | Notes |
|------|-----------------|-------|
| CONV-01 | 162 | Universal -- every spec has QRL calls |
| CONV-02 | 157 | Nearly universal -- most specs use `$`-suffixed APIs |
| CONV-03 | 113 | All specs with JSX transpilation enabled |
| CONV-04 | 25 | Specs with reactive signal/store prop access |
| CONV-05 | 45 | Specs with captured variables in segments |
| CONV-06 | 107 | Specs using Segment/Smart/Component entry strategies |
| CONV-07 | 139 | Nearly universal -- PURE annotations on framework calls |
| CONV-08 | 121 | Specs that extract segments (not inline-only) |
| CONV-09 | 8 | Specs with strip_ctx_name configuration |
| CONV-10 | 5 | Specs with isServer/const folding |
| CONV-11 | 5 | Specs with destructured component props |
| CONV-12 | 2 | Specs with bind: directives |
| CONV-13 | 3 | Specs with sync$ usage |
| CONV-14 | 17 | Specs with computed signal expressions |

### 11.3 CONV Dependency Ordering

Some CONVs must execute before others:

```
CONV-11 (Props Destructuring) BEFORE CONV-05 (Capture Analysis)
  Reason: Destructuring changes variable references that capture analysis reads

CONV-01 ($() Detection) BEFORE CONV-08 (Segment Extraction)
  Reason: Detection identifies segment boundaries; extraction uses them

CONV-01 ($() Detection) BEFORE CONV-09 (Code Stripping)
  Reason: Stripping decisions depend on detected $() call ctx_name

CONV-05 (Capture Analysis) BEFORE CONV-08 (Segment Extraction)
  Reason: Segments need capture lists to build _captures imports

CONV-03 (JSX Transforms) CONCURRENT WITH CONV-04 (Signal Helpers)
  Reason: Signal helpers are generated during JSX prop analysis

CONV-03 (JSX Transforms) BEFORE CONV-07 (PURE Annotations)
  Reason: PURE annotations are added to JSX call expressions during/after construction

CONV-10 (Const Replacement) BEFORE CONV-09 (Code Stripping)
  Reason: isServer evaluation can create additional dead code for stripping
```

---

## 12. Prior Phase Dependencies

### 12.1 What Is Already Mapped

| Requirement | Phase | Document | Status |
|-------------|-------|----------|--------|
| APIM-01 ($() extraction) | Phase 4 | API-MAPPING.md Pattern 1 | Complete |
| APIM-02 (QRL wrapping) | Phase 4 | API-MAPPING.md Pattern 2 | Complete |
| APIM-03 (Capture analysis) | Phase 5 | CAPTURE-ANALYSIS-MAPPING.md | Complete |
| APIM-05 (Import rewriting) | Phase 4 | API-MAPPING.md Pattern 3 | Complete |
| APIM-06 (Multi-module output) | Phase 5 | MULTI-MODULE-OUTPUT-MAPPING.md | Complete |

### 12.2 What Phase 6 Must Produce

| Requirement | Deliverable | Key Content |
|-------------|------------|-------------|
| APIM-04 | JSX-TRANSFORMS-MAPPING.md | _jsxSorted/_jsxSplit construction, prop classification, Fragment handling, event handler transform, dev mode |
| APIM-07 | SOURCE-MAPS-MAPPING.md (or integrated section) | Span strategy for all new node types, complete codegen pipeline |
| APIM-08 | CONV-CROSS-REFERENCE.md | The table from Section 11 with full OXC API patterns for each CONV |
| APIM-09 | PROPS-SIGNALS-MAPPING.md | Props destructuring (_rawProps, _restProps), signal helpers (_wrapProp, _fnSignal, _hfN) |
| APIM-10 | ENTRY-STRIPPING-CONST-MAPPING.md | Entry strategy variants, code stripping (_noopQrl), const folding (isServer), side effect analysis |

---

## 13. Key Risks and Research Gaps

### 13.1 PURE Annotation Insertion (CONV-07)

**Risk:** OXC's codegen may not have a built-in mechanism for `/*#__PURE__*/` comment annotations on call expressions. The SWC-based optimizer uses `@__PURE__` annotations in the AST. Need to verify:
- Does `oxc_codegen` respect `LeadingComment` nodes on expressions?
- Can we attach `/*#__PURE__*/` as a leading comment to CallExpression nodes?
- Alternative: Is there an `annotate_pure` flag on CallExpression in OXC's AST?

**Mitigation:** OXC's `oxc_ast` has a `Comment` type and `Program.comments` vec. The codegen should print leading comments. Verify in OXC 0.113 docs.

### 13.2 JSX Flags Semantics

**Risk:** The exact semantics of the `flags` argument to `_jsxSorted` are not fully documented. Values 0, 1, 2, 3 are observed but the encoding logic is unclear. May need to reverse-engineer from Qwik runtime or existing SWC optimizer.

**Mitigation:** The flags can be pattern-matched from spec file analysis: `3` = leaf/simple element, `1` = container with children array, `0` = spread element, `2` = event-only element. This heuristic covers all observed cases.

### 13.3 Static Expression Evaluator Scope

**Risk:** The const folding (Section 6.3) requires a mini expression evaluator. How complex must this be? Must it handle:
- String concatenation? (Yes, per evidence)
- Ternary with known condition? (Yes, per evidence)
- Template literal evaluation? (Yes -- `\`text${12}\`` evaluates)
- `typeof` operator? (Yes -- `typeof \`text${12}\`` === 'string'`)

**Mitigation:** Scope the evaluator to handle: string concat, numeric arithmetic, boolean ops, ternary, typeof, template literals. Do not attempt to evaluate function calls or complex expressions.

### 13.4 Side Effect Analysis for Stripping

**Risk:** Determining which declarations are "only used by stripped code" requires full reference analysis. This is more complex than simple dead code elimination.

**Mitigation:** Leverage `oxc_semantic` reference tracking. After marking stripped segments, walk all remaining code to find live references. Any declaration with zero live references can be removed.

### 13.5 Props Destructuring + Capture Interaction

**Risk:** Props destructuring must run before capture analysis, but both need scoping information. If props destructuring modifies the AST, the scoping data becomes stale.

**Mitigation:** Two-pass approach: (1) Detect and transform props destructuring patterns during initial traverse, (2) Re-run semantic analysis (or track changes manually) before capture analysis pass. Alternatively, perform props destructuring as a pre-pass before the main transformation traverse.

---

## 14. Recommended Plan Structure

### 14.1 Suggested Task Breakdown

**Plan 06-01: JSX Transform + Signal Optimization Mapping**
- Document _jsxSorted/_jsxSplit construction with OXC AstBuilder
- Document prop classification algorithm (var vs const)
- Document signal helper generation (_wrapProp, _fnSignal, hoisted functions)
- Document Fragment handling and event handler transformation
- Deliverable: JSX-TRANSFORMS-MAPPING.md + PROPS-SIGNALS-MAPPING.md

**Plan 06-02: Entry Strategy, Stripping, and Const Folding Mapping**
- Document entry strategy variants (Segment, Inline, Hoist, Smart, etc.)
- Document code stripping (_noopQrl, nested preservation, side effect analysis)
- Document const folding (isServer, static expression evaluation)
- Document Dev mode patterns (qrlDEV, _noopQrlDEV, JSX devInfo)
- Deliverable: ENTRY-STRIPPING-CONST-MAPPING.md

**Plan 06-03: Cross-Reference Table + Source Map Integration**
- Compile the 14 CONV type cross-reference with full OXC API patterns
- Document span strategy for all Phase 6 node types
- Document PURE annotation mechanism
- Validate dependency ordering across all CONVs
- Deliverable: CONV-CROSS-REFERENCE.md + SOURCE-MAPS-MAPPING.md (or integrated)

### 14.2 Estimated Complexity

| Task | Complexity | Rationale |
|------|-----------|-----------|
| JSX Transform Mapping | Medium-High | Many edge cases in prop classification, two function variants |
| Signal Optimization | Medium | Well-documented in specs, mostly pattern matching |
| Props Destructuring | Medium | Clear transformation rules, limited edge cases |
| Entry Strategy | Low-Medium | Mostly configuration-driven branching of existing patterns |
| Code Stripping | Medium-High | Side effect analysis is complex, nested preservation rules |
| Const Folding | Low-Medium | Small evaluator scope, well-defined cases |
| Cross-Reference | Low | Compilation of existing research |
| Source Maps | Low | Extension of Phase 5 work with span assignments |

---

## Sources

### Primary Sources (Spec Files)
- `.planning/spec/example_jsx.md` -- JSX transform patterns (CONV-03)
- `.planning/spec/example_derived_signals_cmp.md` -- Signal optimization, Hoist strategy (CONV-04, CONV-14)
- `.planning/spec/example_getter_generation.md` -- _fnSignal patterns, optional chaining (CONV-04, CONV-14)
- `.planning/spec/should_destructure_args.md` -- Props destructuring (CONV-11)
- `.planning/spec/example_strip_server_code.md` -- Code stripping, const folding (CONV-09, CONV-10)
- `.planning/spec/example_drop_side_effects.md` -- Dev mode, side effect analysis (CONV-09)
- `.planning/spec/example_inlined_entry_strategy.md` -- Inline strategy (CONV-01)
- `.planning/spec/example_input_bind.md` -- Input binding (CONV-12)
- `.planning/spec/example_of_synchronous_qrl.md` -- Sync$ serialization (CONV-13)
- `.planning/spec/example_manual_chunks.md` -- Smart strategy (CONV-08)

### Prior Phase Documents
- `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md` -- APIM-01, APIM-02, APIM-05
- `.planning/phases/04-core-api-mapping-architecture/ARCHITECTURE-BLUEPRINT.md` -- Crate layout, public API
- `.planning/phases/05-deep-research-proof-of-concept/CAPTURE-ANALYSIS-MAPPING.md` -- APIM-03
- `.planning/phases/05-deep-research-proof-of-concept/MULTI-MODULE-OUTPUT-MAPPING.md` -- APIM-06
- `.planning/phases/05-deep-research-proof-of-concept/05-VERIFICATION.md` -- Phase 5 validation

---

*Research completed: 2026-02-10*
*Researcher: Claude (gsd-phase-researcher)*
