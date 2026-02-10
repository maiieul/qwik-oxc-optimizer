# Test: example_2

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

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
                              "start": 90,
                              "end": 97
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 98,
                              "end": 101
                            },
                            "optional": false,
                            "computed": false,
                            "start": 90,
                            "end": 101
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "mount",
                              "raw": "\"mount\"",
                              "start": 102,
                              "end": 109
                            }
                          ],
                          "optional": false,
                          "start": 90,
                          "end": 110
                        },
                        "directive": null,
                        "start": 90,
                        "end": 111
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
                                "start": 125,
                                "end": 128
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick",
                                    "start": 129,
                                    "end": 136
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
                                        "start": 138,
                                        "end": 139
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
                                              "start": 141,
                                              "end": 144
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
                                                "start": 149,
                                                "end": 156
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 157,
                                                "end": 160
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 149,
                                              "end": 160
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ctx",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 161,
                                                "end": 164
                                              }
                                            ],
                                            "optional": false,
                                            "start": 149,
                                            "end": 165
                                          },
                                          "id": null,
                                          "generator": false,
                                          "start": 140,
                                          "end": 165
                                        }
                                      ],
                                      "optional": false,
                                      "start": 138,
                                      "end": 166
                                    },
                                    "start": 137,
                                    "end": 167
                                  },
                                  "start": 129,
                                  "end": 167
                                }
                              ],
                              "selfClosing": true,
                              "start": 124,
                              "end": 169
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 124,
                            "end": 169
                          },
                          "start": 120,
                          "end": 172
                        },
                        "start": 113,
                        "end": 173
                      }
                    ],
                    "start": 87,
                    "end": 175
                  },
                  "id": null,
                  "generator": false,
                  "start": 81,
                  "end": 175
                }
              ],
              "optional": false,
              "start": 70,
              "end": 176
            },
            "definite": false,
            "start": 61,
            "end": 176
          }
        ],
        "declare": false,
        "start": 55,
        "end": 177
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 48,
      "end": 177
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 177
}
```

</details>

## Output

### Module: test.tsx_Header_component_J4uyIhaBNR4.tsx [ENTRY POINT]

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
    83,
    177
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
    142,
    167
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
