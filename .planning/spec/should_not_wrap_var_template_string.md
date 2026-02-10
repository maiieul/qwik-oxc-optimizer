# Test: should_not_wrap_var_template_string

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useComputed$ } from '@qwik.dev/core';
import { inlineTranslate } from 'translate-lib';

export default component$(() => {
	const t = inlineTranslate();

	const productTitle = useComputed$(() => {
		return 'Test title';
	});

	return (
		<img
			attr={t('home.imageAlt.founded-product:')}
			alt={`${t('home.imageAlt.founded-product:')} ${productTitle.value}`} />
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

### Module: test.tsx_test_component_productTitle_useComputed_ZVQVUkxqtiQ.js (ENTRY POINT)

```javascript
export const test_component_productTitle_useComputed_ZVQVUkxqtiQ = ()=>{
    return 'Test title';
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_productTitle_useComputed_ZVQVUkxqtiQ",
  "entry": null, "displayName": "test.tsx_test_component_productTitle_useComputed",
  "hash": "ZVQVUkxqtiQ", "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "function", "ctxName": "useComputed$",
  "captures": false, "loc": [221, 258]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { inlineTranslate } from "translate-lib";
import { qrl } from "@qwik.dev/core";
import { useComputedQrl } from "@qwik.dev/core";
const i_ZVQVUkxqtiQ = ()=>import("./test.tsx_test_component_productTitle_useComputed_ZVQVUkxqtiQ");
export const test_component_LUXeXe0DQrg = ()=>{
    const t = inlineTranslate();
    const productTitle = useComputedQrl(/*#__PURE__*/ qrl(i_ZVQVUkxqtiQ, "test_component_productTitle_useComputed_ZVQVUkxqtiQ"));
    return /*#__PURE__*/ _jsxSorted("img", {
        alt: `${t('home.imageAlt.founded-product:')} ${productTitle.value}`,
        attr: t('home.imageAlt.founded-product:')
    }, null, null, 3, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null, "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [143, 418]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy; `useComputedQrl()` wraps computed callback
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `useComputed$` to `useComputedQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for `<img>` with var props
- **[CONV-06] Lazy Imports**: Lazy imports for component and useComputed segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on framework calls
- **[CONV-08] Segment Extraction**: Component body and useComputed callback extracted

**Key behavior**: The `alt` prop contains a template literal `` `${t(...)} ${productTitle.value}` `` which mixes a function call (`t(...)`) with signal access (`productTitle.value`). The optimizer does NOT wrap this with `_fnSignal` because the expression contains function calls. Both `alt` and `attr` are placed in the var props object. The `attr` prop calls `t()` which is also non-trackable. The `useComputed$` callback has no captures (it returns a constant) and is extracted to a separate module.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component segment | @qwik.dev/core | 2 |
| `useComputedQrl` | component segment | @qwik.dev/core | 1 |
| `_jsxSorted` | component segment | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
