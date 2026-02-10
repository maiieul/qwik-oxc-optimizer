# Test: example_6

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
export const sym1 = $((ctx) => console.log("1"));
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
              "name": "sym1",
              "optional": false,
              "typeAnnotation": null,
              "start": 61,
              "end": 65
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "$",
                "optional": false,
                "typeAnnotation": null,
                "start": 68,
                "end": 69
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
                      "start": 71,
                      "end": 74
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
                        "start": 79,
                        "end": 86
                      },
                      "property": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "log",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 87,
                        "end": 90
                      },
                      "optional": false,
                      "computed": false,
                      "start": 79,
                      "end": 90
                    },
                    "typeArguments": null,
                    "arguments": [
                      {
                        "type": "Literal",
                        "value": "1",
                        "raw": "\"1\"",
                        "start": 91,
                        "end": 94
                      }
                    ],
                    "optional": false,
                    "start": 79,
                    "end": 95
                  },
                  "id": null,
                  "generator": false,
                  "start": 70,
                  "end": 95
                }
              ],
              "optional": false,
              "start": 68,
              "end": 96
            },
            "definite": false,
            "start": 61,
            "end": 96
          }
        ],
        "declare": false,
        "start": 55,
        "end": 97
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 48,
      "end": 97
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 97
}
```

</details>

## Output

### Module: test.tsx_sym1_aXUrPXX5Lak.tsx (ENTRY POINT)

```tsx
export const sym1_aXUrPXX5Lak = (ctx)=>console.log("1");
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
              "name": "sym1_aXUrPXX5Lak",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 29
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
                  "start": 33,
                  "end": 36
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
                    "start": 39,
                    "end": 46
                  },
                  "property": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "log",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 47,
                    "end": 50
                  },
                  "optional": false,
                  "computed": false,
                  "start": 39,
                  "end": 50
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "1",
                    "raw": "\"1\"",
                    "start": 51,
                    "end": 54
                  }
                ],
                "optional": false,
                "start": 39,
                "end": 55
              },
              "id": null,
              "generator": false,
              "start": 32,
              "end": 55
            },
            "definite": false,
            "start": 13,
            "end": 55
          }
        ],
        "declare": false,
        "start": 7,
        "end": 56
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 56
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 56
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "sym1_aXUrPXX5Lak",
  "entry": null,
  "displayName": "test.tsx_sym1",
  "hash": "aXUrPXX5Lak",
  "canonicalFilename": "test.tsx_sym1_aXUrPXX5Lak",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    72,
    97
  ],
  "paramNames": [
    "ctx"
  ]
}
```

### Module: test.tsx

```tsx
import { qrl } from "@qwik.dev/core";
const i_aXUrPXX5Lak = ()=>import("./test.tsx_sym1_aXUrPXX5Lak");
export const sym1 = /*#__PURE__*/ qrl(i_aXUrPXX5Lak, "sym1_aXUrPXX5Lak");
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
            "name": "i_aXUrPXX5Lak",
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
                "value": "./test.tsx_sym1_aXUrPXX5Lak",
                "raw": "\"./test.tsx_sym1_aXUrPXX5Lak\"",
                "start": 71,
                "end": 100
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 101
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 101
          },
          "definite": false,
          "start": 44,
          "end": 101
        }
      ],
      "declare": false,
      "start": 38,
      "end": 102
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
              "name": "sym1",
              "optional": false,
              "typeAnnotation": null,
              "start": 116,
              "end": 120
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "qrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 137,
                "end": 140
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "i_aXUrPXX5Lak",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 141,
                  "end": 154
                },
                {
                  "type": "Literal",
                  "value": "sym1_aXUrPXX5Lak",
                  "raw": "\"sym1_aXUrPXX5Lak\"",
                  "start": 156,
                  "end": 174
                }
              ],
              "optional": false,
              "start": 137,
              "end": 175
            },
            "definite": false,
            "start": 116,
            "end": 175
          }
        ],
        "declare": false,
        "start": 110,
        "end": 176
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 103,
      "end": 176
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 176
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (1 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (1 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
