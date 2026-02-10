# Test: example_capturing_fn_class

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
import { $, component$ } from '@qwik.dev/core';

export const App = component$(() => {
	function hola() {
		console.log('hola');
	}
	class Thing {}
	class Other {}

	return $(() => {
		hola();
		new Thing();
		return (
			<div></div>
		)
	});
})
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
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU");
export const App_component_ckEPmXZlub0 = ()=>{
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU");
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
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [81, 246]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_1_w0t0o3QMovU = ()=>{
    hola();
    new Thing();
    return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_0");
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
  "name": "App_component_1_w0t0o3QMovU",
  "entry": null,
  "displayName": "test.tsx_App_component_1",
  "hash": "w0t0o3QMovU",
  "canonicalFilename": "test.tsx_App_component_1_w0t0o3QMovU",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [177, 242]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used in main module and component segment to create lazy references
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `$()` returns a QRL via `qrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` in the nested `$()` segment
- **[CONV-06] Lazy Imports**: Dynamic imports for both component and nested `$()` segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, and `_jsxSorted()`
- **[CONV-08] Segment Extraction**: Two levels of extraction -- component body and nested `$()` callback. The `hola()` function and `Thing` class are used inside the `$()` scope but defined outside it, which produces diagnostics errors (C02)

**Note:** The `hola` function and `Thing` class references in the `$()` scope are NOT captured (no `_captures` usage). Instead, they appear as bare references (`hola()`, `new Thing()`) which generates C02 diagnostic errors because functions and classes cannot be serialized across Qrl boundaries.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component entry | @qwik.dev/core | 2 |
| `_jsxSorted` | nested entry | @qwik.dev/core | 1 |

## Diagnostics

```json
[
  {
    "category": "error",
    "code": "C02",
    "file": "test.tsx",
    "message": "Reference to identifier 'Thing' can not be used inside a Qrl($) scope because it's a function",
    "highlights": null,
    "suggestions": null,
    "scope": "optimizer"
  },
  {
    "category": "error",
    "code": "C02",
    "file": "test.tsx",
    "message": "Reference to identifier 'hola' can not be used inside a Qrl($) scope because it's a function",
    "highlights": null,
    "suggestions": null,
    "scope": "optimizer"
  }
]
```
