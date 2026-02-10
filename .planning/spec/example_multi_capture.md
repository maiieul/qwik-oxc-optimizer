# Test: example_multi_capture

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true (non-default) |
| Transpile JSX | false (default) |

## Input

### Source Code
```tsx
import { $, component$ } from '@qwik.dev/core';

export const Foo = component$(({foo}) => {
	const arg0 = 20;
	return $(() => {
		const fn = ({aaa}) => aaa;
		return (
			<div>
				{foo}{fn()}{arg0}
			</div>
		)
	});
})

export const Bar = component$(({bar}) => {
	return $(() => {
		return (
			<div>
				{bar}
			</div>
		)
	});
})
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.jsx

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
const i_L80pS8Hxf1Y = ()=>import("./test.tsx_Bar_component_L80pS8Hxf1Y");
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"));
export const Bar = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_L80pS8Hxf1Y, "Bar_component_L80pS8Hxf1Y"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util jsx`

</details>

### Module: test.tsx_Foo_component_HTDRsvUbLiE.jsx (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_DvU6FitWglY = ()=>import("./test.tsx_Foo_component_1_DvU6FitWglY");
export const Foo_component_HTDRsvUbLiE = (_rawProps)=>{
    return /*#__PURE__*/ qrl(i_DvU6FitWglY, "Foo_component_1_DvU6FitWglY", [
        _rawProps
    ]);
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util jsx`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Foo_component_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "jsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [81, 221],
  "paramNames": ["_rawProps"]
}
```

### Module: test.tsx_Bar_component_L80pS8Hxf1Y.jsx (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_0xSyNSnVu3k = ()=>import("./test.tsx_Bar_component_1_0xSyNSnVu3k");
export const Bar_component_L80pS8Hxf1Y = (_rawProps)=>{
    return /*#__PURE__*/ qrl(i_0xSyNSnVu3k, "Bar_component_1_0xSyNSnVu3k", [
        _rawProps
    ]);
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util jsx`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Bar_component_L80pS8Hxf1Y",
  "entry": null,
  "displayName": "test.tsx_Bar_component",
  "hash": "L80pS8Hxf1Y",
  "canonicalFilename": "test.tsx_Bar_component_L80pS8Hxf1Y",
  "path": "",
  "extension": "jsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [254, 335],
  "paramNames": ["_rawProps"]
}
```

### Module: test.tsx_Foo_component_1_DvU6FitWglY.jsx (ENTRY POINT)

```jsx
import { _captures } from "@qwik.dev/core";
export const Foo_component_1_DvU6FitWglY = ()=>{
    const _rawProps = _captures[0];
    const fn = ({ aaa })=>aaa;
    return <div>
				{_rawProps.foo}{fn()}{20}
			</div>;
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util jsx`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Foo_component_1_DvU6FitWglY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_1",
  "hash": "DvU6FitWglY",
  "canonicalFilename": "test.tsx_Foo_component_1_DvU6FitWglY",
  "path": "",
  "extension": "jsx",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [122, 217],
  "captureNames": ["_rawProps"]
}
```

### Module: test.tsx_Bar_component_1_0xSyNSnVu3k.jsx (ENTRY POINT)

```jsx
import { _captures } from "@qwik.dev/core";
export const Bar_component_1_0xSyNSnVu3k = ()=>{
    const _rawProps = _captures[0];
    return <div>
				{_rawProps.bar}
			</div>;
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util jsx`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Bar_component_1_0xSyNSnVu3k",
  "entry": null,
  "displayName": "test.tsx_Bar_component_1",
  "hash": "0xSyNSnVu3k",
  "canonicalFilename": "test.tsx_Bar_component_1_0xSyNSnVu3k",
  "path": "",
  "extension": "jsx",
  "parent": "Bar_component_L80pS8Hxf1Y",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [277, 331],
  "captureNames": ["_rawProps"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` used extensively -- main module references two components, each component references its `$()` child segment, with capture arrays as third argument
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `$()` returns QRL references
- **[CONV-05] Capture Patterns**: Both `Foo_component_1` and `Bar_component_1` use `_captures[0]` to restore `_rawProps` from parent scope. The `const arg0 = 20` is inlined as literal `20` (not captured) while `_rawProps` is captured because it contains runtime-variable data (`.foo`, `.bar`)
- **[CONV-06] Lazy Imports**: Six dynamic imports total -- two for components, two for `$()` segments, all using `const i_HASH = ()=>import(...)` pattern
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all `componentQrl()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Four entry point segments plus main module. Two-level nesting: component body -> `$()` callback
- **[CONV-11] Props Destructuring**: Both `({foo})` and `({bar})` destructured props are renamed to `_rawProps` in component segments. In nested `$()` segments, `_rawProps.foo` and `_rawProps.bar` access the original prop names

**Note:** Output uses `.jsx` extension (not `.js`) because `transpile_jsx: false` -- JSX remains untransformed in the output. The nested segments contain raw `<div>` JSX elements.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.jsx | @qwik.dev/core | 2 |
| `qrl` | test.jsx, Foo entry, Bar entry | @qwik.dev/core | 4 |
| `_captures` | Foo_1, Bar_1 entries | @qwik.dev/core | 2 (array access) |

## Diagnostics

```json
[]
```
