# Test: example_strip_client_code

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | components/component.tsx (non-default) |
| Entry Strategy | Inline (non-default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |
| Strip Ctx Name | ["useClientMount$"] (non-default) |
| Strip Event Handlers | true (non-default) |

## Input

### Source Code
```tsx
import { component$, useClientMount$, useStore, useTask$ } from '@qwik.dev/core';
import mongo from 'mongodb';
import redis from 'redis';
import threejs from 'threejs';
import { a } from './keep';
import { b } from '../keep2';
import { c } from '../../remove';

export const Parent = component$(() => {
	const state = useStore({
		text: ''
	});

	// Double count watch
	useClientMount$(async () => {
		state.text = await mongo.users();
		redis.set(state.text, a, b, c);
	});

	useTask$(() => {
		// Code
	});

	return (
		<div
			shouldRemove$={() => state.text}
			onClick$={() => console.log('parent', state, threejs)}
		>
			<Div
				onClick$={() => console.log('keep')}
				render$={() => state.text}
			/>
			{state.text}
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: components/component.js

```javascript
import "./keep";
import "../keep2";
import { componentQrl } from "@qwik.dev/core";
import { useClientMountQrl } from "@qwik.dev/core";
import { _noopQrl } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { useStore } from '@qwik.dev/core';
export const Parent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    const state = useStore({
        text: ''
    });
    // Double count watch
    useClientMountQrl(/*#__PURE__*/ _noopQrl("Parent_component_useClientMount_Yn2kIDABoYw", [
        state
    ]));
    useTaskQrl(/*#__PURE__*/ inlinedQrl(()=>{
    // Code
    }, "Parent_component_useTask_ngmvcygWux8"));
    return /*#__PURE__*/ _jsxSorted("div", null, {
        shouldRemove$: /*#__PURE__*/ _noopQrl("Parent_component_div_shouldRemove_EBj69wTX1do", [
            state
        ]),
        "q-e:click": /*#__PURE__*/ _noopQrl("Parent_component_div_q_e_click_oqNnfO6ubjU", [
            state
        ])
    }, [
        /*#__PURE__*/ _jsxSorted(Div, null, {
            onClick$: /*#__PURE__*/ inlinedQrl(()=>console.log('keep'), "Parent_component_div_Div_onClick_kgowuto5dR0"),
            render$: /*#__PURE__*/ inlinedQrl(()=>{
                const state = _captures[0];
                return state.text;
            }, "Parent_component_div_Div_render_CkMybN6xzQk", [
                state
            ])
        }, null, 3, "7R_0"),
        _wrapProp(state, "text")
    ], 1, "7R_1");
}, "Parent_component_t6Wy3C0Q0XM"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` used because of Inline entry strategy -- all code inlined in single module. No dynamic imports
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useClientMount$` to `useClientMountQrl()`, `useTask$` to `useTaskQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` and `_jsxSorted(Div, ...)` replace JSX
- **[CONV-04] Signal Helpers**: `_wrapProp(state, "text")` wraps `state.text` for reactive child rendering
- **[CONV-05] Capture Patterns**: `const state = _captures[0]` in the `render$` callback restores the `state` variable from captures. The `Div`'s `onClick$` has no captures (only uses string literal)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `inlinedQrl()`, `_noopQrl()`, `_jsxSorted()`
- **[CONV-09] Code Stripping**:
  - `useClientMount$` callback stripped to `_noopQrl("...", [state])` because `strip_ctx_name: ["useClientMount$"]`
  - `shouldRemove$` and `onClick$` on the outer `<div>` stripped to `_noopQrl()` because `strip_event_handlers: true`
  - `Div`'s `onClick$` is NOT stripped -- it uses `inlinedQrl()` with the actual handler, because `Div` is a custom component (not a host element), so `strip_event_handlers` does not apply to custom component prop handlers
  - Imports tree-shaken: `mongo`, `redis`, `threejs`, `c` from '../../remove' all removed because they're only used in stripped code
  - Imports kept: `./keep` and `../keep2` are kept as bare imports (side-effect only) because they exist in the component scope
  - The `_noopQrl` calls preserve capture arrays `[state]` even though the code is stripped

**Key behavior:** The `Inline` entry strategy combined with `strip_event_handlers` and `strip_ctx_name` produces a single-file output where stripped handlers become `_noopQrl()` placeholders while non-stripped handlers use `inlinedQrl()`. The file path in the module name reflects the custom filename `components/component.tsx` (hash suffix `7R` vs default `u6`).

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | component.js | @qwik.dev/core | 1 |
| `inlinedQrl` | component.js | @qwik.dev/core | 4 |
| `_noopQrl` | component.js | @qwik.dev/core | 3 |
| `useClientMountQrl` | component.js | @qwik.dev/core | 1 |
| `useTaskQrl` | component.js | @qwik.dev/core | 1 |
| `useStore` | component.js | @qwik.dev/core | 1 |
| `_jsxSorted` | component.js | @qwik.dev/core | 2 |
| `_wrapProp` | component.js | @qwik.dev/core | 1 |
| `_captures` | component.js | @qwik.dev/core | 1 (array access) |

## Diagnostics

```json
[]
```
