# Test: example_qwik_react_inline

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | ../node_modules/@qwik.dev/react/index.qwik.mjs |
| Entry Strategy | Inline |
| Mode | Test |
| Transpile TS | false |
| Transpile JSX | false |
| Explicit Extensions | true |

**Key insight:** Same Qwik React integration code as `example_qwik_react` but with Inline strategy. All code stays in one module. The `inlinedQrl` calls from the pre-compiled input are preserved (since Inline strategy keeps code inline). The `jsx()` calls are transformed to `_jsxSorted()`. The `filterProps` helper gets an `_auto_filterProps` export alias.

## Input

### Source Code

*Same pre-compiled Qwik React integration code as example_qwik_react (~90 lines).*

<details>
<summary>Input AST (OXC)</summary>

*MJS input. Same as example_qwik_react.*

</details>

## Output

### Module: ../node_modules/@qwik.dev/react/index.qwik.mjs (main module -- single output)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { componentQrl, inlinedQrl, useLexicalScope, useHostElement, useStore, useTaskQrl, noSerialize, SkipRerender, implicit$FirstArg } from '@qwik.dev/core';
import { Fragment } from '@qwik.dev/core/jsx-runtime';
import { isBrowser, isServer } from '@qwik.dev/core';
function qwikifyQrl(reactCmpQrl) {
    return /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
        const [reactCmpQrl] = useLexicalScope();
        // ... full component body inline ...
        useTaskQrl(/*#__PURE__*/ inlinedQrl(async (track)=>{
            // ... task handler inline ...
        }, "qwikifyQrl_component_useWatch_x04JC5xeP1U", [hostElement, props, reactCmpQrl, store]), { run });
        // ... server/client rendering with _jsxSorted ...
    }, "qwikifyQrl_component_zH94hIe0Ick", [reactCmpQrl]), {
        tagName: 'qwik-wrap'
    });
}
const filterProps = (props)=>{ /* ... */ };
const qwikify$ = implicit$FirstArg(qwikifyQrl);
async function renderToString(rootNode, opts) { /* ... */ }
export { qwikify$, qwikifyQrl, renderToString };
export { filterProps as _auto_filterProps };
```

*Full module is ~190 lines with all code inline.*

<details>
<summary>Output AST (OXC)</summary>

*Parsed as `.mjs`. All code in one module. inlinedQrl calls preserved from input. jsx() calls transformed to _jsxSorted(). _auto_filterProps alias added.*

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: `inlinedQrl()` preserved from input (Inline strategy)
- **[CONV-03] JSX Transforms**: `jsx()` from input transformed to `_jsxSorted()` calls
- **[CONV-05] Capture Patterns**: Capture arrays in inlinedQrl third argument preserved
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` added on componentQrl and inlinedQrl calls
- **Note**: No CONV-06/08 -- Inline strategy, no segment extraction or lazy imports

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | index.qwik.mjs | @qwik.dev/core | 1 |
| inlinedQrl | index.qwik.mjs | @qwik.dev/core | 2 |
| useLexicalScope | index.qwik.mjs | @qwik.dev/core | 2 |
| useTaskQrl | index.qwik.mjs | @qwik.dev/core | 1 |
| _jsxSorted | index.qwik.mjs | @qwik.dev/core | 4 |
| implicit$FirstArg | index.qwik.mjs | @qwik.dev/core | 1 |
| noSerialize | index.qwik.mjs | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
