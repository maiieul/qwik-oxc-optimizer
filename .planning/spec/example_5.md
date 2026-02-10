# Test: example_5

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
export const Header = component$(() => {
	return (
		<>
			<div onClick={(ctx) => console.log("1")}/>
			<div onClick={$((ctx) => console.log("2"))}/>
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
              "start": 61,
              "end": 67
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 70,
                "end": 80
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
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 101,
                              "end": 103
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 103,
                                "end": 107
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 108,
                                    "end": 111
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "onClick",
                                        "start": 112,
                                        "end": 119
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
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
                                              "start": 122,
                                              "end": 125
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
                                                "start": 130,
                                                "end": 137
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 138,
                                                "end": 141
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 130,
                                              "end": 141
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "1",
                                                "raw": "\"1\"",
                                                "start": 142,
                                                "end": 145
                                              }
                                            ],
                                            "optional": false,
                                            "start": 130,
                                            "end": 146
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 121,
                                          "end": 146
                                        },
                                        "start": 120,
                                        "end": 147
                                      },
                                      "start": 112,
                                      "end": 147
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 107,
                                  "end": 149
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 107,
                                "end": 149
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 149,
                                "end": 153
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 154,
                                    "end": 157
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "onClick",
                                        "start": 158,
                                        "end": 165
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
                                            "start": 167,
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
                                                  "name": "ctx",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 170,
                                                  "end": 173
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
                                                    "start": 178,
                                                    "end": 185
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "log",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 186,
                                                    "end": 189
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 178,
                                                  "end": 189
                                                },
                                                "typeArguments": null,
                                                "arguments": [
                                                  {
                                                    "type": "Literal",
                                                    "value": "2",
                                                    "raw": "\"2\"",
                                                    "start": 190,
                                                    "end": 193
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 178,
                                                "end": 194
                                              },
                                              "id": null,
                                              "generator": false,
                                              "start": 169,
                                              "end": 194
                                            }
                                          ],
                                          "optional": false,
                                          "start": 167,
                                          "end": 195
                                        },
                                        "start": 166,
                                        "end": 196
                                      },
                                      "start": 158,
                                      "end": 196
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 153,
                                  "end": 198
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 153,
                                "end": 198
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 198,
                                "end": 201
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 201,
                              "end": 204
                            },
                            "start": 101,
                            "end": 204
                          },
                          "start": 97,
                          "end": 207
                        },
                        "start": 90,
                        "end": 208
                      }
                    ],
                    "start": 87,
                    "end": 210
                  },
                  "id": null,
                  "generator": false,
                  "start": 81,
                  "end": 210
                }
              ],
              "optional": false,
              "start": 70,
              "end": 211
            },
            "definite": false,
            "start": 61,
            "end": 211
          }
        ],
        "declare": false,
        "start": 55,
        "end": 212
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 48,
      "end": 212
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 212
}
```

</details>

## Output

### Module: test.tsx_Header_component_J4uyIhaBNR4.tsx [ENTRY POINT]

```tsx
import { qrl } from "@qwik.dev/core";
const i_i7ekvWH3674 = ()=>import("./test.tsx_Header_component_div_onClick_i7ekvWH3674");
export const Header_component_J4uyIhaBNR4 = ()=>{
    return <>
			<div onClick={(ctx)=>console.log("1")}/>
			<div onClick={/*#__PURE__*/ qrl(i_i7ekvWH3674, "Header_component_div_onClick_i7ekvWH3674")}/>
		</>;
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
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXFragment",
                      "openingFragment": {
                        "type": "JSXOpeningFragment",
                        "start": 188,
                        "end": 190
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 190,
                          "end": 194
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 195,
                              "end": 198
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick",
                                  "start": 199,
                                  "end": 206
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
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
                                        "start": 209,
                                        "end": 212
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
                                          "start": 215,
                                          "end": 222
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "log",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 223,
                                          "end": 226
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 215,
                                        "end": 226
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "1",
                                          "raw": "\"1\"",
                                          "start": 227,
                                          "end": 230
                                        }
                                      ],
                                      "optional": false,
                                      "start": 215,
                                      "end": 231
                                    },
                                    "id": null,
                                    "generator": false,
                                    "start": 208,
                                    "end": 231
                                  },
                                  "start": 207,
                                  "end": 232
                                },
                                "start": 199,
                                "end": 232
                              }
                            ],
                            "selfClosing": true,
                            "start": 194,
                            "end": 234
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 194,
                          "end": 234
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 234,
                          "end": 238
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 239,
                              "end": 242
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick",
                                  "start": 243,
                                  "end": 250
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
                                      "start": 266,
                                      "end": 269
                                    },
                                    "typeArguments": null,
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "i_i7ekvWH3674",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 270,
                                        "end": 283
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "Header_component_div_onClick_i7ekvWH3674",
                                        "raw": "\"Header_component_div_onClick_i7ekvWH3674\"",
                                        "start": 285,
                                        "end": 327
                                      }
                                    ],
                                    "optional": false,
                                    "start": 266,
                                    "end": 328
                                  },
                                  "start": 251,
                                  "end": 329
                                },
                                "start": 243,
                                "end": 329
                              }
                            ],
                            "selfClosing": true,
                            "start": 238,
                            "end": 331
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 238,
                          "end": 331
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t",
                          "raw": "\n\t\t",
                          "start": 331,
                          "end": 334
                        }
                      ],
                      "closingFragment": {
                        "type": "JSXClosingFragment",
                        "start": 334,
                        "end": 337
                      },
                      "start": 188,
                      "end": 337
                    },
                    "start": 181,
                    "end": 338
                  }
                ],
                "start": 175,
                "end": 340
              },
              "id": null,
              "generator": false,
              "start": 171,
              "end": 340
            },
            "definite": false,
            "start": 140,
            "end": 340
          }
        ],
        "declare": false,
        "start": 134,
        "end": 341
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 127,
      "end": 341
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 341
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
    83,
    212
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
export const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));
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
              "start": 175,
              "end": 181
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 198,
                "end": 210
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
                    "start": 225,
                    "end": 228
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_J4uyIhaBNR4",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 229,
                      "end": 242
                    },
                    {
                      "type": "Literal",
                      "value": "Header_component_J4uyIhaBNR4",
                      "raw": "\"Header_component_J4uyIhaBNR4\"",
                      "start": 244,
                      "end": 274
                    }
                  ],
                  "optional": false,
                  "start": 225,
                  "end": 275
                }
              ],
              "optional": false,
              "start": 198,
              "end": 276
            },
            "definite": false,
            "start": 175,
            "end": 276
          }
        ],
        "declare": false,
        "start": 169,
        "end": 277
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 162,
      "end": 277
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 277
}
```

</details>

### Module: test.tsx_Header_component_div_onClick_i7ekvWH3674.tsx [ENTRY POINT]

```tsx
export const Header_component_div_onClick_i7ekvWH3674 = (ctx)=>console.log("2");
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
                    "type": "Literal",
                    "value": "2",
                    "raw": "\"2\"",
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
    171,
    196
  ],
  "paramNames": [
    "ctx"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (2 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (3 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx_Header_component_J4uyIhaBNR4.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

No diagnostics.
