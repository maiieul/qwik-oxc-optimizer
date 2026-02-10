# Test: example_functional_component

## Test Configuration

| Option | Value |
|--------|-------|
| Minify | None |

## Input

### Source Code

```tsx
import { $, component$, useStore } from '@qwik.dev/core';
const Header = component$(() => {
	const thing = useStore();
	const {foo, bar} = foo();

	return (
		<div>{thing}</div>
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 40,
        "end": 56
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 57
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
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 64,
            "end": 70
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 73,
              "end": 83
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
                            "name": "thing",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 99,
                            "end": 104
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useStore",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 107,
                              "end": 115
                            },
                            "typeArguments": null,
                            "arguments": [],
                            "optional": false,
                            "start": 107,
                            "end": 117
                          },
                          "definite": false,
                          "start": 99,
                          "end": 117
                        }
                      ],
                      "declare": false,
                      "start": 93,
                      "end": 118
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
                                  "name": "foo",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 127,
                                  "end": 130
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "foo",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 127,
                                  "end": 130
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "optional": false,
                                "start": 127,
                                "end": 130
                              },
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "bar",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 132,
                                  "end": 135
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "bar",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 132,
                                  "end": 135
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "optional": false,
                                "start": 132,
                                "end": 135
                              }
                            ],
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 126,
                            "end": 136
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "foo",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 139,
                              "end": 142
                            },
                            "typeArguments": null,
                            "arguments": [],
                            "optional": false,
                            "start": 139,
                            "end": 144
                          },
                          "definite": false,
                          "start": 126,
                          "end": 144
                        }
                      ],
                      "declare": false,
                      "start": 120,
                      "end": 145
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
                              "start": 160,
                              "end": 163
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 159,
                            "end": 164
                          },
                          "children": [
                            {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "thing",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 165,
                                "end": 170
                              },
                              "start": 164,
                              "end": 171
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 173,
                              "end": 176
                            },
                            "start": 171,
                            "end": 177
                          },
                          "start": 159,
                          "end": 177
                        },
                        "start": 155,
                        "end": 180
                      },
                      "start": 148,
                      "end": 181
                    }
                  ],
                  "start": 90,
                  "end": 183
                },
                "id": null,
                "generator": false,
                "start": 84,
                "end": 183
              }
            ],
            "optional": false,
            "start": 73,
            "end": 184
          },
          "definite": false,
          "start": 64,
          "end": 184
        }
      ],
      "declare": false,
      "start": 58,
      "end": 185
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 185
}
```

</details>

## Output

### Module: test.tsx_Header_component_J4uyIhaBNR4.tsx [ENTRY POINT]

```tsx
import { useStore } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
export const Header_component_J4uyIhaBNR4 = ()=>{
    const thing = useStore();
    const { foo, bar } = foo();
    return <div>{thing}</div>;
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
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 17
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 17
          },
          "importKind": "value",
          "start": 9,
          "end": 17
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 25,
        "end": 41
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 42
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
            "start": 49,
            "end": 62
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
                "start": 76,
                "end": 117
              },
              "options": null,
              "phase": null,
              "start": 69,
              "end": 118
            },
            "id": null,
            "generator": false,
            "start": 65,
            "end": 118
          },
          "definite": false,
          "start": 49,
          "end": 118
        }
      ],
      "declare": false,
      "start": 43,
      "end": 119
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
              "start": 133,
              "end": 161
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
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "thing",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 180,
                          "end": 185
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useStore",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 188,
                            "end": 196
                          },
                          "typeArguments": null,
                          "arguments": [],
                          "optional": false,
                          "start": 188,
                          "end": 198
                        },
                        "definite": false,
                        "start": 180,
                        "end": 198
                      }
                    ],
                    "declare": false,
                    "start": 174,
                    "end": 199
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
                                "name": "foo",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 212,
                                "end": 215
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "foo",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 212,
                                "end": 215
                              },
                              "method": false,
                              "shorthand": true,
                              "computed": false,
                              "optional": false,
                              "start": 212,
                              "end": 215
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "bar",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 217,
                                "end": 220
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "bar",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 217,
                                "end": 220
                              },
                              "method": false,
                              "shorthand": true,
                              "computed": false,
                              "optional": false,
                              "start": 217,
                              "end": 220
                            }
                          ],
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 210,
                          "end": 222
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "foo",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 225,
                            "end": 228
                          },
                          "typeArguments": null,
                          "arguments": [],
                          "optional": false,
                          "start": 225,
                          "end": 230
                        },
                        "definite": false,
                        "start": 210,
                        "end": 230
                      }
                    ],
                    "declare": false,
                    "start": 204,
                    "end": 231
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
                          "start": 244,
                          "end": 247
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 243,
                        "end": 248
                      },
                      "children": [
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "thing",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 249,
                            "end": 254
                          },
                          "start": 248,
                          "end": 255
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 257,
                          "end": 260
                        },
                        "start": 255,
                        "end": 261
                      },
                      "start": 243,
                      "end": 261
                    },
                    "start": 236,
                    "end": 262
                  }
                ],
                "start": 168,
                "end": 264
              },
              "id": null,
              "generator": false,
              "start": 164,
              "end": 264
            },
            "definite": false,
            "start": 133,
            "end": 264
          }
        ],
        "declare": false,
        "start": 127,
        "end": 265
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 120,
      "end": 265
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 265
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
    86,
    185
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
import { $, component$, useStore } from '@qwik.dev/core';
const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));
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
            "start": 171,
            "end": 172
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 171,
            "end": 172
          },
          "importKind": "value",
          "start": 171,
          "end": 172
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 174,
            "end": 184
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "component$",
            "optional": false,
            "typeAnnotation": null,
            "start": 174,
            "end": 184
          },
          "importKind": "value",
          "start": 174,
          "end": 184
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 186,
            "end": 194
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 186,
            "end": 194
          },
          "importKind": "value",
          "start": 186,
          "end": 194
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 202,
        "end": 218
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 162,
      "end": 219
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
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 226,
            "end": 232
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "componentQrl",
              "optional": false,
              "typeAnnotation": null,
              "start": 249,
              "end": 261
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
                  "start": 276,
                  "end": 279
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "i_J4uyIhaBNR4",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 280,
                    "end": 293
                  },
                  {
                    "type": "Literal",
                    "value": "Header_component_J4uyIhaBNR4",
                    "raw": "\"Header_component_J4uyIhaBNR4\"",
                    "start": 295,
                    "end": 325
                  }
                ],
                "optional": false,
                "start": 276,
                "end": 326
              }
            ],
            "optional": false,
            "start": 249,
            "end": 327
          },
          "definite": false,
          "start": 226,
          "end": 327
        }
      ],
      "declare": false,
      "start": 220,
      "end": 328
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 328
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (2 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (2 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `useStore` | test.tsx_Header_component_J4uyIhaBNR4.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

No diagnostics.
