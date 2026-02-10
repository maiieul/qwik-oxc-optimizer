# Test: should_not_wrap_fn

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useSignal } from "@qwik.dev/core";
import { A } from "./componentA";

export const Cmp = component$(() => {
	const currentStep = useSignal('STEP_1');
	const currentType = useSignal<'NEXT' | 'PREVIOUS'>('PREVIOUS');

	const getStep = (step: string, type: 'NEXT' | 'PREVIOUS') => {
		return step === 'STEP_1' ? 'STEP_2' : 'STEP_1';
	};

	return (
		<>
			<button onClick$={() => (currentType.value = 'NEXT')}>CLICK</button>
			<A href={getStep(currentStep.value, currentType.value)} />
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
export const Cmp = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_4ryKJTOKjWE, "Cmp_component_4ryKJTOKjWE"));
```

### Module: test.tsx_Cmp_component_4ryKJTOKjWE.js (ENTRY POINT)

```javascript
import { A } from "./componentA";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_veAZ2ow0cnM = ()=>import("./test.tsx_Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM");
export const Cmp_component_4ryKJTOKjWE = ()=>{
    const currentStep = useSignal('STEP_1');
    const currentType = useSignal('PREVIOUS');
    const getStep = (step, type)=>{
        return step === 'STEP_1' ? 'STEP_2' : 'STEP_1';
    };
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_veAZ2ow0cnM, "Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM", [currentType])
        }, "CLICK", 3, null),
        /*#__PURE__*/ _jsxSorted(A, {
            href: getStep(currentStep.value, currentType.value)
        }, null, null, 3, "u6_0")
    ], 1, "u6_1");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_4ryKJTOKjWE",
  "entry": null, "displayName": "test.tsx_Cmp_component",
  "hash": "4ryKJTOKjWE", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [123, 518]
}
```

### Module: test.tsx_Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM = ()=>{
    const currentType = _captures[0];
    return currentType.value = 'NEXT';
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM",
  "entry": null, "displayName": "test.tsx_Cmp_component_Fragment_button_q_e_click",
  "hash": "veAZ2ow0cnM", "extension": "js",
  "parent": "Cmp_component_4ryKJTOKjWE",
  "ctxKind": "eventHandler", "ctxName": "onClick$",
  "captures": true, "loc": [394, 428],
  "captureNames": ["currentType"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy with captures `[currentType]`
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `onClick$` to `q-e:click`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements
- **[CONV-05] Capture Patterns**: `_captures[0]` restores `currentType` in click handler
- **[CONV-06] Lazy Imports**: Lazy imports for component and click handler segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-08] Segment Extraction**: Component body and click handler extracted

**Key behavior**: The `href` prop value `getStep(currentStep.value, currentType.value)` is a function call -- the optimizer does NOT wrap it with `_fnSignal` or `_wrapProp`. Function call results are not trackable by the signal system because the optimizer cannot determine the function's behavior. The `href` prop goes in the var props (first argument to `_jsxSorted`) since it involves calling a local function with signal values. TypeScript type annotations (`'NEXT' | 'PREVIOUS'`, `step: string`) are stripped.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component segment | @qwik.dev/core | 2 |
| `_jsxSorted` | component segment | @qwik.dev/core | 3 |
| `_captures` | click handler | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
