# Test: issue_5008

## Test Configuration

**Note:** Regression test for issue 5008 -- store array access with map using both function expression and arrow function callbacks, with key props.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, useStore } from "@qwik.dev/core";

		export default component$(() => {
		const store = useStore([{ value: 0 }]);
		return (
			<>
			<button onClick$={() => store[0].value++}>+1</button>
			{store.map(function (v, idx) {
				return <div key={"fn_" + idx}>Function: {v.value}</div>;
			})}
			{store.map((v, idx) => (
				<div key={"arrow_" + idx}>Arrow: {v.value}</div>
			))}
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 37,
        "end": 53
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 54
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
          "start": 73,
          "end": 83
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
                        "name": "store",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 100,
                        "end": 105
                      },
                      "init": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "useStore",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 108,
                          "end": 116
                        },
                        "typeArguments": null,
                        "arguments": [
                          {
                            "type": "ArrayExpression",
                            "elements": [
                              {
                                "type": "ObjectExpression",
                                "properties": [
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "value",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 120,
                                      "end": 125
                                    },
                                    "value": {
                                      "type": "Literal",
                                      "value": 0,
                                      "raw": "0",
                                      "start": 127,
                                      "end": 128
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "optional": false,
                                    "start": 120,
                                    "end": 128
                                  }
                                ],
                                "start": 118,
                                "end": 130
                              }
                            ],
                            "start": 117,
                            "end": 131
                          }
                        ],
                        "optional": false,
                        "start": 108,
                        "end": 132
                      },
                      "definite": false,
                      "start": 100,
                      "end": 132
                    }
                  ],
                  "declare": false,
                  "start": 94,
                  "end": 133
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "ParenthesizedExpression",
                    "expression": {
                      "type": "JSXFragment",
                      "openingFragment": {
                        "type": "JSXOpeningFragment",
                        "start": 148,
                        "end": 150
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 150,
                          "end": 154
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "button",
                              "start": 155,
                              "end": 161
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick$",
                                  "start": 162,
                                  "end": 170
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
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "store",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 178,
                                            "end": 183
                                          },
                                          "property": {
                                            "type": "Literal",
                                            "value": 0,
                                            "raw": "0",
                                            "start": 184,
                                            "end": 185
                                          },
                                          "optional": false,
                                          "computed": true,
                                          "start": 178,
                                          "end": 186
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 187,
                                          "end": 192
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 178,
                                        "end": 192
                                      },
                                      "start": 178,
                                      "end": 194
                                    },
                                    "id": null,
                                    "generator": false,
                                    "start": 172,
                                    "end": 194
                                  },
                                  "start": 171,
                                  "end": 195
                                },
                                "start": 162,
                                "end": 195
                              }
                            ],
                            "selfClosing": false,
                            "start": 154,
                            "end": 196
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "+1",
                              "raw": "+1",
                              "start": 196,
                              "end": 198
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "button",
                              "start": 200,
                              "end": 206
                            },
                            "start": 198,
                            "end": 207
                          },
                          "start": 154,
                          "end": 207
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 207,
                          "end": 211
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
                                "name": "store",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 212,
                                "end": 217
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "map",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 218,
                                "end": 221
                              },
                              "optional": false,
                              "computed": false,
                              "start": 212,
                              "end": 221
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
                                    "name": "v",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 232,
                                    "end": 233
                                  },
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "idx",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 235,
                                    "end": 238
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
                                            "start": 254,
                                            "end": 257
                                          },
                                          "typeArguments": null,
                                          "attributes": [
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "key",
                                                "start": 258,
                                                "end": 261
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "BinaryExpression",
                                                  "left": {
                                                    "type": "Literal",
                                                    "value": "fn_",
                                                    "raw": "\"fn_\"",
                                                    "start": 263,
                                                    "end": 268
                                                  },
                                                  "operator": "+",
                                                  "right": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "idx",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 271,
                                                    "end": 274
                                                  },
                                                  "start": 263,
                                                  "end": 274
                                                },
                                                "start": 262,
                                                "end": 275
                                              },
                                              "start": 258,
                                              "end": 275
                                            }
                                          ],
                                          "selfClosing": false,
                                          "start": 253,
                                          "end": 276
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "Function: ",
                                            "raw": "Function: ",
                                            "start": 276,
                                            "end": 286
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "v",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 287,
                                                "end": 288
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "value",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 289,
                                                "end": 294
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 287,
                                              "end": 294
                                            },
                                            "start": 286,
                                            "end": 295
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 297,
                                            "end": 300
                                          },
                                          "start": 295,
                                          "end": 301
                                        },
                                        "start": 253,
                                        "end": 301
                                      },
                                      "start": 246,
                                      "end": 302
                                    }
                                  ],
                                  "start": 240,
                                  "end": 307
                                },
                                "expression": false,
                                "start": 222,
                                "end": 307
                              }
                            ],
                            "optional": false,
                            "start": 212,
                            "end": 308
                          },
                          "start": 211,
                          "end": 309
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 309,
                          "end": 313
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
                                "name": "store",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 314,
                                "end": 319
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "map",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 320,
                                "end": 323
                              },
                              "optional": false,
                              "computed": false,
                              "start": 314,
                              "end": 323
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
                                    "name": "v",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 325,
                                    "end": 326
                                  },
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "idx",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 328,
                                    "end": 331
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
                                        "start": 343,
                                        "end": 346
                                      },
                                      "typeArguments": null,
                                      "attributes": [
                                        {
                                          "type": "JSXAttribute",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "key",
                                            "start": 347,
                                            "end": 350
                                          },
                                          "value": {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "BinaryExpression",
                                              "left": {
                                                "type": "Literal",
                                                "value": "arrow_",
                                                "raw": "\"arrow_\"",
                                                "start": 352,
                                                "end": 360
                                              },
                                              "operator": "+",
                                              "right": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "idx",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 363,
                                                "end": 366
                                              },
                                              "start": 352,
                                              "end": 366
                                            },
                                            "start": 351,
                                            "end": 367
                                          },
                                          "start": 347,
                                          "end": 367
                                        }
                                      ],
                                      "selfClosing": false,
                                      "start": 342,
                                      "end": 368
                                    },
                                    "children": [
                                      {
                                        "type": "JSXText",
                                        "value": "Arrow: ",
                                        "raw": "Arrow: ",
                                        "start": 368,
                                        "end": 375
                                      },
                                      {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "v",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 376,
                                            "end": 377
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "value",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 378,
                                            "end": 383
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 376,
                                          "end": 383
                                        },
                                        "start": 375,
                                        "end": 384
                                      }
                                    ],
                                    "closingElement": {
                                      "type": "JSXClosingElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "div",
                                        "start": 386,
                                        "end": 389
                                      },
                                      "start": 384,
                                      "end": 390
                                    },
                                    "start": 342,
                                    "end": 390
                                  },
                                  "start": 336,
                                  "end": 395
                                },
                                "id": null,
                                "generator": false,
                                "start": 324,
                                "end": 395
                              }
                            ],
                            "optional": false,
                            "start": 314,
                            "end": 396
                          },
                          "start": 313,
                          "end": 397
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 397,
                          "end": 401
                        }
                      ],
                      "closingFragment": {
                        "type": "JSXClosingFragment",
                        "start": 401,
                        "end": 404
                      },
                      "start": 148,
                      "end": 404
                    },
                    "start": 143,
                    "end": 408
                  },
                  "start": 136,
                  "end": 409
                }
              ],
              "start": 90,
              "end": 413
            },
            "id": null,
            "generator": false,
            "start": 84,
            "end": 413
          }
        ],
        "optional": false,
        "start": 73,
        "end": 414
      },
      "exportKind": "value",
      "start": 58,
      "end": 415
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 415
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

