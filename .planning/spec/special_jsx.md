# Test: special_jsx

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
// don't transpile jsx with non-plain-object props
import { jsx } from '@qwik.dev/core';

export const App = () => {
    const props = {}
    return jsx('div', props, 'Hello Qwik');
}
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
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 60,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 60,
            "end": 63
          },
          "importKind": "value",
          "start": 60,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 51,
      "end": 88
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
              "start": 103,
              "end": 106
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
                          "name": "props",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 127,
                          "end": 132
                        },
                        "init": {
                          "type": "ObjectExpression",
                          "properties": [],
                          "start": 135,
                          "end": 137
                        },
                        "definite": false,
                        "start": 127,
                        "end": 137
                      }
                    ],
                    "declare": false,
                    "start": 121,
                    "end": 137
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "jsx",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 149,
                        "end": 152
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "'div'",
                          "start": 153,
                          "end": 158
                        },
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "props",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 160,
                          "end": 165
                        },
                        {
                          "type": "Literal",
                          "value": "Hello Qwik",
                          "raw": "'Hello Qwik'",
                          "start": 167,
                          "end": 179
                        }
                      ],
                      "optional": false,
                      "start": 149,
                      "end": 180
                    },
                    "start": 142,
                    "end": 181
                  }
                ],
                "start": 115,
                "end": 183
              },
              "id": null,
              "generator": false,
              "start": 109,
              "end": 183
            },
            "definite": false,
            "start": 103,
            "end": 183
          }
        ],
        "declare": false,
        "start": 97,
        "end": 183
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 90,
      "end": 183
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 51,
  "end": 183
}
```

</details>

## Output

### Module: test.tsx

```tsx
// don't transpile jsx with non-plain-object props
import { jsx } from '@qwik.dev/core';
export const App = ()=>{
    const props = {};
    return jsx('div', props, 'Hello Qwik');
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
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 60,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 60,
            "end": 63
          },
          "importKind": "value",
          "start": 60,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 51,
      "end": 88
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
              "start": 102,
              "end": 105
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
                          "name": "props",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 124,
                          "end": 129
                        },
                        "init": {
                          "type": "ObjectExpression",
                          "properties": [],
                          "start": 132,
                          "end": 134
                        },
                        "definite": false,
                        "start": 124,
                        "end": 134
                      }
                    ],
                    "declare": false,
                    "start": 118,
                    "end": 135
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "jsx",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 147,
                        "end": 150
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "'div'",
                          "start": 151,
                          "end": 156
                        },
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "props",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 158,
                          "end": 163
                        },
                        {
                          "type": "Literal",
                          "value": "Hello Qwik",
                          "raw": "'Hello Qwik'",
                          "start": 165,
                          "end": 177
                        }
                      ],
                      "optional": false,
                      "start": 147,
                      "end": 178
                    },
                    "start": 140,
                    "end": 179
                  }
                ],
                "start": 112,
                "end": 181
              },
              "id": null,
              "generator": false,
              "start": 108,
              "end": 181
            },
            "definite": false,
            "start": 102,
            "end": 181
          }
        ],
        "declare": false,
        "start": 96,
        "end": 182
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 89,
      "end": 182
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 51,
  "end": 182
}
```

</details>

## Conventions Applied

*No optimizer conventions detected in output.*

## Function Calls in Output

*No notable function calls detected.*

## Diagnostics

```json
[]
```
