# Test: example_custom_inlined_functions

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { component$, $, useStore, wrap, useEffect } from '@qwik.dev/core';

export const useMemoQrl = (qrt) => {
	useEffect(qrt);
};

export const useMemo$ = wrap(useMemoQrl);

export const App = component$((props) => {
	const state = useStore({count: 0});
	useMemo$(() => {
		console.log(state.count);
	});
	return $(() => (
		<div>{state.count}</div>
	));
});

export const Lightweight = (props) => {
	useMemo$(() => {
		console.log(state.count);
	});
};
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
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 32
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 32
          },
          "importKind": "value",
          "start": 24,
          "end": 32
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "wrap",
            "optional": false,
            "typeAnnotation": null,
            "start": 34,
            "end": 38
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "wrap",
            "optional": false,
            "typeAnnotation": null,
            "start": 34,
            "end": 38
          },
          "importKind": "value",
          "start": 34,
          "end": 38
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useEffect",
            "optional": false,
            "typeAnnotation": null,
            "start": 40,
            "end": 49
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useEffect",
            "optional": false,
            "typeAnnotation": null,
            "start": 40,
            "end": 49
          },
          "importKind": "value",
          "start": 40,
          "end": 49
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 57,
        "end": 73
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 74
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
              "name": "useMemoQrl",
              "optional": false,
              "typeAnnotation": null,
              "start": 89,
              "end": 99
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
                  "start": 103,
                  "end": 106
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
                        "start": 114,
                        "end": 123
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "qrt",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 124,
                          "end": 127
                        }
                      ],
                      "optional": false,
                      "start": 114,
                      "end": 128
                    },
                    "directive": null,
                    "start": 114,
                    "end": 129
                  }
                ],
                "start": 111,
                "end": 131
              },
              "id": null,
              "generator": false,
              "start": 102,
              "end": 131
            },
            "definite": false,
            "start": 89,
            "end": 131
          }
        ],
        "declare": false,
        "start": 83,
        "end": 132
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 76,
      "end": 132
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
              "start": 147,
              "end": 155
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "wrap",
                "optional": false,
                "typeAnnotation": null,
                "start": 158,
                "end": 162
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "useMemoQrl",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 163,
                  "end": 173
                }
              ],
              "optional": false,
              "start": 158,
              "end": 174
            },
            "definite": false,
            "start": 147,
            "end": 174
          }
        ],
        "declare": false,
        "start": 141,
        "end": 175
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 134,
      "end": 175
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
              "start": 190,
              "end": 193
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 196,
                "end": 206
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
                      "start": 208,
                      "end": 213
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
                              "start": 227,
                              "end": 232
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 235,
                                "end": 243
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
                                        "start": 245,
                                        "end": 250
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 252,
                                        "end": 253
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 245,
                                      "end": 253
                                    }
                                  ],
                                  "start": 244,
                                  "end": 254
                                }
                              ],
                              "optional": false,
                              "start": 235,
                              "end": 255
                            },
                            "definite": false,
                            "start": 227,
                            "end": 255
                          }
                        ],
                        "declare": false,
                        "start": 221,
                        "end": 256
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
                            "start": 258,
                            "end": 266
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
                                          "start": 277,
                                          "end": 284
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "log",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 285,
                                          "end": 288
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 277,
                                        "end": 288
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
                                            "start": 289,
                                            "end": 294
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "count",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 295,
                                            "end": 300
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 289,
                                          "end": 300
                                        }
                                      ],
                                      "optional": false,
                                      "start": 277,
                                      "end": 301
                                    },
                                    "directive": null,
                                    "start": 277,
                                    "end": 302
                                  }
                                ],
                                "start": 273,
                                "end": 305
                              },
                              "id": null,
                              "generator": false,
                              "start": 267,
                              "end": 305
                            }
                          ],
                          "optional": false,
                          "start": 258,
                          "end": 306
                        },
                        "directive": null,
                        "start": 258,
                        "end": 307
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
                            "start": 316,
                            "end": 317
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
                                      "start": 329,
                                      "end": 332
                                    },
                                    "typeArguments": null,
                                    "attributes": [],
                                    "selfClosing": false,
                                    "start": 328,
                                    "end": 333
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
                                          "start": 334,
                                          "end": 339
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "count",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 340,
                                          "end": 345
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 334,
                                        "end": 345
                                      },
                                      "start": 333,
                                      "end": 346
                                    }
                                  ],
                                  "closingElement": {
                                    "type": "JSXClosingElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "div",
                                      "start": 348,
                                      "end": 351
                                    },
                                    "start": 346,
                                    "end": 352
                                  },
                                  "start": 328,
                                  "end": 352
                                },
                                "start": 324,
                                "end": 355
                              },
                              "id": null,
                              "generator": false,
                              "start": 318,
                              "end": 355
                            }
                          ],
                          "optional": false,
                          "start": 316,
                          "end": 356
                        },
                        "start": 309,
                        "end": 357
                      }
                    ],
                    "start": 218,
                    "end": 359
                  },
                  "id": null,
                  "generator": false,
                  "start": 207,
                  "end": 359
                }
              ],
              "optional": false,
              "start": 196,
              "end": 360
            },
            "definite": false,
            "start": 190,
            "end": 360
          }
        ],
        "declare": false,
        "start": 184,
        "end": 361
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 177,
      "end": 361
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
              "name": "Lightweight",
              "optional": false,
              "typeAnnotation": null,
              "start": 376,
              "end": 387
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
                  "name": "props",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 391,
                  "end": 396
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
                        "name": "useMemo$",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 404,
                        "end": 412
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
                                      "start": 423,
                                      "end": 430
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "log",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 431,
                                      "end": 434
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 423,
                                    "end": 434
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
                                        "start": 435,
                                        "end": 440
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "count",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 441,
                                        "end": 446
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 435,
                                      "end": 446
                                    }
                                  ],
                                  "optional": false,
                                  "start": 423,
                                  "end": 447
                                },
                                "directive": null,
                                "start": 423,
                                "end": 448
                              }
                            ],
                            "start": 419,
                            "end": 451
                          },
                          "id": null,
                          "generator": false,
                          "start": 413,
                          "end": 451
                        }
                      ],
                      "optional": false,
                      "start": 404,
                      "end": 452
                    },
                    "directive": null,
                    "start": 404,
                    "end": 453
                  }
                ],
                "start": 401,
                "end": 455
              },
              "id": null,
              "generator": false,
              "start": 390,
              "end": 455
            },
            "definite": false,
            "start": 376,
            "end": 455
          }
        ],
        "declare": false,
        "start": 370,
        "end": 456
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 363,
      "end": 456
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 456
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_UIcxVTQF1a8 = ()=>import("./test.tsx_Lightweight_useMemo_UIcxVTQF1a8");
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
import { wrap, useEffect } from '@qwik.dev/core';
export const useMemoQrl = (qrt)=>{
    useEffect(qrt);
};
export const useMemo$ = wrap(useMemoQrl);
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
export const Lightweight = (props)=>{
    useMemoQrl(/*#__PURE__*/ qrl(i_UIcxVTQF1a8, "Lightweight_useMemo_UIcxVTQF1a8"));
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
            "name": "i_UIcxVTQF1a8",
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
                "value": "./test.tsx_Lightweight_useMemo_UIcxVTQF1a8",
                "raw": "\"./test.tsx_Lightweight_useMemo_UIcxVTQF1a8\"",
                "start": 118,
                "end": 162
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 163
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 163
          },
          "start": 91,
          "end": 163
        }
      ],
      "start": 85,
      "end": 164
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
            "start": 171,
            "end": 184
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
                "start": 198,
                "end": 236
              },
              "options": null,
              "phase": null,
              "start": 191,
              "end": 237
            },
            "id": null,
            "generator": false,
            "start": 187,
            "end": 237
          },
          "start": 171,
          "end": 237
        }
      ],
      "start": 165,
      "end": 238
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "wrap",
            "start": 248,
            "end": 252
          },
          "local": {
            "type": "Identifier",
            "name": "wrap",
            "start": 248,
            "end": 252
          },
          "start": 248,
          "end": 252
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useEffect",
            "start": 254,
            "end": 263
          },
          "local": {
            "type": "Identifier",
            "name": "useEffect",
            "start": 254,
            "end": 263
          },
          "start": 254,
          "end": 263
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 271,
        "end": 287
      },
      "phase": null,
      "attributes": [],
      "start": 239,
      "end": 288
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
              "name": "useMemoQrl",
              "start": 302,
              "end": 312
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "qrt",
                  "start": 316,
                  "end": 319
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
                        "start": 328,
                        "end": 337
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "qrt",
                          "start": 338,
                          "end": 341
                        }
                      ],
                      "optional": false,
                      "start": 328,
                      "end": 342
                    },
                    "start": 328,
                    "end": 343
                  }
                ],
                "start": 322,
                "end": 345
              },
              "id": null,
              "generator": false,
              "start": 315,
              "end": 345
            },
            "start": 302,
            "end": 345
          }
        ],
        "start": 296,
        "end": 346
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 289,
      "end": 346
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
              "start": 360,
              "end": 368
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "wrap",
                "start": 371,
                "end": 375
              },
              "arguments": [
                {
                  "type": "Identifier",
                  "name": "useMemoQrl",
                  "start": 376,
                  "end": 386
                }
              ],
              "optional": false,
              "start": 371,
              "end": 387
            },
            "start": 360,
            "end": 387
          }
        ],
        "start": 354,
        "end": 388
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 347,
      "end": 388
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
              "start": 402,
              "end": 405
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 422,
                "end": 434
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 449,
                    "end": 452
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_ckEPmXZlub0",
                      "start": 453,
                      "end": 466
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 468,
                      "end": 495
                    }
                  ],
                  "optional": false,
                  "start": 449,
                  "end": 496
                }
              ],
              "optional": false,
              "start": 422,
              "end": 497
            },
            "start": 402,
            "end": 497
          }
        ],
        "start": 396,
        "end": 498
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 389,
      "end": 498
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
              "name": "Lightweight",
              "start": 512,
              "end": 523
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 527,
                  "end": 532
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
                        "name": "useMemoQrl",
                        "start": 541,
                        "end": 551
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 566,
                            "end": 569
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_UIcxVTQF1a8",
                              "start": 570,
                              "end": 583
                            },
                            {
                              "type": "Literal",
                              "value": "Lightweight_useMemo_UIcxVTQF1a8",
                              "raw": "\"Lightweight_useMemo_UIcxVTQF1a8\"",
                              "start": 585,
                              "end": 618
                            }
                          ],
                          "optional": false,
                          "start": 566,
                          "end": 619
                        }
                      ],
                      "optional": false,
                      "start": 541,
                      "end": 620
                    },
                    "start": 541,
                    "end": 621
                  }
                ],
                "start": 535,
                "end": 623
              },
              "id": null,
              "generator": false,
              "start": 526,
              "end": 623
            },
            "start": 512,
            "end": 623
          }
        ],
        "start": 506,
        "end": 624
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 499,
      "end": 624
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 624
}
```

</details>

### Module: test.tsx_App_component_useMemo_6Sc9KVki3Y0.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_useMemo_6Sc9KVki3Y0 = ()=>{
    const state = _captures[0];
    console.log(state.count);
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
              "name": "App_component_useMemo_6Sc9KVki3Y0",
              "start": 57,
              "end": 90
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
                          "name": "state",
                          "start": 109,
                          "end": 114
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 117,
                            "end": 126
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 127,
                            "end": 128
                          },
                          "optional": false,
                          "computed": true,
                          "start": 117,
                          "end": 129
                        },
                        "start": 109,
                        "end": 129
                      }
                    ],
                    "start": 103,
                    "end": 130
                  },
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
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "state",
                            "start": 147,
                            "end": 152
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "count",
                            "start": 153,
                            "end": 158
                          },
                          "optional": false,
                          "computed": false,
                          "start": 147,
                          "end": 158
                        }
                      ],
                      "optional": false,
                      "start": 135,
                      "end": 159
                    },
                    "start": 135,
                    "end": 160
                  }
                ],
                "start": 97,
                "end": 162
              },
              "id": null,
              "generator": false,
              "start": 93,
              "end": 162
            },
            "start": 57,
            "end": 162
          }
        ],
        "start": 51,
        "end": 163
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 163
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 163
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_useMemo_6Sc9KVki3Y0",
  "entry": null,
  "displayName": "test.tsx_App_component_useMemo",
  "hash": "6Sc9KVki3Y0",
  "canonicalFilename": "test.tsx_App_component_useMemo_6Sc9KVki3Y0",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "useMemo$",
  "captures": true,
  "loc": [
    269,
    307
  ],
  "captureNames": [
    "state"
  ]
}
```

