# Test: example_1

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

## Input

### Source Code

```tsx
import { $, component, onRender } from '@qwik.dev/core';

export const renderHeader = $(() => {
	return (
		<div onClick={$((ctx) => console.log(ctx))}/>
	);
});
const renderHeader = component($(() => {
	console.log("mount");
	return render;
}));
```

<details>
<summary>Input AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 10
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 10
          },
          "importKind": "value",
          "start": 9,
          "end": 10
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "component",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 21
          },
          "importKind": "value",
          "start": 12,
          "end": 21
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "onRender",
            "optional": false,
            "typeAnnotation": null,
            "start": 23,
            "end": 31
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "onRender",
            "optional": false,
            "typeAnnotation": null,
            "start": 23,
            "end": 31
          },
          "importKind": "value",
          "start": 23,
          "end": 31
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 39,
        "end": 55
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 56
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "decorators": [],
              "name": "renderHeader",
              "optional": false,
              "typeAnnotation": null,
              "start": 71,
              "end": 83
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "$",
                "optional": false,
                "typeAnnotation": null,
                "start": 86,
                "end": 87
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "ArrowFunctionExpression",
                  "expression": false,
                  "async": false,
                  "typeParameters": null,
                  "params": [],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 109,
                                "end": 112
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick",
                                    "start": 113,
                                    "end": 120
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "$",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 122,
                                        "end": 123
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "ArrowFunctionExpression",
                                          "expression": true,
                                          "async": false,
                                          "typeParameters": null,
                                          "params": [
                                            {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "ctx",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 125,
                                              "end": 128
                                            }
                                          ],
                                          "returnType": null,
                                          "body": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "console",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 133,
                                                "end": 140
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 141,
                                                "end": 144
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 133,
                                              "end": 144
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ctx",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 145,
                                                "end": 148
                                              }
                                            ],
                                            "optional": false,
                                            "start": 133,
                                            "end": 149
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 124,
                                          "end": 149
                                        }
                                      ],
                                      "optional": false,
                                      "start": 122,
                                      "end": 150
                                    },
                                    "start": 121,
                                    "end": 151
                                  },
                                  "start": 113,
                                  "end": 151
                                }
                              ],
                              "selfClosing": true,
                              "start": 108,
                              "end": 153
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 108,
                            "end": 153
                          },
                          "start": 104,
                          "end": 156
                        },
                        "start": 97,
                        "end": 157
                      }
                    ],
                    "start": 94,
                    "end": 159
                  },
                  "id": null,
                  "generator": false,
                  "start": 88,
                  "end": 159
                }
              ],
              "optional": false,
              "start": 86,
              "end": 160
            },
            "definite": false,
            "start": 71,
            "end": 160
          }
        ],
        "declare": false,
        "start": 65,
        "end": 161
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 58,
      "end": 161
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "renderHeader",
            "optional": false,
            "typeAnnotation": null,
            "start": 168,
            "end": 180
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component",
              "optional": false,
              "typeAnnotation": null,
              "start": 183,
              "end": 192
            },
            "typeArguments": null,
            "arguments": [
              {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "$",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 193,
                  "end": 194
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "ArrowFunctionExpression",
                    "expression": false,
                    "async": false,
                    "typeParameters": null,
                    "params": [],
                    "returnType": null,
                    "body": {
                      "type": "BlockStatement",
                      "body": [
                        {
                          "type": "ExpressionStatement",
                          "expression": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "console",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 204,
                                "end": 211
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "log",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 212,
                                "end": 215
                              },
                              "optional": false,
                              "computed": false,
                              "start": 204,
                              "end": 215
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "mount",
                                "raw": "\"mount\"",
                                "start": 216,
                                "end": 223
                              }
                            ],
                            "optional": false,
                            "start": 204,
                            "end": 224
                          },
                          "directive": null,
                          "start": 204,
                          "end": 225
                        },
                        {
                          "type": "ReturnStatement",
                          "argument": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "render",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 234,
                            "end": 240
                          },
                          "start": 227,
                          "end": 241
                        }
                      ],
                      "start": 201,
                      "end": 243
                    },
                    "id": null,
                    "generator": false,
                    "start": 195,
                    "end": 243
                  }
                ],
                "optional": false,
                "start": 193,
                "end": 244
              }
            ],
            "optional": false,
            "start": 183,
            "end": 245
          },
          "definite": false,
          "start": 168,
          "end": 245
        }
      ],
      "declare": false,
      "start": 162,
      "end": 246
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 246
}
```

</details>

## Output

### Module: test.tsx_renderHeader_zBbHWn4e8Cg.tsx [ENTRY POINT]

```tsx
import { qrl } from "@qwik.dev/core";
const i_fV2uzAL99u4 = ()=>import("./test.tsx_renderHeader_div_onClick_fV2uzAL99u4");
export const renderHeader_zBbHWn4e8Cg = ()=>{
    return <div onClick={/*#__PURE__*/ qrl(i_fV2uzAL99u4, "renderHeader_div_onClick_fV2uzAL99u4")}/>;
};
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "importKind": "value",
          "start": 9,
          "end": 12
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 20,
        "end": 36
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 37
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "i_fV2uzAL99u4",
            "optional": false,
            "typeAnnotation": null,
            "start": 44,
            "end": 57
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_renderHeader_div_onClick_fV2uzAL99u4",
                "raw": "\"./test.tsx_renderHeader_div_onClick_fV2uzAL99u4\"",
                "start": 71,
                "end": 120
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 121
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 121
          },
          "definite": false,
          "start": 44,
          "end": 121
        }
      ],
      "declare": false,
      "start": 38,
      "end": 122
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "decorators": [],
              "name": "renderHeader_zBbHWn4e8Cg",
              "optional": false,
              "typeAnnotation": null,
              "start": 136,
              "end": 160
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "typeParameters": null,
              "params": [],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 181,
                          "end": 184
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick",
                              "start": 185,
                              "end": 192
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "qrl",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 208,
                                  "end": 211
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "i_fV2uzAL99u4",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 212,
                                    "end": 225
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "renderHeader_div_onClick_fV2uzAL99u4",
                                    "raw": "\"renderHeader_div_onClick_fV2uzAL99u4\"",
                                    "start": 227,
                                    "end": 265
                                  }
                                ],
                                "optional": false,
                                "start": 208,
                                "end": 266
                              },
                              "start": 193,
                              "end": 267
                            },
                            "start": 185,
                            "end": 267
                          }
                        ],
                        "selfClosing": true,
                        "start": 180,
                        "end": 269
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 180,
                      "end": 269
                    },
                    "start": 173,
                    "end": 270
                  }
                ],
                "start": 167,
                "end": 272
              },
              "id": null,
              "generator": false,
              "start": 163,
              "end": 272
            },
            "definite": false,
            "start": 136,
            "end": 272
          }
        ],
        "declare": false,
        "start": 130,
        "end": 273
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 123,
      "end": 273
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 273
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "renderHeader_zBbHWn4e8Cg",
  "entry": null,
  "displayName": "test.tsx_renderHeader",
  "hash": "zBbHWn4e8Cg",
  "canonicalFilename": "test.tsx_renderHeader_zBbHWn4e8Cg",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    90,
    161
  ]
}
```

### Module: test.tsx_renderHeader_component_U6Kkv07sbpQ.tsx [ENTRY POINT]

```tsx
export const renderHeader_component_U6Kkv07sbpQ = ()=>{
    console.log("mount");
    return render;
};
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "decorators": [],
              "name": "renderHeader_component_U6Kkv07sbpQ",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 47
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "typeParameters": null,
              "params": [],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "console",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 60,
                          "end": 67
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "log",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 68,
                          "end": 71
                        },
                        "optional": false,
                        "computed": false,
                        "start": 60,
                        "end": 71
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "mount",
                          "raw": "\"mount\"",
                          "start": 72,
                          "end": 79
                        }
                      ],
                      "optional": false,
                      "start": 60,
                      "end": 80
                    },
                    "directive": null,
                    "start": 60,
                    "end": 81
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "render",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 93,
                      "end": 99
                    },
                    "start": 86,
                    "end": 100
                  }
                ],
                "start": 54,
                "end": 102
              },
              "id": null,
              "generator": false,
              "start": 50,
              "end": 102
            },
            "definite": false,
            "start": 13,
            "end": 102
          }
        ],
        "declare": false,
        "start": 7,
        "end": 103
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 103
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 103
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "renderHeader_component_U6Kkv07sbpQ",
  "entry": null,
  "displayName": "test.tsx_renderHeader_component",
  "hash": "U6Kkv07sbpQ",
  "canonicalFilename": "test.tsx_renderHeader_component_U6Kkv07sbpQ",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    197,
    245
  ]
}
```

### Module: test.tsx

```tsx
import { qrl } from "@qwik.dev/core";
const i_U6Kkv07sbpQ = ()=>import("./test.tsx_renderHeader_component_U6Kkv07sbpQ");
const i_zBbHWn4e8Cg = ()=>import("./test.tsx_renderHeader_zBbHWn4e8Cg");
import { component } from '@qwik.dev/core';
export const renderHeader = /*#__PURE__*/ qrl(i_zBbHWn4e8Cg, "renderHeader_zBbHWn4e8Cg");
const renderHeader = component(/*#__PURE__*/ qrl(i_U6Kkv07sbpQ, "renderHeader_component_U6Kkv07sbpQ"));
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "importKind": "value",
          "start": 9,
          "end": 12
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 20,
        "end": 36
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 37
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "i_U6Kkv07sbpQ",
            "optional": false,
            "typeAnnotation": null,
            "start": 44,
            "end": 57
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_renderHeader_component_U6Kkv07sbpQ",
                "raw": "\"./test.tsx_renderHeader_component_U6Kkv07sbpQ\"",
                "start": 71,
                "end": 118
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 119
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 119
          },
          "definite": false,
          "start": 44,
          "end": 119
        }
      ],
      "declare": false,
      "start": 38,
      "end": 120
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "i_zBbHWn4e8Cg",
            "optional": false,
            "typeAnnotation": null,
            "start": 127,
            "end": 140
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_renderHeader_zBbHWn4e8Cg",
                "raw": "\"./test.tsx_renderHeader_zBbHWn4e8Cg\"",
                "start": 154,
                "end": 191
              },
              "options": null,
              "phase": null,
              "start": 147,
              "end": 192
            },
            "id": null,
            "generator": false,
            "start": 143,
            "end": 192
          },
          "definite": false,
          "start": 127,
          "end": 192
        }
      ],
      "declare": false,
      "start": 121,
      "end": 193
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "component",
            "optional": false,
            "typeAnnotation": null,
            "start": 203,
            "end": 212
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component",
            "optional": false,
            "typeAnnotation": null,
            "start": 203,
            "end": 212
          },
          "importKind": "value",
          "start": 203,
          "end": 212
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 220,
        "end": 236
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 194,
      "end": 237
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "decorators": [],
              "name": "renderHeader",
              "optional": false,
              "typeAnnotation": null,
              "start": 251,
              "end": 263
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "qrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 280,
                "end": 283
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "i_zBbHWn4e8Cg",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 284,
                  "end": 297
                },
                {
                  "type": "Literal",
                  "value": "renderHeader_zBbHWn4e8Cg",
                  "raw": "\"renderHeader_zBbHWn4e8Cg\"",
                  "start": 299,
                  "end": 325
                }
              ],
              "optional": false,
              "start": 280,
              "end": 326
            },
            "definite": false,
            "start": 251,
            "end": 326
          }
        ],
        "declare": false,
        "start": 245,
        "end": 327
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 238,
      "end": 327
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "renderHeader",
            "optional": false,
            "typeAnnotation": null,
            "start": 334,
            "end": 346
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component",
              "optional": false,
              "typeAnnotation": null,
              "start": 349,
              "end": 358
            },
            "typeArguments": null,
            "arguments": [
              {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "qrl",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 373,
                  "end": 376
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "i_U6Kkv07sbpQ",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 377,
                    "end": 390
                  },
                  {
                    "type": "Literal",
                    "value": "renderHeader_component_U6Kkv07sbpQ",
                    "raw": "\"renderHeader_component_U6Kkv07sbpQ\"",
                    "start": 392,
                    "end": 428
                  }
                ],
                "optional": false,
                "start": 373,
                "end": 429
              }
            ],
            "optional": false,
            "start": 349,
            "end": 430
          },
          "definite": false,
          "start": 334,
          "end": 430
        }
      ],
      "declare": false,
      "start": 328,
      "end": 431
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 431
}
```

</details>

### Module: test.tsx_renderHeader_div_onClick_fV2uzAL99u4.tsx [ENTRY POINT]

```tsx
export const renderHeader_div_onClick_fV2uzAL99u4 = (ctx)=>console.log(ctx);
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "decorators": [],
              "name": "renderHeader_div_onClick_fV2uzAL99u4",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 49
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "typeParameters": null,
              "params": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "ctx",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 53,
                  "end": 56
                }
              ],
              "returnType": null,
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "console",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 59,
                    "end": 66
                  },
                  "property": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "log",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 67,
                    "end": 70
                  },
                  "optional": false,
                  "computed": false,
                  "start": 59,
                  "end": 70
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "ctx",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 71,
                    "end": 74
                  }
                ],
                "optional": false,
                "start": 59,
                "end": 75
              },
              "id": null,
              "generator": false,
              "start": 52,
              "end": 75
            },
            "definite": false,
            "start": 13,
            "end": 75
          }
        ],
        "declare": false,
        "start": 7,
        "end": 76
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 76
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 76
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "renderHeader_div_onClick_fV2uzAL99u4",
  "entry": null,
  "displayName": "test.tsx_renderHeader_div_onClick",
  "hash": "fV2uzAL99u4",
  "canonicalFilename": "test.tsx_renderHeader_div_onClick_fV2uzAL99u4",
  "path": "",
  "extension": "tsx",
  "parent": "renderHeader_zBbHWn4e8Cg",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    126,
    151
  ],
  "paramNames": [
    "ctx"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (3 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx_renderHeader_zBbHWn4e8Cg.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 2 |

## Diagnostics

No diagnostics.
