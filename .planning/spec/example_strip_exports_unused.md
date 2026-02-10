# Test: example_strip_exports_unused

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
import { component$ } from '@qwik.dev/core';
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

### Module: test.tsx_test_component_LUXeXe0DQrg.tsx (ENTRY POINT)

```tsx
export const test_component_LUXeXe0DQrg = ()=>{
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
  "loc": [215, 246]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module for component reference
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()` and `qrl()`
- **[CONV-08] Segment Extraction**: Component body extracted
- **[CONV-09] Code Stripping (Export Stripping)**: The `onGet` export is replaced with a stub that throws `"Symbol removed by Qwik Optimizer, it can not be called from current platform"`. The original `onGet` body is completely removed. The `mongodb` import is tree-shaken because it was only used inside the stripped `onGet` body

**Key behavior:** `strip_exports: ["onGet"]` causes:
1. The `onGet` export is preserved (the export declaration remains) but its body is replaced with a `throw` statement
2. The error message indicates the symbol was intentionally removed and cannot be called on the current platform
3. All dependencies used exclusively by `onGet` (the `mongodb` import) are removed
4. The component is unaffected -- the `onGet` was "unused" by the component in this test (compare with `example_strip_exports_used` where the component references `onGet`)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
