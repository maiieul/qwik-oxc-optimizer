# Test: example_strip_exports_used

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | false (default) |
| Transpile JSX | false (default) |
| Strip Exports | ["onGet"] (non-default) |

## Input

### Source Code
```tsx
import { component$, useResource$ } from '@qwik.dev/core';
import mongodb from 'mongodb';

export const onGet = () => {
	const data = mongodb.collection.whatever;
	return {
		body: {
		data
		}
	}
};

export default component$(()=> {
	useResource$(() => {
		return onGet();
	})
	return <div>cmp</div>
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
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export const onGet = ()=>{
    throw "Symbol removed by Qwik Optimizer, it can not be called from current platform";
};
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

### Module: test.tsx_test_component_useResource_4a8wVY7wh38.tsx (ENTRY POINT)

```tsx
import { onGet } from "./test";
export const test_component_useResource_4a8wVY7wh38 = ()=>{
    return onGet();
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_useResource_4a8wVY7wh38",
  "entry": null,
  "displayName": "test.tsx_test_component_useResource",
  "hash": "4a8wVY7wh38",
  "canonicalFilename": "test.tsx_test_component_useResource_4a8wVY7wh38",
  "path": "",
  "extension": "tsx",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "function",
  "ctxName": "useResource$",
  "captures": false,
  "loc": [250, 278]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
import { useResourceQrl } from "@qwik.dev/core";
const i_4a8wVY7wh38 = ()=>import("./test.tsx_test_component_useResource_4a8wVY7wh38");
export const test_component_LUXeXe0DQrg = ()=>{
    useResourceQrl(/*#__PURE__*/ qrl(i_4a8wVY7wh38, "test_component_useResource_4a8wVY7wh38"));
    return <div>cmp</div>;
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

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
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [229, 304]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module and component segment for `useResource$` reference
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useResource$` to `useResourceQrl()`
- **[CONV-06] Lazy Imports**: Dynamic imports for component and useResource segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, and `useResourceQrl()`
- **[CONV-08] Segment Extraction**: Component body and `useResource$` callback extracted to separate entry points
- **[CONV-09] Code Stripping (Export Stripping)**: `onGet` export body replaced with `throw "Symbol removed by Qwik Optimizer..."`. The `mongodb` import is tree-shaken

**Key difference from `example_strip_exports_unused`:** Here the component's `useResource$` callback calls `onGet()`. The `useResource$` segment imports `onGet` from `"./test"` (the main module). At runtime, calling `onGet()` will throw the "Symbol removed" error. This demonstrates that the optimizer does NOT follow cross-segment references when stripping -- it strips the export body but keeps the reference in the consuming segment, resulting in a runtime error if the stripped symbol is actually invoked on the wrong platform.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx, component entry | @qwik.dev/core | 2 |
| `useResourceQrl` | component entry | @qwik.dev/core | 1 |
| `onGet` | useResource entry | ./test | 1 |

## Diagnostics

```json
[]
```
