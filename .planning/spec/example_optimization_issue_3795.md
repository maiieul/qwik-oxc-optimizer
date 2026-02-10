# Test: example_optimization_issue_3795

## Test Configuration

**Note:** Optimization fix for issue 3795 -- variable reassignment (let + +=) correctly preserved in inline component.

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |
| Is Server | false |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

export const Issue3795 = component$(() => {
	let base = "foo";
	const firstAssignment = base;
	base += "bar";
	const secondAssignment = base;
	return (
		<div id='issue-3795-result'>{firstAssignment} {secondAssignment}</div>
	)
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
              "name": "Issue3795",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
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
                        "type": "VariableDeclaration",
                        "kind": "let",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "base",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 95,
                              "end": 99
                            },
                            "init": {
                              "type": "Literal",
                              "value": "foo",
                              "raw": "\"foo\"",
                              "start": 102,
                              "end": 107
                            },
                            "definite": false,
                            "start": 95,
                            "end": 107
                          }
                        ],
                        "declare": false,
                        "start": 91,
                        "end": 108
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
                              "name": "firstAssignment",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 116,
                              "end": 131
                            },
                            "init": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "base",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 134,
                              "end": 138
                            },
                            "definite": false,
                            "start": 116,
                            "end": 138
                          }
                        ],
                        "declare": false,
                        "start": 110,
                        "end": 139
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "AssignmentExpression",
                          "operator": "+=",
                          "left": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "base",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 141,
                            "end": 145
                          },
                          "right": {
                            "type": "Literal",
                            "value": "bar",
                            "raw": "\"bar\"",
                            "start": 149,
                            "end": 154
                          },
                          "start": 141,
                          "end": 154
                        },
                        "directive": null,
                        "start": 141,
                        "end": 155
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
                              "name": "secondAssignment",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 163,
                              "end": 179
                            },
                            "init": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "base",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 182,
                              "end": 186
                            },
                            "definite": false,
                            "start": 163,
                            "end": 186
                          }
                        ],
                        "declare": false,
                        "start": 157,
                        "end": 187
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
                                "start": 201,
                                "end": 204
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "id",
                                    "start": 205,
                                    "end": 207
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": "issue-3795-result",
                                    "raw": "'issue-3795-result'",
                                    "start": 208,
                                    "end": 227
                                  },
                                  "start": 205,
                                  "end": 227
                                }
                              ],
                              "selfClosing": false,
                              "start": 200,
                              "end": 228
                            },
                            "children": [
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "firstAssignment",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 229,
                                  "end": 244
                                },
                                "start": 228,
                                "end": 245
                              },
                              {
                                "type": "JSXText",
                                "value": " ",
                                "raw": " ",
                                "start": 245,
                                "end": 246
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "secondAssignment",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 247,
                                  "end": 263
                                },
                                "start": 246,
                                "end": 264
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 266,
                                "end": 269
                              },
                              "start": 264,
                              "end": 270
                            },
                            "start": 200,
                            "end": 270
                          },
                          "start": 196,
                          "end": 273
                        },
                        "start": 189,
                        "end": 273
                      }
                    ],
                    "start": 88,
                    "end": 276
                  },
                  "id": null,
                  "generator": false,
                  "start": 82,
                  "end": 276
                }
              ],
              "optional": false,
              "start": 71,
              "end": 277
            },
            "definite": false,
            "start": 59,
            "end": 277
          }
        ],
        "declare": false,
        "start": 53,
        "end": 278
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 278
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 278
}

