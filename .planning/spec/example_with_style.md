# Test: example_with_style

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

## Input

### Source Code

```tsx
import { $, component$, useStyles$ } from '@qwik.dev/core';

export const Foo = component$(() => {
	useStyles$('.class {}');
	return (
		<div class="class"/>
	);
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
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 34
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 34
          },
          "importKind": "value",
          "start": 24,
          "end": 34
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 42,
        "end": 58
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 59
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
              "start": 74,
              "end": 77
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 80,
                "end": 90
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
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useStyles$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 100,
                            "end": 110
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": ".class {}",
                              "raw": "'.class {}'",
                              "start": 111,
                              "end": 122
                            }
                          ],
                          "optional": false,
                          "start": 100,
                          "end": 123
                        },
                        "directive": null,
                        "start": 100,
                        "end": 124
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
                                "start": 138,
                                "end": 141
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "class",
                                    "start": 142,
                                    "end": 147
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": "class",
                                    "raw": "\"class\"",
                                    "start": 148,
                                    "end": 155
                                  },
                                  "start": 142,
                                  "end": 155
                                }
                              ],
                              "selfClosing": true,
                              "start": 137,
                              "end": 157
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 137,
                            "end": 157
                          },
                          "start": 133,
                          "end": 160
                        },
                        "start": 126,
                        "end": 161
                      }
                    ],
                    "start": 97,
                    "end": 163
                  },
                  "id": null,
                  "generator": false,
                  "start": 91,
                  "end": 163
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
                        "start": 168,
                        "end": 175
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 177,
                        "end": 185
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 168,
                      "end": 185
                    }
                  ],
                  "start": 165,
                  "end": 188
                }
              ],
              "optional": false,
              "start": 80,
              "end": 189
            },
            "definite": false,
            "start": 74,
            "end": 189
          }
        ],
        "declare": false,
        "start": 68,
        "end": 190
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 61,
      "end": 190
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 190
}
```

</details>

## Output

### Module: test.tsx_Foo_component_useStyles_pV9TSCBIhw8.tsx [ENTRY POINT]

```tsx
export const Foo_component_useStyles_pV9TSCBIhw8 = '.class {}';
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
              "name": "Foo_component_useStyles_pV9TSCBIhw8",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 48
            },
            "init": {
              "type": "Literal",
              "value": ".class {}",
              "raw": "'.class {}'",
              "start": 51,
              "end": 62
            },
            "definite": false,
            "start": 13,
            "end": 62
          }
        ],
        "declare": false,
        "start": 7,
        "end": 63
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 63
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 63
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_useStyles_pV9TSCBIhw8",
  "entry": null,
  "displayName": "test.tsx_Foo_component_useStyles",
  "hash": "pV9TSCBIhw8",
  "canonicalFilename": "test.tsx_Foo_component_useStyles_pV9TSCBIhw8",
  "path": "",
  "extension": "tsx",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "function",
  "ctxName": "useStyles$",
  "captures": false,
  "loc": [
    113,
    124
  ]
}
```

### Module: test.tsx_Foo_component_HTDRsvUbLiE.tsx [ENTRY POINT]

```tsx
import { qrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
const i_pV9TSCBIhw8 = ()=>import("./test.tsx_Foo_component_useStyles_pV9TSCBIhw8");
export const Foo_component_HTDRsvUbLiE = ()=>{
    useStylesQrl(/*#__PURE__*/ qrl(i_pV9TSCBIhw8, "Foo_component_useStyles_pV9TSCBIhw8"));
    return <div class="class"/>;
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStylesQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 47,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStylesQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 47,
            "end": 59
          },
          "importKind": "value",
          "start": 47,
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
      "start": 38,
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
            "name": "i_pV9TSCBIhw8",
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
                "value": "./test.tsx_Foo_component_useStyles_pV9TSCBIhw8",
                "raw": "\"./test.tsx_Foo_component_useStyles_pV9TSCBIhw8\"",
                "start": 118,
                "end": 166
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 167
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 167
          },
          "definite": false,
          "start": 91,
          "end": 167
        }
      ],
      "declare": false,
      "start": 85,
      "end": 168
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
              "start": 182,
              "end": 207
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
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "useStylesQrl",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 220,
                        "end": 232
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
                            "start": 247,
                            "end": 250
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "i_pV9TSCBIhw8",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 251,
                              "end": 264
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_useStyles_pV9TSCBIhw8",
                              "raw": "\"Foo_component_useStyles_pV9TSCBIhw8\"",
                              "start": 266,
                              "end": 303
                            }
                          ],
                          "optional": false,
                          "start": 247,
                          "end": 304
                        }
                      ],
                      "optional": false,
                      "start": 220,
                      "end": 305
                    },
                    "directive": null,
                    "start": 220,
                    "end": 306
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 319,
                          "end": 322
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "class",
                              "start": 323,
                              "end": 328
                            },
                            "value": {
                              "type": "Literal",
                              "value": "class",
                              "raw": "\"class\"",
                              "start": 329,
                              "end": 336
                            },
                            "start": 323,
                            "end": 336
                          }
                        ],
                        "selfClosing": true,
                        "start": 318,
                        "end": 338
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 318,
                      "end": 338
                    },
                    "start": 311,
                    "end": 339
                  }
                ],
                "start": 214,
                "end": 341
              },
              "id": null,
              "generator": false,
              "start": 210,
              "end": 341
            },
            "definite": false,
            "start": 182,
            "end": 341
          }
        ],
        "declare": false,
        "start": 176,
        "end": 342
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 169,
      "end": 342
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 342
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
    93,
    165
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
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `useStylesQrl`, `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (2 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (3 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx_Foo_component_HTDRsvUbLiE.tsx | @qwik.dev/core | 1 |
| `useStylesQrl` | test.tsx_Foo_component_HTDRsvUbLiE.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

No diagnostics.
