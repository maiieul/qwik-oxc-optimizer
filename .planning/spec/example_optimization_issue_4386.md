# Test: example_optimization_issue_4386

## Test Configuration

**Note:** Optimization fix for issue 4386 -- constant folding of computed property access (FOO_MAPPING[key]) in inline mode.

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | false |
| Is Server | false |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

export const FOO_MAPPING = {
	A: 1,
	B: 2,
	C: 3,
	};

	export default component$(() => {
	const key = 'A';
	const value = FOO_MAPPING[key];

	return <>{value}</>;
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
              "name": "FOO_MAPPING",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
              "end": 70
            },
            "init": {
              "type": "ObjectExpression",
              "properties": [
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "A",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 76,
                    "end": 77
                  },
                  "value": {
                    "type": "Literal",
                    "value": 1,
                    "raw": "1",
                    "start": 79,
                    "end": 80
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "optional": false,
                  "start": 76,
                  "end": 80
                },
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "B",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 83,
                    "end": 84
                  },
                  "value": {
                    "type": "Literal",
                    "value": 2,
                    "raw": "2",
                    "start": 86,
                    "end": 87
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "optional": false,
                  "start": 83,
                  "end": 87
                },
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "C",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 90,
                    "end": 91
                  },
                  "value": {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 93,
                    "end": 94
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "optional": false,
                  "start": 90,
                  "end": 94
                }
              ],
              "start": 73,
              "end": 98
            },
            "definite": false,
            "start": 59,
            "end": 98
          }
        ],
        "declare": false,
        "start": 53,
        "end": 99
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 99
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "decorators": [],
          "name": "component$",
          "optional": false,
          "typeAnnotation": null,
          "start": 117,
          "end": 127
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
                        "name": "key",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 143,
                        "end": 146
                      },
                      "init": {
                        "type": "Literal",
                        "value": "A",
                        "raw": "'A'",
                        "start": 149,
                        "end": 152
                      },
                      "definite": false,
                      "start": 143,
                      "end": 152
                    }
                  ],
                  "declare": false,
                  "start": 137,
                  "end": 153
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
                        "name": "value",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 161,
                        "end": 166
                      },
                      "init": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "FOO_MAPPING",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 169,
                          "end": 180
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "key",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 181,
                          "end": 184
                        },
                        "optional": false,
                        "computed": true,
                        "start": 169,
                        "end": 185
                      },
                      "definite": false,
                      "start": 161,
                      "end": 185
                    }
                  ],
                  "declare": false,
                  "start": 155,
                  "end": 186
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "JSXFragment",
                    "openingFragment": {
                      "type": "JSXOpeningFragment",
                      "start": 196,
                      "end": 198
                    },
                    "children": [
                      {
                        "type": "JSXExpressionContainer",
                        "expression": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "value",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 199,
                          "end": 204
                        },
                        "start": 198,
                        "end": 205
                      }
                    ],
                    "closingFragment": {
                      "type": "JSXClosingFragment",
                      "start": 205,
                      "end": 208
                    },
                    "start": 196,
                    "end": 208
                  },
                  "start": 189,
                  "end": 209
                }
              ],
              "start": 134,
              "end": 212
            },
            "id": null,
            "generator": false,
            "start": 128,
            "end": 212
          }
        ],
        "optional": false,
        "start": 117,
        "end": 213
      },
      "exportKind": "value",
      "start": 102,
      "end": 214
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 214
}

```

</details>

## Output

### Module: test.jsx

```jsx
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const FOO_MAPPING = {
    A: 1,
    B: 2,
    C: 3
};
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    return <>{FOO_MAPPING['A']}</>;
}, "test_component_LUXeXe0DQrg"));
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
            "name": "componentQrl",
            "start": 9,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 9,
            "end": 21
          },
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
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "start": 56,
          "end": 66
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 74,
        "end": 90
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 91
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
              "name": "FOO_MAPPING",
              "start": 105,
              "end": 116
            },
            "init": {
              "type": "ObjectExpression",
              "properties": [
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "name": "A",
                    "start": 125,
                    "end": 126
                  },
                  "value": {
                    "type": "Literal",
                    "value": 1,
                    "raw": "1",
                    "start": 128,
                    "end": 129
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "start": 125,
                  "end": 129
                },
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "name": "B",
                    "start": 135,
                    "end": 136
                  },
                  "value": {
                    "type": "Literal",
                    "value": 2,
                    "raw": "2",
                    "start": 138,
                    "end": 139
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "start": 135,
                  "end": 139
                },
                {
                  "type": "Property",
                  "kind": "init",
                  "key": {
                    "type": "Identifier",
                    "name": "C",
                    "start": 145,
                    "end": 146
                  },
                  "value": {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 148,
                    "end": 149
                  },
                  "method": false,
                  "shorthand": false,
                  "computed": false,
                  "start": 145,
                  "end": 149
                }
              ],
              "start": 119,
              "end": 151
            },
            "start": 105,
            "end": 151
          }
        ],
        "start": 99,
        "end": 152
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 92,
      "end": 152
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 182,
          "end": 194
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "inlinedQrl",
              "start": 209,
              "end": 219
            },
            "arguments": [
              {
                "type": "ArrowFunctionExpression",
                "expression": false,
                "async": false,
                "params": [],
                "body": {
                  "type": "BlockStatement",
                  "body": [
                    {
                      "type": "ReturnStatement",
                      "argument": {
                        "type": "JSXFragment",
                        "openingFragment": {
                          "type": "JSXOpeningFragment",
                          "attributes": [],
                          "selfClosing": false,
                          "start": 237,
                          "end": 239
                        },
                        "children": [
                          {
                            "type": "JSXExpressionContainer",
                            "expression": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "FOO_MAPPING",
                                "start": 240,
                                "end": 251
                              },
                              "property": {
                                "type": "Literal",
                                "value": "A",
                                "raw": "'A'",
                                "start": 252,
                                "end": 255
                              },
                              "optional": false,
                              "computed": true,
                              "start": 240,
                              "end": 256
                            },
                            "start": 239,
                            "end": 257
                          }
                        ],
                        "closingFragment": {
                          "type": "JSXClosingFragment",
                          "start": 257,
                          "end": 260
                        },
                        "start": 237,
                        "end": 260
                      },
                      "start": 230,
                      "end": 261
                    }
                  ],
                  "start": 224,
                  "end": 263
                },
                "id": null,
                "generator": false,
                "start": 220,
                "end": 263
              },
              {
                "type": "Literal",
                "value": "test_component_LUXeXe0DQrg",
                "raw": "\"test_component_LUXeXe0DQrg\"",
                "start": 265,
                "end": 293
              }
            ],
            "optional": false,
            "start": 209,
            "end": 294
          }
        ],
        "optional": false,
        "start": 182,
        "end": 295
      },
      "start": 153,
      "end": 296
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 296
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-07] PURE Annotations**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.jsx | @qwik.dev/core | 1 |
| inlinedQrl | test.jsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
