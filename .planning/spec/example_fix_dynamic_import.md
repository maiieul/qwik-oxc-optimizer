# Test: example_fix_dynamic_import

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | project/folder/test.tsx |
| Entry Strategy | Single |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |

**Key insight:** Tests dynamic `import()` expressions in both non-component and component code. The `foo()` function's `import("../foo/state2")` stays in the main module since it's not in a component. The component's `import("../folder/state3")` is preserved inside the extracted segment. The `thing` import from `"../state"` is also moved to the segment. Since no transpile flags are set, the output preserves TSX/JSX syntax. The custom filename `project/folder/test.tsx` affects path resolution. The segment entry is `"entry_segments"` (Single strategy).

## Input

### Source Code
```tsx
import { $, component$ } from '@qwik.dev/core';
import thing from "../state";

export function foo() {
	return import("../foo/state2")
}

export const Header = component$(() => {
	return (
		<div>
			{import("../folder/state3")}
			{thing}
		</div>
	);
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Custom filename project/folder/test.tsx. Static import of `thing` from relative path. Dynamic import() in both regular function and component body. Component with JSX containing dynamic import expression.*

</details>

## Output

### Module: project/folder/test.tsx (main module)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_RGgm7Ks9QWI = ()=>import("./test.tsx_Header_component_RGgm7Ks9QWI");
export function foo() {
    return import("../foo/state2");
}
export const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_RGgm7Ks9QWI, "Header_component_RGgm7Ks9QWI"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.tsx`. Dynamic import in foo() preserved as-is in main module. component$ transformed to componentQrl with lazy segment reference. The `thing` import removed from main module (moved to segment).*

</details>

### Module: project/folder/test.tsx_Header_component_RGgm7Ks9QWI.tsx (ENTRY POINT)

```tsx
import thing from "../state";
export const Header_component_RGgm7Ks9QWI = ()=>{
    return <div>
			{import("../folder/state3")}
			{thing}
		</div>;
};
```

#### Segment Metadata
```json
{
  "origin": "project/folder/test.tsx",
  "name": "Header_component_RGgm7Ks9QWI",
  "entry": "entry_segments",
  "displayName": "test.tsx_Header_component",
  "hash": "RGgm7Ks9QWI",
  "path": "project/folder",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for segment reference with lazy import
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`
- **[CONV-06] Lazy Imports**: Standard lazy import pattern for segment
- **[CONV-07] PURE Annotations**: On componentQrl and qrl calls
- **[CONV-08] Segment Extraction**: One segment extracted; dynamic import() preserved inside segment
- **Note**: No JSX transform (transpile_jsx: false) -- JSX preserved as-is in segment output
- **Note**: Single strategy sets entry to "entry_segments" in metadata

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.tsx | @qwik.dev/core | 1 |
| qrl | test.tsx | @qwik.dev/core | 1 |
| import() | test.tsx / Header_component.tsx | (dynamic) | 2 |

## Diagnostics

```json
[]
```
