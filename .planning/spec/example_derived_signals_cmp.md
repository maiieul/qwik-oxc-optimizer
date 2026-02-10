# Test: example_derived_signals_cmp

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
import {Cmp} from './cmp';

export const App = component$(() => {
	const signal = useSignal(0);
	const store = useStore({});
	return (
		<Cmp
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
import { _wrapProp } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
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
import { Cmp } from './cmp';
const App_component_ckEPmXZlub0 = ()=>{
    const signal = useSignal(0);
    const store = useStore({});
    return /*#__PURE__*/ _jsxSorted(Cmp, {
        global: globalThing,
        globalAccess: globalThing.thing,
        globalComputed: globalThing.thing + 'stuff',
        noInline: signal.value(),
        noInline2: signal.value + unknown(),
        noInline3: mutable(signal),
        noInline4: signal.value + dep
    }, {
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

- **[CONV-01] QRL Calls**: `inlinedQrl()` for inline strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with component (`Cmp`) as first argument; props split into var (1st object) and const (2nd object) arguments
- **[CONV-04] Signal Helpers**: `_wrapProp(signal)`, `_fnSignal(_hf0, [signal], _hf0_str)`, `_fnSignal(_hf1, [store], _hf1_str)` for reactive prop values
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl`, `inlinedQrl`, `_jsxSorted`
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` with string representations for computed signal expressions

**Key behavior**: When passing props to a **component** (vs a native element), `_jsxSorted` uses two separate prop objects: the first for mutable/variable props (globals, function calls, mixed expressions), the second for constant/trackable props (static values, signals, store accesses, imported deps). This separation enables the component to optimize re-rendering by distinguishing props that may change at any time from those that are reactive or immutable.

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