### Module: test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const test_component_Fragment_button_q_e_click_7MTd2pAiliw = ()=>{
    const store = _captures[0];
    return store[0].value++;
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
              "name": "test_component_Fragment_button_q_e_click_7MTd2pAiliw",
              "start": 57,
              "end": 109
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
                          "name": "store",
                          "start": 128,
                          "end": 133
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 136,
                            "end": 145
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 146,
                            "end": 147
                          },
                          "optional": false,
                          "computed": true,
                          "start": 136,
                          "end": 148
                        },
                        "start": 128,
                        "end": 148
                      }
                    ],
                    "start": 122,
                    "end": 149
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
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "store",
                            "start": 161,
                            "end": 166
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 167,
                            "end": 168
                          },
                          "optional": false,
                          "computed": true,
                          "start": 161,
                          "end": 169
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 170,
                          "end": 175
                        },
                        "optional": false,
                        "computed": false,
                        "start": 161,
                        "end": 175
                      },
                      "start": 161,
                      "end": 177
                    },
                    "start": 154,
                    "end": 178
                  }
                ],
                "start": 116,
                "end": 180
              },
              "id": null,
              "generator": false,
              "start": 112,
              "end": 180
            },
            "start": 57,
            "end": 180
          }
        ],
        "start": 51,
        "end": 181
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 181
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 181
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_Fragment_button_q_e_click_7MTd2pAiliw",
  "entry": null,
  "displayName": "test.tsx_test_component_Fragment_button_q_e_click",
  "hash": "7MTd2pAiliw",
  "canonicalFilename": "test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    176,
    198
  ],
  "captureNames": [
    "store"
  ]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const i_7MTd2pAiliw = ()=>import("./test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw");
