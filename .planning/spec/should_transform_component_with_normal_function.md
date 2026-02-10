# Test: should_transform_component_with_normal_function

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, useSignal, Signal } from '@qwik.dev/core';
const Foo = component$(function() {
  const data = useSignal<Signal<any>[]>([]);
  const Inner = component$(function(props) {
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
                "type": "FunctionExpression",
                "id": null,
                "generator": false,
                "async": false,
                "declare": false,
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
                            "start": 111,
                            "end": 115
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 118,
                              "end": 127
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
                                      "start": 128,
                                      "end": 134
                                    },
                                    "typeArguments": {
                                      "type": "TSTypeParameterInstantiation",
                                      "params": [
                                        {
                                          "type": "TSAnyKeyword",
                                          "start": 135,
                                          "end": 138
                                        }
                                      ],
                                      "start": 134,
                                      "end": 139
                                    },
                                    "start": 128,
                                    "end": 139
                                  },
                                  "start": 128,
                                  "end": 141
                                }
                              ],
                              "start": 127,
                              "end": 142
                            },
                            "arguments": [
                              {
                                "type": "ArrayExpression",
                                "elements": [],
                                "start": 143,
                                "end": 145
                              }
                            ],
                            "optional": false,
                            "start": 118,
                            "end": 146
                          },
                          "definite": false,
                          "start": 111,
                          "end": 146
                        }
                      ],
                      "declare": false,
                      "start": 105,
                      "end": 147
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
                            "start": 156,
                            "end": 161
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "component$",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 164,
                              "end": 174
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "FunctionExpression",
                                "id": null,
                                "generator": false,
                                "async": false,
                                "declare": false,
                                "typeParameters": null,
                                "params": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "props",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 184,
                                    "end": 189
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
                                            "start": 203,
                                            "end": 207
                                          },
                                          "init": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "props",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 210,
                                              "end": 215
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "data",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 216,
                                              "end": 220
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 210,
                                            "end": 220
                                          },
                                          "definite": false,
                                          "start": 203,
                                          "end": 220
                                        }
                                      ],
                                      "declare": false,
                                      "start": 197,
                                      "end": 220
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
                                            "start": 233,
                                            "end": 236
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 232,
                                          "end": 237
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
                                                    "start": 238,
                                                    "end": 242
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "value",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 243,
                                                    "end": 248
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 238,
                                                  "end": 248
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "map",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 249,
                                                  "end": 252
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 238,
                                                "end": 252
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
                                                      "start": 253,
                                                      "end": 257
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
                                                        "start": 262,
                                                        "end": 263
                                                      },
                                                      "typeArguments": null,
                                                      "attributes": [
                                                        {
                                                          "type": "JSXAttribute",
                                                          "name": {
                                                            "type": "JSXIdentifier",
                                                            "name": "onClick$",
                                                            "start": 264,
                                                            "end": 272
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
                                                                    "start": 280,
                                                                    "end": 287
                                                                  },
                                                                  "property": {
                                                                    "type": "Identifier",
                                                                    "decorators": [],
                                                                    "name": "log",
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 288,
                                                                    "end": 291
                                                                  },
                                                                  "optional": false,
                                                                  "computed": false,
                                                                  "start": 280,
                                                                  "end": 291
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
                                                                        "start": 292,
                                                                        "end": 296
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 297,
                                                                        "end": 302
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 292,
                                                                      "end": 302
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "id",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 303,
                                                                      "end": 305
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 292,
                                                                    "end": 305
                                                                  }
                                                                ],
                                                                "optional": false,
                                                                "start": 280,
                                                                "end": 306
                                                              },
                                                              "id": null,
                                                              "generator": false,
                                                              "start": 274,
                                                              "end": 306
                                                            },
                                                            "start": 273,
                                                            "end": 307
                                                          },
                                                          "start": 264,
                                                          "end": 307
                                                        }
                                                      ],
                                                      "selfClosing": false,
                                                      "start": 261,
                                                      "end": 308
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
                                                              "start": 309,
                                                              "end": 313
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 314,
                                                              "end": 319
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 309,
                                                            "end": 319
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 320,
                                                            "end": 322
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 309,
                                                          "end": 322
                                                        },
                                                        "start": 308,
                                                        "end": 323
                                                      }
                                                    ],
                                                    "closingElement": {
                                                      "type": "JSXClosingElement",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "p",
                                                        "start": 325,
                                                        "end": 326
                                                      },
                                                      "start": 323,
                                                      "end": 327
                                                    },
                                                    "start": 261,
                                                    "end": 327
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 253,
                                                  "end": 327
                                                }
                                              ],
                                              "optional": false,
                                              "start": 238,
                                              "end": 328
                                            },
                                            "start": 237,
                                            "end": 329
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 331,
                                            "end": 334
                                          },
                                          "start": 329,
                                          "end": 335
                                        },
                                        "start": 232,
                                        "end": 335
                                      },
                                      "start": 225,
                                      "end": 335
                                    }
                                  ],
                                  "start": 191,
                                  "end": 339
                                },
                                "expression": false,
                                "start": 175,
                                "end": 339
                              }
                            ],
                            "optional": false,
                            "start": 164,
                            "end": 340
                          },
                          "definite": false,
                          "start": 156,
                          "end": 340
                        }
                      ],
                      "declare": false,
                      "start": 150,
                      "end": 340
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
                            "start": 351,
                            "end": 356
                          },
                          "typeArguments": null,
                          "attributes": [
                            {
                              "type": "JSXAttribute",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "data",
                                "start": 357,
                                "end": 361
                              },
                              "value": {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "data",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 363,
                                  "end": 367
                                },
                                "start": 362,
                                "end": 368
                              },
                              "start": 357,
                              "end": 368
                            }
                          ],
                          "selfClosing": true,
                          "start": 350,
                          "end": 371
                        },
                        "children": [],
                        "closingElement": null,
                        "start": 350,
                        "end": 371
                      },
                      "start": 343,
                      "end": 371
                    }
                  ],
                  "start": 101,
                  "end": 373
                },
                "expression": false,
                "start": 90,
                "end": 373
              }
            ],
            "optional": false,
            "start": 79,
            "end": 374
          },
          "definite": false,
          "start": 73,
          "end": 374
        }
      ],
      "declare": false,
      "start": 67,
      "end": 374
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 374
}
```

</details>

## Output

### Module: `test.js`

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

### Module: `test.tsx_Foo_component_HTDRsvUbLiE.js` (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_AuJ9mTBx5YA = ()=>import("./test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA");
export const Foo_component_HTDRsvUbLiE = function() {
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
              "type": "FunctionExpression",
              "id": null,
              "generator": false,
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
                          "start": 328,
                          "end": 332
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 335,
                            "end": 344
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 345,
                              "end": 347
                            }
                          ],
                          "optional": false,
                          "start": 335,
                          "end": 348
                        },
                        "start": 328,
                        "end": 348
                      }
                    ],
                    "start": 322,
                    "end": 349
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
                          "start": 360,
                          "end": 365
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "componentQrl",
                            "start": 382,
                            "end": 394
                          },
                          "arguments": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "qrl",
                                "start": 409,
                                "end": 412
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "i_AuJ9mTBx5YA",
                                  "start": 413,
                                  "end": 426
                                },
                                {
                                  "type": "Literal",
                                  "value": "Foo_component_Inner_component_AuJ9mTBx5YA",
                                  "raw": "\"Foo_component_Inner_component_AuJ9mTBx5YA\"",
                                  "start": 428,
                                  "end": 471
                                }
                              ],
                              "optional": false,
                              "start": 409,
                              "end": 472
                            }
                          ],
                          "optional": false,
                          "start": 382,
                          "end": 473
                        },
                        "start": 360,
                        "end": 473
                      }
                    ],
                    "start": 354,
                    "end": 474
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 500,
                        "end": 510
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Inner",
                          "start": 511,
                          "end": 516
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 518,
                          "end": 522
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
                                "start": 534,
                                "end": 538
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "data",
                                "start": 540,
                                "end": 544
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 534,
                              "end": 544
                            }
                          ],
                          "start": 524,
                          "end": 550
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 552,
                          "end": 556
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 558,
                          "end": 559
                        },
                        {
                          "type": "Literal",
                          "value": "u6_2",
                          "raw": "\"u6_2\"",
                          "start": 561,
                          "end": 567
                        }
                      ],
                      "optional": false,
                      "start": 500,
                      "end": 568
                    },
                    "start": 479,
                    "end": 569
                  }
                ],
                "start": 316,
                "end": 571
              },
              "expression": false,
              "start": 305,
              "end": 571
            },
            "start": 277,
            "end": 571
          }
        ],
        "start": 271,
        "end": 572
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 264,
      "end": 572
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 572
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
    375
  ]
}
```

