# Test: should_wrap_object_with_fn_signal

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';
export default component$((props) => {
	// not destructure it so it is a var prop
	const item = props.something.count;
	return (
		<>
			<div data-no-wrap={item ? item * 2 : null} data-wrap={props.myobj.id + "test"}></div>
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
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.myobj.id + "test";
const _hf0_str = 'p0.myobj.id+"test"';
export const test_component_LUXeXe0DQrg = (props)=>{
    // not destructure it so it is a var prop
    const item = props.something.count;
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("div", {
        "data-no-wrap": item ? item * 2 : null,
        "data-wrap": _fnSignal(_hf0, [props], _hf0_str)
    }, null, null, 3, null), 1, "u6_0");
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
  "captures": false, "loc": [73, 281],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for Fragment and `<div>`; `data-no-wrap` and `data-wrap` in var props
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [props], _hf0_str)` wraps `props.myobj.id + "test"` (direct nested prop access with concatenation)
- **[CONV-06] Lazy Imports**: Lazy import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: Component body extracted
- **[CONV-14] Hoisted Functions**: `_hf0 = (p0)=>p0.myobj.id + "test"` with string representation

**Key behavior**: Two contrasting prop behaviors:
1. `data-no-wrap={item ? item * 2 : null}`: `item` is assigned from `props.something.count` earlier, making it a **local variable**. Since it is a local derived value (not a direct prop/store access), the optimizer does NOT wrap it.
2. `data-wrap={props.myobj.id + "test"}`: Direct nested prop access (`props.myobj.id`) plus a string concatenation IS wrapped with `_fnSignal` because the optimizer can trace through to `props` as the reactive source.

This distinction is critical: assigned-from-prop variables lose their reactivity tracking, while direct `props.x.y.z` access chains maintain it.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 2 |
| `_fnSignal` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
