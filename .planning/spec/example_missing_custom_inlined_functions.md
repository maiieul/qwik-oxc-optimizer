# Test: example_missing_custom_inlined_functions

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { component$ as Component, $ as onRender, useStore, wrap, useEffect } from '@qwik.dev/core';


export const useMemo$ = (qrt) => {
	useEffect(qrt);
};

export const App = component$((props) => {
	const state = useStore({count: 0});
	useMemo$(() => {
		console.log(state.count);
	});
	return $(() => (
		<div>{state.count}</div>
	));
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
            "name": "Component",
            "optional": false,
            "typeAnnotation": null,
            "start": 23,
            "end": 32
          },
          "importKind": "value",
          "start": 9,
          "end": 32
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 34,
            "end": 35
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "onRender",
            "optional": false,
            "typeAnnotation": null,
            "start": 39,
            "end": 47
          },
          "importKind": "value",
          "start": 34,
          "end": 47
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 49,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 49,
            "end": 57
          },
          "importKind": "value",
          "start": 49,
          "end": 57
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "wrap",
            "optional": false,
            "typeAnnotation": null,
            "start": 59,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "wrap",
            "optional": false,
            "typeAnnotation": null,
            "start": 59,
            "end": 63
          },
          "importKind": "value",
          "start": 59,
          "end": 63
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useEffect",
            "optional": false,
            "typeAnnotation": null,
            "start": 65,
            "end": 74
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useEffect",
            "optional": false,
            "typeAnnotation": null,
            "start": 65,
            "end": 74
          },
          "importKind": "value",
          "start": 65,
          "end": 74
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 82,
        "end": 98
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 99
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
              "name": "useMemo$",
              "optional": false,
              "typeAnnotation": null,
              "start": 115,
              "end": 123
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "typeParameters": null,
              "params": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "qrt",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 127,
                  "end": 130
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
                        "type": "Identifier",
                        "decorators": [],
                        "name": "useEffect",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 138,
                        "end": 147
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "qrt",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 148,
                          "end": 151
                        }
                      ],
                      "optional": false,
                      "start": 138,
                      "end": 152
                    },
                    "directive": null,
                    "start": 138,
                    "end": 153
                  }
                ],
                "start": 135,
                "end": 155
              },
              "id": null,
              "generator": false,
              "start": 126,
              "end": 155
            },
            "definite": false,
            "start": 115,
            "end": 155
          }
        ],
        "declare": false,
        "start": 109,
        "end": 156
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 102,
      "end": 156
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
              "start": 171,
              "end": 174
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 177,
                "end": 187
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
                      "typeAnnotation": null,
                      "start": 189,
                      "end": 194
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
                              "name": "state",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 208,
                              "end": 213
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 216,
                                "end": 224
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "count",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 226,
                                        "end": 231
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 233,
                                        "end": 234
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 226,
                                      "end": 234
                                    }
                                  ],
                                  "start": 225,
                                  "end": 235
                                }
                              ],
                              "optional": false,
                              "start": 216,
                              "end": 236
                            },
                            "definite": false,
                            "start": 208,
                            "end": 236
                          }
                        ],
                        "declare": false,
                        "start": 202,
                        "end": 237
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useMemo$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 239,
                            "end": 247
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
                                          "start": 258,
                                          "end": 265
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "log",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 266,
                                          "end": 269
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 258,
                                        "end": 269
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "state",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 270,
                                            "end": 275
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "count",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 276,
                                            "end": 281
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 270,
                                          "end": 281
                                        }
                                      ],
                                      "optional": false,
                                      "start": 258,
                                      "end": 282
                                    },
                                    "directive": null,
                                    "start": 258,
                                    "end": 283
                                  }
                                ],
                                "start": 254,
                                "end": 286
                              },
                              "id": null,
                              "generator": false,
                              "start": 248,
                              "end": 286
                            }
                          ],
                          "optional": false,
                          "start": 239,
                          "end": 287
                        },
                        "directive": null,
                        "start": 239,
                        "end": 288
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 297,
                            "end": 298
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "typeParameters": null,
                              "params": [],
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
                                      "start": 310,
                                      "end": 313
                                    },
                                    "typeArguments": null,
                                    "attributes": [],
                                    "selfClosing": false,
                                    "start": 309,
                                    "end": 314
                                  },
                                  "children": [
                                    {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "state",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 315,
                                          "end": 320
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "count",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 321,
                                          "end": 326
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 315,
                                        "end": 326
                                      },
                                      "start": 314,
                                      "end": 327
                                    }
                                  ],
                                  "closingElement": {
                                    "type": "JSXClosingElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "div",
                                      "start": 329,
                                      "end": 332
                                    },
                                    "start": 327,
                                    "end": 333
                                  },
                                  "start": 309,
                                  "end": 333
                                },
                                "start": 305,
                                "end": 336
                              },
                              "id": null,
                              "generator": false,
                              "start": 299,
                              "end": 336
                            }
                          ],
                          "optional": false,
                          "start": 297,
                          "end": 337
                        },
                        "start": 290,
                        "end": 338
                      }
                    ],
                    "start": 199,
                    "end": 340
                  },
                  "id": null,
                  "generator": false,
                  "start": 188,
                  "end": 340
                }
              ],
              "optional": false,
              "start": 177,
              "end": 341
            },
            "definite": false,
            "start": 171,
            "end": 341
          }
        ],
        "declare": false,
        "start": 165,
        "end": 342
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 158,
      "end": 342
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 342
}
```

</details>

## Output

### Module: test.js

```javascript
import { _wrapProp } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { useStore, useEffect } from '@qwik.dev/core';
export const useMemo$ = (qrt)=>{
    useEffect(qrt);
};
export const App = component$((props)=>{
    const state = useStore({
        count: 0
    });
    useMemo$(()=>{
        console.log(state.count);
    });
    return $(()=>/*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(state, "count"), 3, "u6_0"));
});
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
            "name": "_wrapProp",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "start": 53,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 88
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 98,
            "end": 106
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 98,
            "end": 106
          },
          "start": 98,
          "end": 106
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useEffect",
            "start": 108,
            "end": 117
          },
          "local": {
            "type": "Identifier",
            "name": "useEffect",
            "start": 108,
            "end": 117
          },
          "start": 108,
          "end": 117
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 125,
        "end": 141
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 142
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
              "name": "useMemo$",
              "start": 156,
              "end": 164
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "qrt",
                  "start": 168,
                  "end": 171
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
                        "type": "Identifier",
                        "name": "useEffect",
                        "start": 180,
                        "end": 189
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "qrt",
                          "start": 190,
                          "end": 193
                        }
                      ],
                      "optional": false,
                      "start": 180,
                      "end": 194
                    },
                    "start": 180,
                    "end": 195
                  }
                ],
                "start": 174,
                "end": 197
              },
              "id": null,
              "generator": false,
              "start": 167,
              "end": 197
            },
            "start": 156,
            "end": 197
          }
        ],
        "start": 150,
        "end": 198
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 143,
      "end": 198
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
              "start": 212,
              "end": 215
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "component$",
                "start": 218,
                "end": 228
              },
              "arguments": [
                {
                  "type": "ArrowFunctionExpression",
                  "expression": false,
                  "async": false,
                  "params": [
                    {
                      "type": "Identifier",
                      "name": "props",
                      "start": 230,
                      "end": 235
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
                              "name": "state",
                              "start": 250,
                              "end": 255
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "useStore",
                                "start": 258,
                                "end": 266
                              },
                              "arguments": [
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "count",
                                        "start": 277,
                                        "end": 282
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 284,
                                        "end": 285
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 277,
                                      "end": 285
                                    }
                                  ],
                                  "start": 267,
                                  "end": 291
                                }
                              ],
                              "optional": false,
                              "start": 258,
                              "end": 292
                            },
                            "start": 250,
                            "end": 292
                          }
                        ],
                        "start": 244,
                        "end": 293
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useMemo$",
                            "start": 298,
                            "end": 306
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
                                    "type": "ExpressionStatement",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "console",
                                          "start": 321,
                                          "end": 328
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "log",
                                          "start": 329,
                                          "end": 332
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 321,
                                        "end": 332
                                      },
                                      "arguments": [
                                        {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "name": "state",
                                            "start": 333,
                                            "end": 338
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "count",
                                            "start": 339,
                                            "end": 344
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 333,
                                          "end": 344
                                        }
                                      ],
                                      "optional": false,
                                      "start": 321,
                                      "end": 345
                                    },
                                    "start": 321,
                                    "end": 346
                                  }
                                ],
                                "start": 311,
                                "end": 352
                              },
                              "id": null,
                              "generator": false,
                              "start": 307,
                              "end": 352
                            }
                          ],
                          "optional": false,
                          "start": 298,
                          "end": 353
                        },
                        "start": 298,
                        "end": 354
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "$",
                            "start": 366,
                            "end": 367
                          },
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "params": [],
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_jsxSorted",
                                  "start": 386,
                                  "end": 396
                                },
                                "arguments": [
                                  {
                                    "type": "Literal",
                                    "value": "div",
                                    "raw": "\"div\"",
                                    "start": 397,
                                    "end": 402
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 404,
                                    "end": 408
                                  },
                                  {
                                    "type": "Literal",
                                    "value": null,
                                    "raw": "null",
                                    "start": 410,
                                    "end": 414
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_wrapProp",
                                      "start": 416,
                                      "end": 425
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "state",
                                        "start": 426,
                                        "end": 431
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "count",
                                        "raw": "\"count\"",
                                        "start": 433,
                                        "end": 440
                                      }
                                    ],
                                    "optional": false,
                                    "start": 416,
                                    "end": 441
                                  },
                                  {
                                    "type": "Literal",
                                    "value": 3,
                                    "raw": "3",
                                    "start": 443,
                                    "end": 444
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "u6_0",
                                    "raw": "\"u6_0\"",
                                    "start": 446,
                                    "end": 452
                                  }
                                ],
                                "optional": false,
                                "start": 386,
                                "end": 453
                              },
                              "id": null,
                              "generator": false,
                              "start": 368,
                              "end": 453
                            }
                          ],
                          "optional": false,
                          "start": 366,
                          "end": 454
                        },
                        "start": 359,
                        "end": 455
                      }
                    ],
                    "start": 238,
                    "end": 457
                  },
                  "id": null,
                  "generator": false,
                  "start": 229,
                  "end": 457
                }
              ],
              "optional": false,
              "start": 218,
              "end": 458
            },
            "start": 212,
            "end": 458
          }
        ],
        "start": 206,
        "end": 459
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 199,
      "end": 459
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 459
}
```

</details>

## Conventions Applied

- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (1 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_jsxSorted` | test.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.js | @qwik.dev/core | 1 |
| `useStore` | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[
  {
    "category": "error",
    "code": "C05",
    "file": "test.tsx",
    "message": "Found 'useMemo$' but did not find the corresponding 'useMemoQrl' exported in the same file. Please check that it is exported and spelled correctly",
    "highlights": [
      {
        "lo": 241,
        "hi": 249,
        "startLine": 11,
        "startCol": 5,
        "endLine": 11,
        "endCol": 12
      }
    ],
    "suggestions": null,
    "scope": "optimizer"
  }
]
```
