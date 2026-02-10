# Test: should_destructure_args

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
import { component$ } from "@qwik.dev/core";

// the count results in _fnSignal because of the rename
// would be nice to consider it a prop too
export default component$(({ message, id, count: c, ...rest }: Record<string, any>) => {
	const renders = useStore({ renders: 0 }, { reactive: false });
	renders.renders++;
	const rerenders = renders.renders + 0;
	return (
		<div id={id}>
			<span {...rest}>
			{message} {c}
			</span>
			<div class="renders">{rerenders}</div>
		</div>
	);
}
);
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
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
// the count results in _fnSignal because of the rename
// would be nice to consider it a prop too
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { _restProps } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (_rawProps)=>{
    const rest = _restProps(_rawProps, [
        "message",
        "id",
        "count"
    ]);
    const renders = useStore({
        renders: 0
    }, {
        reactive: false
    });
    renders.renders++;
    const rerenders = renders.renders + 0;
    return /*#__PURE__*/ _jsxSorted("div", {
        id: _wrapProp(_rawProps, "id")
    }, null, [
        /*#__PURE__*/ _jsxSplit("span", {
            ..._getVarProps(rest)
        }, _getConstProps(rest), [
            _wrapProp(_rawProps, "message"),
            " ",
            _wrapProp(_rawProps, "count")
        ], 0, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            class: "renders"
        }, rerenders, 1, null)
    ], 1, "u6_0");
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
  "name": "test_component_LUXeXe0DQrg",
  "entry": null,
  "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg",
  "canonicalFilename": "test.tsx_test_component_LUXeXe0DQrg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [181, 522],
  "paramNames": ["_rawProps"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` in main module
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` and `_jsxSplit("span", ...)` replace JSX. `_jsxSplit` is used when a spread operator (`{...rest}`) is present on the element, requiring split between variable and constant props
- **[CONV-04] Signal Helpers**: `_wrapProp(_rawProps, "id")`, `_wrapProp(_rawProps, "message")`, `_wrapProp(_rawProps, "count")` -- named destructured props accessed reactively through `_rawProps`; `_getVarProps(rest)` and `_getConstProps(rest)` separate rest props into reactive and static categories for the spread element
- **[CONV-06] Lazy Imports**: Dynamic import for component segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, `_jsxSorted()`, `_jsxSplit()`
- **[CONV-08] Segment Extraction**: Component body extracted to entry point
- **[CONV-11] Props Destructuring**: Complex destructuring `{ message, id, count: c, ...rest }` transformed to `(_rawProps)` with `_restProps(_rawProps, ["message", "id", "count"])`. Named props accessed via `_wrapProp(_rawProps, "propName")`. The renamed prop `count: c` is still accessed as `_wrapProp(_rawProps, "count")`. Rest spread uses `_getVarProps(rest)` and `_getConstProps(rest)` for proper JSX split rendering

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | entry point | @qwik.dev/core | 2 |
| `_jsxSplit` | entry point | @qwik.dev/core | 1 |
| `_restProps` | entry point | @qwik.dev/core | 1 |
| `_wrapProp` | entry point | @qwik.dev/core | 3 |
| `_getVarProps` | entry point | @qwik.dev/core | 1 |
| `_getConstProps` | entry point | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
