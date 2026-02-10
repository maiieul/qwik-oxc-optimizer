# Test: example_derived_signals_multiple_children

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
			<div>First text</div>
			<div>First {`text`}</div>
			<div>First {1}</div>
			<div>First {true}</div>
			<div>First {`text${12}`}</div>
			<div>First {typeof `text${12}` === 'string' ? 12 : 43}</div>
			<div>First {signal}</div>
			<div>First {signal.value}</div>
			<div>First {12 + signal.value}</div>
			<div>First {store.address.city.name}</div>
			<div>First {store.address.city.name ? 'true' : 'false'}</div>
			<div>First {dep}</div>
			<div>First {dep.thing}</div>
			<div>First {dep.thing + 'stuff'}</div>
			<div>First {globalThing}</div>
			<div>First {globalThing.thing}</div>
			<div>First {globalThing.thing + 'stuff'}</div>
			<div>First {signal.value()}</div>
			<div>First {signal.value + unknown()}</div>
			<div>First {mutable(signal)}</div>
			<div>First {signal.value + dep}</div>
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main, Inline -- single module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
const _hf0 = (p0)=>12 + p0.value;
const _hf0_str = "12+p0.value";
const _hf1 = (p0)=>p0.address.city.name;
const _hf1_str = "p0.address.city.name";
const _hf2 = (p0)=>p0.address.city.name ? 'true' : 'false';
const _hf2_str = 'p0.address.city.name?"true":"false"';
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { useStore, mutable } from '@qwik.dev/core';
import { dep } from './file';
const App_component_ckEPmXZlub0 = ()=>{
    const signal = useSignal(0);
    const store = useStore({});
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, "First text", 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", `text`], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", 1], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", true], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", `text${12}`], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", typeof `text${12}` === 'string' ? 12 : 43], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", signal], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", _wrapProp(signal)], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", _fnSignal(_hf0, [signal], _hf0_str)], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", _fnSignal(_hf1, [store], _hf1_str)], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", _fnSignal(_hf2, [store], _hf2_str)], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", dep], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", dep.thing], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", dep.thing + 'stuff'], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", globalThing], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", globalThing.thing], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", globalThing.thing + 'stuff'], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", signal.value()], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", signal.value + unknown()], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", mutable(signal)], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, ["First ", signal.value + dep], 1, null)
    ], 1, "u6_0");
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
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; children with multiple nodes become arrays
- **[CONV-04] Signal Helpers**: `_wrapProp(signal)` and `_fnSignal()` for reactive children alongside static text
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` for computed signal expressions

**Key behavior**: When a `<div>` has both static text ("First ") and a dynamic expression as children, they become an array. The signal wrapping rules still apply to each individual child expression. The static text prefix "First " is always included as a plain string, while the dynamic part uses the same wrapping logic as single-child cases.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 22 |
| `_wrapProp` | test.js | @qwik.dev/core | 1 |
| `_fnSignal` | test.js | @qwik.dev/core | 3 |

## Diagnostics

None (`[]`)
