# Test: should_transform_event_names_without_jsx_transpile

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | false (default) |
| Transpile JSX | false (default) |

## Input

### Source Code
```tsx
import { component$, $ } from '@qwik.dev/core';
import mongo from 'mongodb';

export const Greeter = component$(() => {
	// Double count watch
	useTask$(async () => {
		await mongo.users();
	});
	return (
		<div>
			<div onClick$={() => {}}/>
			<div onClick$={() => {}}/>
			<div onClick$={() => {}}/>
		</div>
	)
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.tsx_Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo.tsx (ENTRY POINT)

```tsx
export const Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo = ()=>{};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_div_div_q_e_click_1",
  "hash": "s7p0zjWZpqo",
  "canonicalFilename": "test.tsx_Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo",
  "path": "",
  "extension": "tsx",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [263, 271]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_n7HuG2hhU0Q = ()=>import("./test.tsx_Greeter_component_n7HuG2hhU0Q");
export const Greeter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_n7HuG2hhU0Q, "Greeter_component_n7HuG2hhU0Q"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

### Module: test.tsx_Greeter_component_div_div_q_e_click_wYSPnQEGCbA.tsx (ENTRY POINT)

```tsx
export const Greeter_component_div_div_q_e_click_wYSPnQEGCbA = ()=>{};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_div_div_q_e_click_wYSPnQEGCbA",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_div_div_q_e_click",
  "hash": "wYSPnQEGCbA",
  "canonicalFilename": "test.tsx_Greeter_component_div_div_q_e_click_wYSPnQEGCbA",
  "path": "",
  "extension": "tsx",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [233, 241]
}
```

### Module: test.tsx_Greeter_component_div_div_q_e_click_2_B9tqzgApK9E.tsx (ENTRY POINT)

```tsx
export const Greeter_component_div_div_q_e_click_2_B9tqzgApK9E = ()=>{};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_div_div_q_e_click_2_B9tqzgApK9E",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_div_div_q_e_click_2",
  "hash": "B9tqzgApK9E",
  "canonicalFilename": "test.tsx_Greeter_component_div_div_q_e_click_2_B9tqzgApK9E",
  "path": "",
  "extension": "tsx",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [293, 301]
}
```

### Module: test.tsx_Greeter_component_n7HuG2hhU0Q.tsx (ENTRY POINT)

```tsx
import mongo from "mongodb";
import { qrl } from "@qwik.dev/core";
const i_B9tqzgApK9E = ()=>import("./test.tsx_Greeter_component_div_div_q_e_click_2_B9tqzgApK9E");
const i_s7p0zjWZpqo = ()=>import("./test.tsx_Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo");
const i_wYSPnQEGCbA = ()=>import("./test.tsx_Greeter_component_div_div_q_e_click_wYSPnQEGCbA");
export const Greeter_component_n7HuG2hhU0Q = ()=>{
    // Double count watch
    useTask$(async ()=>{
        await mongo.users();
    });
    return <div>
			<div q-e:click={/*#__PURE__*/ qrl(i_wYSPnQEGCbA, "Greeter_component_div_div_q_e_click_wYSPnQEGCbA")}/>
			<div q-e:click={/*#__PURE__*/ qrl(i_s7p0zjWZpqo, "Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo")}/>
			<div q-e:click={/*#__PURE__*/ qrl(i_B9tqzgApK9E, "Greeter_component_div_div_q_e_click_2_B9tqzgApK9E")}/>
		</div>;
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_n7HuG2hhU0Q",
  "entry": null,
  "displayName": "test.tsx_Greeter_component",
  "hash": "n7HuG2hhU0Q",
  "canonicalFilename": "test.tsx_Greeter_component_n7HuG2hhU0Q",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [114, 318]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` wraps lazy references for each event handler segment
- **[CONV-02] Dollar-to-QRL**: `component$` transformed to `componentQrl`; `onClick$` on native `<div>` elements transformed to `q-e:click` attribute
- **[CONV-06] Lazy Imports**: Lazy import statements for each extracted segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Component body and 3 click handlers extracted to separate entry point modules

**Key behavior**: Without JSX transpilation (`transpile_jsx: false`), JSX syntax is preserved in the output. Event handler props on native HTML elements (`<div>`) are transformed from `onClick$` to `q-e:click` attribute names. Multiple identical event types get numeric suffixes in their display names (`_q_e_click`, `_q_e_click_1`, `_q_e_click_2`).

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx, component module | @qwik.dev/core | 4 |

## Diagnostics

```json
[]
```
