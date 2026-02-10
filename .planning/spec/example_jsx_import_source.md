# Test: example_jsx_import_source

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |
| Explicit Extensions | `True` |

## Input

### Source Code

```tsx
/* @jsxImportSource react */

import { qwikify$ } from './qwikfy';

export const App = () => (
	<div onClick$={()=>console.log('App')}></div>
);

export const App2 = qwikify$(() => (
	<div onClick$={()=>console.log('App2')}></div>
));
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
            "name": "qwikify$",
            "optional": false,
            "typeAnnotation": null,
            "start": 39,
            "end": 47
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qwikify$",
            "optional": false,
            "typeAnnotation": null,
            "start": 39,
            "end": 47
          },
          "importKind": "value",
          "start": 39,
          "end": 47
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./qwikfy",
        "raw": "'./qwikfy'",
        "start": 55,
        "end": 65
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 30,
      "end": 66
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
              "start": 81,
              "end": 84
            },
            "init": {
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
                      "start": 97,
                      "end": 100
                    },
                    "typeArguments": null,
                    "attributes": [
                      {
                        "type": "JSXAttribute",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "onClick$",
                          "start": 101,
                          "end": 109
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
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "console",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 115,
                                  "end": 122
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "log",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 123,
                                  "end": 126
                                },
                                "optional": false,
                                "computed": false,
                                "start": 115,
                                "end": 126
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "App",
                                  "raw": "'App'",
                                  "start": 127,
                                  "end": 132
                                }
                              ],
                              "optional": false,
                              "start": 115,
                              "end": 133
                            },
                            "id": null,
                            "generator": false,
                            "start": 111,
                            "end": 133
                          },
                          "start": 110,
                          "end": 134
                        },
                        "start": 101,
                        "end": 134
                      }
                    ],
                    "selfClosing": false,
                    "start": 96,
                    "end": 135
                  },
                  "children": [],
                  "closingElement": {
                    "type": "JSXClosingElement",
                    "name": {
                      "type": "JSXIdentifier",
                      "name": "div",
                      "start": 137,
                      "end": 140
                    },
                    "start": 135,
                    "end": 141
                  },
                  "start": 96,
                  "end": 141
                },
                "start": 93,
                "end": 143
              },
              "id": null,
              "generator": false,
              "start": 87,
              "end": 143
            },
            "definite": false,
            "start": 81,
            "end": 143
          }
        ],
        "declare": false,
        "start": 75,
        "end": 144
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 68,
      "end": 144
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
              "name": "App2",
              "optional": false,
              "typeAnnotation": null,
              "start": 159,
              "end": 163
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "qwikify$",
                "optional": false,
                "typeAnnotation": null,
                "start": 166,
                "end": 174
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
                          "start": 185,
                          "end": 188
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick$",
                              "start": 189,
                              "end": 197
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
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "console",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 203,
                                      "end": 210
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "log",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 211,
                                      "end": 214
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 203,
                                    "end": 214
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "App2",
                                      "raw": "'App2'",
                                      "start": 215,
                                      "end": 221
                                    }
                                  ],
                                  "optional": false,
                                  "start": 203,
                                  "end": 222
                                },
                                "id": null,
                                "generator": false,
                                "start": 199,
                                "end": 222
                              },
                              "start": 198,
                              "end": 223
                            },
                            "start": 189,
                            "end": 223
                          }
                        ],
                        "selfClosing": false,
                        "start": 184,
                        "end": 224
                      },
                      "children": [],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 226,
                          "end": 229
                        },
                        "start": 224,
                        "end": 230
                      },
                      "start": 184,
                      "end": 230
                    },
                    "start": 181,
                    "end": 232
                  },
                  "id": null,
                  "generator": false,
                  "start": 175,
                  "end": 232
                }
              ],
              "optional": false,
              "start": 166,
              "end": 233
            },
            "definite": false,
            "start": 159,
            "end": 233
          }
        ],
        "declare": false,
        "start": 153,
        "end": 234
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 146,
      "end": 234
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 30,
  "end": 234
}
```

</details>

## Output

### Module: `test.js`

