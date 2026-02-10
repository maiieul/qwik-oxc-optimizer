# Test: issue_964

## Test Configuration

**Note:** Regression test for issue 964 -- generator function (function*) with yield expressions inside component$. Tests that generator syntax is preserved through transformation.

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

export const App = component$(() => {
	console.log(function*(lo: any, t: any) {
	console.log(yield (yield lo)(t.href).then((r) => r.json()));
	});

	return <p>Hello Qwik</p>;
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
              "start": 59,
              "end": 62
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 65,
                "end": 75
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
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "console",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 85,
                              "end": 92
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 93,
                              "end": 96
                            },
                            "optional": false,
                            "computed": false,
                            "start": 85,
                            "end": 96
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "FunctionExpression",
                              "id": null,
                              "generator": true,
                              "async": false,
                              "declare": false,
                              "typeParameters": null,
                              "params": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "lo",
                                  "optional": false,
                                  "typeAnnotation": {
                                    "type": "TSTypeAnnotation",
                                    "typeAnnotation": {
                                      "type": "TSAnyKeyword",
                                      "start": 111,
                                      "end": 114
                                    },
                                    "start": 109,
                                    "end": 114
                                  },
                                  "start": 107,
                                  "end": 114
                                },
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "t",
                                  "optional": false,
                                  "typeAnnotation": {
                                    "type": "TSTypeAnnotation",
                                    "typeAnnotation": {
                                      "type": "TSAnyKeyword",
                                      "start": 119,
                                      "end": 122
                                    },
                                    "start": 117,
                                    "end": 122
                                  },
                                  "start": 116,
                                  "end": 122
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
                                          "name": "console",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 127,
                                          "end": 134
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "log",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 135,
                                          "end": 138
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 127,
                                        "end": 138
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "YieldExpression",
                                          "delegate": false,
                                          "argument": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "ParenthesizedExpression",
                                                  "expression": {
                                                    "type": "YieldExpression",
                                                    "delegate": false,
                                                    "argument": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "lo",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 152,
                                                      "end": 154
                                                    },
                                                    "start": 146,
                                                    "end": 154
                                                  },
                                                  "start": 145,
                                                  "end": 155
                                                },
                                                "typeArguments": null,
                                                "arguments": [
                                                  {
                                                    "type": "MemberExpression",
                                                    "object": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "t",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 156,
                                                      "end": 157
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "href",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 158,
                                                      "end": 162
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 156,
                                                    "end": 162
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 145,
                                                "end": 163
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "then",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 164,
                                                "end": 168
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 145,
                                              "end": 168
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
                                                    "name": "r",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 170,
                                                    "end": 171
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
                                                      "name": "r",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 176,
                                                      "end": 177
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "json",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 178,
                                                      "end": 182
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 176,
                                                    "end": 182
                                                  },
                                                  "typeArguments": null,
                                                  "arguments": [],
                                                  "optional": false,
                                                  "start": 176,
                                                  "end": 184
                                                },
                                                "id": null,
                                                "generator": false,
                                                "start": 169,
                                                "end": 184
                                              }
                                            ],
                                            "optional": false,
                                            "start": 145,
                                            "end": 185
                                          },
                                          "start": 139,
                                          "end": 185
                                        }
                                      ],
                                      "optional": false,
                                      "start": 127,
                                      "end": 186
                                    },
                                    "directive": null,
                                    "start": 127,
                                    "end": 187
                                  }
                                ],
                                "start": 124,
                                "end": 190
                              },
                              "expression": false,
                              "start": 97,
                              "end": 190
                            }
                          ],
                          "optional": false,
                          "start": 85,
                          "end": 191
                        },
                        "directive": null,
                        "start": 85,
                        "end": 192
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "p",
                              "start": 203,
                              "end": 204
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 202,
                            "end": 205
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "Hello Qwik",
                              "raw": "Hello Qwik",
                              "start": 205,
                              "end": 215
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "p",
                              "start": 217,
                              "end": 218
                            },
                            "start": 215,
                            "end": 219
                          },
                          "start": 202,
                          "end": 219
                        },
                        "start": 195,
                        "end": 220
                      }
                    ],
                    "start": 82,
                    "end": 222
                  },
                  "id": null,
                  "generator": false,
                  "start": 76,
                  "end": 222
                }
              ],
              "optional": false,
              "start": 65,
              "end": 223
            },
            "definite": false,
            "start": 59,
            "end": 223
          }
        ],
        "declare": false,
        "start": 53,
        "end": 224
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 224
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 224
}

