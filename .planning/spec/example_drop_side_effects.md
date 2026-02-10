# Test: example_drop_side_effects

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Dev (non-default) |
| Transpile TS | true |
| Transpile JSX | true |
| Strip Ctx Name | ["server"] (non-default) |
| Is Server | false (non-default) |

## Input

### Source Code
```tsx
import { component$ } from '@qwik.dev/core';
import { server$ } from '@qwik.dev/router';
import { clientSupabase } from 'supabase';
import { Client } from 'openai';
import { secret } from './secret';
import { sideEffect } from './secret';

const supabase = clientSupabase();
const dfd = new Client(secret);

(function() {
	console.log('run');
	})();
	(() => {
	console.log('run');
	})();

sideEffect();

export const api = server$(() => {
	supabase.from('ffg').do(dfd);
});

export default component$(() => {
	return (
		<button onClick$={() => await api()}></button>
	)
	});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { serverQrl } from "@qwik.dev/router";
import { _noopQrlDEV } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
import { sideEffect } from './secret';
(function() {
    console.log('run');
})();
(()=>{
    console.log('run');
})();
sideEffect();
export const api = serverQrl(/*#__PURE__*/ _noopQrlDEV("api_server_JonPp043gH0", {
    file: "/user/qwik/src/test.tsx",
    lo: 0,
    hi: 0,
    displayName: "test.tsx_api_server"
}));
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrlDEV(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg", {
    file: "/user/qwik/src/test.tsx",
    lo: 503,
    hi: 575,
    displayName: "test.tsx_test_component"
}));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_test_component_button_q_e_click_qwSL5gM03T4.js (ENTRY POINT)

```javascript
import { api } from "./test";
export const test_component_button_q_e_click_qwSL5gM03T4 = ()=>await api();
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "test_component_button_q_e_click_qwSL5gM03T4",
  "entry": null,
  "displayName": "test.tsx_test_component_button_q_e_click",
  "hash": "qwSL5gM03T4",
  "canonicalFilename": "test.tsx_test_component_button_q_e_click_qwSL5gM03T4",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [541, 558]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_qwSL5gM03T4 = ()=>import("./test.tsx_test_component_button_q_e_click_qwSL5gM03T4");
export const test_component_LUXeXe0DQrg = ()=>{
    return /*#__PURE__*/ _jsxSorted("button", null, {
        "q-e:click": /*#__PURE__*/ qrlDEV(i_qwSL5gM03T4, "test_component_button_q_e_click_qwSL5gM03T4", {
            file: "/user/qwik/src/test.tsx",
            lo: 541,
            hi: 558,
            displayName: "test.tsx_test_component_button_q_e_click"
        })
    }, null, 3, "u6_0", {
        fileName: "test.tsx",
        lineNumber: 27,
        columnNumber: 3
    });
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
  "loc": [503, 575]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls (Dev variant)**: `qrlDEV()` with source metadata for component and onClick handler. `_noopQrlDEV()` for stripped server$ code
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `server$` to `serverQrl()`
- **[CONV-03] JSX Transforms**: `_jsxSorted("button", ...)` with dev-mode source location
- **[CONV-06] Lazy Imports**: Dynamic imports for component and onClick handler segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on QRL, noop, and JSX calls
- **[CONV-08] Segment Extraction**: Component body and onClick handler extracted
- **[CONV-09] Code Stripping**: `server$` callback stripped to `_noopQrlDEV()` because `strip_ctx_name: ["server"]` with `is_server: false` (client-side build). The server callback body is completely removed

**Key side-effect dropping behavior:**
1. Imports used ONLY by the stripped `server$` callback are removed: `clientSupabase` from 'supabase', `Client` from 'openai', `secret` from './secret'
2. Module-level side-effect expressions (`const supabase = clientSupabase()`, `const dfd = new Client(secret)`) are ALSO dropped because they only feed into the stripped server code
3. However, IIFE side effects (`(function(){...})()`, `(()=>{...})()`) and `sideEffect()` calls are PRESERVED because they have observable side effects unrelated to the stripped code
4. The `import { sideEffect } from './secret'` is kept because `sideEffect` is still called

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `serverQrl` | test.js | @qwik.dev/router | 1 |
| `_noopQrlDEV` | test.js | @qwik.dev/core | 1 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrlDEV` | test.js, component entry | @qwik.dev/core | 2 |
| `_jsxSorted` | component entry | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
