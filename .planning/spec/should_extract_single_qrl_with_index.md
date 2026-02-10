# Test: should_extract_single_qrl_with_index

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, useSignal, Signal } from '@qwik.dev/core';
export const App = component$(() => {
	const data = useSignal<Signal<any>[]>([]);
	const selectedItem = useSignal<any | null>(null);
	const clickedIndex = useSignal<number | null>(null);
    return (
        <div>
          {data.value.map((row, idx) => (
              <tr
                key={untrack(() => row.value.id)}
                class={row.value.selected.value ? "danger" : ""}
              >
                <td class="col-md-1">{row.value.id}</td>
                <td class="col-md-4">
                  <a
                    onClick$={() => {
                      if (selectedItem.value) {
                        selectedItem.value.selected.value = false;
                      }
                      selectedItem.value = row.value;
                      row.value.selected.value = true;
					  clickedIndex.value = idx;
                    }}
                  >
                    {row.value.label.value}
                  </a>
                </td>
                <td class="col-md-1">
                  <a
                    onClick$={() => {
                      const dataValue = untrack(() => data.value);
                      data.value = dataValue.toSpliced(
                        dataValue.findIndex((d) => d.value.id === row.value.id),
                        1,
                      );
                    }}
                  >
                    <span aria-hidden="true">x</span>
                  </a>
                </td>
                <td class="col-md-6" />
              </tr>
          ))}
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
              "name": "App",
              "optional": false,
              "typeAnnotation": null,
              "start": 80,
              "end": 83
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 86,
                "end": 96
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
                              "start": 112,
                              "end": 116
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 119,
                                "end": 128
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
                                        "start": 129,
                                        "end": 135
                                      },
                                      "typeArguments": {
                                        "type": "TSTypeParameterInstantiation",
                                        "params": [
                                          {
                                            "type": "TSAnyKeyword",
                                            "start": 136,
                                            "end": 139
                                          }
                                        ],
                                        "start": 135,
                                        "end": 140
                                      },
                                      "start": 129,
                                      "end": 140
                                    },
                                    "start": 129,
                                    "end": 142
                                  }
                                ],
                                "start": 128,
                                "end": 143
                              },
                              "arguments": [
                                {
                                  "type": "ArrayExpression",
                                  "elements": [],
                                  "start": 144,
                                  "end": 146
                                }
                              ],
                              "optional": false,
                              "start": 119,
                              "end": 147
                            },
                            "definite": false,
                            "start": 112,
                            "end": 147
                          }
                        ],
                        "declare": false,
                        "start": 106,
                        "end": 148
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
                              "name": "selectedItem",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 156,
                              "end": 168
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 171,
                                "end": 180
                              },
                              "typeArguments": {
                                "type": "TSTypeParameterInstantiation",
                                "params": [
                                  {
                                    "type": "TSUnionType",
                                    "types": [
                                      {
                                        "type": "TSAnyKeyword",
                                        "start": 181,
                                        "end": 184
                                      },
                                      {
                                        "type": "TSNullKeyword",
                                        "start": 187,
                                        "end": 191
                                      }
                                    ],
                                    "start": 181,
                                    "end": 191
                                  }
                                ],
                                "start": 180,
                                "end": 192
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 193,
                                  "end": 197
                                }
                              ],
                              "optional": false,
                              "start": 171,
                              "end": 198
                            },
                            "definite": false,
                            "start": 156,
                            "end": 198
                          }
                        ],
                        "declare": false,
                        "start": 150,
                        "end": 199
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
                              "name": "clickedIndex",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 207,
                              "end": 219
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 222,
                                "end": 231
                              },
                              "typeArguments": {
                                "type": "TSTypeParameterInstantiation",
                                "params": [
                                  {
                                    "type": "TSUnionType",
                                    "types": [
                                      {
                                        "type": "TSNumberKeyword",
                                        "start": 232,
                                        "end": 238
                                      },
                                      {
                                        "type": "TSNullKeyword",
                                        "start": 241,
                                        "end": 245
                                      }
                                    ],
                                    "start": 232,
                                    "end": 245
                                  }
                                ],
                                "start": 231,
                                "end": 246
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 247,
                                  "end": 251
                                }
                              ],
                              "optional": false,
                              "start": 222,
                              "end": 252
                            },
                            "definite": false,
                            "start": 207,
                            "end": 252
                          }
                        ],
                        "declare": false,
                        "start": 201,
                        "end": 253
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
                                "start": 276,
                                "end": 279
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 275,
                              "end": 280
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n          ",
                                "raw": "\n          ",
                                "start": 280,
                                "end": 291
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
                                      "name": "map",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 303,
                                      "end": 306
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 292,
                                    "end": 306
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
                                          "start": 308,
                                          "end": 311
                                        },
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "idx",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 313,
                                          "end": 316
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
                                              "name": "tr",
                                              "start": 338,
                                              "end": 340
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "key",
                                                  "start": 357,
                                                  "end": 360
                                                },
                                                "value": {
                                                  "type": "JSXExpressionContainer",
                                                  "expression": {
                                                    "type": "CallExpression",
                                                    "callee": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "untrack",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 362,
                                                      "end": 369
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "ArrowFunctionExpression",
                                                        "expression": true,
                                                        "async": false,
                                                        "typeParameters": null,
                                                        "params": [],
                                                        "returnType": null,
                                                        "body": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "MemberExpression",
                                                            "object": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "row",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 376,
                                                              "end": 379
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 380,
                                                              "end": 385
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 376,
                                                            "end": 385
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 386,
                                                            "end": 388
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 376,
                                                          "end": 388
                                                        },
                                                        "id": null,
                                                        "generator": false,
                                                        "start": 370,
                                                        "end": 388
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 362,
                                                    "end": 389
                                                  },
                                                  "start": 361,
                                                  "end": 390
                                                },
                                                "start": 357,
                                                "end": 390
                                              },
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "class",
                                                  "start": 407,
                                                  "end": 412
                                                },
                                                "value": {
                                                  "type": "JSXExpressionContainer",
                                                  "expression": {
                                                    "type": "ConditionalExpression",
                                                    "test": {
                                                      "type": "MemberExpression",
                                                      "object": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "row",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 414,
                                                            "end": 417
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "value",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 418,
                                                            "end": 423
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 414,
                                                          "end": 423
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "selected",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 424,
                                                          "end": 432
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 414,
                                                        "end": 432
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 433,
                                                        "end": 438
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 414,
                                                      "end": 438
                                                    },
                                                    "consequent": {
                                                      "type": "Literal",
                                                      "value": "danger",
                                                      "raw": "\"danger\"",
                                                      "start": 441,
                                                      "end": 449
                                                    },
                                                    "alternate": {
                                                      "type": "Literal",
                                                      "value": "",
                                                      "raw": "\"\"",
                                                      "start": 452,
                                                      "end": 454
                                                    },
                                                    "start": 414,
                                                    "end": 454
                                                  },
                                                  "start": 413,
                                                  "end": 455
                                                },
                                                "start": 407,
                                                "end": 455
                                              }
                                            ],
                                            "selfClosing": false,
                                            "start": 337,
                                            "end": 471
                                          },
                                          "children": [
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 471,
                                              "end": 488
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 489,
                                                  "end": 491
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 492,
                                                      "end": 497
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-1",
                                                      "raw": "\"col-md-1\"",
                                                      "start": 498,
                                                      "end": 508
                                                    },
                                                    "start": 492,
                                                    "end": 508
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 488,
                                                "end": 509
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
                                                        "start": 510,
                                                        "end": 513
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 514,
                                                        "end": 519
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 510,
                                                      "end": 519
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "id",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 520,
                                                      "end": 522
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 510,
                                                    "end": 522
                                                  },
                                                  "start": 509,
                                                  "end": 523
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 525,
                                                  "end": 527
                                                },
                                                "start": 523,
                                                "end": 528
                                              },
                                              "start": 488,
                                              "end": 528
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 528,
                                              "end": 545
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 546,
                                                  "end": 548
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 549,
                                                      "end": 554
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-4",
                                                      "raw": "\"col-md-4\"",
                                                      "start": 555,
                                                      "end": 565
                                                    },
                                                    "start": 549,
                                                    "end": 565
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 545,
                                                "end": 566
                                              },
                                              "children": [
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                  ",
                                                  "raw": "\n                  ",
                                                  "start": 566,
                                                  "end": 585
                                                },
                                                {
                                                  "type": "JSXElement",
                                                  "openingElement": {
                                                    "type": "JSXOpeningElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 586,
                                                      "end": 587
                                                    },
                                                    "typeArguments": null,
                                                    "attributes": [
                                                      {
                                                        "type": "JSXAttribute",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "onClick$",
                                                          "start": 608,
                                                          "end": 616
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
                                                                  "type": "IfStatement",
                                                                  "test": {
                                                                    "type": "MemberExpression",
                                                                    "object": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "selectedItem",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 652,
                                                                      "end": 664
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "value",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 665,
                                                                      "end": 670
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 652,
                                                                    "end": 670
                                                                  },
                                                                  "consequent": {
                                                                    "type": "BlockStatement",
                                                                    "body": [
                                                                      {
                                                                        "type": "ExpressionStatement",
                                                                        "expression": {
                                                                          "type": "AssignmentExpression",
                                                                          "operator": "=",
                                                                          "left": {
                                                                            "type": "MemberExpression",
                                                                            "object": {
                                                                              "type": "MemberExpression",
                                                                              "object": {
                                                                                "type": "MemberExpression",
                                                                                "object": {
                                                                                  "type": "Identifier",
                                                                                  "decorators": [],
                                                                                  "name": "selectedItem",
                                                                                  "optional": false,
                                                                                  "typeAnnotation": null,
                                                                                  "start": 698,
                                                                                  "end": 710
                                                                                },
                                                                                "property": {
                                                                                  "type": "Identifier",
                                                                                  "decorators": [],
                                                                                  "name": "value",
                                                                                  "optional": false,
                                                                                  "typeAnnotation": null,
                                                                                  "start": 711,
                                                                                  "end": 716
                                                                                },
                                                                                "optional": false,
                                                                                "computed": false,
                                                                                "start": 698,
                                                                                "end": 716
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "selected",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 717,
                                                                                "end": 725
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 698,
                                                                              "end": 725
                                                                            },
                                                                            "property": {
                                                                              "type": "Identifier",
                                                                              "decorators": [],
                                                                              "name": "value",
                                                                              "optional": false,
                                                                              "typeAnnotation": null,
                                                                              "start": 726,
                                                                              "end": 731
                                                                            },
                                                                            "optional": false,
                                                                            "computed": false,
                                                                            "start": 698,
                                                                            "end": 731
                                                                          },
                                                                          "right": {
                                                                            "type": "Literal",
                                                                            "value": false,
                                                                            "raw": "false",
                                                                            "start": 734,
                                                                            "end": 739
                                                                          },
                                                                          "start": 698,
                                                                          "end": 739
                                                                        },
                                                                        "directive": null,
                                                                        "start": 698,
                                                                        "end": 740
                                                                      }
                                                                    ],
                                                                    "start": 672,
                                                                    "end": 764
                                                                  },
                                                                  "alternate": null,
                                                                  "start": 648,
                                                                  "end": 764
                                                                },
                                                                {
                                                                  "type": "ExpressionStatement",
                                                                  "expression": {
                                                                    "type": "AssignmentExpression",
                                                                    "operator": "=",
                                                                    "left": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "selectedItem",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 787,
                                                                        "end": 799
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 800,
                                                                        "end": 805
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 787,
                                                                      "end": 805
                                                                    },
                                                                    "right": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "row",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 808,
                                                                        "end": 811
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 812,
                                                                        "end": 817
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 808,
                                                                      "end": 817
                                                                    },
                                                                    "start": 787,
                                                                    "end": 817
                                                                  },
                                                                  "directive": null,
                                                                  "start": 787,
                                                                  "end": 818
                                                                },
                                                                {
                                                                  "type": "ExpressionStatement",
                                                                  "expression": {
                                                                    "type": "AssignmentExpression",
                                                                    "operator": "=",
                                                                    "left": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "MemberExpression",
                                                                        "object": {
                                                                          "type": "MemberExpression",
                                                                          "object": {
                                                                            "type": "Identifier",
                                                                            "decorators": [],
                                                                            "name": "row",
                                                                            "optional": false,
                                                                            "typeAnnotation": null,
                                                                            "start": 841,
                                                                            "end": 844
                                                                          },
                                                                          "property": {
                                                                            "type": "Identifier",
                                                                            "decorators": [],
                                                                            "name": "value",
                                                                            "optional": false,
                                                                            "typeAnnotation": null,
                                                                            "start": 845,
                                                                            "end": 850
                                                                          },
                                                                          "optional": false,
                                                                          "computed": false,
                                                                          "start": 841,
                                                                          "end": 850
                                                                        },
                                                                        "property": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "selected",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 851,
                                                                          "end": 859
                                                                        },
                                                                        "optional": false,
                                                                        "computed": false,
                                                                        "start": 841,
                                                                        "end": 859
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 860,
                                                                        "end": 865
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 841,
                                                                      "end": 865
                                                                    },
                                                                    "right": {
                                                                      "type": "Literal",
                                                                      "value": true,
                                                                      "raw": "true",
                                                                      "start": 868,
                                                                      "end": 872
                                                                    },
                                                                    "start": 841,
                                                                    "end": 872
                                                                  },
                                                                  "directive": null,
                                                                  "start": 841,
                                                                  "end": 873
                                                                },
                                                                {
                                                                  "type": "ExpressionStatement",
                                                                  "expression": {
                                                                    "type": "AssignmentExpression",
                                                                    "operator": "=",
                                                                    "left": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "clickedIndex",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 881,
                                                                        "end": 893
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 894,
                                                                        "end": 899
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 881,
                                                                      "end": 899
                                                                    },
                                                                    "right": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "idx",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 902,
                                                                      "end": 905
                                                                    },
                                                                    "start": 881,
                                                                    "end": 905
                                                                  },
                                                                  "directive": null,
                                                                  "start": 881,
                                                                  "end": 906
                                                                }
                                                              ],
                                                              "start": 624,
                                                              "end": 928
                                                            },
                                                            "id": null,
                                                            "generator": false,
                                                            "start": 618,
                                                            "end": 928
                                                          },
                                                          "start": 617,
                                                          "end": 929
                                                        },
                                                        "start": 608,
                                                        "end": 929
                                                      }
                                                    ],
                                                    "selfClosing": false,
                                                    "start": 585,
                                                    "end": 949
                                                  },
                                                  "children": [
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                    ",
                                                      "raw": "\n                    ",
                                                      "start": 949,
                                                      "end": 970
                                                    },
                                                    {
                                                      "type": "JSXExpressionContainer",
                                                      "expression": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "MemberExpression",
                                                            "object": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "row",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 971,
                                                              "end": 974
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 975,
                                                              "end": 980
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 971,
                                                            "end": 980
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "label",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 981,
                                                            "end": 986
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 971,
                                                          "end": 986
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "value",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 987,
                                                          "end": 992
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 971,
                                                        "end": 992
                                                      },
                                                      "start": 970,
                                                      "end": 993
                                                    },
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                  ",
                                                      "raw": "\n                  ",
                                                      "start": 993,
                                                      "end": 1012
                                                    }
                                                  ],
                                                  "closingElement": {
                                                    "type": "JSXClosingElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 1014,
                                                      "end": 1015
                                                    },
                                                    "start": 1012,
                                                    "end": 1016
                                                  },
                                                  "start": 585,
                                                  "end": 1016
                                                },
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                ",
                                                  "raw": "\n                ",
                                                  "start": 1016,
                                                  "end": 1033
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1035,
                                                  "end": 1037
                                                },
                                                "start": 1033,
                                                "end": 1038
                                              },
                                              "start": 545,
                                              "end": 1038
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 1038,
                                              "end": 1055
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1056,
                                                  "end": 1058
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 1059,
                                                      "end": 1064
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-1",
                                                      "raw": "\"col-md-1\"",
                                                      "start": 1065,
                                                      "end": 1075
                                                    },
                                                    "start": 1059,
                                                    "end": 1075
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 1055,
                                                "end": 1076
                                              },
                                              "children": [
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                  ",
                                                  "raw": "\n                  ",
                                                  "start": 1076,
                                                  "end": 1095
                                                },
                                                {
                                                  "type": "JSXElement",
                                                  "openingElement": {
                                                    "type": "JSXOpeningElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 1096,
                                                      "end": 1097
                                                    },
                                                    "typeArguments": null,
                                                    "attributes": [
                                                      {
                                                        "type": "JSXAttribute",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "onClick$",
                                                          "start": 1118,
                                                          "end": 1126
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
                                                                  "type": "VariableDeclaration",
                                                                  "kind": "const",
                                                                  "declarations": [
                                                                    {
                                                                      "type": "VariableDeclarator",
                                                                      "id": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "dataValue",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 1164,
                                                                        "end": 1173
                                                                      },
                                                                      "init": {
                                                                        "type": "CallExpression",
                                                                        "callee": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "untrack",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 1176,
                                                                          "end": 1183
                                                                        },
                                                                        "typeArguments": null,
                                                                        "arguments": [
                                                                          {
                                                                            "type": "ArrowFunctionExpression",
                                                                            "expression": true,
                                                                            "async": false,
                                                                            "typeParameters": null,
                                                                            "params": [],
                                                                            "returnType": null,
                                                                            "body": {
                                                                              "type": "MemberExpression",
                                                                              "object": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "data",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 1190,
                                                                                "end": 1194
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "value",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 1195,
                                                                                "end": 1200
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 1190,
                                                                              "end": 1200
                                                                            },
                                                                            "id": null,
                                                                            "generator": false,
                                                                            "start": 1184,
                                                                            "end": 1200
                                                                          }
                                                                        ],
                                                                        "optional": false,
                                                                        "start": 1176,
                                                                        "end": 1201
                                                                      },
                                                                      "definite": false,
                                                                      "start": 1164,
                                                                      "end": 1201
                                                                    }
                                                                  ],
                                                                  "declare": false,
                                                                  "start": 1158,
                                                                  "end": 1202
                                                                },
                                                                {
                                                                  "type": "ExpressionStatement",
                                                                  "expression": {
                                                                    "type": "AssignmentExpression",
                                                                    "operator": "=",
                                                                    "left": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "data",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 1225,
                                                                        "end": 1229
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 1230,
                                                                        "end": 1235
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 1225,
                                                                      "end": 1235
                                                                    },
                                                                    "right": {
                                                                      "type": "CallExpression",
                                                                      "callee": {
                                                                        "type": "MemberExpression",
                                                                        "object": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "dataValue",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 1238,
                                                                          "end": 1247
                                                                        },
                                                                        "property": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "toSpliced",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 1248,
                                                                          "end": 1257
                                                                        },
                                                                        "optional": false,
                                                                        "computed": false,
                                                                        "start": 1238,
                                                                        "end": 1257
                                                                      },
                                                                      "typeArguments": null,
                                                                      "arguments": [
                                                                        {
                                                                          "type": "CallExpression",
                                                                          "callee": {
                                                                            "type": "MemberExpression",
                                                                            "object": {
                                                                              "type": "Identifier",
                                                                              "decorators": [],
                                                                              "name": "dataValue",
                                                                              "optional": false,
                                                                              "typeAnnotation": null,
                                                                              "start": 1283,
                                                                              "end": 1292
                                                                            },
                                                                            "property": {
                                                                              "type": "Identifier",
                                                                              "decorators": [],
                                                                              "name": "findIndex",
                                                                              "optional": false,
                                                                              "typeAnnotation": null,
                                                                              "start": 1293,
                                                                              "end": 1302
                                                                            },
                                                                            "optional": false,
                                                                            "computed": false,
                                                                            "start": 1283,
                                                                            "end": 1302
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
                                                                                  "name": "d",
                                                                                  "optional": false,
                                                                                  "typeAnnotation": null,
                                                                                  "start": 1304,
                                                                                  "end": 1305
                                                                                }
                                                                              ],
                                                                              "returnType": null,
                                                                              "body": {
                                                                                "type": "BinaryExpression",
                                                                                "left": {
                                                                                  "type": "MemberExpression",
                                                                                  "object": {
                                                                                    "type": "MemberExpression",
                                                                                    "object": {
                                                                                      "type": "Identifier",
                                                                                      "decorators": [],
                                                                                      "name": "d",
                                                                                      "optional": false,
                                                                                      "typeAnnotation": null,
                                                                                      "start": 1310,
                                                                                      "end": 1311
                                                                                    },
                                                                                    "property": {
                                                                                      "type": "Identifier",
                                                                                      "decorators": [],
                                                                                      "name": "value",
                                                                                      "optional": false,
                                                                                      "typeAnnotation": null,
                                                                                      "start": 1312,
                                                                                      "end": 1317
                                                                                    },
                                                                                    "optional": false,
                                                                                    "computed": false,
                                                                                    "start": 1310,
                                                                                    "end": 1317
                                                                                  },
                                                                                  "property": {
                                                                                    "type": "Identifier",
                                                                                    "decorators": [],
                                                                                    "name": "id",
                                                                                    "optional": false,
                                                                                    "typeAnnotation": null,
                                                                                    "start": 1318,
                                                                                    "end": 1320
                                                                                  },
                                                                                  "optional": false,
                                                                                  "computed": false,
                                                                                  "start": 1310,
                                                                                  "end": 1320
                                                                                },
                                                                                "operator": "===",
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
                                                                                      "start": 1325,
                                                                                      "end": 1328
                                                                                    },
                                                                                    "property": {
                                                                                      "type": "Identifier",
                                                                                      "decorators": [],
                                                                                      "name": "value",
                                                                                      "optional": false,
                                                                                      "typeAnnotation": null,
                                                                                      "start": 1329,
                                                                                      "end": 1334
                                                                                    },
                                                                                    "optional": false,
                                                                                    "computed": false,
                                                                                    "start": 1325,
                                                                                    "end": 1334
                                                                                  },
                                                                                  "property": {
                                                                                    "type": "Identifier",
                                                                                    "decorators": [],
                                                                                    "name": "id",
                                                                                    "optional": false,
                                                                                    "typeAnnotation": null,
                                                                                    "start": 1335,
                                                                                    "end": 1337
                                                                                  },
                                                                                  "optional": false,
                                                                                  "computed": false,
                                                                                  "start": 1325,
                                                                                  "end": 1337
                                                                                },
                                                                                "start": 1310,
                                                                                "end": 1337
                                                                              },
                                                                              "id": null,
                                                                              "generator": false,
                                                                              "start": 1303,
                                                                              "end": 1337
                                                                            }
                                                                          ],
                                                                          "optional": false,
                                                                          "start": 1283,
                                                                          "end": 1338
                                                                        },
                                                                        {
                                                                          "type": "Literal",
                                                                          "value": 1,
                                                                          "raw": "1",
                                                                          "start": 1364,
                                                                          "end": 1365
                                                                        }
                                                                      ],
                                                                      "optional": false,
                                                                      "start": 1238,
                                                                      "end": 1390
                                                                    },
                                                                    "start": 1225,
                                                                    "end": 1390
                                                                  },
                                                                  "directive": null,
                                                                  "start": 1225,
                                                                  "end": 1391
                                                                }
                                                              ],
                                                              "start": 1134,
                                                              "end": 1413
                                                            },
                                                            "id": null,
                                                            "generator": false,
                                                            "start": 1128,
                                                            "end": 1413
                                                          },
                                                          "start": 1127,
                                                          "end": 1414
                                                        },
                                                        "start": 1118,
                                                        "end": 1414
                                                      }
                                                    ],
                                                    "selfClosing": false,
                                                    "start": 1095,
                                                    "end": 1434
                                                  },
                                                  "children": [
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                    ",
                                                      "raw": "\n                    ",
                                                      "start": 1434,
                                                      "end": 1455
                                                    },
                                                    {
                                                      "type": "JSXElement",
                                                      "openingElement": {
                                                        "type": "JSXOpeningElement",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "span",
                                                          "start": 1456,
                                                          "end": 1460
                                                        },
                                                        "typeArguments": null,
                                                        "attributes": [
                                                          {
                                                            "type": "JSXAttribute",
                                                            "name": {
                                                              "type": "JSXIdentifier",
                                                              "name": "aria-hidden",
                                                              "start": 1461,
                                                              "end": 1472
                                                            },
                                                            "value": {
                                                              "type": "Literal",
                                                              "value": "true",
                                                              "raw": "\"true\"",
                                                              "start": 1473,
                                                              "end": 1479
                                                            },
                                                            "start": 1461,
                                                            "end": 1479
                                                          }
                                                        ],
                                                        "selfClosing": false,
                                                        "start": 1455,
                                                        "end": 1480
                                                      },
                                                      "children": [
                                                        {
                                                          "type": "JSXText",
                                                          "value": "x",
                                                          "raw": "x",
                                                          "start": 1480,
                                                          "end": 1481
                                                        }
                                                      ],
                                                      "closingElement": {
                                                        "type": "JSXClosingElement",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "span",
                                                          "start": 1483,
                                                          "end": 1487
                                                        },
                                                        "start": 1481,
                                                        "end": 1488
                                                      },
                                                      "start": 1455,
                                                      "end": 1488
                                                    },
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                  ",
                                                      "raw": "\n                  ",
                                                      "start": 1488,
                                                      "end": 1507
                                                    }
                                                  ],
                                                  "closingElement": {
                                                    "type": "JSXClosingElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 1509,
                                                      "end": 1510
                                                    },
                                                    "start": 1507,
                                                    "end": 1511
                                                  },
                                                  "start": 1095,
                                                  "end": 1511
                                                },
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                ",
                                                  "raw": "\n                ",
                                                  "start": 1511,
                                                  "end": 1528
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1530,
                                                  "end": 1532
                                                },
                                                "start": 1528,
                                                "end": 1533
                                              },
                                              "start": 1055,
                                              "end": 1533
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 1533,
                                              "end": 1550
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1551,
                                                  "end": 1553
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 1554,
                                                      "end": 1559
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-6",
                                                      "raw": "\"col-md-6\"",
                                                      "start": 1560,
                                                      "end": 1570
                                                    },
                                                    "start": 1554,
                                                    "end": 1570
                                                  }
                                                ],
                                                "selfClosing": true,
                                                "start": 1550,
                                                "end": 1573
                                              },
                                              "children": [],
                                              "closingElement": null,
                                              "start": 1550,
                                              "end": 1573
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n              ",
                                              "raw": "\n              ",
                                              "start": 1573,
                                              "end": 1588
                                            }
                                          ],
                                          "closingElement": {
                                            "type": "JSXClosingElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "tr",
                                              "start": 1590,
                                              "end": 1592
                                            },
                                            "start": 1588,
                                            "end": 1593
                                          },
                                          "start": 337,
                                          "end": 1593
                                        },
                                        "start": 321,
                                        "end": 1605
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 307,
                                      "end": 1605
                                    }
                                  ],
                                  "optional": false,
                                  "start": 292,
                                  "end": 1606
                                },
                                "start": 291,
                                "end": 1607
                              },
                              {
                                "type": "JSXText",
                                "value": "\n        ",
                                "raw": "\n        ",
                                "start": 1607,
                                "end": 1616
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 1618,
                                "end": 1621
                              },
                              "start": 1616,
                              "end": 1622
                            },
                            "start": 275,
                            "end": 1622
                          },
                          "start": 265,
                          "end": 1630
                        },
                        "start": 258,
                        "end": 1631
                      }
                    ],
                    "start": 103,
                    "end": 1637
                  },
                  "id": null,
                  "generator": false,
                  "start": 97,
                  "end": 1637
                }
              ],
              "optional": false,
              "start": 86,
              "end": 1638
            },
            "definite": false,
            "start": 80,
            "end": 1638
          }
        ],
        "declare": false,
        "start": 74,
        "end": 1639
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 67,
      "end": 1639
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1639
}
```

</details>

## Output

### Module: `test.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
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
            "name": "i_ckEPmXZlub0",
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
                "value": "./test.tsx_App_component_ckEPmXZlub0",
                "raw": "\"./test.tsx_App_component_ckEPmXZlub0\"",
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
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "name": "App",
              "start": 172,
              "end": 175
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 192,
                "end": 204
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 219,
                    "end": 222
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_ckEPmXZlub0",
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 238,
                      "end": 265
                    }
                  ],
                  "optional": false,
                  "start": 219,
                  "end": 266
                }
              ],
              "optional": false,
              "start": 192,
              "end": 267
            },
            "start": 172,
            "end": 267
          }
        ],
        "start": 166,
        "end": 268
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 159,
      "end": 268
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 268
}
```

</details>

### Module: `test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48 = (_, _1, row)=>{
    const data = _captures[0];
    const dataValue = untrack(()=>data.value);
    data.value = dataValue.toSpliced(dataValue.findIndex((d)=>d.value.id === row.value.id), 1);
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
              "name": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
              "start": 57,
              "end": 106
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 110,
                  "end": 111
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 113,
                  "end": 115
                },
                {
                  "type": "Identifier",
                  "name": "row",
                  "start": 117,
                  "end": 120
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
                          "start": 135,
                          "end": 139
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 142,
                            "end": 151
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 152,
                            "end": 153
                          },
                          "optional": false,
                          "computed": true,
                          "start": 142,
                          "end": 154
                        },
                        "start": 135,
                        "end": 154
                      }
                    ],
                    "start": 129,
                    "end": 155
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "dataValue",
                          "start": 166,
                          "end": 175
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "untrack",
                            "start": 178,
                            "end": 185
                          },
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "params": [],
                              "body": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "data",
                                  "start": 190,
                                  "end": 194
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "value",
                                  "start": 195,
                                  "end": 200
                                },
                                "optional": false,
                                "computed": false,
                                "start": 190,
                                "end": 200
                              },
                              "id": null,
                              "generator": false,
                              "start": 186,
                              "end": 200
                            }
                          ],
                          "optional": false,
                          "start": 178,
                          "end": 201
                        },
                        "start": 166,
                        "end": 201
                      }
                    ],
                    "start": 160,
                    "end": 202
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "data",
                          "start": 207,
                          "end": 211
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 212,
                          "end": 217
                        },
                        "optional": false,
                        "computed": false,
                        "start": 207,
                        "end": 217
                      },
                      "right": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "dataValue",
                            "start": 220,
                            "end": 229
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "toSpliced",
                            "start": 230,
                            "end": 239
                          },
                          "optional": false,
                          "computed": false,
                          "start": 220,
                          "end": 239
                        },
                        "arguments": [
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "dataValue",
                                "start": 240,
                                "end": 249
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "findIndex",
                                "start": 250,
                                "end": 259
                              },
                              "optional": false,
                              "computed": false,
                              "start": 240,
                              "end": 259
                            },
                            "arguments": [
                              {
                                "type": "ArrowFunctionExpression",
                                "expression": true,
                                "async": false,
                                "params": [
                                  {
                                    "type": "Identifier",
                                    "name": "d",
                                    "start": 261,
                                    "end": 262
                                  }
                                ],
                                "body": {
                                  "type": "BinaryExpression",
                                  "left": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "name": "d",
                                        "start": 265,
                                        "end": 266
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "value",
                                        "start": 267,
                                        "end": 272
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 265,
                                      "end": 272
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "id",
                                      "start": 273,
                                      "end": 275
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 265,
                                    "end": 275
                                  },
                                  "operator": "===",
                                  "right": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "name": "row",
                                        "start": 280,
                                        "end": 283
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "value",
                                        "start": 284,
                                        "end": 289
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 280,
                                      "end": 289
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "id",
                                      "start": 290,
                                      "end": 292
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 280,
                                    "end": 292
                                  },
                                  "start": 265,
                                  "end": 292
                                },
                                "id": null,
                                "generator": false,
                                "start": 260,
                                "end": 292
                              }
                            ],
                            "optional": false,
                            "start": 240,
                            "end": 293
                          },
                          {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 295,
                            "end": 296
                          }
                        ],
                        "optional": false,
                        "start": 220,
                        "end": 297
                      },
                      "start": 207,
                      "end": 297
                    },
                    "start": 207,
                    "end": 298
                  }
                ],
                "start": 123,
                "end": 300
              },
              "id": null,
              "generator": false,
              "start": 109,
              "end": 300
            },
            "start": 57,
            "end": 300
          }
        ],
        "start": 51,
        "end": 301
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 301
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 301
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
  "entry": null,
  "displayName": "test.tsx_App_component_div_tr_td_a_q_e_click_1",
  "hash": "40fnSAlYI48",
  "canonicalFilename": "test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    1130,
    1415
  ],
  "paramNames": [
    "_",
    "_",
    "row"
  ],
  "captureNames": [
    "data"
  ]
}
```

### Module: `test.tsx_App_component_ckEPmXZlub0.js` (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value.selected.value ? "danger" : "";
const _hf0_str = 'p0.value.selected.value?"danger":""';
const _hf1 = (p0)=>p0.value.id;
const _hf1_str = "p0.value.id";
const _hf2 = (p0)=>p0.value.label.value;
const _hf2_str = "p0.value.label.value";
const i_40fnSAlYI48 = ()=>import("./test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48");
const i_lgbZkJXyLtg = ()=>import("./test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg");
export const App_component_ckEPmXZlub0 = ()=>{
    const data = useSignal([]);
    const selectedItem = useSignal(null);
    const clickedIndex = useSignal(null);
    const App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg = /*#__PURE__*/ qrl(i_lgbZkJXyLtg, "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg", [
        clickedIndex,
        selectedItem
    ]);
    const App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48 = /*#__PURE__*/ qrl(i_40fnSAlYI48, "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48", [
        data
    ]);
    return /*#__PURE__*/ _jsxSorted("div", null, null, data.value.map((row, idx)=>/*#__PURE__*/ _jsxSorted("tr", {
            class: _fnSignal(_hf0, [
                row
            ], _hf0_str)
        }, null, [
            /*#__PURE__*/ _jsxSorted("td", null, {
                class: "col-md-1"
            }, _fnSignal(_hf1, [
                row
            ], _hf1_str), 1, null),
            /*#__PURE__*/ _jsxSorted("td", null, {
                class: "col-md-4"
            }, /*#__PURE__*/ _jsxSorted("a", {
                "q-e:click": App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg,
                "q:ps": [
                    row,
                    idx
                ]
            }, null, _fnSignal(_hf2, [
                row
            ], _hf2_str), 0, null), 1, null),
            /*#__PURE__*/ _jsxSorted("td", null, {
                class: "col-md-1"
            }, /*#__PURE__*/ _jsxSorted("a", {
                "q-e:click": App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48,
                "q:p": row
            }, null, /*#__PURE__*/ _jsxSorted("span", null, {
                "aria-hidden": "true"
            }, "x", 3, null), 2, null), 1, null),
            /*#__PURE__*/ _jsxSorted("td", null, {
                class: "col-md-6"
            }, null, 3, null)
        ], 1, untrack(()=>row.value.id))), 1, "u6_0");
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
              "type": "ConditionalExpression",
              "test": {
                "type": "MemberExpression",
                "object": {
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
                    "name": "selected",
                    "start": 199,
                    "end": 207
                  },
                  "optional": false,
                  "computed": false,
                  "start": 190,
                  "end": 207
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 208,
                  "end": 213
                },
                "optional": false,
                "computed": false,
                "start": 190,
                "end": 213
              },
              "consequent": {
                "type": "Literal",
                "value": "danger",
                "raw": "\"danger\"",
                "start": 216,
                "end": 224
              },
              "alternate": {
                "type": "Literal",
                "value": "",
                "raw": "\"\"",
                "start": 227,
                "end": 229
              },
              "start": 190,
              "end": 229
            },
            "id": null,
            "generator": false,
            "start": 184,
            "end": 229
          },
          "start": 177,
          "end": 229
        }
      ],
      "start": 171,
      "end": 230
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
            "start": 237,
            "end": 245
          },
          "init": {
            "type": "Literal",
            "value": "p0.value.selected.value?\"danger\":\"\"",
            "raw": "'p0.value.selected.value?\"danger\":\"\"'",
            "start": 248,
            "end": 285
          },
          "start": 237,
          "end": 285
        }
      ],
      "start": 231,
      "end": 286
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf1",
            "start": 293,
            "end": 297
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 301,
                "end": 303
              }
            ],
            "body": {
              "type": "MemberExpression",
              "object": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "name": "p0",
                  "start": 306,
                  "end": 308
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 309,
                  "end": 314
                },
                "optional": false,
                "computed": false,
                "start": 306,
                "end": 314
              },
              "property": {
                "type": "Identifier",
                "name": "id",
                "start": 315,
                "end": 317
              },
              "optional": false,
              "computed": false,
              "start": 306,
              "end": 317
            },
            "id": null,
            "generator": false,
            "start": 300,
            "end": 317
          },
          "start": 293,
          "end": 317
        }
      ],
      "start": 287,
      "end": 318
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf1_str",
            "start": 325,
            "end": 333
          },
          "init": {
            "type": "Literal",
            "value": "p0.value.id",
            "raw": "\"p0.value.id\"",
            "start": 336,
            "end": 349
          },
          "start": 325,
          "end": 349
        }
      ],
      "start": 319,
      "end": 350
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf2",
            "start": 357,
            "end": 361
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 365,
                "end": 367
              }
            ],
            "body": {
              "type": "MemberExpression",
              "object": {
                "type": "MemberExpression",
                "object": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "p0",
                    "start": 370,
                    "end": 372
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "value",
                    "start": 373,
                    "end": 378
                  },
                  "optional": false,
                  "computed": false,
                  "start": 370,
                  "end": 378
                },
                "property": {
                  "type": "Identifier",
                  "name": "label",
                  "start": 379,
                  "end": 384
                },
                "optional": false,
                "computed": false,
                "start": 370,
                "end": 384
              },
              "property": {
                "type": "Identifier",
                "name": "value",
                "start": 385,
                "end": 390
              },
              "optional": false,
              "computed": false,
              "start": 370,
              "end": 390
            },
            "id": null,
            "generator": false,
            "start": 364,
            "end": 390
          },
          "start": 357,
          "end": 390
        }
      ],
      "start": 351,
      "end": 391
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf2_str",
            "start": 398,
            "end": 406
          },
          "init": {
            "type": "Literal",
            "value": "p0.value.label.value",
            "raw": "\"p0.value.label.value\"",
            "start": 409,
            "end": 431
          },
          "start": 398,
          "end": 431
        }
      ],
      "start": 392,
      "end": 432
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_40fnSAlYI48",
            "start": 439,
            "end": 452
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
                "value": "./test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                "raw": "\"./test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48\"",
                "start": 466,
                "end": 528
              },
              "options": null,
              "phase": null,
              "start": 459,
              "end": 529
            },
            "id": null,
            "generator": false,
            "start": 455,
            "end": 529
          },
          "start": 439,
          "end": 529
        }
      ],
      "start": 433,
      "end": 530
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_lgbZkJXyLtg",
            "start": 537,
            "end": 550
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
                "value": "./test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                "raw": "\"./test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg\"",
                "start": 564,
                "end": 624
              },
              "options": null,
              "phase": null,
              "start": 557,
              "end": 625
            },
            "id": null,
            "generator": false,
            "start": 553,
            "end": 625
          },
          "start": 537,
          "end": 625
        }
      ],
      "start": 531,
      "end": 626
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
              "name": "App_component_ckEPmXZlub0",
              "start": 640,
              "end": 665
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
                          "start": 684,
                          "end": 688
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 691,
                            "end": 700
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [],
                              "start": 701,
                              "end": 703
                            }
                          ],
                          "optional": false,
                          "start": 691,
                          "end": 704
                        },
                        "start": 684,
                        "end": 704
                      }
                    ],
                    "start": 678,
                    "end": 705
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "selectedItem",
                          "start": 716,
                          "end": 728
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 731,
                            "end": 740
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 741,
                              "end": 745
                            }
                          ],
                          "optional": false,
                          "start": 731,
                          "end": 746
                        },
                        "start": 716,
                        "end": 746
                      }
                    ],
                    "start": 710,
                    "end": 747
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "clickedIndex",
                          "start": 758,
                          "end": 770
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 773,
                            "end": 782
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 783,
                              "end": 787
                            }
                          ],
                          "optional": false,
                          "start": 773,
                          "end": 788
                        },
                        "start": 758,
                        "end": 788
                      }
                    ],
                    "start": 752,
                    "end": 789
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                          "start": 800,
                          "end": 847
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 864,
                            "end": 867
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_lgbZkJXyLtg",
                              "start": 868,
                              "end": 881
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                              "raw": "\"App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg\"",
                              "start": 883,
                              "end": 932
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "clickedIndex",
                                  "start": 944,
                                  "end": 956
                                },
                                {
                                  "type": "Identifier",
                                  "name": "selectedItem",
                                  "start": 966,
                                  "end": 978
                                }
                              ],
                              "start": 934,
                              "end": 984
                            }
                          ],
                          "optional": false,
                          "start": 864,
                          "end": 985
                        },
                        "start": 800,
                        "end": 985
                      }
                    ],
                    "start": 794,
                    "end": 986
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                          "start": 997,
                          "end": 1046
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 1063,
                            "end": 1066
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_40fnSAlYI48",
                              "start": 1067,
                              "end": 1080
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                              "raw": "\"App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48\"",
                              "start": 1082,
                              "end": 1133
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "data",
                                  "start": 1145,
                                  "end": 1149
                                }
                              ],
                              "start": 1135,
                              "end": 1155
                            }
                          ],
                          "optional": false,
                          "start": 1063,
                          "end": 1156
                        },
                        "start": 997,
                        "end": 1156
                      }
                    ],
                    "start": 991,
                    "end": 1157
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 1183,
                        "end": 1193
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 1194,
                          "end": 1199
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1201,
                          "end": 1205
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1207,
                          "end": 1211
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
                                "start": 1213,
                                "end": 1217
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 1218,
                                "end": 1223
                              },
                              "optional": false,
                              "computed": false,
                              "start": 1213,
                              "end": 1223
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 1224,
                              "end": 1227
                            },
                            "optional": false,
                            "computed": false,
                            "start": 1213,
                            "end": 1227
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
                                  "start": 1229,
                                  "end": 1232
                                },
                                {
                                  "type": "Identifier",
                                  "name": "idx",
                                  "start": 1234,
                                  "end": 1237
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 1254,
                                  "end": 1264
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "tr",
                                    "raw": "\"tr\"",
                                    "start": 1265,
                                    "end": 1269
                                  },
                                  {
                                    "type": "ObjectExpression",
                                    "properties": [
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Identifier",
                                          "name": "class",
                                          "start": 1285,
                                          "end": 1290
                                        },
                                        "value": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "name": "_fnSignal",
                                            "start": 1292,
                                            "end": 1301
                                          },
                                          "arguments": [
                                            {
                                              "type": "Identifier",
                                              "name": "_hf0",
                                              "start": 1302,
                                              "end": 1306
                                            },
                                            {
                                              "type": "ArrayExpression",
                                              "elements": [
                                                {
                                                  "type": "Identifier",
                                                  "name": "row",
                                                  "start": 1326,
                                                  "end": 1329
                                                }
                                              ],
                                              "start": 1308,
                                              "end": 1343
                                            },
                                            {
                                              "type": "Identifier",
                                              "name": "_hf0_str",
                                              "start": 1345,
                                              "end": 1353
                                            }
                                          ],
                                          "optional": false,
                                          "start": 1292,
                                          "end": 1354
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1285,
                                        "end": 1354
                                      }
                                    ],
                                    "start": 1271,
                                    "end": 1364
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 1366,
                                    "end": 1370
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1400,
                                          "end": 1410
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1411,
                                            "end": 1415
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1417,
                                            "end": 1421
                                          },
                                          {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "class",
                                                  "start": 1441,
                                                  "end": 1446
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-1",
                                                  "raw": "\"col-md-1\"",
                                                  "start": 1448,
                                                  "end": 1458
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 1441,
                                                "end": 1458
                                              }
                                            ],
                                            "start": 1423,
                                            "end": 1472
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_fnSignal",
                                              "start": 1474,
                                              "end": 1483
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "_hf1",
                                                "start": 1484,
                                                "end": 1488
                                              },
                                              {
                                                "type": "ArrayExpression",
                                                "elements": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "row",
                                                    "start": 1508,
                                                    "end": 1511
                                                  }
                                                ],
                                                "start": 1490,
                                                "end": 1525
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "_hf1_str",
                                                "start": 1527,
                                                "end": 1535
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1474,
                                            "end": 1536
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 1538,
                                            "end": 1539
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1541,
                                            "end": 1545
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1400,
                                        "end": 1546
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1574,
                                          "end": 1584
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1585,
                                            "end": 1589
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1591,
                                            "end": 1595
                                          },
                                          {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "class",
                                                  "start": 1615,
                                                  "end": 1620
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-4",
                                                  "raw": "\"col-md-4\"",
                                                  "start": 1622,
                                                  "end": 1632
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 1615,
                                                "end": 1632
                                              }
                                            ],
                                            "start": 1597,
                                            "end": 1646
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_jsxSorted",
                                              "start": 1662,
                                              "end": 1672
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "a",
                                                "raw": "\"a\"",
                                                "start": 1673,
                                                "end": 1676
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
                                                      "start": 1696,
                                                      "end": 1707
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                                                      "start": 1709,
                                                      "end": 1756
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 1696,
                                                    "end": 1756
                                                  },
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Literal",
                                                      "value": "q:ps",
                                                      "raw": "\"q:ps\"",
                                                      "start": 1774,
                                                      "end": 1780
                                                    },
                                                    "value": {
                                                      "type": "ArrayExpression",
                                                      "elements": [
                                                        {
                                                          "type": "Identifier",
                                                          "name": "row",
                                                          "start": 1804,
                                                          "end": 1807
                                                        },
                                                        {
                                                          "type": "Identifier",
                                                          "name": "idx",
                                                          "start": 1829,
                                                          "end": 1832
                                                        }
                                                      ],
                                                      "start": 1782,
                                                      "end": 1850
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 1774,
                                                    "end": 1850
                                                  }
                                                ],
                                                "start": 1678,
                                                "end": 1864
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1866,
                                                "end": 1870
                                              },
                                              {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "_fnSignal",
                                                  "start": 1872,
                                                  "end": 1881
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_hf2",
                                                    "start": 1882,
                                                    "end": 1886
                                                  },
                                                  {
                                                    "type": "ArrayExpression",
                                                    "elements": [
                                                      {
                                                        "type": "Identifier",
                                                        "name": "row",
                                                        "start": 1906,
                                                        "end": 1909
                                                      }
                                                    ],
                                                    "start": 1888,
                                                    "end": 1923
                                                  },
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_hf2_str",
                                                    "start": 1925,
                                                    "end": 1933
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 1872,
                                                "end": 1934
                                              },
                                              {
                                                "type": "Literal",
                                                "value": 0,
                                                "raw": "0",
                                                "start": 1936,
                                                "end": 1937
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1939,
                                                "end": 1943
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1662,
                                            "end": 1944
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 1946,
                                            "end": 1947
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1949,
                                            "end": 1953
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1574,
                                        "end": 1954
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1982,
                                          "end": 1992
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1993,
                                            "end": 1997
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1999,
                                            "end": 2003
                                          },
                                          {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "class",
                                                  "start": 2023,
                                                  "end": 2028
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-1",
                                                  "raw": "\"col-md-1\"",
                                                  "start": 2030,
                                                  "end": 2040
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 2023,
                                                "end": 2040
                                              }
                                            ],
                                            "start": 2005,
                                            "end": 2054
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_jsxSorted",
                                              "start": 2070,
                                              "end": 2080
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "a",
                                                "raw": "\"a\"",
                                                "start": 2081,
                                                "end": 2084
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
                                                      "start": 2104,
                                                      "end": 2115
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                                                      "start": 2117,
                                                      "end": 2166
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 2104,
                                                    "end": 2166
                                                  },
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Literal",
                                                      "value": "q:p",
                                                      "raw": "\"q:p\"",
                                                      "start": 2184,
                                                      "end": 2189
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "row",
                                                      "start": 2191,
                                                      "end": 2194
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 2184,
                                                    "end": 2194
                                                  }
                                                ],
                                                "start": 2086,
                                                "end": 2208
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 2210,
                                                "end": 2214
                                              },
                                              {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "_jsxSorted",
                                                  "start": 2230,
                                                  "end": 2240
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Literal",
                                                    "value": "span",
                                                    "raw": "\"span\"",
                                                    "start": 2241,
                                                    "end": 2247
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": null,
                                                    "raw": "null",
                                                    "start": 2249,
                                                    "end": 2253
                                                  },
                                                  {
                                                    "type": "ObjectExpression",
                                                    "properties": [
                                                      {
                                                        "type": "Property",
                                                        "kind": "init",
                                                        "key": {
                                                          "type": "Literal",
                                                          "value": "aria-hidden",
                                                          "raw": "\"aria-hidden\"",
                                                          "start": 2273,
                                                          "end": 2286
                                                        },
                                                        "value": {
                                                          "type": "Literal",
                                                          "value": "true",
                                                          "raw": "\"true\"",
                                                          "start": 2288,
                                                          "end": 2294
                                                        },
                                                        "method": false,
                                                        "shorthand": false,
                                                        "computed": false,
                                                        "start": 2273,
                                                        "end": 2294
                                                      }
                                                    ],
                                                    "start": 2255,
                                                    "end": 2308
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": "x",
                                                    "raw": "\"x\"",
                                                    "start": 2310,
                                                    "end": 2313
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": 3,
                                                    "raw": "3",
                                                    "start": 2315,
                                                    "end": 2316
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": null,
                                                    "raw": "null",
                                                    "start": 2318,
                                                    "end": 2322
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 2230,
                                                "end": 2323
                                              },
                                              {
                                                "type": "Literal",
                                                "value": 2,
                                                "raw": "2",
                                                "start": 2325,
                                                "end": 2326
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 2328,
                                                "end": 2332
                                              }
                                            ],
                                            "optional": false,
                                            "start": 2070,
                                            "end": 2333
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 2335,
                                            "end": 2336
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2338,
                                            "end": 2342
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1982,
                                        "end": 2343
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 2371,
                                          "end": 2381
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 2382,
                                            "end": 2386
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2388,
                                            "end": 2392
                                          },
                                          {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "class",
                                                  "start": 2412,
                                                  "end": 2417
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-6",
                                                  "raw": "\"col-md-6\"",
                                                  "start": 2419,
                                                  "end": 2429
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 2412,
                                                "end": 2429
                                              }
                                            ],
                                            "start": 2394,
                                            "end": 2443
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2445,
                                            "end": 2449
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 3,
                                            "raw": "3",
                                            "start": 2451,
                                            "end": 2452
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2454,
                                            "end": 2458
                                          }
                                        ],
                                        "optional": false,
                                        "start": 2371,
                                        "end": 2459
                                      }
                                    ],
                                    "start": 1372,
                                    "end": 2469
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 1,
                                    "raw": "1",
                                    "start": 2471,
                                    "end": 2472
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "untrack",
                                      "start": 2474,
                                      "end": 2481
                                    },
                                    "arguments": [
                                      {
                                        "type": "ArrowFunctionExpression",
                                        "expression": true,
                                        "async": false,
                                        "params": [],
                                        "body": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "name": "row",
                                              "start": 2486,
                                              "end": 2489
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "name": "value",
                                              "start": 2490,
                                              "end": 2495
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 2486,
                                            "end": 2495
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "id",
                                            "start": 2496,
                                            "end": 2498
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 2486,
                                          "end": 2498
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 2482,
                                        "end": 2498
                                      }
                                    ],
                                    "optional": false,
                                    "start": 2474,
                                    "end": 2499
                                  }
                                ],
                                "optional": false,
                                "start": 1254,
                                "end": 2500
                              },
                              "id": null,
                              "generator": false,
                              "start": 1228,
                              "end": 2500
                            }
                          ],
                          "optional": false,
                          "start": 1213,
                          "end": 2501
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 2503,
                          "end": 2504
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 2506,
                          "end": 2512
                        }
                      ],
                      "optional": false,
                      "start": 1183,
                      "end": 2513
                    },
                    "start": 1162,
                    "end": 2514
                  }
                ],
                "start": 672,
                "end": 2516
              },
              "id": null,
              "generator": false,
              "start": 668,
              "end": 2516
            },
            "start": 640,
            "end": 2516
          }
        ],
        "start": 634,
        "end": 2517
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 627,
      "end": 2517
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 2517
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    99,
    1639
  ]
}
```

