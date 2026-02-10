# Test: issue_476

## Test Configuration

**Note:** Regression test for issue 476 -- JSX without any Qwik component$, just regular function component. Output should pass through with minimal transformation.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | false (default) |
| Transpile JSX | false (default) |

## Input

### Source Code

```tsx
import { Counter } from "./counter.tsx";

export const Root = () => {
	return (
		<html>
			<head>
				<meta charset="utf-8" />
				<title>Qwik Blank App</title>
			</head>
			<body>
				<Counter initial={3} />
			</body>
		</html>
	);
};
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
            "name": "Counter",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Counter",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "importKind": "value",
          "start": 9,
          "end": 16
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./counter.tsx",
        "raw": "\"./counter.tsx\"",
        "start": 24,
        "end": 39
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 40
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
              "name": "Root",
              "optional": false,
              "typeAnnotation": null,
              "start": 55,
              "end": 59
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
                      "type": "ParenthesizedExpression",
                      "expression": {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "html",
                            "start": 83,
                            "end": 87
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 82,
                          "end": 88
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "\n\t\t\t",
                            "raw": "\n\t\t\t",
                            "start": 88,
                            "end": 92
                          },
                          {
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "head",
                                "start": 93,
                                "end": 97
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 92,
                              "end": 98
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t",
                                "raw": "\n\t\t\t\t",
                                "start": 98,
                                "end": 103
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "meta",
                                    "start": 104,
                                    "end": 108
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "charset",
                                        "start": 109,
                                        "end": 116
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "utf-8",
                                        "raw": "\"utf-8\"",
                                        "start": 117,
                                        "end": 124
                                      },
                                      "start": 109,
                                      "end": 124
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 103,
                                  "end": 127
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 103,
                                "end": 127
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t",
                                "raw": "\n\t\t\t\t",
                                "start": 127,
                                "end": 132
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 133,
                                    "end": 138
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 132,
                                  "end": 139
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Qwik Blank App",
                                    "raw": "Qwik Blank App",
                                    "start": 139,
                                    "end": 153
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 155,
                                    "end": 160
                                  },
                                  "start": 153,
                                  "end": 161
                                },
                                "start": 132,
                                "end": 161
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 161,
                                "end": 165
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "head",
                                "start": 167,
                                "end": 171
                              },
                              "start": 165,
                              "end": 172
                            },
                            "start": 92,
                            "end": 172
                          },
                          {
                            "type": "JSXText",
                            "value": "\n\t\t\t",
                            "raw": "\n\t\t\t",
                            "start": 172,
                            "end": 176
                          },
                          {
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "body",
                                "start": 177,
                                "end": 181
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 176,
                              "end": 182
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t",
                                "raw": "\n\t\t\t\t",
                                "start": 182,
                                "end": 187
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Counter",
                                    "start": 188,
                                    "end": 195
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "initial",
                                        "start": 196,
                                        "end": 203
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 205,
                                          "end": 206
                                        },
                                        "start": 204,
                                        "end": 207
                                      },
                                      "start": 196,
                                      "end": 207
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 187,
                                  "end": 210
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 187,
                                "end": 210
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 210,
                                "end": 214
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "body",
                                "start": 216,
                                "end": 220
                              },
                              "start": 214,
                              "end": 221
                            },
                            "start": 176,
                            "end": 221
                          },
                          {
                            "type": "JSXText",
                            "value": "\n\t\t",
                            "raw": "\n\t\t",
                            "start": 221,
                            "end": 224
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "html",
                            "start": 226,
                            "end": 230
                          },
                          "start": 224,
                          "end": 231
                        },
                        "start": 82,
                        "end": 231
                      },
                      "start": 78,
                      "end": 234
                    },
                    "start": 71,
                    "end": 235
                  }
                ],
                "start": 68,
                "end": 237
              },
              "id": null,
              "generator": false,
              "start": 62,
              "end": 237
            },
            "definite": false,
            "start": 55,
            "end": 237
          }
        ],
        "declare": false,
        "start": 49,
        "end": 238
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 42,
      "end": 238
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 238
}

```

</details>

## Output

