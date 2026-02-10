# Test: example_skip_transform

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Despite the name "skip_transform", this test actually DOES transform the code (transpile_ts and transpile_jsx are both true). The test uses renamed imports (`component$ as Component`, `$ as onRender`) to verify the optimizer handles import aliases correctly. The output preserves the original `$` and `component$` names in calls because transpile flags mean all code stays in one module (no segment extraction occurs since the optimizer recognizes the aliased imports).

## Input

### Source Code
```tsx
import { component$ as Component, $ as onRender } from '@qwik.dev/core';

export const handler = $(()=>console.log('hola'));

export const App = component$((props) => {
	useStyles$('hola');
	return $(() => (
		<div>{state.thing}</div>
	));
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Import specifiers use aliases (component$ as Component, $ as onRender). Three exported values: handler, App. Uses useStyles$ which is not imported (would be a runtime reference).*

</details>

## Output

### Module: test.js

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const handler = $(()=>console.log('hola'));
export const App = component$((props)=>{
    useStyles$('hola');
    return $(()=>/*#__PURE__*/ _jsxSorted("div", null, null, state.thing, 1, "u6_0"));
});
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. The code is mostly pass-through -- $, component$, and useStyles$ calls remain as-is because the aliased imports prevent the optimizer from recognizing them as Qwik APIs. Only JSX is transpiled to _jsxSorted. Source map present.*

</details>

## Conventions Applied

- **[CONV-03] JSX Transforms**: `<div>{state.thing}</div>` transpiled to `_jsxSorted("div", null, null, state.thing, 1, "u6_0")`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `_jsxSorted` call
- **Note**: No CONV-01/02/06/08 -- the aliased imports (`component$ as Component`, `$ as onRender`) cause the optimizer to not recognize these as Qwik transformation targets. The `$()`, `component$()`, and `useStyles$()` calls pass through untransformed.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| $ | test.js | (passthrough) | 2 |
| component$ | test.js | (passthrough) | 1 |
| useStyles$ | test.js | (passthrough) | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 1 |
| console.log | test.js | (global) | 1 |

## Diagnostics

```json
[]
```
