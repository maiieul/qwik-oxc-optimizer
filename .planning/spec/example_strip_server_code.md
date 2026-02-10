# Test: example_strip_server_code

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Prod (non-default) |
| Transpile TS | true |
| Transpile JSX | true |
| Strip Ctx Name | ["server"] (non-default) |

## Input

### Source Code
```tsx
import { component$, serverLoader$, serverStuff$, $, client$, useStore, useTask$ } from '@qwik.dev/core';
import { isServer } from '@qwik.dev/core';
import mongo from 'mongodb';
import redis from 'redis';
import { handler } from 'serverless';

export const Parent = component$(() => {
	const state = useStore({
		text: ''
	});

	// Double count watch
	useTask$(async () => {
		if (!isServer) return;
		state.text = await mongo.users();
		redis.set(state.text);
	});

	serverStuff$(async () => {
		// should be removed too
		const a = $(() => {
			// from $(), should not be removed
		});
		const b = client$(() => {
			// from clien$(), should not be removed
		});
		return [a,b];
	})

	serverLoader$(handler);

	useTask$(() => {
		// Code
	});

	return (
		<div onClick$={() => console.log('parent')}>
			{state.text}
		</div>
	);
});
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
const i_0TaiDayHrlo = ()=>import("./test.tsx_Parent_component_0TaiDayHrlo");
export const Parent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0TaiDayHrlo, "s_0TaiDayHrlo"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util js`

</details>

### Module: test.tsx_Parent_component_useTask_gDH1EtUWqBU.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import mongo from "mongodb";
import redis from "redis";
export const s_gDH1EtUWqBU = async ()=>{
    const state = _captures[0];
    state.text = await mongo.users();
    redis.set(state.text);
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
  "name": "s_gDH1EtUWqBU",
  "entry": null,
  "displayName": "test.tsx_Parent_component_useTask",
  "hash": "gDH1EtUWqBU",
  "canonicalFilename": "test.tsx_Parent_component_useTask_gDH1EtUWqBU",
  "path": "",
  "extension": "js",
  "parent": "s_0TaiDayHrlo",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": true,
  "loc": [363, 465],
  "captureNames": ["state"]
}
```

### Module: test.tsx_Parent_component_serverStuff_a_2ca3HLDC7yc.js (ENTRY POINT)

```javascript
export const s_2ca3HLDC7yc = ()=>{
// from $(), should not be removed
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "s_2ca3HLDC7yc",
  "entry": null,
  "displayName": "test.tsx_Parent_component_serverStuff_a",
  "hash": "2ca3HLDC7yc",
  "canonicalFilename": "test.tsx_Parent_component_serverStuff_a_2ca3HLDC7yc",
  "path": "",
  "extension": "js",
  "parent": "s_r1qAHX7Opp0",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [538, 587]
}
```

### Module: test.tsx_Parent_component_serverStuff_b_client_v9qawr2Inkk.js (ENTRY POINT)

```javascript
export const s_v9qawr2Inkk = ()=>{
// from clien$(), should not be removed
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "s_v9qawr2Inkk",
  "entry": null,
  "displayName": "test.tsx_Parent_component_serverStuff_b_client",
  "hash": "v9qawr2Inkk",
  "canonicalFilename": "test.tsx_Parent_component_serverStuff_b_client_v9qawr2Inkk",
  "path": "",
  "extension": "js",
  "parent": "s_r1qAHX7Opp0",
  "ctxKind": "function",
  "ctxName": "client$",
  "captures": false,
  "loc": [610, 664]
}
```

### Module: test.tsx_Parent_component_0TaiDayHrlo.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _noopQrl } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { serverLoaderQrl } from "@qwik.dev/core";
import { serverStuffQrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const i_P8oRQhHsurk = ()=>import("./test.tsx_Parent_component_useTask_1_P8oRQhHsurk");
const i_gDH1EtUWqBU = ()=>import("./test.tsx_Parent_component_useTask_gDH1EtUWqBU");
const i_zM9okM0TYrA = ()=>import("./test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA");
export const s_0TaiDayHrlo = ()=>{
    const state = useStore({
        text: ''
    });
    // Double count watch
    useTaskQrl(/*#__PURE__*/ qrl(i_gDH1EtUWqBU, "s_gDH1EtUWqBU", [
        state
    ]));
    serverStuffQrl(/*#__PURE__*/ _noopQrl("s_r1qAHX7Opp0"));
    serverLoaderQrl(/*#__PURE__*/ _noopQrl("s_k1L0DiPQV1I"));
    useTaskQrl(/*#__PURE__*/ qrl(i_P8oRQhHsurk, "s_P8oRQhHsurk"));
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_zM9okM0TYrA, "s_zM9okM0TYrA")
    }, _wrapProp(state, "text"), 3, "u6_0");
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
  "name": "s_0TaiDayHrlo",
  "entry": null,
  "displayName": "test.tsx_Parent_component",
  "hash": "0TaiDayHrlo",
  "canonicalFilename": "test.tsx_Parent_component_0TaiDayHrlo",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [279, 835]
}
```

