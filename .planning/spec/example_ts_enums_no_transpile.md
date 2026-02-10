# Test: example_ts_enums_no_transpile

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |

**Key insight:** With neither transpile flag set, TypeScript enums are preserved as-is in the output (the `export enum Thing { A, B }` syntax stays). The extracted segment imports `Thing` from the main module and uses `Thing.A` directly instead of inlining the value `0`. JSX is also preserved. Output uses `.tsx` extension.

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

*TSX input. Exported TSEnumDeclaration and component$ with JSX Fragment.*

</details>

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export enum Thing {
    A,
    B
}
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. TS enum declaration preserved verbatim. Standard componentQrl/qrl pattern.*

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.tsx (ENTRY POINT)

```tsx
import { Thing } from "./test";
export const App_component_ckEPmXZlub0 = ()=>{
    console.log(Thing.A);
    return <>
			<p class="stuff">Hello Qwik</p>
		</>;
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Imports Thing from main module (not inlined). JSX Fragment preserved. Uses Thing.A member expression (not constant-folded).*

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
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [118, 210]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for lazy segment reference
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`
- **[CONV-06] Lazy Imports**: Standard lazy import pattern (no extension since explicit_extensions is false)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on componentQrl and qrl calls
- **[CONV-08] Segment Extraction**: Component body extracted, imports `Thing` from main module
- **Note**: No CONV-03 (JSX transforms) -- JSX preserved as-is since transpile_jsx is false

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.tsx | @qwik.dev/core | 1 |
| qrl | test.tsx | @qwik.dev/core | 1 |
| console.log | App_component_ckEPmXZlub0.tsx | (global) | 1 |

## Diagnostics

```json
[]
```
