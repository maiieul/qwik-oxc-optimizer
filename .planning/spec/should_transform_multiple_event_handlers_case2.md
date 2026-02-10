# Test: should_transform_multiple_event_handlers_case2

## Test Configuration

**Note:** Case 2 of multiple event handlers: Same as case 1 but .map() callback has (row, idx) parameters. Tests q:ps (plural) for multiple parent context values.

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
  return <div>
	{data.value.map((row, idx) => (
	  <div onClick$={() => console.log(row.value.id, idx)} onMouseOver$={() => console.log('over' + row.value.id)}>
		<p onClick$={() => console.log('inner' + row.value.id)}>{item.value.id}</p>
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
                      "type": "ReturnStatement",
                      "argument": {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 155,
                            "end": 158
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 154,
                          "end": 159
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "\n\t",
                            "raw": "\n\t",
                            "start": 159,
                            "end": 161
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
                                    "start": 162,
                                    "end": 166
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "value",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 167,
                                    "end": 172
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 162,
                                  "end": 172
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "map",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 173,
                                  "end": 176
                                },
                                "optional": false,
                                "computed": false,
                                "start": 162,
                                "end": 176
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
                                      "start": 178,
                                      "end": 181
                                    },
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "idx",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 183,
                                      "end": 186
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
                                          "start": 197,
                                          "end": 200
                                        },
                                        "typeArguments": null,
                                        "attributes": [
                                          {
                                            "type": "JSXAttribute",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "onClick$",
                                              "start": 201,
                                              "end": 209
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
                                                      "start": 217,
                                                      "end": 224
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "log",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 225,
                                                      "end": 228
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 217,
                                                    "end": 228
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
                                                          "start": 229,
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
                                                        "start": 229,
                                                        "end": 238
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "id",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 239,
                                                        "end": 241
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 229,
                                                      "end": 241
                                                    },
                                                    {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "idx",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 243,
                                                      "end": 246
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 217,
                                                  "end": 247
                                                },
                                                "id": null,
                                                "generator": false,
                                                "start": 211,
                                                "end": 247
                                              },
                                              "start": 210,
                                              "end": 248
                                            },
                                            "start": 201,
                                            "end": 248
                                          },
                                          {
                                            "type": "JSXAttribute",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "onMouseOver$",
                                              "start": 249,
                                              "end": 261
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
                                                      "start": 269,
                                                      "end": 276
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "log",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 277,
                                                      "end": 280
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 269,
                                                    "end": 280
                                                  },
                                                  "typeArguments": null,
                                                  "arguments": [
                                                    {
                                                      "type": "BinaryExpression",
                                                      "left": {
                                                        "type": "Literal",
                                                        "value": "over",
                                                        "raw": "'over'",
                                                        "start": 281,
                                                        "end": 287
                                                      },
                                                      "operator": "+",
                                                      "right": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "row",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 290,
                                                            "end": 293
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "value",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 294,
                                                            "end": 299
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 290,
                                                          "end": 299
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "id",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 300,
                                                          "end": 302
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 290,
                                                        "end": 302
                                                      },
                                                      "start": 281,
                                                      "end": 302
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 269,
                                                  "end": 303
                                                },
                                                "id": null,
                                                "generator": false,
                                                "start": 263,
                                                "end": 303
                                              },
                                              "start": 262,
                                              "end": 304
                                            },
                                            "start": 249,
                                            "end": 304
                                          }
                                        ],
                                        "selfClosing": false,
                                        "start": 196,
                                        "end": 305
                                      },
                                      "children": [
                                        {
                                          "type": "JSXText",
                                          "value": "\n\t\t",
                                          "raw": "\n\t\t",
                                          "start": 305,
                                          "end": 308
                                        },
                                        {
                                          "type": "JSXElement",
                                          "openingElement": {
                                            "type": "JSXOpeningElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "p",
                                              "start": 309,
                                              "end": 310
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onClick$",
                                                  "start": 311,
                                                  "end": 319
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
                                                          "start": 327,
                                                          "end": 334
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "log",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 335,
                                                          "end": 338
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 327,
                                                        "end": 338
                                                      },
                                                      "typeArguments": null,
                                                      "arguments": [
                                                        {
                                                          "type": "BinaryExpression",
                                                          "left": {
                                                            "type": "Literal",
                                                            "value": "inner",
                                                            "raw": "'inner'",
                                                            "start": 339,
                                                            "end": 346
                                                          },
                                                          "operator": "+",
                                                          "right": {
                                                            "type": "MemberExpression",
                                                            "object": {
                                                              "type": "MemberExpression",
                                                              "object": {
                                                                "type": "Identifier",
                                                                "decorators": [],
                                                                "name": "row",
                                                                "optional": false,
                                                                "typeAnnotation": null,
                                                                "start": 349,
                                                                "end": 352
                                                              },
                                                              "property": {
                                                                "type": "Identifier",
                                                                "decorators": [],
                                                                "name": "value",
                                                                "optional": false,
                                                                "typeAnnotation": null,
                                                                "start": 353,
                                                                "end": 358
                                                              },
                                                              "optional": false,
                                                              "computed": false,
                                                              "start": 349,
                                                              "end": 358
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "id",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 359,
                                                              "end": 361
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 349,
                                                            "end": 361
                                                          },
                                                          "start": 339,
                                                          "end": 361
                                                        }
                                                      ],
                                                      "optional": false,
                                                      "start": 327,
                                                      "end": 362
                                                    },
                                                    "id": null,
                                                    "generator": false,
                                                    "start": 321,
                                                    "end": 362
                                                  },
                                                  "start": 320,
                                                  "end": 363
                                                },
                                                "start": 311,
                                                "end": 363
                                              }
                                            ],
                                            "selfClosing": false,
                                            "start": 308,
                                            "end": 364
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
                                                    "start": 365,
                                                    "end": 369
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "value",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 370,
                                                    "end": 375
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 365,
                                                  "end": 375
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "id",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 376,
                                                  "end": 378
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 365,
                                                "end": 378
                                              },
                                              "start": 364,
                                              "end": 379
                                            }
                                          ],
                                          "closingElement": {
                                            "type": "JSXClosingElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "p",
                                              "start": 381,
                                              "end": 382
                                            },
                                            "start": 379,
                                            "end": 383
                                          },
                                          "start": 308,
                                          "end": 383
                                        },
                                        {
                                          "type": "JSXText",
                                          "value": "\n\t  ",
                                          "raw": "\n\t  ",
                                          "start": 383,
                                          "end": 387
                                        }
                                      ],
                                      "closingElement": {
                                        "type": "JSXClosingElement",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "div",
                                          "start": 389,
                                          "end": 392
                                        },
                                        "start": 387,
                                        "end": 393
                                      },
                                      "start": 196,
                                      "end": 393
                                    },
                                    "start": 191,
                                    "end": 396
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 177,
                                  "end": 396
                                }
                              ],
                              "optional": false,
                              "start": 162,
                              "end": 397
                            },
                            "start": 161,
                            "end": 398
                          },
                          {
                            "type": "JSXText",
                            "value": "\n  ",
                            "raw": "\n  ",
                            "start": 398,
                            "end": 401
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 403,
                            "end": 406
                          },
                          "start": 401,
                          "end": 407
                        },
                        "start": 154,
                        "end": 407
                      },
                      "start": 147,
                      "end": 408
                    }
                  ],
                  "start": 98,
                  "end": 410
                },
                "expression": false,
                "start": 87,
                "end": 410
              }
            ],
            "optional": false,
            "start": 76,
            "end": 411
          },
          "definite": false,
          "start": 70,
          "end": 411
        }
      ],
      "declare": false,
      "start": 64,
      "end": 411
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 411
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
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_P9F6Osn3X6Y = ()=>import("./test.tsx_Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y");
const i_PjMbeUzoAMk = ()=>import("./test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk");
const i_vKrX4PmH2aM = ()=>import("./test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM");
export const Foo_component_HTDRsvUbLiE = function() {
    const data = useSignal([]);
    const Foo_component_div_div_q_e_click_vKrX4PmH2aM = /*#__PURE__*/ qrl(i_vKrX4PmH2aM, "Foo_component_div_div_q_e_click_vKrX4PmH2aM");
    const Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y = /*#__PURE__*/ qrl(i_P9F6Osn3X6Y, "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y");
    const Foo_component_div_div_p_q_e_click_PjMbeUzoAMk = /*#__PURE__*/ qrl(i_PjMbeUzoAMk, "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk");
    return /*#__PURE__*/ _jsxSorted("div", null, null, data.value.map((row, idx)=>/*#__PURE__*/ _jsxSorted("div", {
            "q-e:click": Foo_component_div_div_q_e_click_vKrX4PmH2aM,
            "q-e:mouseover": Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y,
            "q:ps": [
                row,
                idx
            ]
        }, null, /*#__PURE__*/ _jsxSorted("p", {
            "q-e:click": Foo_component_div_div_p_q_e_click_PjMbeUzoAMk,
            "q:p": row
        }, null, item.value.id, 0, null), 0, "u6_0")), 1, "u6_1");
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
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "start": 54,
          "end": 57
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 65,
        "end": 81
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 82
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 92,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 92,
            "end": 101
          },
          "start": 92,
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
      "start": 83,
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
            "name": "i_P9F6Osn3X6Y",
            "start": 133,
            "end": 146
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
                "value": "./test.tsx_Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
                "raw": "\"./test.tsx_Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y\"",
                "start": 160,
                "end": 220
              },
              "options": null,
              "phase": null,
              "start": 153,
              "end": 221
            },
            "id": null,
            "generator": false,
            "start": 149,
            "end": 221
          },
          "start": 133,
          "end": 221
        }
      ],
      "start": 127,
      "end": 222
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
            "start": 229,
            "end": 242
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
                "start": 256,
                "end": 314
              },
              "options": null,
              "phase": null,
              "start": 249,
              "end": 315
            },
            "id": null,
            "generator": false,
            "start": 245,
            "end": 315
          },
          "start": 229,
          "end": 315
        }
      ],
      "start": 223,
      "end": 316
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
            "start": 323,
            "end": 336
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
                "start": 350,
                "end": 406
              },
              "options": null,
              "phase": null,
              "start": 343,
              "end": 407
            },
            "id": null,
            "generator": false,
            "start": 339,
            "end": 407
          },
          "start": 323,
          "end": 407
        }
      ],
      "start": 317,
      "end": 408
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
              "start": 422,
              "end": 447
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
                          "start": 473,
                          "end": 477
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 480,
                            "end": 489
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 490,
                              "end": 492
                            }
                          ],
                          "optional": false,
                          "start": 480,
                          "end": 493
                        },
                        "start": 473,
                        "end": 493
                      }
                    ],
                    "start": 467,
                    "end": 494
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
                          "start": 505,
                          "end": 548
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 565,
                            "end": 568
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_vKrX4PmH2aM",
                              "start": 569,
                              "end": 582
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                              "raw": "\"Foo_component_div_div_q_e_click_vKrX4PmH2aM\"",
                              "start": 584,
                              "end": 629
                            }
                          ],
                          "optional": false,
                          "start": 565,
                          "end": 630
                        },
                        "start": 505,
                        "end": 630
                      }
                    ],
                    "start": 499,
                    "end": 631
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
                          "start": 642,
                          "end": 689
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 706,
                            "end": 709
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_P9F6Osn3X6Y",
                              "start": 710,
                              "end": 723
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
                              "raw": "\"Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y\"",
                              "start": 725,
                              "end": 774
                            }
                          ],
                          "optional": false,
                          "start": 706,
                          "end": 775
                        },
                        "start": 642,
                        "end": 775
                      }
                    ],
                    "start": 636,
                    "end": 776
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
                          "start": 787,
                          "end": 832
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 849,
                            "end": 852
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_PjMbeUzoAMk",
                              "start": 853,
                              "end": 866
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                              "raw": "\"Foo_component_div_div_p_q_e_click_PjMbeUzoAMk\"",
                              "start": 868,
                              "end": 915
                            }
                          ],
                          "optional": false,
                          "start": 849,
                          "end": 916
                        },
                        "start": 787,
                        "end": 916
                      }
                    ],
                    "start": 781,
                    "end": 917
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 943,
                        "end": 953
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 954,
                          "end": 959
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 961,
                          "end": 965
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 967,
                          "end": 971
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
                                "start": 973,
                                "end": 977
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 978,
                                "end": 983
                              },
                              "optional": false,
                              "computed": false,
                              "start": 973,
                              "end": 983
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 984,
                              "end": 987
                            },
                            "optional": false,
                            "computed": false,
                            "start": 973,
                            "end": 987
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
                                  "start": 989,
                                  "end": 992
                                },
                                {
                                  "type": "Identifier",
                                  "name": "idx",
                                  "start": 994,
                                  "end": 997
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 1014,
                                  "end": 1024
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "div",
                                    "raw": "\"div\"",
                                    "start": 1025,
                                    "end": 1030
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
                                          "start": 1046,
                                          "end": 1057
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "Foo_component_div_div_q_e_click_vKrX4PmH2aM",
                                          "start": 1059,
                                          "end": 1102
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1046,
                                        "end": 1102
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q-e:mouseover",
                                          "raw": "\"q-e:mouseover\"",
                                          "start": 1116,
                                          "end": 1131
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
                                          "start": 1133,
                                          "end": 1180
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1116,
                                        "end": 1180
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Literal",
                                          "value": "q:ps",
                                          "raw": "\"q:ps\"",
                                          "start": 1194,
                                          "end": 1200
                                        },
                                        "value": {
                                          "type": "ArrayExpression",
                                          "elements": [
                                            {
                                              "type": "Identifier",
                                              "name": "row",
                                              "start": 1220,
                                              "end": 1223
                                            },
                                            {
                                              "type": "Identifier",
                                              "name": "idx",
                                              "start": 1241,
                                              "end": 1244
                                            }
                                          ],
                                          "start": 1202,
                                          "end": 1258
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1194,
                                        "end": 1258
                                      }
                                    ],
                                    "start": 1032,
                                    "end": 1268
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 1270,
                                    "end": 1274
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_jsxSorted",
                                      "start": 1290,
                                      "end": 1300
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "p",
                                        "raw": "\"p\"",
                                        "start": 1301,
                                        "end": 1304
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
                                              "start": 1320,
                                              "end": 1331
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
                                              "start": 1333,
                                              "end": 1378
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1320,
                                            "end": 1378
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Literal",
                                              "value": "q:p",
                                              "raw": "\"q:p\"",
                                              "start": 1392,
                                              "end": 1397
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "row",
                                              "start": 1399,
                                              "end": 1402
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1392,
                                            "end": 1402
                                          }
                                        ],
                                        "start": 1306,
                                        "end": 1412
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1414,
                                        "end": 1418
                                      },
                                      {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "name": "item",
                                            "start": 1420,
                                            "end": 1424
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "value",
                                            "start": 1425,
                                            "end": 1430
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 1420,
                                          "end": 1430
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "id",
                                          "start": 1431,
                                          "end": 1433
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1420,
                                        "end": 1433
                                      },
                                      {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 1435,
                                        "end": 1436
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1438,
                                        "end": 1442
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1290,
                                    "end": 1443
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 1445,
                                    "end": 1446
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "u6_0",
                                    "raw": "\"u6_0\"",
                                    "start": 1448,
                                    "end": 1454
                                  }
                                ],
                                "optional": false,
                                "start": 1014,
                                "end": 1455
                              },
                              "id": null,
                              "generator": false,
                              "start": 988,
                              "end": 1455
                            }
                          ],
                          "optional": false,
                          "start": 973,
                          "end": 1456
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1458,
                          "end": 1459
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 1461,
                          "end": 1467
                        }
                      ],
                      "optional": false,
                      "start": 943,
                      "end": 1468
                    },
                    "start": 922,
                    "end": 1469
                  }
                ],
                "start": 461,
                "end": 1471
              },
              "expression": false,
              "start": 450,
              "end": 1471
            },
            "start": 422,
            "end": 1471
          }
        ],
        "start": 416,
        "end": 1472
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 409,
      "end": 1472
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1472
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
    412
  ]
}
```

### Module: test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.js (ENTRY POINT)

```javascript
export const Foo_component_div_div_q_e_click_vKrX4PmH2aM = (_, _1, row, idx)=>console.log(row.value.id, idx);
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
                },
                {
                  "type": "Identifier",
                  "name": "idx",
                  "start": 72,
                  "end": 75
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 78,
                    "end": 85
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 86,
                    "end": 89
                  },
                  "optional": false,
                  "computed": false,
                  "start": 78,
                  "end": 89
                },
                "arguments": [
                  {
                    "type": "MemberExpression",
                    "object": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "row",
                        "start": 90,
                        "end": 93
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 94,
                        "end": 99
                      },
                      "optional": false,
                      "computed": false,
                      "start": 90,
                      "end": 99
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "id",
                      "start": 100,
                      "end": 102
                    },
                    "optional": false,
                    "computed": false,
                    "start": 90,
                    "end": 102
                  },
                  {
                    "type": "Identifier",
                    "name": "idx",
                    "start": 104,
                    "end": 107
                  }
                ],
                "optional": false,
                "start": 78,
                "end": 108
              },
              "id": null,
              "generator": false,
              "start": 59,
              "end": 108
            },
            "start": 13,
            "end": 108
          }
        ],
        "start": 7,
        "end": 109
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 109
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 109
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
    213,
    249
  ],
  "paramNames": [
    "_",
    "_",
    "row",
    "idx"
  ]
}
```

### Module: test.tsx_Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y.js (ENTRY POINT)

```javascript
export const Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y = (_, _1, row)=>console.log('over' + row.value.id);
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
              "name": "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
              "start": 13,
              "end": 60
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 64,
                  "end": 65
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 67,
                  "end": 69
                },
                {
                  "type": "Identifier",
                  "name": "row",
                  "start": 71,
                  "end": 74
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 77,
                    "end": 84
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 85,
                    "end": 88
                  },
                  "optional": false,
                  "computed": false,
                  "start": 77,
                  "end": 88
                },
                "arguments": [
                  {
                    "type": "BinaryExpression",
                    "left": {
                      "type": "Literal",
                      "value": "over",
                      "raw": "'over'",
                      "start": 89,
                      "end": 95
                    },
                    "operator": "+",
                    "right": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "row",
                          "start": 98,
                          "end": 101
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 102,
                          "end": 107
                        },
                        "optional": false,
                        "computed": false,
                        "start": 98,
                        "end": 107
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "id",
                        "start": 108,
                        "end": 110
                      },
                      "optional": false,
                      "computed": false,
                      "start": 98,
                      "end": 110
                    },
                    "start": 89,
                    "end": 110
                  }
                ],
                "optional": false,
                "start": 77,
                "end": 111
              },
              "id": null,
              "generator": false,
              "start": 63,
              "end": 111
            },
            "start": 13,
            "end": 111
          }
        ],
        "start": 7,
        "end": 112
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 112
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 112
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_div_q_e_mouseover",
  "hash": "P9F6Osn3X6Y",
  "canonicalFilename": "test.tsx_Foo_component_div_div_q_e_mouseover_P9F6Osn3X6Y",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "eventHandler",
  "ctxName": "onMouseOver$",
  "captures": false,
  "loc": [
    265,
    305
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
export const Foo_component_div_div_p_q_e_click_PjMbeUzoAMk = (_, _1, row)=>console.log('inner' + row.value.id);
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
              "name": "Foo_component_div_div_p_q_e_click_PjMbeUzoAMk",
              "start": 13,
              "end": 58
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 62,
                  "end": 63
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 65,
                  "end": 67
                },
                {
                  "type": "Identifier",
                  "name": "row",
                  "start": 69,
                  "end": 72
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 75,
                    "end": 82
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 83,
                    "end": 86
                  },
                  "optional": false,
                  "computed": false,
                  "start": 75,
                  "end": 86
                },
                "arguments": [
                  {
                    "type": "BinaryExpression",
                    "left": {
                      "type": "Literal",
                      "value": "inner",
                      "raw": "'inner'",
                      "start": 87,
                      "end": 94
                    },
                    "operator": "+",
                    "right": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "row",
                          "start": 97,
                          "end": 100
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 101,
                          "end": 106
                        },
                        "optional": false,
                        "computed": false,
                        "start": 97,
                        "end": 106
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "id",
                        "start": 107,
                        "end": 109
                      },
                      "optional": false,
                      "computed": false,
                      "start": 97,
                      "end": 109
                    },
                    "start": 87,
                    "end": 109
                  }
                ],
                "optional": false,
                "start": 75,
                "end": 110
              },
              "id": null,
              "generator": false,
              "start": 61,
              "end": 110
            },
            "start": 13,
            "end": 110
          }
        ],
        "start": 7,
        "end": 111
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 111
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 111
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
  "captures": false,
  "loc": [
    323,
    364
  ],
  "paramNames": [
    "_",
    "_",
    "row"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 3 |
| _jsxSorted | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 3 |
| useSignal | test.tsx_Foo_component_HTDRsvUbLiE.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
