# Test: example_issue_4438

## Test Configuration

**Note:** Regression test for issue 4438 -- handling $localize tagged template literals in ternary expressions within JSX props and children.

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

export const App = component$(() => {
	const toggle = useSignal(false);
	return (
		<>
			<div data-nu={toggle.value ? $localize`singular` : 'plural'}></div>
			<div>{toggle.value ? $localize`singular` : $localize`plural`}</div>
		</>
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
              "name": "App",
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
                              "name": "toggle",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 102,
                              "end": 108
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 111,
                                "end": 120
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": false,
                                  "raw": "false",
                                  "start": 121,
                                  "end": 126
                                }
                              ],
                              "optional": false,
                              "start": 111,
                              "end": 127
                            },
                            "definite": false,
                            "start": 102,
                            "end": 127
                          }
                        ],
                        "declare": false,
                        "start": 96,
                        "end": 128
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 141,
                              "end": 143
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 143,
                                "end": 147
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 148,
                                    "end": 151
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "data-nu",
                                        "start": 152,
                                        "end": 159
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "ConditionalExpression",
                                          "test": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "toggle",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 161,
                                              "end": 167
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "value",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 168,
                                              "end": 173
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 161,
                                            "end": 173
                                          },
                                          "consequent": {
                                            "type": "TaggedTemplateExpression",
                                            "tag": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "$localize",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 176,
                                              "end": 185
                                            },
                                            "typeArguments": null,
                                            "quasi": {
                                              "type": "TemplateLiteral",
                                              "quasis": [
                                                {
                                                  "type": "TemplateElement",
                                                  "value": {
                                                    "raw": "singular",
                                                    "cooked": "singular"
                                                  },
                                                  "tail": true,
                                                  "start": 185,
                                                  "end": 195
                                                }
                                              ],
                                              "expressions": [],
                                              "start": 185,
                                              "end": 195
                                            },
                                            "start": 176,
                                            "end": 195
                                          },
                                          "alternate": {
                                            "type": "Literal",
                                            "value": "plural",
                                            "raw": "'plural'",
                                            "start": 198,
                                            "end": 206
                                          },
                                          "start": 161,
                                          "end": 206
                                        },
                                        "start": 160,
                                        "end": 207
                                      },
                                      "start": 152,
                                      "end": 207
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 147,
                                  "end": 208
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 210,
                                    "end": 213
                                  },
                                  "start": 208,
                                  "end": 214
                                },
                                "start": 147,
                                "end": 214
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 214,
                                "end": 218
                              },
                              {
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
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "toggle",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 224,
                                          "end": 230
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 231,
                                          "end": 236
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 224,
                                        "end": 236
                                      },
                                      "consequent": {
                                        "type": "TaggedTemplateExpression",
                                        "tag": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "$localize",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 239,
                                          "end": 248
                                        },
                                        "typeArguments": null,
                                        "quasi": {
                                          "type": "TemplateLiteral",
                                          "quasis": [
                                            {
                                              "type": "TemplateElement",
                                              "value": {
                                                "raw": "singular",
                                                "cooked": "singular"
                                              },
                                              "tail": true,
                                              "start": 248,
                                              "end": 258
                                            }
                                          ],
                                          "expressions": [],
                                          "start": 248,
                                          "end": 258
                                        },
                                        "start": 239,
                                        "end": 258
                                      },
                                      "alternate": {
                                        "type": "TaggedTemplateExpression",
                                        "tag": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "$localize",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 261,
                                          "end": 270
                                        },
                                        "typeArguments": null,
                                        "quasi": {
                                          "type": "TemplateLiteral",
                                          "quasis": [
                                            {
                                              "type": "TemplateElement",
                                              "value": {
                                                "raw": "plural",
                                                "cooked": "plural"
                                              },
                                              "tail": true,
                                              "start": 270,
                                              "end": 278
                                            }
                                          ],
                                          "expressions": [],
                                          "start": 270,
                                          "end": 278
                                        },
                                        "start": 261,
                                        "end": 278
                                      },
                                      "start": 224,
                                      "end": 278
                                    },
                                    "start": 223,
                                    "end": 279
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 281,
                                    "end": 284
                                  },
                                  "start": 279,
                                  "end": 285
                                },
                                "start": 218,
                                "end": 285
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 285,
                                "end": 288
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 288,
                              "end": 291
                            },
                            "start": 141,
                            "end": 291
                          },
                          "start": 137,
                          "end": 294
                        },
                        "start": 130,
                        "end": 295
                      }
                    ],
                    "start": 93,
                    "end": 297
                  },
                  "id": null,
                  "generator": false,
                  "start": 87,
                  "end": 297
                }
              ],
              "optional": false,
              "start": 76,
              "end": 298
            },
            "definite": false,
            "start": 70,
            "end": 298
          }
        ],
        "declare": false,
        "start": 64,
        "end": 299
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 57,
      "end": 299
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 299
}

