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