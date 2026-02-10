# Test: example_exports

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | project/test.tsx |
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | false |

**Key insight:** Tests comprehensive export patterns: destructured exports (`const [a, {b, ...}]`), named exports (`export {exp1, internal as expr2}`), function/class exports, default function export, and component$. The extracted segment imports all referenced values from the main module. Since `transpile_jsx: false` but `transpile_ts: true`, output uses `.jsx` extension (JSX preserved, TS removed).

## Input

### Source Code
```tsx
import { $, component$ } from '@qwik.dev/core';

export const [a, {b, v1: [c], d=v2, ...e}, f=v3, ...g] = obj;

const exp1 = 1;
const internal = 2;
export {exp1, internal as expr2};

export function foo() { }
export class bar {}

export default function DefaultFn() {}

export const Header = component$(() => {
	return $(() => (
		<Footer>
			<div>{a}{b}{c}{d}{e}{f}{exp1}{internal}{foo}{bar}{DefaultFn}</div>
			<div>{v1}{v2}{v3}{obj}</div>
		</Footer>
	))
});

export const Footer = component$();
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Complex destructured export, named re-exports, function/class exports, default function export, two component$ declarations.*

</details>

## Output

### Module: project/test.tsx_Header_component_UVBJuFYfvDo.jsx (ENTRY POINT)

```jsx
import { qrl } from "@qwik.dev/core";
const i_uWM1kg0IGO0 = ()=>import("./test.tsx_Header_component_1_uWM1kg0IGO0");
export const Header_component_UVBJuFYfvDo = ()=>{
    return /*#__PURE__*/ qrl(i_uWM1kg0IGO0, "Header_component_1_uWM1kg0IGO0");
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.jsx`. Component body returns a qrl reference to the inner $ callback.*

</details>

#### Segment Metadata
```json
{
  "origin": "project/test.tsx",
  "name": "Header_component_UVBJuFYfvDo",
  "entry": null,
  "displayName": "test.tsx_Header_component",
  "hash": "UVBJuFYfvDo",
  "canonicalFilename": "test.tsx_Header_component_UVBJuFYfvDo",
  "path": "project",
  "extension": "jsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [305, 461]
}
```

### Module: project/test.jsx (main module)

```jsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_UVBJuFYfvDo = ()=>import("./test.tsx_Header_component_UVBJuFYfvDo");
export const [a, { b, v1: [c], d = v2, ...e }, f = v3, ...g] = obj;
const exp1 = 1;
const internal = 2;
export { exp1, internal as expr2 };
export function foo() {}
export class bar {
}
export default function DefaultFn() {}
export const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_UVBJuFYfvDo, "Header_component_UVBJuFYfvDo"));
export const Footer = /*#__PURE__*/ componentQrl();
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.jsx`. All export patterns preserved. TS type annotations removed. Destructured export kept. Footer has componentQrl() with no args (empty component).*

</details>

### Module: project/test.tsx_Header_component_1_uWM1kg0IGO0.jsx (ENTRY POINT)

```jsx
import { default as DefaultFn } from "./test";
import { Footer } from "./test";
import { a } from "./test";
import { b } from "./test";
import { bar } from "./test";
import { c } from "./test";
import { d } from "./test";
import { e } from "./test";
import { exp1 } from "./test";
import { f } from "./test";
import { foo } from "./test";
import { expr2 as internal } from "./test";
export const Header_component_1_uWM1kg0IGO0 = ()=><Footer>
			<div>{a}{b}{c}{d}{e}{f}{exp1}{internal}{foo}{bar}{DefaultFn}</div>
			<div>{v1}{v2}{v3}{obj}</div>
		</Footer>;
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.jsx`. Imports ALL referenced values from main module including default import, named exports, and aliases. `internal` is imported via its exported name `expr2`. JSX preserved.*

</details>

#### Segment Metadata
```json
{
  "origin": "project/test.tsx",
  "name": "Header_component_1_uWM1kg0IGO0",
  "entry": null,
  "displayName": "test.tsx_Header_component_1",
  "hash": "uWM1kg0IGO0",
  "canonicalFilename": "test.tsx_Header_component_1_uWM1kg0IGO0",
  "path": "project",
  "extension": "jsx",
  "parent": "Header_component_UVBJuFYfvDo",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [323, 458]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for segment references in both main module and component body
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `$` -> `qrl`
- **[CONV-06] Lazy Imports**: Standard lazy import pattern
- **[CONV-07] PURE Annotations**: On componentQrl and qrl calls
- **[CONV-08] Segment Extraction**: Two segments: Header component body and inner $ callback. The inner callback imports 12 values from the main module.
- **Note**: No CONV-03 (JSX transforms) -- JSX preserved since transpile_jsx is false

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.jsx | @qwik.dev/core | 2 |
| qrl | test.jsx | @qwik.dev/core | 1 |
| qrl | Header_component_UVBJuFYfvDo.jsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
