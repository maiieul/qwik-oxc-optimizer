# Test: should_not_wrap_ternary_function_operator_with_fn

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
	const toggle = useSignal(true);
	const t = (key: string) => key;
	return (
		<button
			type="button"
			title={
				toggle.value !== ''
					? t('app.message.exists@@there is a message for you')
					: t('app.message.not_exists@@click to get a message!')
			}
		></button>
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
import { _jsxSorted } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = ()=>{
    const toggle = useSignal(true);
    const t = (key)=>key;
    return /*#__PURE__*/ _jsxSorted("button", {
        title: toggle.value !== '' ? t('app.message.exists@@there is a message for you') : t('app.message.not_exists@@click to get a message!')
    }, {
        type: "button"
    }, null, 3, "u6_0");
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
  "captures": false, "loc": [90, 389]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with `title` in var props, `type` in const props
- **[CONV-06] Lazy Imports**: Lazy import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: Component body extracted

**Key behavior**: The `title` prop contains a ternary expression where both branches call a function `t()`. Even though `toggle.value` is a signal access, the overall expression involves function calls (`t(...)`) which makes it non-trackable. The optimizer places `title` in the var props object (not wrapped with `_fnSignal`). The `type="button"` static prop goes in the const props object. This test verifies that the optimizer does not attempt to wrap ternary expressions containing function calls.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
