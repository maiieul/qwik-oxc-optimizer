# Test: example_immutable_function_components

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist |
| Transpile TS | true |
| Transpile JSX | true |
| Explicit Extensions | true |

## Input

### Source Code
```tsx
import { component$, useStore, Slot } from '@qwik.dev/core';

export const App = component$((props: Stuff) => {
	return (
		<div>
			<Slot/>
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main, Inline -- single module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { Slot } from '@qwik.dev/core';
const App_component_ckEPmXZlub0 = (props)=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, /*#__PURE__*/ _jsxSorted(Slot, null, null, null, 3, "u6_0"), 1, "u6_1");
};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Hoist strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<div>` and `<Slot/>`; TypeScript type annotation `(props: Stuff)` stripped from output
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl`, `inlinedQrl`, `_jsxSorted`

**Key behavior**: Simple component with `<Slot/>`. The Slot component is imported and used directly (not extracted as a segment). The TypeScript type annotation `Stuff` is stripped when `transpile_ts: true`. The `Slot` component gets flag=3 (immutable) and a dev key. The `<div>` wrapper gets flag=1 (mutable children since Slot content can change).

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 2 |

## Diagnostics

None (`[]`)