```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const Issue3795 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    let base = "foo";
    const firstAssignment = base;
    base += "bar";
    const secondAssignment = base;
    return /*#__PURE__*/ _jsxSorted("div", null, {
        id: "issue-3795-result"
    }, [
        firstAssignment,
        " ",
        secondAssignment
    ], 1, "u6_0");
}, "Issue3795_component_wsE8beycatI"));
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
            "name": "_jsxSorted",
            "start": 56,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "start": 101,
          "end": 111
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 119,
        "end": 135
      },
      "phase": null,
      "attributes": [],
      "start": 92,
      "end": 136
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
              "name": "Issue3795",
              "start": 150,
              "end": 159
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 176,
                "end": 188
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 203,
                    "end": 213
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
                            "type": "VariableDeclaration",
                            "kind": "let",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "base",
                                  "start": 228,
                                  "end": 232
                                },
                                "init": {
                                  "type": "Literal",
                                  "value": "foo",
                                  "raw": "\"foo\"",
                                  "start": 235,
                                  "end": 240
                                },
                                "start": 228,
                                "end": 240
                              }
                            ],
                            "start": 224,
                            "end": 241
                          },
                          {
                            "type": "VariableDeclaration",
                            "kind": "const",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "firstAssignment",
                                  "start": 252,
                                  "end": 267
                                },
                                "init": {
                                  "type": "Identifier",
                                  "name": "base",
                                  "start": 270,
                                  "end": 274
                                },
                                "start": 252,
                                "end": 274
                              }
                            ],
                            "start": 246,
                            "end": 275
                          },
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "AssignmentExpression",
                              "operator": "+=",
                              "left": {
                                "type": "Identifier",
                                "name": "base",
                                "start": 280,
                                "end": 284
                              },
                              "right": {
                                "type": "Literal",
                                "value": "bar",
                                "raw": "\"bar\"",
                                "start": 288,
                                "end": 293
                              },
                              "start": 280,
                              "end": 293
                            },
                            "start": 280,
                            "end": 294
                          },
                          {
                            "type": "VariableDeclaration",
                            "kind": "const",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "secondAssignment",
                                  "start": 305,
                                  "end": 321
                                },
                                "init": {
                                  "type": "Identifier",
                                  "name": "base",
                                  "start": 324,
                                  "end": 328
                                },
                                "start": 305,
                                "end": 328
                              }
                            ],
                            "start": 299,
                            "end": 329
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 355,
                                "end": 365
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 366,
                                  "end": 371
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 373,
                                  "end": 377
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "id",
                                        "start": 389,
                                        "end": 391
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "issue-3795-result",
                                        "raw": "\"issue-3795-result\"",
                                        "start": 393,
                                        "end": 412
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 389,
                                      "end": 412
                                    }
                                  ],
                                  "start": 379,
                                  "end": 418
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "Identifier",
                                      "name": "firstAssignment",
                                      "start": 430,
                                      "end": 445
                                    },
                                    {
                                      "type": "Literal",
                                      "value": " ",
                                      "raw": "\" \"",
                                      "start": 455,
                                      "end": 458
                                    },
                                    {
                                      "type": "Identifier",
                                      "name": "secondAssignment",
                                      "start": 468,
                                      "end": 484
                                    }
                                  ],
                                  "start": 420,
                                  "end": 490
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 492,
                                  "end": 493
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_0",
                                  "raw": "\"u6_0\"",
                                  "start": 495,
                                  "end": 501
                                }
                              ],
                              "optional": false,
                              "start": 355,
                              "end": 502
                            },
                            "start": 334,
                            "end": 503
                          }
                        ],
                        "start": 218,
                        "end": 505
                      },
                      "id": null,
                      "generator": false,
                      "start": 214,
                      "end": 505
                    },
                    {
                      "type": "Literal",
                      "value": "Issue3795_component_wsE8beycatI",
                      "raw": "\"Issue3795_component_wsE8beycatI\"",
                      "start": 507,
                      "end": 540
                    }
                  ],
                  "optional": false,
                  "start": 203,
                  "end": 541
                }
              ],
              "optional": false,
              "start": 176,
              "end": 542
            },
            "start": 150,
            "end": 542
          }
        ],
        "start": 144,
        "end": 543
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 137,
      "end": 543
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 543
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-07] PURE Annotations**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| inlinedQrl | test.js | @qwik.dev/core | 1 |
| _jsxSorted | test.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
