# Test: destructure_args_inline_cmp_block_stmt

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
export default ({ data }: { data: any }) => {
    return (
        <div
            data-is-active={data.selectedOutputDetail === 'options'}
            onClick$={() => {
                data.selectedOutputDetail = 'options';
            }}
        />
    );
};
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.data.selectedOutputDetail === 'options';
const _hf0_str = 'p0.data.selectedOutputDetail==="options"';
const i_pFqTss400MA = ()=>import("./test.tsx_test_div_q_e_click_pFqTss400MA");
export default ((_rawProps)=>{
    return /*#__PURE__*/ _jsxSorted("div", {
        "data-is-active": _fnSignal(_hf0, [
            _rawProps
        ], _hf0_str),
        "q-e:click": /*#__PURE__*/ qrl(i_pFqTss400MA, "test_div_q_e_click_pFqTss400MA", [
            _rawProps
        ])
    }, null, null, 2, "u6_0");
});
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_div_q_e_click_pFqTss400MA.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const test_div_q_e_click_pFqTss400MA = ()=>{
    const _rawProps = _captures[0];
    _rawProps.data.selectedOutputDetail = 'options';
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
  "name": "test_div_q_e_click_pFqTss400MA",
  "entry": null,
  "displayName": "test.tsx_test_div_q_e_click",
  "hash": "pFqTss400MA",
  "canonicalFilename": "test.tsx_test_div_q_e_click_pFqTss400MA",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [181, 259],
  "captureNames": ["_rawProps"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl(i_pFqTss400MA, "test_div_q_e_click_pFqTss400MA", [_rawProps])` with capture array as third argument
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` replaces JSX `<div />`
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [_rawProps], _hf0_str)` wraps `data.selectedOutputDetail === 'options'` for reactive attribute binding
- **[CONV-05] Capture Patterns**: `const _rawProps = _captures[0]` in entry point -- the `_rawProps` reference is captured from the parent scope and restored via `_captures` array
- **[CONV-06] Lazy Imports**: `const i_pFqTss400MA = ()=>import(...)` for event handler segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `_jsxSorted()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Event handler extracted to `test_div_q_e_click_pFqTss400MA.js`
- **[CONV-11] Props Destructuring**: `{ data }` destructured parameter renamed to `_rawProps` -- the inline component (not wrapped in `component$`) has its destructured parameter replaced with `_rawProps` for raw prop access
- **[CONV-14] Hoisted Functions**: `_hf0` hoists the comparison expression for reactive tracking

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.js | @qwik.dev/core | 1 |
| `_fnSignal` | test.js | @qwik.dev/core | 1 |
| `_captures` | entry point | @qwik.dev/core | 1 (array access) |

## Diagnostics

```json
[]
```
