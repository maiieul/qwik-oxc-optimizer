# Test: lib_mode_fn_signal

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | false (default) |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';
export const Counter = component$(() => {
	const count = useSignal(0);

	return (
		<div>
			<p>Count: {count.value}</p>
			<p>
				<button onClick$={() => count.value++}>Increment</button>
			</p>
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.tsx_Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI.ts (ENTRY POINT)

```typescript
import { _captures } from "@qwik.dev/core";
export const Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI = ()=>{
    const count = _captures[0];
    return count.value++;
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI",
  "entry": null,
  "displayName": "test.tsx_Counter_component_div_p_button_q_e_click",
  "hash": "Pq1pmfmJWUI",
  "canonicalFilename": "test.tsx_Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI",
  "path": "",
  "extension": "ts",
  "parent": "Counter_component_zTmRHlL09Gg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [213, 232],
  "captureNames": ["count"]
}
```

### Module: test.tsx_Counter_component_zTmRHlL09Gg.ts (ENTRY POINT)

```typescript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_Pq1pmfmJWUI = ()=>import("./test.tsx_Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI");
export const Counter_component_zTmRHlL09Gg = ()=>{
    const count = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("p", null, null, [
            "Count: ",
            _wrapProp(count)
        ], 3, null),
        /*#__PURE__*/ _jsxSorted("p", null, null, /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_Pq1pmfmJWUI, "Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI", [count])
        }, "Increment", 3, null), 3, null)
    ], 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Counter_component_zTmRHlL09Gg",
  "entry": null,
  "displayName": "test.tsx_Counter_component",
  "hash": "zTmRHlL09Gg",
  "canonicalFilename": "test.tsx_Counter_component_zTmRHlL09Gg",
  "path": "",
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [85, 283]
}
```

### Module: test.ts (main)

```typescript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_zTmRHlL09Gg = ()=>import("./test.tsx_Counter_component_zTmRHlL09Gg");
export const Counter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_zTmRHlL09Gg, "Counter_component_zTmRHlL09Gg"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util ts`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy with captures `[count]`
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `onClick$` to `q-e:click`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements
- **[CONV-04] Signal Helpers**: `_wrapProp(count)` for `count.value` (single-arg form wraps signal's `.value`)
- **[CONV-05] Capture Patterns**: `_captures[0]` restores `count` in click handler
- **[CONV-06] Lazy Imports**: Lazy imports for component and click handler segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-08] Segment Extraction**: Component body and click handler extracted

**Key behavior**: Note the output file extensions: main module is `.ts` (not `.js`) and segments are also `.ts` because `transpile_jsx: true` strips JSX but `transpile_ts: false (default)` preserves TypeScript. The main module has no JSX but the segment module uses `_jsxSorted()` function calls (transpiled JSX). `_wrapProp(count)` with a single argument wraps the signal's `.value` property for child text.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.ts | @qwik.dev/core | 1 |
| `qrl` | test.ts, component segment | @qwik.dev/core | 2 |
| `_jsxSorted` | component segment | @qwik.dev/core | 4 |
| `_wrapProp` | component segment | @qwik.dev/core | 1 |
| `_captures` | click handler | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