### Module: `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value.id;
const _hf0_str = "p0.value.id";
const i_7oFgEhSmgvY = ()=>import("./test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY");
export const Foo_component_Inner_component_AuJ9mTBx5YA = function(props) {
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
              "type": "FunctionExpression",
              "id": null,
              "generator": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 363,
                  "end": 368
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
                          "start": 382,
                          "end": 386
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "props",
                            "start": 389,
                            "end": 394
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "data",
                            "start": 395,
                            "end": 399
                          },
                          "optional": false,
                          "computed": false,
                          "start": 389,
                          "end": 399
                        },
                        "start": 382,
                        "end": 399
                      }
                    ],
                    "start": 376,
                    "end": 400
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
                          "start": 411,
                          "end": 468
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 485,
                            "end": 488
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_7oFgEhSmgvY",
                              "start": 489,
                              "end": 502
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                              "raw": "\"Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY\"",
                              "start": 504,
                              "end": 563
                            }
                          ],
                          "optional": false,
                          "start": 485,
                          "end": 564
                        },
                        "start": 411,
                        "end": 564
                      }
                    ],
                    "start": 405,
                    "end": 565
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 591,
                        "end": 601
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 602,
                          "end": 607
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 609,
                          "end": 613
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 615,
                          "end": 619
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
                                "start": 621,
                                "end": 625
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 626,
                                "end": 631
                              },
                              "optional": false,
                              "computed": false,
                              "start": 621,
                              "end": 631
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 632,
                              "end": 635
                            },
                            "optional": false,
                            "computed": false,
                            "start": 621,
                            "end": 635
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
                                  "start": 637,
                                  "end": 641
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 658,
                                  "end": 668
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "p",
                                    "raw": "\"p\"",
                                    "start": 669,
                                    "end": 672
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
                                          "start": 688,
                                          "end": 699
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY",
                                          "start": 701,
                                          "end": 758
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 688,
                                        "end": 758
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q:p",
                                          "raw": "\"q:p\"",
                                          "start": 772,
                                          "end": 777
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "item",
                                          "start": 779,
                                          "end": 783
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 772,
                                        "end": 783
                                      }
                                    ],
                                    "start": 674,
                                    "end": 793
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 795,
                                    "end": 799
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_fnSignal",
                                      "start": 801,
                                      "end": 810
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "_hf0",
                                        "start": 811,
                                        "end": 815
                                      },
                                      {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Identifier",
                                            "name": "item",
                                            "start": 831,
                                            "end": 835
                                          }
                                        ],
                                        "start": 817,
                                        "end": 845
                                      },
                                      {
                                        "type": "Identifier",
                                        "name": "_hf0_str",
                                        "start": 847,
                                        "end": 855
                                      }
                                    ],
                                    "optional": false,
                                    "start": 801,
                                    "end": 856
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 858,
                                    "end": 859
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "u6_0",
                                    "raw": "\"u6_0\"",
                                    "start": 861,
                                    "end": 867
                                  }
                                ],
                                "optional": false,
                                "start": 658,
                                "end": 868
                              },
                              "id": null,
                              "generator": false,
                              "start": 636,
                              "end": 868
                            }
                          ],
                          "optional": false,
                          "start": 621,
                          "end": 869
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 871,
                          "end": 872
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 874,
                          "end": 880
                        }
                      ],
                      "optional": false,
                      "start": 591,
                      "end": 881
                    },
                    "start": 570,
                    "end": 882
                  }
                ],
                "start": 370,
                "end": 884
              },
              "expression": false,
              "start": 354,
              "end": 884
            },
            "start": 310,
            "end": 884
          }
        ],
        "start": 304,
        "end": 885
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 297,
      "end": 885
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 885
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
    177,
    341
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: `test.tsx_Foo_component_Inner_component_div_p_q_e_click_7oFgEhSmgvY.js` (ENTRY POINT)

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
    276,
    308
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
