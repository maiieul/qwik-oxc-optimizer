# Test: should_move_props_related_to_iteration_variables_to_var_props

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useSignal } from "@qwik.dev/core";
import { TestComponent } from "./testComponent";

export const Child = component$(() => {
  const propCounterWithNested = useSignal(() => {
  return {
      data: [
        { attributeInArray: { counter: globalThis.propsCounter++ } },
        { attributeInArray: { counter: globalThis.propsCounter } },
      ],
    };
  })
  return (
    <div>
        {propCounterWithNested.value.data.map((item, index) => {
          return (
            <TestComponent
              counter={item.attributeInArray.counter}
              logString="Nested read through array"
              key={index}
            />
          );
        })}
    </div>
  );
})
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_9GyF01GDKqw = ()=>import("./test.tsx_Child_component_9GyF01GDKqw");
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_9GyF01GDKqw, "Child_component_9GyF01GDKqw"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_Child_component_9GyF01GDKqw.js (ENTRY POINT)

```javascript
import { TestComponent } from "./testComponent";
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.attributeInArray.counter;
const _hf0_str = "p0.attributeInArray.counter";
export const Child_component_9GyF01GDKqw = ()=>{
    const propCounterWithNested = useSignal(()=>{
        return {
            data: [
                {
                    attributeInArray: {
                        counter: globalThis.propsCounter++
                    }
                },
                {
                    attributeInArray: {
                        counter: globalThis.propsCounter
                    }
                }
            ]
        };
    });
    return /*#__PURE__*/ _jsxSorted("div", null, null, propCounterWithNested.value.data.map((item, index)=>{
        return /*#__PURE__*/ _jsxSorted(TestComponent, {
            counter: _fnSignal(_hf0, [
                item
            ], _hf0_str)
        }, {
            logString: "Nested read through array"
        }, null, 3, index);
    }), 1, "u6_0");
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Child_component_9GyF01GDKqw",
  "entry": null,
  "displayName": "test.tsx_Child_component",
  "hash": "9GyF01GDKqw",
  "canonicalFilename": "test.tsx_Child_component_9GyF01GDKqw",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [140, 703]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module to reference component segment
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` and `_jsxSorted(TestComponent, ...)` replace JSX. The `TestComponent` is passed as a component reference (not string tag). Props are split between variable props (object) and constant props (object) in separate arguments
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [item], _hf0_str)` wraps `item.attributeInArray.counter` for reactive tracking. The `counter` prop uses `_fnSignal` because `item` is an iteration variable (from `.map()`) that varies per render cycle. The `logString` prop is a string literal and goes into constant props
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, and `_jsxSorted()` calls
- **[CONV-08] Segment Extraction**: Component body extracted to entry point
- **[CONV-14] Hoisted Functions**: `const _hf0 = (p0)=>p0.attributeInArray.counter` with `_hf0_str` -- the deep property access is hoisted for efficient reactive signal creation within iteration

**Key behavior:** Props related to iteration variables (`item` from `.map()`) are moved to the variable props argument (first props object) of `_jsxSorted`, while static string props like `logString` are placed in the constant props argument (second props object). This separation enables the optimizer to differentiate between props that change per iteration and those that remain constant.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 2 |
| `_fnSignal` | entry point | @qwik.dev/core | 1 |
| `useSignal` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
