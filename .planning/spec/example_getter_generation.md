# Test: example_getter_generation

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$(() => {
	const store = useStore({
		count: 0,
		stuff: 0,
		nested: {
			count: 0
		}
	});
	const signal = useSignal(0);
	return (
		<Cmp
			prop={'true' + 1 ? 'true' : ''}
			count={store.count}
			nested={store.nested.count}
			signal={signal}
			store={store.stuff + 12}
			value={signal.formData?.get('username')}
		>
		</Cmp>
	);
});

export const Cmp = component$((props) => {
	return (
		<>
			<p data-value={props.count}>{props.nested.count}</p>
			<p>Value {props.count}<span></span></p>
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
import { qrl } from "@qwik.dev/core";
const i_4ryKJTOKjWE = ()=>import("./test.tsx_Cmp_component_4ryKJTOKjWE");
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
export const Cmp = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_4ryKJTOKjWE, "Cmp_component_4ryKJTOKjWE"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

### Module: test.tsx_Cmp_component_4ryKJTOKjWE.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.nested.count;
const _hf0_str = "p0.nested.count";
export const Cmp_component_4ryKJTOKjWE = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("p", {
            "data-value": _wrapProp(props, "count")
        }, null, _fnSignal(_hf0, [props], _hf0_str), 1, null),
        /*#__PURE__*/ _jsxSorted("p", null, null, [
            "Value ",
            _wrapProp(props, "count"),
            /*#__PURE__*/ _jsxSorted("span", null, null, null, 3, null)
        ], 1, null)
    ], 1, "u6_1");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_4ryKJTOKjWE",
  "entry": null,
  "displayName": "test.tsx_Cmp_component",
  "hash": "4ryKJTOKjWE",
  "canonicalFilename": "test.tsx_Cmp_component_4ryKJTOKjWE",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [458, 596],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { Cmp } from "./test";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.nested.count;
const _hf0_str = "p0.nested.count";
const _hf1 = (p0)=>p0.stuff + 12;
const _hf1_str = "p0.stuff+12";
const _hf2 = (p0)=>p0.formData?.get('username');
const _hf2_str = 'p0.formData?.get("username")';
export const App_component_ckEPmXZlub0 = ()=>{
    const store = useStore({
        count: 0,
        stuff: 0,
        nested: { count: 0 }
    });
    const signal = useSignal(0);
    return /*#__PURE__*/ _jsxSorted(Cmp, null, {
        prop: 'true',
        count: _wrapProp(store, "count"),
        nested: _fnSignal(_hf0, [store], _hf0_str),
        signal: signal,
        store: _fnSignal(_hf1, [store], _hf1_str),
        value: _fnSignal(_hf2, [signal], _hf2_str)
    }, null, 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [88, 424]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy (separate modules)
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl` for both `App` and `Cmp`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements
- **[CONV-04] Signal Helpers**: Extensive use:
  - `_wrapProp(store, "count")` for direct store property access
  - `_wrapProp(props, "count")` for prop access in receiving component
  - `_fnSignal(_hf0, [store], "p0.nested.count")` for nested property access
  - `_fnSignal(_hf1, [store], "p0.stuff+12")` for computed store expressions
  - `_fnSignal(_hf2, [signal], 'p0.formData?.get("username")')` for optional chaining with method calls
- **[CONV-06] Lazy Imports**: Lazy imports for both component segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-08] Segment Extraction**: Both `App` and `Cmp` component bodies extracted
- **[CONV-14] Hoisted Functions**: Multiple `_hf` functions with string representations; note `_hf0` appears in both components (for `nested.count` access), `_hf2` handles optional chaining

**Key behavior**: The `prop` attribute with expression `'true' + 1 ? 'true' : ''` is statically evaluated to `'true'` at compile time. The `_fnSignal` string representation for optional chaining (`p0.formData?.get("username")`) preserves the `?.` syntax. Both the sending component (`App`) and receiving component (`Cmp`) generate signal wrappers -- the sender wraps store props, the receiver wraps its incoming props.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 2 |
| `qrl` | test.js | @qwik.dev/core | 2 |
| `_jsxSorted` | App segment, Cmp segment | @qwik.dev/core | 5 |
| `_wrapProp` | App segment, Cmp segment | @qwik.dev/core | 4 |
| `_fnSignal` | App segment, Cmp segment | @qwik.dev/core | 4 |

## Diagnostics

```json
[]
```
