# Test: example_invalid_segment_expr1

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { $, component$, useStyles$ } from '@qwik.dev/core';
import css1 from './global.css';
import css2 from './style.css';

export const App = component$(() => {
	const style = `${css1}${css2}`;
	useStyles$(style);
	const render = () => {
		return (
			<div></div>
		)
	};
	return $(render);
})
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
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 10
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 10
          },
          "importKind": "value",
          "start": 9,
          "end": 10
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 12,
            "end": 22
          },
          "importKind": "value",
          "start": 12,
          "end": 22
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 34
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 34
          },
          "importKind": "value",
          "start": 24,
          "end": 34
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 42,
        "end": 58
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 59
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "css1",
            "optional": false,
            "typeAnnotation": null,
            "start": 67,
            "end": 71
          },
          "start": 67,
          "end": 71
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./global.css",
        "raw": "'./global.css'",
        "start": 77,
        "end": 91
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 60,
      "end": 92
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "css2",
            "optional": false,
            "typeAnnotation": null,
            "start": 100,
            "end": 104
          },
          "start": 100,
          "end": 104
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./style.css",
        "raw": "'./style.css'",
        "start": 110,
        "end": 123
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 93,
      "end": 124
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
              "start": 139,
              "end": 142
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 145,
                "end": 155
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
                              "name": "style",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 171,
                              "end": 176
                            },
                            "init": {
                              "type": "TemplateLiteral",
                              "quasis": [
                                {
                                  "type": "TemplateElement",
                                  "value": {
                                    "raw": "",
                                    "cooked": ""
                                  },
                                  "tail": false,
                                  "start": 179,
                                  "end": 182
                                },
                                {
                                  "type": "TemplateElement",
                                  "value": {
                                    "raw": "",
                                    "cooked": ""
                                  },
                                  "tail": false,
                                  "start": 186,
                                  "end": 189
                                },
                                {
                                  "type": "TemplateElement",
                                  "value": {
                                    "raw": "",
                                    "cooked": ""
                                  },
                                  "tail": true,
                                  "start": 193,
                                  "end": 195
                                }
                              ],
                              "expressions": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "css1",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 182,
                                  "end": 186
                                },
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "css2",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 189,
                                  "end": 193
                                }
                              ],
                              "start": 179,
                              "end": 195
                            },
                            "definite": false,
                            "start": 171,
                            "end": 195
                          }
                        ],
                        "declare": false,
                        "start": 165,
                        "end": 196
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useStyles$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 198,
                            "end": 208
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "style",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 209,
                              "end": 214
                            }
                          ],
                          "optional": false,
                          "start": 198,
                          "end": 215
                        },
                        "directive": null,
                        "start": 198,
                        "end": 216
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
                              "name": "render",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 224,
                              "end": 230
                            },
                            "init": {
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
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 256,
                                            "end": 259
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 255,
                                          "end": 260
                                        },
                                        "children": [],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 262,
                                            "end": 265
                                          },
                                          "start": 260,
                                          "end": 266
                                        },
                                        "start": 255,
                                        "end": 266
                                      },
                                      "start": 250,
                                      "end": 270
                                    },
                                    "start": 243,
                                    "end": 270
                                  }
                                ],
                                "start": 239,
                                "end": 273
                              },
                              "id": null,
                              "generator": false,
                              "start": 233,
                              "end": 273
                            },
                            "definite": false,
                            "start": 224,
                            "end": 273
                          }
                        ],
                        "declare": false,
                        "start": 218,
                        "end": 274
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
                            "start": 283,
                            "end": 284
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "render",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 285,
                              "end": 291
                            }
                          ],
                          "optional": false,
                          "start": 283,
                          "end": 292
                        },
                        "start": 276,
                        "end": 293
                      }
                    ],
                    "start": 162,
                    "end": 295
                  },
                  "id": null,
                  "generator": false,
                  "start": 156,
                  "end": 295
                }
              ],
              "optional": false,
              "start": 145,
              "end": 296
            },
            "definite": false,
            "start": 139,
            "end": 296
          }
        ],
        "declare": false,
        "start": 133,
        "end": 296
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 126,
      "end": 296
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 297
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
  "end": 269
}
```

</details>

### Module: test.tsx_App_component_useStyles_t35nSa5UV7U.js (ENTRY POINT)

```javascript
export const App_component_useStyles_t35nSa5UV7U = style;
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
              "name": "App_component_useStyles_t35nSa5UV7U",
              "start": 13,
              "end": 48
            },
            "init": {
              "type": "Identifier",
              "name": "style",
              "start": 51,
              "end": 56
            },
            "start": 13,
            "end": 56
          }
        ],
        "start": 7,
        "end": 57
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 57
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 58
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_useStyles_t35nSa5UV7U",
  "entry": null,
  "displayName": "test.tsx_App_component_useStyles",
  "hash": "t35nSa5UV7U",
  "canonicalFilename": "test.tsx_App_component_useStyles_t35nSa5UV7U",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "useStyles$",
  "captures": false,
  "loc": [211, 216]
}
```

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import css1 from "./global.css";
import css2 from "./style.css";
import { qrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
const i_t35nSa5UV7U = ()=>import("./test.tsx_App_component_useStyles_t35nSa5UV7U");
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU");
export const App_component_ckEPmXZlub0 = ()=>{
    useStylesQrl(/*#__PURE__*/ qrl(i_t35nSa5UV7U, "App_component_useStyles_t35nSa5UV7U"));
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU");
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
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "css1",
            "start": 7,
            "end": 11
          },
          "start": 7,
          "end": 11
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./global.css",
        "raw": "\"./global.css\"",
        "start": 17,
        "end": 31
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 32
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "css2",
            "start": 40,
            "end": 44
          },
          "start": 40,
          "end": 44
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./style.css",
        "raw": "\"./style.css\"",
        "start": 50,
        "end": 63
      },
      "phase": null,
      "attributes": [],
      "start": 33,
      "end": 64
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 74,
            "end": 77
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 74,
            "end": 77
          },
          "start": 74,
          "end": 77
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 85,
        "end": 101
      },
      "phase": null,
      "attributes": [],
      "start": 65,
      "end": 102
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStylesQrl",
            "start": 112,
            "end": 124
          },
          "local": {
            "type": "Identifier",
            "name": "useStylesQrl",
            "start": 112,
            "end": 124
          },
          "start": 112,
          "end": 124
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 132,
        "end": 148
      },
      "phase": null,
      "attributes": [],
      "start": 103,
      "end": 149
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_t35nSa5UV7U",
            "start": 156,
            "end": 169
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
                "value": "./test.tsx_App_component_useStyles_t35nSa5UV7U",
                "raw": "\"./test.tsx_App_component_useStyles_t35nSa5UV7U\"",
                "start": 183,
                "end": 231
              },
              "options": null,
              "phase": null,
              "start": 176,
              "end": 232
            },
            "id": null,
            "generator": false,
            "start": 172,
            "end": 232
          },
          "start": 156,
          "end": 232
        }
      ],
      "start": 150,
      "end": 233
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
            "start": 240,
            "end": 253
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
                "start": 267,
                "end": 307
              },
              "options": null,
              "phase": null,
              "start": 260,
              "end": 308
            },
            "id": null,
            "generator": false,
            "start": 256,
            "end": 308
          },
          "start": 240,
          "end": 308
        }
      ],
      "start": 234,
      "end": 309
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
              "start": 323,
              "end": 348
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
                        "type": "Identifier",
                        "name": "useStylesQrl",
                        "start": 361,
                        "end": 373
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 388,
                            "end": 391
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_t35nSa5UV7U",
                              "start": 392,
                              "end": 405
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_useStyles_t35nSa5UV7U",
                              "raw": "\"App_component_useStyles_t35nSa5UV7U\"",
                              "start": 407,
                              "end": 444
                            }
                          ],
                          "optional": false,
                          "start": 388,
                          "end": 445
                        }
                      ],
                      "optional": false,
                      "start": 361,
                      "end": 446
                    },
                    "start": 361,
                    "end": 447
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "qrl",
                        "start": 473,
                        "end": 476
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_w0t0o3QMovU",
                          "start": 477,
                          "end": 490
                        },
                        {
                          "type": "Literal",
                          "value": "App_component_1_w0t0o3QMovU",
                          "raw": "\"App_component_1_w0t0o3QMovU\"",
                          "start": 492,
                          "end": 521
                        }
                      ],
                      "optional": false,
                      "start": 473,
                      "end": 522
                    },
                    "start": 452,
                    "end": 523
                  }
                ],
                "start": 355,
                "end": 525
              },
              "id": null,
              "generator": false,
              "start": 351,
              "end": 525
            },
            "start": 323,
            "end": 525
          }
        ],
        "start": 317,
        "end": 526
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 310,
      "end": 526
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 527
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
  "loc": [158, 297]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
export const App_component_1_w0t0o3QMovU = render;
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
              "name": "App_component_1_w0t0o3QMovU",
              "start": 13,
              "end": 40
            },
            "init": {
              "type": "Identifier",
              "name": "render",
              "start": 43,
              "end": 49
            },
            "start": 13,
            "end": 49
          }
        ],
        "start": 7,
        "end": 50
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 50
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 51
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
  "captures": false,
  "loc": [287, 293]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: `qrl()` calls used throughout for segment references
- **[CONV-02] Dollar-to-Qrl Conversion**: `component$` -> `componentQrl`, `useStyles$` -> `useStylesQrl`, `$()` -> `qrl()`
- **[CONV-06] Lazy Imports**: Lazy import constants for each extracted segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()` and `qrl()` calls
- **[CONV-08] Segment Extraction**: Component body, useStyles argument, and `$()` callback each extracted to separate entry point segments

**Note:** The `useStyles$` argument `style` is a computed template literal that captures local identifiers `css1` and `css2`. The segment for it exports just the identifier `style` (not a function), which is unusual -- this represents an invalid segment expression that the optimizer detects but still processes.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| useStylesQrl | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| qrl | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |

## Diagnostics

```json
[
  {
    "category": "error",
    "code": "C03",
    "file": "test.tsx",
    "message": "Qrl($) scope is not a function, but it's capturing local identifiers: style",
    "highlights": [{"lo": 211, "hi": 216, "startLine": 8, "startCol": 16, "endLine": 8, "endCol": 20}],
    "suggestions": null,
    "scope": "optimizer"
  },
  {
    "category": "error",
    "code": "C03",
    "file": "test.tsx",
    "message": "Qrl($) scope is not a function, but it's capturing local identifiers: render",
    "highlights": [{"lo": 287, "hi": 293, "startLine": 14, "startCol": 14, "endLine": 14, "endCol": 19}],
    "suggestions": null,
    "scope": "optimizer"
  }
]
```
