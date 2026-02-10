# Test: should_convert_jsx_events

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

		const ManyEventsComponent = component$(() => {
			return (
				<div>
					<button
						onClick$={() => {}}
						onDblClick$={() => {}}
					>
						click
					</button>
					<button
						onClick$={() => {}}
						onBlur$={() => {}}
						on-anotherCustom$={() => {}}
						document:onFocus$={() => {}}
						window:onClick$={() => {}}
					>
						click
					</button>
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 44
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
            "name": "ManyEventsComponent",
            "optional": false,
            "typeAnnotation": null,
            "start": 54,
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
                              "start": 112,
                              "end": 115
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 111,
                            "end": 116
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t\t",
                              "raw": "\n\t\t\t\t\t",
                              "start": 116,
                              "end": 122
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 123,
                                  "end": 129
                                },
                                "typeArguments": null,
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "onClick$",
                                      "start": 136,
                                      "end": 144
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
                                          "body": [],
                                          "start": 152,
                                          "end": 154
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 146,
                                        "end": 154
                                      },
                                      "start": 145,
                                      "end": 155
                                    },
                                    "start": 136,
                                    "end": 155
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "onDblClick$",
                                      "start": 162,
                                      "end": 173
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
                                          "body": [],
                                          "start": 181,
                                          "end": 183
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 175,
                                        "end": 183
                                      },
                                      "start": 174,
                                      "end": 184
                                    },
                                    "start": 162,
                                    "end": 184
                                  }
                                ],
                                "selfClosing": false,
                                "start": 122,
                                "end": 191
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\tclick\n\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\tclick\n\t\t\t\t\t",
                                  "start": 191,
                                  "end": 209
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 211,
                                  "end": 217
                                },
                                "start": 209,
                                "end": 218
                              },
                              "start": 122,
                              "end": 218
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t\t",
                              "raw": "\n\t\t\t\t\t",
                              "start": 218,
                              "end": 224
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 225,
                                  "end": 231
                                },
                                "typeArguments": null,
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "onClick$",
                                      "start": 238,
                                      "end": 246
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
                                          "body": [],
                                          "start": 254,
                                          "end": 256
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 248,
                                        "end": 256
                                      },
                                      "start": 247,
                                      "end": 257
                                    },
                                    "start": 238,
                                    "end": 257
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "onBlur$",
                                      "start": 264,
                                      "end": 271
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
                                          "body": [],
                                          "start": 279,
                                          "end": 281
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 273,
                                        "end": 281
                                      },
                                      "start": 272,
                                      "end": 282
                                    },
                                    "start": 264,
                                    "end": 282
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "on-anotherCustom$",
                                      "start": 289,
                                      "end": 306
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
                                          "body": [],
                                          "start": 314,
                                          "end": 316
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 308,
                                        "end": 316
                                      },
                                      "start": 307,
                                      "end": 317
                                    },
                                    "start": 289,
                                    "end": 317
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXNamespacedName",
                                      "namespace": {
                                        "type": "JSXIdentifier",
                                        "name": "document",
                                        "start": 324,
                                        "end": 332
                                      },
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "onFocus$",
                                        "start": 333,
                                        "end": 341
                                      },
                                      "start": 324,
                                      "end": 341
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
                                          "body": [],
                                          "start": 349,
                                          "end": 351
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 343,
                                        "end": 351
                                      },
                                      "start": 342,
                                      "end": 352
                                    },
                                    "start": 324,
                                    "end": 352
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXNamespacedName",
                                      "namespace": {
                                        "type": "JSXIdentifier",
                                        "name": "window",
                                        "start": 359,
                                        "end": 365
                                      },
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "onClick$",
                                        "start": 366,
                                        "end": 374
                                      },
                                      "start": 359,
                                      "end": 374
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
                                          "body": [],
                                          "start": 382,
                                          "end": 384
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 376,
                                        "end": 384
                                      },
                                      "start": 375,
                                      "end": 385
                                    },
                                    "start": 359,
                                    "end": 385
                                  }
                                ],
                                "selfClosing": false,
                                "start": 224,
                                "end": 392
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\tclick\n\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\tclick\n\t\t\t\t\t",
                                  "start": 392,
                                  "end": 410
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "button",
                                  "start": 412,
                                  "end": 418
                                },
                                "start": 410,
                                "end": 419
                              },
                              "start": 224,
                              "end": 419
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t",
                              "raw": "\n\t\t\t\t",
                              "start": 419,
                              "end": 424
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 426,
                              "end": 429
                            },
                            "start": 424,
                            "end": 430
                          },
                          "start": 111,
                          "end": 430
                        },
                        "start": 105,
                        "end": 435
                      },
                      "start": 98,
                      "end": 436
                    }
                  ],
                  "start": 93,
                  "end": 440
                },
                "id": null,
                "generator": false,
                "start": 87,
                "end": 440
              }
            ],
            "optional": false,
            "start": 76,
            "end": 441
          },
          "definite": false,
          "start": 54,
          "end": 441
        }
      ],
      "declare": false,
      "start": 48,
      "end": 442
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 442
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ndbrzxiyG3g = ()=>import("./test.tsx_ManyEventsComponent_component_ndbrzxiyG3g");
/*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ndbrzxiyG3g, "ManyEventsComponent_component_ndbrzxiyG3g"));
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
            "name": "i_ndbrzxiyG3g",
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
                "value": "./test.tsx_ManyEventsComponent_component_ndbrzxiyG3g",
                "raw": "\"./test.tsx_ManyEventsComponent_component_ndbrzxiyG3g\"",
                "start": 118,
                "end": 172
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 173
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 173
          },
          "start": 91,
          "end": 173
        }
      ],
      "start": 85,
      "end": 174
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 189,
          "end": 201
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 216,
              "end": 219
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_ndbrzxiyG3g",
                "start": 220,
                "end": 233
              },
              {
                "type": "Literal",
                "value": "ManyEventsComponent_component_ndbrzxiyG3g",
                "raw": "\"ManyEventsComponent_component_ndbrzxiyG3g\"",
                "start": 235,
                "end": 278
              }
            ],
            "optional": false,
            "start": 216,
            "end": 279
          }
        ],
        "optional": false,
        "start": 189,
        "end": 280
      },
      "start": 189,
      "end": 281
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 281
}
```

</details>

### Module: test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA",
              "start": 13,
              "end": 77
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 84,
                "end": 86
              },
              "id": null,
              "generator": false,
              "start": 80,
              "end": 86
            },
            "start": 13,
            "end": 86
          }
        ],
        "start": 7,
        "end": 87
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 87
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 87
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_e_click_1",
  "hash": "BOulU3QpiyA",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    252,
    260
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs",
              "start": 13,
              "end": 75
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 82,
                "end": 84
              },
              "id": null,
              "generator": false,
              "start": 78,
              "end": 84
            },
            "start": 13,
            "end": 84
          }
        ],
        "start": 7,
        "end": 85
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 85
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 85
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_d_focus",
  "hash": "fR03UCeyKSs",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "document:onFocus$",
  "captures": false,
  "loc": [
    347,
    355
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY",
              "start": 13,
              "end": 74
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 81,
                "end": 83
              },
              "id": null,
              "generator": false,
              "start": 77,
              "end": 83
            },
            "start": 13,
            "end": 83
          }
        ],
        "start": 7,
        "end": 84
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 84
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 84
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_e_blur",
  "hash": "LVmlrV6TsTY",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "onBlur$",
  "captures": false,
  "loc": [
    277,
    285
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_ndbrzxiyG3g.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_BOulU3QpiyA = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA");
const i_DjpgJ0bcgJs = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs");
const i_LVmlrV6TsTY = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY");
const i_VZGqMEVFles = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles");
const i_ZC5Gt2v0qH0 = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0");
const i_fR03UCeyKSs = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs");
const i_z0X12CPQocg = ()=>import("./test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg");
export const ManyEventsComponent_component_ndbrzxiyG3g = ()=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_z0X12CPQocg, "ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg"),
            "q-e:dblclick": /*#__PURE__*/ qrl(i_VZGqMEVFles, "ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles")
        }, "click", 3, null),
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_BOulU3QpiyA, "ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA"),
            "q-e:blur": /*#__PURE__*/ qrl(i_LVmlrV6TsTY, "ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY"),
            "q-e:another-custom": /*#__PURE__*/ qrl(i_ZC5Gt2v0qH0, "ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0"),
            "q-d:focus": /*#__PURE__*/ qrl(i_fR03UCeyKSs, "ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs"),
            "q-w:click": /*#__PURE__*/ qrl(i_DjpgJ0bcgJs, "ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs")
        }, "click", 3, null)
    ], 3, "u6_0");
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
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_BOulU3QpiyA",
            "start": 89,
            "end": 102
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA\"",
                "start": 116,
                "end": 193
              },
              "options": null,
              "phase": null,
              "start": 109,
              "end": 194
            },
            "id": null,
            "generator": false,
            "start": 105,
            "end": 194
          },
          "start": 89,
          "end": 194
        }
      ],
      "start": 83,
      "end": 195
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_DjpgJ0bcgJs",
            "start": 202,
            "end": 215
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs\"",
                "start": 229,
                "end": 304
              },
              "options": null,
              "phase": null,
              "start": 222,
              "end": 305
            },
            "id": null,
            "generator": false,
            "start": 218,
            "end": 305
          },
          "start": 202,
          "end": 305
        }
      ],
      "start": 196,
      "end": 306
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_LVmlrV6TsTY",
            "start": 313,
            "end": 326
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY\"",
                "start": 340,
                "end": 414
              },
              "options": null,
              "phase": null,
              "start": 333,
              "end": 415
            },
            "id": null,
            "generator": false,
            "start": 329,
            "end": 415
          },
          "start": 313,
          "end": 415
        }
      ],
      "start": 307,
      "end": 416
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_VZGqMEVFles",
            "start": 423,
            "end": 436
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles\"",
                "start": 450,
                "end": 528
              },
              "options": null,
              "phase": null,
              "start": 443,
              "end": 529
            },
            "id": null,
            "generator": false,
            "start": 439,
            "end": 529
          },
          "start": 423,
          "end": 529
        }
      ],
      "start": 417,
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
            "name": "i_ZC5Gt2v0qH0",
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0\"",
                "start": 564,
                "end": 648
              },
              "options": null,
              "phase": null,
              "start": 557,
              "end": 649
            },
            "id": null,
            "generator": false,
            "start": 553,
            "end": 649
          },
          "start": 537,
          "end": 649
        }
      ],
      "start": 531,
      "end": 650
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_fR03UCeyKSs",
            "start": 657,
            "end": 670
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs\"",
                "start": 684,
                "end": 759
              },
              "options": null,
              "phase": null,
              "start": 677,
              "end": 760
            },
            "id": null,
            "generator": false,
            "start": 673,
            "end": 760
          },
          "start": 657,
          "end": 760
        }
      ],
      "start": 651,
      "end": 761
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_z0X12CPQocg",
            "start": 768,
            "end": 781
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
                "value": "./test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg",
                "raw": "\"./test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg\"",
                "start": 795,
                "end": 870
              },
              "options": null,
              "phase": null,
              "start": 788,
              "end": 871
            },
            "id": null,
            "generator": false,
            "start": 784,
            "end": 871
          },
          "start": 768,
          "end": 871
        }
      ],
      "start": 762,
      "end": 872
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
              "name": "ManyEventsComponent_component_ndbrzxiyG3g",
              "start": 886,
              "end": 927
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
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 961,
                        "end": 971
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 972,
                          "end": 977
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 979,
                          "end": 983
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 985,
                          "end": 989
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1015,
                                "end": 1025
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "button",
                                  "raw": "\"button\"",
                                  "start": 1026,
                                  "end": 1034
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1036,
                                  "end": 1040
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
                                        "start": 1056,
                                        "end": 1067
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1083,
                                          "end": 1086
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_z0X12CPQocg",
                                            "start": 1087,
                                            "end": 1100
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg\"",
                                            "start": 1102,
                                            "end": 1166
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1083,
                                        "end": 1167
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1056,
                                      "end": 1167
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-e:dblclick",
                                        "raw": "\"q-e:dblclick\"",
                                        "start": 1181,
                                        "end": 1195
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1211,
                                          "end": 1214
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_VZGqMEVFles",
                                            "start": 1215,
                                            "end": 1228
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles\"",
                                            "start": 1230,
                                            "end": 1297
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1211,
                                        "end": 1298
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1181,
                                      "end": 1298
                                    }
                                  ],
                                  "start": 1042,
                                  "end": 1308
                                },
                                {
                                  "type": "Literal",
                                  "value": "click",
                                  "raw": "\"click\"",
                                  "start": 1310,
                                  "end": 1317
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1319,
                                  "end": 1320
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1322,
                                  "end": 1326
                                }
                              ],
                              "optional": false,
                              "start": 1015,
                              "end": 1327
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1351,
                                "end": 1361
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "button",
                                  "raw": "\"button\"",
                                  "start": 1362,
                                  "end": 1370
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1372,
                                  "end": 1376
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
                                        "start": 1392,
                                        "end": 1403
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1419,
                                          "end": 1422
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_BOulU3QpiyA",
                                            "start": 1423,
                                            "end": 1436
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA\"",
                                            "start": 1438,
                                            "end": 1504
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1419,
                                        "end": 1505
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1392,
                                      "end": 1505
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-e:blur",
                                        "raw": "\"q-e:blur\"",
                                        "start": 1519,
                                        "end": 1529
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1545,
                                          "end": 1548
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_LVmlrV6TsTY",
                                            "start": 1549,
                                            "end": 1562
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_e_blur_LVmlrV6TsTY\"",
                                            "start": 1564,
                                            "end": 1627
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1545,
                                        "end": 1628
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1519,
                                      "end": 1628
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-e:another-custom",
                                        "raw": "\"q-e:another-custom\"",
                                        "start": 1642,
                                        "end": 1662
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1678,
                                          "end": 1681
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_ZC5Gt2v0qH0",
                                            "start": 1682,
                                            "end": 1695
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0\"",
                                            "start": 1697,
                                            "end": 1770
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1678,
                                        "end": 1771
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1642,
                                      "end": 1771
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-d:focus",
                                        "raw": "\"q-d:focus\"",
                                        "start": 1785,
                                        "end": 1796
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1812,
                                          "end": 1815
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_fR03UCeyKSs",
                                            "start": 1816,
                                            "end": 1829
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_d_focus_fR03UCeyKSs\"",
                                            "start": 1831,
                                            "end": 1895
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1812,
                                        "end": 1896
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1785,
                                      "end": 1896
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "q-w:click",
                                        "raw": "\"q-w:click\"",
                                        "start": 1910,
                                        "end": 1921
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 1937,
                                          "end": 1940
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_DjpgJ0bcgJs",
                                            "start": 1941,
                                            "end": 1954
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs",
                                            "raw": "\"ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs\"",
                                            "start": 1956,
                                            "end": 2020
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1937,
                                        "end": 2021
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1910,
                                      "end": 2021
                                    }
                                  ],
                                  "start": 1378,
                                  "end": 2031
                                },
                                {
                                  "type": "Literal",
                                  "value": "click",
                                  "raw": "\"click\"",
                                  "start": 2033,
                                  "end": 2040
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 2042,
                                  "end": 2043
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 2045,
                                  "end": 2049
                                }
                              ],
                              "optional": false,
                              "start": 1351,
                              "end": 2050
                            }
                          ],
                          "start": 991,
                          "end": 2056
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 2058,
                          "end": 2059
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 2061,
                          "end": 2067
                        }
                      ],
                      "optional": false,
                      "start": 961,
                      "end": 2068
                    },
                    "start": 940,
                    "end": 2069
                  }
                ],
                "start": 934,
                "end": 2071
              },
              "id": null,
              "generator": false,
              "start": 930,
              "end": 2071
            },
            "start": 886,
            "end": 2071
          }
        ],
        "start": 880,
        "end": 2072
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 873,
      "end": 2072
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 2072
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_ndbrzxiyG3g",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component",
  "hash": "ndbrzxiyG3g",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_ndbrzxiyG3g",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    91,
    444
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0 = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0",
              "start": 13,
              "end": 84
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 91,
                "end": 93
              },
              "id": null,
              "generator": false,
              "start": 87,
              "end": 93
            },
            "start": 13,
            "end": 93
          }
        ],
        "start": 7,
        "end": 94
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 94
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 94
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom",
  "hash": "ZC5Gt2v0qH0",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_e_another_custom_ZC5Gt2v0qH0",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "on-anotherCustom$",
  "captures": false,
  "loc": [
    312,
    320
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs",
              "start": 13,
              "end": 75
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 82,
                "end": 84
              },
              "id": null,
              "generator": false,
              "start": 78,
              "end": 84
            },
            "start": 13,
            "end": 84
          }
        ],
        "start": 7,
        "end": 85
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 85
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 85
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_w_click",
  "hash": "DjpgJ0bcgJs",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_w_click_DjpgJ0bcgJs",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "window:onClick$",
  "captures": false,
  "loc": [
    380,
    388
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg",
              "start": 13,
              "end": 75
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 82,
                "end": 84
              },
              "id": null,
              "generator": false,
              "start": 78,
              "end": 84
            },
            "start": 13,
            "end": 84
          }
        ],
        "start": 7,
        "end": 85
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 85
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 85
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_e_click",
  "hash": "z0X12CPQocg",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    150,
    158
  ]
}
```

### Module: test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles.js (ENTRY POINT)

```javascript
export const ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles = ()=>{};
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
              "name": "ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles",
              "start": 13,
              "end": 78
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 85,
                "end": 87
              },
              "id": null,
              "generator": false,
              "start": 81,
              "end": 87
            },
            "start": 13,
            "end": 87
          }
        ],
        "start": 7,
        "end": 88
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 88
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 88
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles",
  "entry": null,
  "displayName": "test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick",
  "hash": "VZGqMEVFles",
  "canonicalFilename": "test.tsx_ManyEventsComponent_component_div_button_q_e_dblclick_VZGqMEVFles",
  "path": "",
  "extension": "js",
  "parent": "ManyEventsComponent_component_ndbrzxiyG3g",
  "ctxKind": "eventHandler",
  "ctxName": "onDblClick$",
  "captures": false,
  "loc": [
    179,
    187
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (8 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 12 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 8 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_ManyEventsComponent_component_ndbrzxiyG3g.js` | `@qwik.dev/core` | 8 |
| `_jsxSorted` | `test.tsx_ManyEventsComponent_component_ndbrzxiyG3g.js` | `@qwik.dev/core` | 4 |

## Diagnostics

```json
[]
```
