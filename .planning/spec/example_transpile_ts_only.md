# Test: example_transpile_ts_only

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | false |
| Explicit Extensions | true |

**Key insight:** TypeScript is transpiled away but JSX is preserved. With Inline strategy, all code stays in one module. Output uses `.jsx` extension (TS removed, JSX retained). The `inlinedQrl` pattern replaces `qrl` + lazy imports.

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$((props: Stuff) => {
	return (
		<Cmp>
			<p class="stuff" onClick$={() => console.log('warn')}>Hello Qwik</p>
		</Cmp>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Key nodes: ImportDeclaration, ExportNamedDeclaration with component$ call containing TypeScript type annotation `props: Stuff`, JSXElement tree.*

</details>

## Output

### Module: test.jsx (main module)

```jsx
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    return <Cmp>
			<p class="stuff" q-e:click={/*#__PURE__*/ inlinedQrl(()=>console.log('warn'), "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4")}>Hello Qwik</p>
		</Cmp>;
}, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.jsx`. TypeScript type annotation `Stuff` removed. JSX elements preserved as-is. `inlinedQrl` wraps the component body and event handler inline instead of extracting to separate segments.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` used instead of `qrl()` because of Inline entry strategy -- code stays inline rather than being extracted
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, event handler `onClick$` -> `q-e:click` attribute with `inlinedQrl`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()` and `inlinedQrl()` calls
- **Note**: No CONV-03 (JSX transforms) because `transpile_jsx: false` -- JSX angle bracket syntax is preserved
- **Note**: No CONV-06 (lazy imports) because Inline strategy keeps all code in one module
- **Note**: No CONV-08 (segment extraction) because Inline strategy does not extract segments

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.jsx | @qwik.dev/core | 1 |
| inlinedQrl | test.jsx | @qwik.dev/core | 2 |
| console.log | test.jsx | (global) | 1 |

## Diagnostics

```json
[]
```
