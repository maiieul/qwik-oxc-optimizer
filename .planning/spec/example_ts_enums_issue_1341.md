# Test: example_ts_enums_issue_1341

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Similar to `example_ts_enums` but the enum is NOT exported (just `enum Thing`, not `export enum Thing`). The non-exported enum is still transpiled to an IIFE but wrapped in `/*#__PURE__*/` and not exported. `Thing.A` is still replaced with `0` in the segment. This tests issue #1341 where non-exported enums needed correct handling.

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

enum Thing {
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

*TSX input. Non-exported TSEnumDeclaration (Thing). component$ with console.log(Thing.A) and JSX Fragment.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
/*#__PURE__*/ (function(Thing) {
    Thing[Thing["A"] = 0] = "A";
    Thing[Thing["B"] = 1] = "B";
    return Thing;
})({});
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Non-exported enum transpiled to IIFE expression statement (not assigned to variable since it has no export and the variable is unused in the main module). The enum IIFE is marked PURE so tree-shakers can remove it.*

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

*Parsed as `.js`. Identical output to example_ts_enums -- Thing.A replaced with 0.*

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
  "loc": [111, 203]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for lazy segment reference
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls
- **[CONV-06] Lazy Imports**: Standard lazy import pattern
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on enum IIFE, componentQrl, qrl, _jsxSorted
- **[CONV-08] Segment Extraction**: Component body extracted

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