### Module: test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA.js (ENTRY POINT)

```javascript
export const s_zM9okM0TYrA = ()=>console.log('parent');
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "s_zM9okM0TYrA",
  "entry": null,
  "displayName": "test.tsx_Parent_component_div_q_e_click",
  "hash": "zM9okM0TYrA",
  "canonicalFilename": "test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA",
  "path": "",
  "extension": "js",
  "parent": "s_0TaiDayHrlo",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [775, 802]
}
```

### Module: test.tsx_Parent_component_useTask_1_P8oRQhHsurk.js (ENTRY POINT)

```javascript
export const s_P8oRQhHsurk = ()=>{
// Code
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "s_P8oRQhHsurk",
  "entry": null,
  "displayName": "test.tsx_Parent_component_useTask_1",
  "hash": "P8oRQhHsurk",
  "canonicalFilename": "test.tsx_Parent_component_useTask_1_P8oRQhHsurk",
  "path": "",
  "extension": "js",
  "parent": "s_0TaiDayHrlo",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": false,
  "loc": [724, 744]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` with `s_HASH` naming (Prod mode). Used for component, useTask, and onClick handler references
- **[CONV-02] Dollar-to-Qrl**: `component$` to `componentQrl()`, `useTask$` to `useTaskQrl()`, `serverStuff$` to `serverStuffQrl()`, `serverLoader$` to `serverLoaderQrl()`, `$()` and `client$()` are extracted as segments
- **[CONV-03] JSX Transforms**: `_jsxSorted("div", ...)` replaces JSX
- **[CONV-04] Signal Helpers**: `_wrapProp(state, "text")` for reactive text display
- **[CONV-05] Capture Patterns**: `const state = _captures[0]` in the first useTask segment
- **[CONV-06] Lazy Imports**: Dynamic imports for all non-stripped segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all QRL and JSX calls
- **[CONV-08] Segment Extraction**: Seven entry points total -- component body, two useTasks, onClick, and two nested `$`/`client$` segments from within the stripped serverStuff
- **[CONV-09] Code Stripping**: `serverStuff$` and `serverLoader$` callbacks stripped to `_noopQrl("s_HASH")` because `strip_ctx_name: ["server"]`. The `if (!isServer) return;` guard in the first useTask is removed (dead code in Prod mode where isServer is known). However, the nested `$()` and `client$()` inside the stripped `serverStuff$` are NOT stripped -- they produce their own entry point segments
- **[CONV-10] Const Replacement**: The `if (!isServer) return;` guard in the first useTask is eliminated because in Prod mode the const replacement evaluates `isServer` and removes the dead branch

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.js, component entry | @qwik.dev/core | 4 |
| `_noopQrl` | component entry | @qwik.dev/core | 2 |
| `serverStuffQrl` | component entry | @qwik.dev/core | 1 |
| `serverLoaderQrl` | component entry | @qwik.dev/core | 1 |
| `useTaskQrl` | component entry | @qwik.dev/core | 2 |
| `useStore` | component entry | @qwik.dev/core | 1 |
| `_jsxSorted` | component entry | @qwik.dev/core | 1 |
| `_wrapProp` | component entry | @qwik.dev/core | 1 |
| `_captures` | useTask entry | @qwik.dev/core | 1 (array access) |

## Diagnostics

```json
[]
```
