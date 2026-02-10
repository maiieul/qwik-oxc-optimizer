# Test: example_7

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

export const Header = component$(() => {
	console.log("mount");
	return (
		<div onClick={$((ctx) => console.log(ctx))}/>
	);
	});

const App = component$(() => {
	return (
		<Header/>
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
              "start": 62,
              "end": 68
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 71,
                "end": 81
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
                              "start": 91,
                              "end": 98
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 99,
                              "end": 102
                            },
                            "optional": false,
                            "computed": false,
                            "start": 91,
                            "end": 102
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "mount",
                              "raw": "\"mount\"",
                              "start": 103,
                              "end": 110
                            }
                          ],
                          "optional": false,
                          "start": 91,
                          "end": 111
                        },
                        "directive": null,
                        "start": 91,
                        "end": 112
                      },
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
                                "start": 126,
                                "end": 129
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick",
                                    "start": 130,
                                    "end": 137
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
                                        "start": 139,
                                        "end": 140
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
                                              "name": "ctx",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 142,
                                              "end": 145
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
                                                "name": "console",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 150,
                                                "end": 157
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 158,
                                                "end": 161
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 150,
                                              "end": 161
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ctx",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 162,
                                                "end": 165
                                              }
                                            ],
                                            "optional": false,
                                            "start": 150,
                                            "end": 166
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 141,
                                          "end": 166
                                        }
                                      ],
                                      "optional": false,
                                      "start": 139,
                                      "end": 167
                                    },
                                    "start": 138,
                                    "end": 168
                                  },
                                  "start": 130,
                                  "end": 168
                                }
                              ],
                              "selfClosing": true,
                              "start": 125,
                              "end": 170
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 125,
                            "end": 170
                          },
                          "start": 121,
                          "end": 173
                        },
                        "start": 114,
                        "end": 174
                      }
                    ],
                    "start": 88,
                    "end": 177
                  },
                  "id": null,
                  "generator": false,
                  "start": 82,
                  "end": 177
                }
              ],
              "optional": false,
              "start": 71,
              "end": 178
            },
            "definite": false,
            "start": 62,
            "end": 178
          }
        ],
        "declare": false,
        "start": 56,
        "end": 179
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 179
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
            "name": "App",
            "optional": false,
            "typeAnnotation": null,
            "start": 187,
            "end": 190
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 193,
              "end": 203
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
                              "start": 225,
                              "end": 231
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": true,
                            "start": 224,
                            "end": 233
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 224,
                          "end": 233
                        },
                        "start": 220,
                        "end": 236
                      },
                      "start": 213,
                      "end": 237
                    }
                  ],
                  "start": 210,
                  "end": 239
                },
                "id": null,
                "generator": false,
                "start": 204,
                "end": 239
              }
            ],
            "optional": false,
            "start": 193,
            "end": 240
          },
          "definite": false,
          "start": 187,
          "end": 240
        }
      ],
      "declare": false,
      "start": 181,
      "end": 241
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 241
}
```

</details>

## Output

### Module: test.tsx_Header_component_J4uyIhaBNR4.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
const i_i7ekvWH3674 = ()=>import("./test.tsx_Header_component_div_onClick_i7ekvWH3674");
export const Header_component_J4uyIhaBNR4 = ()=>{
    console.log("mount");
    return <div onClick={/*#__PURE__*/ qrl(i_i7ekvWH3674, "Header_component_div_onClick_i7ekvWH3674")}/>;
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
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "importKind": "value",
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
      "importKind": "value",
      "start": 0,
      "end": 37
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
            "name": "i_i7ekvWH3674",
            "optional": false,
            "typeAnnotation": null,
            "start": 44,
            "end": 57
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
                "value": "./test.tsx_Header_component_div_onClick_i7ekvWH3674",
                "raw": "\"./test.tsx_Header_component_div_onClick_i7ekvWH3674\"",
                "start": 71,
                "end": 124
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 125
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 125
          },
          "definite": false,
          "start": 44,
          "end": 125
        }
      ],
      "declare": false,
      "start": 38,
      "end": 126
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
              "name": "Header_component_J4uyIhaBNR4",
              "optional": false,
              "typeAnnotation": null,
              "start": 140,
              "end": 168
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
                          "start": 181,
                          "end": 188
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "log",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 189,
                          "end": 192
                        },
                        "optional": false,
                        "computed": false,
                        "start": 181,
                        "end": 192
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "mount",
                          "raw": "\"mount\"",
                          "start": 193,
                          "end": 200
                        }
                      ],
                      "optional": false,
                      "start": 181,
                      "end": 201
                    },
                    "directive": null,
                    "start": 181,
                    "end": 202
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 215,
                          "end": 218
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick",
                              "start": 219,
                              "end": 226
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
                                  "start": 242,
                                  "end": 245
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "i_i7ekvWH3674",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 246,
                                    "end": 259
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Header_component_div_onClick_i7ekvWH3674",
                                    "raw": "\"Header_component_div_onClick_i7ekvWH3674\"",
                                    "start": 261,
                                    "end": 303
                                  }
                                ],
                                "optional": false,
                                "start": 242,
                                "end": 304
                              },
                              "start": 227,
                              "end": 305
                            },
                            "start": 219,
                            "end": 305
                          }
                        ],
                        "selfClosing": true,
                        "start": 214,
                        "end": 307
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 214,
                      "end": 307
                    },
                    "start": 207,
                    "end": 308
                  }
                ],
                "start": 175,
                "end": 310
              },
              "id": null,
              "generator": false,
              "start": 171,
              "end": 310
            },
            "definite": false,
            "start": 140,
            "end": 310
          }
        ],
        "declare": false,
        "start": 134,
        "end": 311
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 127,
      "end": 311
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 311
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Header_component_J4uyIhaBNR4",
  "entry": null,
  "displayName": "test.tsx_Header_component",
  "hash": "J4uyIhaBNR4",
  "canonicalFilename": "test.tsx_Header_component_J4uyIhaBNR4",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    84,
    179
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));
/*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
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
            "name": "i_J4uyIhaBNR4",
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
                "value": "./test.tsx_Header_component_J4uyIhaBNR4",
                "raw": "\"./test.tsx_Header_component_J4uyIhaBNR4\"",
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
            "name": "i_ckEPmXZlub0",
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
                "value": "./test.tsx_App_component_ckEPmXZlub0",
                "raw": "\"./test.tsx_App_component_ckEPmXZlub0\"",
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
                      "name": "i_J4uyIhaBNR4",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 303,
                      "end": 316
                    },
                    {
                      "type": "Literal",
                      "value": "Header_component_J4uyIhaBNR4",
                      "raw": "\"Header_component_J4uyIhaBNR4\"",
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
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "decorators": [],
          "name": "componentQrl",
          "optional": false,
          "typeAnnotation": null,
          "start": 366,
          "end": 378
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
              "start": 393,
              "end": 396
            },
            "typeArguments": null,
            "arguments": [
              {
                "type": "Identifier",
                "decorators": [],
                "name": "i_ckEPmXZlub0",
                "optional": false,
                "typeAnnotation": null,
                "start": 397,
                "end": 410
              },
              {
                "type": "Literal",
                "value": "App_component_ckEPmXZlub0",
                "raw": "\"App_component_ckEPmXZlub0\"",
                "start": 412,
                "end": 439
              }
            ],
            "optional": false,
            "start": 393,
            "end": 440
          }
        ],
        "optional": false,
        "start": 366,
        "end": 441
      },
      "directive": null,
      "start": 366,
      "end": 442
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 442
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.tsx (ENTRY POINT)

```tsx
import { Header } from "./test";
export const App_component_ckEPmXZlub0 = ()=>{
    return <Header/>;
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
              "name": "App_component_ckEPmXZlub0",
              "optional": false,
              "typeAnnotation": null,
              "start": 46,
              "end": 71
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
                          "start": 92,
                          "end": 98
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": true,
                        "start": 91,
                        "end": 100
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 91,
                      "end": 100
                    },
                    "start": 84,
                    "end": 101
                  }
                ],
                "start": 78,
                "end": 103
              },
              "id": null,
              "generator": false,
              "start": 74,
              "end": 103
            },
            "definite": false,
            "start": 46,
            "end": 103
          }
        ],
        "declare": false,
        "start": 40,
        "end": 104
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 33,
      "end": 104
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 104
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
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    206,
    241
  ]
}
```

### Module: test.tsx_Header_component_div_onClick_i7ekvWH3674.tsx (ENTRY POINT)

```tsx
export const Header_component_div_onClick_i7ekvWH3674 = (ctx)=>console.log(ctx);
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
              "decorators": [],
              "name": "Header_component_div_onClick_i7ekvWH3674",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 53
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
                  "name": "ctx",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 57,
                  "end": 60
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
                    "name": "console",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 63,
                    "end": 70
                  },
                  "property": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "log",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 71,
                    "end": 74
                  },
                  "optional": false,
                  "computed": false,
                  "start": 63,
                  "end": 74
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "ctx",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 75,
                    "end": 78
                  }
                ],
                "optional": false,
                "start": 63,
                "end": 79
              },
              "id": null,
              "generator": false,
              "start": 56,
              "end": 79
            },
            "definite": false,
            "start": 13,
            "end": 79
          }
        ],
        "declare": false,
        "start": 7,
        "end": 80
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 80
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 80
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Header_component_div_onClick_i7ekvWH3674",
  "entry": null,
  "displayName": "test.tsx_Header_component_div_onClick",
  "hash": "i7ekvWH3674",
  "canonicalFilename": "test.tsx_Header_component_div_onClick_i7ekvWH3674",
  "path": "",
  "extension": "tsx",
  "parent": "Header_component_J4uyIhaBNR4",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    143,
    168
  ],
  "paramNames": [
    "ctx"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (5 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx_Header_component_J4uyIhaBNR4.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 2 |
| `componentQrl` | test.tsx | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
