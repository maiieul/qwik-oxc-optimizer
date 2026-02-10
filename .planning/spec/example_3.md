# Test: example_3

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
export const App = () => {
	const Header = component$(() => {
		console.log("mount");
		return (
			<div onClick={$((ctx) => console.log(ctx))}/>
		);
	});
	return Header;
});
```

<details>
<summary>Input AST (OXC)</summary>

```json
AST generation failed (parser panic)
```

</details>

## Output

### Module: test.tsx_App_Header_component_B9F3YeqcO1w.tsx (ENTRY POINT)

```tsx
import { qrl } from "@qwik.dev/core";
const i_aO7uI7Iw6oQ = ()=>import("./test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ");
export const App_Header_component_B9F3YeqcO1w = ()=>{
    console.log("mount");
    return <div onClick={/*#__PURE__*/ qrl(i_aO7uI7Iw6oQ, "App_Header_component_div_onClick_aO7uI7Iw6oQ")}/>;
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
            "name": "i_aO7uI7Iw6oQ",
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
                "value": "./test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ",
                "raw": "\"./test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ\"",
                "start": 71,
                "end": 128
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 129
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 129
          },
          "definite": false,
          "start": 44,
          "end": 129
        }
      ],
      "declare": false,
      "start": 38,
      "end": 130
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
              "name": "App_Header_component_B9F3YeqcO1w",
              "optional": false,
              "typeAnnotation": null,
              "start": 144,
              "end": 176
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
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "console",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 189,
                          "end": 196
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "log",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 197,
                          "end": 200
                        },
                        "optional": false,
                        "computed": false,
                        "start": 189,
                        "end": 200
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "mount",
                          "raw": "\"mount\"",
                          "start": 201,
                          "end": 208
                        }
                      ],
                      "optional": false,
                      "start": 189,
                      "end": 209
                    },
                    "directive": null,
                    "start": 189,
                    "end": 210
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
                          "start": 223,
                          "end": 226
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick",
                              "start": 227,
                              "end": 234
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "qrl",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 250,
                                  "end": 253
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "i_aO7uI7Iw6oQ",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 254,
                                    "end": 267
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "App_Header_component_div_onClick_aO7uI7Iw6oQ",
                                    "raw": "\"App_Header_component_div_onClick_aO7uI7Iw6oQ\"",
                                    "start": 269,
                                    "end": 315
                                  }
                                ],
                                "optional": false,
                                "start": 250,
                                "end": 316
                              },
                              "start": 235,
                              "end": 317
                            },
                            "start": 227,
                            "end": 317
                          }
                        ],
                        "selfClosing": true,
                        "start": 222,
                        "end": 319
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 222,
                      "end": 319
                    },
                    "start": 215,
                    "end": 320
                  }
                ],
                "start": 183,
                "end": 322
              },
              "id": null,
              "generator": false,
              "start": 179,
              "end": 322
            },
            "definite": false,
            "start": 144,
            "end": 322
          }
        ],
        "declare": false,
        "start": 138,
        "end": 323
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 131,
      "end": 323
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 323
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_Header_component_B9F3YeqcO1w",
  "entry": null,
  "displayName": "test.tsx_App_Header_component",
  "hash": "B9F3YeqcO1w",
  "canonicalFilename": "test.tsx_App_Header_component_B9F3YeqcO1w",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    104,
    203
  ]
}
```

### Module: test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ.tsx (ENTRY POINT)

```tsx
export const App_Header_component_div_onClick_aO7uI7Iw6oQ = (ctx)=>console.log(ctx);
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
              "name": "App_Header_component_div_onClick_aO7uI7Iw6oQ",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 57
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
                  "start": 61,
                  "end": 64
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
                    "start": 67,
                    "end": 74
                  },
                  "property": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "log",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 75,
                    "end": 78
                  },
                  "optional": false,
                  "computed": false,
                  "start": 67,
                  "end": 78
                },
                "typeArguments": null,
                "arguments": [
                  {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "ctx",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 79,
                    "end": 82
                  }
                ],
                "optional": false,
                "start": 67,
                "end": 83
              },
              "id": null,
              "generator": false,
              "start": 60,
              "end": 83
            },
            "definite": false,
            "start": 13,
            "end": 83
          }
        ],
        "declare": false,
        "start": 7,
        "end": 84
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 84
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 84
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_Header_component_div_onClick_aO7uI7Iw6oQ",
  "entry": null,
  "displayName": "test.tsx_App_Header_component_div_onClick",
  "hash": "aO7uI7Iw6oQ",
  "canonicalFilename": "test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ",
  "path": "",
  "extension": "tsx",
  "parent": "App_Header_component_B9F3YeqcO1w",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    166,
    191
  ],
  "paramNames": [
    "ctx"
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_B9F3YeqcO1w = ()=>import("./test.tsx_App_Header_component_B9F3YeqcO1w");
export const App = ()=>{
    const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_B9F3YeqcO1w, "App_Header_component_B9F3YeqcO1w"));
    return Header;
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
            "name": "i_B9F3YeqcO1w",
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
                "value": "./test.tsx_App_Header_component_B9F3YeqcO1w",
                "raw": "\"./test.tsx_App_Header_component_B9F3YeqcO1w\"",
                "start": 118,
                "end": 163
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 164
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 164
          },
          "definite": false,
          "start": 91,
          "end": 164
        }
      ],
      "declare": false,
      "start": 85,
      "end": 165
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
              "start": 179,
              "end": 182
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
                          "name": "Header",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 201,
                          "end": 207
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "componentQrl",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 224,
                            "end": 236
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
                                "start": 251,
                                "end": 254
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "i_B9F3YeqcO1w",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 255,
                                  "end": 268
                                },
                                {
                                  "type": "Literal",
                                  "value": "App_Header_component_B9F3YeqcO1w",
                                  "raw": "\"App_Header_component_B9F3YeqcO1w\"",
                                  "start": 270,
                                  "end": 304
                                }
                              ],
                              "optional": false,
                              "start": 251,
                              "end": 305
                            }
                          ],
                          "optional": false,
                          "start": 224,
                          "end": 306
                        },
                        "definite": false,
                        "start": 201,
                        "end": 306
                      }
                    ],
                    "declare": false,
                    "start": 195,
                    "end": 307
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "Header",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 319,
                      "end": 325
                    },
                    "start": 312,
                    "end": 326
                  }
                ],
                "start": 189,
                "end": 328
              },
              "id": null,
              "generator": false,
              "start": 185,
              "end": 328
            },
            "definite": false,
            "start": 179,
            "end": 328
          }
        ],
        "declare": false,
        "start": 173,
        "end": 329
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 166,
      "end": 329
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 329
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
| `qrl` | test.tsx_App_Header_component_B9F3YeqcO1w.tsx | @qwik.dev/core | 1 |
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
