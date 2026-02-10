# Test: example_explicit_ext_no_transpile

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Single |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |
| Explicit Extensions | true |

**Key insight:** Neither TS nor JSX is transpiled, so output keeps `.tsx` extension. `explicit_extensions: true` means import paths include file extensions. Single entry strategy groups all segments into one file.

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

*TSX input. ImportDeclaration with component$, $, useStyles$. ExportNamedDeclaration containing component$ call with useStyles$ and nested $ call returning JSX.*

</details>

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.tsx");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Lazy import has `.tsx` extension. Standard componentQrl/qrl pattern.*

</details>

### Module: test.tsx_App_component_useStyles_t35nSa5UV7U.tsx

```tsx
export const App_component_useStyles_t35nSa5UV7U = 'hola';
```

<details>
<summary>Output AST (OXC)</summary>

*Single ExportNamedDeclaration with string literal value.*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_useStyles_t35nSa5UV7U",
  "entry": "entry_segments",
  "displayName": "test.tsx_App_component_useStyles",
  "hash": "t35nSa5UV7U",
  "canonicalFilename": "test.tsx_App_component_useStyles_t35nSa5UV7U",
  "path": "",
  "extension": "tsx",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "useStyles$",
  "captures": false,
  "loc": [118, 124]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.tsx

```tsx
import { qrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
const i_t35nSa5UV7U = ()=>import("./test.tsx_App_component_useStyles_t35nSa5UV7U.tsx");
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU.tsx");
export const App_component_ckEPmXZlub0 = (props)=>{
    useStylesQrl(/*#__PURE__*/ qrl(i_t35nSa5UV7U, "App_component_useStyles_t35nSa5UV7U"));
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Contains useStylesQrl call with qrl reference and return of another qrl reference.*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": "entry_segments",
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [93, 165],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.tsx

```tsx
export const App_component_1_w0t0o3QMovU = ()=><div></div>;
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. JSX preserved (no transpile_jsx). Arrow function returning JSXElement.*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_1_w0t0o3QMovU",
  "entry": "entry_segments",
  "displayName": "test.tsx_App_component_1",
  "hash": "w0t0o3QMovU",
  "canonicalFilename": "test.tsx_App_component_1_w0t0o3QMovU",
  "path": "",
  "extension": "tsx",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [137, 161]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used throughout for lazy references to extracted segments
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `useStyles$` -> `useStylesQrl`, `$` -> `qrl`
- **[CONV-06] Lazy Imports**: All segment references use lazy `() => import("./...")` with `.tsx` extensions (explicit_extensions + no transpile)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()` calls
- **[CONV-08] Segment Extraction**: Three segments extracted (useStyles value, component body, inner $ callback) despite Single strategy -- segments are still created but grouped under `entry_segments`

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.tsx | @qwik.dev/core | 1 |
| qrl | test.tsx | @qwik.dev/core | 1 |
| qrl | App_component_ckEPmXZlub0.tsx | @qwik.dev/core | 2 |
| useStylesQrl | App_component_ckEPmXZlub0.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
