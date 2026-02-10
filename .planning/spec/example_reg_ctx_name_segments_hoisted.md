# Test: example_reg_ctx_name_segments_hoisted

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Reg Ctx Name | ["server"] |

**Key insight:** Hoist strategy combines segment extraction with hoisting -- segments are defined as top-level variables in the main module rather than in separate files. `_regSymbol` is used to register the server$ handler for server-side execution. `useStyle$` -> `useStyleQrl` with inline style value. The `STYLES` constant is exported via `_auto_STYLES` alias for cross-reference. All code stays in one file with hoisted segment definitions.

## Input

### Source Code
```tsx
import { $, component$, server$, useStyle$ } from '@qwik.dev/core';

export const Works = component$((props) => {
	useStyle$(STYLES);
	const text = 'hola';
	return (
		<div onClick$={server$(() => console.log('in server', text))}></div>
	);
});

const STYLES = '.class {}';
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Component with useStyle$, server$ handler, and bottom-declared STYLES constant.*

</details>

## Output

### Module: test.js (main module -- single output with hoisted segments)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { useStyleQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { serverQrl } from "@qwik.dev/core";
import { _regSymbol } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const Works_component_div_q_e_click_server_q39lOt7xGrI = /*#__PURE__*/ _regSymbol(()=>console.log('in server', 'hola'), "q39lOt7xGrI");
const Works_component_t45qL4vNGv0 = (props)=>{
    useStyleQrl(/*#__PURE__*/ inlinedQrl(STYLES, "Works_component_useStyle_i40UL9JyQpg"));
    return /*#__PURE__*/ _jsxSorted("div", {
        "q-e:click": serverQrl(/*#__PURE__*/ inlinedQrl(Works_component_div_q_e_click_server_q39lOt7xGrI, "Works_component_div_q_e_click_server_q39lOt7xGrI"))
    }, null, null, 2, "u6_0");
};
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(Works_component_t45qL4vNGv0, "Works_component_t45qL4vNGv0"));
const STYLES = '.class {}';
export { STYLES as _auto_STYLES };
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Hoist strategy: segments defined as top-level const variables. Server handler defined before component, registered with _regSymbol. STYLES exported via _auto_ alias. useStyle$ -> useStyleQrl with inlinedQrl wrapping the STYLES value.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for hoisted references (code defined in same file)
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `useStyle$` -> `useStyleQrl`, `server$` -> `serverQrl`
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()`
- **[CONV-07] PURE Annotations**: On componentQrl, inlinedQrl, _regSymbol, _jsxSorted

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 3 |
| useStyleQrl | test.js | @qwik.dev/core | 1 |
| serverQrl | test.js | @qwik.dev/core | 1 |
| _regSymbol | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 1 |
| console.log | test.js | (global) | 1 |

## Diagnostics

```json
[]
```
