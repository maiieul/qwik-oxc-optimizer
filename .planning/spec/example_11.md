# Test: example_11

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | project/test.tsx |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
import {foo, bar as bbar} from "../state";
import * as dep2 from "dep2";
import dep3 from "dep3/something";

export const Header = component$(() => {
	return (
		<Header onClick={$((ev) => dep3(ev))}>
			{dep2.stuff()}{bbar()}
		</Header>
	);
});

export const App = component$(() => {
	return (
		<Header>{foo()}</Header>
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 30,
        "end": 46
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 47
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "importKind": "value",
          "start": 56,
          "end": 59
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "bar",
            "optional": false,
            "typeAnnotation": null,
            "start": 61,
            "end": 64
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "bbar",
            "optional": false,
            "typeAnnotation": null,
            "start": 68,
            "end": 72
          },
          "importKind": "value",
          "start": 61,
          "end": 72
        }
      ],
      "source": {
        "type": "Literal",
        "value": "../state",
        "raw": "\"../state\"",
        "start": 79,
        "end": 89
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 48,
      "end": 90
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportNamespaceSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "dep2",
            "optional": false,
            "typeAnnotation": null,
            "start": 103,
            "end": 107
          },
          "start": 98,
          "end": 107
        }
      ],
      "source": {
        "type": "Literal",
        "value": "dep2",
        "raw": "\"dep2\"",
        "start": 113,
        "end": 119
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 91,
      "end": 120
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "dep3",
            "optional": false,
            "typeAnnotation": null,
            "start": 128,
            "end": 132
          },
          "start": 128,
          "end": 132
        }
      ],
      "source": {
        "type": "Literal",
        "value": "dep3/something",
        "raw": "\"dep3/something\"",
        "start": 138,
        "end": 154
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 121,
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
              "decorators": [],
              "name": "Header",
              "optional": false,
              "typeAnnotation": null,
              "start": 170,
              "end": 176
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 179,
                "end": 189
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
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Header",
                                "start": 211,
                                "end": 217
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick",
                                    "start": 218,
                                    "end": 225
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "$",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 227,
                                        "end": 228
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
                                              "name": "ev",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 230,
                                              "end": 232
                                            }
                                          ],
                                          "returnType": null,
                                          "body": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "dep3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 237,
                                              "end": 241
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ev",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 242,
                                                "end": 244
                                              }
                                            ],
                                            "optional": false,
                                            "start": 237,
                                            "end": 245
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 229,
                                          "end": 245
                                        }
                                      ],
                                      "optional": false,
                                      "start": 227,
                                      "end": 246
                                    },
                                    "start": 226,
                                    "end": 247
                                  },
                                  "start": 218,
                                  "end": 247
                                }
                              ],
                              "selfClosing": false,
                              "start": 210,
                              "end": 248
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 248,
                                "end": 252
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
                                      "name": "dep2",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 253,
                                      "end": 257
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "stuff",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 258,
                                      "end": 263
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 253,
                                    "end": 263
                                  },
                                  "typeArguments": null,
                                  "arguments": [],
                                  "optional": false,
                                  "start": 253,
                                  "end": 265
                                },
                                "start": 252,
                                "end": 266
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "bbar",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 267,
                                    "end": 271
                                  },
                                  "typeArguments": null,
                                  "arguments": [],
                                  "optional": false,
                                  "start": 267,
                                  "end": 273
                                },
                                "start": 266,
                                "end": 274
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 274,
                                "end": 277
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Header",
                                "start": 279,
                                "end": 285
                              },
                              "start": 277,
                              "end": 286
                            },
                            "start": 210,
                            "end": 286
                          },
                          "start": 206,
                          "end": 289
                        },
                        "start": 199,
                        "end": 290
                      }
                    ],
                    "start": 196,
                    "end": 292
                  },
                  "id": null,
                  "generator": false,
                  "start": 190,
                  "end": 292
                }
              ],
              "optional": false,
              "start": 179,
              "end": 293
            },
            "definite": false,
            "start": 170,
            "end": 293
          }
        ],
        "declare": false,
        "start": 164,
        "end": 294
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 157,
      "end": 294
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
              "start": 309,
              "end": 312
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 315,
                "end": 325
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
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Header",
                                "start": 347,
                                "end": 353
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 346,
                              "end": 354
                            },
                            "children": [
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "foo",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 355,
                                    "end": 358
                                  },
                                  "typeArguments": null,
                                  "arguments": [],
                                  "optional": false,
                                  "start": 355,
                                  "end": 360
                                },
                                "start": 354,
                                "end": 361
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Header",
                                "start": 363,
                                "end": 369
                              },
                              "start": 361,
                              "end": 370
                            },
                            "start": 346,
                            "end": 370
                          },
                          "start": 342,
                          "end": 373
                        },
                        "start": 335,
                        "end": 374
                      }
                    ],
                    "start": 332,
                    "end": 376
                  },
                  "id": null,
                  "generator": false,
                  "start": 326,
                  "end": 376
                }
              ],
              "optional": false,
              "start": 315,
              "end": 377
            },
            "definite": false,
            "start": 309,
            "end": 377
          }
        ],
        "declare": false,
        "start": 303,
        "end": 378
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 296,
      "end": 378
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 378
}
```

</details>

## Output

### Module: project/test.tsx_Header_component_Header_onClick_KjD9TCNkNxY.tsx

```tsx
import dep3 from "dep3/something";
export const Header_component_Header_onClick_KjD9TCNkNxY = (ev)=>dep3(ev);
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
            "decorators": [],
            "name": "dep3",
            "optional": false,
            "typeAnnotation": null,
            "start": 7,
            "end": 11
          },
          "start": 7,
          "end": 11
        }
      ],
      "source": {
        "type": "Literal",
        "value": "dep3/something",
        "raw": "\"dep3/something\"",
        "start": 17,
        "end": 33
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 34
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
              "name": "Header_component_Header_onClick_KjD9TCNkNxY",
              "optional": false,
              "typeAnnotation": null,
              "start": 48,
              "end": 91
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "typeParameters": null,
              "params": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "ev",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 95,
                  "end": 97
                }
              ],
              "returnType": null,
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "dep3",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 100,
                  "end": 104
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "ev",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 105,
                    "end": 107
                  }
                ],
                "optional": false,
                "start": 100,
                "end": 108
              },
              "id": null,
              "generator": false,
              "start": 94,
              "end": 108
            },
            "definite": false,
            "start": 48,
            "end": 108
          }
        ],
        "declare": false,
        "start": 42,
        "end": 109
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 35,
      "end": 109
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 109
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "project/test.tsx",
  "name": "Header_component_Header_onClick_KjD9TCNkNxY",
  "entry": "entry_segments",
  "displayName": "test.tsx_Header_component_Header_onClick",
  "hash": "KjD9TCNkNxY",
  "canonicalFilename": "test.tsx_Header_component_Header_onClick_KjD9TCNkNxY",
  "path": "project",
  "extension": "tsx",
  "parent": "Header_component_UVBJuFYfvDo",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    231,
    247
  ],
  "paramNames": [
    "ev"
  ]
}
```

### Module: project/test.tsx_Header_component_UVBJuFYfvDo.tsx

```tsx
import { Header } from "./test";
import { bar as bbar } from "../state";
import * as dep2 from "dep2";
import { qrl } from "@qwik.dev/core";
const i_KjD9TCNkNxY = ()=>import("./test.tsx_Header_component_Header_onClick_KjD9TCNkNxY");
export const Header_component_UVBJuFYfvDo = ()=>{
    return <Header onClick={/*#__PURE__*/ qrl(i_KjD9TCNkNxY, "Header_component_Header_onClick_KjD9TCNkNxY")}>
			{dep2.stuff()}{bbar()}
		</Header>;
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
            "decorators": [],
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "importKind": "value",
          "start": 9,
          "end": 15
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 23,
        "end": 31
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 32
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "bar",
            "optional": false,
            "typeAnnotation": null,
            "start": 42,
            "end": 45
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "bbar",
            "optional": false,
            "typeAnnotation": null,
            "start": 49,
            "end": 53
          },
          "importKind": "value",
          "start": 42,
          "end": 53
        }
      ],
      "source": {
        "type": "Literal",
        "value": "../state",
        "raw": "\"../state\"",
        "start": 61,
        "end": 71
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 33,
      "end": 72
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportNamespaceSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "dep2",
            "optional": false,
            "typeAnnotation": null,
            "start": 85,
            "end": 89
          },
          "start": 80,
          "end": 89
        }
      ],
      "source": {
        "type": "Literal",
        "value": "dep2",
        "raw": "\"dep2\"",
        "start": 95,
        "end": 101
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 73,
      "end": 102
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 112,
            "end": 115
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 112,
            "end": 115
          },
          "importKind": "value",
          "start": 112,
          "end": 115
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 123,
        "end": 139
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 103,
      "end": 140
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
            "name": "i_KjD9TCNkNxY",
            "optional": false,
            "typeAnnotation": null,
            "start": 147,
            "end": 160
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_Header_component_Header_onClick_KjD9TCNkNxY",
                "raw": "\"./test.tsx_Header_component_Header_onClick_KjD9TCNkNxY\"",
                "start": 174,
                "end": 230
              },
              "options": null,
              "phase": null,
              "start": 167,
              "end": 231
            },
            "id": null,
            "generator": false,
            "start": 163,
            "end": 231
          },
          "definite": false,
          "start": 147,
          "end": 231
        }
      ],
      "declare": false,
      "start": 141,
      "end": 232
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
              "name": "Header_component_UVBJuFYfvDo",
              "optional": false,
              "typeAnnotation": null,
              "start": 246,
              "end": 274
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
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Header",
                          "start": 295,
                          "end": 301
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick",
                              "start": 302,
                              "end": 309
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "qrl",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 325,
                                  "end": 328
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "i_KjD9TCNkNxY",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 329,
                                    "end": 342
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Header_component_Header_onClick_KjD9TCNkNxY",
                                    "raw": "\"Header_component_Header_onClick_KjD9TCNkNxY\"",
                                    "start": 344,
                                    "end": 389
                                  }
                                ],
                                "optional": false,
                                "start": 325,
                                "end": 390
                              },
                              "start": 310,
                              "end": 391
                            },
                            "start": 302,
                            "end": 391
                          }
                        ],
                        "selfClosing": false,
                        "start": 294,
                        "end": 392
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 392,
                          "end": 396
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
                                "name": "dep2",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 397,
                                "end": 401
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "stuff",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 402,
                                "end": 407
                              },
                              "optional": false,
                              "computed": false,
                              "start": 397,
                              "end": 407
                            },
                            "typeArguments": null,
                            "arguments": [],
                            "optional": false,
                            "start": 397,
                            "end": 409
                          },
                          "start": 396,
                          "end": 410
                        },
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "bbar",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 411,
                              "end": 415
                            },
                            "typeArguments": null,
                            "arguments": [],
                            "optional": false,
                            "start": 411,
                            "end": 417
                          },
                          "start": 410,
                          "end": 418
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t",
                          "raw": "\n\t\t",
                          "start": 418,
                          "end": 421
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Header",
                          "start": 423,
                          "end": 429
                        },
                        "start": 421,
                        "end": 430
                      },
                      "start": 294,
                      "end": 430
                    },
                    "start": 287,
                    "end": 431
                  }
                ],
                "start": 281,
                "end": 433
              },
              "id": null,
              "generator": false,
              "start": 277,
              "end": 433
            },
            "definite": false,
            "start": 246,
            "end": 433
          }
        ],
        "declare": false,
        "start": 240,
        "end": 434
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 233,
      "end": 434
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 434
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "project/test.tsx",
  "name": "Header_component_UVBJuFYfvDo",
  "entry": "entry_segments",
  "displayName": "test.tsx_Header_component",
  "hash": "UVBJuFYfvDo",
  "canonicalFilename": "test.tsx_Header_component_UVBJuFYfvDo",
  "path": "project",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    192,
    294
  ]
}
```

### Module: project/test.tsx_App_component_wGkRHWXaqjs.tsx

```tsx
import { Header } from "./test";
import { foo } from "../state";
export const App_component_wGkRHWXaqjs = ()=>{
    return <Header>{foo()}</Header>;
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
            "decorators": [],
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "importKind": "value",
          "start": 9,
          "end": 15
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 23,
        "end": 31
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 32
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 42,
            "end": 45
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 42,
            "end": 45
          },
          "importKind": "value",
          "start": 42,
          "end": 45
        }
      ],
      "source": {
        "type": "Literal",
        "value": "../state",
        "raw": "\"../state\"",
        "start": 53,
        "end": 63
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 33,
      "end": 64
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
              "name": "App_component_wGkRHWXaqjs",
              "optional": false,
              "typeAnnotation": null,
              "start": 78,
              "end": 103
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
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Header",
                          "start": 124,
                          "end": 130
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 123,
                        "end": 131
                      },
                      "children": [
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "foo",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 132,
                              "end": 135
                            },
                            "typeArguments": null,
                            "arguments": [],
                            "optional": false,
                            "start": 132,
                            "end": 137
                          },
                          "start": 131,
                          "end": 138
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Header",
                          "start": 140,
                          "end": 146
                        },
                        "start": 138,
                        "end": 147
                      },
                      "start": 123,
                      "end": 147
                    },
                    "start": 116,
                    "end": 148
                  }
                ],
                "start": 110,
                "end": 150
              },
              "id": null,
              "generator": false,
              "start": 106,
              "end": 150
            },
            "definite": false,
            "start": 78,
            "end": 150
          }
        ],
        "declare": false,
        "start": 72,
        "end": 151
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 65,
      "end": 151
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 151
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "project/test.tsx",
  "name": "App_component_wGkRHWXaqjs",
  "entry": "entry_segments",
  "displayName": "test.tsx_App_component",
  "hash": "wGkRHWXaqjs",
  "canonicalFilename": "test.tsx_App_component_wGkRHWXaqjs",
  "path": "project",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    328,
    378
  ]
}
```

### Module: project/test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_UVBJuFYfvDo = ()=>import("./test.tsx_Header_component_UVBJuFYfvDo");
const i_wGkRHWXaqjs = ()=>import("./test.tsx_App_component_wGkRHWXaqjs");
export const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_UVBJuFYfvDo, "Header_component_UVBJuFYfvDo"));
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_wGkRHWXaqjs, "App_component_wGkRHWXaqjs"));
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
            "decorators": [],
            "name": "componentQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "componentQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 21
          },
          "importKind": "value",
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
      "importKind": "value",
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
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "importKind": "value",
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
      "importKind": "value",
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
            "decorators": [],
            "name": "i_UVBJuFYfvDo",
            "optional": false,
            "typeAnnotation": null,
            "start": 91,
            "end": 104
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_Header_component_UVBJuFYfvDo",
                "raw": "\"./test.tsx_Header_component_UVBJuFYfvDo\"",
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
          "definite": false,
          "start": 91,
          "end": 160
        }
      ],
      "declare": false,
      "start": 85,
      "end": 161
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
            "name": "i_wGkRHWXaqjs",
            "optional": false,
            "typeAnnotation": null,
            "start": 168,
            "end": 181
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_App_component_wGkRHWXaqjs",
                "raw": "\"./test.tsx_App_component_wGkRHWXaqjs\"",
                "start": 195,
                "end": 233
              },
              "options": null,
              "phase": null,
              "start": 188,
              "end": 234
            },
            "id": null,
            "generator": false,
            "start": 184,
            "end": 234
          },
          "definite": false,
          "start": 168,
          "end": 234
        }
      ],
      "declare": false,
      "start": 162,
      "end": 235
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
              "name": "Header",
              "optional": false,
              "typeAnnotation": null,
              "start": 249,
              "end": 255
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 272,
                "end": 284
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "qrl",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 299,
                    "end": 302
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_UVBJuFYfvDo",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 303,
                      "end": 316
                    },
                    {
                      "type": "Literal",
                      "value": "Header_component_UVBJuFYfvDo",
                      "raw": "\"Header_component_UVBJuFYfvDo\"",
                      "start": 318,
                      "end": 348
                    }
                  ],
                  "optional": false,
                  "start": 299,
                  "end": 349
                }
              ],
              "optional": false,
              "start": 272,
              "end": 350
            },
            "definite": false,
            "start": 249,
            "end": 350
          }
        ],
        "declare": false,
        "start": 243,
        "end": 351
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 236,
      "end": 351
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
              "start": 365,
              "end": 368
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 385,
                "end": 397
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "qrl",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 412,
                    "end": 415
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_wGkRHWXaqjs",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 416,
                      "end": 429
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_wGkRHWXaqjs",
                      "raw": "\"App_component_wGkRHWXaqjs\"",
                      "start": 431,
                      "end": 458
                    }
                  ],
                  "optional": false,
                  "start": 412,
                  "end": 459
                }
              ],
              "optional": false,
              "start": 385,
              "end": 460
            },
            "definite": false,
            "start": 365,
            "end": 460
          }
        ],
        "declare": false,
        "start": 359,
        "end": 461
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 352,
      "end": 461
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 461
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (5 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | project/test.tsx_Header_component_UVBJuFYfvDo.tsx | @qwik.dev/core | 1 |
| `qrl` | project/test.tsx | @qwik.dev/core | 2 |
| `componentQrl` | project/test.tsx | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
