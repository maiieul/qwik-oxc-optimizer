# Test: should_wrap_logical_expression_in_template

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useSignal } from '@qwik.dev/core';

export default component$(() => {
	const count = useSignal(0);
	const count2 = useSignal(0);
	return (
		<div>
			{(count || count2).value}
		</div>
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
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0, p1)=>(p0 || p1).value;
const _hf0_str = "(p0||p1).value";
export const test_component_LUXeXe0DQrg = ()=>{
    const count = useSignal(0);
    const count2 = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", null, null, _fnSignal(_hf0, [count, count2], _hf0_str), 3, "u6_0");
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
  "captures": false, "loc": [89, 233]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<div>` with single child
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [count, count2], _hf0_str)` wraps `(count || count2).value`
- **[CONV-06] Lazy Imports**: Lazy import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: Component body extracted
- **[CONV-14] Hoisted Functions**: `_hf0 = (p0, p1)=>(p0 || p1).value` with string `"(p0||p1).value"`

**Key behavior**: The optimizer wraps `(count || count2).value` with `_fnSignal` because it is a `.value` access on a logical expression of signals. Both signal dependencies (`count` and `count2`) are passed as parameters to the hoisted function. The string representation `"(p0||p1).value"` preserves the logical OR operator. The child gets flag=3 (immutable) since both sources are local signals.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 1 |
| `_fnSignal` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
