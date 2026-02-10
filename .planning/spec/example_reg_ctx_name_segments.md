# Test: example_reg_ctx_name_segments

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Reg Ctx Name | ["server"] |
| Strip Event Handlers | true |

**Key insight:** Tests server-side code registration with `reg_ctx_name: ["server"]` and `strip_event_handlers: true`. The `server$()` call produces `serverQrl(inlinedQrl(_regSymbol(...)))` -- the handler is wrapped in `_regSymbol` for server-side registration, then wrapped in `inlinedQrl` for QRL serialization, then wrapped in `serverQrl` for the server$ API. The `strip_event_handlers: true` option causes the non-server onClick$ handler to be replaced with `_noopQrl` (a no-op placeholder). The `./foo` import is kept as a side-effect-only import since `foo()` was only used inside the stripped event handler.

## Input

### Source Code
```tsx
import { $, component$, server$ } from '@qwik.dev/core';
import { foo } from './foo';
export const Works = component$((props) => {
	const text = 'hola';
	return (
		<>
		<div onClick$={server$(() => console.log('in server', text))}></div>
		<div onClick$={() => foo()}></div>
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Component with two click handlers: one wrapped in server$() capturing local text variable, one regular handler calling imported foo(). Fragment wrapper for two div elements.*

</details>

## Output

### Module: test.js

```javascript
import "./foo";
import { componentQrl } from "@qwik.dev/core";
import { serverQrl } from "@qwik.dev/core";
import { _regSymbol } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _noopQrl } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", {
            "q-e:click": serverQrl(/*#__PURE__*/ inlinedQrl(/*#__PURE__*/ _regSymbol(()=>console.log('in server', 'hola'), "YY85RDCwwvA"), "Works_component_Fragment_div_q_e_click_server_YY85RDCwwvA"))
        }, null, null, 2, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            "q-e:click": /*#__PURE__*/ _noopQrl("Works_component_Fragment_div_q_e_click_0UiSo8yqgZw")
        }, null, 3, null)
    ], 1, "u6_0");
}, "Works_component_t45qL4vNGv0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. All code inline (Inline strategy). server$ handler triple-wrapped: _regSymbol -> inlinedQrl -> serverQrl. Captured `text` variable constant-folded to `'hola'`. Non-server onClick$ replaced with _noopQrl placeholder. Fragment import from jsx-runtime. Side-effect-only import of "./foo".*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for component body and server handler (Inline strategy)
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `server$` -> `serverQrl`, `onClick$` -> `"q-e:click"` event binding
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` with Fragment from jsx-runtime, array children
- **[CONV-07] PURE Annotations**: On componentQrl, inlinedQrl, _jsxSorted, _regSymbol calls
- **[CONV-09] Code Stripping**: `_noopQrl` replaces non-server event handler (strip_event_handlers: true)
- **[CONV-12] Server Registration**: `_regSymbol()` wraps server$ handler for registration; `serverQrl()` wraps the QRL for server$ API
- **Note**: No CONV-06/08 -- Inline strategy, no segment extraction
- **Note**: Captured variable `text` is constant-folded from `'hola'` directly into the handler

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 2 |
| serverQrl | test.js | @qwik.dev/core | 1 |
| _regSymbol | test.js | @qwik.dev/core | 1 |
| _noopQrl | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 3 |
| console.log | test.js | (global) | 1 |

## Diagnostics

```json
[]
```
