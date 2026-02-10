# Test: example_of_synchronous_qrl

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { sync$, component$ } from "@qwik.dev/core";

		export default component$(() => {
		return (
			<>
				<input onClick$={sync$(function(event, target) {
					// comment should be removed
					event.preventDefault();
				})}/>
				<input onClick$={sync$((event, target) => {
					event.preventDefault();
				})}/>
				<input onClick$={sync$((event, target) => event.preventDefault())}/>
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
            "name": "sync$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 14
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "sync$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 14
          },
          "importKind": "value",
          "start": 9,
          "end": 14
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 16,
            "end": 26
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 16,
            "end": 26
          },
          "importKind": "value",
          "start": 16,
          "end": 26
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 34,
        "end": 50
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 51
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "decorators": [],
          "name": "component$",
          "optional": false,
          "typeAnnotation": null,
          "start": 70,
          "end": 80
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
                      "type": "JSXFragment",
                      "openingFragment": {
                        "type": "JSXOpeningFragment",
                        "start": 103,
                        "end": 105
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t\t",
                          "raw": "\n\t\t\t\t",
                          "start": 105,
                          "end": 110
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "input",
                              "start": 111,
                              "end": 116
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick$",
                                  "start": 117,
                                  "end": 125
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "sync$",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 127,
                                      "end": 132
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
                                            "name": "event",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 142,
                                            "end": 147
                                          },
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "target",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 149,
                                            "end": 155
                                          }
                                        ],
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
                                                    "name": "event",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 198,
                                                    "end": 203
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "preventDefault",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 204,
                                                    "end": 218
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 198,
                                                  "end": 218
                                                },
                                                "typeArguments": null,
                                                "arguments": [],
                                                "optional": false,
                                                "start": 198,
                                                "end": 220
                                              },
                                              "directive": null,
                                              "start": 198,
                                              "end": 221
                                            }
                                          ],
                                          "start": 157,
                                          "end": 227
                                        },
                                        "expression": false,
                                        "start": 133,
                                        "end": 227
                                      }
                                    ],
                                    "optional": false,
                                    "start": 127,
                                    "end": 228
                                  },
                                  "start": 126,
                                  "end": 229
                                },
                                "start": 117,
                                "end": 229
                              }
                            ],
                            "selfClosing": true,
                            "start": 110,
                            "end": 231
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 110,
                          "end": 231
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t\t",
                          "raw": "\n\t\t\t\t",
                          "start": 231,
                          "end": 236
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "input",
                              "start": 237,
                              "end": 242
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick$",
                                  "start": 243,
                                  "end": 251
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "sync$",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 253,
                                      "end": 258
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
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "event",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 260,
                                            "end": 265
                                          },
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "target",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 267,
                                            "end": 273
                                          }
                                        ],
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
                                                    "name": "event",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 285,
                                                    "end": 290
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "preventDefault",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 291,
                                                    "end": 305
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 285,
                                                  "end": 305
                                                },
                                                "typeArguments": null,
                                                "arguments": [],
                                                "optional": false,
                                                "start": 285,
                                                "end": 307
                                              },
                                              "directive": null,
                                              "start": 285,
                                              "end": 308
                                            }
                                          ],
                                          "start": 278,
                                          "end": 314
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 259,
                                        "end": 314
                                      }
                                    ],
                                    "optional": false,
                                    "start": 253,
                                    "end": 315
                                  },
                                  "start": 252,
                                  "end": 316
                                },
                                "start": 243,
                                "end": 316
                              }
                            ],
                            "selfClosing": true,
                            "start": 236,
                            "end": 318
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 236,
                          "end": 318
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t\t",
                          "raw": "\n\t\t\t\t",
                          "start": 318,
                          "end": 323
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "input",
                              "start": 324,
                              "end": 329
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick$",
                                  "start": 330,
                                  "end": 338
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "sync$",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 340,
                                      "end": 345
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
                                            "name": "event",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 347,
                                            "end": 352
                                          },
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "target",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 354,
                                            "end": 360
                                          }
                                        ],
                                        "returnType": null,
                                        "body": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "event",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 365,
                                              "end": 370
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "preventDefault",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 371,
                                              "end": 385
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 365,
                                            "end": 385
                                          },
                                          "typeArguments": null,
                                          "arguments": [],
                                          "optional": false,
                                          "start": 365,
                                          "end": 387
                                        },
                                        "id": null,
                                        "generator": false,
                                        "start": 346,
                                        "end": 387
                                      }
                                    ],
                                    "optional": false,
                                    "start": 340,
                                    "end": 388
                                  },
                                  "start": 339,
                                  "end": 389
                                },
                                "start": 330,
                                "end": 389
                              }
                            ],
                            "selfClosing": true,
                            "start": 323,
                            "end": 391
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 323,
                          "end": 391
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 391,
                          "end": 395
                        }
                      ],
                      "closingFragment": {
                        "type": "JSXClosingFragment",
                        "start": 395,
                        "end": 398
                      },
                      "start": 103,
                      "end": 398
                    },
                    "start": 98,
                    "end": 402
                  },
                  "start": 91,
                  "end": 403
                }
              ],
              "start": 87,
              "end": 407
            },
            "id": null,
            "generator": false,
            "start": 81,
            "end": 407
          }
        ],
        "optional": false,
        "start": 70,
        "end": 408
      },
      "exportKind": "value",
      "start": 55,
      "end": 409
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 409
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
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
            "name": "i_LUXeXe0DQrg",
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
                "value": "./test.tsx_test_component_LUXeXe0DQrg",
                "raw": "\"./test.tsx_test_component_LUXeXe0DQrg\"",
                "start": 118,
                "end": 157
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 158
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 158
          },
          "start": 91,
          "end": 158
        }
      ],
      "start": 85,
      "end": 159
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
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
                "name": "i_LUXeXe0DQrg",
                "start": 220,
                "end": 233
              },
              {
                "type": "Literal",
                "value": "test_component_LUXeXe0DQrg",
                "raw": "\"test_component_LUXeXe0DQrg\"",
                "start": 235,
                "end": 263
              }
            ],
            "optional": false,
            "start": 216,
            "end": 264
          }
        ],
        "optional": false,
        "start": 189,
        "end": 265
      },
      "start": 160,
      "end": 266
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 266
}
```

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js [ENTRY POINT]

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { _qrlSync } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = ()=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("input", {
            "q-e:click": _qrlSync(function(event, target) {
                // comment should be removed
                event.preventDefault();
            }, "function(event,target){event.preventDefault();}")
        }, null, null, 2, null),
        /*#__PURE__*/ _jsxSorted("input", {
            "q-e:click": _qrlSync((event, target)=>{
                event.preventDefault();
            }, "(event,target)=>{event.preventDefault();}")
        }, null, null, 2, null),
        /*#__PURE__*/ _jsxSorted("input", {
            "q-e:click": _qrlSync((event, target)=>event.preventDefault(), "(event,target)=>event.preventDefault()")
        }, null, null, 2, null)
    ], 1, "u6_0");
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
            "name": "Fragment",
            "start": 9,
            "end": 17
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 21,
            "end": 30
          },
          "start": 9,
          "end": 30
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 38,
        "end": 66
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 67
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 77,
            "end": 87
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 77,
            "end": 87
          },
          "start": 77,
          "end": 87
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 95,
        "end": 111
      },
      "phase": null,
      "attributes": [],
      "start": 68,
      "end": 112
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_qrlSync",
            "start": 122,
            "end": 130
          },
          "local": {
            "type": "Identifier",
            "name": "_qrlSync",
            "start": 122,
            "end": 130
          },
          "start": 122,
          "end": 130
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 138,
        "end": 154
      },
      "phase": null,
      "attributes": [],
      "start": 113,
      "end": 155
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
              "name": "test_component_LUXeXe0DQrg",
              "start": 169,
              "end": 195
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
                        "start": 229,
                        "end": 239
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 240,
                          "end": 249
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 251,
                          "end": 255
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 257,
                          "end": 261
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 287,
                                "end": 297
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 298,
                                  "end": 305
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
                                        "start": 321,
                                        "end": 332
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_qrlSync",
                                          "start": 334,
                                          "end": 342
                                        },
                                        "arguments": [
                                          {
                                            "type": "FunctionExpression",
                                            "id": null,
                                            "generator": false,
                                            "async": false,
                                            "params": [
                                              {
                                                "type": "Identifier",
                                                "name": "event",
                                                "start": 352,
                                                "end": 357
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "target",
                                                "start": 359,
                                                "end": 365
                                              }
                                            ],
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
                                                        "name": "event",
                                                        "start": 430,
                                                        "end": 435
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "name": "preventDefault",
                                                        "start": 436,
                                                        "end": 450
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 430,
                                                      "end": 450
                                                    },
                                                    "arguments": [],
                                                    "optional": false,
                                                    "start": 430,
                                                    "end": 452
                                                  },
                                                  "start": 430,
                                                  "end": 453
                                                }
                                              ],
                                              "start": 367,
                                              "end": 467
                                            },
                                            "expression": false,
                                            "start": 343,
                                            "end": 467
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "function(event,target){event.preventDefault();}",
                                            "raw": "\"function(event,target){event.preventDefault();}\"",
                                            "start": 469,
                                            "end": 518
                                          }
                                        ],
                                        "optional": false,
                                        "start": 334,
                                        "end": 519
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 321,
                                      "end": 519
                                    }
                                  ],
                                  "start": 307,
                                  "end": 529
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 531,
                                  "end": 535
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 537,
                                  "end": 541
                                },
                                {
                                  "type": "Literal",
                                  "value": 2,
                                  "raw": "2",
                                  "start": 543,
                                  "end": 544
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 546,
                                  "end": 550
                                }
                              ],
                              "optional": false,
                              "start": 287,
                              "end": 551
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 575,
                                "end": 585
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 586,
                                  "end": 593
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
                                        "start": 609,
                                        "end": 620
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_qrlSync",
                                          "start": 622,
                                          "end": 630
                                        },
                                        "arguments": [
                                          {
                                            "type": "ArrowFunctionExpression",
                                            "expression": false,
                                            "async": false,
                                            "params": [
                                              {
                                                "type": "Identifier",
                                                "name": "event",
                                                "start": 632,
                                                "end": 637
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "target",
                                                "start": 639,
                                                "end": 645
                                              }
                                            ],
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
                                                        "name": "event",
                                                        "start": 666,
                                                        "end": 671
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "name": "preventDefault",
                                                        "start": 672,
                                                        "end": 686
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 666,
                                                      "end": 686
                                                    },
                                                    "arguments": [],
                                                    "optional": false,
                                                    "start": 666,
                                                    "end": 688
                                                  },
                                                  "start": 666,
                                                  "end": 689
                                                }
                                              ],
                                              "start": 648,
                                              "end": 703
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 631,
                                            "end": 703
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "(event,target)=>{event.preventDefault();}",
                                            "raw": "\"(event,target)=>{event.preventDefault();}\"",
                                            "start": 705,
                                            "end": 748
                                          }
                                        ],
                                        "optional": false,
                                        "start": 622,
                                        "end": 749
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 609,
                                      "end": 749
                                    }
                                  ],
                                  "start": 595,
                                  "end": 759
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 761,
                                  "end": 765
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 767,
                                  "end": 771
                                },
                                {
                                  "type": "Literal",
                                  "value": 2,
                                  "raw": "2",
                                  "start": 773,
                                  "end": 774
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 776,
                                  "end": 780
                                }
                              ],
                              "optional": false,
                              "start": 575,
                              "end": 781
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 805,
                                "end": 815
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 816,
                                  "end": 823
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
                                        "start": 839,
                                        "end": 850
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_qrlSync",
                                          "start": 852,
                                          "end": 860
                                        },
                                        "arguments": [
                                          {
                                            "type": "ArrowFunctionExpression",
                                            "expression": true,
                                            "async": false,
                                            "params": [
                                              {
                                                "type": "Identifier",
                                                "name": "event",
                                                "start": 862,
                                                "end": 867
                                              },
                                              {
                                                "type": "Identifier",
                                                "name": "target",
                                                "start": 869,
                                                "end": 875
                                              }
                                            ],
                                            "body": {
                                              "type": "CallExpression",
                                              "callee": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "Identifier",
                                                  "name": "event",
                                                  "start": 878,
                                                  "end": 883
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "name": "preventDefault",
                                                  "start": 884,
                                                  "end": 898
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 878,
                                                "end": 898
                                              },
                                              "arguments": [],
                                              "optional": false,
                                              "start": 878,
                                              "end": 900
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 861,
                                            "end": 900
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "(event,target)=>event.preventDefault()",
                                            "raw": "\"(event,target)=>event.preventDefault()\"",
                                            "start": 902,
                                            "end": 942
                                          }
                                        ],
                                        "optional": false,
                                        "start": 852,
                                        "end": 943
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 839,
                                      "end": 943
                                    }
                                  ],
                                  "start": 825,
                                  "end": 953
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 955,
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
                                  "value": 2,
                                  "raw": "2",
                                  "start": 967,
                                  "end": 968
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 970,
                                  "end": 974
                                }
                              ],
                              "optional": false,
                              "start": 805,
                              "end": 975
                            }
                          ],
                          "start": 263,
                          "end": 981
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 983,
                          "end": 984
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 986,
                          "end": 992
                        }
                      ],
                      "optional": false,
                      "start": 229,
                      "end": 993
                    },
                    "start": 208,
                    "end": 994
                  }
                ],
                "start": 202,
                "end": 996
              },
              "id": null,
              "generator": false,
              "start": 198,
              "end": 996
            },
            "start": 169,
            "end": 996
          }
        ],
        "start": 163,
        "end": 997
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 156,
      "end": 997
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 997
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null,
  "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg",
  "canonicalFilename": "test.tsx_test_component_LUXeXe0DQrg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    85,
    411
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (1 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (6 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point segment(s)
- **[CONV-13] Sync$ Serialization**: `sync$()` calls serialized via `_qrlSync()` with stringified function body (3 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.js | @qwik.dev/core | 1 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 4 |
| `_qrlSync` | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 3 |
| `Fragment` | test.tsx_test_component_LUXeXe0DQrg.js | local/scope | 1 |

## Diagnostics

No diagnostics.
