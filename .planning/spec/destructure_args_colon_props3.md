# Test: destructure_args_colon_props3

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
	const { test, ...rest } = props;
	const test = useSignal(rest['bind:value']);
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
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _restProps } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.test.value;
const _hf0_str = "p0.test.value";
export const test_component_LUXeXe0DQrg = (props)=>{
    const rest = _restProps(props, [
        "test"
    ]);
    useSignal(rest['bind:value']);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, _fnSignal(_hf0, [
        props
    ], _hf0_str), 1, "u6_0");
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
  "loc": [88, 237],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted(_Fragment, ...)` replaces JSX fragment
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [props], _hf0_str)` wraps `test.value` access for reactive tracking
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on component, QRL, and JSX calls
- **[CONV-08] Segment Extraction**: Component body extracted to entry point
- **[CONV-11] Props Destructuring**: `{ test, ...rest } = props` transformed to `_restProps(props, ["test"])` -- the rest element is handled via `_restProps()` which creates a proxy excluding the named props
- **[CONV-12] Input Binding**: `rest['bind:value']` accessed directly for colon-prop binding
- **[CONV-14] Hoisted Functions**: `const _hf0 = (p0)=>p0.test.value` with `_hf0_str` -- the `test.value` property access is hoisted for efficient reactive tracking

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 1 |
| `_fnSignal` | entry point | @qwik.dev/core | 1 |
| `_restProps` | entry point | @qwik.dev/core | 1 |
| `useSignal` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
