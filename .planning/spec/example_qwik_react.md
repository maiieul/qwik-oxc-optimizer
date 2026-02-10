# Test: example_qwik_react

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | ../node_modules/@qwik.dev/react/index.qwik.mjs |
| Entry Strategy | Segment |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |
| Explicit Extensions | true |

**Key insight:** This tests the Qwik React integration package itself. The input is pre-compiled code (already uses `componentQrl`, `inlinedQrl`, `useLexicalScope`, etc.) from `@qwik.dev/react`. The optimizer extracts segments from this pre-compiled code, creating separate files for the component body and task handler. The `filterProps` helper gets an `_auto_filterProps` export alias so segments can import it. Output uses `.mjs` extension matching the input filename.

## Input

### Source Code
```javascript
import { componentQrl, inlinedQrl, useLexicalScope, useHostElement, useStore, useTaskQrl, noSerialize, SkipRerender, implicit$FirstArg } from '@qwik.dev/core';
import { jsx, Fragment } from '@qwik.dev/core/jsx-runtime';
import { isBrowser, isServer } from '@qwik.dev/core';

function qwikifyQrl(reactCmpQrl) {
	return /*#__PURE__*/ componentQrl(inlinedQrl((props)=>{
		// ... component body with useHostElement, useStore, useTaskQrl, server/client rendering
	}, "qwikifyQrl_component_zH94hIe0Ick", [reactCmpQrl]), {
		tagName: 'qwik-wrap'
	});
}
const filterProps = (props)=>{ /* filter client: prefixed props */ };
const qwikify$ = implicit$FirstArg(qwikifyQrl);
async function renderToString(rootNode, opts) { /* server render */ }
export { qwikify$, qwikifyQrl, renderToString };
```

*Full input is ~90 lines of pre-compiled Qwik React integration code.*

<details>
<summary>Input AST (OXC)</summary>

*MJS input. Pre-compiled code with existing QRL references, inlinedQrl calls with hash strings, useLexicalScope, jsx calls.*

</details>

## Output

### Module: index.qwik.mjs_qwikifyQrl_component_useWatch_x04JC5xeP1U.mjs (ENTRY POINT)

```javascript
import { _auto_filterProps as filterProps } from "./index.qwik.mjs";
import { isBrowser } from "@qwik.dev/core";
import { noSerialize } from "@qwik.dev/core";
import { useLexicalScope } from "@qwik.dev/core";
export const qwikifyQrl_component_useWatch_x04JC5xeP1U = async (track)=>{
    const [hostElement, props, reactCmpQrl, store] = useLexicalScope();
    track(props);
    if (isBrowser) { /* client-side hydration/rendering logic */ }
};
```

*Task handler segment with filterProps imported via `_auto_` alias.*

#### Segment Metadata
```json
{
  "origin": "../node_modules/@qwik.dev/react/index.qwik.mjs",
  "name": "qwikifyQrl_component_useWatch_x04JC5xeP1U",
  "entry": null,
  "displayName": "index.qwik.mjs_qwikifyQrl_component_useWatch",
  "hash": "x04JC5xeP1U",
  "extension": "mjs",
  "parent": "qwikifyQrl_component_zH94hIe0Ick",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": true,
  "captureNames": ["hostElement", "props", "reactCmpQrl", "store"]
}
```

### Module: index.qwik.mjs_qwikifyQrl_component_zH94hIe0Ick.mjs (ENTRY POINT)

```javascript
import { Fragment } from "@qwik.dev/core/jsx-runtime";
import { SkipRerender } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
// ... other imports ...
import { qrl } from "@qwik.dev/core";
const i_x04JC5xeP1U = ()=>import("./index.qwik.mjs_qwikifyQrl_component_useWatch_x04JC5xeP1U.mjs");
export const qwikifyQrl_component_zH94hIe0Ick = (props)=>{
    const [reactCmpQrl] = useLexicalScope();
    // ... component logic with useHostElement, useStore, useTaskQrl ...
    // JSX transpiled to _jsxSorted for Host, Fragment, SkipRerender
};
```

*Component body segment with full rendering logic.*

#### Segment Metadata
```json
{
  "origin": "../node_modules/@qwik.dev/react/index.qwik.mjs",
  "name": "qwikifyQrl_component_zH94hIe0Ick",
  "entry": null,
  "displayName": "index.qwik.mjs_qwikifyQrl_component",
  "hash": "zH94hIe0Ick",
  "extension": "mjs",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": true,
  "captureNames": ["reactCmpQrl"]
}
```

### Module: ../node_modules/@qwik.dev/react/index.qwik.mjs

```javascript
import { qrl } from "@qwik.dev/core";
const i_zH94hIe0Ick = ()=>import("./index.qwik.mjs_qwikifyQrl_component_zH94hIe0Ick.mjs");
import { componentQrl, implicit$FirstArg } from '@qwik.dev/core';
function qwikifyQrl(reactCmpQrl) {
    return /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_zH94hIe0Ick, "qwikifyQrl_component_zH94hIe0Ick", [reactCmpQrl]), {
        tagName: 'qwik-wrap'
    });
}
const filterProps = (props)=>{ /* ... */ };
const qwikify$ = implicit$FirstArg(qwikifyQrl);
async function renderToString(rootNode, opts) { /* ... */ }
export { qwikify$, qwikifyQrl, renderToString };
export { filterProps as _auto_filterProps };
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.mjs`. Pre-compiled code restructured: inlinedQrl replaced with qrl + lazy import. filterProps exported via _auto_ alias. componentQrl receives qrl with capture array [reactCmpQrl].*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` with capture arrays replacing `inlinedQrl()` from input (Segment strategy extracts)
- **[CONV-02] Dollar-to-Qrl**: Pre-compiled input already uses Qrl forms; optimizer re-processes segment extraction
- **[CONV-03] JSX Transforms**: `jsx()` calls from input transformed to `_jsxSorted()` in segments
- **[CONV-05] Capture Patterns**: Capture arrays `[reactCmpQrl]` and `[hostElement, props, reactCmpQrl, store]` in qrl calls
- **[CONV-06] Lazy Imports**: `.mjs` extension in lazy import paths (matching input file extension)
- **[CONV-07] PURE Annotations**: On componentQrl and qrl calls
- **[CONV-08] Segment Extraction**: Two segments extracted from pre-compiled code

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | index.qwik.mjs | @qwik.dev/core | 1 |
| qrl | index.qwik.mjs / component segment | @qwik.dev/core | 2 |
| implicit$FirstArg | index.qwik.mjs | @qwik.dev/core | 1 |
| useLexicalScope | component segment / task segment | @qwik.dev/core | 2 |
| useTaskQrl | component segment | @qwik.dev/core | 1 |
| _jsxSorted | component segment | @qwik.dev/core | 4 |

## Diagnostics

```json
[]
```
