# Test: example_props_wrapping2

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { $, component$, useSignal } from '@qwik.dev/core';
export const Works = component$((props: { fromProps: number }) => {
	let fromLocal = useSignal(0);
	return (
		<div
			computed={fromLocal + props.fromProps}
			local={fromLocal}
			props-wrap={props.fromProps}
			props-only={{props: props.fromProps}}
			props={{props: props.fromProps, local: fromLocal}}
		>
		</div>
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
import { _fnSignal } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
const _hf0 = (p0, p1)=>p0 + p1.fromProps;
const _hf0_str = "p0+p1.fromProps";
const _hf1 = (p0)=>({ props: p0.fromProps });
const _hf1_str = "{props:p0.fromProps}";
const _hf2 = (p0, p1)=>({ props: p1.fromProps, local: p0 });
const _hf2_str = "{props:p1.fromProps,local:p0}";
import { useSignal } from '@qwik.dev/core';
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    let fromLocal = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", {
        computed: _fnSignal(_hf0, [fromLocal, props], _hf0_str),
        local: fromLocal,
        props: _fnSignal(_hf2, [fromLocal, props], _hf2_str),
        "props-only": _fnSignal(_hf1, [props], _hf1_str),
        "props-wrap": _wrapProp(props, "fromProps")
    }, null, null, 3, "u6_0");
}, "Works_component_t45qL4vNGv0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Inline strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<div>` element
- **[CONV-04] Signal Helpers**: `_wrapProp(props, "fromProps")` for `props.fromProps`; `_fnSignal()` for computed expressions involving `props.fromProps`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` -- note parameter order differs from `example_props_wrapping` because props is NOT destructured (no `_rawProps` transform)

**Key behavior**: Contrast with `example_props_wrapping`: when props are NOT destructured (accessed via `props.fromProps` instead of `({fromProps})`), the optimizer does NOT rename the parameter to `_rawProps`. Instead, `props` is kept as-is and used directly in `_wrapProp(props, "fromProps")`. The hoisted function parameter order also differs: `_hf0 = (p0, p1)=>p0 + p1.fromProps` (where p0=fromLocal, p1=props), compared to `_hf0 = (p0, p1)=>p1 + p0.fromProps` (where p0=_rawProps, p1=fromLocal) in the destructured version.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.js | @qwik.dev/core | 1 |
| `_fnSignal` | test.js | @qwik.dev/core | 3 |

## Diagnostics

None (`[]`)
