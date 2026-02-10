# Test: should_convert_rest_props

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useTask$ } from '@qwik.dev/core'

export default component$<any>(({ ...props }) => {
	useTask$(() => {
		props.checked
	})

	return 'hi'
})
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
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_component_useTask_jewzFYh3XmQ.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const test_component_useTask_jewzFYh3XmQ = ()=>{
    const props = _captures[0];
    props.checked;
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
  "name": "test_component_useTask_jewzFYh3XmQ",
  "entry": null,
  "displayName": "test.tsx_test_component_useTask",
  "hash": "jewzFYh3XmQ",
  "canonicalFilename": "test.tsx_test_component_useTask_jewzFYh3XmQ",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": true,
  "loc": [123, 151],
  "captureNames": ["props"]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _restProps } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const i_jewzFYh3XmQ = ()=>import("./test.tsx_test_component_useTask_jewzFYh3XmQ");
export const test_component_LUXeXe0DQrg = (_rawProps)=>{
    const props = _restProps(_rawProps);
    useTaskQrl(/*#__PURE__*/ qrl(i_jewzFYh3XmQ, "test_component_useTask_jewzFYh3XmQ", [
        props
    ]));
    return 'hi';
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
  "loc": [92, 171],
  "paramNames": ["_rawProps"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module and component segment with capture array for `useTask$`
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useTask$` to `useTaskQrl()`
- **[CONV-05] Capture Patterns**: `const props = _captures[0]` in useTask segment -- `props` (the rest proxy) is captured from component scope to be used in the task
- **[CONV-06] Lazy Imports**: Dynamic imports for component and task segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Component body and `useTask$` callback extracted as separate entry points
- **[CONV-11] Props Destructuring**: `({ ...props })` rest-only destructuring converted to `(_rawProps)` parameter with `const props = _restProps(_rawProps)` -- the `_restProps()` call with no excluded keys creates a proxy for all props. This is the pure rest-props case (no named props to exclude)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component entry | @qwik.dev/core | 2 |
| `_restProps` | component entry | @qwik.dev/core | 1 |
| `useTaskQrl` | component entry | @qwik.dev/core | 1 |
| `_captures` | useTask entry | @qwik.dev/core | 1 (array access) |

## Diagnostics

```json
[]
```
