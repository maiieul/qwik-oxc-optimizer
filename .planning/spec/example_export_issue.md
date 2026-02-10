# Test: example_export_issue

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Tests complex export scenarios: a non-exported component (`const App`), an exported component (`export const Root`), a re-export alias (`export { Other as App }`), and a default export (`export default App`). The optimizer creates an `_auto_App` alias so that extracted segments can import the non-exported `App` component. The `{ Other as App }` named export is preserved alongside the auto-generated alias.

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';

const App = component$(() => {
	return (
		<div>hola</div>
	);
});


export const Root = component$((props: Stuff) => {
	return (
		<App/>
	);
});

const Other = 12;
export { Other as App };

export default App;
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Non-exported component App, exported Root component, re-export alias, default export.*

</details>

## Output

### Module: test.js (main module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
const i_royhjYaCbYE = ()=>import("./test.tsx_Root_component_royhjYaCbYE");
const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
export const Root = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_royhjYaCbYE, "Root_component_royhjYaCbYE"));
const Other = 12;
export { Other as App };
export default App;
export { App as _auto_App };
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Two component declarations. The `_auto_App` export alias is auto-generated so that extracted segments can import the non-exported App component. Original re-export `{ Other as App }` preserved.*

</details>

### Module: test.tsx_Root_component_royhjYaCbYE.js (ENTRY POINT)

```javascript
import { _auto_App as App } from "./test";
import { _jsxSorted } from "@qwik.dev/core";
export const Root_component_royhjYaCbYE = (props)=>{
    return /*#__PURE__*/ _jsxSorted(App, null, null, null, 3, "u6_1");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Imports App via the auto-generated `_auto_App` alias. Renders App component via _jsxSorted.*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Root_component_royhjYaCbYE",
  "entry": null,
  "displayName": "test.tsx_Root_component",
  "hash": "royhjYaCbYE",
  "canonicalFilename": "test.tsx_Root_component_royhjYaCbYE",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [148, 192],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_ckEPmXZlub0 = ()=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, "hola", 3, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Simple component rendering a div with text "hola".*

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
  "loc": [71, 112]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for both App and Root component segment references
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl` for both components
- **[CONV-03] JSX Transforms**: `<App/>` and `<div>hola</div>` transpiled to `_jsxSorted()` calls
- **[CONV-06] Lazy Imports**: Standard lazy import pattern for both segments
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted calls
- **[CONV-08] Segment Extraction**: Two component bodies extracted. Root's segment imports App via `_auto_App` alias

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 2 |
| qrl | test.js | @qwik.dev/core | 2 |
| _jsxSorted | Root_component_royhjYaCbYE.js | @qwik.dev/core | 1 |
| _jsxSorted | App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
