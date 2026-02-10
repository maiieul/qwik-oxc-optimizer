# Test: destructure_args_colon_props2

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useSignal } from "@qwik.dev/core";
export default component$((props) => {
	const { 'bind:value': bindValue } = props;
	const test = useSignal(bindValue);
	return (
		<>
		{test.value}
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (props)=>{
    const test = useSignal(props["bind:value"]);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, _wrapProp(test), 3, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

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
  "loc": [88, 238],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module to reference component segment
- **[CONV-02] Dollar-to-Qrl**: `component$` transformed to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted(_Fragment, ...)` replaces JSX fragment
- **[CONV-04] Signal Helpers**: `_wrapProp(test)` wraps `test.value` in JSX for reactive tracking; also `props["bind:value"]` passed directly to `useSignal`
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on component and QRL calls
- **[CONV-08] Segment Extraction**: Component body extracted to entry point
- **[CONV-11] Props Destructuring**: `{ 'bind:value': bindValue } = props` transformed -- instead of destructuring, the entry point receives `props` parameter and accesses `props["bind:value"]` directly
- **[CONV-12] Input Binding**: `bind:value` colon-prop accessed as `props["bind:value"]` and passed to `useSignal`

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 1 |
| `_wrapProp` | entry point | @qwik.dev/core | 1 |
| `useSignal` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
