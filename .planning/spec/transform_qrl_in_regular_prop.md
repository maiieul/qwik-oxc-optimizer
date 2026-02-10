# Test: transform_qrl_in_regular_prop

## Test Configuration

**Note:** QRL ($()) used in a non-event prop (foo={$(() => ...)}) on a component. Tests that $ in regular props is still transformed to qrl().

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, $ } from '@qwik.dev/core';
		export const Cmp = component$(() =>
			<Cmp foo={$(() => console.log('hi there'))}>Hello Qwik</Cmp>);
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
              "name": "Cmp",
              "optional": false,
              "typeAnnotation": null,
              "start": 63,
              "end": 66
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 69,
                "end": 79
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
                    "type": "JSXElement",
                    "openingElement": {
                      "type": "JSXOpeningElement",
                      "name": {
                        "type": "JSXIdentifier",
                        "name": "Cmp",
                        "start": 90,
                        "end": 93
                      },
                      "typeArguments": null,
                      "attributes": [
                        {
                          "type": "JSXAttribute",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "foo",
                            "start": 94,
                            "end": 97
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
                                "start": 99,
                                "end": 100
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
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "console",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 107,
                                        "end": 114
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "log",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 115,
                                        "end": 118
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 107,
                                      "end": 118
                                    },
                                    "typeArguments": null,
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "hi there",
                                        "raw": "'hi there'",
                                        "start": 119,
                                        "end": 129
                                      }
                                    ],
                                    "optional": false,
                                    "start": 107,
                                    "end": 130
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 101,
                                  "end": 130
                                }
                              ],
                              "optional": false,
                              "start": 99,
                              "end": 131
                            },
                            "start": 98,
                            "end": 132
                          },
                          "start": 94,
                          "end": 132
                        }
                      ],
                      "selfClosing": false,
                      "start": 89,
                      "end": 133
                    },
                    "children": [
                      {
                        "type": "JSXText",
                        "value": "Hello Qwik",
                        "raw": "Hello Qwik",
                        "start": 133,
                        "end": 143
                      }
                    ],
                    "closingElement": {
                      "type": "JSXClosingElement",
                      "name": {
                        "type": "JSXIdentifier",
                        "name": "Cmp",
                        "start": 145,
                        "end": 148
                      },
                      "start": 143,
                      "end": 149
                    },
                    "start": 89,
                    "end": 149
                  },
                  "id": null,
                  "generator": false,
                  "start": 80,
                  "end": 149
                }
              ],
              "optional": false,
              "start": 69,
              "end": 150
            },
            "definite": false,
            "start": 63,
            "end": 150
          }
        ],
        "declare": false,
        "start": 57,
        "end": 151
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 50,
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

## Output

### Module: test.tsx_Cmp_component_4ryKJTOKjWE.ts (ENTRY POINT)

