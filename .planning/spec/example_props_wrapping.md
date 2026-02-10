# Test: example_props_wrapping

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
export const Works = component$(({fromProps}) => {
	let fromLocal = useSignal(0);
	return (
		<div
			computed={fromLocal + fromProps}
			local={fromLocal}
			props-wrap={fromProps}
			props-only={{props: fromProps}}
			props={{props: fromProps, local: fromLocal}}
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
const _hf0 = (p0, p1)=>p1 + p0.fromProps;
const _hf0_str = "p1+p0.fromProps";
const _hf1 = (p0)=>({ props: p0.fromProps });
const _hf1_str = "{props:p0.fromProps}";
const _hf2 = (p0, p1)=>({ props: p0.fromProps, local: p1 });
const _hf2_str = "{props:p0.fromProps,local:p1}";
import { useSignal } from '@qwik.dev/core';
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((_rawProps)=>{
    let fromLocal = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", {
        computed: _fnSignal(_hf0, [_rawProps, fromLocal], _hf0_str),
        local: fromLocal,
        props: _fnSignal(_hf2, [_rawProps, fromLocal], _hf2_str),
        "props-only": _fnSignal(_hf1, [_rawProps], _hf1_str),
        "props-wrap": _wrapProp(_rawProps, "fromProps")
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
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with var props object (first arg), null const props (second arg)
- **[CONV-04] Signal Helpers**:
  - `_wrapProp(_rawProps, "fromProps")` for direct prop access
  - `_fnSignal(_hf0, [_rawProps, fromLocal], ...)` for `fromLocal + fromProps` (mixed local+prop)
  - `_fnSignal(_hf1, [_rawProps], ...)` for `{props: fromProps}` (object with prop)
  - `_fnSignal(_hf2, [_rawProps, fromLocal], ...)` for `{props: fromProps, local: fromLocal}` (mixed)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-11] Props Destructuring**: `({fromProps})` replaced with `(_rawProps)` for prop tracking
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1`, `_hf2` for computed signal expressions with multiple parameters

**Key behavior**: Demonstrates prop wrapping for different expression types:
- Direct prop access (`fromProps`) -> `_wrapProp(_rawProps, "fromProps")`
- Local signal (`fromLocal`) -> passed directly (signal object)
- Mixed expression (`fromLocal + fromProps`) -> `_fnSignal` with both dependencies
- Object with prop (`{props: fromProps}`) -> `_fnSignal` with _rawProps
- Object with both -> `_fnSignal` with both dependencies

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
