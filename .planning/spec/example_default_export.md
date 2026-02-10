# Test: example_default_export

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | src/routes/_repl/[id]/[[...slug]].tsx |
| Entry Strategy | Smart |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Explicit Extensions | true |

**Key insight:** Tests default export naming with a complex filename containing route parameters (`[id]`, `[[...slug]]`). The default export component gets named `slug_component` (derived from the filename stem `[[...slug]]`). Smart entry strategy extracts segments. The `.js` output extension comes from both transpile flags being true.

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';
import { sibling } from './sibling';

export default component$(() => {
	return (
		<div onClick$={() => console.log(mongodb, sibling)}>
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. ExportDefaultDeclaration with component$ call. Imports sibling from relative path. Click handler references undefined `mongodb` and imported `sibling`.*

</details>

## Output

### Module: src/routes/_repl/[id]/[[...slug]].js (main module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_0AM8HPnkNs4 = ()=>import("./[[...slug]].tsx_slug_component_0AM8HPnkNs4.js");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0AM8HPnkNs4, "slug_component_0AM8HPnkNs4"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. ExportDefaultDeclaration with componentQrl. Segment name derived from filename: `slug_component`.*

</details>

### Module: [[...slug]].tsx_slug_component_div_q_e_click_bCwVPYSTQ0w.js (ENTRY POINT)

```javascript
import { sibling } from "./sibling";
export const slug_component_div_q_e_click_bCwVPYSTQ0w = ()=>console.log(mongodb, sibling);
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Imports sibling from relative path. References mongodb as unresolved identifier.*

</details>

#### Segment Metadata
```json
{
  "origin": "src/routes/_repl/[id]/[[...slug]].tsx",
  "name": "slug_component_div_q_e_click_bCwVPYSTQ0w",
  "entry": null,
  "displayName": "[[...slug]].tsx_slug_component_div_q_e_click",
  "hash": "bCwVPYSTQ0w",
  "canonicalFilename": "[[...slug]].tsx_slug_component_div_q_e_click_bCwVPYSTQ0w",
  "path": "src/routes/_repl/[id]",
  "extension": "js",
  "parent": "slug_component_0AM8HPnkNs4",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [146, 181]
}
```

### Module: [[...slug]].tsx_slug_component_0AM8HPnkNs4.js

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_bCwVPYSTQ0w = ()=>import("./[[...slug]].tsx_slug_component_div_q_e_click_bCwVPYSTQ0w.js");
export const slug_component_0AM8HPnkNs4 = ()=>{
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_bCwVPYSTQ0w, "slug_component_div_q_e_click_bCwVPYSTQ0w")
    }, null, 3, "W4_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. JSX transpiled. qrl reference to click handler segment.*

</details>

#### Segment Metadata
```json
{
  "origin": "src/routes/_repl/[id]/[[...slug]].tsx",
  "name": "slug_component_0AM8HPnkNs4",
  "entry": "src/routes/_repl/[id]/[[...slug]].tsx_entry_[[...slug]]",
  "displayName": "[[...slug]].tsx_slug_component",
  "hash": "0AM8HPnkNs4",
  "canonicalFilename": "[[...slug]].tsx_slug_component_0AM8HPnkNs4",
  "path": "src/routes/_repl/[id]",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [111, 198]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for lazy segment references
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `q-e:click` attribute with qrl
- **[CONV-03] JSX Transforms**: `<div>` transpiled to `_jsxSorted("div", ...)` with click handler in static props
- **[CONV-06] Lazy Imports**: `() => import("./[[...slug]].tsx_slug_component_*.js")` with explicit extensions
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted
- **[CONV-08] Segment Extraction**: Two segments: component body and click handler. Entry field shows `entry_[[...slug]]` pattern

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | [[...slug]].js | @qwik.dev/core | 1 |
| qrl | [[...slug]].js | @qwik.dev/core | 1 |
| qrl | slug_component_0AM8HPnkNs4.js | @qwik.dev/core | 1 |
| _jsxSorted | slug_component_0AM8HPnkNs4.js | @qwik.dev/core | 1 |
| console.log | slug_component_div_q_e_click.js | (global) | 1 |

## Diagnostics

```json
[]
```
