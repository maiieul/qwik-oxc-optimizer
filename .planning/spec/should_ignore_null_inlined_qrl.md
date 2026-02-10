# Test: should_ignore_null_inlined_qrl

## Test Configuration

**Note:** Tests that inlinedQrl(null, ...) is preserved as-is without attempting to extract a segment from null.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { inlinedQrl } from '@qwik.dev/core';

		const foo = inlinedQrl(null, 'some_hash');
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
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
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
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 54,
            "end": 57
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "inlinedQrl",
              "optional": false,
              "typeAnnotation": null,
              "start": 60,
              "end": 70
            },
            "typeArguments": null,
            "arguments": [
              {
                "type": "Literal",
                "value": null,
                "raw": "null",
                "start": 71,
                "end": 75
              },
              {
                "type": "Literal",
                "value": "some_hash",
                "raw": "'some_hash'",
                "start": 77,
                "end": 88
              }
            ],
            "optional": false,
            "start": 60,
            "end": 89
          },
          "definite": false,
          "start": 54,
          "end": 89
        }
      ],
      "declare": false,
      "start": 48,
      "end": 90
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 90
}

```

</details>

## Output

### Module: test.js

```javascript
import { inlinedQrl } from '@qwik.dev/core';
inlinedQrl(null, 'some_hash');
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
            "name": "inlinedQrl",
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 9,
            "end": 19
          },
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
      "start": 0,
      "end": 44
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "inlinedQrl",
          "start": 45,
          "end": 55
        },
        "arguments": [
          {
            "type": "Literal",
            "value": null,
            "raw": "null",
            "start": 56,
            "end": 60
          },
          {
            "type": "Literal",
            "value": "some_hash",
            "raw": "'some_hash'",
            "start": 62,
            "end": 73
          }
        ],
        "optional": false,
        "start": 45,
        "end": 74
      },
      "start": 45,
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

## Conventions Applied

- **[CONV-01] QRL Wrapping**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| inlinedQrl | test.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
