# Test: support_windows_paths

## Test Configuration

**Note:** Windows path handling -- backslash file paths in filename and src_dir. Verifies output module paths use forward slashes.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile JSX | true |
| Is Server | false |
| Filename | components\\apps\\apps.tsx |
| Src Dir | C:\\users\\apps |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';
export const Greeter = component$(() => <div/>)
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 44
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
              "name": "Greeter",
              "optional": false,
              "typeAnnotation": null,
              "start": 58,
              "end": 65
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 68,
                "end": 78
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
                        "name": "div",
                        "start": 86,
                        "end": 89
                      },
                      "typeArguments": null,
                      "attributes": [],
                      "selfClosing": true,
                      "start": 85,
                      "end": 91
                    },
                    "children": [],
                    "closingElement": null,
                    "start": 85,
                    "end": 91
                  },
                  "id": null,
                  "generator": false,
                  "start": 79,
                  "end": 91
                }
              ],
              "optional": false,
              "start": 68,
              "end": 92
            },
            "definite": false,
            "start": 58,
            "end": 92
          }
        ],
        "declare": false,
        "start": 52,
        "end": 92
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 45,
      "end": 92
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 92
}

```

</details>

## Output

### Module: components/apps/apps.tsx_Greeter_component_0jjOvx068y0.ts (ENTRY POINT)

```ts
import { _jsxSorted } from "@qwik.dev/core";
export const Greeter_component_0jjOvx068y0 = ()=>/*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "KD_0");
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
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 19
          },
          "importKind": "value",
          "start": 9,
          "end": 19
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 44
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
              "name": "Greeter_component_0jjOvx068y0",
              "optional": false,
              "typeAnnotation": null,
              "start": 58,
              "end": 87
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
                  "start": 108,
                  "end": 118
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 119,
                    "end": 124
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 126,
                    "end": 130
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 132,
                    "end": 136
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 138,
                    "end": 142
                  },
                  {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 144,
                    "end": 145
                  },
                  {
                    "type": "Literal",
                    "value": "KD_0",
                    "raw": "\"KD_0\"",
                    "start": 147,
                    "end": 153
                  }
                ],
                "optional": false,
                "start": 108,
                "end": 154
              },
              "id": null,
              "generator": false,
              "start": 90,
              "end": 154
            },
            "definite": false,
            "start": 58,
            "end": 154
          }
        ],
        "declare": false,
        "start": 52,
        "end": 155
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 45,
      "end": 155
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 155
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "components/apps/apps.tsx",
  "name": "Greeter_component_0jjOvx068y0",
  "entry": null,
  "displayName": "apps.tsx_Greeter_component",
  "hash": "0jjOvx068y0",
  "canonicalFilename": "apps.tsx_Greeter_component_0jjOvx068y0",
  "path": "components/apps",
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    81,
    93
  ]
}
```

### Module: components/apps/apps.ts

```ts
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_0jjOvx068y0 = ()=>import("./apps.tsx_Greeter_component_0jjOvx068y0");
export const Greeter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0jjOvx068y0, "Greeter_component_0jjOvx068y0"));
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
            "name": "i_0jjOvx068y0",
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
                "value": "./apps.tsx_Greeter_component_0jjOvx068y0",
                "raw": "\"./apps.tsx_Greeter_component_0jjOvx068y0\"",
                "start": 118,
                "end": 160
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 161
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 161
          },
          "definite": false,
          "start": 91,
          "end": 161
        }
      ],
      "declare": false,
      "start": 85,
      "end": 162
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
              "name": "Greeter",
              "optional": false,
              "typeAnnotation": null,
              "start": 176,
              "end": 183
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 200,
                "end": 212
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
                    "start": 227,
                    "end": 230
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_0jjOvx068y0",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 231,
                      "end": 244
                    },
                    {
                      "type": "Literal",
                      "value": "Greeter_component_0jjOvx068y0",
                      "raw": "\"Greeter_component_0jjOvx068y0\"",
                      "start": 246,
                      "end": 277
                    }
                  ],
                  "optional": false,
                  "start": 227,
                  "end": 278
                }
              ],
              "optional": false,
              "start": 200,
              "end": 279
            },
            "definite": false,
            "start": 176,
            "end": 279
          }
        ],
        "declare": false,
        "start": 170,
        "end": 280
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 163,
      "end": 280
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 280
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
| _jsxSorted | ...pps/apps.tsx_Greeter_component_0jjOvx068y0.ts | @qwik.dev/core | 1 |
| componentQrl | components/apps/apps.ts | @qwik.dev/core | 1 |
| qrl | components/apps/apps.ts | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
