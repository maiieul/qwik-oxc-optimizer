# Test: should_split_spread_props

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
	return (
		<div {...props}></div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit("div", {
        ..._getVarProps(props)
    }, _getConstProps(props), null, 0, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null,
  "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg",
  "canonicalFilename": "test.tsx_test_component_LUXeXe0DQrg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [78, 139],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` wraps lazy reference to the component segment
- **[CONV-02] Dollar-to-QRL**: `component$` transformed to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSplit()` used instead of `_jsxSorted()` because of spread props
- **[CONV-04] Signal Helpers**: `_getConstProps(props)` and `_getVarProps(props)` split spread props into const/var categories for reactivity
- **[CONV-06] Lazy Imports**: `const i_LUXeXe0DQrg = ()=>import(...)` for lazy segment loading
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()`, `qrl()`, and `_jsxSplit()` calls
- **[CONV-08] Segment Extraction**: Component body extracted to separate entry point module

**Key behavior**: When a component spreads its entire props object onto an element (`{...props}`), the optimizer uses `_jsxSplit()` instead of `_jsxSorted()` and splits props into variable props (`_getVarProps`) and constant props (`_getConstProps`) to enable fine-grained reactivity tracking.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSplit` | entry point | @qwik.dev/core | 1 |
| `_getVarProps` | entry point | @qwik.dev/core | 1 |
| `_getConstProps` | entry point | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
