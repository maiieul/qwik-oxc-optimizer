# Test: example_explicit_ext_transpile

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Explicit Extensions | true |

**Key insight:** Both TS and JSX are transpiled, so output uses `.js` extension. `explicit_extensions: true` causes import paths to include `.js` file extensions. Compared to `example_explicit_ext_no_transpile`, this shows how transpile flags change the output extension.

## Input

### Source Code
```tsx
import { component$, $, useStyles$ } from '@qwik.dev/core';

export const App = component$((props) => {
	useStyles$('hola');
	return $(() => (
		<div></div>
	));
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input identical to example_explicit_ext_no_transpile. Same component structure with useStyles$ and nested $ call.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.js");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Import paths include `.js` extension due to explicit_extensions.*

</details>

### Module: test.tsx_App_component_useStyles_t35nSa5UV7U.js (ENTRY POINT)

```javascript
export const App_component_useStyles_t35nSa5UV7U = 'hola';
```

<details>
<summary>Output AST (OXC)</summary>

*Single ExportNamedDeclaration with string literal.*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_useStyles_t35nSa5UV7U",
  "entry": null,
  "displayName": "test.tsx_App_component_useStyles",
  "hash": "t35nSa5UV7U",
  "canonicalFilename": "test.tsx_App_component_useStyles_t35nSa5UV7U",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "useStyles$",
  "captures": false,
  "loc": [118, 124]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
const i_t35nSa5UV7U = ()=>import("./test.tsx_App_component_useStyles_t35nSa5UV7U.js");
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU.js");
export const App_component_ckEPmXZlub0 = (props)=>{
    useStylesQrl(/*#__PURE__*/ qrl(i_t35nSa5UV7U, "App_component_useStyles_t35nSa5UV7U"));
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Lazy imports reference `.js` files. useStylesQrl and qrl calls present.*

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
  "loc": [93, 165],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_1_w0t0o3QMovU = ()=>/*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_0");
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. JSX has been transpiled to _jsxSorted() call (since transpile_jsx: true).*

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
  "loc": [137, 161]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used for all lazy segment references
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `useStyles$` -> `useStylesQrl`, `$` -> `qrl`
- **[CONV-03] JSX Transforms**: `<div></div>` transpiled to `_jsxSorted("div", null, null, null, 3, "u6_0")` in the innermost segment
- **[CONV-06] Lazy Imports**: `() => import("./...js")` with `.js` extensions (transpile + explicit_extensions)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl`, `qrl`, `_jsxSorted` calls
- **[CONV-08] Segment Extraction**: Three entry point segments extracted

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |
| useStylesQrl | App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| _jsxSorted | App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
