# Test: example_props_wrapping_children

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
		<div>
			{fromLocal}
			{fromProps}
			{fromLocal + fromProps}
			{{props: fromProps}}
			{{local: fromLocal}}
			{{props: fromProps, local: fromLocal}}
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (single file, Inline strategy)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
const _hf0 = (p0, p1)=>p1 + p0.fromProps;
const _hf0_str = "p1+p0.fromProps";
const _hf1 = (p0)=>({
        props: p0.fromProps
    });
const _hf1_str = "{props:p0.fromProps}";
const _hf2 = (p0, p1)=>({
        props: p0.fromProps,
        local: p1
    });
const _hf2_str = "{props:p0.fromProps,local:p1}";
import { useSignal } from '@qwik.dev/core';
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((_rawProps)=>{
    let fromLocal = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        fromLocal,
        _wrapProp(_rawProps, "fromProps"),
        _fnSignal(_hf0, [
            _rawProps,
            fromLocal
        ], _hf0_str),
        _fnSignal(_hf1, [
            _rawProps
        ], _hf1_str),
        {
            local: fromLocal
        },
        _fnSignal(_hf2, [
            _rawProps,
            fromLocal
        ], _hf2_str)
    ], 1, "u6_0");
}, "Works_component_t45qL4vNGv0"));
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Inline strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<div>` with children array
- **[CONV-04] Signal Helpers**: `_wrapProp(_rawProps, "fromProps")` for direct prop child; `_fnSignal` for mixed expressions; `{local: fromLocal}` is NOT wrapped (only local signal, no prop access)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-11] Props Destructuring**: `({fromProps})` is destructured, so the parameter is renamed to `_rawProps` and prop access becomes `_rawProps.fromProps`
- **[CONV-14] Hoisted Functions**: Three hoisted functions: `_hf0` (addition), `_hf1` (object with prop), `_hf2` (object with prop + local)

**Key behavior**: When component props are destructured (`{fromProps}`), the optimizer renames the parameter to `_rawProps` and converts all prop references to `_rawProps.fromProps`. In children arrays:
1. `{fromLocal}` -- local signal, passed directly (no wrapping needed, runtime handles signal children)
2. `{fromProps}` -- destructured prop, wrapped with `_wrapProp(_rawProps, "fromProps")`
3. `{fromLocal + fromProps}` -- mixed expression, wrapped with `_fnSignal(_hf0, [_rawProps, fromLocal], ...)`, hoisted function parameter order: `_rawProps` first (p0), `fromLocal` second (p1)
4. `{{props: fromProps}}` -- object with only prop, wrapped with `_fnSignal(_hf1, [_rawProps], ...)`
5. `{{local: fromLocal}}` -- object with only local signal, NOT wrapped (passed as plain object literal `{local: fromLocal}`)
6. `{{props: fromProps, local: fromLocal}}` -- mixed object, wrapped with `_fnSignal(_hf2, [_rawProps, fromLocal], ...)`

The key distinction: objects containing ONLY local signals are not wrapped, while objects containing prop references (or mixed) are wrapped with `_fnSignal`.

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