### Module: `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg = (_, _1, row, idx)=>{
    const clickedIndex = _captures[0], selectedItem = _captures[1];
    if (selectedItem.value) selectedItem.value.selected.value = false;
    selectedItem.value = row.value;
    row.value.selected.value = true;
    clickedIndex.value = idx;
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
              "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
              "start": 57,
              "end": 104
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 108,
                  "end": 109
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 111,
                  "end": 113
                },
                {
                  "type": "Identifier",
                  "name": "row",
                  "start": 115,
                  "end": 118
                },
                {
                  "type": "Identifier",
                  "name": "idx",
                  "start": 120,
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
                          "name": "clickedIndex",
                          "start": 138,
                          "end": 150
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 153,
                            "end": 162
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 163,
                            "end": 164
                          },
                          "optional": false,
                          "computed": true,
                          "start": 153,
                          "end": 165
                        },
                        "start": 138,
                        "end": 165
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "selectedItem",
                          "start": 167,
                          "end": 179
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 182,
                            "end": 191
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 192,
                            "end": 193
                          },
                          "optional": false,
                          "computed": true,
                          "start": 182,
                          "end": 194
                        },
                        "start": 167,
                        "end": 194
                      }
                    ],
                    "start": 132,
                    "end": 195
                  },
                  {
                    "type": "IfStatement",
                    "test": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "selectedItem",
                        "start": 204,
                        "end": 216
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 217,
                        "end": 222
                      },
                      "optional": false,
                      "computed": false,
                      "start": 204,
                      "end": 222
                    },
                    "consequent": {
                      "type": "ExpressionStatement",
                      "expression": {
                        "type": "AssignmentExpression",
                        "operator": "=",
                        "left": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "selectedItem",
                                "start": 224,
                                "end": 236
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 237,
                                "end": 242
                              },
                              "optional": false,
                              "computed": false,
                              "start": 224,
                              "end": 242
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "selected",
                              "start": 243,
                              "end": 251
                            },
                            "optional": false,
                            "computed": false,
                            "start": 224,
                            "end": 251
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "value",
                            "start": 252,
                            "end": 257
                          },
                          "optional": false,
                          "computed": false,
                          "start": 224,
                          "end": 257
                        },
                        "right": {
                          "type": "Literal",
                          "value": false,
                          "raw": "false",
                          "start": 260,
                          "end": 265
                        },
                        "start": 224,
                        "end": 265
                      },
                      "start": 224,
                      "end": 266
                    },
                    "alternate": null,
                    "start": 200,
                    "end": 266
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "selectedItem",
                          "start": 271,
                          "end": 283
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 284,
                          "end": 289
                        },
                        "optional": false,
                        "computed": false,
                        "start": 271,
                        "end": 289
                      },
                      "right": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "row",
                          "start": 292,
                          "end": 295
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 296,
                          "end": 301
                        },
                        "optional": false,
                        "computed": false,
                        "start": 292,
                        "end": 301
                      },
                      "start": 271,
                      "end": 301
                    },
                    "start": 271,
                    "end": 302
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "row",
                              "start": 307,
                              "end": 310
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "value",
                              "start": 311,
                              "end": 316
                            },
                            "optional": false,
                            "computed": false,
                            "start": 307,
                            "end": 316
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "selected",
                            "start": 317,
                            "end": 325
                          },
                          "optional": false,
                          "computed": false,
                          "start": 307,
                          "end": 325
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 326,
                          "end": 331
                        },
                        "optional": false,
                        "computed": false,
                        "start": 307,
                        "end": 331
                      },
                      "right": {
                        "type": "Literal",
                        "value": true,
                        "raw": "true",
                        "start": 334,
                        "end": 338
                      },
                      "start": 307,
                      "end": 338
                    },
                    "start": 307,
                    "end": 339
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "clickedIndex",
                          "start": 344,
                          "end": 356
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 357,
                          "end": 362
                        },
                        "optional": false,
                        "computed": false,
                        "start": 344,
                        "end": 362
                      },
                      "right": {
                        "type": "Identifier",
                        "name": "idx",
                        "start": 365,
                        "end": 368
                      },
                      "start": 344,
                      "end": 368
                    },
                    "start": 344,
                    "end": 369
                  }
                ],
                "start": 126,
                "end": 371
              },
              "id": null,
              "generator": false,
              "start": 107,
              "end": 371
            },
            "start": 57,
            "end": 371
          }
        ],
        "start": 51,
        "end": 372
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 372
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 372
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
  "entry": null,
  "displayName": "test.tsx_App_component_div_tr_td_a_q_e_click",
  "hash": "lgbZkJXyLtg",
  "canonicalFilename": "test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    620,
    930
  ],
  "paramNames": [
    "_",
    "_",
    "row",
    "idx"
  ],
  "captureNames": [
    "clickedIndex",
    "selectedItem"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_fnSignal()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (3 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 13 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point module(s)
- **[CONV-14] Hoisted Functions**: 3 hoisted function(s) (`_hfN`/`_hfN_str` pairs) for signal-derived expressions

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_captures` | `test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 3 |
| `_jsxSorted` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 10 |
| `_fnSignal` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 4 |
| `useSignal` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 4 |
| `_captures` | `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` | `@qwik.dev/core` | 3 |

## Diagnostics

```json
[]
```
