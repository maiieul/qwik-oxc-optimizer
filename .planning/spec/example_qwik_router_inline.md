# Test: example_qwik_router_inline

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | ../node_modules/@qwik.dev/router/index.qwik.mjs |
| Entry Strategy | Smart |
| Mode | Lib |
| Transpile TS | false |
| Transpile JSX | false |
| Explicit Extensions | true |

**Key insight:** This is the largest test -- the full Qwik Router package (~1074 lines of pre-compiled code). Uses `mode: Lib` (library mode) and `Smart` entry strategy. The output is a single large module since Smart strategy in Lib mode keeps code inline with `inlinedQrl` rather than extracting segments. The code includes Router components (RouterOutlet, QwikRouterProvider, Link, Form), route action/loader utilities, navigation logic, and helper functions. Many Qwik conventions are demonstrated at scale.

## Input

### Source Code

*1074 lines of pre-compiled Qwik Router code from `@qwik.dev/router`. Includes:*
- *RouterOutlet, QwikRouterProvider, Link, Form, GetForm components*
- *routeAction$, routeLoader$, globalAction$, server$, validator$, zod$ utilities*
- *Navigation logic, client data fetching, route matching*
- *Imports from @qwik.dev/core, @qwik.dev/core/build, zod*

<details>
<summary>Input AST (OXC)</summary>

*MJS input. ~1074 lines. Multiple componentQrl, inlinedQrl, useContext, useContextProvider, useTaskQrl, etc.*

</details>

## Output

### Module: ../node_modules/@qwik.dev/router/index.qwik.mjs

*~850 lines of transformed output. All code stays inline (Smart + Lib mode). Key transformations:*

```javascript
import * as qwikRouterConfig from '@qwik-router-config';
import swRegister from '@qwik-router-sw-register';
import { _deserializeData, _fnSignal, _getContextElement, _jsxBranch, _jsxSplit, _restProps, _serializeData, _weakSerialize, _wrapSignal, componentQrl, createContextId, eventQrl, getLocale, implicit$FirstArg, inlinedQrl, noSerialize, SkipRender, Slot, untrack, useContext, useContextProvider, useLexicalScope, useOnDocument, useServerData, useSignal, useStore, useStylesQrl, useTaskQrl, withLocale } from '@qwik.dev/core';
import { isBrowser, isDev, isServer } from '@qwik.dev/core/build';
import { z, z as z2 } from 'zod';

const RouteStateContext = /* @__PURE__ */ createContextId('qc-s');
// ... 7 context IDs ...

const RouterOutlet = /* @__PURE__ */ componentQrl(/* @__PURE__ */ inlinedQrl(()=>{
    _jsxBranch();
    useOnDocument('qinit', eventQrl(/* @__PURE__ */ inlinedQrl(()=>{ /* popstate fallback */ }, 'RouterOutlet_component_useOnDocument_event_KnNE9eL0qfc')));
    // ... content rendering with _jsxSplit ...
}, 'RouterOutlet_component_AKetNByE5TM'));

// ... QwikRouterProvider with complex navigation logic ...
// ... Link component with _restProps, _fnSignal, _wrapSignal ...
// ... Form, GetForm components ...
// ... routeAction$, routeLoader$, globalAction$, server$, validator$, zod$ ...

export { Form, globalAction$, globalActionQrl, Link, QwikRouterMockProvider, QwikRouterProvider, routeAction$, routeActionQrl, routeLoader$, routeLoaderQrl, RouterOutlet, server$, serverQrl, ServiceWorkerRegister, useContent, useDocumentHead, useLocation, useNavigate, validator$, validatorQrl, z2 as z, zod$, zodQrl };
```

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.mjs`. Massive single-module output. inlinedQrl used throughout (Smart/Lib mode keeps inline). Multiple Qwik helper functions used: _jsxBranch, _jsxSplit, _restProps, _fnSignal, _wrapSignal, _getContextElement, _deserializeData, _serializeData, _weakSerialize.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` throughout (Smart+Lib keeps code inline), `eventQrl()` for document events
- **[CONV-02] Dollar-to-Qrl**: All `$` functions -> `Qrl` forms (componentQrl, useStylesQrl, useTaskQrl, serverQrl, etc.)
- **[CONV-03] JSX Transforms**: All JSX transpiled via `_jsxSplit()` and `_jsxSorted()` (even without transpile_jsx, the pre-compiled input already uses these)
- **[CONV-04] Signal Helpers**: `_wrapSignal()` for reactive signal wrapping, `_fnSignal()` for computed signal expressions (e.g., `!p0.reloadDocument`, `p0.spaReset?"true":undefined`)
- **[CONV-05] Capture Patterns**: `useLexicalScope()` for accessing captured variables in inlinedQrl callbacks
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` and `/* @__PURE__ */` on component and context definitions
- **[CONV-10] Const Replacement**: `isServer`, `isBrowser`, `isDev` imported from `@qwik.dev/core/build` and used in conditionals
- **[CONV-11] Props Destructuring**: `_restProps()` used in Link component to separate known props from rest props

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | index.qwik.mjs | @qwik.dev/core | 6+ |
| inlinedQrl | index.qwik.mjs | @qwik.dev/core | 20+ |
| createContextId | index.qwik.mjs | @qwik.dev/core | 7 |
| useContext | index.qwik.mjs | @qwik.dev/core | 10+ |
| useContextProvider | index.qwik.mjs | @qwik.dev/core | 10+ |
| useTaskQrl | index.qwik.mjs | @qwik.dev/core | 3+ |
| useSignal | index.qwik.mjs | @qwik.dev/core | 5+ |
| useStore | index.qwik.mjs | @qwik.dev/core | 3+ |
| _fnSignal | index.qwik.mjs | @qwik.dev/core | 5+ |
| _restProps | index.qwik.mjs | @qwik.dev/core | 2+ |
| _wrapSignal | index.qwik.mjs | @qwik.dev/core | 1+ |
| _jsxSplit | index.qwik.mjs | @qwik.dev/core | 2+ |
| _jsxBranch | index.qwik.mjs | @qwik.dev/core | 1+ |
| implicit$FirstArg | index.qwik.mjs | @qwik.dev/core | 5+ |

## Diagnostics

```json
[]
```
