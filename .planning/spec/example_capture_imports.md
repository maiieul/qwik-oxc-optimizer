# Test: example_capture_imports

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
import { component$, useStyles$ } from '@qwik.dev/core';
import css1 from './global.css';
import css2 from './style.css';
import css3 from './style.css';

export const App = component$(() => {
	useStyles$(`${css1}${css2}`);
	useStyles$(css3);
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

### Module: test.tsx_App_component_useStyles_t35nSa5UV7U.js (ENTRY POINT)

```javascript
import css1 from "./global.css";
import css2 from "./style.css";
export const App_component_useStyles_t35nSa5UV7U = `${css1}${css2}`;
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

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
  "loc": [207, 223]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
const i_t35nSa5UV7U = ()=>import("./test.tsx_App_component_useStyles_t35nSa5UV7U");
const i_xBK4W0ZKWe8 = ()=>import("./test.tsx_App_component_useStyles_1_xBK4W0ZKWe8");
export const App_component_ckEPmXZlub0 = ()=>{
    useStylesQrl(/*#__PURE__*/ qrl(i_t35nSa5UV7U, "App_component_useStyles_t35nSa5UV7U"));
    useStylesQrl(/*#__PURE__*/ qrl(i_xBK4W0ZKWe8, "App_component_useStyles_1_xBK4W0ZKWe8"));
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
  "loc": [187, 246]
}
```

### Module: test.tsx_App_component_useStyles_1_xBK4W0ZKWe8.js (ENTRY POINT)

```javascript
import css3 from "./style.css";
export const App_component_useStyles_1_xBK4W0ZKWe8 = css3;
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_useStyles_1_xBK4W0ZKWe8",
  "entry": null,
  "displayName": "test.tsx_App_component_useStyles_1",
  "hash": "xBK4W0ZKWe8",
  "canonicalFilename": "test.tsx_App_component_useStyles_1_xBK4W0ZKWe8",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "useStyles$",
  "captures": false,
  "loc": [238, 242]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used in main module and component segment to reference style and component segments
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useStyles$` to `useStylesQrl()`
- **[CONV-05] Capture Patterns**: CSS imports (`css1`, `css2`, `css3`) are captured by their respective style segments -- each `useStyles$` argument becomes a separate segment that imports the CSS modules it references. The imports are "captured" by being re-imported in the extracted segment rather than through `_captures[]`
- **[CONV-06] Lazy Imports**: Multiple dynamic imports: one for component segment, two for style segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, and `useStylesQrl()` calls
- **[CONV-08] Segment Extraction**: Three entry point segments extracted: one for the component body, two for the `useStyles$` arguments (template literal and direct CSS reference)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component entry | @qwik.dev/core | 3 |
| `useStylesQrl` | component entry | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
