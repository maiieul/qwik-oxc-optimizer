# Test: example_jsx_keyed

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |
| Explicit Extensions | True |

## Input

### Source Code

```tsx
import { component$, useStore } from '@qwik.dev/core';

export const App = component$((props: Stuff) => {
	return (
		<>
			<Cmp key="stuff"></Cmp>
			<Cmp></Cmp>
			<Cmp prop="23"></Cmp>
			<Cmp prop="23" key={props.stuff}></Cmp>
			<p key={props.stuff}>Hello Qwik</p>
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
        "raw": "'@qwik.dev/core'",
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
              "start": 69,
              "end": 72
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 75,
                "end": 85
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
                      "name": "props",
                      "optional": false,
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 94,
                            "end": 99
                          },
                          "typeArguments": null,
                          "start": 94,
                          "end": 99
                        },
                        "start": 92,
                        "end": 99
                      },
                      "start": 87,
                      "end": 99
                    }
                  ],
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
                              "start": 118,
                              "end": 120
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 120,
                                "end": 124
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 125,
                                    "end": 128
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "key",
                                        "start": 129,
                                        "end": 132
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "stuff",
                                        "raw": "\"stuff\"",
                                        "start": 133,
                                        "end": 140
                                      },
                                      "start": 129,
                                      "end": 140
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 124,
                                  "end": 141
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 143,
                                    "end": 146
                                  },
                                  "start": 141,
                                  "end": 147
                                },
                                "start": 124,
                                "end": 147
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 147,
                                "end": 151
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 152,
                                    "end": 155
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 151,
                                  "end": 156
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 158,
                                    "end": 161
                                  },
                                  "start": 156,
                                  "end": 162
                                },
                                "start": 151,
                                "end": 162
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 162,
                                "end": 166
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 167,
                                    "end": 170
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "prop",
                                        "start": 171,
                                        "end": 175
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 176,
                                        "end": 180
                                      },
                                      "start": 171,
                                      "end": 180
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 166,
                                  "end": 181
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 183,
                                    "end": 186
                                  },
                                  "start": 181,
                                  "end": 187
                                },
                                "start": 166,
                                "end": 187
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 187,
                                "end": 191
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 192,
                                    "end": 195
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "prop",
                                        "start": 196,
                                        "end": 200
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 201,
                                        "end": 205
                                      },
                                      "start": 196,
                                      "end": 205
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "key",
                                        "start": 206,
                                        "end": 209
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "props",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 211,
                                            "end": 216
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "stuff",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 217,
                                            "end": 222
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 211,
                                          "end": 222
                                        },
                                        "start": 210,
                                        "end": 223
                                      },
                                      "start": 206,
                                      "end": 223
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 191,
                                  "end": 224
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Cmp",
                                    "start": 226,
                                    "end": 229
                                  },
                                  "start": 224,
                                  "end": 230
                                },
                                "start": 191,
                                "end": 230
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 230,
                                "end": 234
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "p",
                                    "start": 235,
                                    "end": 236
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "key",
                                        "start": 237,
                                        "end": 240
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "props",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 242,
                                            "end": 247
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "stuff",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 248,
                                            "end": 253
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 242,
                                          "end": 253
                                        },
                                        "start": 241,
                                        "end": 254
                                      },
                                      "start": 237,
                                      "end": 254
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 234,
                                  "end": 255
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Hello Qwik",
                                    "raw": "Hello Qwik",
                                    "start": 255,
                                    "end": 265
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "p",
                                    "start": 267,
                                    "end": 268
                                  },
                                  "start": 265,
                                  "end": 269
                                },
                                "start": 234,
                                "end": 269
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 269,
                                "end": 272
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 272,
                              "end": 275
                            },
                            "start": 118,
                            "end": 275
                          },
                          "start": 114,
                          "end": 278
                        },
                        "start": 107,
                        "end": 279
                      }
                    ],
                    "start": 104,
                    "end": 281
                  },
                  "id": null,
                  "generator": false,
                  "start": 86,
                  "end": 281
                }
              ],
              "optional": false,
              "start": 75,
              "end": 282
            },
            "definite": false,
            "start": 69,
            "end": 282
          }
        ],
        "declare": false,
        "start": 63,
        "end": 283
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 56,
      "end": 283
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 283
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0.js");
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
                "value": "./test.tsx_App_component_ckEPmXZlub0.js",
                "raw": "\"./test.tsx_App_component_ckEPmXZlub0.js\"",
                "start": 118,
                "end": 159
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 160
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 160
          },
          "start": 91,
          "end": 160
        }
      ],
      "start": 85,
      "end": 161
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
              "start": 175,
              "end": 178
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 195,
                "end": 207
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 222,
                    "end": 225
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_ckEPmXZlub0",
                      "start": 226,
                      "end": 239
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 241,
                      "end": 268
                    }
                  ],
                  "optional": false,
                  "start": 222,
                  "end": 269
                }
              ],
              "optional": false,
              "start": 195,
              "end": 270
            },
            "start": 175,
            "end": 270
          }
        ],
        "start": 169,
        "end": 271
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 162,
      "end": 271
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 271
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_ckEPmXZlub0 = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted(Cmp, null, null, null, 3, "stuff"),
        /*#__PURE__*/ _jsxSorted(Cmp, null, null, null, 3, "u6_0"),
        /*#__PURE__*/ _jsxSorted(Cmp, null, {
            prop: "23"
        }, null, 3, "u6_1"),
        /*#__PURE__*/ _jsxSorted(Cmp, null, {
            prop: "23"
        }, null, 3, props.stuff),
        /*#__PURE__*/ _jsxSorted("p", null, null, "Hello Qwik", 3, props.stuff)
    ], 1, "u6_2");
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
              "start": 126,
              "end": 151
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 155,
                  "end": 160
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
                        "start": 190,
                        "end": 200
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 201,
                          "end": 210
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 212,
                          "end": 216
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 218,
                          "end": 222
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 248,
                                "end": 258
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 259,
                                  "end": 262
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 264,
                                  "end": 268
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 270,
                                  "end": 274
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 276,
                                  "end": 280
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 282,
                                  "end": 283
                                },
                                {
                                  "type": "Literal",
                                  "value": "stuff",
                                  "raw": "\"stuff\"",
                                  "start": 285,
                                  "end": 292
                                }
                              ],
                              "optional": false,
                              "start": 248,
                              "end": 293
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 317,
                                "end": 327
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 328,
                                  "end": 331
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 333,
                                  "end": 337
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 339,
                                  "end": 343
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 345,
                                  "end": 349
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 351,
                                  "end": 352
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_0",
                                  "raw": "\"u6_0\"",
                                  "start": 354,
                                  "end": 360
                                }
                              ],
                              "optional": false,
                              "start": 317,
                              "end": 361
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 385,
                                "end": 395
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 396,
                                  "end": 399
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 401,
                                  "end": 405
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "prop",
                                        "start": 421,
                                        "end": 425
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 427,
                                        "end": 431
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 421,
                                      "end": 431
                                    }
                                  ],
                                  "start": 407,
                                  "end": 441
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 443,
                                  "end": 447
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 449,
                                  "end": 450
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_1",
                                  "raw": "\"u6_1\"",
                                  "start": 452,
                                  "end": 458
                                }
                              ],
                              "optional": false,
                              "start": 385,
                              "end": 459
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 483,
                                "end": 493
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 494,
                                  "end": 497
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 499,
                                  "end": 503
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "prop",
                                        "start": 519,
                                        "end": 523
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 525,
                                        "end": 529
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 519,
                                      "end": 529
                                    }
                                  ],
                                  "start": 505,
                                  "end": 539
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 541,
                                  "end": 545
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 547,
                                  "end": 548
                                },
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 550,
                                    "end": 555
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "stuff",
                                    "start": 556,
                                    "end": 561
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 550,
                                  "end": 561
                                }
                              ],
                              "optional": false,
                              "start": 483,
                              "end": 562
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 586,
                                "end": 596
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "p",
                                  "raw": "\"p\"",
                                  "start": 597,
                                  "end": 600
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 602,
                                  "end": 606
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 608,
                                  "end": 612
                                },
                                {
                                  "type": "Literal",
                                  "value": "Hello Qwik",
                                  "raw": "\"Hello Qwik\"",
                                  "start": 614,
                                  "end": 626
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 628,
                                  "end": 629
                                },
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 631,
                                    "end": 636
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "stuff",
                                    "start": 637,
                                    "end": 642
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 631,
                                  "end": 642
                                }
                              ],
                              "optional": false,
                              "start": 586,
                              "end": 643
                            }
                          ],
                          "start": 224,
                          "end": 649
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 651,
                          "end": 652
                        },
                        {
                          "type": "Literal",
                          "value": "u6_2",
                          "raw": "\"u6_2\"",
                          "start": 654,
                          "end": 660
                        }
                      ],
                      "optional": false,
                      "start": 190,
                      "end": 661
                    },
                    "start": 169,
                    "end": 662
                  }
                ],
                "start": 163,
                "end": 664
              },
              "id": null,
              "generator": false,
              "start": 154,
              "end": 664
            },
            "start": 126,
            "end": 664
          }
        ],
        "start": 120,
        "end": 665
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 113,
      "end": 665
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 665
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
    88,
    283
  ],
  "paramNames": [
    "props"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 8 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_App_component_ckEPmXZlub0.js` | `@qwik.dev/core` | 7 |

## Diagnostics

```json
[]
```
