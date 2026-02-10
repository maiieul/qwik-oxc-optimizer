# Test: example_dev_mode_inlined

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline (non-default) |
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
import { inlinedQrlDEV } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrlDEV(()=>{
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        "q-e:click": /*#__PURE__*/ inlinedQrlDEV(()=>console.log('warn'), "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4", {
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
}, "App_component_ckEPmXZlub0", {
    file: "/user/qwik/src/test.tsx",
    lo: 88,
    hi: 200,
    displayName: "test.tsx_App_component"
}));
```

## Conventions Applied

- **[CONV-01] QRL Calls (Dev+Inline variant)**: `inlinedQrlDEV()` combines inlining with dev metadata. No separate segments
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with dev source locations
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all calls

**Key behavior:** Inline+Dev produces `inlinedQrlDEV()` -- all code in one file, no dynamic imports, with dev source metadata.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrlDEV` | test.js | @qwik.dev/core | 2 |
| `_jsxSorted` | test.js | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
