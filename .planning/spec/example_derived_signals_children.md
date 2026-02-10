# Test: example_derived_signals_children

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

export const TextContent = component$((props) => {
	return (
		<>
			<div>data-nu: {props['data-nu']}</div>
			<div>class: {props.class}</div>
		</>
	);
});

export const App = component$(() => {
	const signal = useSignal(0);
	const store = useStore({});
	return (
		<>
			<div>text</div>
			<div>{`text`}</div>
			<div>{1}</div>
			<div>{true}</div>
			<div>{`text${12}`}</div>
			<div>{typeof `text${12}` === 'string' ? 12 : 43}</div>
			<div>{signal}</div>
			<div>{signal.value}</div>
			<div>{12 + signal.value}</div>
			<div>{store.address.city.name}</div>
			<div>{store.address.city.name ? 'true' : 'false'}</div>
			<div>{dep}</div>
			<div>{dep.thing}</div>
			<div>{dep.thing + 'stuff'}</div>
			<div>{globalThing}</div>
			<div>{globalThing.thing}</div>
			<div>{globalThing.thing + 'stuff'}</div>
			<div>{signal.value()}</div>
			<div>{signal.value + unknown()}</div>
			<div>{mutable(signal)}</div>
			<div>{signal.value + dep}</div>
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
import { _wrapProp } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>12 + p0.value;
const _hf0_str = "12+p0.value";
const _hf1 = (p0)=>p0.address.city.name;
const _hf1_str = "p0.address.city.name";
const _hf2 = (p0)=>p0.address.city.name ? 'true' : 'false';
const _hf2_str = 'p0.address.city.name?"true":"false"';
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { useStore, mutable } from '@qwik.dev/core';
import { dep } from './file';
const TextContent_component_puSwpKXO7Kg = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "data-nu: ",
            _wrapProp(props, "data-nu")
        ], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "class: ",
            _wrapProp(props, "class")
        ], 1, null)
    ], 1, "u6_0");
};
export const TextContent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(TextContent_component_puSwpKXO7Kg, "TextContent_component_puSwpKXO7Kg"));
const App_component_ckEPmXZlub0 = ()=>{
    const signal = useSignal(0);
    const store = useStore({});
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, "text", 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, `text`, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, 1, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, true, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, `text${12}`, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, typeof `text${12}` === 'string' ? 12 : 43, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, signal, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(signal), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, _fnSignal(_hf0, [signal], _hf0_str), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, _fnSignal(_hf1, [store], _hf1_str), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, _fnSignal(_hf2, [store], _hf2_str), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, dep, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, dep.thing, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, dep.thing + 'stuff', 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, globalThing, 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, globalThing.thing, 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, globalThing.thing + 'stuff', 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, signal.value(), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, signal.value + unknown(), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, mutable(signal), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, signal.value + dep, 1, null)
    ], 1, "u6_1");
};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` used (Hoist entry strategy inlines QRLs)
- **[CONV-02] Dollar-to-QRL**: `component$` transformed to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` used for all JSX elements with transpiled JSX
- **[CONV-04] Signal Helpers**: Heavy signal helper usage:
  - `_wrapProp(signal)` for `signal.value` (single-arg form for signals used as children)
  - `_wrapProp(props, "data-nu")` for `props['data-nu']` (two-arg form for named prop access)
  - `_fnSignal(_hf0, [signal], _hf0_str)` for computed expressions like `12 + signal.value`
  - `_fnSignal(_hf1, [store], _hf1_str)` for store access paths like `store.address.city.name`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()`, `inlinedQrl()`, and `_jsxSorted()` calls
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` with corresponding `_hf0_str`, `_hf1_str`, `_hf2_str` for derived signal computations

**Key behavior**: This test demonstrates the signal wrapping decision tree for children:
- Static values (text, numbers, booleans, template literals) are passed directly (flag=3 for immutable)
- `signal` object passed directly (the framework knows how to subscribe)
- `signal.value` becomes `_wrapProp(signal)` (subscribe to .value)
- `12 + signal.value` becomes `_fnSignal()` with a hoisted function
- `store.address.city.name` becomes `_fnSignal()` with a hoisted accessor
- `dep` (imported module) passed directly (not reactive)
- `globalThing` passed directly but with flag=1 (mutable, non-trackable)
- Function calls and mixed expressions are NOT wrapped (flag=1)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 2 |
| `inlinedQrl` | test.js | @qwik.dev/core | 2 |
| `_jsxSorted` | test.js | @qwik.dev/core | 24 |
| `_wrapProp` | test.js | @qwik.dev/core | 3 |
| `_fnSignal` | test.js | @qwik.dev/core | 3 |

## Diagnostics

```json
[]
```
