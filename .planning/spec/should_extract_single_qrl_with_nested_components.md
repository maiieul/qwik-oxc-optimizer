# Test: should_extract_single_qrl_with_nested_components

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { $, component$, useSignal, Signal } from '@qwik.dev/core';
const Foo = component$(() => {
  const data = useSignal<Signal<any>[]>([]);
  const Inner = component$((props) => {
    const data = props.data
    return <div>{data.value.map(item => <p onClick$={() => console.log(item.value.id)}>{item.value.id}</p>)}</div>
  })
  return <Inner data={data} />
})
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
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 22
          },
          "importKind": "value",
          "start": 12,
          "end": 22
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 33
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 33
          },
          "importKind": "value",
          "start": 24,
          "end": 33
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "Signal",
            "optional": false,
            "typeAnnotation": null,
            "start": 35,
            "end": 41
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Signal",
            "optional": false,
            "typeAnnotation": null,
            "start": 35,
            "end": 41
          },
          "importKind": "value",
          "start": 35,
          "end": 41
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 49,
        "end": 65
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 66
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
            "name": "Foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 73,
            "end": 76
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 79,
              "end": 89
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
                      "type": "VariableDeclaration",
                      "kind": "const",
                      "declarations": [
                        {
                          "type": "VariableDeclarator",
                          "id": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "data",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 106,
                            "end": 110
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 113,
                              "end": 122
                            },
                            "typeArguments": {
                              "type": "TSTypeParameterInstantiation",
                              "params": [
                                {
                                  "type": "TSArrayType",
                                  "elementType": {
                                    "type": "TSTypeReference",
                                    "typeName": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "Signal",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 123,
                                      "end": 129
                                    },
                                    "typeArguments": {
                                      "type": "TSTypeParameterInstantiation",
                                      "params": [
                                        {
                                          "type": "TSAnyKeyword",
                                          "start": 130,
                                          "end": 133
                                        }
                                      ],
                                      "start": 129,
                                      "end": 134
                                    },
                                    "start": 123,
                                    "end": 134
                                  },
                                  "start": 123,
                                  "end": 136
                                }
                              ],
                              "start": 122,
                              "end": 137
                            },
                            "arguments": [
                              {
                                "type": "ArrayExpression",
                                "elements": [],
                                "start": 138,
                                "end": 140
                              }
                            ],
                            "optional": false,
                            "start": 113,
                            "end": 141
                          },
                          "definite": false,
                          "start": 106,
                          "end": 141
                        }
                      ],
                      "declare": false,
                      "start": 100,
                      "end": 142
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
                            "name": "Inner",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 151,
                            "end": 156
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "component$",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 159,
                              "end": 169
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "ArrowFunctionExpression",
                                "expression": false,
                                "async": false,
                                "typeParameters": null,
                                "params": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "props",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 171,
                                    "end": 176
                                  }
                                ],
                                "returnType": null,
                                "body": {
                                  "type": "BlockStatement",
                                  "body": [
                                    {
                                      "type": "VariableDeclaration",
                                      "kind": "const",
                                      "declarations": [
                                        {
                                          "type": "VariableDeclarator",
                                          "id": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "data",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 193,
                                            "end": 197
                                          },
                                          "init": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "props",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 200,
                                              "end": 205
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "data",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 206,
                                              "end": 210
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 200,
                                            "end": 210
                                          },
                                          "definite": false,
                                          "start": 193,
                                          "end": 210
                                        }
                                      ],
                                      "declare": false,
                                      "start": 187,
                                      "end": 210
                                    },
                                    {
                                      "type": "ReturnStatement",
                                      "argument": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 223,
                                            "end": 226
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 222,
                                          "end": 227
                                        },
                                        "children": [
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "CallExpression",
                                              "callee": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "data",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 228,
                                                    "end": 232
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "value",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 233,
                                                    "end": 238
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 228,
                                                  "end": 238
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "map",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 239,
                                                  "end": 242
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 228,
                                                "end": 242
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
                                                      "name": "item",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 243,
                                                      "end": 247
                                                    }
                                                  ],
                                                  "returnType": null,
                                                  "body": {
                                                    "type": "JSXElement",
                                                    "openingElement": {
                                                      "type": "JSXOpeningElement",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "p",
                                                        "start": 252,
                                                        "end": 253
                                                      },
                                                      "typeArguments": null,
                                                      "attributes": [
                                                        {
                                                          "type": "JSXAttribute",
                                                          "name": {
                                                            "type": "JSXIdentifier",
                                                            "name": "onClick$",
                                                            "start": 254,
                                                            "end": 262
                                                          },
                                                          "value": {
                                                            "type": "JSXExpressionContainer",
                                                            "expression": {
                                                              "type": "ArrowFunctionExpression",
                                                              "expression": true,
                                                              "async": false,
                                                              "typeParameters": null,
                                                              "params": [],
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
                                                                    "start": 270,
                                                                    "end": 277
                                                                  },
                                                                  "property": {
                                                                    "type": "Identifier",
                                                                    "decorators": [],
                                                                    "name": "log",
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 278,
                                                                    "end": 281
                                                                  },
                                                                  "optional": false,
                                                                  "computed": false,
                                                                  "start": 270,
                                                                  "end": 281
                                                                },
                                                                "typeArguments": null,
                                                                "arguments": [
                                                                  {
                                                                    "type": "MemberExpression",
                                                                    "object": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "item",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 282,
                                                                        "end": 286
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 287,
                                                                        "end": 292
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 282,
                                                                      "end": 292
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "id",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 293,
                                                                      "end": 295
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 282,
                                                                    "end": 295
                                                                  }
                                                                ],
                                                                "optional": false,
                                                                "start": 270,
                                                                "end": 296
                                                              },
                                                              "id": null,
                                                              "generator": false,
                                                              "start": 264,
                                                              "end": 296
                                                            },
                                                            "start": 263,
                                                            "end": 297
                                                          },
                                                          "start": 254,
                                                          "end": 297
                                                        }
                                                      ],
                                                      "selfClosing": false,
                                                      "start": 251,
                                                      "end": 298
                                                    },
                                                    "children": [
                                                      {
                                                        "type": "JSXExpressionContainer",
                                                        "expression": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "MemberExpression",
                                                            "object": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "item",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 299,
                                                              "end": 303
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 304,
                                                              "end": 309
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 299,
                                                            "end": 309
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 310,
                                                            "end": 312
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 299,
                                                          "end": 312
                                                        },
                                                        "start": 298,
                                                        "end": 313
                                                      }
                                                    ],
                                                    "closingElement": {
                                                      "type": "JSXClosingElement",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "p",
                                                        "start": 315,
                                                        "end": 316
                                                      },
                                                      "start": 313,
                                                      "end": 317
                                                    },
                                                    "start": 251,
                                                    "end": 317
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 243,
                                                  "end": 317
                                                }
                                              ],
                                              "optional": false,
                                              "start": 228,
                                              "end": 318
                                            },
                                            "start": 227,
                                            "end": 319
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 321,
                                            "end": 324
                                          },
                                          "start": 319,
                                          "end": 325
                                        },
                                        "start": 222,
                                        "end": 325
                                      },
                                      "start": 215,
                                      "end": 325
                                    }
                                  ],
                                  "start": 181,
                                  "end": 329
                                },
                                "id": null,
                                "generator": false,
                                "start": 170,
                                "end": 329
                              }
                            ],
                            "optional": false,
                            "start": 159,
                            "end": 330
                          },
                          "definite": false,
                          "start": 151,
                          "end": 330
                        }
                      ],
                      "declare": false,
                      "start": 145,
                      "end": 330
                    },
                    {
                      "type": "ReturnStatement",
                      "argument": {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "Inner",
                            "start": 341,
                            "end": 346
                          },
                          "typeArguments": null,
                          "attributes": [
                            {
                              "type": "JSXAttribute",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "data",
                                "start": 347,
                                "end": 351
                              },
                              "value": {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "data",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 353,
                                  "end": 357
                                },
                                "start": 352,
                                "end": 358
                              },
                              "start": 347,
                              "end": 358
                            }
                          ],
                          "selfClosing": true,
                          "start": 340,
                          "end": 361
                        },
                        "children": [],
                        "closingElement": null,
                        "start": 340,
                        "end": 361
                      },
                      "start": 333,
                      "end": 361
                    }
                  ],
                  "start": 96,
                  "end": 363
                },
                "id": null,
                "generator": false,
                "start": 90,
                "end": 363
              }
            ],
            "optional": false,
            "start": 79,
            "end": 364
          },
          "definite": false,
          "start": 73,
          "end": 364
        }
      ],
      "declare": false,
      "start": 67,
      "end": 364
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 364
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
/*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"));
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
            "name": "componentQrl",
            "start": 9,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 9,
            "end": 21
          },
          "start": 9,
          "end": 21
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 29,
        "end": 45
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 46
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 56,
            "end": 59
          },
          "start": 56,
          "end": 59
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 67,
        "end": 83
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 84
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_HTDRsvUbLiE",
            "start": 91,
            "end": 104
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [],
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_Foo_component_HTDRsvUbLiE",
                "raw": "\"./test.tsx_Foo_component_HTDRsvUbLiE\"",
                "start": 118,
                "end": 156
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 157
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 157
          },
          "start": 91,
          "end": 157
        }
      ],
      "start": 85,
      "end": 158
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 173,
          "end": 185
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 200,
              "end": 203
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_HTDRsvUbLiE",
                "start": 204,
                "end": 217
              },
              {
                "type": "Literal",
                "value": "Foo_component_HTDRsvUbLiE",
                "raw": "\"Foo_component_HTDRsvUbLiE\"",
                "start": 219,
                "end": 246
              }
            ],
            "optional": false,
            "start": 200,
            "end": 247
          }
        ],
        "optional": false,
        "start": 173,
        "end": 248
      },
      "start": 173,
      "end": 249
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 249
}
```

</details>

### Module: test.tsx_Foo_component_HTDRsvUbLiE.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_AuJ9mTBx5YA = ()=>import("./test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA");
export const Foo_component_HTDRsvUbLiE = ()=>{
    const data = useSignal([]);
    const Inner = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_AuJ9mTBx5YA, "Foo_component_Inner_component_AuJ9mTBx5YA"));
    return /*#__PURE__*/ _jsxSorted(Inner, null, {
        data: data
    }, null, 3, "u6_2");
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
            "name": "_jsxSorted",
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 9,
            "end": 19
          },
          "start": 9,
          "end": 19
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 44
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 54,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 54,
            "end": 66
          },
          "start": 54,
          "end": 66
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 74,
        "end": 90
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 91
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 101,
            "end": 104
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 101,
            "end": 104
          },
          "start": 101,
          "end": 104
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 112,
        "end": 128
      },
      "phase": null,
      "attributes": [],
      "start": 92,
      "end": 129
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 139,
            "end": 148
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 139,
            "end": 148
          },
          "start": 139,
          "end": 148
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 156,
        "end": 172
      },
      "phase": null,
      "attributes": [],
      "start": 130,
      "end": 173
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_AuJ9mTBx5YA",
            "start": 180,
            "end": 193
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [],
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA",
                "raw": "\"./test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA\"",
                "start": 207,
                "end": 261
              },
              "options": null,
              "phase": null,
              "start": 200,
              "end": 262
            },
            "id": null,
            "generator": false,
            "start": 196,
            "end": 262
          },
          "start": 180,
          "end": 262
        }
      ],
      "start": 174,
      "end": 263
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
              "name": "Foo_component_HTDRsvUbLiE",
              "start": 277,
              "end": 302
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "data",
                          "start": 321,
                          "end": 325
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 328,
                            "end": 337
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 338,
                              "end": 340
                            }
                          ],
                          "optional": false,
                          "start": 328,
                          "end": 341
                        },
                        "start": 321,
                        "end": 341
                      }
                    ],
                    "start": 315,
                    "end": 342
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Inner",
                          "start": 353,
                          "end": 358
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "componentQrl",
                            "start": 375,
                            "end": 387
                          },
                          "arguments": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "qrl",
                                "start": 402,
                                "end": 405
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "i_AuJ9mTBx5YA",
                                  "start": 406,
                                  "end": 419
                                },
                                {
                                  "type": "Literal",
                                  "value": "Foo_component_Inner_component_AuJ9mTBx5YA",
                                  "raw": "\"Foo_component_Inner_component_AuJ9mTBx5YA\"",
                                  "start": 421,
                                  "end": 464
                                }
                              ],
                              "optional": false,
                              "start": 402,
                              "end": 465
                            }
                          ],
                          "optional": false,
                          "start": 375,
                          "end": 466
                        },
                        "start": 353,
                        "end": 466
                      }
                    ],
                    "start": 347,
                    "end": 467
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 493,
                        "end": 503
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Inner",
                          "start": 504,
                          "end": 509
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 511,
                          "end": 515
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "data",
                                "start": 527,
                                "end": 531
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "data",
                                "start": 533,
                                "end": 537
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 527,
                              "end": 537
                            }
                          ],
                          "start": 517,
                          "end": 543
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 545,
                          "end": 549
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 551,
                          "end": 552
                        },
                        {
                          "type": "Literal",
                          "value": "u6_2",
                          "raw": "\"u6_2\"",
                          "start": 554,
                          "end": 560
                        }
                      ],
                      "optional": false,
                      "start": 493,
                      "end": 561
                    },
                    "start": 472,
                    "end": 562
                  }
                ],
                "start": 309,
                "end": 564
              },
              "id": null,
              "generator": false,
              "start": 305,
              "end": 564
            },
            "start": 277,
            "end": 564
          }
        ],
        "start": 271,
        "end": 565
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 264,
      "end": 565
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 565
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    92,
    365
  ]
}
```

### Module: test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value.id;
const _hf0_str = "p0.value.id";
const i_7oFgEhSmgvY = ()=>import("./test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY");
export const Foo_component_Inner_component_AuJ9mTBx5YA = (props)=>{
    const data = props.data;
    const Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY = /*#__PURE__*/ qrl(i_7oFgEhSmgvY, "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY");
    return /*#__PURE__*/ _jsxSorted("div", null, null, data.value.map((item)=>/*#__PURE__*/ _jsxSorted("p", {
            "q-e:click": Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY,
            "q:p": item
        }, null, _fnSignal(_hf0, [
            item
        ], _hf0_str), 0, "u6_0")), 1, "u6_1");
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
            "name": "_fnSignal",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_fnSignal",
            "start": 9,
            "end": 18
          },
          "start": 9,
          "end": 18
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 26,
        "end": 42
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 43
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "start": 53,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 88
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "start": 98,
          "end": 101
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 109,
        "end": 125
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 126
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf0",
            "start": 133,
            "end": 137
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 141,
                "end": 143
              }
            ],
            "body": {
              "type": "MemberExpression",
              "object": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "name": "p0",
                  "start": 146,
                  "end": 148
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 149,
                  "end": 154
                },
                "optional": false,
                "computed": false,
                "start": 146,
                "end": 154
              },
              "property": {
                "type": "Identifier",
                "name": "id",
                "start": 155,
                "end": 157
              },
              "optional": false,
              "computed": false,
              "start": 146,
              "end": 157
            },
            "id": null,
            "generator": false,
            "start": 140,
            "end": 157
          },
          "start": 133,
          "end": 157
        }
      ],
      "start": 127,
      "end": 158
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf0_str",
            "start": 165,
            "end": 173
          },
          "init": {
            "type": "Literal",
            "value": "p0.value.id",
            "raw": "\"p0.value.id\"",
            "start": 176,
            "end": 189
          },
          "start": 165,
          "end": 189
        }
      ],
      "start": 159,
      "end": 190
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_7oFgEhSmgvY",
            "start": 197,
            "end": 210
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [],
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                "raw": "\"./test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY\"",
                "start": 224,
                "end": 294
              },
              "options": null,
              "phase": null,
              "start": 217,
              "end": 295
            },
            "id": null,
            "generator": false,
            "start": 213,
            "end": 295
          },
          "start": 197,
          "end": 295
        }
      ],
      "start": 191,
      "end": 296
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
              "name": "Foo_component_Inner_component_AuJ9mTBx5YA",
              "start": 310,
              "end": 351
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 355,
                  "end": 360
                }
              ],
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "data",
                          "start": 375,
                          "end": 379
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "props",
                            "start": 382,
                            "end": 387
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "data",
                            "start": 388,
                            "end": 392
                          },
                          "optional": false,
                          "computed": false,
                          "start": 382,
                          "end": 392
                        },
                        "start": 375,
                        "end": 392
                      }
                    ],
                    "start": 369,
                    "end": 393
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                          "start": 404,
                          "end": 461
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 478,
                            "end": 481
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_7oFgEhSmgvY",
                              "start": 482,
                              "end": 495
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                              "raw": "\"Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY\"",
                              "start": 497,
                              "end": 556
                            }
                          ],
                          "optional": false,
                          "start": 478,
                          "end": 557
                        },
                        "start": 404,
                        "end": 557
                      }
                    ],
                    "start": 398,
                    "end": 558
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 584,
                        "end": 594
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 595,
                          "end": 600
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 602,
                          "end": 606
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 608,
                          "end": 612
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "data",
                                "start": 614,
                                "end": 618
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 619,
                                "end": 624
                              },
                              "optional": false,
                              "computed": false,
                              "start": 614,
                              "end": 624
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 625,
                              "end": 628
                            },
                            "optional": false,
                            "computed": false,
                            "start": 614,
                            "end": 628
                          },
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "params": [
                                {
                                  "type": "Identifier",
                                  "name": "item",
                                  "start": 630,
                                  "end": 634
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 651,
                                  "end": 661
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "p",
                                    "raw": "\"p\"",
                                    "start": 662,
                                    "end": 665
                                  },
                                  {
                                    "type": "ObjectExpression",
                                    "properties": [
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q-e:click",
                                          "raw": "\"q-e:click\"",
                                          "start": 681,
                                          "end": 692
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                                          "start": 694,
                                          "end": 751
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 681,
                                        "end": 751
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q:p",
                                          "raw": "\"q:p\"",
                                          "start": 765,
                                          "end": 770
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "item",
                                          "start": 772,
                                          "end": 776
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 765,
                                        "end": 776
                                      }
                                    ],
                                    "start": 667,
                                    "end": 786
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 788,
                                    "end": 792
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_fnSignal",
                                      "start": 794,
                                      "end": 803
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "_hf0",
                                        "start": 804,
                                        "end": 808
                                      },
                                      {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Identifier",
                                            "name": "item",
                                            "start": 824,
                                            "end": 828
                                          }
                                        ],
                                        "start": 810,
                                        "end": 838
                                      },
                                      {
                                        "type": "Identifier",
                                        "name": "_hf0_str",
                                        "start": 840,
                                        "end": 848
                                      }
                                    ],
                                    "optional": false,
                                    "start": 794,
                                    "end": 849
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 851,
                                    "end": 852
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "u6_0",
                                    "raw": "\"u6_0\"",
                                    "start": 854,
                                    "end": 860
                                  }
                                ],
                                "optional": false,
                                "start": 651,
                                "end": 861
                              },
                              "id": null,
                              "generator": false,
                              "start": 629,
                              "end": 861
                            }
                          ],
                          "optional": false,
                          "start": 614,
                          "end": 862
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 864,
                          "end": 865
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 867,
                          "end": 873
                        }
                      ],
                      "optional": false,
                      "start": 584,
                      "end": 874
                    },
                    "start": 563,
                    "end": 875
                  }
                ],
                "start": 363,
                "end": 877
              },
              "id": null,
              "generator": false,
              "start": 354,
              "end": 877
            },
            "start": 310,
            "end": 877
          }
        ],
        "start": 304,
        "end": 878
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 297,
      "end": 878
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 878
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_Inner_component_AuJ9mTBx5YA",
  "entry": null,
  "displayName": "test.tsx_Foo_component_Inner_component",
  "hash": "AuJ9mTBx5YA",
  "canonicalFilename": "test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    172,
    331
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY.js (ENTRY POINT)

```javascript
export const Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY = (_, _1, item)=>console.log(item.value.id);
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
              "name": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
              "start": 13,
              "end": 70
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 74,
                  "end": 75
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 77,
                  "end": 79
                },
                {
                  "type": "Identifier",
                  "name": "item",
                  "start": 81,
                  "end": 85
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 88,
                    "end": 95
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 96,
                    "end": 99
                  },
                  "optional": false,
                  "computed": false,
                  "start": 88,
                  "end": 99
                },
                "arguments": [
                  {
                    "type": "MemberExpression",
                    "object": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "item",
                        "start": 100,
                        "end": 104
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 105,
                        "end": 110
                      },
                      "optional": false,
                      "computed": false,
                      "start": 100,
                      "end": 110
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "id",
                      "start": 111,
                      "end": 113
                    },
                    "optional": false,
                    "computed": false,
                    "start": 100,
                    "end": 113
                  }
                ],
                "optional": false,
                "start": 88,
                "end": 114
              },
              "id": null,
              "generator": false,
              "start": 73,
              "end": 114
            },
            "start": 13,
            "end": 114
          }
        ],
        "start": 7,
        "end": 115
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 115
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 115
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_Inner_component_div_p_q_e_click",
  "hash": "7oFgEhSmgvY",
  "canonicalFilename": "test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_Inner_component_AuJ9mTBx5YA",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    266,
    298
  ],
  "paramNames": [
    "_",
    "_",
    "item"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_fnSignal()`
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (3 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 8 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point module(s)
- **[CONV-14] Hoisted Functions**: 1 hoisted function(s) (`_hfN`/`_hfN_str` pairs) for signal-derived expressions

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `componentQrl` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` | `@qwik.dev/core` | 3 |
| `_fnSignal` | `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
