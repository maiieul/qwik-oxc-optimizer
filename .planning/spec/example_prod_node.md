# Test: example_prod_node

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Prod (non-default) |
| Transpile TS | false (default) |
| Transpile JSX | false (default) |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';

export const Foo = component$(() => {
	return (
		<div>
			<div onClick$={() => console.log('first')}/>
			<div onClick$={() => console.log('second')}/>
			<div onClick$={() => console.log('third')}/>
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "s_HTDRsvUbLiE"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

### Module: test.tsx_Foo_component_HTDRsvUbLiE.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
const i_VSoqbTjzr4w = ()=>import("./test.tsx_Foo_component_div_div_q_e_click_1_VSoqbTjzr4w");
const i_n19LdlqL6To = ()=>import("./test.tsx_Foo_component_div_div_q_e_click_2_n19LdlqL6To");
const i_vKrX4PmH2aM = ()=>import("./test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM");
export const s_HTDRsvUbLiE = ()=>{
    return <div>
			<div q-e:click={/*#__PURE__*/ qrl(i_vKrX4PmH2aM, "s_vKrX4PmH2aM")}/>
			<div q-e:click={/*#__PURE__*/ qrl(i_VSoqbTjzr4w, "s_VSoqbTjzr4w")}/>
			<div q-e:click={/*#__PURE__*/ qrl(i_n19LdlqL6To, "s_n19LdlqL6To")}/>
		</div>;
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
  "name": "s_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [78, 263]
}
```

### Module: test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.tsx (ENTRY POINT)

```tsx
export const s_vKrX4PmH2aM = ()=>console.log('first');
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

### Module: test.tsx_Foo_component_div_div_q_e_click_1_VSoqbTjzr4w.tsx (ENTRY POINT)

```tsx
export const s_VSoqbTjzr4w = ()=>console.log('second');
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

### Module: test.tsx_Foo_component_div_div_q_e_click_2_n19LdlqL6To.tsx (ENTRY POINT)

```tsx
export const s_n19LdlqL6To = ()=>console.log('third');
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` with `s_HASH` naming (Prod mode) -- no dev metadata
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-06] Lazy Imports**: Dynamic imports for all segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on calls
- **[CONV-08] Segment Extraction**: Component and three onClick handlers extracted

**Key Prod behavior:** `s_HASH` names, no dev metadata, `.tsx` output (no JSX transpilation).

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx, component entry | @qwik.dev/core | 4 |

## Diagnostics

```json
[]
```
