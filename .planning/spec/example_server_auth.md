# Test: example_server_auth

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
import GitHub from '@auth/core/providers/github'
import Facebook from 'next-auth/providers/facebook'
import Google from 'next-auth/providers/google'
import {serverAuth$, auth$} from '@auth/qwik';

export const { onRequest, logout, getSession, signup } = serverAuth$({
	providers: [
	GitHub({ clientId: process.env.GITHUB_ID, clientSecret: process.env.GITHUB_SECRET }),
	Facebook({ clientId: import.meta.env.FACEBOOK_ID, clientSecret: import.meta.env.FACEBOOK_SECRET }),
	Google({ clientId: process.env.GOOGLE_ID, clientSecret: process.env.GOOGLE_SECRET })
	]
});

export const { onRequest, logout, getSession, signup } = auth$({
	providers: [
	GitHub({ clientId: process.env.GITHUB_ID, clientSecret: process.env.GITHUB_SECRET }),
	Facebook({ clientId: process.env.FACEBOOK_ID, clientSecret: process.env.FACEBOOK_SECRET }),
	Google({ clientId: process.env.GOOGLE_ID, clientSecret: process.env.GOOGLE_SECRET })
	]
});
```

<details>
<summary>Input AST (OXC)</summary>

AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`

</details>

## Output

### Module: test.js

```javascript
import { serverAuthQrl } from "@auth/qwik";
import { qrl } from "@qwik.dev/core";
import { authQrl } from "@auth/qwik";
const i_GU0aY5QCETY = ()=>import("./test.tsx_auth_GU0aY5QCETY");
const i_qVqpX2a0p9Y = ()=>import("./test.tsx_serverAuth_qVqpX2a0p9Y");
export const { onRequest, logout, getSession, signup } = serverAuthQrl(/*#__PURE__*/ qrl(i_qVqpX2a0p9Y, "serverAuth_qVqpX2a0p9Y"));
export const { onRequest, logout, getSession, signup } = authQrl(/*#__PURE__*/ qrl(i_GU0aY5QCETY, "auth_GU0aY5QCETY"));
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

### Module: test.tsx_serverAuth_qVqpX2a0p9Y.js (ENTRY POINT)

```javascript
import Facebook from "next-auth/providers/facebook";
import GitHub from "@auth/core/providers/github";
import Google from "next-auth/providers/google";
export const serverAuth_qVqpX2a0p9Y = {
    providers: [
        GitHub({ clientId: process.env.GITHUB_ID, clientSecret: process.env.GITHUB_SECRET }),
        Facebook({ clientId: import.meta.env.FACEBOOK_ID, clientSecret: import.meta.env.FACEBOOK_SECRET }),
        Google({ clientId: process.env.GOOGLE_ID, clientSecret: process.env.GOOGLE_SECRET })
    ]
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "serverAuth_qVqpX2a0p9Y",
  "entry": null,
  "displayName": "test.tsx_serverAuth",
  "hash": "qVqpX2a0p9Y",
  "canonicalFilename": "test.tsx_serverAuth_qVqpX2a0p9Y",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "serverAuth$",
  "captures": false,
  "loc": [268, 577]
}
```

### Module: test.tsx_auth_GU0aY5QCETY.js (ENTRY POINT)

```javascript
import Facebook from "next-auth/providers/facebook";
import GitHub from "@auth/core/providers/github";
import Google from "next-auth/providers/google";
export const auth_GU0aY5QCETY = {
    providers: [
        GitHub({ clientId: process.env.GITHUB_ID, clientSecret: process.env.GITHUB_SECRET }),
        Facebook({ clientId: process.env.FACEBOOK_ID, clientSecret: process.env.FACEBOOK_SECRET }),
        Google({ clientId: process.env.GOOGLE_ID, clientSecret: process.env.GOOGLE_SECRET })
    ]
};
```

<details>
<summary>Output AST (OXC)</summary>

AST omitted for brevity.

</details>

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "auth_GU0aY5QCETY",
  "entry": null,
  "displayName": "test.tsx_auth",
  "hash": "GU0aY5QCETY",
  "canonicalFilename": "test.tsx_auth_GU0aY5QCETY",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "auth$",
  "captures": false,
  "loc": [644, 945]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for both auth config segments
- **[CONV-02] Dollar-to-Qrl**: `serverAuth$` to `serverAuthQrl()`, `auth$` to `authQrl()` (third-party functions from `@auth/qwik`)
- **[CONV-06] Lazy Imports**: Dynamic imports for auth config segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `qrl()` calls
- **[CONV-08] Segment Extraction**: Config objects extracted to entry points with their provider imports

**Key behavior:** Third-party `$`-suffixed functions handled same as core Qwik functions. Config object (not function) exported directly.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `serverAuthQrl` | test.js | @auth/qwik | 1 |
| `authQrl` | test.js | @auth/qwik | 1 |
| `qrl` | test.js | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
