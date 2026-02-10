# Test: example_derived_signals_complext_children

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useStore, mutable } from '@qwik.dev/core';

import {dep} from './file';

export const App = component$(() => {
	const signal = useSignal(0);
	const store = useStore({});
	return (
		<>
			<ul id="issue-2800-result">
				{Object.entries(store).map(([key, value]) => (
				<li>
					{key} - {value}
				</li>
				))}
			</ul>
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { useStore } from '@qwik.dev/core';
const App_component_ckEPmXZlub0 = ()=>{
    useSignal(0);
    const store = useStore({});
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("ul", null, {
        id: "issue-2800-result"
    }, Object.entries(store).map(([key, value])=>/*#__PURE__*/ _jsxSorted("li", null, null, [
            key,
            " - ",
            value
        ], 1, "u6_0")), 1, null), 1, "u6_1");
};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Hoist strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; Fragment import from `@qwik.dev/core/jsx-runtime`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl`, `inlinedQrl`, all `_jsxSorted` calls

**Key behavior**: Complex children expressions like `.map()` calls are NOT wrapped in signal helpers. The map callback produces JSX elements via `_jsxSorted()`, and the overall expression remains as-is with flag=1 (mutable). Note that `useSignal(0)` is called but its return value is not used (unused signal), yet the call is preserved in the output.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 4 |

## Diagnostics

```json
[]
```
