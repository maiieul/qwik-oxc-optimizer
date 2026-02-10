# Test: should_not_transform_events_on_non_elements

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | false (default) |
| Transpile JSX | false (default) |

## Input

### Source Code
```tsx
import { component$, $ } from '@qwik.dev/core';
import { CustomComponent } from './custom-component';
import { AnotherComponent } from './another-component';

export const Greeter = component$(() => {
	return (
		<div>
			<CustomComponent onClick$={() => {}}/>
			{array.map(item => (
				<AnotherComponent onClick$={() => {}}/>
			))}
		</div>
	)
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.tsx_Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA.tsx (ENTRY POINT)

```tsx
export const Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA = ()=>{};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_div_AnotherComponent_onClick",
  "hash": "9BwXJW3s0yA",
  "canonicalFilename": "test.tsx_Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA",
  "path": "",
  "extension": "tsx",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [319, 327]
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

### Module: test.tsx_Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w.tsx (ENTRY POINT)

```tsx
export const Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w = ()=>{};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_div_CustomComponent_onClick",
  "hash": "6xTF8kMcS9w",
  "canonicalFilename": "test.tsx_Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w",
  "path": "",
  "extension": "tsx",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [251, 259]
}
```

### Module: test.tsx_Greeter_component_n7HuG2hhU0Q.tsx (ENTRY POINT)

```tsx
import { AnotherComponent } from "./another-component";
import { CustomComponent } from "./custom-component";
import { qrl } from "@qwik.dev/core";
const i_6xTF8kMcS9w = ()=>import("./test.tsx_Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w");
const i_9BwXJW3s0yA = ()=>import("./test.tsx_Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA");
export const Greeter_component_n7HuG2hhU0Q = ()=>{
    return <div>
			<CustomComponent onClick$={/*#__PURE__*/ qrl(i_6xTF8kMcS9w, "Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w")}/>
			{array.map((item)=><AnotherComponent onClick$={/*#__PURE__*/ qrl(i_9BwXJW3s0yA, "Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA")}/>)}
		</div>;
};
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

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
  "loc": [195, 351]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used to wrap lazy references for event handlers on components (not native elements)
- **[CONV-02] Dollar-to-QRL**: `component$` transformed to `componentQrl`
- **[CONV-06] Lazy Imports**: `const i_n7HuG2hhU0Q = ()=>import(...)` for lazy-loading segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` before `componentQrl()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Component body and event handlers extracted to separate entry point modules

**Key behavior**: Event handlers on non-element components (`CustomComponent`, `AnotherComponent`) are NOT transformed to `q-e:click` form. They remain as `onClick$` prop with a QRL value. This is the opposite of what happens with native HTML elements (like `<div>`), where `onClick$` becomes `q-e:click`. This test verifies that component event props are preserved as-is since the component needs to receive them as regular props.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx, component module | @qwik.dev/core | 3 |

## Diagnostics

```json
[]
```
