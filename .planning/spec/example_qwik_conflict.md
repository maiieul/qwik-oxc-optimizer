# Test: example_qwik_conflict

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Tests naming conflicts between user code and Qwik internals. The user declares `const componentQrl = () => ...` and uses `qrl` as both an import from `@qwik.dev/core/what` and a local variable. The optimizer renames conflicting identifiers: user's `componentQrl` -> `componentQrl1`, user's `qrl` import -> `qrl1`. Inside the Foo component's extracted segment, the local `const qrl = 23` is used in the click handler and gets constant-folded to `23` in the output.

## Input

### Source Code
```tsx
import { $, component$, useStyles } from '@qwik.dev/core';
import { qrl } from '@qwik.dev/core/what';

export const hW = 12;
export const handleWatch = 42;

const componentQrl = () => console.log('not this', qrl());

componentQrl();
export const Foo = component$(() => {
	useStyles$('thing');
	const qwik = hW + handleWatch;
	console.log(qwik);
	const qrl = 23;
	return (
		<div onClick$={()=> console.log(qrl)}/>
	)
}, {
	tagName: "my-foo",
});

export const Root = component$(() => {
	useStyles($('thing'));
	return $(() => {
		return (
			<div/>
		)
	});
}, {
	tagName: "my-foo",
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Multiple naming conflicts: user-defined componentQrl, qrl import from subpath, local qrl variable. Two components with tagName options.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
const i_royhjYaCbYE = ()=>import("./test.tsx_Root_component_royhjYaCbYE");
import { qrl as qrl1 } from '@qwik.dev/core/what';
export const hW = 12;
export const handleWatch = 42;
const componentQrl1 = ()=>console.log('not this', qrl1());
componentQrl1();
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"), {
    tagName: "my-foo"
});
export const Root = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_royhjYaCbYE, "Root_component_royhjYaCbYE"), {
    tagName: "my-foo"
});
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. User's `componentQrl` renamed to `componentQrl1`. User's `qrl` import renamed to `qrl1`. Optimizer's own componentQrl/qrl imported from @qwik.dev/core. tagName option passed as second argument.*

</details>

### Module: test.tsx_Foo_component_HTDRsvUbLiE.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { hW } from "./test";
import { handleWatch } from "./test";
import { qrl } from "@qwik.dev/core";
const i_YEa2A5ADUOg = ()=>import("./test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg");
export const Foo_component_HTDRsvUbLiE = ()=>{
    useStyles$('thing');
    const qwik = hW + handleWatch;
    console.log(qwik);
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_YEa2A5ADUOg, "Foo_component_div_q_e_click_YEa2A5ADUOg")
    }, null, 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Foo_component_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false
}
```

### Module: test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_click_YEa2A5ADUOg = ()=>console.log(23);
```

*Local `qrl = 23` constant-folded into the click handler output.*

### Module: test.tsx_Root_component_royhjYaCbYE.js (ENTRY POINT)

*(Component body with useStyles(qrl(...)) and inner $ callback returning div)*

### Module: test.tsx_Root_component_useStyles_u5DkUxGrGnU.js (ENTRY POINT)

```javascript
export const Root_component_useStyles_u5DkUxGrGnU = 'thing';
```

### Module: test.tsx_Root_component_1_cBpQNYDUHI4.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const Root_component_1_cBpQNYDUHI4 = ()=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_1");
};
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for segment references with lazy imports
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl` (both Foo and Root)
- **[CONV-03] JSX Transforms**: JSX transpiled to `_jsxSorted()` calls
- **[CONV-06] Lazy Imports**: Standard lazy import pattern for all segments
- **[CONV-07] PURE Annotations**: On componentQrl, qrl, _jsxSorted
- **[CONV-08] Segment Extraction**: 5 segments total across two components

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 2 |
| qrl | test.js / Foo_component.js / Root_component.js | @qwik.dev/core | 4 |
| _jsxSorted | Foo_component.js / Root_component_1.js | @qwik.dev/core | 2 |
| useStyles | Root_component.js | @qwik.dev/core | 1 |
| console.log | test.js / Foo_component.js / click handler | (global) | 3 |

## Diagnostics

```json
[]
```