### Module: test.tsx

```tsx
import { Counter } from "./counter.tsx";
export const Root = ()=>{
    return <html>
			<head>
				<meta charset="utf-8"/>
				<title>Qwik Blank App</title>
			</head>
			<body>
				<Counter initial={3}/>
			</body>
		</html>;
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
            "name": "Counter",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Counter",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "importKind": "value",
          "start": 9,
          "end": 16
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./counter.tsx",
        "raw": "\"./counter.tsx\"",
        "start": 24,
        "end": 39
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 40
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
              "name": "Root",
              "optional": false,
              "typeAnnotation": null,
              "start": 54,
              "end": 58
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
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "html",
                          "start": 79,
                          "end": 83
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 78,
                        "end": 84
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 84,
                          "end": 88
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "head",
                              "start": 89,
                              "end": 93
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 88,
                            "end": 94
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t",
                              "raw": "\n\t\t\t\t",
                              "start": 94,
                              "end": 99
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "meta",
                                  "start": 100,
                                  "end": 104
                                },
                                "typeArguments": null,
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "charset",
                                      "start": 105,
                                      "end": 112
                                    },
                                    "value": {
                                      "type": "Literal",
                                      "value": "utf-8",
                                      "raw": "\"utf-8\"",
                                      "start": 113,
                                      "end": 120
                                    },
                                    "start": 105,
                                    "end": 120
                                  }
                                ],
                                "selfClosing": true,
                                "start": 99,
                                "end": 122
                              },
                              "children": [],
                              "closingElement": null,
                              "start": 99,
                              "end": 122
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t",
                              "raw": "\n\t\t\t\t",
                              "start": 122,
                              "end": 127
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "title",
                                  "start": 128,
                                  "end": 133
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 127,
                                "end": 134
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "Qwik Blank App",
                                  "raw": "Qwik Blank App",
                                  "start": 134,
                                  "end": 148
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "title",
                                  "start": 150,
                                  "end": 155
                                },
                                "start": 148,
                                "end": 156
                              },
                              "start": 127,
                              "end": 156
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t",
                              "raw": "\n\t\t\t",
                              "start": 156,
                              "end": 160
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "head",
                              "start": 162,
                              "end": 166
                            },
                            "start": 160,
                            "end": 167
                          },
                          "start": 88,
                          "end": 167
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 167,
                          "end": 171
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "body",
                              "start": 172,
                              "end": 176
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 171,
                            "end": 177
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t",
                              "raw": "\n\t\t\t\t",
                              "start": 177,
                              "end": 182
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "Counter",
                                  "start": 183,
                                  "end": 190
                                },
                                "typeArguments": null,
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "initial",
                                      "start": 191,
                                      "end": 198
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "Literal",
                                        "value": 3,
                                        "raw": "3",
                                        "start": 200,
                                        "end": 201
                                      },
                                      "start": 199,
                                      "end": 202
                                    },
                                    "start": 191,
                                    "end": 202
                                  }
                                ],
                                "selfClosing": true,
                                "start": 182,
                                "end": 204
                              },
                              "children": [],
                              "closingElement": null,
                              "start": 182,
                              "end": 204
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t",
                              "raw": "\n\t\t\t",
                              "start": 204,
                              "end": 208
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "body",
                              "start": 210,
                              "end": 214
                            },
                            "start": 208,
                            "end": 215
                          },
                          "start": 171,
                          "end": 215
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t",
                          "raw": "\n\t\t",
                          "start": 215,
                          "end": 218
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "html",
                          "start": 220,
                          "end": 224
                        },
                        "start": 218,
                        "end": 225
                      },
                      "start": 78,
                      "end": 225
                    },
                    "start": 71,
                    "end": 226
                  }
                ],
                "start": 65,
                "end": 228
              },
              "id": null,
              "generator": false,
              "start": 61,
              "end": 228
            },
            "definite": false,
            "start": 54,
            "end": 228
          }
        ],
        "declare": false,
        "start": 48,
        "end": 229
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 41,
      "end": 229
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 229
}

```

</details>

## Conventions Applied


## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|

## Diagnostics

None (`[]`)
