# Test: example_derived_signals_div

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
import styles from './styles.module.css';

export const App = component$((props) => {
	const signal = useSignal(0);
	const store = useStore({});
	const count = props.counter.count;

	return (
		<div
			class={{
				even: count % 2 === 0,
				odd: count % 2 === 1,
				stable0: true,
				hidden: false,
			}}
			staticClass={styles.foo}
			staticDocument={window.document}
			staticText="text"
			staticText2={`text`}
			staticNumber={1}
			staticBoolean={true}
			staticExpr={`text${12}`}
			staticExpr2={typeof `text${12}` === 'string' ? 12 : 43}
			signal={signal}
			signalValue={signal.value}
			signalComputedValue={12 + signal.value}
			store={store.address.city.name}
			storeComputed={store.address.city.name ? 'true' : 'false'}
			dep={dep}
			depAccess={dep.thing}
			depComputed={dep.thing + 'stuff'}
			global={globalThing}
			globalAccess={globalThing.thing}
			globalComputed={globalThing.thing + 'stuff'}
			noInline={signal.value()}
			noInline2={signal.value + unknown()}
			noInline3={mutable(signal)}
			noInline4={signal.value + dep}
		/>
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
import { _fnSignal } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
const _hf0 = (p0)=>12 + p0.value;
const _hf0_str = "12+p0.value";
const _hf1 = (p0)=>p0.address.city.name;
const _hf1_str = "p0.address.city.name";
const _hf2 = (p0)=>p0.address.city.name ? 'true' : 'false';
const _hf2_str = 'p0.address.city.name?"true":"false"';
import { useStore, mutable } from '@qwik.dev/core';
import { dep } from './file';
import styles from './styles.module.css';
const App_component_ckEPmXZlub0 = (props)=>{
    const signal = useSignal(0);
    const store = useStore({});
    const count = props.counter.count;
    return /*#__PURE__*/ _jsxSorted("div", {
        class: { even: count % 2 === 0, odd: count % 2 === 1, stable0: true, hidden: false },
        global: globalThing,
        globalAccess: globalThing.thing,
        globalComputed: globalThing.thing + 'stuff',
        noInline: signal.value(),
        noInline2: signal.value + unknown(),
        noInline3: mutable(signal),
        noInline4: signal.value + dep,
        staticDocument: window.document
    }, {
        staticClass: styles.foo,
        staticText: "text",
        staticText2: `text`,
        staticNumber: 1,
        staticBoolean: true,
        staticExpr: `text${12}`,
        staticExpr2: typeof `text${12}` === 'string' ? 12 : 43,
        signal: signal,
        signalValue: _wrapProp(signal),
        signalComputedValue: _fnSignal(_hf0, [signal], _hf0_str),
        store: _fnSignal(_hf1, [store], _hf1_str),
        storeComputed: _fnSignal(_hf2, [store], _hf2_str),
        dep: dep,
        depAccess: dep.thing,
        depComputed: dep.thing + 'stuff'
    }, null, 3, "u6_0");
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
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", varProps, constProps, children, flags, key)` -- props split into var/const objects for native `<div>` element
- **[CONV-04] Signal Helpers**: `_wrapProp(signal)` for `.value` access, `_fnSignal()` for computed expressions
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` with string representations for signal computations

**Key behavior**: For native HTML elements (like `<div>`), props are split into two categories just like for components. The `class` prop with a dynamic object expression (using `count` from `props.counter.count`) goes into the var props object since `count` is derived from component props. CSS module access (`styles.foo`) is correctly categorized as a constant prop. `window.document` goes in var props because it is a global member access.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.js | @qwik.dev/core | 1 |
| `_fnSignal` | test.js | @qwik.dev/core | 3 |

## Diagnostics

```json
[]
```
