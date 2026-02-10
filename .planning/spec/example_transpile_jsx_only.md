# Test: example_transpile_jsx_only

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | true |
| Explicit Extensions | true |

**Key insight:** JSX is transpiled to function calls, but TypeScript is preserved. Output modules use `.ts` extension (not `.tsx` because JSX has been removed, not `.js` because TS is preserved).

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$((props) => {
	return (
		<Cmp>
			<p class="stuff" onClick$={() => console.log('warn')}>Hello Qwik</p>
		</Cmp>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input parsed by OXC. 341 lines of AST JSON omitted for brevity. Key nodes: ImportDeclaration, ExportNamedDeclaration with VariableDeclaration (component$ call), JSXElement (Cmp > p).*

</details>

## Output

### Module: test.tsx_App_component_ckEPmXZlub0.ts (ENTRY POINT)

```typescript
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_Yl4ybrJWrt4 = ()=>import("./test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4.ts");
export const App_component_ckEPmXZlub0 = (props)=>{
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        "q-e:click": /*#__PURE__*/ qrl(i_Yl4ybrJWrt4, "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4")
    }, "Hello Qwik", 3, null), 3, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.ts`. Key nodes: ImportDeclaration (2x from @qwik.dev/core), VariableDeclaration (lazy import), ExportNamedDeclaration with arrow function containing _jsxSorted calls and qrl() call.*

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
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [88, 205],
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4.ts (ENTRY POINT)

```typescript
export const App_component_Cmp_p_q_e_click_Yl4ybrJWrt4 = ()=>console.log('warn');
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.ts`. Single ExportNamedDeclaration with arrow function calling console.log.*

</details>

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
  "extension": "ts",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [149, 174]
}
```

### Module: test.ts

```typescript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.ts");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.ts`. ImportDeclarations for componentQrl and qrl, lazy import declaration, ExportNamedDeclaration wrapping componentQrl(qrl(...)) with PURE annotations.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used in both the main module (`qrl(i_ckEPmXZlub0, ...)`) and entry point module (`qrl(i_Yl4ybrJWrt4, ...)`) to create lazy-loading references
- **[CONV-02] Dollar-to-Qrl**: `component$` transformed to `componentQrl` in the main module
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls in the entry point module (since `transpile_jsx: true`)
- **[CONV-06] Lazy Imports**: `const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.ts")` -- note `.ts` extension due to `explicit_extensions: true` and `transpile_jsx: true, transpile_ts: false`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()`, `qrl()`, and `_jsxSorted()` calls
- **[CONV-08] Segment Extraction**: Two entry point segments extracted: the component body and the click handler

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.ts | @qwik.dev/core | 1 |
| qrl | test.ts | @qwik.dev/core | 1 |
| qrl | App_component_ckEPmXZlub0.ts | @qwik.dev/core | 1 |
| _jsxSorted | App_component_ckEPmXZlub0.ts | @qwik.dev/core | 2 |
| console.log | Cmp_p_q_e_click_Yl4ybrJWrt4.ts | (global) | 1 |

## Diagnostics

```json
[]
```
