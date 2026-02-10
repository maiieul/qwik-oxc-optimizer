# Test: should_split_spread_props_with_additional_prop5

## Test Configuration

**Note:** Spread props variant 5: Non-component function `Hola` with spread props, used from a component$. Tests _jsxSplit in non-component context plus _auto_ re-export.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

		function Hola(props: any) {
			return <div {...props}></div>;
		}

		export default component$(() => {
		return <Hola>
			<div>1</div>
			<div>2</div>
		</Hola>;
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
      "type": "FunctionDeclaration",
      "id": {
        "type": "Identifier",
        "decorators": [],
        "name": "Hola",
        "optional": false,
        "typeAnnotation": null,
        "start": 57,
        "end": 61
      },
      "generator": false,
      "async": false,
      "declare": false,
      "typeParameters": null,
      "params": [
        {
          "type": "Identifier",
          "decorators": [],
          "name": "props",
          "optional": false,
          "typeAnnotation": {
            "type": "TSTypeAnnotation",
            "typeAnnotation": {
              "type": "TSAnyKeyword",
              "start": 69,
              "end": 72
            },
            "start": 67,
            "end": 72
          },
          "start": 62,
          "end": 72
        }
      ],
      "returnType": null,
      "body": {
        "type": "BlockStatement",
        "body": [
          {
            "type": "ReturnStatement",
            "argument": {
              "type": "JSXElement",
              "openingElement": {
                "type": "JSXOpeningElement",
                "name": {
                  "type": "JSXIdentifier",
                  "name": "div",
                  "start": 87,
                  "end": 90
                },
                "typeArguments": null,
                "attributes": [
                  {
                    "type": "JSXSpreadAttribute",
                    "argument": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "props",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 95,
                      "end": 100
                    },
                    "start": 91,
                    "end": 101
                  }
                ],
                "selfClosing": false,
                "start": 86,
                "end": 102
              },
              "children": [],
              "closingElement": {
                "type": "JSXClosingElement",
                "name": {
                  "type": "JSXIdentifier",
                  "name": "div",
                  "start": 104,
                  "end": 107
                },
                "start": 102,
                "end": 108
              },
              "start": 86,
              "end": 108
            },
            "start": 79,
            "end": 109
          }
        ],
        "start": 74,
        "end": 113
      },
      "expression": false,
      "start": 48,
      "end": 113
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
          "start": 132,
          "end": 142
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
                    "type": "JSXElement",
                    "openingElement": {
                      "type": "JSXOpeningElement",
                      "name": {
                        "type": "JSXIdentifier",
                        "name": "Hola",
                        "start": 161,
                        "end": 165
                      },
                      "typeArguments": null,
                      "attributes": [],
                      "selfClosing": false,
                      "start": 160,
                      "end": 166
                    },
                    "children": [
                      {
                        "type": "JSXText",
                        "value": "\n\t\t\t",
                        "raw": "\n\t\t\t",
                        "start": 166,
                        "end": 170
                      },
                      {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 171,
                            "end": 174
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 170,
                          "end": 175
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "1",
                            "raw": "1",
                            "start": 175,
                            "end": 176
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 178,
                            "end": 181
                          },
                          "start": 176,
                          "end": 182
                        },
                        "start": 170,
                        "end": 182
                      },
                      {
                        "type": "JSXText",
                        "value": "\n\t\t\t",
                        "raw": "\n\t\t\t",
                        "start": 182,
                        "end": 186
                      },
                      {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 187,
                            "end": 190
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 186,
                          "end": 191
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "2",
                            "raw": "2",
                            "start": 191,
                            "end": 192
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 194,
                            "end": 197
                          },
                          "start": 192,
                          "end": 198
                        },
                        "start": 186,
                        "end": 198
                      },
                      {
                        "type": "JSXText",
                        "value": "\n\t\t",
                        "raw": "\n\t\t",
                        "start": 198,
                        "end": 201
                      }
                    ],
                    "closingElement": {
                      "type": "JSXClosingElement",
                      "name": {
                        "type": "JSXIdentifier",
                        "name": "Hola",
                        "start": 203,
                        "end": 207
                      },
                      "start": 201,
                      "end": 208
                    },
                    "start": 160,
                    "end": 208
                  },
                  "start": 153,
                  "end": 209
                }
              ],
              "start": 149,
              "end": 213
            },
            "id": null,
            "generator": false,
            "start": 143,
            "end": 213
          }
        ],
        "optional": false,
        "start": 132,
        "end": 214
      },
      "exportKind": "value",
      "start": 117,
      "end": 215
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 215
}

