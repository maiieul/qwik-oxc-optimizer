# Test: issue_117

## Test Configuration

**Note:** Regression test for issue 117 -- simple expression export with no Qwik-specific transforms. Tests that non-component code passes through unchanged.

| Option | Value |
|--------|-------|
| Entry Strategy | Single |
| Mode | Test (default) |
| Filename | project/test.tsx |

## Input

### Source Code

```tsx
export const cache = patternCache[cacheKey] || (patternCache[cacheKey]={});
```

<details>
<summary>Input AST (OXC)</summary>

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
              "name": "cache",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 18
            },
            "init": {
              "type": "LogicalExpression",
              "left": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "patternCache",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 21,
                  "end": 33
                },
                "property": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "cacheKey",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 34,
                  "end": 42
                },
                "optional": false,
                "computed": true,
                "start": 21,
                "end": 43
              },
              "operator": "||",
              "right": {
                "type": "ParenthesizedExpression",
                "expression": {
                  "type": "AssignmentExpression",
                  "operator": "=",
                  "left": {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "patternCache",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 48,
                      "end": 60
                    },
                    "property": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "cacheKey",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 61,
                      "end": 69
                    },
                    "optional": false,
                    "computed": true,
                    "start": 48,
                    "end": 70
                  },
                  "right": {
                    "type": "ObjectExpression",
                    "properties": [],
                    "start": 71,
                    "end": 73
                  },
                  "start": 48,
                  "end": 73
                },
                "start": 47,
                "end": 74
              },
              "start": 21,
              "end": 74
            },
            "definite": false,
            "start": 13,
            "end": 74
          }
        ],
        "declare": false,
        "start": 7,
        "end": 75
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 75
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 75
}

```

</details>

## Output

### Module: project/test.tsx

```tsx
export const cache = patternCache[cacheKey] || (patternCache[cacheKey] = {});
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
              "name": "cache",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 18
            },
            "init": {
              "type": "LogicalExpression",
              "left": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "patternCache",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 21,
                  "end": 33
                },
                "property": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "cacheKey",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 34,
                  "end": 42
                },
                "optional": false,
                "computed": true,
                "start": 21,
                "end": 43
              },
              "operator": "||",
              "right": {
                "type": "ParenthesizedExpression",
                "expression": {
                  "type": "AssignmentExpression",
                  "operator": "=",
                  "left": {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "patternCache",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 48,
                      "end": 60
                    },
                    "property": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "cacheKey",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 61,
                      "end": 69
                    },
                    "optional": false,
                    "computed": true,
                    "start": 48,
                    "end": 70
                  },
                  "right": {
                    "type": "ObjectExpression",
                    "properties": [],
                    "start": 73,
                    "end": 75
                  },
                  "start": 48,
                  "end": 75
                },
                "start": 47,
                "end": 76
              },
              "start": 21,
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

## Conventions Applied

None — passthrough test with no Qwik transformations applied.

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|

## Diagnostics

None (`[]`)
