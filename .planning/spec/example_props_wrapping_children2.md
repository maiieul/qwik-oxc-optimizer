# Test: example_props_wrapping_children2

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
export const Works = component$((props) => {
	let fromLocal = useSignal(0);
	return (
		<div>
		  before-
			{fromLocal}
			{props.fromProps}
			{fromLocal + props.fromProps}
			{{props: props.fromProps}}
			{{local: fromLocal}}
			{{props: props.fromProps, local: fromLocal}}
			-after
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
const _hf0 = (p0, p1)=>p0 + p1.fromProps;
const _hf0_str = "p0+p1.fromProps";
const _hf1 = (p0)=>({
        props: p0.fromProps
    });
const _hf1_str = "{props:p0.fromProps}";
const _hf2 = (p0, p1)=>({
        props: p1.fromProps,
        local: p0
    });
const _hf2_str = "{props:p1.fromProps,local:p0}";
import { useSignal } from '@qwik.dev/core';
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    let fromLocal = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        "before-",
        fromLocal,
        _wrapProp(props, "fromProps"),
        _fnSignal(_hf0, [
            fromLocal,
            props
        ], _hf0_str),
        _fnSignal(_hf1, [
            props
        ], _hf1_str),
        {
            local: fromLocal
        },
        _fnSignal(_hf2, [
            fromLocal,
            props
        ], _hf2_str),
        "-after"
    ], 1, "u6_0");
}, "Works_component_t45qL4vNGv0"));
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Inline strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<div>` with children array; string literals `"before-"` and `"-after"` in children
- **[CONV-04] Signal Helpers**: `_wrapProp(props, "fromProps")` for direct prop child; `_fnSignal` for mixed expressions; `{local: fromLocal}` NOT wrapped
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-14] Hoisted Functions**: Three hoisted functions with different parameter ordering than `example_props_wrapping_children`

**Key behavior**: Contrasts with `example_props_wrapping_children` -- here `props` is NOT destructured (parameter remains `props`), so there is no `_rawProps` renaming. The prop access is `props.fromProps` directly.

Critical difference in hoisted function parameter order:
- In `_hf0`: `(p0, p1)=>p0 + p1.fromProps` -- `fromLocal` is p0, `props` is p1 (different from children variant where `_rawProps` was p0)
- In `_hf2`: `(p0, p1)=>({props: p1.fromProps, local: p0})` -- again `fromLocal` first, `props` second

This shows the optimizer assigns parameter positions based on the order variables appear in the source expression, NOT based on any fixed rule about props-first-or-local-first.

Additional behaviors:
- Static text `"before-"` and `"-after"` are preserved as string children in the array
- `{local: fromLocal}` with only a local signal is still NOT wrapped (same rule as children variant)
- `_wrapProp(props, "fromProps")` for single direct prop access in children

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