```

</details>

## Output

### Module: test.js

```javascript
import { _getVarProps } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
function Hola(props) {
    return /*#__PURE__*/ _jsxSplit("div", {
        ..._getVarProps(props)
    }, _getConstProps(props), null, 0, "u6_0");
}
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
export { Hola as _auto_Hola };
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
            "name": "_getVarProps",
            "start": 9,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
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
            "name": "_getConstProps",
            "start": 56,
            "end": 70
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 56,
            "end": 70
          },
          "start": 56,
          "end": 70
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 78,
        "end": 94
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 95
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 105,
            "end": 114
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 105,
            "end": 114
          },
          "start": 105,
          "end": 114
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 122,
        "end": 138
      },
      "phase": null,
      "attributes": [],
      "start": 96,
      "end": 139
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 149,
            "end": 161
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 149,
            "end": 161
          },
          "start": 149,
          "end": 161
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 169,
        "end": 185
      },
      "phase": null,
      "attributes": [],
      "start": 140,
      "end": 186
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 196,
            "end": 199
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 196,
            "end": 199
          },
          "start": 196,
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
      "start": 187,
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
            "name": "i_LUXeXe0DQrg",
            "start": 231,
            "end": 244
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
                "start": 258,
                "end": 297
              },
              "options": null,
              "phase": null,
              "start": 251,
              "end": 298
            },
            "id": null,
            "generator": false,
            "start": 247,
            "end": 298
          },
          "start": 231,
          "end": 298
        }
      ],
      "start": 225,
      "end": 299
    },
    {
      "type": "FunctionDeclaration",
      "id": {
        "type": "Identifier",
        "name": "Hola",
        "start": 309,
        "end": 313
      },
      "generator": false,
      "async": false,
      "params": [
        {
          "type": "Identifier",
          "name": "props",
          "start": 314,
          "end": 319
        }
      ],
      "body": {
        "type": "BlockStatement",
        "body": [
          {
            "type": "ReturnStatement",
            "argument": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "_jsxSplit",
                "start": 348,
                "end": 357
              },
              "arguments": [
                {
                  "type": "Literal",
                  "value": "div",
                  "raw": "\"div\"",
                  "start": 358,
                  "end": 363
                },
                {
                  "type": "ObjectExpression",
                  "properties": [
                    {
                      "type": "SpreadElement",
                      "argument": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "name": "_getVarProps",
                          "start": 378,
                          "end": 390
                        },
                        "arguments": [
                          {
                            "type": "Identifier",
                            "name": "props",
                            "start": 391,
                            "end": 396
                          }
                        ],
                        "optional": false,
                        "start": 378,
                        "end": 397
                      },
                      "start": 375,
                      "end": 397
                    }
                  ],
                  "start": 365,
                  "end": 403
                },
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "_getConstProps",
                    "start": 405,
                    "end": 419
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "props",
                      "start": 420,
                      "end": 425
                    }
                  ],
                  "optional": false,
                  "start": 405,
                  "end": 426
                },
                {
                  "type": "Literal",
                  "value": null,
                  "raw": "null",
                  "start": 428,
                  "end": 432
                },
                {
                  "type": "Literal",
                  "value": 0,
                  "raw": "0",
                  "start": 434,
                  "end": 435
                },
                {
                  "type": "Literal",
                  "value": "u6_0",
                  "raw": "\"u6_0\"",
                  "start": 437,
                  "end": 443
                }
              ],
              "optional": false,
              "start": 348,
              "end": 444
            },
            "start": 327,
            "end": 445
          }
        ],
        "start": 321,
        "end": 447
      },
      "expression": false,
      "start": 300,
      "end": 447
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 477,
          "end": 489
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 504,
              "end": 507
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_LUXeXe0DQrg",
                "start": 508,
                "end": 521
              },
              {
                "type": "Literal",
                "value": "test_component_LUXeXe0DQrg",
                "raw": "\"test_component_LUXeXe0DQrg\"",
                "start": 523,
                "end": 551
              }
            ],
            "optional": false,
            "start": 504,
            "end": 552
          }
        ],
        "optional": false,
        "start": 477,
        "end": 553
      },
      "start": 448,
      "end": 554
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "Hola",
            "start": 564,
            "end": 568
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_Hola",
            "start": 572,
            "end": 582
          },
          "start": 564,
          "end": 582
        }
      ],
      "source": null,
      "attributes": [],
      "start": 555,
      "end": 585
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 585
}

```

</details>

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _auto_Hola as Hola } from "./test";
import { _jsxSorted } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = ()=>{
    return /*#__PURE__*/ _jsxSorted(Hola, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, "1", 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, "2", 3, null)
    ], 3, "u6_1");
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
            "name": "_auto_Hola",
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "name": "Hola",
            "start": 23,
            "end": 27
          },
          "start": 9,
          "end": 27
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 35,
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
            "name": "_jsxSorted",
            "start": 54,
            "end": 64
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 54,
            "end": 64
          },
          "start": 54,
          "end": 64
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 72,
        "end": 88
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 89
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
              "start": 103,
              "end": 129
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
                        "start": 163,
                        "end": 173
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Hola",
                          "start": 174,
                          "end": 178
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 180,
                          "end": 184
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 186,
                          "end": 190
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 216,
                                "end": 226
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 227,
                                  "end": 232
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 234,
                                  "end": 238
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 240,
                                  "end": 244
                                },
                                {
                                  "type": "Literal",
                                  "value": "1",
                                  "raw": "\"1\"",
                                  "start": 246,
                                  "end": 249
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 251,
                                  "end": 252
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 254,
                                  "end": 258
                                }
                              ],
                              "optional": false,
                              "start": 216,
                              "end": 259
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 283,
                                "end": 293
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 294,
                                  "end": 299
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 301,
                                  "end": 305
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 307,
                                  "end": 311
                                },
                                {
                                  "type": "Literal",
                                  "value": "2",
                                  "raw": "\"2\"",
                                  "start": 313,
                                  "end": 316
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 318,
                                  "end": 319
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 321,
                                  "end": 325
                                }
                              ],
                              "optional": false,
                              "start": 283,
                              "end": 326
                            }
                          ],
                          "start": 192,
                          "end": 332
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 334,
                          "end": 335
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 337,
                          "end": 343
                        }
                      ],
                      "optional": false,
                      "start": 163,
                      "end": 344
                    },
                    "start": 142,
                    "end": 345
                  }
                ],
                "start": 136,
                "end": 347
              },
              "id": null,
              "generator": false,
              "start": 132,
              "end": 347
            },
            "start": 103,
            "end": 347
          }
        ],
        "start": 97,
        "end": 348
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 90,
      "end": 348
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 348
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
    147,
    217
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-04] Signal Helpers**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| _jsxSplit | test.js | @qwik.dev/core | 1 |
| _getVarProps | test.js | @qwik.dev/core | 1 |
| _getConstProps | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 3 |

## Diagnostics

```json
[]
```
