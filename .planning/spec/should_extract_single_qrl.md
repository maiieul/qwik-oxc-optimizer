# Test: should_extract_single_qrl

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
    return (
        <div>
          {data.value.map((row) => (
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
                                "start": 222,
                                "end": 225
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 221,
                              "end": 226
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n          ",
                                "raw": "\n          ",
                                "start": 226,
                                "end": 237
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
                                          "name": "row",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 254,
                                          "end": 257
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
                                              "start": 279,
                                              "end": 281
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "key",
                                                  "start": 298,
                                                  "end": 301
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
                                                      "start": 303,
                                                      "end": 310
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
                                                              "start": 317,
                                                              "end": 320
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 321,
                                                              "end": 326
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 317,
                                                            "end": 326
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "id",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 327,
                                                            "end": 329
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 317,
                                                          "end": 329
                                                        },
                                                        "id": null,
                                                        "generator": false,
                                                        "start": 311,
                                                        "end": 329
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 303,
                                                    "end": 330
                                                  },
                                                  "start": 302,
                                                  "end": 331
                                                },
                                                "start": 298,
                                                "end": 331
                                              },
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "class",
                                                  "start": 348,
                                                  "end": 353
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
                                                            "start": 355,
                                                            "end": 358
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "value",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 359,
                                                            "end": 364
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 355,
                                                          "end": 364
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "selected",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 365,
                                                          "end": 373
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 355,
                                                        "end": 373
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 374,
                                                        "end": 379
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 355,
                                                      "end": 379
                                                    },
                                                    "consequent": {
                                                      "type": "Literal",
                                                      "value": "danger",
                                                      "raw": "\"danger\"",
                                                      "start": 382,
                                                      "end": 390
                                                    },
                                                    "alternate": {
                                                      "type": "Literal",
                                                      "value": "",
                                                      "raw": "\"\"",
                                                      "start": 393,
                                                      "end": 395
                                                    },
                                                    "start": 355,
                                                    "end": 395
                                                  },
                                                  "start": 354,
                                                  "end": 396
                                                },
                                                "start": 348,
                                                "end": 396
                                              }
                                            ],
                                            "selfClosing": false,
                                            "start": 278,
                                            "end": 412
                                          },
                                          "children": [
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 412,
                                              "end": 429
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 430,
                                                  "end": 432
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 433,
                                                      "end": 438
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-1",
                                                      "raw": "\"col-md-1\"",
                                                      "start": 439,
                                                      "end": 449
                                                    },
                                                    "start": 433,
                                                    "end": 449
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 429,
                                                "end": 450
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
                                                        "start": 451,
                                                        "end": 454
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 455,
                                                        "end": 460
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 451,
                                                      "end": 460
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "id",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 461,
                                                      "end": 463
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 451,
                                                    "end": 463
                                                  },
                                                  "start": 450,
                                                  "end": 464
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 466,
                                                  "end": 468
                                                },
                                                "start": 464,
                                                "end": 469
                                              },
                                              "start": 429,
                                              "end": 469
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 469,
                                              "end": 486
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 487,
                                                  "end": 489
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 490,
                                                      "end": 495
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-4",
                                                      "raw": "\"col-md-4\"",
                                                      "start": 496,
                                                      "end": 506
                                                    },
                                                    "start": 490,
                                                    "end": 506
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 486,
                                                "end": 507
                                              },
                                              "children": [
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                  ",
                                                  "raw": "\n                  ",
                                                  "start": 507,
                                                  "end": 526
                                                },
                                                {
                                                  "type": "JSXElement",
                                                  "openingElement": {
                                                    "type": "JSXOpeningElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 527,
                                                      "end": 528
                                                    },
                                                    "typeArguments": null,
                                                    "attributes": [
                                                      {
                                                        "type": "JSXAttribute",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "onClick$",
                                                          "start": 549,
                                                          "end": 557
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
                                                                      "start": 593,
                                                                      "end": 605
                                                                    },
                                                                    "property": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "value",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 606,
                                                                      "end": 611
                                                                    },
                                                                    "optional": false,
                                                                    "computed": false,
                                                                    "start": 593,
                                                                    "end": 611
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
                                                                                  "start": 639,
                                                                                  "end": 651
                                                                                },
                                                                                "property": {
                                                                                  "type": "Identifier",
                                                                                  "decorators": [],
                                                                                  "name": "value",
                                                                                  "optional": false,
                                                                                  "typeAnnotation": null,
                                                                                  "start": 652,
                                                                                  "end": 657
                                                                                },
                                                                                "optional": false,
                                                                                "computed": false,
                                                                                "start": 639,
                                                                                "end": 657
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "selected",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 658,
                                                                                "end": 666
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 639,
                                                                              "end": 666
                                                                            },
                                                                            "property": {
                                                                              "type": "Identifier",
                                                                              "decorators": [],
                                                                              "name": "value",
                                                                              "optional": false,
                                                                              "typeAnnotation": null,
                                                                              "start": 667,
                                                                              "end": 672
                                                                            },
                                                                            "optional": false,
                                                                            "computed": false,
                                                                            "start": 639,
                                                                            "end": 672
                                                                          },
                                                                          "right": {
                                                                            "type": "Literal",
                                                                            "value": false,
                                                                            "raw": "false",
                                                                            "start": 675,
                                                                            "end": 680
                                                                          },
                                                                          "start": 639,
                                                                          "end": 680
                                                                        },
                                                                        "directive": null,
                                                                        "start": 639,
                                                                        "end": 681
                                                                      }
                                                                    ],
                                                                    "start": 613,
                                                                    "end": 705
                                                                  },
                                                                  "alternate": null,
                                                                  "start": 589,
                                                                  "end": 705
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
                                                                        "start": 728,
                                                                        "end": 740
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 741,
                                                                        "end": 746
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 728,
                                                                      "end": 746
                                                                    },
                                                                    "right": {
                                                                      "type": "MemberExpression",
                                                                      "object": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "row",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 749,
                                                                        "end": 752
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 753,
                                                                        "end": 758
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 749,
                                                                      "end": 758
                                                                    },
                                                                    "start": 728,
                                                                    "end": 758
                                                                  },
                                                                  "directive": null,
                                                                  "start": 728,
                                                                  "end": 759
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
                                                                            "start": 782,
                                                                            "end": 785
                                                                          },
                                                                          "property": {
                                                                            "type": "Identifier",
                                                                            "decorators": [],
                                                                            "name": "value",
                                                                            "optional": false,
                                                                            "typeAnnotation": null,
                                                                            "start": 786,
                                                                            "end": 791
                                                                          },
                                                                          "optional": false,
                                                                          "computed": false,
                                                                          "start": 782,
                                                                          "end": 791
                                                                        },
                                                                        "property": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "selected",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 792,
                                                                          "end": 800
                                                                        },
                                                                        "optional": false,
                                                                        "computed": false,
                                                                        "start": 782,
                                                                        "end": 800
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 801,
                                                                        "end": 806
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 782,
                                                                      "end": 806
                                                                    },
                                                                    "right": {
                                                                      "type": "Literal",
                                                                      "value": true,
                                                                      "raw": "true",
                                                                      "start": 809,
                                                                      "end": 813
                                                                    },
                                                                    "start": 782,
                                                                    "end": 813
                                                                  },
                                                                  "directive": null,
                                                                  "start": 782,
                                                                  "end": 814
                                                                }
                                                              ],
                                                              "start": 565,
                                                              "end": 836
                                                            },
                                                            "id": null,
                                                            "generator": false,
                                                            "start": 559,
                                                            "end": 836
                                                          },
                                                          "start": 558,
                                                          "end": 837
                                                        },
                                                        "start": 549,
                                                        "end": 837
                                                      }
                                                    ],
                                                    "selfClosing": false,
                                                    "start": 526,
                                                    "end": 857
                                                  },
                                                  "children": [
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                    ",
                                                      "raw": "\n                    ",
                                                      "start": 857,
                                                      "end": 878
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
                                                              "start": 879,
                                                              "end": 882
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "value",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 883,
                                                              "end": 888
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 879,
                                                            "end": 888
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "label",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 889,
                                                            "end": 894
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 879,
                                                          "end": 894
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "value",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 895,
                                                          "end": 900
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 879,
                                                        "end": 900
                                                      },
                                                      "start": 878,
                                                      "end": 901
                                                    },
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                  ",
                                                      "raw": "\n                  ",
                                                      "start": 901,
                                                      "end": 920
                                                    }
                                                  ],
                                                  "closingElement": {
                                                    "type": "JSXClosingElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 922,
                                                      "end": 923
                                                    },
                                                    "start": 920,
                                                    "end": 924
                                                  },
                                                  "start": 526,
                                                  "end": 924
                                                },
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                ",
                                                  "raw": "\n                ",
                                                  "start": 924,
                                                  "end": 941
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 943,
                                                  "end": 945
                                                },
                                                "start": 941,
                                                "end": 946
                                              },
                                              "start": 486,
                                              "end": 946
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 946,
                                              "end": 963
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 964,
                                                  "end": 966
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 967,
                                                      "end": 972
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-1",
                                                      "raw": "\"col-md-1\"",
                                                      "start": 973,
                                                      "end": 983
                                                    },
                                                    "start": 967,
                                                    "end": 983
                                                  }
                                                ],
                                                "selfClosing": false,
                                                "start": 963,
                                                "end": 984
                                              },
                                              "children": [
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                  ",
                                                  "raw": "\n                  ",
                                                  "start": 984,
                                                  "end": 1003
                                                },
                                                {
                                                  "type": "JSXElement",
                                                  "openingElement": {
                                                    "type": "JSXOpeningElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 1004,
                                                      "end": 1005
                                                    },
                                                    "typeArguments": null,
                                                    "attributes": [
                                                      {
                                                        "type": "JSXAttribute",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "onClick$",
                                                          "start": 1026,
                                                          "end": 1034
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
                                                                        "start": 1072,
                                                                        "end": 1081
                                                                      },
                                                                      "init": {
                                                                        "type": "CallExpression",
                                                                        "callee": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "untrack",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 1084,
                                                                          "end": 1091
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
                                                                                "start": 1098,
                                                                                "end": 1102
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "value",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 1103,
                                                                                "end": 1108
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 1098,
                                                                              "end": 1108
                                                                            },
                                                                            "id": null,
                                                                            "generator": false,
                                                                            "start": 1092,
                                                                            "end": 1108
                                                                          }
                                                                        ],
                                                                        "optional": false,
                                                                        "start": 1084,
                                                                        "end": 1109
                                                                      },
                                                                      "definite": false,
                                                                      "start": 1072,
                                                                      "end": 1109
                                                                    }
                                                                  ],
                                                                  "declare": false,
                                                                  "start": 1066,
                                                                  "end": 1110
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
                                                                        "start": 1133,
                                                                        "end": 1137
                                                                      },
                                                                      "property": {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "value",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 1138,
                                                                        "end": 1143
                                                                      },
                                                                      "optional": false,
                                                                      "computed": false,
                                                                      "start": 1133,
                                                                      "end": 1143
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
                                                                          "start": 1146,
                                                                          "end": 1155
                                                                        },
                                                                        "property": {
                                                                          "type": "Identifier",
                                                                          "decorators": [],
                                                                          "name": "toSpliced",
                                                                          "optional": false,
                                                                          "typeAnnotation": null,
                                                                          "start": 1156,
                                                                          "end": 1165
                                                                        },
                                                                        "optional": false,
                                                                        "computed": false,
                                                                        "start": 1146,
                                                                        "end": 1165
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
                                                                              "start": 1191,
                                                                              "end": 1200
                                                                            },
                                                                            "property": {
                                                                              "type": "Identifier",
                                                                              "decorators": [],
                                                                              "name": "findIndex",
                                                                              "optional": false,
                                                                              "typeAnnotation": null,
                                                                              "start": 1201,
                                                                              "end": 1210
                                                                            },
                                                                            "optional": false,
                                                                            "computed": false,
                                                                            "start": 1191,
                                                                            "end": 1210
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
                                                                                  "start": 1212,
                                                                                  "end": 1213
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
                                                                                      "start": 1218,
                                                                                      "end": 1219
                                                                                    },
                                                                                    "property": {
                                                                                      "type": "Identifier",
                                                                                      "decorators": [],
                                                                                      "name": "value",
                                                                                      "optional": false,
                                                                                      "typeAnnotation": null,
                                                                                      "start": 1220,
                                                                                      "end": 1225
                                                                                    },
                                                                                    "optional": false,
                                                                                    "computed": false,
                                                                                    "start": 1218,
                                                                                    "end": 1225
                                                                                  },
                                                                                  "property": {
                                                                                    "type": "Identifier",
                                                                                    "decorators": [],
                                                                                    "name": "id",
                                                                                    "optional": false,
                                                                                    "typeAnnotation": null,
                                                                                    "start": 1226,
                                                                                    "end": 1228
                                                                                  },
                                                                                  "optional": false,
                                                                                  "computed": false,
                                                                                  "start": 1218,
                                                                                  "end": 1228
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
                                                                                      "start": 1233,
                                                                                      "end": 1236
                                                                                    },
                                                                                    "property": {
                                                                                      "type": "Identifier",
                                                                                      "decorators": [],
                                                                                      "name": "value",
                                                                                      "optional": false,
                                                                                      "typeAnnotation": null,
                                                                                      "start": 1237,
                                                                                      "end": 1242
                                                                                    },
                                                                                    "optional": false,
                                                                                    "computed": false,
                                                                                    "start": 1233,
                                                                                    "end": 1242
                                                                                  },
                                                                                  "property": {
                                                                                    "type": "Identifier",
                                                                                    "decorators": [],
                                                                                    "name": "id",
                                                                                    "optional": false,
                                                                                    "typeAnnotation": null,
                                                                                    "start": 1243,
                                                                                    "end": 1245
                                                                                  },
                                                                                  "optional": false,
                                                                                  "computed": false,
                                                                                  "start": 1233,
                                                                                  "end": 1245
                                                                                },
                                                                                "start": 1218,
                                                                                "end": 1245
                                                                              },
                                                                              "id": null,
                                                                              "generator": false,
                                                                              "start": 1211,
                                                                              "end": 1245
                                                                            }
                                                                          ],
                                                                          "optional": false,
                                                                          "start": 1191,
                                                                          "end": 1246
                                                                        },
                                                                        {
                                                                          "type": "Literal",
                                                                          "value": 1,
                                                                          "raw": "1",
                                                                          "start": 1272,
                                                                          "end": 1273
                                                                        }
                                                                      ],
                                                                      "optional": false,
                                                                      "start": 1146,
                                                                      "end": 1298
                                                                    },
                                                                    "start": 1133,
                                                                    "end": 1298
                                                                  },
                                                                  "directive": null,
                                                                  "start": 1133,
                                                                  "end": 1299
                                                                }
                                                              ],
                                                              "start": 1042,
                                                              "end": 1321
                                                            },
                                                            "id": null,
                                                            "generator": false,
                                                            "start": 1036,
                                                            "end": 1321
                                                          },
                                                          "start": 1035,
                                                          "end": 1322
                                                        },
                                                        "start": 1026,
                                                        "end": 1322
                                                      }
                                                    ],
                                                    "selfClosing": false,
                                                    "start": 1003,
                                                    "end": 1342
                                                  },
                                                  "children": [
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                    ",
                                                      "raw": "\n                    ",
                                                      "start": 1342,
                                                      "end": 1363
                                                    },
                                                    {
                                                      "type": "JSXElement",
                                                      "openingElement": {
                                                        "type": "JSXOpeningElement",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "span",
                                                          "start": 1364,
                                                          "end": 1368
                                                        },
                                                        "typeArguments": null,
                                                        "attributes": [
                                                          {
                                                            "type": "JSXAttribute",
                                                            "name": {
                                                              "type": "JSXIdentifier",
                                                              "name": "aria-hidden",
                                                              "start": 1369,
                                                              "end": 1380
                                                            },
                                                            "value": {
                                                              "type": "Literal",
                                                              "value": "true",
                                                              "raw": "\"true\"",
                                                              "start": 1381,
                                                              "end": 1387
                                                            },
                                                            "start": 1369,
                                                            "end": 1387
                                                          }
                                                        ],
                                                        "selfClosing": false,
                                                        "start": 1363,
                                                        "end": 1388
                                                      },
                                                      "children": [
                                                        {
                                                          "type": "JSXText",
                                                          "value": "x",
                                                          "raw": "x",
                                                          "start": 1388,
                                                          "end": 1389
                                                        }
                                                      ],
                                                      "closingElement": {
                                                        "type": "JSXClosingElement",
                                                        "name": {
                                                          "type": "JSXIdentifier",
                                                          "name": "span",
                                                          "start": 1391,
                                                          "end": 1395
                                                        },
                                                        "start": 1389,
                                                        "end": 1396
                                                      },
                                                      "start": 1363,
                                                      "end": 1396
                                                    },
                                                    {
                                                      "type": "JSXText",
                                                      "value": "\n                  ",
                                                      "raw": "\n                  ",
                                                      "start": 1396,
                                                      "end": 1415
                                                    }
                                                  ],
                                                  "closingElement": {
                                                    "type": "JSXClosingElement",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "a",
                                                      "start": 1417,
                                                      "end": 1418
                                                    },
                                                    "start": 1415,
                                                    "end": 1419
                                                  },
                                                  "start": 1003,
                                                  "end": 1419
                                                },
                                                {
                                                  "type": "JSXText",
                                                  "value": "\n                ",
                                                  "raw": "\n                ",
                                                  "start": 1419,
                                                  "end": 1436
                                                }
                                              ],
                                              "closingElement": {
                                                "type": "JSXClosingElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1438,
                                                  "end": 1440
                                                },
                                                "start": 1436,
                                                "end": 1441
                                              },
                                              "start": 963,
                                              "end": 1441
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n                ",
                                              "raw": "\n                ",
                                              "start": 1441,
                                              "end": 1458
                                            },
                                            {
                                              "type": "JSXElement",
                                              "openingElement": {
                                                "type": "JSXOpeningElement",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "td",
                                                  "start": 1459,
                                                  "end": 1461
                                                },
                                                "typeArguments": null,
                                                "attributes": [
                                                  {
                                                    "type": "JSXAttribute",
                                                    "name": {
                                                      "type": "JSXIdentifier",
                                                      "name": "class",
                                                      "start": 1462,
                                                      "end": 1467
                                                    },
                                                    "value": {
                                                      "type": "Literal",
                                                      "value": "col-md-6",
                                                      "raw": "\"col-md-6\"",
                                                      "start": 1468,
                                                      "end": 1478
                                                    },
                                                    "start": 1462,
                                                    "end": 1478
                                                  }
                                                ],
                                                "selfClosing": true,
                                                "start": 1458,
                                                "end": 1481
                                              },
                                              "children": [],
                                              "closingElement": null,
                                              "start": 1458,
                                              "end": 1481
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n              ",
                                              "raw": "\n              ",
                                              "start": 1481,
                                              "end": 1496
                                            }
                                          ],
                                          "closingElement": {
                                            "type": "JSXClosingElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "tr",
                                              "start": 1498,
                                              "end": 1500
                                            },
                                            "start": 1496,
                                            "end": 1501
                                          },
                                          "start": 278,
                                          "end": 1501
                                        },
                                        "start": 262,
                                        "end": 1513
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 253,
                                      "end": 1513
                                    }
                                  ],
                                  "optional": false,
                                  "start": 238,
                                  "end": 1514
                                },
                                "start": 237,
                                "end": 1515
                              },
                              {
                                "type": "JSXText",
                                "value": "\n        ",
                                "raw": "\n        ",
                                "start": 1515,
                                "end": 1524
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 1526,
                                "end": 1529
                              },
                              "start": 1524,
                              "end": 1530
                            },
                            "start": 221,
                            "end": 1530
                          },
                          "start": 211,
                          "end": 1538
                        },
                        "start": 204,
                        "end": 1539
                      }
                    ],
                    "start": 103,
                    "end": 1545
                  },
                  "id": null,
                  "generator": false,
                  "start": 97,
                  "end": 1545
                }
              ],
              "optional": false,
              "start": 86,
              "end": 1546
            },
            "definite": false,
            "start": 80,
            "end": 1546
          }
        ],
        "declare": false,
        "start": 74,
        "end": 1547
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 67,
      "end": 1547
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1547
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
    1038,
    1323
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
    const App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg = /*#__PURE__*/ qrl(i_lgbZkJXyLtg, "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg", [
        selectedItem
    ]);
    const App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48 = /*#__PURE__*/ qrl(i_40fnSAlYI48, "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48", [
        data
    ]);
    return /*#__PURE__*/ _jsxSorted("div", null, null, data.value.map((row)=>/*#__PURE__*/ _jsxSorted("tr", {
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
                "q:p": row
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
                          "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                          "start": 758,
                          "end": 805
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 822,
                            "end": 825
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_lgbZkJXyLtg",
                              "start": 826,
                              "end": 839
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                              "raw": "\"App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg\"",
                              "start": 841,
                              "end": 890
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "selectedItem",
                                  "start": 902,
                                  "end": 914
                                }
                              ],
                              "start": 892,
                              "end": 920
                            }
                          ],
                          "optional": false,
                          "start": 822,
                          "end": 921
                        },
                        "start": 758,
                        "end": 921
                      }
                    ],
                    "start": 752,
                    "end": 922
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
                          "start": 933,
                          "end": 982
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 999,
                            "end": 1002
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_40fnSAlYI48",
                              "start": 1003,
                              "end": 1016
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                              "raw": "\"App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48\"",
                              "start": 1018,
                              "end": 1069
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "data",
                                  "start": 1081,
                                  "end": 1085
                                }
                              ],
                              "start": 1071,
                              "end": 1091
                            }
                          ],
                          "optional": false,
                          "start": 999,
                          "end": 1092
                        },
                        "start": 933,
                        "end": 1092
                      }
                    ],
                    "start": 927,
                    "end": 1093
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 1119,
                        "end": 1129
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 1130,
                          "end": 1135
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1137,
                          "end": 1141
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1143,
                          "end": 1147
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
                                "start": 1149,
                                "end": 1153
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 1154,
                                "end": 1159
                              },
                              "optional": false,
                              "computed": false,
                              "start": 1149,
                              "end": 1159
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "map",
                              "start": 1160,
                              "end": 1163
                            },
                            "optional": false,
                            "computed": false,
                            "start": 1149,
                            "end": 1163
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
                                  "start": 1165,
                                  "end": 1168
                                }
                              ],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 1185,
                                  "end": 1195
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "tr",
                                    "raw": "\"tr\"",
                                    "start": 1196,
                                    "end": 1200
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
                                          "start": 1216,
                                          "end": 1221
                                        },
                                        "value": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "name": "_fnSignal",
                                            "start": 1223,
                                            "end": 1232
                                          },
                                          "arguments": [
                                            {
                                              "type": "Identifier",
                                              "name": "_hf0",
                                              "start": 1233,
                                              "end": 1237
                                            },
                                            {
                                              "type": "ArrayExpression",
                                              "elements": [
                                                {
                                                  "type": "Identifier",
                                                  "name": "row",
                                                  "start": 1257,
                                                  "end": 1260
                                                }
                                              ],
                                              "start": 1239,
                                              "end": 1274
                                            },
                                            {
                                              "type": "Identifier",
                                              "name": "_hf0_str",
                                              "start": 1276,
                                              "end": 1284
                                            }
                                          ],
                                          "optional": false,
                                          "start": 1223,
                                          "end": 1285
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 1216,
                                        "end": 1285
                                      }
                                    ],
                                    "start": 1202,
                                    "end": 1295
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 1297,
                                    "end": 1301
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1331,
                                          "end": 1341
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1342,
                                            "end": 1346
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1348,
                                            "end": 1352
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
                                                  "start": 1372,
                                                  "end": 1377
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-1",
                                                  "raw": "\"col-md-1\"",
                                                  "start": 1379,
                                                  "end": 1389
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 1372,
                                                "end": 1389
                                              }
                                            ],
                                            "start": 1354,
                                            "end": 1403
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_fnSignal",
                                              "start": 1405,
                                              "end": 1414
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "_hf1",
                                                "start": 1415,
                                                "end": 1419
                                              },
                                              {
                                                "type": "ArrayExpression",
                                                "elements": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "row",
                                                    "start": 1439,
                                                    "end": 1442
                                                  }
                                                ],
                                                "start": 1421,
                                                "end": 1456
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "_hf1_str",
                                                "start": 1458,
                                                "end": 1466
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1405,
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
                                            "value": null,
                                            "raw": "null",
                                            "start": 1472,
                                            "end": 1476
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1331,
                                        "end": 1477
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1505,
                                          "end": 1515
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1516,
                                            "end": 1520
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1522,
                                            "end": 1526
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
                                                  "start": 1546,
                                                  "end": 1551
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-4",
                                                  "raw": "\"col-md-4\"",
                                                  "start": 1553,
                                                  "end": 1563
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 1546,
                                                "end": 1563
                                              }
                                            ],
                                            "start": 1528,
                                            "end": 1577
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_jsxSorted",
                                              "start": 1593,
                                              "end": 1603
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "a",
                                                "raw": "\"a\"",
                                                "start": 1604,
                                                "end": 1607
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
                                                      "start": 1627,
                                                      "end": 1638
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg",
                                                      "start": 1640,
                                                      "end": 1687
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 1627,
                                                    "end": 1687
                                                  },
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Literal",
                                                      "value": "q:p",
                                                      "raw": "\"q:p\"",
                                                      "start": 1705,
                                                      "end": 1710
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "row",
                                                      "start": 1712,
                                                      "end": 1715
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 1705,
                                                    "end": 1715
                                                  }
                                                ],
                                                "start": 1609,
                                                "end": 1729
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1731,
                                                "end": 1735
                                              },
                                              {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "_fnSignal",
                                                  "start": 1737,
                                                  "end": 1746
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_hf2",
                                                    "start": 1747,
                                                    "end": 1751
                                                  },
                                                  {
                                                    "type": "ArrayExpression",
                                                    "elements": [
                                                      {
                                                        "type": "Identifier",
                                                        "name": "row",
                                                        "start": 1771,
                                                        "end": 1774
                                                      }
                                                    ],
                                                    "start": 1753,
                                                    "end": 1788
                                                  },
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_hf2_str",
                                                    "start": 1790,
                                                    "end": 1798
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 1737,
                                                "end": 1799
                                              },
                                              {
                                                "type": "Literal",
                                                "value": 0,
                                                "raw": "0",
                                                "start": 1801,
                                                "end": 1802
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 1804,
                                                "end": 1808
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1593,
                                            "end": 1809
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 1811,
                                            "end": 1812
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1814,
                                            "end": 1818
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1505,
                                        "end": 1819
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 1847,
                                          "end": 1857
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 1858,
                                            "end": 1862
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 1864,
                                            "end": 1868
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
                                                  "start": 1888,
                                                  "end": 1893
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-1",
                                                  "raw": "\"col-md-1\"",
                                                  "start": 1895,
                                                  "end": 1905
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 1888,
                                                "end": 1905
                                              }
                                            ],
                                            "start": 1870,
                                            "end": 1919
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_jsxSorted",
                                              "start": 1935,
                                              "end": 1945
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "a",
                                                "raw": "\"a\"",
                                                "start": 1946,
                                                "end": 1949
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
                                                      "start": 1969,
                                                      "end": 1980
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48",
                                                      "start": 1982,
                                                      "end": 2031
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 1969,
                                                    "end": 2031
                                                  },
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Literal",
                                                      "value": "q:p",
                                                      "raw": "\"q:p\"",
                                                      "start": 2049,
                                                      "end": 2054
                                                    },
                                                    "value": {
                                                      "type": "Identifier",
                                                      "name": "row",
                                                      "start": 2056,
                                                      "end": 2059
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 2049,
                                                    "end": 2059
                                                  }
                                                ],
                                                "start": 1951,
                                                "end": 2073
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 2075,
                                                "end": 2079
                                              },
                                              {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "_jsxSorted",
                                                  "start": 2095,
                                                  "end": 2105
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Literal",
                                                    "value": "span",
                                                    "raw": "\"span\"",
                                                    "start": 2106,
                                                    "end": 2112
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": null,
                                                    "raw": "null",
                                                    "start": 2114,
                                                    "end": 2118
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
                                                          "start": 2138,
                                                          "end": 2151
                                                        },
                                                        "value": {
                                                          "type": "Literal",
                                                          "value": "true",
                                                          "raw": "\"true\"",
                                                          "start": 2153,
                                                          "end": 2159
                                                        },
                                                        "method": false,
                                                        "shorthand": false,
                                                        "computed": false,
                                                        "start": 2138,
                                                        "end": 2159
                                                      }
                                                    ],
                                                    "start": 2120,
                                                    "end": 2173
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": "x",
                                                    "raw": "\"x\"",
                                                    "start": 2175,
                                                    "end": 2178
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": 3,
                                                    "raw": "3",
                                                    "start": 2180,
                                                    "end": 2181
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": null,
                                                    "raw": "null",
                                                    "start": 2183,
                                                    "end": 2187
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 2095,
                                                "end": 2188
                                              },
                                              {
                                                "type": "Literal",
                                                "value": 2,
                                                "raw": "2",
                                                "start": 2190,
                                                "end": 2191
                                              },
                                              {
                                                "type": "Literal",
                                                "value": null,
                                                "raw": "null",
                                                "start": 2193,
                                                "end": 2197
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1935,
                                            "end": 2198
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 1,
                                            "raw": "1",
                                            "start": 2200,
                                            "end": 2201
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2203,
                                            "end": 2207
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1847,
                                        "end": 2208
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_jsxSorted",
                                          "start": 2236,
                                          "end": 2246
                                        },
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "td",
                                            "raw": "\"td\"",
                                            "start": 2247,
                                            "end": 2251
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2253,
                                            "end": 2257
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
                                                  "start": 2277,
                                                  "end": 2282
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": "col-md-6",
                                                  "raw": "\"col-md-6\"",
                                                  "start": 2284,
                                                  "end": 2294
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 2277,
                                                "end": 2294
                                              }
                                            ],
                                            "start": 2259,
                                            "end": 2308
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2310,
                                            "end": 2314
                                          },
                                          {
                                            "type": "Literal",
                                            "value": 3,
                                            "raw": "3",
                                            "start": 2316,
                                            "end": 2317
                                          },
                                          {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 2319,
                                            "end": 2323
                                          }
                                        ],
                                        "optional": false,
                                        "start": 2236,
                                        "end": 2324
                                      }
                                    ],
                                    "start": 1303,
                                    "end": 2334
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 1,
                                    "raw": "1",
                                    "start": 2336,
                                    "end": 2337
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "untrack",
                                      "start": 2339,
                                      "end": 2346
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
                                              "start": 2351,
                                              "end": 2354
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "name": "value",
                                              "start": 2355,
                                              "end": 2360
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 2351,
                                            "end": 2360
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "id",
                                            "start": 2361,
                                            "end": 2363
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 2351,
                                          "end": 2363
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 2347,
                                        "end": 2363
                                      }
                                    ],
                                    "optional": false,
                                    "start": 2339,
                                    "end": 2364
                                  }
                                ],
                                "optional": false,
                                "start": 1185,
                                "end": 2365
                              },
                              "id": null,
                              "generator": false,
                              "start": 1164,
                              "end": 2365
                            }
                          ],
                          "optional": false,
                          "start": 1149,
                          "end": 2366
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 2368,
                          "end": 2369
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 2371,
                          "end": 2377
                        }
                      ],
                      "optional": false,
                      "start": 1119,
                      "end": 2378
                    },
                    "start": 1098,
                    "end": 2379
                  }
                ],
                "start": 672,
                "end": 2381
              },
              "id": null,
              "generator": false,
              "start": 668,
              "end": 2381
            },
            "start": 640,
            "end": 2381
          }
        ],
        "start": 634,
        "end": 2382
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 627,
      "end": 2382
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 2382
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
    1547
  ]
}
```

### Module: `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg = (_, _1, row)=>{
    const selectedItem = _captures[0];
    if (selectedItem.value) selectedItem.value.selected.value = false;
    selectedItem.value = row.value;
    row.value.selected.value = true;
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
                          "name": "selectedItem",
                          "start": 133,
                          "end": 145
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 148,
                            "end": 157
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 158,
                            "end": 159
                          },
                          "optional": false,
                          "computed": true,
                          "start": 148,
                          "end": 160
                        },
                        "start": 133,
                        "end": 160
                      }
                    ],
                    "start": 127,
                    "end": 161
                  },
                  {
                    "type": "IfStatement",
                    "test": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "selectedItem",
                        "start": 170,
                        "end": 182
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 183,
                        "end": 188
                      },
                      "optional": false,
                      "computed": false,
                      "start": 170,
                      "end": 188
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
                                "start": 190,
                                "end": 202
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 203,
                                "end": 208
                              },
                              "optional": false,
                              "computed": false,
                              "start": 190,
                              "end": 208
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "selected",
                              "start": 209,
                              "end": 217
                            },
                            "optional": false,
                            "computed": false,
                            "start": 190,
                            "end": 217
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "value",
                            "start": 218,
                            "end": 223
                          },
                          "optional": false,
                          "computed": false,
                          "start": 190,
                          "end": 223
                        },
                        "right": {
                          "type": "Literal",
                          "value": false,
                          "raw": "false",
                          "start": 226,
                          "end": 231
                        },
                        "start": 190,
                        "end": 231
                      },
                      "start": 190,
                      "end": 232
                    },
                    "alternate": null,
                    "start": 166,
                    "end": 232
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
                          "start": 237,
                          "end": 249
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 250,
                          "end": 255
                        },
                        "optional": false,
                        "computed": false,
                        "start": 237,
                        "end": 255
                      },
                      "right": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "row",
                          "start": 258,
                          "end": 261
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 262,
                          "end": 267
                        },
                        "optional": false,
                        "computed": false,
                        "start": 258,
                        "end": 267
                      },
                      "start": 237,
                      "end": 267
                    },
                    "start": 237,
                    "end": 268
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
                              "start": 273,
                              "end": 276
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "value",
                              "start": 277,
                              "end": 282
                            },
                            "optional": false,
                            "computed": false,
                            "start": 273,
                            "end": 282
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "selected",
                            "start": 283,
                            "end": 291
                          },
                          "optional": false,
                          "computed": false,
                          "start": 273,
                          "end": 291
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 292,
                          "end": 297
                        },
                        "optional": false,
                        "computed": false,
                        "start": 273,
                        "end": 297
                      },
                      "right": {
                        "type": "Literal",
                        "value": true,
                        "raw": "true",
                        "start": 300,
                        "end": 304
                      },
                      "start": 273,
                      "end": 304
                    },
                    "start": 273,
                    "end": 305
                  }
                ],
                "start": 121,
                "end": 307
              },
              "id": null,
              "generator": false,
              "start": 107,
              "end": 307
            },
            "start": 57,
            "end": 307
          }
        ],
        "start": 51,
        "end": 308
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 308
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 308
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
    561,
    838
  ],
  "paramNames": [
    "_",
    "_",
    "row"
  ],
  "captureNames": [
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
| `useSignal` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 3 |
| `_captures` | `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
