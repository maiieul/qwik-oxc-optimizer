# Test: example_component_with_event_listeners_inside_loop

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Tests event handler extraction from multiple loop patterns (map, for-i, for-of, for-in, while). Each loop's click handler is extracted into its own segment. Handlers inside indexed loops (for-i, for-in, while) capture the loop variable via `_captures` plus receive it as a parameter (`q:p`), while map/for-of handlers receive the item directly. Uses `_fnSignal` and hoisted functions (`_hf0`) for reactive index-based access patterns.

## Input

### Source Code
```tsx
import { $, component$, useStore, useSignal } from '@qwik.dev/core';
export const App = component$(() => {
      const cart = useStore<string[]>([]);
      const results = useSignal(['foo']);
      function loopArrowFn(results: string[]) { /* map with onClick$ */ }
      function loopForI(results: string[]) { /* for-i with onClick$ */ }
      function loopForOf(results: string[]) { /* for-of with onClick$ */ }
      function loopForIn(results: string[]) { /* for-in with onClick$ */ }
      function loopWhile(results: string[]) { /* while with onClick$ */ }
      return (
        <div>
          {results.value.map((item) => (<button id="second" onClick$={...}>{item}</button>))}
          {loopArrowFn(results.value)}
          {/* ... other loop calls */}
        </div>
      );
    });
```

*Full input is ~100 lines with 5 loop patterns and a direct map in the return JSX.*

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Component with useStore, useSignal, 5 loop helper functions each containing onClick$ handlers, plus a direct map in JSX return.*

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

*Parsed as `.js`. Standard componentQrl/qrl wrapper.*

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const _hf0 = (p0, p1)=>p1[p0];
const _hf0_str = "p1[p0]";
// ... lazy imports for 6 event handlers ...
export const App_component_ckEPmXZlub0 = ()=>{
    const cart = useStore([]);
    const results = useSignal(['foo']);
    // Each loop function creates a qrl with captures and uses _jsxSorted with "q:p"
    // for-i/for-in/while use _fnSignal(_hf0, [index, array], _hf0_str) for reactive access
    // map/for-of pass item directly
    return /*#__PURE__*/ _jsxSorted("div", null, null, [...], 1, "u6_6");
};
```

*Full module is ~90 lines with 6 lazy imports and 5 loop functions.*

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. Complex component body with hoisted function `_hf0`, multiple qrl() calls with capture arrays, _fnSignal for index-based reactive access, and _jsxSorted for JSX.*

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
  "loc": [101, 2370]
}
```

### Module: test.tsx_App_component_loopArrowFn_span_q_e_click_Wau7C836nf0.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_loopArrowFn_span_q_e_click_Wau7C836nf0 = (_, _1, item)=>{
    const cart = _captures[0];
    cart.push(item);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart"])

### Module: test.tsx_App_component_loopForI_span_q_e_click_PbCYbPM6etI.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_loopForI_span_q_e_click_PbCYbPM6etI = (_, _1, i)=>{
    const cart = _captures[0], results = _captures[1];
    cart.push(results[i]);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart", "results"])

### Module: test.tsx_App_component_loopForOf_span_q_e_click_zlNGHYu926I.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_loopForOf_span_q_e_click_zlNGHYu926I = (_, _1, item)=>{
    const cart = _captures[0];
    cart.push(item);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart"])

### Module: test.tsx_App_component_loopForIn_span_q_e_click_adzBGickx1U.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_loopForIn_span_q_e_click_adzBGickx1U = (_, _1, key)=>{
    const cart = _captures[0], results = _captures[1];
    cart.push(results[key]);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart", "results"])

### Module: test.tsx_App_component_loopWhile_span_q_e_click_05kCMrZVn5E.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_loopWhile_span_q_e_click_05kCMrZVn5E = (_, _1, i)=>{
    const cart = _captures[0], results = _captures[1];
    cart.push(results[i]);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart", "results"])

### Module: test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_button_q_e_click_UB6Fs5a3bd8 = (_, _1, item)=>{
    const cart = _captures[0];
    cart.push(item);
};
```

#### Segment Metadata (captures: true, captureNames: ["cart"])

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` with capture arrays (`[cart]`, `[cart, results]`) for each loop handler
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `q-e:click` attribute with qrl
- **[CONV-03] JSX Transforms**: All JSX transpiled to `_jsxSorted()` calls
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [i, results], _hf0_str)` used for reactive index-based array access in for-i, for-in, while loops
- **[CONV-05] Capture Patterns**: `_captures[0]`, `_captures[1]` used in all 6 extracted handlers to access closed-over variables. The `q:p` attribute passes loop variables as extra parameters.
- **[CONV-06] Lazy Imports**: 6 lazy imports for the 6 extracted handlers, plus 1 for the component body
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted calls
- **[CONV-08] Segment Extraction**: 7 segments total: 1 component body + 6 event handlers (one per loop pattern + direct map)
- **[CONV-14] Hoisted Functions**: `const _hf0 = (p0, p1)=>p1[p0]; const _hf0_str = "p1[p0]"` -- hoisted array-access function for use with _fnSignal

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js / App_component.js | @qwik.dev/core | 7 |
| _jsxSorted | App_component.js | @qwik.dev/core | 12 |
| _fnSignal | App_component.js | @qwik.dev/core | 3 |
| useStore | App_component.js | @qwik.dev/core | 1 |
| useSignal | App_component.js | @qwik.dev/core | 1 |
| _captures | (6 handler segments) | @qwik.dev/core | 6 |

## Diagnostics

```json
[]
```
