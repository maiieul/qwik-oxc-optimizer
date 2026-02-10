# Test: example_default_export_invalid_ident

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | src/components/mongo/404.tsx |
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |

**Key insight:** The filename `404.tsx` starts with a digit, which is not a valid JavaScript identifier. The optimizer prefixes it with `_` to create `_404_component`. This demonstrates how invalid identifier characters in filenames are handled for segment naming. JSX is preserved (no transpile flags).

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';

export default component$(() => {
	return (
		<div onClick$={() => console.log(mongodb)}>
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Same code as example_default_export_index but with different filename.*

</details>

## Output

### Module: src/components/mongo/404.tsx__404_component_div_q_e_click_aMLnLWtkRhc.tsx (ENTRY POINT)

```tsx
export const _404_component_div_q_e_click_aMLnLWtkRhc = ()=>console.log(mongodb);
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Simple arrow function. Name prefixed with `_` because `404` is not a valid identifier start.*

</details>

#### Segment Metadata
```json
{
  "origin": "src/components/mongo/404.tsx",
  "name": "_404_component_div_q_e_click_aMLnLWtkRhc",
  "entry": null,
  "displayName": "404.tsx__404_component_div_q_e_click",
  "hash": "aMLnLWtkRhc",
  "canonicalFilename": "404.tsx__404_component_div_q_e_click_aMLnLWtkRhc",
  "path": "src/components/mongo",
  "extension": "tsx",
  "parent": "_404_component_zRvoWc98eqo",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [109, 135]
}
```

### Module: src/components/mongo/404.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_zRvoWc98eqo = ()=>import("./404.tsx__404_component_zRvoWc98eqo");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_zRvoWc98eqo, "_404_component_zRvoWc98eqo"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. ExportDefaultDeclaration with componentQrl. Segment name uses `_404_component` prefix.*

</details>

### Module: 404.tsx__404_component_zRvoWc98eqo.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
const i_aMLnLWtkRhc = ()=>import("./404.tsx__404_component_div_q_e_click_aMLnLWtkRhc");
export const _404_component_zRvoWc98eqo = ()=>{
    return <div q-e:click={/*#__PURE__*/ qrl(i_aMLnLWtkRhc, "_404_component_div_q_e_click_aMLnLWtkRhc")}>
		</div>;
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. JSX preserved with q-e:click attribute. qrl reference to click handler segment.*

</details>

#### Segment Metadata
```json
{
  "origin": "src/components/mongo/404.tsx",
  "name": "_404_component_zRvoWc98eqo",
  "entry": null,
  "displayName": "404.tsx__404_component",
  "hash": "zRvoWc98eqo",
  "canonicalFilename": "404.tsx__404_component_zRvoWc98eqo",
  "path": "src/components/mongo",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [74, 152]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for lazy segment references
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `q-e:click` with qrl
- **[CONV-06] Lazy Imports**: Standard lazy import pattern
- **[CONV-07] PURE Annotations**: On componentQrl and qrl calls
- **[CONV-08] Segment Extraction**: Two segments extracted (component body and click handler)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | 404.tsx | @qwik.dev/core | 1 |
| qrl | 404.tsx | @qwik.dev/core | 1 |
| qrl | _404_component_zRvoWc98eqo.tsx | @qwik.dev/core | 1 |
| console.log | _404_component_div_q_e_click.tsx | (global) | 1 |

## Diagnostics

```json
[]
```