### Module: test.tsx_Lightweight_useMemo_UIcxVTQF1a8.js (ENTRY POINT)

```javascript
export const Lightweight_useMemo_UIcxVTQF1a8 = ()=>{
    console.log(state.count);
};
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
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
              "name": "Lightweight_useMemo_UIcxVTQF1a8",
              "start": 13,
              "end": 44
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
                          "start": 57,
                          "end": 64
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 65,
                          "end": 68
                        },
                        "optional": false,
                        "computed": false,
                        "start": 57,
                        "end": 68
                      },
                      "arguments": [
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "state",
                            "start": 69,
                            "end": 74
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "count",
                            "start": 75,
                            "end": 80
                          },
                          "optional": false,
                          "computed": false,
                          "start": 69,
                          "end": 80
                        }
                      ],
                      "optional": false,
                      "start": 57,
                      "end": 81
                    },
                    "start": 57,
                    "end": 82
                  }
                ],
                "start": 51,
                "end": 84
              },
              "id": null,
              "generator": false,
              "start": 47,
              "end": 84
            },
            "start": 13,
            "end": 84
          }
        ],
        "start": 7,
        "end": 85
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 85
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 85
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Lightweight_useMemo_UIcxVTQF1a8",
  "entry": null,
  "displayName": "test.tsx_Lightweight_useMemo",
  "hash": "UIcxVTQF1a8",
  "canonicalFilename": "test.tsx_Lightweight_useMemo_UIcxVTQF1a8",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "useMemo$",
  "captures": false,
  "loc": [
    415,
    453
  ]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
import { useMemoQrl } from "./test";
import { useStore } from "@qwik.dev/core";
const i_6Sc9KVki3Y0 = ()=>import("./test.tsx_App_component_useMemo_6Sc9KVki3Y0");
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU");
export const App_component_ckEPmXZlub0 = (props)=>{
    const state = useStore({
        count: 0
    });
    useMemoQrl(/*#__PURE__*/ qrl(i_6Sc9KVki3Y0, "App_component_useMemo_6Sc9KVki3Y0", [
        state
    ]));
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU", [
        state
    ]);
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
            "name": "qrl",
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 9,
            "end": 12
          },
          "start": 9,
          "end": 12
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 20,
        "end": 36
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 37
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useMemoQrl",
            "start": 47,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "name": "useMemoQrl",
            "start": 47,
            "end": 57
          },
          "start": 47,
          "end": 57
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 65,
        "end": 73
      },
      "phase": null,
      "attributes": [],
      "start": 38,
      "end": 74
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 84,
            "end": 92
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 84,
            "end": 92
          },
          "start": 84,
          "end": 92
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 100,
        "end": 116
      },
      "phase": null,
      "attributes": [],
      "start": 75,
      "end": 117
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_6Sc9KVki3Y0",
            "start": 124,
            "end": 137
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
                "value": "./test.tsx_App_component_useMemo_6Sc9KVki3Y0",
                "raw": "\"./test.tsx_App_component_useMemo_6Sc9KVki3Y0\"",
                "start": 151,
                "end": 197
              },
              "options": null,
              "phase": null,
              "start": 144,
              "end": 198
            },
            "id": null,
            "generator": false,
            "start": 140,
            "end": 198
          },
          "start": 124,
          "end": 198
        }
      ],
      "start": 118,
      "end": 199
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_w0t0o3QMovU",
            "start": 206,
            "end": 219
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
                "value": "./test.tsx_App_component_1_w0t0o3QMovU",
                "raw": "\"./test.tsx_App_component_1_w0t0o3QMovU\"",
                "start": 233,
                "end": 273
              },
              "options": null,
              "phase": null,
              "start": 226,
              "end": 274
            },
            "id": null,
            "generator": false,
            "start": 222,
            "end": 274
          },
          "start": 206,
          "end": 274
        }
      ],
      "start": 200,
      "end": 275
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
              "start": 289,
              "end": 314
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 318,
                  "end": 323
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
                          "start": 338,
                          "end": 343
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 346,
                            "end": 354
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
                                    "start": 365,
                                    "end": 370
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 372,
                                    "end": 373
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 365,
                                  "end": 373
                                }
                              ],
                              "start": 355,
                              "end": 379
                            }
                          ],
                          "optional": false,
                          "start": 346,
                          "end": 380
                        },
                        "start": 338,
                        "end": 380
                      }
                    ],
                    "start": 332,
                    "end": 381
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "useMemoQrl",
                        "start": 386,
                        "end": 396
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 411,
                            "end": 414
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_6Sc9KVki3Y0",
                              "start": 415,
                              "end": 428
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_useMemo_6Sc9KVki3Y0",
                              "raw": "\"App_component_useMemo_6Sc9KVki3Y0\"",
                              "start": 430,
                              "end": 465
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "state",
                                  "start": 477,
                                  "end": 482
                                }
                              ],
                              "start": 467,
                              "end": 488
                            }
                          ],
                          "optional": false,
                          "start": 411,
                          "end": 489
                        }
                      ],
                      "optional": false,
                      "start": 386,
                      "end": 490
                    },
                    "start": 386,
                    "end": 491
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "qrl",
                        "start": 517,
                        "end": 520
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_w0t0o3QMovU",
                          "start": 521,
                          "end": 534
                        },
                        {
                          "type": "Literal",
                          "value": "App_component_1_w0t0o3QMovU",
                          "raw": "\"App_component_1_w0t0o3QMovU\"",
                          "start": 536,
                          "end": 565
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 577,
                              "end": 582
                            }
                          ],
                          "start": 567,
                          "end": 588
                        }
                      ],
                      "optional": false,
                      "start": 517,
                      "end": 589
                    },
                    "start": 496,
                    "end": 590
                  }
                ],
                "start": 326,
                "end": 592
              },
              "id": null,
              "generator": false,
              "start": 317,
              "end": 592
            },
            "start": 289,
            "end": 592
          }
        ],
        "start": 283,
        "end": 593
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 276,
      "end": 593
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 593
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
    209,
    361
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
export const App_component_1_w0t0o3QMovU = ()=>{
    const state = _captures[0];
    return /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(state, "count"), 3, "u6_0");
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
            "name": "_wrapProp",
            "start": 98,
            "end": 107
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 98,
            "end": 107
          },
          "start": 98,
          "end": 107
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 115,
        "end": 131
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 132
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
              "name": "App_component_1_w0t0o3QMovU",
              "start": 146,
              "end": 173
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
                          "name": "state",
                          "start": 192,
                          "end": 197
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 200,
                            "end": 209
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 210,
                            "end": 211
                          },
                          "optional": false,
                          "computed": true,
                          "start": 200,
                          "end": 212
                        },
                        "start": 192,
                        "end": 212
                      }
                    ],
                    "start": 186,
                    "end": 213
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 239,
                        "end": 249
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 250,
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
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 263,
                          "end": 267
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_wrapProp",
                            "start": 269,
                            "end": 278
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 279,
                              "end": 284
                            },
                            {
                              "type": "Literal",
                              "value": "count",
                              "raw": "\"count\"",
                              "start": 286,
                              "end": 293
                            }
                          ],
                          "optional": false,
                          "start": 269,
                          "end": 294
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 296,
                          "end": 297
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 299,
                          "end": 305
                        }
                      ],
                      "optional": false,
                      "start": 239,
                      "end": 306
                    },
                    "start": 218,
                    "end": 307
                  }
                ],
                "start": 180,
                "end": 309
              },
              "id": null,
              "generator": false,
              "start": 176,
              "end": 309
            },
            "start": 146,
            "end": 309
          }
        ],
        "start": 140,
        "end": 310
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 133,
      "end": 310
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 310
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_1_w0t0o3QMovU",
  "entry": null,
  "displayName": "test.tsx_App_component_1",
  "hash": "w0t0o3QMovU",
  "canonicalFilename": "test.tsx_App_component_1_w0t0o3QMovU",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    320,
    357
  ],
  "captureNames": [
    "state"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`, `useMemoQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (4 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (6 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 4 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.js | @qwik.dev/core | 2 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `useMemoQrl` | test.js | local/scope | 1 |
| `_captures[]` | test.tsx_App_component_useMemo_6Sc9KVki3Y0.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |
| `useStore` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| `useMemoQrl` | test.tsx_App_component_ckEPmXZlub0.js | ./test | 1 |
| `_jsxSorted` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
