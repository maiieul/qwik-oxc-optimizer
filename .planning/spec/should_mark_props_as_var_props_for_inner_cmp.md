# Test: should_mark_props_as_var_props_for_inner_cmp

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code
```tsx
import { component$, useResource$, Resource } from "@qwik.dev/core";
import { type ModelProps } from "./modelMenu";
import { serverImg } from "~/routes/(authenticated)/layout";

export const Image = component$((props) => {
  return (
    <>
      <img src={`${props.src}`} />
    </>
  );
});

export const ModelImg = component$<ModelProps>((props) => {
  const imgLoc = useResource$(async ({ track }) => {
    track(() => props.store.model);
    return await serverImg('some.png');
  });
  return (
    <>
      <Resource
        value={imgLoc}
        onRejected={() => <p>error ...</p>}
        onResolved={(res) =>
          res && (
            <>
              <Image src={res} />
            </>
          )
        }
      />
    </>
  );
});
```

<details>
<summary>Input AST (OXC)</summary>

*Omitted for brevity -- parse with `oxc-ast-util tsx`*

</details>

## Output

### Module: test.js (main)

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_CS20HgBlRYI = ()=>import("./test.tsx_Image_component_CS20HgBlRYI");
const i_iJe6ICWVnyA = ()=>import("./test.tsx_ModelImg_component_iJe6ICWVnyA");
export const Image = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_CS20HgBlRYI, "Image_component_CS20HgBlRYI"));
export const ModelImg = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_iJe6ICWVnyA, "ModelImg_component_iJe6ICWVnyA"));
```

### Module: test.tsx_ModelImg_component_iJe6ICWVnyA.js (ENTRY POINT)

```javascript
import { Image } from "./test";
import { Resource } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useResourceQrl } from "@qwik.dev/core";
const i_Ogi9hEJvtmI = ()=>import("./test.tsx_ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI");
export const ModelImg_component_iJe6ICWVnyA = (props)=>{
    const imgLoc = useResourceQrl(/*#__PURE__*/ qrl(i_Ogi9hEJvtmI, "ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI", [props]));
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted(Resource, null, {
        value: imgLoc,
        onRejected: ()=>/*#__PURE__*/ _jsxSorted("p", null, null, "error ...", 3, "u6_1"),
        onResolved: (res)=>res && /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted(Image, { src: res }, null, null, 3, "u6_2"), 1, "u6_3")
    }, null, 3, "u6_4"), 1, "u6_5");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "ModelImg_component_iJe6ICWVnyA",
  "entry": null, "displayName": "test.tsx_ModelImg_component",
  "hash": "iJe6ICWVnyA", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [343, 780],
  "paramNames": ["props"]
}
```

### Module: test.tsx_ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { serverImg } from "~/routes/(authenticated)/layout";
export const ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI = async (_rawProps)=>{
    const props = _captures[0];
    _rawProps.track(()=>props.store.model);
    return await serverImg('some.png');
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI",
  "entry": null, "displayName": "test.tsx_ModelImg_component_imgLoc_useResource",
  "hash": "Ogi9hEJvtmI", "extension": "js",
  "parent": "ModelImg_component_iJe6ICWVnyA",
  "ctxKind": "function", "ctxName": "useResource$",
  "captures": true, "loc": [386, 488],
  "paramNames": ["_rawProps"], "captureNames": ["props"]
}
```

### Module: test.tsx_Image_component_CS20HgBlRYI.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
export const Image_component_CS20HgBlRYI = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("img", {
        src: `${props.src}`
    }, null, null, 3, null), 1, "u6_0");
};
```

#### Segment Metadata
```json
{
  "origin": "test.tsx",
  "name": "Image_component_CS20HgBlRYI",
  "entry": null, "displayName": "test.tsx_Image_component",
  "hash": "CS20HgBlRYI", "extension": "js",
  "parent": null, "ctxKind": "function", "ctxName": "component$",
  "captures": false, "loc": [212, 292],
  "paramNames": ["props"]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: `qrl()` for all segments; `useResourceQrl()` wraps the resource callback
- **[CONV-02] Dollar-to-QRL**: `component$` to `componentQrl`; `useResource$` to `useResourceQrl`
- **[CONV-03] JSX Transforms**: `_jsxSorted()` for all elements; `Image` component `src` prop in var props (because `props.src` uses template literal)
- **[CONV-05] Capture Patterns**: `_captures[0]` in useResource handler restores `props`
- **[CONV-06] Lazy Imports**: Lazy imports for all 3 component segments and useResource segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on all framework calls
- **[CONV-08] Segment Extraction**: 4 entry point modules: Image component, ModelImg component, useResource handler

**Key behavior**: The `Image` component has `src` as a var prop (`{src: \`${props.src}\`}`) because template literals with prop access are considered dynamic. The `Resource` component receives `onRejected` and `onResolved` as inline arrow functions (not extracted) because they are passed as const props. The `useResource$` callback captures `props` and renames its own parameter to `_rawProps` (the track/cleanup parameter object).

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | test.js | @qwik.dev/core | 2 |
| `qrl` | test.js, ModelImg segment | @qwik.dev/core | 3 |
| `useResourceQrl` | ModelImg segment | @qwik.dev/core | 1 |
| `_jsxSorted` | ModelImg/Image segments | @qwik.dev/core | 6 |
| `_captures` | useResource handler | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
