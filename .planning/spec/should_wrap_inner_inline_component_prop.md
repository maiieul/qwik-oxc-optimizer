# Test: should_wrap_inner_inline_component_prop

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { $, component$, useStore, useSignal } from '@qwik.dev/core';
export default component$((props: { id: number }) => {
      const renders = useStore(
        { count: 0 },
        { reactive: false }
      );
      renders.count++;
      const rerenders = renders.count + 0;
      const Id = (props: any) => <div>Id: {props.id}</div>;
      return (
        <>
          <Id id={props.id} />
          {rerenders}
        </>
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

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (props)=>{
    const renders = useStore({ count: 0 }, { reactive: false });
    renders.count++;
    const rerenders = renders.count + 0;
    const Id = (props)=>/*#__PURE__*/ _jsxSorted("div", null, null, [
            "Id: ",
            _wrapProp(props, "id")
        ], 1, "u6_0");
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted(Id, null, {
            id: _wrapProp(props, "id")
        }, null, 3, "u6_1"),
        rerenders
    ], 1, "u6_2");
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
  "captures": false, "loc": [97, 467],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for Segment strategy
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; inner inline component `Id` also uses `_jsxSorted`
- **[CONV-04] Signal Helpers**: `_wrapProp(props, "id")` used in TWO places:
  1. When passing `props.id` to inner `<Id id={props.id}/>` component (const props)
  2. Inside the `Id` function definition for `props.id` child text
- **[CONV-06] Lazy Imports**: Lazy import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all `_jsxSorted` calls
- **[CONV-08] Segment Extraction**: Component body extracted

**Key behavior**: Inner inline function components (like `const Id = (props) => ...`) are NOT extracted to separate segments. They remain inline in the parent component's segment. The optimizer still applies `_wrapProp` wrapping inside the inline component's body AND at the call site. Both the outer component's `props.id` access and the inner component's `props.id` access are wrapped, enabling reactivity at both levels. The `{ reactive: false }` option for `useStore` is preserved as-is.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 4 |
| `_wrapProp` | entry point | @qwik.dev/core | 2 |

## Diagnostics

None (`[]`)
