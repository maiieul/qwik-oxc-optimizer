# Test: example_issue_33443

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, useSignal } from '@qwik.dev/core';

export const Issue3742 = component$(({description = '', other}: any) => {
	const counter = useSignal(0);
	return (
		<div
		title={(description && 'description' in other) ? `Hello ${counter.value}` : `Bye ${counter.value}`}
		>
		Issue3742
		<button onClick$={() => counter.value++}>
			Increment
		</button>
		</div>
	)
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 38,
        "end": 54
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 55
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
              "name": "Issue3742",
              "optional": false,
              "typeAnnotation": null,
              "start": 70,
              "end": 79
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 82,
                "end": 92
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
                      "type": "ObjectPattern",
                      "decorators": [],
                      "properties": [
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "description",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 95,
                            "end": 106
                          },
                          "value": {
                            "type": "AssignmentPattern",
                            "decorators": [],
                            "left": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "description",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 95,
                              "end": 106
                            },
                            "right": {
                              "type": "Literal",
                              "value": "",
                              "raw": "''",
                              "start": 109,
                              "end": 111
                            },
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 95,
                            "end": 111
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 95,
                          "end": 111
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "other",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 113,
                            "end": 118
                          },
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "other",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 113,
                            "end": 118
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 113,
                          "end": 118
                        }
                      ],
                      "optional": false,
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSAnyKeyword",
                          "start": 121,
                          "end": 124
                        },
                        "start": 119,
                        "end": 124
                      },
                      "start": 94,
                      "end": 124
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
                              "name": "counter",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 138,
                              "end": 145
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 148,
                                "end": 157
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 158,
                                  "end": 159
                                }
                              ],
                              "optional": false,
                              "start": 148,
                              "end": 160
                            },
                            "definite": false,
                            "start": 138,
                            "end": 160
                          }
                        ],
                        "declare": false,
                        "start": 132,
                        "end": 161
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
                                "start": 175,
                                "end": 178
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 181,
                                    "end": 186
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "ParenthesizedExpression",
                                        "expression": {
                                          "type": "LogicalExpression",
                                          "left": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "description",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 189,
                                            "end": 200
                                          },
                                          "operator": "&&",
                                          "right": {
                                            "type": "BinaryExpression",
                                            "left": {
                                              "type": "Literal",
                                              "value": "description",
                                              "raw": "'description'",
                                              "start": 204,
                                              "end": 217
                                            },
                                            "operator": "in",
                                            "right": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "other",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 221,
                                              "end": 226
                                            },
                                            "start": 204,
                                            "end": 226
                                          },
                                          "start": 189,
                                          "end": 226
                                        },
                                        "start": 188,
                                        "end": 227
                                      },
                                      "consequent": {
                                        "type": "TemplateLiteral",
                                        "quasis": [
                                          {
                                            "type": "TemplateElement",
                                            "value": {
                                              "raw": "Hello ",
                                              "cooked": "Hello "
                                            },
                                            "tail": false,
                                            "start": 230,
                                            "end": 239
                                          },
                                          {
                                            "type": "TemplateElement",
                                            "value": {
                                              "raw": "",
                                              "cooked": ""
                                            },
                                            "tail": true,
                                            "start": 252,
                                            "end": 254
                                          }
                                        ],
                                        "expressions": [
                                          {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "counter",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 239,
                                              "end": 246
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "value",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 247,
                                              "end": 252
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 239,
                                            "end": 252
                                          }
                                        ],
                                        "start": 230,
                                        "end": 254
                                      },
                                      "alternate": {
                                        "type": "TemplateLiteral",
                                        "quasis": [
                                          {
                                            "type": "TemplateElement",
                                            "value": {
                                              "raw": "Bye ",
                                              "cooked": "Bye "
                                            },
                                            "tail": false,
                                            "start": 257,
                                            "end": 264
                                          },
                                          {
                                            "type": "TemplateElement",
                                            "value": {
                                              "raw": "",
                                              "cooked": ""
                                            },
                                            "tail": true,
                                            "start": 277,
                                            "end": 279
                                          }
                                        ],
                                        "expressions": [
                                          {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "counter",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 264,
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
                                            "start": 264,
                                            "end": 277
                                          }
                                        ],
                                        "start": 257,
                                        "end": 279
                                      },
                                      "start": 188,
                                      "end": 279
                                    },
                                    "start": 187,
                                    "end": 280
                                  },
                                  "start": 181,
                                  "end": 280
                                }
                              ],
                              "selfClosing": false,
                              "start": 174,
                              "end": 284
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\tIssue3742\n\t\t",
                                "raw": "\n\t\tIssue3742\n\t\t",
                                "start": 284,
                                "end": 299
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "button",
                                    "start": 300,
                                    "end": 306
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "onClick$",
                                        "start": 307,
                                        "end": 315
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
                                            "type": "UpdateExpression",
                                            "operator": "++",
                                            "prefix": false,
                                            "argument": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "counter",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 323,
                                                "end": 330
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "value",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 331,
                                                "end": 336
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 323,
                                              "end": 336
                                            },
                                            "start": 323,
                                            "end": 338
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 317,
                                          "end": 338
                                        },
                                        "start": 316,
                                        "end": 339
                                      },
                                      "start": 307,
                                      "end": 339
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 299,
                                  "end": 340
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "\n\t\t\tIncrement\n\t\t",
                                    "raw": "\n\t\t\tIncrement\n\t\t",
                                    "start": 340,
                                    "end": 356
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "button",
                                    "start": 358,
                                    "end": 364
                                  },
                                  "start": 356,
                                  "end": 365
                                },
                                "start": 299,
                                "end": 365
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 365,
                                "end": 368
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 370,
                                "end": 373
                              },
                              "start": 368,
                              "end": 374
                            },
                            "start": 174,
                            "end": 374
                          },
                          "start": 170,
                          "end": 377
                        },
                        "start": 163,
                        "end": 377
                      }
                    ],
                    "start": 129,
                    "end": 380
                  },
                  "id": null,
                  "generator": false,
                  "start": 93,
                  "end": 380
                }
              ],
              "optional": false,
              "start": 82,
              "end": 381
            },
            "definite": false,
            "start": 70,
            "end": 381
          }
        ],
        "declare": false,
        "start": 64,
        "end": 382
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 57,
      "end": 382
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 383
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _fnSignal } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
const _hf0 = (p0, p1)=>(p0.description ?? '') && 'description' in p0.other ? `Hello ${p1.value}` : `Bye ${p1.value}`;
const _hf0_str = '(p0.description??"")&&"description"in p0.other?`Hello ${p1.value}`:`Bye ${p1.value}`';
import { useSignal } from '@qwik.dev/core';
const Issue3742_component_div_button_q_e_click_95Hlm8WgsYY = ()=>{
    const counter = _captures[0];
    return counter.value++;
};
const Issue3742_component_svSy0PlWTAw = (_rawProps)=>{
    const counter = useSignal(0);
    return /*#__PURE__*/ _jsxSorted("div", {
        title: _fnSignal(_hf0, [
            _rawProps,
            counter
        ], _hf0_str)
    }, null, [
        "Issue3742",
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ inlinedQrl(Issue3742_component_div_button_q_e_click_95Hlm8WgsYY, "Issue3742_component_div_button_q_e_click_95Hlm8WgsYY", [
                counter
            ])
        }, "Increment", 3, null)
    ], 3, "u6_0");
};
export const Issue3742 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(Issue3742_component_svSy0PlWTAw, "Issue3742_component_svSy0PlWTAw"));
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
            "name": "_fnSignal",
            "start": 56,
            "end": 65
          },
          "local": {
            "type": "Identifier",
            "name": "_fnSignal",
            "start": 56,
            "end": 65
          },
          "start": 56,
          "end": 65
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 73,
        "end": 89
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 90
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_captures",
            "start": 100,
            "end": 109
          },
          "local": {
            "type": "Identifier",
            "name": "_captures",
            "start": 100,
            "end": 109
          },
          "start": 100,
          "end": 109
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 117,
        "end": 133
      },
      "phase": null,
      "attributes": [],
      "start": 91,
      "end": 134
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 144,
            "end": 154
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 144,
            "end": 154
          },
          "start": 144,
          "end": 154
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 162,
        "end": 178
      },
      "phase": null,
      "attributes": [],
      "start": 135,
      "end": 179
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 189,
            "end": 199
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 189,
            "end": 199
          },
          "start": 189,
          "end": 199
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 207,
        "end": 223
      },
      "phase": null,
      "attributes": [],
      "start": 180,
      "end": 224
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
            "start": 231,
            "end": 235
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 239,
                "end": 241
              },
              {
                "type": "Identifier",
                "name": "p1",
                "start": 243,
                "end": 245
              }
            ],
            "body": {
              "type": "ConditionalExpression",
              "test": {
                "type": "LogicalExpression",
                "left": {
                  "type": "ParenthesizedExpression",
                  "expression": {
                    "type": "LogicalExpression",
                    "left": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "p0",
                        "start": 249,
                        "end": 251
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "description",
                        "start": 252,
                        "end": 263
                      },
                      "optional": false,
                      "computed": false,
                      "start": 249,
                      "end": 263
                    },
                    "operator": "??",
                    "right": {
                      "type": "Literal",
                      "value": "",
                      "raw": "''",
                      "start": 267,
                      "end": 269
                    },
                    "start": 249,
                    "end": 269
                  },
                  "start": 248,
                  "end": 270
                },
                "operator": "&&",
                "right": {
                  "type": "BinaryExpression",
                  "left": {
                    "type": "Literal",
                    "value": "description",
                    "raw": "'description'",
                    "start": 274,
                    "end": 287
                  },
                  "operator": "in",
                  "right": {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "name": "p0",
                      "start": 291,
                      "end": 293
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "other",
                      "start": 294,
                      "end": 299
                    },
                    "optional": false,
                    "computed": false,
                    "start": 291,
                    "end": 299
                  },
                  "start": 274,
                  "end": 299
                },
                "start": 248,
                "end": 299
              },
              "consequent": {
                "type": "TemplateLiteral",
                "quasis": [
                  {
                    "type": "TemplateElement",
                    "value": {
                      "raw": "Hello ",
                      "cooked": "Hello "
                    },
                    "tail": false,
                    "start": 303,
                    "end": 309
                  },
                  {
                    "type": "TemplateElement",
                    "value": {
                      "raw": "",
                      "cooked": ""
                    },
                    "tail": true,
                    "start": 320,
                    "end": 320
                  }
                ],
                "expressions": [
                  {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "name": "p1",
                      "start": 311,
                      "end": 313
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "value",
                      "start": 314,
                      "end": 319
                    },
                    "optional": false,
                    "computed": false,
                    "start": 311,
                    "end": 319
                  }
                ],
                "start": 302,
                "end": 321
              },
              "alternate": {
                "type": "TemplateLiteral",
                "quasis": [
                  {
                    "type": "TemplateElement",
                    "value": {
                      "raw": "Bye ",
                      "cooked": "Bye "
                    },
                    "tail": false,
                    "start": 325,
                    "end": 329
                  },
                  {
                    "type": "TemplateElement",
                    "value": {
                      "raw": "",
                      "cooked": ""
                    },
                    "tail": true,
                    "start": 340,
                    "end": 340
                  }
                ],
                "expressions": [
                  {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "name": "p1",
                      "start": 331,
                      "end": 333
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "value",
                      "start": 334,
                      "end": 339
                    },
                    "optional": false,
                    "computed": false,
                    "start": 331,
                    "end": 339
                  }
                ],
                "start": 324,
                "end": 341
              },
              "start": 248,
              "end": 341
            },
            "id": null,
            "generator": false,
            "start": 238,
            "end": 341
          },
          "start": 231,
          "end": 341
        }
      ],
      "start": 225,
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
            "name": "_hf0_str",
            "start": 349,
            "end": 357
          },
          "init": {
            "type": "Literal",
            "value": "(p0.description??\"\")&&\"description\"in p0.other?`Hello ${p1.value}`:`Bye ${p1.value}`",
            "raw": "'(p0.description??\"\")&&\"description\"in p0.other?`Hello ${p1.value}`:`Bye ${p1.value}`'",
            "start": 360,
            "end": 446
          },
          "start": 349,
          "end": 446
        }
      ],
      "start": 343,
      "end": 447
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 457,
            "end": 466
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 457,
            "end": 466
          },
          "start": 457,
          "end": 466
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 474,
        "end": 490
      },
      "phase": null,
      "attributes": [],
      "start": 448,
      "end": 491
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "Issue3742_component_div_button_q_e_click_95Hlm8WgsYY",
            "start": 498,
            "end": 550
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
                        "name": "counter",
                        "start": 569,
                        "end": 576
                      },
                      "init": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "_captures",
                          "start": 579,
                          "end": 588
                        },
                        "property": {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 589,
                          "end": 590
                        },
                        "optional": false,
                        "computed": true,
                        "start": 579,
                        "end": 591
                      },
                      "start": 569,
                      "end": 591
                    }
                  ],
                  "start": 563,
                  "end": 592
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "UpdateExpression",
                    "operator": "++",
                    "prefix": false,
                    "argument": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "name": "counter",
                        "start": 604,
                        "end": 611
                      },
                      "property": {
                        "type": "Identifier",
                        "name": "value",
                        "start": 612,
                        "end": 617
                      },
                      "optional": false,
                      "computed": false,
                      "start": 604,
                      "end": 617
                    },
                    "start": 604,
                    "end": 619
                  },
                  "start": 597,
                  "end": 620
                }
              ],
              "start": 557,
              "end": 622
            },
            "id": null,
            "generator": false,
            "start": 553,
            "end": 622
          },
          "start": 498,
          "end": 622
        }
      ],
      "start": 492,
      "end": 623
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "Issue3742_component_svSy0PlWTAw",
            "start": 630,
            "end": 661
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "_rawProps",
                "start": 665,
                "end": 674
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
                        "name": "counter",
                        "start": 689,
                        "end": 696
                      },
                      "init": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "name": "useSignal",
                          "start": 699,
                          "end": 708
                        },
                        "arguments": [
                          {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 709,
                            "end": 710
                          }
                        ],
                        "optional": false,
                        "start": 699,
                        "end": 711
                      },
                      "start": 689,
                      "end": 711
                    }
                  ],
                  "start": 683,
                  "end": 712
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "CallExpression",
                    "callee": {
                      "type": "Identifier",
                      "name": "_jsxSorted",
                      "start": 738,
                      "end": 748
                    },
                    "arguments": [
                      {
                        "type": "Literal",
                        "value": "div",
                        "raw": "\"div\"",
                        "start": 749,
                        "end": 754
                      },
                      {
                        "type": "ObjectExpression",
                        "properties": [
                          {
                            "type": "Property",
                            "kind": "init",
                            "key": {
                              "type": "Identifier",
                              "name": "title",
                              "start": 766,
                              "end": 771
                            },
                            "value": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_fnSignal",
                                "start": 773,
                                "end": 782
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "_hf0",
                                  "start": 783,
                                  "end": 787
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "Identifier",
                                      "name": "_rawProps",
                                      "start": 803,
                                      "end": 812
                                    },
                                    {
                                      "type": "Identifier",
                                      "name": "counter",
                                      "start": 826,
                                      "end": 833
                                    }
                                  ],
                                  "start": 789,
                                  "end": 843
                                },
                                {
                                  "type": "Identifier",
                                  "name": "_hf0_str",
                                  "start": 845,
                                  "end": 853
                                }
                              ],
                              "optional": false,
                              "start": 773,
                              "end": 854
                            },
                            "method": false,
                            "shorthand": false,
                            "computed": false,
                            "start": 766,
                            "end": 854
                          }
                        ],
                        "start": 756,
                        "end": 860
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 862,
                        "end": 866
                      },
                      {
                        "type": "ArrayExpression",
                        "elements": [
                          {
                            "type": "Literal",
                            "value": "Issue3742",
                            "raw": "\"Issue3742\"",
                            "start": 878,
                            "end": 889
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 913,
                              "end": 923
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "button",
                                "raw": "\"button\"",
                                "start": 924,
                                "end": 932
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 934,
                                "end": 938
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
                                      "start": 954,
                                      "end": 965
                                    },
                                    "value": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "inlinedQrl",
                                        "start": 981,
                                        "end": 991
                                      },
                                      "arguments": [
                                        {
                                          "type": "Identifier",
                                          "name": "Issue3742_component_div_button_q_e_click_95Hlm8WgsYY",
                                          "start": 992,
                                          "end": 1044
                                        },
                                        {
                                          "type": "Literal",
                                          "value": "Issue3742_component_div_button_q_e_click_95Hlm8WgsYY",
                                          "raw": "\"Issue3742_component_div_button_q_e_click_95Hlm8WgsYY\"",
                                          "start": 1046,
                                          "end": 1100
                                        },
                                        {
                                          "type": "ArrayExpression",
                                          "elements": [
                                            {
                                              "type": "Identifier",
                                              "name": "counter",
                                              "start": 1120,
                                              "end": 1127
                                            }
                                          ],
                                          "start": 1102,
                                          "end": 1141
                                        }
                                      ],
                                      "optional": false,
                                      "start": 981,
                                      "end": 1142
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "start": 954,
                                    "end": 1142
                                  }
                                ],
                                "start": 940,
                                "end": 1152
                              },
                              {
                                "type": "Literal",
                                "value": "Increment",
                                "raw": "\"Increment\"",
                                "start": 1154,
                                "end": 1165
                              },
                              {
                                "type": "Literal",
                                "value": 3,
                                "raw": "3",
                                "start": 1167,
                                "end": 1168
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 1170,
                                "end": 1174
                              }
                            ],
                            "optional": false,
                            "start": 913,
                            "end": 1175
                          }
                        ],
                        "start": 868,
                        "end": 1181
                      },
                      {
                        "type": "Literal",
                        "value": 3,
                        "raw": "3",
                        "start": 1183,
                        "end": 1184
                      },
                      {
                        "type": "Literal",
                        "value": "u6_0",
                        "raw": "\"u6_0\"",
                        "start": 1186,
                        "end": 1192
                      }
                    ],
                    "optional": false,
                    "start": 738,
                    "end": 1193
                  },
                  "start": 717,
                  "end": 1194
                }
              ],
              "start": 677,
              "end": 1196
            },
            "id": null,
            "generator": false,
            "start": 664,
            "end": 1196
          },
          "start": 630,
          "end": 1196
        }
      ],
      "start": 624,
      "end": 1197
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
              "name": "Issue3742",
              "start": 1211,
              "end": 1220
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 1237,
                "end": 1249
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 1264,
                    "end": 1274
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "Issue3742_component_svSy0PlWTAw",
                      "start": 1275,
                      "end": 1306
                    },
                    {
                      "type": "Literal",
                      "value": "Issue3742_component_svSy0PlWTAw",
                      "raw": "\"Issue3742_component_svSy0PlWTAw\"",
                      "start": 1308,
                      "end": 1341
                    }
                  ],
                  "optional": false,
                  "start": 1264,
                  "end": 1342
                }
              ],
              "optional": false,
              "start": 1237,
              "end": 1343
            },
            "start": 1211,
            "end": 1343
          }
        ],
        "start": 1205,
        "end": 1344
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 1198,
      "end": 1344
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1345
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: `inlinedQrl()` calls wrap both component body and event handler (Hoist strategy = inlined, not lazy)
- **[CONV-02] Dollar-to-Qrl Conversion**: `component$` -> `componentQrl(inlinedQrl(...))`, `onClick$` -> `q-e:click` with `inlinedQrl()`
- **[CONV-03] JSX Transforms**: JSX transformed to `_jsxSorted()` calls
- **[CONV-04] Signal Helpers**: `_fnSignal(_hf0, [_rawProps, counter], _hf0_str)` used for reactive ternary title prop
- **[CONV-05] Capture Patterns**: `_captures[0]` used in event handler to access `counter`. Capture array passed as 3rd arg to `inlinedQrl()`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `inlinedQrl()`, `_jsxSorted()` calls
- **[CONV-11] Props Destructuring**: `({description = '', other}: any)` -> `_rawProps` parameter; destructured props accessed via `_rawProps.description`, `_rawProps.other`
- **[CONV-14] Hoisted Functions**: `_hf0` hoisted function extracts ternary expression with `_hf0_str` serialized form

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 2 |
| _jsxSorted | test.js | @qwik.dev/core | 2 |
| _fnSignal | test.js | @qwik.dev/core | 1 |
| _captures | test.js | @qwik.dev/core | 1 |
| useSignal | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