```

</details>

## Output

### Module: test.js

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

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_ckEPmXZlub0 = ()=>{
    console.log(function*(lo, t) {
        console.log((yield (yield lo)(t.href).then((r)=>r.json())));
    });
    return /*#__PURE__*/ _jsxSorted("p", null, null, "Hello Qwik", 3, "u6_0");
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
              "start": 58,
              "end": 83
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
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "console",
                          "start": 96,
                          "end": 103
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 104,
                          "end": 107
                        },
                        "optional": false,
                        "computed": false,
                        "start": 96,
                        "end": 107
                      },
                      "arguments": [
                        {
                          "type": "FunctionExpression",
                          "id": null,
                          "generator": true,
                          "async": false,
                          "params": [
                            {
                              "type": "Identifier",
                              "name": "lo",
                              "start": 118,
                              "end": 120
                            },
                            {
                              "type": "Identifier",
                              "name": "t",
                              "start": 122,
                              "end": 123
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
                                      "name": "console",
                                      "start": 135,
                                      "end": 142
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "log",
                                      "start": 143,
                                      "end": 146
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 135,
                                    "end": 146
                                  },
                                  "arguments": [
                                    {
                                      "type": "ParenthesizedExpression",
                                      "expression": {
                                        "type": "YieldExpression",
                                        "delegate": false,
                                        "argument": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "CallExpression",
                                              "callee": {
                                                "type": "ParenthesizedExpression",
                                                "expression": {
                                                  "type": "YieldExpression",
                                                  "delegate": false,
                                                  "argument": {
                                                    "type": "Identifier",
                                                    "name": "lo",
                                                    "start": 161,
                                                    "end": 163
                                                  },
                                                  "start": 155,
                                                  "end": 163
                                                },
                                                "start": 154,
                                                "end": 164
                                              },
                                              "arguments": [
                                                {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "name": "t",
                                                    "start": 165,
                                                    "end": 166
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "name": "href",
                                                    "start": 167,
                                                    "end": 171
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 165,
                                                  "end": 171
                                                }
                                              ],
                                              "optional": false,
                                              "start": 154,
                                              "end": 172
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "name": "then",
                                              "start": 173,
                                              "end": 177
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 154,
                                            "end": 177
                                          },
                                          "arguments": [
                                            {
                                              "type": "ArrowFunctionExpression",
                                              "expression": true,
                                              "async": false,
                                              "params": [
                                                {
                                                  "type": "Identifier",
                                                  "name": "r",
                                                  "start": 179,
                                                  "end": 180
                                                }
                                              ],
                                              "body": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "name": "r",
                                                    "start": 183,
                                                    "end": 184
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "name": "json",
                                                    "start": 185,
                                                    "end": 189
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 183,
                                                  "end": 189
                                                },
                                                "arguments": [],
                                                "optional": false,
                                                "start": 183,
                                                "end": 191
                                              },
                                              "id": null,
                                              "generator": false,
                                              "start": 178,
                                              "end": 191
                                            }
                                          ],
                                          "optional": false,
                                          "start": 154,
                                          "end": 192
                                        },
                                        "start": 148,
                                        "end": 192
                                      },
                                      "start": 147,
                                      "end": 193
                                    }
                                  ],
                                  "optional": false,
                                  "start": 135,
                                  "end": 194
                                },
                                "start": 135,
                                "end": 195
                              }
                            ],
                            "start": 125,
                            "end": 201
                          },
                          "expression": false,
                          "start": 108,
                          "end": 201
                        }
                      ],
                      "optional": false,
                      "start": 96,
                      "end": 202
                    },
                    "start": 96,
                    "end": 203
                  },
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
                          "type": "Literal",
                          "value": "p",
                          "raw": "\"p\"",
                          "start": 240,
                          "end": 243
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 245,
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
                          "value": "Hello Qwik",
                          "raw": "\"Hello Qwik\"",
                          "start": 257,
                          "end": 269
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 271,
                          "end": 272
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 274,
                          "end": 280
                        }
                      ],
                      "optional": false,
                      "start": 229,
                      "end": 281
                    },
                    "start": 208,
                    "end": 282
                  }
                ],
                "start": 90,
                "end": 284
              },
              "id": null,
              "generator": false,
              "start": 86,
              "end": 284
            },
            "start": 58,
            "end": 284
          }
        ],
        "start": 52,
        "end": 285
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 45,
      "end": 285
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 285
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
    78,
    224
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
