# Test: impure_template_fns

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | false (default) |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useSignal } from '@qwik.dev/core';
const useFoo = (count) => {
	const tag = (s) => {
		const value = typeof s === "string" ? s : s[0];
		return `${value}-${count.value}`;
	}
	return tag;
}

export default component$(() => {
	const count = useSignal(0);
	const foo = useFoo(count);
	return (
		<>
			<p>{foo("test")}</p>
			<p>{foo`test`}</p>
			<button onClick$={() => count.value++}>Count up</button>
		</>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw.ts (ENTRY POINT)

```typescript
import { _captures } from "@qwik.dev/core";
export const test_component_Fragment_button_q_e_click_7MTd2pAiliw = ()=>{
    const count = _captures[0];
    return count.value++;
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_Fragment_button_q_e_click_7MTd2pAiliw",
  "entry": null,
  "displayName": "test.tsx_test_component_Fragment_button_q_e_click",
  "hash": "7MTd2pAiliw",
  "canonicalFilename": "test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw",
  "path": "",
  "extension": "ts",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [418, 437],
  "captureNames": ["count"]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.ts (ENTRY POINT)

```typescript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { _auto_useFoo as useFoo } from "./test";
import { useSignal } from "@qwik.dev/core";
const i_7MTd2pAiliw = ()=>import("./test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw");
export const test_component_LUXeXe0DQrg = ()=>{
    const count = useSignal(0);
    const foo = useFoo(count);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("p", null, null, foo("test"), 1, null),
        /*#__PURE__*/ _jsxSorted("p", null, null, foo`test`, 1, null),
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_7MTd2pAiliw, "test_component_Fragment_button_q_e_click_7MTd2pAiliw", [count])
        }, "Count up", 3, null)
    ], 1, "u6_0");
};
```

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
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [257, 474]
}
```

### Module: test.ts (main)

```typescript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
const useFoo = (count)=>{
    const tag = (s)=>{
        const value = typeof s === "string" ? s : s[0];
        return `${value}-${count.value}`;
    };
    return tag;
};
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
export { useFoo as _auto_useFoo };
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util ts`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy, with captures `[count]` for click handler
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `onClick$` to `q-e:click` on native `<button>`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; `foo("test")` and `` foo`test` `` passed directly (not wrapped) with flag=1
- **[CONV-05] Capture Patterns**: `_captures[0]` in click handler to restore `count`
- **[CONV-06] Lazy Imports**: Lazy imports for component and click handler segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: Component body and click handler extracted; main module retains `useFoo` function with `_auto_` prefix export

**Key behavior**: The `useFoo` function is a closure that captures `count.value` -- its return value (`tag`) is impure because it reads signal values. The optimizer does NOT wrap `foo("test")` or `` foo`test` `` with `_fnSignal` because function call results are inherently non-trackable (flag=1, mutable). The function itself is moved to the main module with an `_auto_useFoo` export alias so the extracted component segment can import it. This `_auto_` prefix pattern is how the optimizer makes non-exported local functions available to extracted segments.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.ts | @qwik.dev/core | 1 |
| `qrl` | test.ts, component segment | @qwik.dev/core | 2 |
| `_jsxSorted` | component segment | @qwik.dev/core | 4 |
| `_captures` | click handler | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
