# Test: should_transform_nested_loops

## Test Configuration

**Note:** Nested .map() loops with event handlers at both levels. Tests _fnSignal for reactive text and capture patterns across nested loop scopes.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, useSignal, Signal } from '@qwik.dev/core';
const Foo = component$(function() {
  const data = useSignal<Signal<any>[]>([]);
  const data2 = useSignal<Signal<any>[]>([]);
  return <div>
	{data.value.map(row => (
	  <div onClick$={() => console.log(row.value.id)}>
		{data2.value.map(item => (
		  <p onClick$={() => console.log(row.value.id, item.value.id)}>{row.value.id}-{item.value.id}</p>
		))}
	  </div>
	))}
  </div>;
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
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 19
          },
          "importKind": "value",
          "start": 9,
          "end": 19
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 30
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 30
          },
          "importKind": "value",
          "start": 21,
          "end": 30
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "Signal",
            "optional": false,
            "typeAnnotation": null,
            "start": 32,
            "end": 38
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Signal",
            "optional": false,
            "typeAnnotation": null,
            "start": 32,
            "end": 38
          },
          "importKind": "value",
          "start": 32,
          "end": 38
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 46,
        "end": 62
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 63
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
            "start": 70,
            "end": 73
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 76,
              "end": 86
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
                            "start": 108,
                            "end": 112
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 115,
                              "end": 124
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
                                      "start": 125,
                                      "end": 131
                                    },
                                    "typeArguments": {
                                      "type": "TSTypeParameterInstantiation",
                                      "params": [
                                        {
                                          "type": "TSAnyKeyword",
                                          "start": 132,
                                          "end": 135
                                        }
                                      ],
                                      "start": 131,
                                      "end": 136
                                    },
                                    "start": 125,
                                    "end": 136
                                  },
                                  "start": 125,
                                  "end": 138
                                }
                              ],
                              "start": 124,
                              "end": 139
                            },
                            "arguments": [
                              {
                                "type": "ArrayExpression",
                                "elements": [],
                                "start": 140,
                                "end": 142
                              }
                            ],
                            "optional": false,
                            "start": 115,
                            "end": 143
                          },
                          "definite": false,
                          "start": 108,
                          "end": 143
                        }
                      ],
                      "declare": false,
                      "start": 102,
                      "end": 144
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
                            "name": "data2",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 153,
                            "end": 158
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 161,
                              "end": 170
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
                                      "start": 171,
                                      "end": 177
                                    },
                                    "typeArguments": {
                                      "type": "TSTypeParameterInstantiation",
                                      "params": [
                                        {
                                          "type": "TSAnyKeyword",
                                          "start": 178,
                                          "end": 181
                                        }
                                      ],
                                      "start": 177,
                                      "end": 182
                                    },
                                    "start": 171,
                                    "end": 182
                                  },
                                  "start": 171,
                                  "end": 184
                                }
                              ],
                              "start": 170,
                              "end": 185
                            },
                            "arguments": [
                              {
                                "type": "ArrayExpression",
                                "elements": [],
                                "start": 186,
                                "end": 188
                              }
                            ],
                            "optional": false,
                            "start": 161,
                            "end": 189
                          },
                          "definite": false,
                          "start": 153,
                          "end": 189
                        }
                      ],
                      "declare": false,
                      "start": 147,
                      "end": 190
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
                            "start": 201,
                            "end": 204
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 200,
                          "end": 205
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "\n\t",
                            "raw": "\n\t",
                            "start": 205,
                            "end": 207
                          },
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
                                    "start": 208,
                                    "end": 212
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "value",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 213,
                                    "end": 218
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 208,
                                  "end": 218
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "map",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 219,
                                  "end": 222
                                },
                                "optional": false,
                                "computed": false,
                                "start": 208,
                                "end": 222
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
                                      "name": "row",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 223,
                                      "end": 226
                                    }
                                  ],
                                  "returnType": null,
                                  "body": {
                                    "type": "ParenthesizedExpression",
                                    "expression": {
                                      "type": "JSXElement",
                                      "openingElement": {
                                        "type": "JSXOpeningElement",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "div",
                                          "start": 236,
                                          "end": 239
                                        },
                                        "typeArguments": null,
                                        "attributes": [
                                          {
                                            "type": "JSXAttribute",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "onClick$",
                                              "start": 240,
                                              "end": 248
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
                                                      "start": 256,
                                                      "end": 263
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "log",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 264,
                                                      "end": 267
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 256,
                                                    "end": 267
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
                                                          "name": "row",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 268,
                                                          "end": 271
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "value",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 272,
                                                          "end": 277
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 268,
                                                        "end": 277
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "id",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 278,
                                                        "end": 280
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 268,
                                                      "end": 280
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 256,
                                                  "end": 281
                                                },
                                                "id": null,
                                                "generator": false,
                                                "start": 250,
                                                "end": 281
                                              },
                                              "start": 249,
                                              "end": 282
                                            },
                                            "start": 240,
                                            "end": 282
                                          }
                                        ],
                                        "selfClosing": false,
                                        "start": 235,
                                        "end": 283
                                      },
                                      "children": [
                                        {
                                          "type": "JSXText",
                                          "value": "\n\t\t",
                                          "raw": "\n\t\t",
                                          "start": 283,
                                          "end": 286
                                        },
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
                                                  "name": "data2",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 287,
                                                  "end": 292
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "value",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 293,
                                                  "end": 298
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 287,
                                                "end": 298
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "map",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 299,
                                                "end": 302
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 287,
                                              "end": 302
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
                                                    "start": 303,
                                                    "end": 307
                                                  }
                                                ],
                                                "returnType": null,
                                                "body": {
                                                  "type": "ParenthesizedExpression",
                                                  "expression": {
                                                    "type": "JSXElement",
                                                    "openingElement": {
                                                      "type": "JSXOpeningElement",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "p",
                                                        "start": 318,
                                                        "end": 319
                                                      },
                                                      "typeArguments": null,
                                                      "attributes": [
                                                        {
                                                          "type": "JSXAttribute",
                                                          "name": {
                                                            "type": "JSXIdentifier",
                                                            "name": "onClick$",
                                                            "start": 320,
                                                            "end": 328
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
                                                                    "start": 336,
                                                                    "end": 343
                                                                  },
                                                                  "property": {
                                                                    "type": "Identifier",
                                                                    "decorators": [],
                                                                    "name": "log",
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 344,
                                                                    "end": 347
                                                                  },
                                                                  "optional": false,
                                                                  "computed": false,
                                                                  "start": 336,
                                                                  "end": 347
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
                                                                        "name": "row",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 348,
                                                                        "end": 351
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 352,
                                                                        "end": 357
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 348,
                                                                      "end": 357
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "id",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 358,
                                                                      "end": 360
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 348,
                                                                    "end": 360
                                                                  },
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
                                                                        "start": 362,
                                                                        "end": 366
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 367,
                                                                        "end": 372
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 362,
                                                                      "end": 372
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "id",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 373,
                                                                      "end": 375
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 362,
                                                                    "end": 375
                                                                  }
                                                                ],
                                                                "optional": false,
                                                                "start": 336,
                                                                "end": 376
                                                              },
                                                              "id": null,
                                                              "generator": false,
                                                              "start": 330,
                                                              "end": 376
                                                            },
                                                            "start": 329,
                                                            "end": 377
                                                          },
                                                          "start": 320,
                                                          "end": 377
                                                        }
                                                      ],
                                                      "selfClosing": false,
                                                      "start": 317,
                                                      "end": 378
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
                                                              "name": "row",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 379,
                                                              "end": 382
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 383,
                                                              "end": 388
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 379,
                                                            "end": 388
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 389,
                                                            "end": 391
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 379,
                                                          "end": 391
                                                        },
                                                        "start": 378,
                                                        "end": 392
                                                      },
                                                      {
                                                        "type": "JSXText",
                                                        "value": "-",
                                                        "raw": "-",
                                                        "start": 392,
                                                        "end": 393
                                                      },
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
                                                              "start": 394,
                                                              "end": 398
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 399,
                                                              "end": 404
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 394,
                                                            "end": 404
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 405,
                                                            "end": 407
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 394,
                                                          "end": 407
                                                        },
                                                        "start": 393,
                                                        "end": 408
                                                      }
                                                    ],
                                                    "closingElement": {
                                                      "type": "JSXClosingElement",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "p",
                                                        "start": 410,
                                                        "end": 411
                                                      },
                                                      "start": 408,
                                                      "end": 412
                                                    },
                                                    "start": 317,
                                                    "end": 412
                                                  },
                                                  "start": 311,
                                                  "end": 416
                                                },
                                                "id": null,
                                                "generator": false,
                                                "start": 303,
                                                "end": 416
                                              }
                                            ],
                                            "optional": false,
                                            "start": 287,
                                            "end": 417
                                          },
                                          "start": 286,
                                          "end": 418
                                        },
                                        {
                                          "type": "JSXText",
                                          "value": "\n\t  ",
                                          "raw": "\n\t  ",
                                          "start": 418,
                                          "end": 422
                                        }
                                      ],
                                      "closingElement": {
                                        "type": "JSXClosingElement",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "div",
                                          "start": 424,
                                          "end": 427
                                        },
                                        "start": 422,
                                        "end": 428
                                      },
                                      "start": 235,
                                      "end": 428
                                    },
                                    "start": 230,
                                    "end": 431
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 223,
                                  "end": 431
                                }
                              ],
                              "optional": false,
                              "start": 208,
                              "end": 432
                            },
                            "start": 207,
                            "end": 433
                          },
                          {
                            "type": "JSXText",
                            "value": "\n  ",
                            "raw": "\n  ",
                            "start": 433,
                            "end": 436
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 438,
                            "end": 441
                          },
                          "start": 436,
                          "end": 442
                        },
                        "start": 200,
                        "end": 442
                      },
                      "start": 193,
                      "end": 443
                    }
                  ],
                  "start": 98,
                  "end": 445
                },
                "expression": false,
                "start": 87,
                "end": 445
              }
            ],
            "optional": false,
            "start": 76,
            "end": 446
          },
          "definite": false,
          "start": 70,
          "end": 446
        }
      ],
      "declare": false,
      "start": 64,
      "end": 446
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 446
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
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value.id;
const _hf0_str = "p0.value.id";
const i_PjMbeUzoAMk = ()=>import("./test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk");
const i_vKrX4PmH2aM = ()=>import("./test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM");
export const Foo_component_HTDRsvUbLiE = function() {
    const data = useSignal([]);
    const data2 = useSignal([]);
    const Foo_component_div_div_q_e_click_vKrX4PmH2aM = /*#__PURE__*/ qrl(i_vKrX4PmH2aM, "Foo_component_div_div_q_e_click_vKrX4PmH2aM");
    const Foo_component_div_div_p_q_e_click_PjMbeUzoAMk = /*#__PURE__*/ qrl(i_PjMbeUzoAMk, "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk", [
        row
    ]);
    return /*#__PURE__*/ _jsxSorted("div", null, null, data.value.map((row)=>/*#__PURE__*/ _jsxSorted("div", {
            "q-e:click": Foo_component_div_div_q_e_click_vKrX4PmH2aM,
            "q:p": row
        }, null, data2.value.map((item)=>/*#__PURE__*/ _jsxSorted("p", {
                "q-e:click": Foo_component_div_div_p_q_e_click_PjMbeUzoAMk,
                "q:p": item
            }, null, [
                _fnSignal(_hf0, [
                    row
                ], _hf0_str),
                "-",
                _fnSignal(_hf0, [
                    item
                ], _hf0_str)
            ], 0, "u6_0")), 0, "u6_1")), 1, "u6_2");
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 136,
            "end": 145
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 136,
            "end": 145
          },
          "start": 136,
          "end": 145
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 153,
        "end": 169
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 170
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
            "start": 177,
            "end": 181
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 185,
                "end": 187
              }
            ],
            "body": {
              "type": "MemberExpression",
              "object": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "name": "p0",
                  "start": 190,
                  "end": 192
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 193,
                  "end": 198
                },
                "optional": false,
                "computed": false,
                "start": 190,
                "end": 198
              },
              "property": {
                "type": "Identifier",
                "name": "id",
                "start": 199,
                "end": 201
              },
              "optional": false,
              "computed": false,
              "start": 190,
              "end": 201
            },
            "id": null,
            "generator": false,
            "start": 184,
            "end": 201
          },
          "start": 177,
          "end": 201
        }
      ],
      "start": 171,
      "end": 202
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
            "start": 209,
            "end": 217
          },
          "init": {
            "type": "Literal",
            "value": "p0.value.id",
            "raw": "\"p0.value.id\"",
            "start": 220,
            "end": 233
          },
          "start": 209,
          "end": 233
        }
      ],
      "start": 203,
      "end": 234
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_PjMbeUzoAMk",
            "start": 241,
            "end": 254
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
                "value": "./test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                "raw": "\"./test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk\"",
                "start": 268,
                "end": 326
              },
              "options": null,
              "phase": null,
              "start": 261,
              "end": 327
            },
            "id": null,
            "generator": false,
            "start": 257,
            "end": 327
          },
          "start": 241,
          "end": 327
        }
      ],
      "start": 235,
      "end": 328
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_vKrX4PmH2aM",
            "start": 335,
            "end": 348
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
                "value": "./test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                "raw": "\"./test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM\"",
                "start": 362,
                "end": 418
              },
              "options": null,
              "phase": null,
              "start": 355,
              "end": 419
            },
            "id": null,
            "generator": false,
            "start": 351,
            "end": 419
          },
          "start": 335,
          "end": 419
        }
      ],
      "start": 329,
      "end": 420
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
              "start": 434,
              "end": 459
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
                          "start": 485,
                          "end": 489
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 492,
                            "end": 501
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 502,
                              "end": 504
                            }
                          ],
                          "optional": false,
                          "start": 492,
                          "end": 505
                        },
                        "start": 485,
                        "end": 505
                      }
                    ],
                    "start": 479,
                    "end": 506
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "data2",
                          "start": 517,
                          "end": 522
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 525,
                            "end": 534
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 535,
                              "end": 537
                            }
                          ],
                          "optional": false,
                          "start": 525,
                          "end": 538
                        },
                        "start": 517,
                        "end": 538
                      }
                    ],
                    "start": 511,
                    "end": 539
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                          "start": 550,
                          "end": 593
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 610,
                            "end": 613
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_vKrX4PmH2aM",
                              "start": 614,
                              "end": 627
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                              "raw": "\"Foo_component_div_div_q_e_click_vKrX4PmH2aM\"",
                              "start": 629,
                              "end": 674
                            }
                          ],
                          "optional": false,
                          "start": 610,
                          "end": 675
                        },
                        "start": 550,
                        "end": 675
                      }
                    ],
                    "start": 544,
                    "end": 676
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                          "start": 687,
                          "end": 732
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 749,
                            "end": 752
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_PjMbeUzoAMk",
                              "start": 753,
                              "end": 766
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                              "raw": "\"Foo_component_div_div_p_q_e_click_PjMbeUzoAMk\"",
                              "start": 768,
                              "end": 815
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "row",
                                  "start": 827,
                                  "end": 830
                                }
                              ],
                              "start": 817,
                              "end": 836
                            }
                          ],
                          "optional": false,
                          "start": 749,
                          "end": 837
                        },
                        "start": 687,
                        "end": 837
                      }
                    ],
                    "start": 681,
                    "end": 838
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 864,
                        "end": 874
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 875,
                          "end": 880
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 882,
                          "end": 886
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 888,
                          "end": 892
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
                                "start": 894,
                                "end": 898
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 899,
                                "end": 904
                              },
                              "optional": false,
                              "computed": false,
                              "start": 894,
                              "end": 904
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 905,
                              "end": 908
                            },
                            "optional": false,
                            "computed": false,
                            "start": 894,
                            "end": 908
                          },
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "params": [
                                {
                                  "type": "Identifier",
                                  "name": "row",
                                  "start": 910,
                                  "end": 913
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 930,
                                  "end": 940
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "div",
                                    "raw": "\"div\"",
                                    "start": 941,
                                    "end": 946
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
                                          "start": 962,
                                          "end": 973
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                                          "start": 975,
                                          "end": 1018
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 962,
                                        "end": 1018
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q:p",
                                          "raw": "\"q:p\"",
                                          "start": 1032,
                                          "end": 1037
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "row",
                                          "start": 1039,
                                          "end": 1042
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1032,
                                        "end": 1042
                                      }
                                    ],
                                    "start": 948,
                                    "end": 1052
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 1054,
                                    "end": 1058
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "data2",
                                          "start": 1060,
                                          "end": 1065
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "value",
                                          "start": 1066,
                                          "end": 1071
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1060,
                                        "end": 1071
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "map",
                                        "start": 1072,
                                        "end": 1075
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 1060,
                                      "end": 1075
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
                                            "start": 1077,
                                            "end": 1081
                                          }
                                        ],
                                        "body": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "name": "_jsxSorted",
                                            "start": 1098,
                                            "end": 1108
                                          },
                                          "arguments": [
                                            {
                                              "type": "Literal",
                                              "value": "p",
                                              "raw": "\"p\"",
                                              "start": 1109,
                                              "end": 1112
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
                                                    "start": 1132,
                                                    "end": 1143
                                                  },
                                                  "value": {
                                                    "type": "Identifier",
                                                    "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                                                    "start": 1145,
                                                    "end": 1190
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "start": 1132,
                                                  "end": 1190
                                                },
                                                {
                                                  "type": "Property",
                                                  "kind": "init",
                                                  "key": {
                                                    "type": "Literal",
                                                    "value": "q:p",
                                                    "raw": "\"q:p\"",
                                                    "start": 1208,
                                                    "end": 1213
                                                  },
                                                  "value": {
                                                    "type": "Identifier",
                                                    "name": "item",
                                                    "start": 1215,
                                                    "end": 1219
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "start": 1208,
                                                  "end": 1219
                                                }
                                              ],
                                              "start": 1114,
                                              "end": 1233
                                            },
                                            {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 1235,
                                              "end": 1239
                                            },
                                            {
                                              "type": "ArrayExpression",
                                              "elements": [
                                                {
                                                  "type": "CallExpression",
                                                  "callee": {
                                                    "type": "Identifier",
                                                    "name": "_fnSignal",
                                                    "start": 1259,
                                                    "end": 1268
                                                  },
                                                  "arguments": [
                                                    {
                                                      "type": "Identifier",
                                                      "name": "_hf0",
                                                      "start": 1269,
                                                      "end": 1273
                                                    },
                                                    {
                                                      "type": "ArrayExpression",
                                                      "elements": [
                                                        {
                                                          "type": "Identifier",
                                                          "name": "row",
                                                          "start": 1297,
                                                          "end": 1300
                                                        }
                                                      ],
                                                      "start": 1275,
                                                      "end": 1318
                                                    },
                                                    {
                                                      "type": "Identifier",
                                                      "name": "_hf0_str",
                                                      "start": 1320,
                                                      "end": 1328
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 1259,
                                                  "end": 1329
                                                },
                                                {
                                                  "type": "Literal",
                                                  "value": "-",
                                                  "raw": "\"-\"",
                                                  "start": 1347,
                                                  "end": 1350
                                                },
                                                {
                                                  "type": "CallExpression",
                                                  "callee": {
                                                    "type": "Identifier",
                                                    "name": "_fnSignal",
                                                    "start": 1368,
                                                    "end": 1377
                                                  },
                                                  "arguments": [
                                                    {
                                                      "type": "Identifier",
                                                      "name": "_hf0",
                                                      "start": 1378,
                                                      "end": 1382
                                                    },
                                                    {
                                                      "type": "ArrayExpression",
                                                      "elements": [
                                                        {
                                                          "type": "Identifier",
                                                          "name": "item",
                                                          "start": 1406,
                                                          "end": 1410
                                                        }
                                                      ],
                                                      "start": 1384,
                                                      "end": 1428
                                                    },
                                                    {
                                                      "type": "Identifier",
                                                      "name": "_hf0_str",
                                                      "start": 1430,
                                                      "end": 1438
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 1368,
                                                  "end": 1439
                                                }
                                              ],
                                              "start": 1241,
                                              "end": 1453
                                            },
                                            {
                                              "type": "Literal",
                                              "value": 0,
                                              "raw": "0",
                                              "start": 1455,
                                              "end": 1456
                                            },
                                            {
                                              "type": "Literal",
                                              "value": "u6_0",
                                              "raw": "\"u6_0\"",
                                              "start": 1458,
                                              "end": 1464
                                            }
                                          ],
                                          "optional": false,
                                          "start": 1098,
                                          "end": 1465
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 1076,
                                        "end": 1465
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1060,
                                    "end": 1466
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 1468,
                                    "end": 1469
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "u6_1",
                                    "raw": "\"u6_1\"",
                                    "start": 1471,
                                    "end": 1477
                                  }
                                ],
                                "optional": false,
                                "start": 930,
                                "end": 1478
                              },
                              "id": null,
                              "generator": false,
                              "start": 909,
                              "end": 1478
                            }
                          ],
                          "optional": false,
                          "start": 894,
                          "end": 1479
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1481,
                          "end": 1482
                        },
                        {
                          "type": "Literal",
                          "value": "u6_2",
                          "raw": "\"u6_2\"",
                          "start": 1484,
                          "end": 1490
                        }
                      ],
                      "optional": false,
                      "start": 864,
                      "end": 1491
                    },
                    "start": 843,
                    "end": 1492
                  }
                ],
                "start": 473,
                "end": 1494
              },
              "expression": false,
              "start": 462,
              "end": 1494
            },
            "start": 434,
            "end": 1494
          }
        ],
        "start": 428,
        "end": 1495
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 421,
      "end": 1495
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1495
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
    89,
    447
  ]
}
```

### Module: test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.js (ENTRY POINT)

```javascript
export const Foo_component_div_div_q_e_click_vKrX4PmH2aM = (_, _1, row)=>console.log(row.value.id);
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
              "name": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
              "start": 13,
              "end": 56
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 60,
                  "end": 61
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 63,
                  "end": 65
                },
                {
                  "type": "Identifier",
                  "name": "row",
                  "start": 67,
                  "end": 70
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 73,
                    "end": 80
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 81,
                    "end": 84
                  },
                  "optional": false,
                  "computed": false,
                  "start": 73,
                  "end": 84
                },
                "arguments": [
                  {
                    "type": "MemberExpression",
                    "object": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "row",
                        "start": 85,
                        "end": 88
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 89,
                        "end": 94
                      },
                      "optional": false,
                      "computed": false,
                      "start": 85,
                      "end": 94
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "id",
                      "start": 95,
                      "end": 97
                    },
                    "optional": false,
                    "computed": false,
                    "start": 85,
                    "end": 97
                  }
                ],
                "optional": false,
                "start": 73,
                "end": 98
              },
              "id": null,
              "generator": false,
              "start": 59,
              "end": 98
            },
            "start": 13,
            "end": 98
          }
        ],
        "start": 7,
        "end": 99
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 99
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 99
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_div_q_e_click",
  "hash": "vKrX4PmH2aM",
  "canonicalFilename": "test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    252,
    283
  ],
  "paramNames": [
    "_",
    "_",
    "row"
  ]
}
```

### Module: test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Foo_component_div_div_p_q_e_click_PjMbeUzoAMk = (_, _1, item)=>{
    const row = _captures[0];
    return console.log(row.value.id, item.value.id);
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
            "name": "_captures",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_captures",
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
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
              "start": 57,
              "end": 102
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 106,
                  "end": 107
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 109,
                  "end": 111
                },
                {
                  "type": "Identifier",
                  "name": "item",
                  "start": 113,
                  "end": 117
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
                          "name": "row",
                          "start": 132,
                          "end": 135
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 138,
                            "end": 147
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 148,
                            "end": 149
                          },
                          "optional": false,
                          "computed": true,
                          "start": 138,
                          "end": 150
                        },
                        "start": 132,
                        "end": 150
                      }
                    ],
                    "start": 126,
                    "end": 151
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "console",
                          "start": 163,
                          "end": 170
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 171,
                          "end": 174
                        },
                        "optional": false,
                        "computed": false,
                        "start": 163,
                        "end": 174
                      },
                      "arguments": [
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "row",
                              "start": 175,
                              "end": 178
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "value",
                              "start": 179,
                              "end": 184
                            },
                            "optional": false,
                            "computed": false,
                            "start": 175,
                            "end": 184
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "id",
                            "start": 185,
                            "end": 187
                          },
                          "optional": false,
                          "computed": false,
                          "start": 175,
                          "end": 187
                        },
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "item",
                              "start": 189,
                              "end": 193
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "value",
                              "start": 194,
                              "end": 199
                            },
                            "optional": false,
                            "computed": false,
                            "start": 189,
                            "end": 199
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "id",
                            "start": 200,
                            "end": 202
                          },
                          "optional": false,
                          "computed": false,
                          "start": 189,
                          "end": 202
                        }
                      ],
                      "optional": false,
                      "start": 163,
                      "end": 203
                    },
                    "start": 156,
                    "end": 204
                  }
                ],
                "start": 120,
                "end": 206
              },
              "id": null,
              "generator": false,
              "start": 105,
              "end": 206
            },
            "start": 57,
            "end": 206
          }
        ],
        "start": 51,
        "end": 207
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 207
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 207
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_div_p_q_e_click",
  "hash": "PjMbeUzoAMk",
  "canonicalFilename": "test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    332,
    378
  ],
  "paramNames": [
    "_",
    "_",
    "item"
  ],
  "captureNames": [
    "row"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-04] Signal Helpers**
- **[CONV-05] Capture Patterns**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**
- **[CONV-14] Hoisted Functions**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 2 |
| _jsxSorted | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 3 |
| _fnSignal | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 2 |
| useSignal | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 2 |

## Diagnostics

None (`[]`)
