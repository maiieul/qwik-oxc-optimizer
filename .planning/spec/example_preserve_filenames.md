# Test: example_preserve_filenames

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | true |
| Preserve Filenames | true |
| Explicit Extensions | true |

**Key insight:** Tests the `preserve_filenames: true` option with Inline strategy. Since Inline strategy keeps all code in a single module (no segments extracted), the preserve_filenames option has minimal visible effect. The output is a single module with `inlinedQrl` calls. JSX is transpiled to `_jsxSorted` but TypeScript is preserved. The onClick$ handler is inlined as a nested `inlinedQrl` within the component's `inlinedQrl`.

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

*TSX input. Single component with JSX containing a custom component Cmp, a paragraph with class, onClick$ handler, and text content.*

</details>

## Output

### Module: test.tsx

```typescript
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        "q-e:click": /*#__PURE__*/ inlinedQrl(()=>console.log('warn'), "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4")
    }, "Hello Qwik", 3, null), 3, "u6_0");
}, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx` (transpile_ts: false preserves TS syntax). All code inline via inlinedQrl. JSX transpiled to _jsxSorted. onClick$ converted to "q-e:click" event binding with nested inlinedQrl. No segments extracted.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for both component body and event handler (Inline strategy)
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `"q-e:click"` event binding
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls with element, immutable props, mutable events, children, flags, dev key
- **[CONV-07] PURE Annotations**: On componentQrl, inlinedQrl, _jsxSorted calls
- **Note**: No CONV-06/08 -- Inline strategy keeps all code in one module, no segment extraction or lazy imports
- **Note**: preserve_filenames has no visible effect with Inline strategy since no segment files are created

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.tsx | @qwik.dev/core | 1 |
| inlinedQrl | test.tsx | @qwik.dev/core | 2 |
| _jsxSorted | test.tsx | @qwik.dev/core | 2 |
| console.log | test.tsx | (global) | 1 |

## Diagnostics

```json
[]
```
