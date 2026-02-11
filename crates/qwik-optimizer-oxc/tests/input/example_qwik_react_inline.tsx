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