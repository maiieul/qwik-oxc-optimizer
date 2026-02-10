# Test: example_dead_code

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Minify | Simplify (default) |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';
import { deps } from 'deps';

export const Foo = component$(({foo}) => {
	useMount$(() => {
		if (false) {
			deps();
		}
	});
	return (
		<div />
	);
})
```

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"));
```

### Module: test.tsx_Foo_component_HTDRsvUbLiE.tsx (ENTRY POINT)

```tsx
export const Foo_component_HTDRsvUbLiE = (_rawProps)=>{
    useMount$(()=>{});
    return <div/>;
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
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [107, 199],
  "paramNames": ["_rawProps"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on calls
- **[CONV-08] Segment Extraction**: Component body extracted
- **[CONV-11] Props Destructuring**: `({foo})` to `(_rawProps)`

**Key dead code behavior:** `MinifyMode::Simplify` eliminates `if (false) { deps(); }`, leaving empty `useMount$` callback. The `deps` import is tree-shaken.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `useMount$` | entry point | (retained as-is) | 1 |

## Diagnostics

```json
[]
```