```ts
import { Cmp } from "./test";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_iPpdemxi0uE = ()=>import("./test.tsx_Cmp_component_Cmp_foo_iPpdemxi0uE");
export const Cmp_component_4ryKJTOKjWE = ()=>/*#__PURE__*/ _jsxSorted(Cmp, {
        foo: /*#__PURE__*/ qrl(i_iPpdemxi0uE, "Cmp_component_Cmp_foo_iPpdemxi0uE")
    }, null, "Hello Qwik", 3, "u6_0");
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
            "name": "Cmp",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Cmp",
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
        "value": "./test",
        "raw": "\"./test\"",
        "start": 20,
        "end": 28
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 29
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 39,
            "end": 49
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 39,
            "end": 49
          },
          "importKind": "value",
          "start": 39,
          "end": 49
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 57,
        "end": 73
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 30,
      "end": 74
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
            "start": 84,
            "end": 87
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 84,
            "end": 87
          },
          "importKind": "value",
          "start": 84,
          "end": 87
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 95,
        "end": 111
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 75,
      "end": 112
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
            "name": "i_iPpdemxi0uE",
            "optional": false,
            "typeAnnotation": null,
            "start": 119,
            "end": 132
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
                "value": "./test.tsx_Cmp_component_Cmp_foo_iPpdemxi0uE",
                "raw": "\"./test.tsx_Cmp_component_Cmp_foo_iPpdemxi0uE\"",
                "start": 146,
                "end": 192
              },
              "options": null,
              "phase": null,
              "start": 139,
              "end": 193
            },
            "id": null,
            "generator": false,
            "start": 135,
            "end": 193
          },
          "definite": false,
          "start": 119,
          "end": 193
        }
      ],
      "declare": false,
      "start": 113,
      "end": 194
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
              "name": "Cmp_component_4ryKJTOKjWE",
              "optional": false,
              "typeAnnotation": null,
              "start": 208,
              "end": 233
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "typeParameters": null,
              "params": [],
              "returnType": null,
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "_jsxSorted",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 254,
                  "end": 264
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "Cmp",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 265,
                    "end": 268
                  },
                  {
                    "type": "ObjectExpression",
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
                          "start": 280,
                          "end": 283
                        },
                        "value": {
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
                              "name": "i_iPpdemxi0uE",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 303,
                              "end": 316
                            },
                            {
                              "type": "Literal",
                              "value": "Cmp_component_Cmp_foo_iPpdemxi0uE",
                              "raw": "\"Cmp_component_Cmp_foo_iPpdemxi0uE\"",
                              "start": 318,
                              "end": 353
                            }
                          ],
                          "optional": false,
                          "start": 299,
                          "end": 354
                        },
                        "method": false,
                        "shorthand": false,
                        "computed": false,
                        "optional": false,
                        "start": 280,
                        "end": 354
                      }
                    ],
                    "start": 270,
                    "end": 360
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 362,
                    "end": 366
                  },
                  {
                    "type": "Literal",
                    "value": "Hello Qwik",
                    "raw": "\"Hello Qwik\"",
                    "start": 368,
                    "end": 380
                  },
                  {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 382,
                    "end": 383
                  },
                  {
                    "type": "Literal",
                    "value": "u6_0",
                    "raw": "\"u6_0\"",
                    "start": 385,
                    "end": 391
                  }
                ],
                "optional": false,
                "start": 254,
                "end": 392
              },
              "id": null,
              "generator": false,
              "start": 236,
              "end": 392
            },
            "definite": false,
            "start": 208,
            "end": 392
          }
        ],
        "declare": false,
        "start": 202,
        "end": 393
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 195,
      "end": 393
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 393
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_4ryKJTOKjWE",
  "entry": null,
  "displayName": "test.tsx_Cmp_component",
  "hash": "4ryKJTOKjWE",
  "canonicalFilename": "test.tsx_Cmp_component_4ryKJTOKjWE",
  "path": "",
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    84,
    153
  ]
}
```

### Module: test.tsx_Cmp_component_Cmp_foo_iPpdemxi0uE.ts (ENTRY POINT)

```ts
export const Cmp_component_Cmp_foo_iPpdemxi0uE = ()=>console.log('hi there');
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
              "name": "Cmp_component_Cmp_foo_iPpdemxi0uE",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 46
            },
            "init": {
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
                    "start": 53,
                    "end": 60
                  },
                  "property": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "log",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 61,
                    "end": 64
                  },
                  "optional": false,
                  "computed": false,
                  "start": 53,
                  "end": 64
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "hi there",
                    "raw": "'hi there'",
                    "start": 65,
                    "end": 75
                  }
                ],
                "optional": false,
                "start": 53,
                "end": 76
              },
              "id": null,
              "generator": false,
              "start": 49,
              "end": 76
            },
            "definite": false,
            "start": 13,
            "end": 76
          }
        ],
        "declare": false,
        "start": 7,
        "end": 77
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 77
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 77
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_Cmp_foo_iPpdemxi0uE",
  "entry": null,
  "displayName": "test.tsx_Cmp_component_Cmp_foo",
  "hash": "iPpdemxi0uE",
  "canonicalFilename": "test.tsx_Cmp_component_Cmp_foo_iPpdemxi0uE",
  "path": "",
  "extension": "ts",
  "parent": "Cmp_component_4ryKJTOKjWE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    105,
    134
  ]
}
```

### Module: test.ts

```ts
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_4ryKJTOKjWE = ()=>import("./test.tsx_Cmp_component_4ryKJTOKjWE");
export const Cmp = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_4ryKJTOKjWE, "Cmp_component_4ryKJTOKjWE"));
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
            "name": "i_4ryKJTOKjWE",
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
                "value": "./test.tsx_Cmp_component_4ryKJTOKjWE",
                "raw": "\"./test.tsx_Cmp_component_4ryKJTOKjWE\"",
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
          "definite": false,
          "start": 91,
          "end": 157
        }
      ],
      "declare": false,
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
              "decorators": [],
              "name": "Cmp",
              "optional": false,
              "typeAnnotation": null,
              "start": 172,
              "end": 175
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 192,
                "end": 204
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
                    "start": 219,
                    "end": 222
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_4ryKJTOKjWE",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "Cmp_component_4ryKJTOKjWE",
                      "raw": "\"Cmp_component_4ryKJTOKjWE\"",
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
            "definite": false,
            "start": 172,
            "end": 267
          }
        ],
        "declare": false,
        "start": 166,
        "end": 268
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 159,
      "end": 268
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 268
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| qrl | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| componentQrl | test.ts | @qwik.dev/core | 1 |
| qrl | test.ts | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
