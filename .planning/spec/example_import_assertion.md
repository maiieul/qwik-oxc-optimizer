# Test: example_import_assertion

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | true |
| Transpile JSX | true |

**Key insight:** Tests import assertion syntax. The input uses `import json from "./foo.json" assert { type: "json" }` (the older `assert` keyword). In the output segment, this is converted to the newer `with` keyword: `import json from "./foo.json" with { type: "json" }`. The JSON import is moved from the main module to the component segment since only the segment body needs it.

## Input

### Source Code
```tsx
import { component$, $ } from '@qwik.dev/core';
import json from "./foo.json" assert { type: "json" };

export const Greeter = component$(() => {
	return json;
});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Import assertion with `assert` keyword for JSON module. Single component that returns the imported JSON value.*

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_n7HuG2hhU0Q = ()=>import("./test.tsx_Greeter_component_n7HuG2hhU0Q");
export const Greeter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_n7HuG2hhU0Q, "Greeter_component_n7HuG2hhU0Q"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.js`. JSON import removed from main module (moved to segment). Standard componentQrl + qrl transformation.*

</details>

### Module: test.tsx_Greeter_component_n7HuG2hhU0Q.js (ENTRY POINT)

```javascript
import json from "./foo.json" with {
    type: "json"
};
export const Greeter_component_n7HuG2hhU0Q = ()=>{
    return json;
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_n7HuG2hhU0Q",
  "entry": null,
  "displayName": "test.tsx_Greeter_component",
  "hash": "n7HuG2hhU0Q",
  "extension": "js",
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
- **[CONV-08] Segment Extraction**: One segment extracted with JSON import moved into it
- **Import Assertion Transform**: `assert` keyword converted to `with` keyword (modernization)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
