# Test: should_wrap_type_asserted_variables_in_template

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
import { component$, useSignal } from '@qwik.dev/core';

export default component$(() => {
	const count = useSignal(0);
	return (
		<div>
			{(count as any).value}
		</div>
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
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = ()=>{
    const count = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(count), 3, "u6_0");
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
  "loc": [89, 198]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used in main module to reference the component segment
- **[CONV-02] Dollar-to-Qrl**: `component$` transformed to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` replaces JSX `<div>` in entry point
- **[CONV-04] Signal Helpers**: `_wrapProp(count)` wraps the `(count as any).value` type-asserted expression -- the optimizer sees through TypeScript type assertions and still applies signal wrapping
- **[CONV-06] Lazy Imports**: `const i_LUXeXe0DQrg = ()=>import(...)` for dynamic segment loading
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()`, `qrl()`, `_jsxSorted()`
- **[CONV-08] Segment Extraction**: Component body extracted to entry point module

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
