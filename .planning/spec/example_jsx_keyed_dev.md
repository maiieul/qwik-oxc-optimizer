# Test: example_jsx_keyed_dev

## Test Configuration

| Option | Value |
|--------|-------|
| Mode | `Dev` |
| Transpile Ts | `True` |
| Transpile Jsx | `True` |
| Explicit Extensions | `True` |
| Filename | `project/index.tsx` |
| Src Dir | `/src/project` |

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

### Module: `project/index.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrlDEV } from "@qwik.dev/core";
const i_KGLYFBhvJc0 = ()=>import("./index.tsx_App_component_KGLYFBhvJc0.js");
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrlDEV(i_KGLYFBhvJc0, "App_component_KGLYFBhvJc0", {
    file: "/src/project/project/index.tsx",
    lo: 88,
    hi: 283,
    displayName: "index.tsx_App_component"
}));
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
            "name": "qrlDEV",
            "start": 56,
            "end": 62
          },
          "local": {
            "type": "Identifier",
            "name": "qrlDEV",
            "start": 56,
            "end": 62
          },
          "start": 56,
          "end": 62
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 70,
        "end": 86
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 87
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_KGLYFBhvJc0",
            "start": 94,
            "end": 107
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
                "value": "./index.tsx_App_component_KGLYFBhvJc0.js",
                "raw": "\"./index.tsx_App_component_KGLYFBhvJc0.js\"",
                "start": 121,
                "end": 163
              },
              "options": null,
              "phase": null,
              "start": 114,
              "end": 164
            },
            "id": null,
            "generator": false,
            "start": 110,
            "end": 164
          },
          "start": 94,
          "end": 164
        }
      ],
      "start": 88,
      "end": 165
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
              "start": 179,
              "end": 182
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 199,
                "end": 211
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrlDEV",
                    "start": 226,
                    "end": 232
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_KGLYFBhvJc0",
                      "start": 233,
                      "end": 246
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_KGLYFBhvJc0",
                      "raw": "\"App_component_KGLYFBhvJc0\"",
                      "start": 248,
                      "end": 275
                    },
                    {
                      "type": "ObjectExpression",
                      "properties": [
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "name": "file",
                            "start": 283,
                            "end": 287
                          },
                          "value": {
                            "type": "Literal",
                            "value": "/src/project/project/index.tsx",
                            "raw": "\"/src/project/project/index.tsx\"",
                            "start": 289,
                            "end": 321
                          },
                          "method": false,
                          "shorthand": false,
                          "computed": false,
                          "start": 283,
                          "end": 321
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "name": "lo",
                            "start": 327,
                            "end": 329
                          },
                          "value": {
                            "type": "Literal",
                            "value": 88,
                            "raw": "88",
                            "start": 331,
                            "end": 333
                          },
                          "method": false,
                          "shorthand": false,
                          "computed": false,
                          "start": 327,
                          "end": 333
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "name": "hi",
                            "start": 339,
                            "end": 341
                          },
                          "value": {
                            "type": "Literal",
                            "value": 283,
                            "raw": "283",
                            "start": 343,
                            "end": 346
                          },
                          "method": false,
                          "shorthand": false,
                          "computed": false,
                          "start": 339,
                          "end": 346
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "name": "displayName",
                            "start": 352,
                            "end": 363
                          },
                          "value": {
                            "type": "Literal",
                            "value": "index.tsx_App_component",
                            "raw": "\"index.tsx_App_component\"",
                            "start": 365,
                            "end": 390
                          },
                          "method": false,
                          "shorthand": false,
                          "computed": false,
                          "start": 352,
                          "end": 390
                        }
                      ],
                      "start": 277,
                      "end": 392
                    }
                  ],
                  "optional": false,
                  "start": 226,
                  "end": 393
                }
              ],
              "optional": false,
              "start": 199,
              "end": 394
            },
            "start": 179,
            "end": 394
          }
        ],
        "start": 173,
        "end": 395
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 166,
      "end": 395
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 395
}
```

</details>

### Module: `project/index.tsx_App_component_KGLYFBhvJc0.js` (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_KGLYFBhvJc0 = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted(Cmp, null, null, null, 3, "stuff", {
            fileName: "project/index.tsx",
            lineNumber: 7,
            columnNumber: 4
        }),
        /*#__PURE__*/ _jsxSorted(Cmp, null, null, null, 3, "Q6_0", {
            fileName: "project/index.tsx",
            lineNumber: 8,
            columnNumber: 4
        }),
        /*#__PURE__*/ _jsxSorted(Cmp, null, {
            prop: "23"
        }, null, 3, "Q6_1", {
            fileName: "project/index.tsx",
            lineNumber: 9,
            columnNumber: 4
        }),
        /*#__PURE__*/ _jsxSorted(Cmp, null, {
            prop: "23"
        }, null, 3, props.stuff, {
            fileName: "project/index.tsx",
            lineNumber: 10,
            columnNumber: 4
        }),
        /*#__PURE__*/ _jsxSorted("p", null, null, "Hello Qwik", 3, props.stuff, {
            fileName: "project/index.tsx",
            lineNumber: 11,
            columnNumber: 4
        })
    ], 1, "Q6_2", {
        fileName: "project/index.tsx",
        lineNumber: 6,
        columnNumber: 3
    });
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
              "name": "App_component_KGLYFBhvJc0",
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
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "fileName",
                                        "start": 308,
                                        "end": 316
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "project/index.tsx",
                                        "raw": "\"project/index.tsx\"",
                                        "start": 318,
                                        "end": 337
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 308,
                                      "end": 337
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "lineNumber",
                                        "start": 351,
                                        "end": 361
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 7,
                                        "raw": "7",
                                        "start": 363,
                                        "end": 364
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 351,
                                      "end": 364
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "columnNumber",
                                        "start": 378,
                                        "end": 390
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 4,
                                        "raw": "4",
                                        "start": 392,
                                        "end": 393
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 378,
                                      "end": 393
                                    }
                                  ],
                                  "start": 294,
                                  "end": 403
                                }
                              ],
                              "optional": false,
                              "start": 248,
                              "end": 404
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 428,
                                "end": 438
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 439,
                                  "end": 442
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 444,
                                  "end": 448
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 450,
                                  "end": 454
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 456,
                                  "end": 460
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 462,
                                  "end": 463
                                },
                                {
                                  "type": "Literal",
                                  "value": "Q6_0",
                                  "raw": "\"Q6_0\"",
                                  "start": 465,
                                  "end": 471
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "fileName",
                                        "start": 487,
                                        "end": 495
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "project/index.tsx",
                                        "raw": "\"project/index.tsx\"",
                                        "start": 497,
                                        "end": 516
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 487,
                                      "end": 516
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "lineNumber",
                                        "start": 530,
                                        "end": 540
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 8,
                                        "raw": "8",
                                        "start": 542,
                                        "end": 543
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 530,
                                      "end": 543
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "columnNumber",
                                        "start": 557,
                                        "end": 569
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 4,
                                        "raw": "4",
                                        "start": 571,
                                        "end": 572
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 557,
                                      "end": 572
                                    }
                                  ],
                                  "start": 473,
                                  "end": 582
                                }
                              ],
                              "optional": false,
                              "start": 428,
                              "end": 583
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 607,
                                "end": 617
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 618,
                                  "end": 621
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 623,
                                  "end": 627
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
                                        "start": 643,
                                        "end": 647
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 649,
                                        "end": 653
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 643,
                                      "end": 653
                                    }
                                  ],
                                  "start": 629,
                                  "end": 663
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 665,
                                  "end": 669
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 671,
                                  "end": 672
                                },
                                {
                                  "type": "Literal",
                                  "value": "Q6_1",
                                  "raw": "\"Q6_1\"",
                                  "start": 674,
                                  "end": 680
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "fileName",
                                        "start": 696,
                                        "end": 704
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "project/index.tsx",
                                        "raw": "\"project/index.tsx\"",
                                        "start": 706,
                                        "end": 725
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 696,
                                      "end": 725
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "lineNumber",
                                        "start": 739,
                                        "end": 749
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 9,
                                        "raw": "9",
                                        "start": 751,
                                        "end": 752
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 739,
                                      "end": 752
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "columnNumber",
                                        "start": 766,
                                        "end": 778
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 4,
                                        "raw": "4",
                                        "start": 780,
                                        "end": 781
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 766,
                                      "end": 781
                                    }
                                  ],
                                  "start": 682,
                                  "end": 791
                                }
                              ],
                              "optional": false,
                              "start": 607,
                              "end": 792
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 816,
                                "end": 826
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Cmp",
                                  "start": 827,
                                  "end": 830
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 832,
                                  "end": 836
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
                                        "start": 852,
                                        "end": 856
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "23",
                                        "raw": "\"23\"",
                                        "start": 858,
                                        "end": 862
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 852,
                                      "end": 862
                                    }
                                  ],
                                  "start": 838,
                                  "end": 872
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 874,
                                  "end": 878
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 880,
                                  "end": 881
                                },
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 883,
                                    "end": 888
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "stuff",
                                    "start": 889,
                                    "end": 894
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 883,
                                  "end": 894
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "fileName",
                                        "start": 910,
                                        "end": 918
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "project/index.tsx",
                                        "raw": "\"project/index.tsx\"",
                                        "start": 920,
                                        "end": 939
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 910,
                                      "end": 939
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "lineNumber",
                                        "start": 953,
                                        "end": 963
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 10,
                                        "raw": "10",
                                        "start": 965,
                                        "end": 967
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 953,
                                      "end": 967
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "columnNumber",
                                        "start": 981,
                                        "end": 993
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 4,
                                        "raw": "4",
                                        "start": 995,
                                        "end": 996
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 981,
                                      "end": 996
                                    }
                                  ],
                                  "start": 896,
                                  "end": 1006
                                }
                              ],
                              "optional": false,
                              "start": 816,
                              "end": 1007
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1031,
                                "end": 1041
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "p",
                                  "raw": "\"p\"",
                                  "start": 1042,
                                  "end": 1045
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1047,
                                  "end": 1051
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1053,
                                  "end": 1057
                                },
                                {
                                  "type": "Literal",
                                  "value": "Hello Qwik",
                                  "raw": "\"Hello Qwik\"",
                                  "start": 1059,
                                  "end": 1071
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1073,
                                  "end": 1074
                                },
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 1076,
                                    "end": 1081
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "stuff",
                                    "start": 1082,
                                    "end": 1087
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 1076,
                                  "end": 1087
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "fileName",
                                        "start": 1103,
                                        "end": 1111
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "project/index.tsx",
                                        "raw": "\"project/index.tsx\"",
                                        "start": 1113,
                                        "end": 1132
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1103,
                                      "end": 1132
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "lineNumber",
                                        "start": 1146,
                                        "end": 1156
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 11,
                                        "raw": "11",
                                        "start": 1158,
                                        "end": 1160
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1146,
                                      "end": 1160
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "columnNumber",
                                        "start": 1174,
                                        "end": 1186
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 4,
                                        "raw": "4",
                                        "start": 1188,
                                        "end": 1189
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1174,
                                      "end": 1189
                                    }
                                  ],
                                  "start": 1089,
                                  "end": 1199
                                }
                              ],
                              "optional": false,
                              "start": 1031,
                              "end": 1200
                            }
                          ],
                          "start": 224,
                          "end": 1206
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1208,
                          "end": 1209
                        },
                        {
                          "type": "Literal",
                          "value": "Q6_2",
                          "raw": "\"Q6_2\"",
                          "start": 1211,
                          "end": 1217
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "fileName",
                                "start": 1229,
                                "end": 1237
                              },
                              "value": {
                                "type": "Literal",
                                "value": "project/index.tsx",
                                "raw": "\"project/index.tsx\"",
                                "start": 1239,
                                "end": 1258
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1229,
                              "end": 1258
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "lineNumber",
                                "start": 1268,
                                "end": 1278
                              },
                              "value": {
                                "type": "Literal",
                                "value": 6,
                                "raw": "6",
                                "start": 1280,
                                "end": 1281
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1268,
                              "end": 1281
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "columnNumber",
                                "start": 1291,
                                "end": 1303
                              },
                              "value": {
                                "type": "Literal",
                                "value": 3,
                                "raw": "3",
                                "start": 1305,
                                "end": 1306
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1291,
                              "end": 1306
                            }
                          ],
                          "start": 1219,
                          "end": 1312
                        }
                      ],
                      "optional": false,
                      "start": 190,
                      "end": 1313
                    },
                    "start": 169,
                    "end": 1314
                  }
                ],
                "start": 163,
                "end": 1316
              },
              "id": null,
              "generator": false,
              "start": 154,
              "end": 1316
            },
            "start": 126,
            "end": 1316
          }
        ],
        "start": 120,
        "end": 1317
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 113,
      "end": 1317
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1317
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "project/index.tsx",
  "name": "App_component_KGLYFBhvJc0",
  "entry": null,
  "displayName": "index.tsx_App_component",
  "hash": "KGLYFBhvJc0",
  "canonicalFilename": "index.tsx_App_component_KGLYFBhvJc0",
  "path": "project",
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

- **[CONV-01] QRL Calls**: Output uses `qrlDEV()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 8 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `project/index.js` | `@qwik.dev/core` | 2 |
| `qrlDEV` | `project/index.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `project/index.tsx_App_component_KGLYFBhvJc0.js` | `@qwik.dev/core` | 7 |

## Diagnostics

```json
[]
```
