# Test: example_reg_ctx_name_segments_inlined

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Reg Ctx Name | ["server"] |

**Key insight:** Inline strategy with `reg_ctx_name: ["server"]`. Similar to `example_reg_ctx_name_segments` but without `strip_event_handlers`, so the regular onClick$ handler is NOT stripped (no `_noopQrl`). All code stays inline. The `server$` handler is wrapped with `serverQrl(inlinedQrl(_regSymbol(...)))`. The captured `text` variable is inlined as the literal `'hola'`.

## Input

### Source Code
```tsx
import { $, component$, server$ } from '@qwik.dev/core';
export const Works = component$((props) => {
	const text = 'hola';
	return (
		<div onClick$={server$(() => console.log('in server', text))}></div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Component with server$ click handler that captures `text`.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { serverQrl } from "@qwik.dev/core";
import { _regSymbol } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    return /*#__PURE__*/ _jsxSorted("div", {
        "q-e:click": serverQrl(/*#__PURE__*/ inlinedQrl(/*#__PURE__*/ _regSymbol(()=>console.log('in server', 'hola'), "q39lOt7xGrI"), "Works_component_div_q_e_click_server_q39lOt7xGrI"))
    }, null, null, 2, "u6_0");
}, "Works_component_t45qL4vNGv0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Inline strategy. server$ handler: serverQrl(inlinedQrl(_regSymbol(...))). Variable `text` inlined as literal 'hola'.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for inline strategy, `serverQrl()` for server handler
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `server$` -> `serverQrl`
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()`
- **[CONV-07] PURE Annotations**: On all wrapper calls

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 2 |
| serverQrl | test.js | @qwik.dev/core | 1 |
| _regSymbol | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 1 |
| console.log | test.js | (global) | 1 |

## Diagnostics

```json
[]
```