export const test_component_LUXeXe0DQrg = ()=>{
    const store = useStore([
        {
            value: 0
        }
    ]);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("button", null, {
            "q-e:click": /*#__PURE__*/ qrl(i_7MTd2pAiliw, "test_component_Fragment_button_q_e_click_7MTd2pAiliw", [
                store
            ])
        }, "+1", 3, null),
        store.map(function(v, idx) {
            return /*#__PURE__*/ _jsxSorted("div", null, null, [
                "Function: ",
                _wrapProp(v)
            ], 1, "fn_" + idx);
        }),
        store.map((v, idx)=>/*#__PURE__*/ _jsxSorted("div", null, null, [
                "Arrow: ",
                _wrapProp(v)
            ], 1, "arrow_" + idx))
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
            "name": "_wrapProp",
            "start": 122,
            "end": 131
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 122,
            "end": 131
          },
          "start": 122,
          "end": 131
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 139,
        "end": 155
      },
      "phase": null,
      "attributes": [],
      "start": 113,
      "end": 156
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 166,
            "end": 169
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 166,
            "end": 169
          },
          "start": 166,
          "end": 169
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 177,
        "end": 193
      },
      "phase": null,
      "attributes": [],
      "start": 157,
      "end": 194
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 204,
            "end": 212
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 204,
            "end": 212
          },
          "start": 204,
          "end": 212
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 220,
        "end": 236
      },
      "phase": null,
      "attributes": [],
      "start": 195,
      "end": 237
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_7MTd2pAiliw",
            "start": 244,
            "end": 257
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
                "value": "./test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw",
                "raw": "\"./test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw\"",
                "start": 271,
                "end": 336
              },
              "options": null,
              "phase": null,
              "start": 264,
              "end": 337
            },
            "id": null,
            "generator": false,
            "start": 260,
            "end": 337
          },
          "start": 244,
          "end": 337
        }
      ],
      "start": 238,
      "end": 338
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
              "start": 352,
              "end": 378
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
                          "name": "store",
                          "start": 397,
                          "end": 402
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 405,
                            "end": 413
                          },
                          "arguments": [
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "value",
                                        "start": 438,
                                        "end": 443
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 445,
                                        "end": 446
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 438,
                                      "end": 446
                                    }
                                  ],
                                  "start": 424,
                                  "end": 456
                                }
                              ],
                              "start": 414,
                              "end": 462
                            }
                          ],
                          "optional": false,
                          "start": 405,
                          "end": 463
                        },
                        "start": 397,
                        "end": 463
                      }
                    ],
                    "start": 391,
                    "end": 464
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 490,
                        "end": 500
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 501,
                          "end": 510
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 512,
                          "end": 516
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 518,
                          "end": 522
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 548,
                                "end": 558
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "button",
                                  "raw": "\"button\"",
                                  "start": 559,
                                  "end": 567
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 569,
                                  "end": 573
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
                                        "start": 589,
                                        "end": 600
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "qrl",
                                          "start": 616,
                                          "end": 619
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "i_7MTd2pAiliw",
                                            "start": 620,
                                            "end": 633
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "test_component_Fragment_button_q_e_click_7MTd2pAiliw",
                                            "raw": "\"test_component_Fragment_button_q_e_click_7MTd2pAiliw\"",
                                            "start": 635,
                                            "end": 689
                                          },
                                          {
                                            "type": "ArrayExpression",
                                            "elements": [
                                              {
                                                "type": "Identifier",
                                                "name": "store",
                                                "start": 709,
                                                "end": 714
                                              }
                                            ],
                                            "start": 691,
                                            "end": 728
                                          }
                                        ],
                                        "optional": false,
                                        "start": 616,
                                        "end": 729
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 589,
                                      "end": 729
                                    }
                                  ],
                                  "start": 575,
                                  "end": 739
                                },
                                {
                                  "type": "Literal",
                                  "value": "+1",
                                  "raw": "\"+1\"",
                                  "start": 741,
                                  "end": 745
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 747,
                                  "end": 748
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 750,
                                  "end": 754
                                }
                              ],
                              "optional": false,
                              "start": 548,
                              "end": 755
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "store",
                                  "start": 765,
                                  "end": 770
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 771,
                                  "end": 774
                                },
                                "optional": false,
                                "computed": false,
                                "start": 765,
                                "end": 774
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
                                      "name": "v",
                                      "start": 784,
                                      "end": 785
                                    },
                                    {
                                      "type": "Identifier",
                                      "name": "idx",
                                      "start": 787,
                                      "end": 790
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
                                            "name": "_jsxSorted",
                                            "start": 827,
                                            "end": 837
                                          },
                                          "arguments": [
                                            {
                                              "type": "Literal",
                                              "value": "div",
                                              "raw": "\"div\"",
                                              "start": 838,
                                              "end": 843
                                            },
                                            {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 845,
                                              "end": 849
                                            },
                                            {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 851,
                                              "end": 855
                                            },
                                            {
                                              "type": "ArrayExpression",
                                              "elements": [
                                                {
                                                  "type": "Literal",
                                                  "value": "Function: ",
                                                  "raw": "\"Function: \"",
                                                  "start": 875,
                                                  "end": 887
                                                },
                                                {
                                                  "type": "CallExpression",
                                                  "callee": {
                                                    "type": "Identifier",
                                                    "name": "_wrapProp",
                                                    "start": 905,
                                                    "end": 914
                                                  },
                                                  "arguments": [
                                                    {
                                                      "type": "Identifier",
                                                      "name": "v",
                                                      "start": 915,
                                                      "end": 916
                                                    }
                                                  ],
                                                  "optional": false,
                                                  "start": 905,
                                                  "end": 917
                                                }
                                              ],
                                              "start": 857,
                                              "end": 931
                                            },
                                            {
                                              "type": "Literal",
                                              "value": 1,
                                              "raw": "1",
                                              "start": 933,
                                              "end": 934
                                            },
                                            {
                                              "type": "BinaryExpression",
                                              "left": {
                                                "type": "Literal",
                                                "value": "fn_",
                                                "raw": "\"fn_\"",
                                                "start": 936,
                                                "end": 941
                                              },
                                              "operator": "+",
                                              "right": {
                                                "type": "Identifier",
                                                "name": "idx",
                                                "start": 944,
                                                "end": 947
                                              },
                                              "start": 936,
                                              "end": 947
                                            }
                                          ],
                                          "optional": false,
                                          "start": 827,
                                          "end": 948
                                        },
                                        "start": 806,
                                        "end": 949
                                      }
                                    ],
                                    "start": 792,
                                    "end": 959
                                  },
                                  "expression": false,
                                  "start": 775,
                                  "end": 959
                                }
                              ],
                              "optional": false,
                              "start": 765,
                              "end": 960
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "store",
                                  "start": 970,
                                  "end": 975
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 976,
                                  "end": 979
                                },
                                "optional": false,
                                "computed": false,
                                "start": 970,
                                "end": 979
                              },
                              "arguments": [
                                {
                                  "type": "ArrowFunctionExpression",
                                  "expression": true,
                                  "async": false,
                                  "params": [
                                    {
                                      "type": "Identifier",
                                      "name": "v",
                                      "start": 981,
                                      "end": 982
                                    },
                                    {
                                      "type": "Identifier",
                                      "name": "idx",
                                      "start": 984,
                                      "end": 987
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_jsxSorted",
                                      "start": 1004,
                                      "end": 1014
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "div",
                                        "raw": "\"div\"",
                                        "start": 1015,
                                        "end": 1020
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1022,
                                        "end": 1026
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1028,
                                        "end": 1032
                                      },
                                      {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Literal",
                                            "value": "Arrow: ",
                                            "raw": "\"Arrow: \"",
                                            "start": 1052,
                                            "end": 1061
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "_wrapProp",
                                              "start": 1079,
                                              "end": 1088
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "v",
                                                "start": 1089,
                                                "end": 1090
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1079,
                                            "end": 1091
                                          }
                                        ],
                                        "start": 1034,
                                        "end": 1105
                                      },
                                      {
                                        "type": "Literal",
                                        "value": 1,
                                        "raw": "1",
                                        "start": 1107,
                                        "end": 1108
                                      },
                                      {
                                        "type": "BinaryExpression",
                                        "left": {
                                          "type": "Literal",
                                          "value": "arrow_",
                                          "raw": "\"arrow_\"",
                                          "start": 1110,
                                          "end": 1118
                                        },
                                        "operator": "+",
                                        "right": {
                                          "type": "Identifier",
                                          "name": "idx",
                                          "start": 1121,
                                          "end": 1124
                                        },
                                        "start": 1110,
                                        "end": 1124
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1004,
                                    "end": 1125
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 980,
                                  "end": 1125
                                }
                              ],
                              "optional": false,
                              "start": 970,
                              "end": 1126
                            }
                          ],
                          "start": 524,
                          "end": 1132
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1134,
                          "end": 1135
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 1137,
                          "end": 1143
                        }
                      ],
                      "optional": false,
                      "start": 490,
                      "end": 1144
                    },
                    "start": 469,
                    "end": 1145
                  }
                ],
                "start": 385,
                "end": 1147
              },
              "id": null,
              "generator": false,
              "start": 381,
              "end": 1147
            },
            "start": 352,
            "end": 1147
          }
        ],
        "start": 346,
        "end": 1148
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 339,
      "end": 1148
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1148
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
    88,
    417
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-04] Signal Helpers**
- **[CONV-05] Capture Patterns**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 4 |
| _wrapProp | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 2 |
| useStore | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
