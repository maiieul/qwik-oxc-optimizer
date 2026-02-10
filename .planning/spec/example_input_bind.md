# Test: example_input_bind

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Prod |
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { component$, $ } from '@qwik.dev/core';

export const Greeter = component$(() => {
	const value = useSignal(0);
	const checked = useSignal(false);
	const stuff = useSignal();
	return (
		<>
			<input bind:value={value} />
			<input bind:checked={checked} />
			<input bind:stuff={stuff} />
			<div>{value}</div>
			<div>{value.value}</div>
		</>

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
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "importKind": "value",
          "start": 21,
          "end": 22
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 30,
        "end": 46
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 47
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
              "name": "Greeter",
              "optional": false,
              "typeAnnotation": null,
              "start": 62,
              "end": 69
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 72,
                "end": 82
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
                              "name": "value",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 98,
                              "end": 103
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 106,
                                "end": 115
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 116,
                                  "end": 117
                                }
                              ],
                              "optional": false,
                              "start": 106,
                              "end": 118
                            },
                            "definite": false,
                            "start": 98,
                            "end": 118
                          }
                        ],
                        "declare": false,
                        "start": 92,
                        "end": 119
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
                              "name": "checked",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 127,
                              "end": 134
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 137,
                                "end": 146
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": false,
                                  "raw": "false",
                                  "start": 147,
                                  "end": 152
                                }
                              ],
                              "optional": false,
                              "start": 137,
                              "end": 153
                            },
                            "definite": false,
                            "start": 127,
                            "end": 153
                          }
                        ],
                        "declare": false,
                        "start": 121,
                        "end": 154
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
                              "name": "stuff",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 162,
                              "end": 167
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 170,
                                "end": 179
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 170,
                              "end": 181
                            },
                            "definite": false,
                            "start": 162,
                            "end": 181
                          }
                        ],
                        "declare": false,
                        "start": 156,
                        "end": 182
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 195,
                              "end": 197
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 197,
                                "end": 201
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 202,
                                    "end": 207
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 208,
                                          "end": 212
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "value",
                                          "start": 213,
                                          "end": 218
                                        },
                                        "start": 208,
                                        "end": 218
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 220,
                                          "end": 225
                                        },
                                        "start": 219,
                                        "end": 226
                                      },
                                      "start": 208,
                                      "end": 226
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 201,
                                  "end": 229
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 201,
                                "end": 229
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 229,
                                "end": 233
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 234,
                                    "end": 239
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 240,
                                          "end": 244
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "checked",
                                          "start": 245,
                                          "end": 252
                                        },
                                        "start": 240,
                                        "end": 252
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "checked",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 254,
                                          "end": 261
                                        },
                                        "start": 253,
                                        "end": 262
                                      },
                                      "start": 240,
                                      "end": 262
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 233,
                                  "end": 265
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 233,
                                "end": 265
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 265,
                                "end": 269
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 270,
                                    "end": 275
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 276,
                                          "end": 280
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "stuff",
                                          "start": 281,
                                          "end": 286
                                        },
                                        "start": 276,
                                        "end": 286
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "stuff",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 288,
                                          "end": 293
                                        },
                                        "start": 287,
                                        "end": 294
                                      },
                                      "start": 276,
                                      "end": 294
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 269,
                                  "end": 297
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 269,
                                "end": 297
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 297,
                                "end": 301
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 302,
                                    "end": 305
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 301,
                                  "end": 306
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "value",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 307,
                                      "end": 312
                                    },
                                    "start": 306,
                                    "end": 313
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 315,
                                    "end": 318
                                  },
                                  "start": 313,
                                  "end": 319
                                },
                                "start": 301,
                                "end": 319
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 319,
                                "end": 323
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 324,
                                    "end": 327
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 323,
                                  "end": 328
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "value",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 329,
                                        "end": 334
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "value",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 335,
                                        "end": 340
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 329,
                                      "end": 340
                                    },
                                    "start": 328,
                                    "end": 341
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 343,
                                    "end": 346
                                  },
                                  "start": 341,
                                  "end": 347
                                },
                                "start": 323,
                                "end": 347
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 347,
                                "end": 350
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 350,
                              "end": 353
                            },
                            "start": 195,
                            "end": 353
                          },
                          "start": 191,
                          "end": 357
                        },
                        "start": 184,
                        "end": 357
                      }
                    ],
                    "start": 89,
                    "end": 359
                  },
                  "id": null,
                  "generator": false,
                  "start": 83,
                  "end": 359
                }
              ],
              "optional": false,
              "start": 72,
              "end": 360
            },
            "definite": false,
            "start": 62,
            "end": 360
          }
        ],
        "declare": false,
        "start": 56,
        "end": 361
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 361
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 361
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _val } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _chk } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
export const Greeter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    const value = useSignal(0);
    const checked = useSignal(false);
    const stuff = useSignal();
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("input", null, {
            "value": value,
            "q-e:input": inlinedQrl(_val, "_val", [
                value
            ])
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("input", null, {
            "checked": checked,
            "q-e:input": inlinedQrl(_chk, "_chk", [
                checked
            ])
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("input", null, {
            "bind:stuff": stuff
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, value, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(value), 3, null)
    ], 3, "u6_0");
}, "s_n7HuG2hhU0Q"));
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
            "name": "_val",
            "start": 56,
            "end": 60
          },
          "local": {
            "type": "Identifier",
            "name": "_val",
            "start": 56,
            "end": 60
          },
          "start": 56,
          "end": 60
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 68,
        "end": 84
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 85
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 95,
            "end": 105
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 95,
            "end": 105
          },
          "start": 95,
          "end": 105
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 113,
        "end": 129
      },
      "phase": null,
      "attributes": [],
      "start": 86,
      "end": 130
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 140,
            "end": 150
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 140,
            "end": 150
          },
          "start": 140,
          "end": 150
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 158,
        "end": 174
      },
      "phase": null,
      "attributes": [],
      "start": 131,
      "end": 175
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_chk",
            "start": 185,
            "end": 189
          },
          "local": {
            "type": "Identifier",
            "name": "_chk",
            "start": 185,
            "end": 189
          },
          "start": 185,
          "end": 189
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 197,
        "end": 213
      },
      "phase": null,
      "attributes": [],
      "start": 176,
      "end": 214
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 224,
            "end": 233
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 224,
            "end": 233
          },
          "start": 224,
          "end": 233
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 241,
        "end": 257
      },
      "phase": null,
      "attributes": [],
      "start": 215,
      "end": 258
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 268,
            "end": 276
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 280,
            "end": 289
          },
          "start": 268,
          "end": 289
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 297,
        "end": 325
      },
      "phase": null,
      "attributes": [],
      "start": 259,
      "end": 326
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
              "name": "Greeter",
              "start": 340,
              "end": 347
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 364,
                "end": 376
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 391,
                    "end": 401
                  },
                  "arguments": [
                    {
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
                                  "name": "value",
                                  "start": 418,
                                  "end": 423
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useSignal",
                                    "start": 426,
                                    "end": 435
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": 0,
                                      "raw": "0",
                                      "start": 436,
                                      "end": 437
                                    }
                                  ],
                                  "optional": false,
                                  "start": 426,
                                  "end": 438
                                },
                                "start": 418,
                                "end": 438
                              }
                            ],
                            "start": 412,
                            "end": 439
                          },
                          {
                            "type": "VariableDeclaration",
                            "kind": "const",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "checked",
                                  "start": 450,
                                  "end": 457
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useSignal",
                                    "start": 460,
                                    "end": 469
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": false,
                                      "raw": "false",
                                      "start": 470,
                                      "end": 475
                                    }
                                  ],
                                  "optional": false,
                                  "start": 460,
                                  "end": 476
                                },
                                "start": 450,
                                "end": 476
                              }
                            ],
                            "start": 444,
                            "end": 477
                          },
                          {
                            "type": "VariableDeclaration",
                            "kind": "const",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "stuff",
                                  "start": 488,
                                  "end": 493
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useSignal",
                                    "start": 496,
                                    "end": 505
                                  },
                                  "arguments": [],
                                  "optional": false,
                                  "start": 496,
                                  "end": 507
                                },
                                "start": 488,
                                "end": 507
                              }
                            ],
                            "start": 482,
                            "end": 508
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 534,
                                "end": 544
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "_Fragment",
                                  "start": 545,
                                  "end": 554
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 556,
                                  "end": 560
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 562,
                                  "end": 566
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 592,
                                        "end": 602
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "input",
                                          "raw": "\"input\"",
                                          "start": 603,
                                          "end": 610
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 612,
                                          "end": 616
                                        },
                                        {
                                          "type": "ObjectExpression",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Literal",
                                                "value": "value",
                                                "raw": "\"value\"",
                                                "start": 632,
                                                "end": 639
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "value",
                                                "start": 641,
                                                "end": 646
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 632,
                                              "end": 646
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Literal",
                                                "value": "q-e:input",
                                                "raw": "\"q-e:input\"",
                                                "start": 660,
                                                "end": 671
                                              },
                                              "value": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "inlinedQrl",
                                                  "start": 673,
                                                  "end": 683
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_val",
                                                    "start": 684,
                                                    "end": 688
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": "_val",
                                                    "raw": "\"_val\"",
                                                    "start": 690,
                                                    "end": 696
                                                  },
                                                  {
                                                    "type": "ArrayExpression",
                                                    "elements": [
                                                      {
                                                        "type": "Identifier",
                                                        "name": "value",
                                                        "start": 716,
                                                        "end": 721
                                                      }
                                                    ],
                                                    "start": 698,
                                                    "end": 735
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 673,
                                                "end": 736
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 660,
                                              "end": 736
                                            }
                                          ],
                                          "start": 618,
                                          "end": 746
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 748,
                                          "end": 752
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 754,
                                          "end": 755
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 757,
                                          "end": 761
                                        }
                                      ],
                                      "optional": false,
                                      "start": 592,
                                      "end": 762
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 786,
                                        "end": 796
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "input",
                                          "raw": "\"input\"",
                                          "start": 797,
                                          "end": 804
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 806,
                                          "end": 810
                                        },
                                        {
                                          "type": "ObjectExpression",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Literal",
                                                "value": "checked",
                                                "raw": "\"checked\"",
                                                "start": 826,
                                                "end": 835
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "checked",
                                                "start": 837,
                                                "end": 844
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 826,
                                              "end": 844
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Literal",
                                                "value": "q-e:input",
                                                "raw": "\"q-e:input\"",
                                                "start": 858,
                                                "end": 869
                                              },
                                              "value": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "inlinedQrl",
                                                  "start": 871,
                                                  "end": 881
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "_chk",
                                                    "start": 882,
                                                    "end": 886
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": "_chk",
                                                    "raw": "\"_chk\"",
                                                    "start": 888,
                                                    "end": 894
                                                  },
                                                  {
                                                    "type": "ArrayExpression",
                                                    "elements": [
                                                      {
                                                        "type": "Identifier",
                                                        "name": "checked",
                                                        "start": 914,
                                                        "end": 921
                                                      }
                                                    ],
                                                    "start": 896,
                                                    "end": 935
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 871,
                                                "end": 936
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 858,
                                              "end": 936
                                            }
                                          ],
                                          "start": 812,
                                          "end": 946
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 948,
                                          "end": 952
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 954,
                                          "end": 955
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 957,
                                          "end": 961
                                        }
                                      ],
                                      "optional": false,
                                      "start": 786,
                                      "end": 962
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 986,
                                        "end": 996
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "input",
                                          "raw": "\"input\"",
                                          "start": 997,
                                          "end": 1004
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1006,
                                          "end": 1010
                                        },
                                        {
                                          "type": "ObjectExpression",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Literal",
                                                "value": "bind:stuff",
                                                "raw": "\"bind:stuff\"",
                                                "start": 1026,
                                                "end": 1038
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "stuff",
                                                "start": 1040,
                                                "end": 1045
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 1026,
                                              "end": 1045
                                            }
                                          ],
                                          "start": 1012,
                                          "end": 1055
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1057,
                                          "end": 1061
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1063,
                                          "end": 1064
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1066,
                                          "end": 1070
                                        }
                                      ],
                                      "optional": false,
                                      "start": 986,
                                      "end": 1071
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1095,
                                        "end": 1105
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 1106,
                                          "end": 1111
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1113,
                                          "end": 1117
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1119,
                                          "end": 1123
                                        },
                                        {
                                          "type": "Identifier",
                                          "name": "value",
                                          "start": 1125,
                                          "end": 1130
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1132,
                                          "end": 1133
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1135,
                                          "end": 1139
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1095,
                                      "end": 1140
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1164,
                                        "end": 1174
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 1175,
                                          "end": 1180
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1182,
                                          "end": 1186
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1188,
                                          "end": 1192
                                        },
                                        {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "name": "_wrapProp",
                                            "start": 1194,
                                            "end": 1203
                                          },
                                          "arguments": [
                                            {
                                              "type": "Identifier",
                                              "name": "value",
                                              "start": 1204,
                                              "end": 1209
                                            }
                                          ],
                                          "optional": false,
                                          "start": 1194,
                                          "end": 1210
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1212,
                                          "end": 1213
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1215,
                                          "end": 1219
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1164,
                                      "end": 1220
                                    }
                                  ],
                                  "start": 568,
                                  "end": 1226
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1228,
                                  "end": 1229
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_0",
                                  "raw": "\"u6_0\"",
                                  "start": 1231,
                                  "end": 1237
                                }
                              ],
                              "optional": false,
                              "start": 534,
                              "end": 1238
                            },
                            "start": 513,
                            "end": 1239
                          }
                        ],
                        "start": 406,
                        "end": 1241
                      },
                      "id": null,
                      "generator": false,
                      "start": 402,
                      "end": 1241
                    },
                    {
                      "type": "Literal",
                      "value": "s_n7HuG2hhU0Q",
                      "raw": "\"s_n7HuG2hhU0Q\"",
                      "start": 1243,
                      "end": 1258
                    }
                  ],
                  "optional": false,
                  "start": 391,
                  "end": 1259
                }
              ],
              "optional": false,
              "start": 364,
              "end": 1260
            },
            "start": 340,
            "end": 1260
          }
        ],
        "start": 334,
        "end": 1261
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 327,
      "end": 1261
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1261
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_wrapProp()`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 8 call expressions
- **[CONV-12] Input Binding**: Input binding via `bind:` directives, `_val` handler, `_chk` handler

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `inlinedQrl` | `test.js` | `@qwik.dev/core` | 4 |
| `_jsxSorted` | `test.js` | `@qwik.dev/core` | 7 |
| `_wrapProp` | `test.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.js` | `-` | 3 |
| `_val` | `test.js` | `@qwik.dev/core` | 3 |
| `_chk` | `test.js` | `@qwik.dev/core` | 3 |

## Diagnostics

```json
[]
```
