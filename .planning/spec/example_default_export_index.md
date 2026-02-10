# Test: example_default_export_index

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | src/components/mongo/index.tsx |
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |

**Key insight:** Default export from an `index.tsx` file. The component gets named `mongo_component` (derived from the parent directory name `mongo`, not the filename `index`). Inline strategy keeps all code in one module. JSX and TS are not transpiled, so JSX attributes use `q-e:click` syntax directly in JSX.

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

*TSX input. ExportDefaultDeclaration with component$, click handler referencing undefined `mongodb`.*

</details>

## Output

### Module: src/components/mongo/index.tsx (main module -- single output)

```tsx
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    return <div q-e:click={/*#__PURE__*/ inlinedQrl(()=>console.log(mongodb), "mongo_component_div_q_e_click_jncbxvZVtWY")}>
		</div>;
}, "mongo_component_ouWLj4jA2oI"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Inline strategy: all code in one module. JSX preserved with q-e:click attribute. inlinedQrl used instead of qrl + lazy imports.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` used (inline strategy keeps code in-place rather than lazy-loading)
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `q-e:click` attribute with `inlinedQrl`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on componentQrl and inlinedQrl calls
- **Note**: No CONV-03 (JSX transforms) -- JSX preserved since transpile_jsx is false
- **Note**: No CONV-06 (lazy imports) -- Inline strategy
- **Note**: No CONV-08 (segment extraction) -- Inline strategy

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | index.tsx | @qwik.dev/core | 1 |
| inlinedQrl | index.tsx | @qwik.dev/core | 2 |
| console.log | index.tsx | (global) | 1 |

## Diagnostics

```json
[]
```