```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { useSignal } from '@qwik.dev/core';
const App_component_ckEPmXZlub0 = ()=>{
    const toggle = useSignal(false);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", {
            "data-nu": toggle.value ? $localize`singular` : 'plural'
        }, null, null, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, toggle.value ? $localize`singular` : $localize`plural`, 1, null)
    ], 1, "u6_0");
};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
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
            "name": "_jsxSorted",
            "start": 56,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 56,
            "end": 66
          },
          "start": 56,
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
      "start": 47,
      "end": 91
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "start": 101,
          "end": 111
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 119,
        "end": 135
      },
      "phase": null,
      "attributes": [],
      "start": 92,
      "end": 136
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 146,
            "end": 154
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 158,
            "end": 167
          },
          "start": 146,
          "end": 167
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 175,
        "end": 203
      },
      "phase": null,
      "attributes": [],
      "start": 137,
      "end": 204
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 214,
            "end": 223
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 214,
            "end": 223
          },
          "start": 214,
          "end": 223
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 231,
        "end": 247
      },
      "phase": null,
      "attributes": [],
      "start": 205,
      "end": 248
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "App_component_ckEPmXZlub0",
            "start": 255,
            "end": 280
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
                        "name": "toggle",
                        "start": 299,
                        "end": 305
                      },
                      "init": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "name": "useSignal",
                          "start": 308,
                          "end": 317
                        },
                        "arguments": [
                          {
                            "type": "Literal",
                            "value": false,
                            "raw": "false",
                            "start": 318,
                            "end": 323
                          }
                        ],
                        "optional": false,
                        "start": 308,
                        "end": 324
                      },
                      "start": 299,
                      "end": 324
                    }
                  ],
                  "start": 293,
                  "end": 325
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "CallExpression",
                    "callee": {
                      "type": "Identifier",
                      "name": "_jsxSorted",
                      "start": 351,
                      "end": 361
                    },
                    "arguments": [
                      {
                        "type": "Identifier",
                        "name": "_Fragment",
                        "start": 362,
                        "end": 371
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 373,
                        "end": 377
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 379,
                        "end": 383
                      },
                      {
                        "type": "ArrayExpression",
                        "elements": [
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 409,
                              "end": 419
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 420,
                                "end": 425
                              },
                              {
                                "type": "ObjectExpression",
                                "properties": [
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Literal",
                                      "value": "data-nu",
                                      "raw": "\"data-nu\"",
                                      "start": 441,
                                      "end": 450
                                    },
                                    "value": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "toggle",
                                          "start": 452,
                                          "end": 458
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "value",
                                          "start": 459,
                                          "end": 464
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 452,
                                        "end": 464
                                      },
                                      "consequent": {
                                        "type": "TaggedTemplateExpression",
                                        "tag": {
                                          "type": "Identifier",
                                          "name": "$localize",
                                          "start": 467,
                                          "end": 476
                                        },
                                        "quasi": {
                                          "type": "TemplateLiteral",
                                          "quasis": [
                                            {
                                              "type": "TemplateElement",
                                              "value": {
                                                "raw": "singular",
                                                "cooked": "singular"
                                              },
                                              "tail": true,
                                              "start": 477,
                                              "end": 485
                                            }
                                          ],
                                          "expressions": [],
                                          "start": 476,
                                          "end": 486
                                        },
                                        "start": 467,
                                        "end": 486
                                      },
                                      "alternate": {
                                        "type": "Literal",
                                        "value": "plural",
                                        "raw": "'plural'",
                                        "start": 489,
                                        "end": 497
                                      },
                                      "start": 452,
                                      "end": 497
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "start": 441,
                                    "end": 497
                                  }
                                ],
                                "start": 427,
                                "end": 507
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 509,
                                "end": 513
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 515,
                                "end": 519
                              },
                              {
                                "type": "Literal",
                                "value": 3,
                                "raw": "3",
                                "start": 521,
                                "end": 522
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 524,
                                "end": 528
                              }
                            ],
                            "optional": false,
                            "start": 409,
                            "end": 529
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 553,
                              "end": 563
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 564,
                                "end": 569
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 571,
                                "end": 575
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 577,
                                "end": 581
                              },
                              {
                                "type": "ConditionalExpression",
                                "test": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "toggle",
                                    "start": 583,
                                    "end": 589
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 590,
                                    "end": 595
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 583,
                                  "end": 595
                                },
                                "consequent": {
                                  "type": "TaggedTemplateExpression",
                                  "tag": {
                                    "type": "Identifier",
                                    "name": "$localize",
                                    "start": 598,
                                    "end": 607
                                  },
                                  "quasi": {
                                    "type": "TemplateLiteral",
                                    "quasis": [
                                      {
                                        "type": "TemplateElement",
                                        "value": {
                                          "raw": "singular",
                                          "cooked": "singular"
                                        },
                                        "tail": true,
                                        "start": 608,
                                        "end": 616
                                      }
                                    ],
                                    "expressions": [],
                                    "start": 607,
                                    "end": 617
                                  },
                                  "start": 598,
                                  "end": 617
                                },
                                "alternate": {
                                  "type": "TaggedTemplateExpression",
                                  "tag": {
                                    "type": "Identifier",
                                    "name": "$localize",
                                    "start": 620,
                                    "end": 629
                                  },
                                  "quasi": {
                                    "type": "TemplateLiteral",
                                    "quasis": [
                                      {
                                        "type": "TemplateElement",
                                        "value": {
                                          "raw": "plural",
                                          "cooked": "plural"
                                        },
                                        "tail": true,
                                        "start": 630,
                                        "end": 636
                                      }
                                    ],
                                    "expressions": [],
                                    "start": 629,
                                    "end": 637
                                  },
                                  "start": 620,
                                  "end": 637
                                },
                                "start": 583,
                                "end": 637
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 639,
                                "end": 640
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 642,
                                "end": 646
                              }
                            ],
                            "optional": false,
                            "start": 553,
                            "end": 647
                          }
                        ],
                        "start": 385,
                        "end": 653
                      },
                      {
                        "type": "Literal",
                        "value": 1,
                        "raw": "1",
                        "start": 655,
                        "end": 656
                      },
                      {
                        "type": "Literal",
                        "value": "u6_0",
                        "raw": "\"u6_0\"",
                        "start": 658,
                        "end": 664
                      }
                    ],
                    "optional": false,
                    "start": 351,
                    "end": 665
                  },
                  "start": 330,
                  "end": 666
                }
              ],
              "start": 287,
              "end": 668
            },
            "id": null,
            "generator": false,
            "start": 283,
            "end": 668
          },
          "start": 255,
          "end": 668
        }
      ],
      "start": 249,
      "end": 669
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
              "start": 683,
              "end": 686
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 703,
                "end": 715
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 730,
                    "end": 740
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "App_component_ckEPmXZlub0",
                      "start": 741,
                      "end": 766
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 768,
                      "end": 795
                    }
                  ],
                  "optional": false,
                  "start": 730,
                  "end": 796
                }
              ],
              "optional": false,
              "start": 703,
              "end": 797
            },
            "start": 683,
            "end": 797
          }
        ],
        "start": 677,
        "end": 798
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 670,
      "end": 798
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 798
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-07] PURE Annotations**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 3 |
| useSignal | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
