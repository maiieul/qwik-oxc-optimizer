# Test: should_not_generate_conflicting_props_identifiers

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist (non-default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useComputed$, useTask$ } from '@qwik.dev/core'

export default component$(({ color, ...props }) => {
	useComputed$(() => color)

	useTask$(() => {
		props.checked
	})

	return 'hi'
})
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { _restProps } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { useComputedQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const test_component_useComputed_PYU291PvidQ = ()=>{
    const _rawProps = _captures[0];
    return _rawProps.color;
};
const test_component_useTask_jewzFYh3XmQ = ()=>{
    const props = _captures[0];
    props.checked;
};
const test_component_LUXeXe0DQrg = (_rawProps)=>{
    const props = _restProps(_rawProps, [
        "color"
    ]);
    useComputedQrl(/*#__PURE__*/ inlinedQrl(test_component_useComputed_PYU291PvidQ, "test_component_useComputed_PYU291PvidQ", [
        _rawProps
    ]));
    useTaskQrl(/*#__PURE__*/ inlinedQrl(test_component_useTask_jewzFYh3XmQ, "test_component_useTask_jewzFYh3XmQ", [
        props
    ]));
    return 'hi';
};
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(test_component_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` used because of Hoist entry strategy -- all segments inlined in one module
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useComputed$` to `useComputedQrl()`, `useTask$` to `useTaskQrl()`
- **[CONV-05] Capture Patterns**: Both callbacks use `_captures[0]` -- useComputed captures `_rawProps` for `.color`, useTask captures `props` (rest proxy) for `.checked`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()` and `inlinedQrl()` calls
- **[CONV-11] Props Destructuring**: `({ color, ...props })` transformed to `(_rawProps)` with `_restProps(_rawProps, ["color"])`. No conflicting identifiers between `_rawProps` and `props`

**Key behavior:** With Hoist entry strategy, all segments are in one file. The test verifies that `_rawProps` (the raw props parameter) and `props` (the rest proxy) do not conflict as identifier names when both appear in the same module.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `inlinedQrl` | test.js | @qwik.dev/core | 3 |
| `_restProps` | test.js | @qwik.dev/core | 1 |
| `useComputedQrl` | test.js | @qwik.dev/core | 1 |
| `useTaskQrl` | test.js | @qwik.dev/core | 1 |
| `_captures` | test.js | @qwik.dev/core | 2 (array access) |

## Diagnostics

```json
[]
```
