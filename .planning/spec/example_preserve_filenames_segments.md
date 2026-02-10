# Test: example_preserve_filenames_segments

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |
| Preserve Filenames | true |
| Explicit Extensions | true |

**Key insight:** Tests `preserve_filenames: true` with Segment strategy where the effect is visible. Segment files use the original filename in their paths. Import paths in segments include `.js` extensions (explicit_extensions). The `foo` function is exported from the main module and imported by the component segment via `import { foo } from "./test.tsx"`. TypeScript type annotation `props: Stuff` is stripped (transpile_ts: true).

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$((props: Stuff) => {
	foo();
	return (
		<Cmp>
			<p class="stuff" onClick$={() => console.log('warn')}>Hello Qwik</p>
		</Cmp>
	);
});

export const foo = () => console.log('foo');
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Component with TypeScript type annotation on props (Stuff). Calls external foo() function. JSX with custom component Cmp, click handler. Separate exported foo arrow function.*

</details>

## Output

### Module: test.tsx (main module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.js");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
export const foo = ()=>console.log('foo');
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Lazy import path includes `.js` extension (explicit_extensions: true). foo function stays in main module. Component transformed to componentQrl with qrl lazy reference.*

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { foo } from "./test.tsx";
import { qrl } from "@qwik.dev/core";
const i_Yl4ybrJWrt4 = ()=>import("./test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4.js");
export const App_component_ckEPmXZlub0 = (props)=>{
    foo();
    return /*#__PURE__*/ _jsxSorted(Cmp, null, null, /*#__PURE__*/ _jsxSorted("p", null, {
        class: "stuff",
        "q-e:click": /*#__PURE__*/ qrl(i_Yl4ybrJWrt4, "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4")
    }, "Hello Qwik", 3, null), 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "paramNames": ["props"]
}
```

### Module: test.tsx_App_component_Cmp_p_q_e_click_Yl4ybrJWrt4.js (ENTRY POINT)

```javascript
export const App_component_Cmp_p_q_e_click_Yl4ybrJWrt4 = ()=>console.log('warn');
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_Cmp_p_q_e_click_Yl4ybrJWrt4",
  "entry": null,
  "displayName": "test.tsx_App_component_Cmp_p_q_e_click",
  "hash": "Yl4ybrJWrt4",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for both component body and click handler segment references
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `onClick$` -> `"q-e:click"` event binding
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls
- **[CONV-06] Lazy Imports**: Lazy import paths include `.js` extension due to explicit_extensions; import from main module uses `./test.tsx` (preserving original filename)
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted calls
- **[CONV-08] Segment Extraction**: Two segments: component body and click handler
- **Preserve Filenames**: Import path `"./test.tsx"` preserves original filename in segment's import of `foo`

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.tsx | @qwik.dev/core | 1 |
| qrl | test.tsx / App_component.js | @qwik.dev/core | 2 |
| _jsxSorted | App_component.js | @qwik.dev/core | 2 |
| foo | App_component.js | ./test.tsx | 1 |
| console.log | click handler / test.tsx | (global) | 2 |

## Diagnostics

```json
[]
```
