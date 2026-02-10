# Test: hoisted_fn_signal_in_loop

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';

export const App = component$(() => {
  const data = { value: [
    { value: { id: 1, selected: { value: true } } },
    { value: { id: 2, selected: { value: false } } },
    { value: { id: 3, selected: { value: true } } }
  ]};

  return (
    <table>
      {data.value.map((row) => {
        return (
          <tr
            key={row.value.id}
            class={row.value.selected.value ? "danger" : ""}
          >
            <td>{row.value.id}</td>
          </tr>
        );
      })}
    </table>
  );
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value.selected.value ? "danger" : "";
const _hf0_str = 'p0.value.selected.value?"danger":""';
const _hf1 = (p0)=>p0.value.id;
const _hf1_str = "p0.value.id";
export const App_component_ckEPmXZlub0 = ()=>{
    const data = {
        value: [
            { value: { id: 1, selected: { value: true } } },
            { value: { id: 2, selected: { value: false } } },
            { value: { id: 3, selected: { value: true } } }
        ]
    };
    return /*#__PURE__*/ _jsxSorted("table", null, null, data.value.map((row)=>{
        return /*#__PURE__*/ _jsxSorted("tr", {
            class: _fnSignal(_hf0, [row], _hf0_str)
        }, null, /*#__PURE__*/ _jsxSorted("td", null, null, _fnSignal(_hf1, [row], _hf1_str), 1, null), 1, row.value.id);
    }), 1, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

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
  "loc": [78, 563]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<table>`, `<tr>`, `<td>` elements
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [row], _hf0_str)` for `row.value.selected.value ? "danger" : ""` (conditional class) and `_fnSignal(_hf1, [row], _hf1_str)` for `row.value.id` (child text)
- **[CONV-06] Lazy Imports**: Lazy import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-08] Segment Extraction**: Component body extracted to separate entry point module
- **[CONV-14] Hoisted Functions**: `_hf0` and `_hf1` are hoisted outside the `.map()` loop. The `row` variable from the loop is passed as a parameter to the hoisted function. This is the key optimization: the function is created once and reused for each iteration, with `row` as the signal source.

**Key behavior**: When signal-like property access occurs inside a loop (`.map()`), the optimizer hoists the accessor functions outside the loop body. `_hf0` handles the conditional class expression and `_hf1` handles the id access. The `row` iteration variable is passed as the signal source, allowing per-row reactive subscriptions. The `key` prop uses `row.value.id` directly (not wrapped) since keys are used for reconciliation, not rendering.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 3 |
| `_fnSignal` | entry point | @qwik.dev/core | 2 |

## Diagnostics

None (`[]`)
