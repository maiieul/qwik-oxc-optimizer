# Test: example_noop_dev_mode

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Dev (non-default) |
| Dev Path | /hello/from/dev/test.tsx (non-default) |
| Transpile TS | true |
| Transpile JSX | true |
| Strip Event Handlers | true (non-default) |
| Strip Ctx Name | ["server"] (non-default) |

## Input

### Source Code
```tsx
import { component$, useStore, serverStuff$, $ } from '@qwik.dev/core';

export const App = component$(() => {
	const stuff = useStore();
	serverStuff$(async () => {
		// should be removed but keep scope
		console.log(stuff.count)
	})
	serverStuff$(async () => {
		// should be removed
	})

	return (
		<Cmp>
			<p class="stuff"
				shouldRemove$={() => stuff.count}
				onClick$={() => console.log('warn')}
			>
				Hello Qwik
			</p>
		</Cmp>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrlDEV(i_ckEPmXZlub0, "App_component_ckEPmXZlub0", {
    file: "/hello/from/dev/test.tsx",
    lo: 105,
    hi: 452,
    displayName: "test.tsx_App_component"
}));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _noopQrlDEV } from "@qwik.dev/core";
import { serverStuffQrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
export const App_component_ckEPmXZlub0 = ()=>{
    const stuff = useStore();
    serverStuffQrl(/*#__PURE__*/ _noopQrlDEV("App_component_serverStuff_ebyHaP15ytQ", {
        file: "/hello/from/dev/test.tsx",
        lo: 0,
        hi: 0,
        displayName: "test.tsx_App_component_serverStuff"
    }, [
        stuff
    ]));
    serverStuffQrl(/*#__PURE__*/ _noopQrlDEV("App_component_serverStuff_1_PQCqO0ANabY", {
        file: "/hello/from/dev/test.tsx",
        lo: 0,
        hi: 0,
        displayName: "test.tsx_App_component_serverStuff_1"
    }));
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        shouldRemove$: /*#__PURE__*/ _noopQrlDEV("App_component_Cmp_p_shouldRemove_uU0MG0jvQD4", {
            file: "/hello/from/dev/test.tsx",
            lo: 0,
            hi: 0,
            displayName: "test.tsx_App_component_Cmp_p_shouldRemove"
        }, [
            stuff
        ]),
        "q-e:click": /*#__PURE__*/ _noopQrlDEV("App_component_Cmp_p_q_e_click_Yl4ybrJWrt4", {
            file: "/hello/from/dev/test.tsx",
            lo: 0,
            hi: 0,
            displayName: "test.tsx_App_component_Cmp_p_q_e_click"
        })
    }, "Hello Qwik", 3, null, {
        fileName: "/hello/from/dev/test.tsx",
        lineNumber: 16,
        columnNumber: 4
    }), 3, "u6_0", {
        fileName: "/hello/from/dev/test.tsx",
        lineNumber: 15,
        columnNumber: 3
    });
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

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
  "loc": [105, 452]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls (Dev variant)**: `qrlDEV()` with custom `dev_path` in source metadata
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `serverStuff$` to `serverStuffQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` with dev source locations using custom dev_path
- **[CONV-06] Lazy Imports**: Dynamic import for component segment only
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all calls
- **[CONV-08] Segment Extraction**: Only component body extracted
- **[CONV-09] Code Stripping**: `_noopQrlDEV()` replaces stripped `serverStuff$` and event handlers. Captures preserved `[stuff]` even for stripped code

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrlDEV` | test.js | @qwik.dev/core | 1 |
| `_noopQrlDEV` | component entry | @qwik.dev/core | 4 |
| `serverStuffQrl` | component entry | @qwik.dev/core | 2 |
| `_jsxSorted` | component entry | @qwik.dev/core | 2 |
| `useStore` | component entry | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
