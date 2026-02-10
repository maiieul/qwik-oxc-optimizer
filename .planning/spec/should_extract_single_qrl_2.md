# Test: should_extract_single_qrl_2

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { component$, useStore, useSignal } from '@qwik.dev/core';
      const Parent = component$(() => {
      const cart = useStore<Cart>([]);
      const results = useSignal(['foo', 'bar']);

      return (
        <div>
          <button id="first" onClick$={() => (results.value = ['item1', 'item2'])}></button>

          {results.value.map((item, key) => (
            <button
              id={'second-' + key}
              onClick$={() => {
                cart.push(item);
              }}
            >
              {item}
            </button>
          ))}
          <ul>
            {cart.map((item) => (
              <li>
                <span>{item}</span>
              </li>
            ))}
          </ul>
        </div>
      );
    });
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
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 29
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 29
          },
          "importKind": "value",
          "start": 21,
          "end": 29
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 40
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 40
          },
          "importKind": "value",
          "start": 31,
          "end": 40
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 48,
        "end": 64
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 65
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
            "name": "Parent",
            "optional": false,
            "typeAnnotation": null,
            "start": 78,
            "end": 84
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 87,
              "end": 97
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
                            "name": "cart",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 118,
                            "end": 122
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useStore",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 125,
                              "end": 133
                            },
                            "typeArguments": {
                              "type": "TSTypeParameterInstantiation",
                              "params": [
                                {
                                  "type": "TSTypeReference",
                                  "typeName": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "Cart",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 134,
                                    "end": 138
                                  },
                                  "typeArguments": null,
                                  "start": 134,
                                  "end": 138
                                }
                              ],
                              "start": 133,
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
                            "start": 125,
                            "end": 143
                          },
                          "definite": false,
                          "start": 118,
                          "end": 143
                        }
                      ],
                      "declare": false,
                      "start": 112,
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
                            "name": "results",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 157,
                            "end": 164
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 167,
                              "end": 176
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "foo",
                                    "raw": "'foo'",
                                    "start": 178,
                                    "end": 183
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "bar",
                                    "raw": "'bar'",
                                    "start": 185,
                                    "end": 190
                                  }
                                ],
                                "start": 177,
                                "end": 191
                              }
                            ],
                            "optional": false,
                            "start": 167,
                            "end": 192
                          },
                          "definite": false,
                          "start": 157,
                          "end": 192
                        }
                      ],
                      "declare": false,
                      "start": 151,
                      "end": 193
                    },
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
                              "start": 219,
                              "end": 222
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 218,
                            "end": 223
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "\n          ",
                              "raw": "\n          ",
                              "start": 223,
                              "end": 234
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 235,
                                  "end": 241
                                },
                                "typeArguments": null,
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "id",
                                      "start": 242,
                                      "end": 244
                                    },
                                    "value": {
                                      "type": "Literal",
                                      "value": "first",
                                      "raw": "\"first\"",
                                      "start": 245,
                                      "end": 252
                                    },
                                    "start": 242,
                                    "end": 252
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "onClick$",
                                      "start": 253,
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
                                          "type": "ParenthesizedExpression",
                                          "expression": {
                                            "type": "AssignmentExpression",
                                            "operator": "=",
                                            "left": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "results",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 270,
                                                "end": 277
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "value",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 278,
                                                "end": 283
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 270,
                                              "end": 283
                                            },
                                            "right": {
                                              "type": "ArrayExpression",
                                              "elements": [
                                                {
                                                  "type": "Literal",
                                                  "value": "item1",
                                                  "raw": "'item1'",
                                                  "start": 287,
                                                  "end": 294
                                                },
                                                {
                                                  "type": "Literal",
                                                  "value": "item2",
                                                  "raw": "'item2'",
                                                  "start": 296,
                                                  "end": 303
                                                }
                                              ],
                                              "start": 286,
                                              "end": 304
                                            },
                                            "start": 270,
                                            "end": 304
                                          },
                                          "start": 269,
                                          "end": 305
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 263,
                                        "end": 305
                                      },
                                      "start": 262,
                                      "end": 306
                                    },
                                    "start": 253,
                                    "end": 306
                                  }
                                ],
                                "selfClosing": false,
                                "start": 234,
                                "end": 307
                              },
                              "children": [],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 309,
                                  "end": 315
                                },
                                "start": 307,
                                "end": 316
                              },
                              "start": 234,
                              "end": 316
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\n          ",
                              "raw": "\n\n          ",
                              "start": 316,
                              "end": 328
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
                                      "name": "results",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 329,
                                      "end": 336
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "value",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 337,
                                      "end": 342
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 329,
                                    "end": 342
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "map",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 343,
                                    "end": 346
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 329,
                                  "end": 346
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
                                        "start": 348,
                                        "end": 352
                                      },
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "key",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 354,
                                        "end": 357
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
                                            "name": "button",
                                            "start": 377,
                                            "end": 383
                                          },
                                          "typeArguments": null,
                                          "attributes": [
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "id",
                                                "start": 398,
                                                "end": 400
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "BinaryExpression",
                                                  "left": {
                                                    "type": "Literal",
                                                    "value": "second-",
                                                    "raw": "'second-'",
                                                    "start": 402,
                                                    "end": 411
                                                  },
                                                  "operator": "+",
                                                  "right": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "key",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 414,
                                                    "end": 417
                                                  },
                                                  "start": 402,
                                                  "end": 417
                                                },
                                                "start": 401,
                                                "end": 418
                                              },
                                              "start": 398,
                                              "end": 418
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onClick$",
                                                "start": 433,
                                                "end": 441
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
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
                                                              "name": "cart",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 467,
                                                              "end": 471
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "push",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 472,
                                                              "end": 476
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 467,
                                                            "end": 476
                                                          },
                                                          "typeArguments": null,
                                                          "arguments": [
                                                            {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "item",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 477,
                                                              "end": 481
                                                            }
                                                          ],
                                                          "optional": false,
                                                          "start": 467,
                                                          "end": 482
                                                        },
                                                        "directive": null,
                                                        "start": 467,
                                                        "end": 483
                                                      }
                                                    ],
                                                    "start": 449,
                                                    "end": 499
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 443,
                                                  "end": 499
                                                },
                                                "start": 442,
                                                "end": 500
                                              },
                                              "start": 433,
                                              "end": 500
                                            }
                                          ],
                                          "selfClosing": false,
                                          "start": 376,
                                          "end": 514
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "\n              ",
                                            "raw": "\n              ",
                                            "start": 514,
                                            "end": 529
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "item",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 530,
                                              "end": 534
                                            },
                                            "start": 529,
                                            "end": 535
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n            ",
                                            "raw": "\n            ",
                                            "start": 535,
                                            "end": 548
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "button",
                                            "start": 550,
                                            "end": 556
                                          },
                                          "start": 548,
                                          "end": 557
                                        },
                                        "start": 376,
                                        "end": 557
                                      },
                                      "start": 362,
                                      "end": 569
                                    },
                                    "id": null,
                                    "generator": false,
                                    "start": 347,
                                    "end": 569
                                  }
                                ],
                                "optional": false,
                                "start": 329,
                                "end": 570
                              },
                              "start": 328,
                              "end": 571
                            },
                            {
                              "type": "JSXText",
                              "value": "\n          ",
                              "raw": "\n          ",
                              "start": 571,
                              "end": 582
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "ul",
                                  "start": 583,
                                  "end": 585
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 582,
                                "end": 586
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n            ",
                                  "raw": "\n            ",
                                  "start": 586,
                                  "end": 599
                                },
                                {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "cart",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 600,
                                        "end": 604
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "map",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 605,
                                        "end": 608
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 600,
                                      "end": 608
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
                                            "start": 610,
                                            "end": 614
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
                                                "name": "li",
                                                "start": 636,
                                                "end": 638
                                              },
                                              "typeArguments": null,
                                              "attributes": [],
                                              "selfClosing": false,
                                              "start": 635,
                                              "end": 639
                                            },
                                            "children": [
                                              {
                                                "type": "JSXText",
                                                "value": "\n                ",
                                                "raw": "\n                ",
                                                "start": 639,
                                                "end": 656
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "span",
                                                    "start": 657,
                                                    "end": 661
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [],
                                                  "selfClosing": false,
                                                  "start": 656,
                                                  "end": 662
                                                },
                                                "children": [
                                                  {
                                                    "type": "JSXExpressionContainer",
                                                    "expression": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "item",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 663,
                                                      "end": 667
                                                    },
                                                    "start": 662,
                                                    "end": 668
                                                  }
                                                ],
                                                "closingElement": {
                                                  "type": "JSXClosingElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "span",
                                                    "start": 670,
                                                    "end": 674
                                                  },
                                                  "start": 668,
                                                  "end": 675
                                                },
                                                "start": 656,
                                                "end": 675
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n              ",
                                                "raw": "\n              ",
                                                "start": 675,
                                                "end": 690
                                              }
                                            ],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "li",
                                                "start": 692,
                                                "end": 694
                                              },
                                              "start": 690,
                                              "end": 695
                                            },
                                            "start": 635,
                                            "end": 695
                                          },
                                          "start": 619,
                                          "end": 709
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 609,
                                        "end": 709
                                      }
                                    ],
                                    "optional": false,
                                    "start": 600,
                                    "end": 710
                                  },
                                  "start": 599,
                                  "end": 711
                                },
                                {
                                  "type": "JSXText",
                                  "value": "\n          ",
                                  "raw": "\n          ",
                                  "start": 711,
                                  "end": 722
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "ul",
                                  "start": 724,
                                  "end": 726
                                },
                                "start": 722,
                                "end": 727
                              },
                              "start": 582,
                              "end": 727
                            },
                            {
                              "type": "JSXText",
                              "value": "\n        ",
                              "raw": "\n        ",
                              "start": 727,
                              "end": 736
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 738,
                              "end": 741
                            },
                            "start": 736,
                            "end": 742
                          },
                          "start": 218,
                          "end": 742
                        },
                        "start": 208,
                        "end": 750
                      },
                      "start": 201,
                      "end": 751
                    }
                  ],
                  "start": 104,
                  "end": 757
                },
                "id": null,
                "generator": false,
                "start": 98,
                "end": 757
              }
            ],
            "optional": false,
            "start": 87,
            "end": 758
          },
          "definite": false,
          "start": 78,
          "end": 758
        }
      ],
      "declare": false,
      "start": 72,
      "end": 759
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 759
}
```

</details>

## Output

### Module: `test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Parent_component_div_button_q_e_click_5khsVRINUws = ()=>{
    const results = _captures[0];
    return results.value = [
        'item1',
        'item2'
    ];
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
              "name": "Parent_component_div_button_q_e_click_5khsVRINUws",
              "start": 57,
              "end": 106
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
                          "name": "results",
                          "start": 125,
                          "end": 132
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 135,
                            "end": 144
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 145,
                            "end": 146
                          },
                          "optional": false,
                          "computed": true,
                          "start": 135,
                          "end": 147
                        },
                        "start": 125,
                        "end": 147
                      }
                    ],
                    "start": 119,
                    "end": 148
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "results",
                          "start": 160,
                          "end": 167
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 168,
                          "end": 173
                        },
                        "optional": false,
                        "computed": false,
                        "start": 160,
                        "end": 173
                      },
                      "right": {
                        "type": "ArrayExpression",
                        "elements": [
                          {
                            "type": "Literal",
                            "value": "item1",
                            "raw": "'item1'",
                            "start": 186,
                            "end": 193
                          },
                          {
                            "type": "Literal",
                            "value": "item2",
                            "raw": "'item2'",
                            "start": 203,
                            "end": 210
                          }
                        ],
                        "start": 176,
                        "end": 216
                      },
                      "start": 160,
                      "end": 216
                    },
                    "start": 153,
                    "end": 217
                  }
                ],
                "start": 113,
                "end": 219
              },
              "id": null,
              "generator": false,
              "start": 109,
              "end": 219
            },
            "start": 57,
            "end": 219
          }
        ],
        "start": 51,
        "end": 220
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 220
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 220
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_div_button_q_e_click_5khsVRINUws",
  "entry": null,
  "displayName": "test.tsx_Parent_component_div_button_q_e_click",
  "hash": "5khsVRINUws",
  "canonicalFilename": "test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws",
  "path": "",
  "extension": "js",
  "parent": "Parent_component_0TaiDayHrlo",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    268,
    310
  ],
  "captureNames": [
    "results"
  ]
}
```

### Module: `test.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_0TaiDayHrlo = ()=>import("./test.tsx_Parent_component_0TaiDayHrlo");
/*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0TaiDayHrlo, "Parent_component_0TaiDayHrlo"));
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
            "name": "i_0TaiDayHrlo",
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
                "value": "./test.tsx_Parent_component_0TaiDayHrlo",
                "raw": "\"./test.tsx_Parent_component_0TaiDayHrlo\"",
                "start": 118,
                "end": 159
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 160
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 160
          },
          "start": 91,
          "end": 160
        }
      ],
      "start": 85,
      "end": 161
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 176,
          "end": 188
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 203,
              "end": 206
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_0TaiDayHrlo",
                "start": 207,
                "end": 220
              },
              {
                "type": "Literal",
                "value": "Parent_component_0TaiDayHrlo",
                "raw": "\"Parent_component_0TaiDayHrlo\"",
                "start": 222,
                "end": 252
              }
            ],
            "optional": false,
            "start": 203,
            "end": 253
          }
        ],
        "optional": false,
        "start": 176,
        "end": 254
      },
      "start": 176,
      "end": 255
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 255
}
```

</details>

### Module: `test.tsx_Parent_component_0TaiDayHrlo.js` (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const i_5khsVRINUws = ()=>import("./test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws");
const i_rAeuW6OvuXM = ()=>import("./test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM");
export const Parent_component_0TaiDayHrlo = ()=>{
    const cart = useStore([]);
    const results = useSignal([
        'foo',
        'bar'
    ]);
    const Parent_component_div_button_q_e_click_1_rAeuW6OvuXM = /*#__PURE__*/ qrl(i_rAeuW6OvuXM, "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM", [
        cart
    ]);
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("button", null, {
            id: "first",
            "q-e:click": /*#__PURE__*/ qrl(i_5khsVRINUws, "Parent_component_div_button_q_e_click_5khsVRINUws", [
                results
            ])
        }, null, 3, null),
        results.value.map((item, key)=>/*#__PURE__*/ _jsxSorted("button", {
                id: 'second-' + key,
                "q-e:click": Parent_component_div_button_q_e_click_1_rAeuW6OvuXM,
                "q:p": item
            }, null, item, 0, "u6_0")),
        /*#__PURE__*/ _jsxSorted("ul", null, null, cart.map((item)=>/*#__PURE__*/ _jsxSorted("li", null, null, /*#__PURE__*/ _jsxSorted("span", null, null, item, 1, null), 1, "u6_1")), 1, null)
    ], 1, "u6_2");
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "start": 136,
          "end": 144
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 152,
        "end": 168
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 169
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_5khsVRINUws",
            "start": 176,
            "end": 189
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
                "value": "./test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws",
                "raw": "\"./test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws\"",
                "start": 203,
                "end": 265
              },
              "options": null,
              "phase": null,
              "start": 196,
              "end": 266
            },
            "id": null,
            "generator": false,
            "start": 192,
            "end": 266
          },
          "start": 176,
          "end": 266
        }
      ],
      "start": 170,
      "end": 267
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_rAeuW6OvuXM",
            "start": 274,
            "end": 287
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
                "value": "./test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
                "raw": "\"./test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM\"",
                "start": 301,
                "end": 365
              },
              "options": null,
              "phase": null,
              "start": 294,
              "end": 366
            },
            "id": null,
            "generator": false,
            "start": 290,
            "end": 366
          },
          "start": 274,
          "end": 366
        }
      ],
      "start": 268,
      "end": 367
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
              "name": "Parent_component_0TaiDayHrlo",
              "start": 381,
              "end": 409
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
                          "name": "cart",
                          "start": 428,
                          "end": 432
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 435,
                            "end": 443
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 444,
                              "end": 446
                            }
                          ],
                          "optional": false,
                          "start": 435,
                          "end": 447
                        },
                        "start": 428,
                        "end": 447
                      }
                    ],
                    "start": 422,
                    "end": 448
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "results",
                          "start": 459,
                          "end": 466
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 469,
                            "end": 478
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Literal",
                                  "value": "foo",
                                  "raw": "'foo'",
                                  "start": 489,
                                  "end": 494
                                },
                                {
                                  "type": "Literal",
                                  "value": "bar",
                                  "raw": "'bar'",
                                  "start": 504,
                                  "end": 509
                                }
                              ],
                              "start": 479,
                              "end": 515
                            }
                          ],
                          "optional": false,
                          "start": 469,
                          "end": 516
                        },
                        "start": 459,
                        "end": 516
                      }
                    ],
                    "start": 453,
                    "end": 517
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
                          "start": 528,
                          "end": 579
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 596,
                            "end": 599
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_rAeuW6OvuXM",
                              "start": 600,
                              "end": 613
                            },
                            {
                              "type": "Literal",
                              "value": "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
                              "raw": "\"Parent_component_div_button_q_e_click_1_rAeuW6OvuXM\"",
                              "start": 615,
                              "end": 668
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "cart",
                                  "start": 680,
                                  "end": 684
                                }
                              ],
                              "start": 670,
                              "end": 690
                            }
                          ],
                          "optional": false,
                          "start": 596,
                          "end": 691
                        },
                        "start": 528,
                        "end": 691
                      }
                    ],
                    "start": 522,
                    "end": 692
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 718,
                        "end": 728
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 729,
                          "end": 734
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 736,
                          "end": 740
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 742,
                          "end": 746
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 772,
                                "end": 782
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "button",
                                  "raw": "\"button\"",
                                  "start": 783,
                                  "end": 791
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 793,
                                  "end": 797
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "id",
                                        "start": 813,
                                        "end": 815
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "first",
                                        "raw": "\"first\"",
                                        "start": 817,
                                        "end": 824
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 813,
                                      "end": 824
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-e:click",
                                        "raw": "\"q-e:click\"",
                                        "start": 838,
                                        "end": 849
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 865,
                                          "end": 868
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_5khsVRINUws",
                                            "start": 869,
                                            "end": 882
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "Parent_component_div_button_q_e_click_5khsVRINUws",
                                            "raw": "\"Parent_component_div_button_q_e_click_5khsVRINUws\"",
                                            "start": 884,
                                            "end": 935
                                          },
                                          {
                                            "type": "ArrayExpression",
                                            "elements": [
                                              {
                                                "type": "Identifier",
                                                "name": "results",
                                                "start": 955,
                                                "end": 962
                                              }
                                            ],
                                            "start": 937,
                                            "end": 976
                                          }
                                        ],
                                        "optional": false,
                                        "start": 865,
                                        "end": 977
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 838,
                                      "end": 977
                                    }
                                  ],
                                  "start": 799,
                                  "end": 987
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 989,
                                  "end": 993
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 995,
                                  "end": 996
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 998,
                                  "end": 1002
                                }
                              ],
                              "optional": false,
                              "start": 772,
                              "end": 1003
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "results",
                                    "start": 1013,
                                    "end": 1020
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 1021,
                                    "end": 1026
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 1013,
                                  "end": 1026
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 1027,
                                  "end": 1030
                                },
                                "optional": false,
                                "computed": false,
                                "start": 1013,
                                "end": 1030
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
                                      "start": 1032,
                                      "end": 1036
                                    },
                                    {
                                      "type": "Identifier",
                                      "name": "key",
                                      "start": 1038,
                                      "end": 1041
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_jsxSorted",
                                      "start": 1058,
                                      "end": 1068
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "button",
                                        "raw": "\"button\"",
                                        "start": 1069,
                                        "end": 1077
                                      },
                                      {
                                        "type": "ObjectExpression",
                                        "properties": [
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "name": "id",
                                              "start": 1097,
                                              "end": 1099
                                            },
                                            "value": {
                                              "type": "BinaryExpression",
                                              "left": {
                                                "type": "Literal",
                                                "value": "second-",
                                                "raw": "'second-'",
                                                "start": 1101,
                                                "end": 1110
                                              },
                                              "operator": "+",
                                              "right": {
                                                "type": "Identifier",
                                                "name": "key",
                                                "start": 1113,
                                                "end": 1116
                                              },
                                              "start": 1101,
                                              "end": 1116
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1097,
                                            "end": 1116
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Literal",
                                              "value": "q-e:click",
                                              "raw": "\"q-e:click\"",
                                              "start": 1134,
                                              "end": 1145
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
                                              "start": 1147,
                                              "end": 1198
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1134,
                                            "end": 1198
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Literal",
                                              "value": "q:p",
                                              "raw": "\"q:p\"",
                                              "start": 1216,
                                              "end": 1221
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "item",
                                              "start": 1223,
                                              "end": 1227
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1216,
                                            "end": 1227
                                          }
                                        ],
                                        "start": 1079,
                                        "end": 1241
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1243,
                                        "end": 1247
                                      },
                                      {
                                        "type": "Identifier",
                                        "name": "item",
                                        "start": 1249,
                                        "end": 1253
                                      },
                                      {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 1255,
                                        "end": 1256
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "u6_0",
                                        "raw": "\"u6_0\"",
                                        "start": 1258,
                                        "end": 1264
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1058,
                                    "end": 1265
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 1031,
                                  "end": 1265
                                }
                              ],
                              "optional": false,
                              "start": 1013,
                              "end": 1266
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
                                  "value": "ul",
                                  "raw": "\"ul\"",
                                  "start": 1301,
                                  "end": 1305
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1307,
                                  "end": 1311
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1313,
                                  "end": 1317
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "name": "cart",
                                      "start": 1319,
                                      "end": 1323
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "map",
                                      "start": 1324,
                                      "end": 1327
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 1319,
                                    "end": 1327
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
                                          "start": 1329,
                                          "end": 1333
                                        }
                                      ],
                                      "body": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1350,
                                          "end": 1360
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "li",
                                            "raw": "\"li\"",
                                            "start": 1361,
                                            "end": 1365
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1367,
                                            "end": 1371
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1373,
                                            "end": 1377
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_jsxSorted",
                                              "start": 1393,
                                              "end": 1403
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "span",
                                                "raw": "\"span\"",
                                                "start": 1404,
                                                "end": 1410
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1412,
                                                "end": 1416
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1418,
                                                "end": 1422
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "item",
                                                "start": 1424,
                                                "end": 1428
                                              },
                                              {
                                                "type": "Literal",
                                                "value": 1,
                                                "raw": "1",
                                                "start": 1430,
                                                "end": 1431
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1433,
                                                "end": 1437
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1393,
                                            "end": 1438
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 1440,
                                            "end": 1441
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "u6_1",
                                            "raw": "\"u6_1\"",
                                            "start": 1443,
                                            "end": 1449
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1350,
                                        "end": 1450
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 1328,
                                      "end": 1450
                                    }
                                  ],
                                  "optional": false,
                                  "start": 1319,
                                  "end": 1451
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 1453,
                                  "end": 1454
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1456,
                                  "end": 1460
                                }
                              ],
                              "optional": false,
                              "start": 1290,
                              "end": 1461
                            }
                          ],
                          "start": 748,
                          "end": 1467
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1469,
                          "end": 1470
                        },
                        {
                          "type": "Literal",
                          "value": "u6_2",
                          "raw": "\"u6_2\"",
                          "start": 1472,
                          "end": 1478
                        }
                      ],
                      "optional": false,
                      "start": 718,
                      "end": 1479
                    },
                    "start": 697,
                    "end": 1480
                  }
                ],
                "start": 416,
                "end": 1482
              },
              "id": null,
              "generator": false,
              "start": 412,
              "end": 1482
            },
            "start": 381,
            "end": 1482
          }
        ],
        "start": 375,
        "end": 1483
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 368,
      "end": 1483
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1483
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_0TaiDayHrlo",
  "entry": null,
  "displayName": "test.tsx_Parent_component",
  "hash": "0TaiDayHrlo",
  "canonicalFilename": "test.tsx_Parent_component_0TaiDayHrlo",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    103,
    762
  ]
}
```

### Module: `test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Parent_component_div_button_q_e_click_1_rAeuW6OvuXM = (_, _1, item)=>{
    const cart = _captures[0];
    cart.push(item);
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
              "name": "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
              "start": 57,
              "end": 108
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 112,
                  "end": 113
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 115,
                  "end": 117
                },
                {
                  "type": "Identifier",
                  "name": "item",
                  "start": 119,
                  "end": 123
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
                          "name": "cart",
                          "start": 138,
                          "end": 142
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 145,
                            "end": 154
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 155,
                            "end": 156
                          },
                          "optional": false,
                          "computed": true,
                          "start": 145,
                          "end": 157
                        },
                        "start": 138,
                        "end": 157
                      }
                    ],
                    "start": 132,
                    "end": 158
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "cart",
                          "start": 163,
                          "end": 167
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "push",
                          "start": 168,
                          "end": 172
                        },
                        "optional": false,
                        "computed": false,
                        "start": 163,
                        "end": 172
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "item",
                          "start": 173,
                          "end": 177
                        }
                      ],
                      "optional": false,
                      "start": 163,
                      "end": 178
                    },
                    "start": 163,
                    "end": 179
                  }
                ],
                "start": 126,
                "end": 181
              },
              "id": null,
              "generator": false,
              "start": 111,
              "end": 181
            },
            "start": 57,
            "end": 181
          }
        ],
        "start": 51,
        "end": 182
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 182
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 182
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
  "entry": null,
  "displayName": "test.tsx_Parent_component_div_button_q_e_click_1",
  "hash": "rAeuW6OvuXM",
  "canonicalFilename": "test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM",
  "path": "",
  "extension": "js",
  "parent": "Parent_component_0TaiDayHrlo",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    448,
    504
  ],
  "paramNames": [
    "_",
    "_",
    "item"
  ],
  "captureNames": [
    "cart"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (3 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 10 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_captures` | `test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws.js` | `@qwik.dev/core` | 2 |
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Parent_component_0TaiDayHrlo.js` | `@qwik.dev/core` | 3 |
| `_jsxSorted` | `test.tsx_Parent_component_0TaiDayHrlo.js` | `@qwik.dev/core` | 7 |
| `useSignal` | `test.tsx_Parent_component_0TaiDayHrlo.js` | `@qwik.dev/core` | 2 |
| `useStore` | `test.tsx_Parent_component_0TaiDayHrlo.js` | `@qwik.dev/core` | 2 |
| `_captures` | `test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
