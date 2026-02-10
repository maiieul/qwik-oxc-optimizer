# Test: example_with_tagname

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

export const Foo = component$(() => {
	return $(() => {
		return (
			<div>
			</div>
		)
	});
}, {
	tagName: "my-foo",
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
              "name": "Foo",
              "optional": false,
              "typeAnnotation": null,
              "start": 62,
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
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 95,
                            "end": 96
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
                                            "start": 120,
                                            "end": 123
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 119,
                                          "end": 124
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t",
                                            "raw": "\n\t\t\t",
                                            "start": 124,
                                            "end": 128
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 130,
                                            "end": 133
                                          },
                                          "start": 128,
                                          "end": 134
                                        },
                                        "start": 119,
                                        "end": 134
                                      },
                                      "start": 114,
                                      "end": 138
                                    },
                                    "start": 107,
                                    "end": 138
                                  }
                                ],
                                "start": 103,
                                "end": 141
                              },
                              "id": null,
                              "generator": false,
                              "start": 97,
                              "end": 141
                            }
                          ],
                          "optional": false,
                          "start": 95,
                          "end": 142
                        },
                        "start": 88,
                        "end": 143
                      }
                    ],
                    "start": 85,
                    "end": 145
                  },
                  "id": null,
                  "generator": false,
                  "start": 79,
                  "end": 145
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
                        "name": "tagName",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 150,
                        "end": 157
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 159,
                        "end": 167
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 150,
                      "end": 167
                    }
                  ],
                  "start": 147,
                  "end": 170
                }
              ],
              "optional": false,
              "start": 68,
              "end": 171
            },
            "definite": false,
            "start": 62,
            "end": 171
          }
        ],
        "declare": false,
        "start": 56,
        "end": 172
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 172
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 172
}
```

</details>

## Output

### Module: test.tsx_Foo_component_HTDRsvUbLiE.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
const i_DvU6FitWglY = ()=>import("./test.tsx_Foo_component_1_DvU6FitWglY");
export const Foo_component_HTDRsvUbLiE = ()=>{
    return /*#__PURE__*/ qrl(i_DvU6FitWglY, "Foo_component_1_DvU6FitWglY");
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
            "name": "i_DvU6FitWglY",
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
                "value": "./test.tsx_Foo_component_1_DvU6FitWglY",
                "raw": "\"./test.tsx_Foo_component_1_DvU6FitWglY\"",
                "start": 71,
                "end": 111
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 112
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 112
          },
          "definite": false,
          "start": 44,
          "end": 112
        }
      ],
      "declare": false,
      "start": 38,
      "end": 113
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
              "name": "Foo_component_HTDRsvUbLiE",
              "optional": false,
              "typeAnnotation": null,
              "start": 127,
              "end": 152
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
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "qrl",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 186,
                        "end": 189
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "i_DvU6FitWglY",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 190,
                          "end": 203
                        },
                        {
                          "type": "Literal",
                          "value": "Foo_component_1_DvU6FitWglY",
                          "raw": "\"Foo_component_1_DvU6FitWglY\"",
                          "start": 205,
                          "end": 234
                        }
                      ],
                      "optional": false,
                      "start": 186,
                      "end": 235
                    },
                    "start": 165,
                    "end": 236
                  }
                ],
                "start": 159,
                "end": 238
              },
              "id": null,
              "generator": false,
              "start": 155,
              "end": 238
            },
            "definite": false,
            "start": 127,
            "end": 238
          }
        ],
        "declare": false,
        "start": 121,
        "end": 239
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 114,
      "end": 239
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 239
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    81,
    147
  ]
}
```

### Module: test.tsx_Foo_component_1_DvU6FitWglY.tsx (ENTRY POINT)

```tsx
export const Foo_component_1_DvU6FitWglY = ()=>{
    return <div>
			</div>;
};
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
              "name": "Foo_component_1_DvU6FitWglY",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 40
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
                          "name": "div",
                          "start": 61,
                          "end": 64
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 60,
                        "end": 65
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 65,
                          "end": 69
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 71,
                          "end": 74
                        },
                        "start": 69,
                        "end": 75
                      },
                      "start": 60,
                      "end": 75
                    },
                    "start": 53,
                    "end": 76
                  }
                ],
                "start": 47,
                "end": 78
              },
              "id": null,
              "generator": false,
              "start": 43,
              "end": 78
            },
            "definite": false,
            "start": 13,
            "end": 78
          }
        ],
        "declare": false,
        "start": 7,
        "end": 79
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 79
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 79
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_1_DvU6FitWglY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_1",
  "hash": "DvU6FitWglY",
  "canonicalFilename": "test.tsx_Foo_component_1_DvU6FitWglY",
  "path": "",
  "extension": "tsx",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    99,
    143
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"), {
    tagName: "my-foo"
});
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
            "name": "i_HTDRsvUbLiE",
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
                "value": "./test.tsx_Foo_component_HTDRsvUbLiE",
                "raw": "\"./test.tsx_Foo_component_HTDRsvUbLiE\"",
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
              "name": "Foo",
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
                      "name": "i_HTDRsvUbLiE",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "Foo_component_HTDRsvUbLiE",
                      "raw": "\"Foo_component_HTDRsvUbLiE\"",
                      "start": 238,
                      "end": 265
                    }
                  ],
                  "optional": false,
                  "start": 219,
                  "end": 266
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
                        "name": "tagName",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 274,
                        "end": 281
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 283,
                        "end": 291
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 274,
                      "end": 291
                    }
                  ],
                  "start": 268,
                  "end": 293
                }
              ],
              "optional": false,
              "start": 192,
              "end": 294
            },
            "definite": false,
            "start": 172,
            "end": 294
          }
        ],
        "declare": false,
        "start": 166,
        "end": 295
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 159,
      "end": 295
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 295
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (2 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (3 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx_Foo_component_HTDRsvUbLiE.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
