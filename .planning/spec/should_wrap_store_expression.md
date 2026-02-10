# Test: should_wrap_store_expression

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |
| Minify | Simplify (default) |

## Input

### Source Code
```tsx
import { component$, useStore } from '@qwik.dev/core';

export default component$(() => {
	const panelStore = useStore(() => ({
		active: 'Input',
		list: PANELS,
	}));

	return (
		<div
			stuff={panelStore.active ? 'yes' : 'no'}
			class={{
				'too-long-to-wrap': true,
				'examples-panel-input': panelStore.active === 'Input',
				'examples-panel-output': panelStore.active === 'Output',
				'examples-panel-console': panelStore.active === 'Console',
			}}
		/>
	);
});
export const PANELS: ActivePanel[] = ['Examples', 'Input', 'Output', 'Console'];
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
export const PANELS = [
    'Examples',
    'Input',
    'Output',
    'Console'
];
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { PANELS } from "./test";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.active ? 'yes' : 'no';
const _hf0_str = 'p0.active?"yes":"no"';
export const test_component_LUXeXe0DQrg = ()=>{
    const panelStore = useStore(()=>({
            active: 'Input',
            list: PANELS
        }));
    return /*#__PURE__*/ _jsxSorted("div", {
        class: {
            'too-long-to-wrap': true,
            'examples-panel-input': panelStore.active === 'Input',
            'examples-panel-output': panelStore.active === 'Output',
            'examples-panel-console': panelStore.active === 'Console'
        }
    }, {
        stuff: _fnSignal(_hf0, [
            panelStore
        ], _hf0_str)
    }, null, 3, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null,
  "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg",
  "canonicalFilename": "test.tsx_test_component_LUXeXe0DQrg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [88, 510]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used in main module to create lazy-loadable reference to component segment
- **[CONV-02] Dollar-to-Qrl**: `component$` transformed to `componentQrl()` in main module
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` replaces JSX `<div ... />` in entry point module
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [panelStore], _hf0_str)` wraps the ternary `panelStore.active ? 'yes' : 'no'` as a reactive signal for the `stuff` prop
- **[CONV-06] Lazy Imports**: `const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg")` creates dynamic import for segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()`, `qrl()`, and `_jsxSorted()` calls
- **[CONV-08] Segment Extraction**: Component body extracted to `test.tsx_test_component_LUXeXe0DQrg.js` entry point module
- **[CONV-14] Hoisted Functions**: `const _hf0 = (p0)=>p0.active ? 'yes' : 'no'` with `_hf0_str` stringification -- ternary expression hoisted out of JSX for reactive tracking

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 1 |
| `_fnSignal` | entry point | @qwik.dev/core | 1 |
| `useStore` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
