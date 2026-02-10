# Test: example_dev_mode

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Dev (non-default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$(() => {
	return (
		<Cmp>
			<p class="stuff" onClick$={() => console.log('warn')}>Hello Qwik</p>
		</Cmp>
	);
});
```

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrlDEV(i_ckEPmXZlub0, "App_component_ckEPmXZlub0", {
    file: "/user/qwik/src/test.tsx",
    lo: 88,
    hi: 200,
    displayName: "test.tsx_App_component"
}));
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_Yl4ybrJWrt4 = ()=>import("./test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4");
export const App_component_ckEPmXZlub0 = ()=>{
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        "q-e:click": /*#__PURE__*/ qrlDEV(i_Yl4ybrJWrt4, "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4", {
            file: "/user/qwik/src/test.tsx",
            lo: 144,
            hi: 169,
            displayName: "test.tsx_App_component_Cmp_p_q_e_click"
        })
    }, "Hello Qwik", 3, null, {
        fileName: "test.tsx",
        lineNumber: 7,
        columnNumber: 4
    }), 3, "u6_0", {
        fileName: "test.tsx",
        lineNumber: 6,
        columnNumber: 3
    });
};
```

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
  "loc": [88, 200]
}
```

### Module: test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4.js (ENTRY POINT)

```javascript
export const App_component_Cmp_p_q_e_click_Yl4ybrJWrt4 = ()=>console.log('warn');
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4",
  "entry": null,
  "displayName": "test.tsx_App_component_Cmp_p_q_e_click",
  "hash": "Yl4ybrJWrt4",
  "canonicalFilename": "test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [144, 169]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls (Dev variant)**: `qrlDEV()` includes `{ file, lo, hi, displayName }` source metadata
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with dev `{ fileName, lineNumber, columnNumber }` as last arg
- **[CONV-06] Lazy Imports**: Dynamic imports for component and event handler
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all QRL and JSX calls
- **[CONV-08] Segment Extraction**: Component body and onClick handler extracted

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrlDEV` | test.js, component entry | @qwik.dev/core | 2 |
| `_jsxSorted` | component entry | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
