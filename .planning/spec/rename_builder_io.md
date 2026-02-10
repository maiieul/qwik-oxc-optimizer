# Test: rename_builder_io

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | true |

**Key insight:** Tests legacy import path renaming. All `@builder.io/qwik` imports are renamed to `@qwik.dev/core`, `@builder.io/qwik/build` to `@qwik.dev/core/build`, `@builder.io/qwik-city` to `@qwik.dev/router`, and `@builder.io/qwik-react` to `@qwik.dev/react`. Non-Qwik `@builder.io` imports (like `@builder.io/sdk`) are left unchanged. The `isDev` import from `@builder.io/qwik/build` is preserved in the segment output (as `@qwik.dev/core/build`).

## Input

### Source Code
```tsx
		import { $, component$ } from "@builder.io/qwik";
		import { isDev } from "@builder.io/qwik/build";
		import { stuff } from "@builder.io/qwik-city";
		import { moreStuff } from "@builder.io/qwik-city/more/here";
		import { qwikify$ } from "@builder.io/qwik-react";
		import sdk from "@builder.io/sdk";

		export const Foo = qwikify$(MyReactComponent);

		export const Bar = $("a thing");

		export const App = component$(() => {
			sdk.hello();
			if (isDev) {
				stuff()
			} else {
				moreStuff()
			}
			return "hi";
		});
```

<details>
<summary>Input AST (OXC)</summary>

*TSX input. Legacy @builder.io import paths. Uses qwikify$ from qwik-react, component$ and $ from qwik, isDev from qwik/build, stuff/moreStuff from qwik-city.*

</details>

## Output

### Module: test.tsx_Bar_GXXnVUtURSw.ts (ENTRY POINT)

```typescript
export const Bar_GXXnVUtURSw = "a thing";
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Bar_GXXnVUtURSw",
  "entry": null,
  "displayName": "test.tsx_Bar",
  "hash": "GXXnVUtURSw",
  "extension": "ts",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false
}
```

### Module: test.tsx_Foo_qwikify_0Yoy9qA0SC0.ts (ENTRY POINT)

```typescript
export const Foo_qwikify_0Yoy9qA0SC0 = MyReactComponent;
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Foo_qwikify_0Yoy9qA0SC0",
  "entry": null,
  "displayName": "test.tsx_Foo_qwikify",
  "hash": "0Yoy9qA0SC0",
  "extension": "ts",
  "ctxKind": "function",
  "ctxName": "qwikify$",
  "captures": false
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.ts (ENTRY POINT)

```typescript
import { isDev } from "@qwik.dev/core/build";
import { moreStuff } from "@qwik.dev/router/more/here";
import sdk from "@builder.io/sdk";
import { stuff } from "@qwik.dev/router";
export const App_component_ckEPmXZlub0 = ()=>{
    sdk.hello();
    if (isDev) stuff();
    else moreStuff();
    return "hi";
};
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.ts`. Import paths renamed: @builder.io/qwik/build -> @qwik.dev/core/build, @builder.io/qwik-city -> @qwik.dev/router, @builder.io/qwik-city/more/here -> @qwik.dev/router/more/here. @builder.io/sdk left unchanged (not a Qwik package).*

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "extension": "ts",
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false
}
```

### Module: test.ts (main module)

```typescript
import { qwikifyQrl } from "@qwik.dev/react";
import { qrl } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
const i_0Yoy9qA0SC0 = ()=>import("./test.tsx_Foo_qwikify_0Yoy9qA0SC0");
const i_GXXnVUtURSw = ()=>import("./test.tsx_Bar_GXXnVUtURSw");
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const Foo = qwikifyQrl(/*#__PURE__*/ qrl(i_0Yoy9qA0SC0, "Foo_qwikify_0Yoy9qA0SC0"));
export const Bar = /*#__PURE__*/ qrl(i_GXXnVUtURSw, "Bar_GXXnVUtURSw");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.ts`. All @builder.io paths renamed: @builder.io/qwik -> @qwik.dev/core, @builder.io/qwik-react -> @qwik.dev/react. qwikify$ -> qwikifyQrl. $ -> qrl. component$ -> componentQrl. Output is .ts because transpile_jsx: true removes JSX but transpile_ts: false preserves TS.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for all three segment references (Foo/qwikify$, Bar/$, App/component$)
- **[CONV-02] Dollar-to-Qrl**: `component$` -> `componentQrl`, `$` -> `qrl`, `qwikify$` -> `qwikifyQrl`
- **[CONV-06] Lazy Imports**: Three lazy imports for the three segments
- **[CONV-07] PURE Annotations**: On qrl and componentQrl calls
- **[CONV-08] Segment Extraction**: Three segments extracted
- **Import Path Renaming** (not a numbered convention but a key transformation):
  - `@builder.io/qwik` -> `@qwik.dev/core`
  - `@builder.io/qwik/build` -> `@qwik.dev/core/build`
  - `@builder.io/qwik-city` -> `@qwik.dev/router`
  - `@builder.io/qwik-city/more/here` -> `@qwik.dev/router/more/here`
  - `@builder.io/qwik-react` -> `@qwik.dev/react`
  - `@builder.io/sdk` -> unchanged (not a Qwik package)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.ts | @qwik.dev/core | 1 |
| qrl | test.ts | @qwik.dev/core | 3 |
| qwikifyQrl | test.ts | @qwik.dev/react | 1 |
| sdk.hello | App_component.ts | @builder.io/sdk | 1 |
| stuff | App_component.ts | @qwik.dev/router | 1 |
| moreStuff | App_component.ts | @qwik.dev/router/more/here | 1 |

## Diagnostics

```json
[]
```
