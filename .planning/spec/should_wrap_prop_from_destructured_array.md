# Test: should_wrap_prop_from_destructured_array

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useStore, useTask$ } from '@qwik.dev/core';
import { useForm, useForm2 } from './some-file.ts';

export const Input = component$<{error: string, error2: string, error3: string}>(
	(props) => {
		useTask$(({ track }) => {
			track(() => props.error);
			track(() => props.error2);
			track(() => props.error3);
		});
		return (<></>);
	}
);

export default component$(() => {
	const [store, math] = [useStore({errors: {}}), Math.random()];
	const [[store2]] = [[useStore({errors: {}})]];
	const { store3, math4 } = { store3: useStore({errors: {}}), math4: Math.random() };
	const math2 = [Math.random()];
	const { math3 } = { math3: Math.random() };
	const [store4] = useForm();
	const {store5} = useForm2();

	return (
		<div>
			<button onClick$={() => {
				store.errors.test = store.errors.test ? undefined : 'ERROR TEST';
			}}>click</button>
			<Input
				error={store.errors.test}
				error2={store2.errors.test}
				error3={store3.errors.test}
				error4={store4.errors.test}
				error5={store5.errors.test}
				math={math}
				math2={math2}
				math3={math3}
				math4={math4}
			/>
		</div>
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
import { qrl } from "@qwik.dev/core";
const i_9metqqaxlN8 = ()=>import("./test.tsx_Input_component_9metqqaxlN8");
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export const Input = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_9metqqaxlN8, "Input_component_9metqqaxlN8"));
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

### Module: test.tsx_test_component_div_button_q_e_click_2TvarUvNGmU.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const test_component_div_button_q_e_click_2TvarUvNGmU = ()=>{
    const store = _captures[0];
    store.errors.test = store.errors.test ? undefined : 'ERROR TEST';
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_div_button_q_e_click_2TvarUvNGmU",
  "entry": null, "displayName": "test.tsx_test_component_div_button_q_e_click",
  "hash": "2TvarUvNGmU", "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler", "ctxName": "onClick$",
  "captures": true, "loc": [835, 921],
  "captureNames": ["store"]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { Input } from "./test";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useForm } from "./some-file.ts";
import { useForm2 } from "./some-file.ts";
import { useStore } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.errors.test;
const _hf0_str = "p0.errors.test";
const _hf1 = (p0)=>p0.store5.errors.test;
const _hf1_str = "p0.store5.errors.test";
const i_2TvarUvNGmU = ()=>import("./test.tsx_test_component_div_button_q_e_click_2TvarUvNGmU");
export const test_component_LUXeXe0DQrg = ()=>{
    const [store, math] = [useStore({ errors: {} }), Math.random()];
    const [[store2]] = [[useStore({ errors: {} })]];
    const { store3, math4 } = { store3: useStore({ errors: {} }), math4: Math.random() };
    const math2 = [Math.random()];
    const { math3 } = { math3: Math.random() };
    const [store4] = useForm();
    const form2 = useForm2();
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_2TvarUvNGmU, "test_component_div_button_q_e_click_2TvarUvNGmU", [store])
        }, "click", 3, null),
        /*#__PURE__*/ _jsxSorted(Input, {
            math: math, math2: math2, math3: math3, math4: math4
        }, {
            error: _fnSignal(_hf0, [store], _hf0_str),
            error2: _fnSignal(_hf0, [store2], _hf0_str),
            error3: _fnSignal(_hf0, [store3], _hf0_str),
            error4: _fnSignal(_hf0, [store4], _hf0_str),
            error5: _fnSignal(_hf1, [form2], _hf1_str)
        }, null, 3, "u6_1")
    ], 1, "u6_2");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null, "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [434, 1225]
}
```

### Module: test.tsx_Input_component_useTask_Sbgs9Wtfkt0.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Input_component_useTask_Sbgs9Wtfkt0 = ({ track })=>{
    const props = _captures[0];
    track(()=>props.error);
    track(()=>props.error2);
    track(()=>props.error3);
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Input_component_useTask_Sbgs9Wtfkt0",
  "entry": null, "displayName": "test.tsx_Input_component_useTask",
  "hash": "Sbgs9Wtfkt0", "extension": "js",
  "parent": "Input_component_9metqqaxlN8",
  "ctxKind": "function", "ctxName": "useTask$",
  "captures": true, "loc": [237, 354],
  "paramNames": ["{track}"], "captureNames": ["props"]
}
```

### Module: test.tsx_Input_component_9metqqaxlN8.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const i_Sbgs9Wtfkt0 = ()=>import("./test.tsx_Input_component_useTask_Sbgs9Wtfkt0");
export const Input_component_9metqqaxlN8 = (props)=>{
    useTaskQrl(/*#__PURE__*/ qrl(i_Sbgs9Wtfkt0, "Input_component_useTask_Sbgs9Wtfkt0", [props]));
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, null, 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Input_component_9metqqaxlN8",
  "entry": null, "displayName": "test.tsx_Input_component",
  "hash": "9metqqaxlN8", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [211, 399],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for all segments; `useTaskQrl()` wraps task callback
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `useTask$` to `useTaskQrl`; `onClick$` to `q-e:click`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; `Input` component props split into var/const
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [store], _hf0_str)` reused for multiple stores with the same access pattern `.errors.test`; `_fnSignal(_hf1, [form2], _hf1_str)` for `store5.errors.test` (different accessor because `store5` is a destructured property)
- **[CONV-05] Capture Patterns**: `_captures[0]` in click handler (captures `store`) and useTask handler (captures `props`)
- **[CONV-06] Lazy Imports**: Lazy imports for all segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: 5 segments total

**Key behavior**: The optimizer tracks store origins through various destructuring patterns:
- `const [store, math] = [useStore(...), ...]` -- `store` recognized as store
- `const [[store2]] = [[useStore(...)]]` -- deeply nested array destructuring, `store2` still recognized
- `const { store3, math4 } = { store3: useStore(...), ... }` -- object destructuring, `store3` recognized
- `const [store4] = useForm()` -- from function return, `store4` recognized (function call return is tracked)
- `const {store5} = useForm2()` -- **renamed to `form2`** in output. The destructuring `{store5}` from `useForm2()` is transformed to `const form2 = useForm2()` and the access path becomes `form2.store5.errors.test` (via `_hf1`)
- `math`, `math2`, `math3`, `math4` are non-reactive (from `Math.random()`), placed in var props

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 2 |
| `qrl` | test.js, segments | @qwik.dev/core | 4 |
| `useTaskQrl` | Input segment | @qwik.dev/core | 1 |
| `_jsxSorted` | component segment | @qwik.dev/core | 4 |
| `_fnSignal` | component segment | @qwik.dev/core | 5 |
| `_captures` | click, useTask handlers | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