```javascript
/* @jsxImportSource react */ import { qwikifyQrl } from "./qwikfy";
import { qrl } from "@qwik.dev/core";
const i_RKJW7oCMdS4 = ()=>import("./test.tsx_App2_qwikify_RKJW7oCMdS4.js");
import { jsx as _jsx } from "react/jsx-runtime";
export const App = ()=>/*#__PURE__*/ _jsx("div", {
        onClick$: ()=>console.log('App')
    });
export const App2 = qwikifyQrl(/*#__PURE__*/ qrl(i_RKJW7oCMdS4, "App2_qwikify_RKJW7oCMdS4"));
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
            "name": "qwikifyQrl",
            "start": 38,
            "end": 48
          },
          "local": {
            "type": "Identifier",
            "name": "qwikifyQrl",
            "start": 38,
            "end": 48
          },
          "start": 38,
          "end": 48
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./qwikfy",
        "raw": "\"./qwikfy\"",
        "start": 56,
        "end": 66
      },
      "phase": null,
      "attributes": [],
      "start": 29,
      "end": 67
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 77,
            "end": 80
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 77,
            "end": 80
          },
          "start": 77,
          "end": 80
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 88,
        "end": 104
      },
      "phase": null,
      "attributes": [],
      "start": 68,
      "end": 105
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_RKJW7oCMdS4",
            "start": 112,
            "end": 125
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
                "value": "./test.tsx_App2_qwikify_RKJW7oCMdS4.js",
                "raw": "\"./test.tsx_App2_qwikify_RKJW7oCMdS4.js\"",
                "start": 139,
                "end": 179
              },
              "options": null,
              "phase": null,
              "start": 132,
              "end": 180
            },
            "id": null,
            "generator": false,
            "start": 128,
            "end": 180
          },
          "start": 112,
          "end": 180
        }
      ],
      "start": 106,
      "end": 181
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "jsx",
            "start": 191,
            "end": 194
          },
          "local": {
            "type": "Identifier",
            "name": "_jsx",
            "start": 198,
            "end": 202
          },
          "start": 191,
          "end": 202
        }
      ],
      "source": {
        "type": "Literal",
        "value": "react/jsx-runtime",
        "raw": "\"react/jsx-runtime\"",
        "start": 210,
        "end": 229
      },
      "phase": null,
      "attributes": [],
      "start": 182,
      "end": 230
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
              "start": 244,
              "end": 247
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "name": "_jsx",
                  "start": 268,
                  "end": 272
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 273,
                    "end": 278
                  },
                  {
                    "type": "ObjectExpression",
                    "properties": [
                      {
                        "type": "Property",
                        "kind": "init",
                        "key": {
                          "type": "Identifier",
                          "name": "onClick$",
                          "start": 290,
                          "end": 298
                        },
                        "value": {
                          "type": "ArrowFunctionExpression",
                          "expression": true,
                          "async": false,
                          "params": [],
                          "body": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "console",
                                "start": 304,
                                "end": 311
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "log",
                                "start": 312,
                                "end": 315
                              },
                              "optional": false,
                              "computed": false,
                              "start": 304,
                              "end": 315
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "App",
                                "raw": "'App'",
                                "start": 316,
                                "end": 321
                              }
                            ],
                            "optional": false,
                            "start": 304,
                            "end": 322
                          },
                          "id": null,
                          "generator": false,
                          "start": 300,
                          "end": 322
                        },
                        "method": false,
                        "shorthand": false,
                        "computed": false,
                        "start": 290,
                        "end": 322
                      }
                    ],
                    "start": 280,
                    "end": 328
                  }
                ],
                "optional": false,
                "start": 268,
                "end": 329
              },
              "id": null,
              "generator": false,
              "start": 250,
              "end": 329
            },
            "start": 244,
            "end": 329
          }
        ],
        "start": 238,
        "end": 330
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 231,
      "end": 330
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
              "name": "App2",
              "start": 344,
              "end": 348
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "qwikifyQrl",
                "start": 351,
                "end": 361
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 376,
                    "end": 379
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_RKJW7oCMdS4",
                      "start": 380,
                      "end": 393
                    },
                    {
                      "type": "Literal",
                      "value": "App2_qwikify_RKJW7oCMdS4",
                      "raw": "\"App2_qwikify_RKJW7oCMdS4\"",
                      "start": 395,
                      "end": 421
                    }
                  ],
                  "optional": false,
                  "start": 376,
                  "end": 422
                }
              ],
              "optional": false,
              "start": 351,
              "end": 423
            },
            "start": 344,
            "end": 423
          }
        ],
        "start": 338,
        "end": 424
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 331,
      "end": 424
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 424
}
```

</details>

### Module: `test.tsx_App2_qwikify_RKJW7oCMdS4.js` (ENTRY POINT)

```javascript
import { jsx as _jsx } from "react/jsx-runtime";
export const App2_qwikify_RKJW7oCMdS4 = ()=>/*#__PURE__*/ _jsx("div", {
        onClick$: ()=>console.log('App2')
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
            "name": "jsx",
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "name": "_jsx",
            "start": 16,
            "end": 20
          },
          "start": 9,
          "end": 20
        }
      ],
      "source": {
        "type": "Literal",
        "value": "react/jsx-runtime",
        "raw": "\"react/jsx-runtime\"",
        "start": 28,
        "end": 47
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 48
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
              "name": "App2_qwikify_RKJW7oCMdS4",
              "start": 62,
              "end": 86
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "name": "_jsx",
                  "start": 107,
                  "end": 111
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 112,
                    "end": 117
                  },
                  {
                    "type": "ObjectExpression",
                    "properties": [
                      {
                        "type": "Property",
                        "kind": "init",
                        "key": {
                          "type": "Identifier",
                          "name": "onClick$",
                          "start": 129,
                          "end": 137
                        },
                        "value": {
                          "type": "ArrowFunctionExpression",
                          "expression": true,
                          "async": false,
                          "params": [],
                          "body": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "console",
                                "start": 143,
                                "end": 150
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "log",
                                "start": 151,
                                "end": 154
                              },
                              "optional": false,
                              "computed": false,
                              "start": 143,
                              "end": 154
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "App2",
                                "raw": "'App2'",
                                "start": 155,
                                "end": 161
                              }
                            ],
                            "optional": false,
                            "start": 143,
                            "end": 162
                          },
                          "id": null,
                          "generator": false,
                          "start": 139,
                          "end": 162
                        },
                        "method": false,
                        "shorthand": false,
                        "computed": false,
                        "start": 129,
                        "end": 162
                      }
                    ],
                    "start": 119,
                    "end": 168
                  }
                ],
                "optional": false,
                "start": 107,
                "end": 169
              },
              "id": null,
              "generator": false,
              "start": 89,
              "end": 169
            },
            "start": 62,
            "end": 169
          }
        ],
        "start": 56,
        "end": 170
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 49,
      "end": 170
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 170
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App2_qwikify_RKJW7oCMdS4",
  "entry": null,
  "displayName": "test.tsx_App2_qwikify",
  "hash": "RKJW7oCMdS4",
  "canonicalFilename": "test.tsx_App2_qwikify_RKJW7oCMdS4",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "qwikify$",
  "captures": false,
  "loc": [
    177,
    234
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `qwikifyQrl()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 3 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
