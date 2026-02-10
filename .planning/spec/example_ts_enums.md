# Test: example_ts_enums

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** With both transpile flags true, TypeScript enums are transpiled to JavaScript IIFE pattern. The exported enum `Thing` becomes `export var Thing = /*#__PURE__*/ function(Thing) { ... }({})`. In the extracted segment, `Thing.A` is replaced with the literal `0` (constant folding).

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export enum Thing {
	A,
	B
}

export const App = component$(() => {
	console.log(Thing.A);
	return (
		<>
			<p class="stuff">Hello Qwik</p>
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Contains TSEnumDeclaration (Thing with members A, B) and component$ with JSX Fragment.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export var Thing = /*#__PURE__*/ function(Thing) {
    Thing[Thing["A"] = 0] = "A";
    Thing[Thing["B"] = 1] = "B";
    return Thing;
}({});
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. TS enum transpiled to IIFE with bidirectional mapping. componentQrl/qrl pattern for App.*

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_ckEPmXZlub0 = ()=>{
    console.log(0);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff"
    }, "Hello Qwik", 3, null), 3, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. `Thing.A` replaced with literal `0` (enum value inlined). Fragment imported from jsx-runtime. JSX transpiled to _jsxSorted calls.*

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
  "loc": [118, 210]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used for lazy segment reference
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls, Fragment imported from `@qwik.dev/core/jsx-runtime`
- **[CONV-06] Lazy Imports**: `const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0")` (no extension -- explicit_extensions is false/default)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on componentQrl, qrl, _jsxSorted, and the enum IIFE
- **[CONV-08] Segment Extraction**: Component body extracted to separate segment

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| _jsxSorted | App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |
| console.log | App_component_ckEPmXZlub0.js | (global) | 1 |

## Diagnostics

```json
[]
```
