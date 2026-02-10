# Test: example_immutable_analysis

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useStore, $ } from '@qwik.dev/core';
import importedValue from 'v';
import styles from './styles.module.css';

export const App = component$((props) => {
	const {Model} = props;
	const state = useStore({count: 0});
	const remove = $((id: number) => {
		const d = state.data;
		d.splice(d.findIndex((d) => d.id === id), 1)
	});
	return (
		<>
			<p class="stuff" onClick$={props.onClick$}>Hello Qwik</p>
			<Div
				class={styles.foo}
				document={window.document}
				onClick$={props.onClick$}
				onEvent$={() => console.log('stuff')}
				transparent$={() => {console.log('stuff')}}
				immutable1="stuff"
				immutable2={{ foo: 'bar', baz: importedValue ? true : false }}
				immutable3={2}
				immutable4$={(ev) => console.log(state.count)}
				immutable5={[1, 2, importedValue, null, {}]}
			>
				<p>Hello Qwik</p>
			</Div>
			[].map(() => (
				<Model
					class={state}
					remove$={remove}
					mutable1={{ foo: 'bar', baz: state.count ? true : false }}
					mutable2={(() => console.log(state.count))()}
					mutable3={[1, 2, state, null, {}]}
				/>
			));
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.tsx_App_component_Fragment_Div_onEvent_zrFduYbT3xM.js (ENTRY POINT)

```javascript
export const App_component_Fragment_Div_onEvent_zrFduYbT3xM = ()=>console.log('stuff');
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_Fragment_Div_onEvent_zrFduYbT3xM",
  "entry": null, "displayName": "test.tsx_App_component_Fragment_Div_onEvent",
  "hash": "zrFduYbT3xM", "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "jSXProp", "ctxName": "onEvent$",
  "captures": false, "loc": [543, 569]
}
```

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

### Module: test.tsx_App_component_Fragment_Div_immutable4_2zF7jA3Yti0.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_Fragment_Div_immutable4_2zF7jA3Yti0 = (ev)=>{
    const state = _captures[0];
    return console.log(state.count);
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_Fragment_Div_immutable4_2zF7jA3Yti0",
  "entry": null, "displayName": "test.tsx_App_component_Fragment_Div_immutable4",
  "hash": "2zF7jA3Yti0", "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "jSXProp", "ctxName": "immutable4$",
  "captures": true, "loc": [760, 792],
  "paramNames": ["ev"], "captureNames": ["state"]
}
```

### Module: test.tsx_App_component_Fragment_Div_transparent_eeDEK6EM1oo.js (ENTRY POINT)

```javascript
export const App_component_Fragment_Div_transparent_eeDEK6EM1oo = ()=>{
    console.log('stuff');
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_Fragment_Div_transparent_eeDEK6EM1oo",
  "entry": null, "displayName": "test.tsx_App_component_Fragment_Div_transparent",
  "hash": "eeDEK6EM1oo", "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "jSXProp", "ctxName": "transparent$",
  "captures": false, "loc": [589, 617]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import importedValue from "v";
import { qrl } from "@qwik.dev/core";
import styles from "./styles.module.css";
import { useStore } from "@qwik.dev/core";
const _hf0 = (p0)=>({ foo: 'bar', baz: p0.count ? true : false });
const _hf0_str = '{foo:"bar",baz:p0.count?true:false}';
const i_2zF7jA3Yti0 = ()=>import("./test.tsx_App_component_Fragment_Div_immutable4_2zF7jA3Yti0");
const i_eeDEK6EM1oo = ()=>import("./test.tsx_App_component_Fragment_Div_transparent_eeDEK6EM1oo");
const i_pU6yOC5P6sY = ()=>import("./test.tsx_App_component_remove_pU6yOC5P6sY");
const i_zrFduYbT3xM = ()=>import("./test.tsx_App_component_Fragment_Div_onEvent_zrFduYbT3xM");
export const App_component_ckEPmXZlub0 = (props)=>{
    const state = useStore({ count: 0 });
    const remove = /*#__PURE__*/ qrl(i_pU6yOC5P6sY, "App_component_remove_pU6yOC5P6sY", [state]);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("p", { "q-e:click": props.onClick$ }, { class: "stuff" }, "Hello Qwik", 2, null),
        /*#__PURE__*/ _jsxSorted(Div, {
            document: window.document, onClick$: props.onClick$
        }, {
            class: styles.foo,
            onEvent$: /*#__PURE__*/ qrl(i_zrFduYbT3xM, "App_component_Fragment_Div_onEvent_zrFduYbT3xM"),
            transparent$: /*#__PURE__*/ qrl(i_eeDEK6EM1oo, "App_component_Fragment_Div_transparent_eeDEK6EM1oo"),
            immutable1: "stuff",
            immutable2: { foo: 'bar', baz: importedValue ? true : false },
            immutable3: 2,
            immutable4$: /*#__PURE__*/ qrl(i_2zF7jA3Yti0, "App_component_Fragment_Div_immutable4_2zF7jA3Yti0", [state]),
            immutable5: [1, 2, importedValue, null, {}]
        }, /*#__PURE__*/ _jsxSorted("p", null, null, "Hello Qwik", 3, null), 2, "u6_0"),
        "[].map(() => (",
        /*#__PURE__*/ _jsxSorted(props.Model, {
            mutable2: (()=>console.log(state.count))()
        }, {
            class: state,
            remove$: remove,
            mutable1: _fnSignal(_hf0, [state], _hf0_str),
            mutable3: [1, 2, state, null, {}]
        }, null, 3, "u6_1"),
        "));"
    ], 1, "u6_2");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null, "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [164, 1148],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_remove_pU6yOC5P6sY.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_remove_pU6yOC5P6sY = (id)=>{
    const state = _captures[0];
    const d = state.data;
    d.splice(d.findIndex((d)=>d.id === id), 1);
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_remove_pU6yOC5P6sY",
  "entry": null, "displayName": "test.tsx_App_component_remove",
  "hash": "pU6yOC5P6sY", "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function", "ctxName": "$",
  "captures": true, "loc": [256, 358],
  "paramNames": ["id"], "captureNames": ["state"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for all extracted segments; `qrl()` with captures array `[state]`
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `$()` to `qrl()`; JSX `$`-props to QRL props
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; native element `<p>` has `onClick$` transformed to `q-e:click`
- **[CONV-04] Signal Helpers**: `_fnSignal()` used for `mutable1` prop containing `state.count` (object with store access)
- **[CONV-05] Capture Patterns**: `_captures[0]` used in `immutable4` and `remove` segments to restore captured `state` variable
- **[CONV-06] Lazy Imports**: Lazy import for every extracted segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all `qrl()`, `componentQrl()`, `_jsxSorted()` calls
- **[CONV-08] Segment Extraction**: 5 segments extracted: component body, remove handler, onEvent, transparent, immutable4
- **[CONV-14] Hoisted Functions**: `_hf0` for object with `state.count` conditional

**Key behavior**: This test is a comprehensive immutability analysis. Props are categorized as immutable or mutable based on their dependencies:
- **Immutable** (const props): `class: styles.foo`, `immutable1: "stuff"`, `immutable2: {foo: 'bar', baz: importedValue...}`, `immutable3: 2`, `immutable5: [...]` -- only depend on imports and literals
- **Mutable** (var props): `document: window.document`, `onClick$: props.onClick$` -- depend on globals or forwarded props
- The `Model` component is accessed from `props` via destructuring, so `<Model>` becomes `props.Model` in the output
- The `mutable1` prop uses `_fnSignal` because the object contains `state.count` (a store access)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component segment | @qwik.dev/core | 5 |
| `_jsxSorted` | component segment | @qwik.dev/core | 5 |
| `_fnSignal` | component segment | @qwik.dev/core | 1 |
| `_captures` | immutable4, remove segments | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
