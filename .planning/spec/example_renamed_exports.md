# Test: example_renamed_exports

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Tests aliased imports from Qwik. `component$` is imported as `Component` and `$` is imported as `onRender`. The optimizer recognizes the aliased names and applies the same transformations: `Component` -> `componentQrl`, `onRender` -> `qrl`. The inner `onRender(() => ...)` produces a nested segment with capture of the `state` variable, accessed via `_captures` and `_wrapProp` in the output.

## Input

### Source Code
```tsx
import { component$ as Component, $ as onRender, useStore } from '@qwik.dev/core';

export const App = Component((props) => {
	const state = useStore({thing: 0});

	return onRender(() => (
		<div>{state.thing}</div>
	));
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Aliased imports: component$ as Component, $ as onRender. Single component with useStore and inner onRender callback returning JSX with reactive state access (state.thing).*

</details>

## Output

### Module: test.js (main module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_NuXFTHRjvXE = ()=>import("./test.tsx_App_Component_NuXFTHRjvXE");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_NuXFTHRjvXE, "App_Component_NuXFTHRjvXE"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Aliased `Component` recognized as component$ and transformed to componentQrl. Aliased `onRender` recognized as $ and transformed to qrl in inner segment. Lazy import for component body segment.*

</details>

### Module: test.tsx_App_Component_NuXFTHRjvXE.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const i_A08tXHb9pEk = ()=>import("./test.tsx_App_Component_1_A08tXHb9pEk");
export const App_Component_NuXFTHRjvXE = (props)=>{
    const state = useStore({
        thing: 0
    });
    return /*#__PURE__*/ qrl(i_A08tXHb9pEk, "App_Component_1_A08tXHb9pEk", [
        state
    ]);
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_Component_NuXFTHRjvXE",
  "entry": null,
  "displayName": "test.tsx_App_Component",
  "hash": "NuXFTHRjvXE",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_Component_1_A08tXHb9pEk.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
export const App_Component_1_A08tXHb9pEk = ()=>{
    const state = _captures[0];
    return /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(state, "thing"), 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_Component_1_A08tXHb9pEk",
  "entry": null,
  "displayName": "test.tsx_App_Component_1",
  "hash": "A08tXHb9pEk",
  "extension": "js",
  "parent": "App_Component_NuXFTHRjvXE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "captureNames": ["state"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for both segment references; inner segment has capture array `[state]`
- **[CONV-02] Dollar-to-Qrl**: `Component` (alias of component$) -> `componentQrl`, `onRender` (alias of $) -> `qrl`
- **[CONV-03] JSX Transforms**: JSX `<div>{state.thing}</div>` transpiled to `_jsxSorted("div", ...)` with `_wrapProp`
- **[CONV-04] Signal Helpers**: `_wrapProp(state, "thing")` wraps reactive store property access for signal tracking
- **[CONV-05] Capture Patterns**: `_captures[0]` in inner segment restores captured `state` variable
- **[CONV-06] Lazy Imports**: Standard lazy import pattern for both segments
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted calls
- **[CONV-08] Segment Extraction**: Two segments: component body and inner $ callback

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js / App_Component.js | @qwik.dev/core | 2 |
| useStore | App_Component.js | @qwik.dev/core | 1 |
| _jsxSorted | App_Component_1.js | @qwik.dev/core | 1 |
| _wrapProp | App_Component_1.js | @qwik.dev/core | 1 |
| _captures | App_Component_1.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
