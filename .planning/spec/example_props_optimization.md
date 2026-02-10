# Test: example_props_optimization

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { $, component$, useTask$ } from '@qwik.dev/core';
import { CONST } from 'const';
export const Works = component$(({
	count,
	some = 1+2,
	hello = CONST,
	stuff: hey,
	stuffDefault: hey2 = 123,
	...rest}) => {
	console.log(hey, some);
	useTask$(({track}) => {
		track(() => count);
		console.log(count, rest, hey, some, hey2);
	});
	return (
		<div some={some} params={{ some }} class={count} {...rest} override>{count}</div>
	);
});

export const NoWorks2 = component$(({count, stuff: {hey}}) => {
	console.log(hey);
	useTask$(({track}) => {
		track(() => count);
		console.log(count);
	});
	return (
		<div class={count}>{count}</div>
	);
});

export const NoWorks3 = component$(({count, stuff = hola()}) => {
	console.log(stuff);
	useTask$(({track}) => {
		track(() => count);
		console.log(count);
	});
	return (
		<div class={count}>{count}</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js

```javascript
import { _restProps } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.some ?? 3;
const _hf0_str = "p0.some??1+2";
const _hf1 = (p0)=>({ some: p0.some ?? 3 });
const _hf1_str = "{some:p0.some??1+2}";
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((_rawProps)=>{
    const rest = _restProps(_rawProps, ["count", "some", "hello", "stuff", "stuffDefault"]);
    console.log(_rawProps.stuff, _rawProps.some ?? 3);
    useTaskQrl(/*#__PURE__*/ inlinedQrl(({ track })=>{
        const _rawProps = _captures[0], rest = _captures[1];
        track(()=>_rawProps.count);
        console.log(_rawProps.count, rest, _rawProps.stuff, _rawProps.some ?? 3, _rawProps.stuffDefault ?? 123);
    }, "Works_component_useTask_pjo5U5Ikll0", [_rawProps, rest]));
    return /*#__PURE__*/ _jsxSplit("div", {
        some: _fnSignal(_hf0, [_rawProps], _hf0_str),
        params: _fnSignal(_hf1, [_rawProps], _hf1_str),
        class: _wrapProp(_rawProps, "count"),
        ..._getVarProps(rest)
    }, { ..._getConstProps(rest), override: true },
    _wrapProp(_rawProps, "count"), 0, "u6_0");
}, "Works_component_t45qL4vNGv0"));
export const NoWorks2 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(({ count, stuff: { hey } })=>{
    console.log(hey);
    useTaskQrl(/*#__PURE__*/ inlinedQrl(({ track })=>{
        const count = _captures[0];
        track(()=>count);
        console.log(count);
    }, "NoWorks2_component_useTask_lXiqwbxxjq0", [count]));
    return /*#__PURE__*/ _jsxSorted("div", { class: count }, null, count, 1, "u6_1");
}, "NoWorks2_component_JPD9t2HyEKg"));
export const NoWorks3 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(({ count, stuff = hola() })=>{
    console.log(stuff);
    useTaskQrl(/*#__PURE__*/ inlinedQrl(({ track })=>{
        const count = _captures[0];
        track(()=>count);
        console.log(count);
    }, "NoWorks3_component_useTask_3cQGU0s1VwU", [count]));
    return /*#__PURE__*/ _jsxSorted("div", { class: count }, null, count, 1, "u6_2");
}, "NoWorks3_component_fc13h5yYn14"));
```

<details>
<summary>Output AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util js`*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` for Inline strategy with name strings
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `useTask$` to `useTaskQrl`
- **[CONV-03] JSX Transforms**: `_jsxSplit()` for `Works` (has spread props), `_jsxSorted()` for `NoWorks2` and `NoWorks3`
- **[CONV-04] Signal Helpers**: `_wrapProp(_rawProps, "count")` for direct prop access; `_fnSignal()` for computed prop expressions; `_getVarProps(rest)` and `_getConstProps(rest)` for spread rest props
- **[CONV-05] Capture Patterns**: `_captures[0]`, `_captures[1]` in useTask handlers to restore `_rawProps` and `rest`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-11] Props Destructuring**: `_rawProps` replaces destructured parameter for `Works` (enabling prop tracking); `_restProps(_rawProps, [...])` computes rest props; default values `some = 1+2` become `_rawProps.some ?? 3`
- **[CONV-14] Hoisted Functions**: `_hf0`, `_hf1` for computed prop values with defaults

**Key behavior**: Props optimization has three cases:
1. **Works** (simple destructuring with rest): Destructured params replaced with `_rawProps`, enabling individual prop tracking via `_wrapProp`. Default values use nullish coalescing (`??`). Rest props handled via `_restProps`.
2. **NoWorks2** (nested destructuring `stuff: {hey}`): Nested destructuring prevents optimization -- destructuring preserved as-is.
3. **NoWorks3** (default value from function call `stuff = hola()`): Function call defaults prevent optimization -- destructuring preserved as-is.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 3 |
| `inlinedQrl` | test.js | @qwik.dev/core | 6 |
| `useTaskQrl` | test.js | @qwik.dev/core | 3 |
| `_jsxSplit` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 2 |
| `_restProps` | test.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.js | @qwik.dev/core | 2 |
| `_fnSignal` | test.js | @qwik.dev/core | 2 |
| `_getVarProps` | test.js | @qwik.dev/core | 1 |
| `_getConstProps` | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
