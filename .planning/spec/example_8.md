# Test: example_8

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

export const Header = component$(() => {
	return $((hola) => {
		const hola = this;
		const {something, styff} = hola;
		const hello = hola.nothere.stuff[global];
		return (
			<Header/>
		);
	});
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
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 98,
                            "end": 99
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
                                  "name": "hola",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 101,
                                  "end": 105
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
                                          "name": "hola",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 120,
                                          "end": 124
                                        },
                                        "init": {
                                          "type": "ThisExpression",
                                          "start": 127,
                                          "end": 131
                                        },
                                        "definite": false,
                                        "start": 120,
                                        "end": 131
                                      }
                                    ],
                                    "declare": false,
                                    "start": 114,
                                    "end": 132
                                  },
                                  {
                                    "type": "VariableDeclaration",
                                    "kind": "const",
                                    "declarations": [
                                      {
                                        "type": "VariableDeclarator",
                                        "id": {
                                          "type": "ObjectPattern",
                                          "decorators": [],
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "something",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 142,
                                                "end": 151
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "something",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 142,
                                                "end": 151
                                              },
                                              "method": false,
                                              "shorthand": true,
                                              "computed": false,
                                              "optional": false,
                                              "start": 142,
                                              "end": 151
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "styff",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 153,
                                                "end": 158
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "styff",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 153,
                                                "end": 158
                                              },
                                              "method": false,
                                              "shorthand": true,
                                              "computed": false,
                                              "optional": false,
                                              "start": 153,
                                              "end": 158
                                            }
                                          ],
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 141,
                                          "end": 159
                                        },
                                        "init": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "hola",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 162,
                                          "end": 166
                                        },
                                        "definite": false,
                                        "start": 141,
                                        "end": 166
                                      }
                                    ],
                                    "declare": false,
                                    "start": 135,
                                    "end": 167
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
                                          "name": "hello",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 176,
                                          "end": 181
                                        },
                                        "init": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "hola",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 184,
                                                "end": 188
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "nothere",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 189,
                                                "end": 196
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 184,
                                              "end": 196
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "stuff",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 197,
                                              "end": 202
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 184,
                                            "end": 202
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "global",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 203,
                                            "end": 209
                                          },
                                          "optional": false,
                                          "computed": true,
                                          "start": 184,
                                          "end": 210
                                        },
                                        "definite": false,
                                        "start": 176,
                                        "end": 210
                                      }
                                    ],
                                    "declare": false,
                                    "start": 170,
                                    "end": 211
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
                                            "name": "Header",
                                            "start": 227,
                                            "end": 233
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": true,
                                          "start": 226,
                                          "end": 235
                                        },
                                        "children": [],
                                        "closingElement": null,
                                        "start": 226,
                                        "end": 235
                                      },
                                      "start": 221,
                                      "end": 239
                                    },
                                    "start": 214,
                                    "end": 240
                                  }
                                ],
                                "start": 110,
                                "end": 243
                              },
                              "id": null,
                              "generator": false,
                              "start": 100,
                              "end": 243
                            }
                          ],
                          "optional": false,
                          "start": 98,
                          "end": 244
                        },
                        "start": 91,
                        "end": 245
                      }
                    ],
                    "start": 88,
                    "end": 247
                  },
                  "id": null,
                  "generator": false,
                  "start": 82,
                  "end": 247
                }
              ],
              "optional": false,
              "start": 71,
              "end": 248
            },
            "definite": false,
            "start": 62,
            "end": 248
          }
        ],
        "declare": false,
        "start": 56,
        "end": 249
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 249
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 249
}
```

</details>

## Output

### Module: test.tsx_Header_component_J4uyIhaBNR4.tsx [ENTRY POINT]

```tsx
import { qrl } from "@qwik.dev/core";
const i_2B8d0oH9ZWc = ()=>import("./test.tsx_Header_component_1_2B8d0oH9ZWc");
export const Header_component_J4uyIhaBNR4 = ()=>{
    return /*#__PURE__*/ qrl(i_2B8d0oH9ZWc, "Header_component_1_2B8d0oH9ZWc");
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
            "name": "i_2B8d0oH9ZWc",
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
                "value": "./test.tsx_Header_component_1_2B8d0oH9ZWc",
                "raw": "\"./test.tsx_Header_component_1_2B8d0oH9ZWc\"",
                "start": 71,
                "end": 114
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 115
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 115
          },
          "definite": false,
          "start": 44,
          "end": 115
        }
      ],
      "declare": false,
      "start": 38,
      "end": 116
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
              "start": 130,
              "end": 158
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
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "qrl",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 192,
                        "end": 195
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "i_2B8d0oH9ZWc",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 196,
                          "end": 209
                        },
                        {
                          "type": "Literal",
                          "value": "Header_component_1_2B8d0oH9ZWc",
                          "raw": "\"Header_component_1_2B8d0oH9ZWc\"",
                          "start": 211,
                          "end": 243
                        }
                      ],
                      "optional": false,
                      "start": 192,
                      "end": 244
                    },
                    "start": 171,
                    "end": 245
                  }
                ],
                "start": 165,
                "end": 247
              },
              "id": null,
              "generator": false,
              "start": 161,
              "end": 247
            },
            "definite": false,
            "start": 130,
            "end": 247
          }
        ],
        "declare": false,
        "start": 124,
        "end": 248
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 117,
      "end": 248
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 248
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
    249
  ]
}
```

### Module: test.tsx_Header_component_1_2B8d0oH9ZWc.tsx [ENTRY POINT]

```tsx
import { Header } from "./test";
export const Header_component_1_2B8d0oH9ZWc = (hola)=>{
    const hola = this;
    hola.nothere.stuff[global];
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
              "name": "Header_component_1_2B8d0oH9ZWc",
              "optional": false,
              "typeAnnotation": null,
              "start": 46,
              "end": 76
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
                  "name": "hola",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 80,
                  "end": 84
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
                          "name": "hola",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 99,
                          "end": 103
                        },
                        "init": {
                          "type": "ThisExpression",
                          "start": 106,
                          "end": 110
                        },
                        "definite": false,
                        "start": 99,
                        "end": 110
                      }
                    ],
                    "declare": false,
                    "start": 93,
                    "end": 111
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "hola",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 116,
                            "end": 120
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "nothere",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 121,
                            "end": 128
                          },
                          "optional": false,
                          "computed": false,
                          "start": 116,
                          "end": 128
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "stuff",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 129,
                          "end": 134
                        },
                        "optional": false,
                        "computed": false,
                        "start": 116,
                        "end": 134
                      },
                      "property": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "global",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 135,
                        "end": 141
                      },
                      "optional": false,
                      "computed": true,
                      "start": 116,
                      "end": 142
                    },
                    "directive": null,
                    "start": 116,
                    "end": 143
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Header",
                          "start": 156,
                          "end": 162
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": true,
                        "start": 155,
                        "end": 164
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 155,
                      "end": 164
                    },
                    "start": 148,
                    "end": 165
                  }
                ],
                "start": 87,
                "end": 167
              },
              "id": null,
              "generator": false,
              "start": 79,
              "end": 167
            },
            "definite": false,
            "start": 46,
            "end": 167
          }
        ],
        "declare": false,
        "start": 40,
        "end": 168
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 33,
      "end": 168
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 168
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Header_component_1_2B8d0oH9ZWc",
  "entry": null,
  "displayName": "test.tsx_Header_component_1",
  "hash": "2B8d0oH9ZWc",
  "canonicalFilename": "test.tsx_Header_component_1_2B8d0oH9ZWc",
  "path": "",
  "extension": "tsx",
  "parent": "Header_component_J4uyIhaBNR4",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    102,
    245
  ],
  "paramNames": [
    "hola"
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
