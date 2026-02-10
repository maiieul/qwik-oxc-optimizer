# Test: ternary_prop

## Test Configuration

**Note:** Ternary expression in data-open prop with signal value. Tests _fnSignal with hoisted function for reactive ternary prop.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, $, useSignal } from '@qwik.dev/core';
		export const Cmp = component$(() => {
			const toggleSig = useSignal(false);

			const handleClick$ = $(() => {
				toggleSig.value = !toggleSig.value;
			});

			return (
				<button onClick$={handleClick$} data-open={toggleSig.value ? true : undefined}>
					Removing data-open re-renders
				</button>
			);
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
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "importKind": "value",
          "start": 21,
          "end": 22
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 33
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 33
          },
          "importKind": "value",
          "start": 24,
          "end": 33
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 41,
        "end": 57
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 58
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
              "name": "Cmp",
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
                        "type": "VariableDeclaration",
                        "kind": "const",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "toggleSig",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 108,
                              "end": 117
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 120,
                                "end": 129
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": false,
                                  "raw": "false",
                                  "start": 130,
                                  "end": 135
                                }
                              ],
                              "optional": false,
                              "start": 120,
                              "end": 136
                            },
                            "definite": false,
                            "start": 108,
                            "end": 136
                          }
                        ],
                        "declare": false,
                        "start": 102,
                        "end": 137
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
                              "name": "handleClick$",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 148,
                              "end": 160
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 163,
                                "end": 164
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
                                          "type": "AssignmentExpression",
                                          "operator": "=",
                                          "left": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "toggleSig",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 177,
                                              "end": 186
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "value",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 187,
                                              "end": 192
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 177,
                                            "end": 192
                                          },
                                          "right": {
                                            "type": "UnaryExpression",
                                            "operator": "!",
                                            "argument": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "toggleSig",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 196,
                                                "end": 205
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "value",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 206,
                                                "end": 211
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 196,
                                              "end": 211
                                            },
                                            "prefix": true,
                                            "start": 195,
                                            "end": 211
                                          },
                                          "start": 177,
                                          "end": 211
                                        },
                                        "directive": null,
                                        "start": 177,
                                        "end": 212
                                      }
                                    ],
                                    "start": 171,
                                    "end": 217
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 165,
                                  "end": 217
                                }
                              ],
                              "optional": false,
                              "start": 163,
                              "end": 218
                            },
                            "definite": false,
                            "start": 148,
                            "end": 218
                          }
                        ],
                        "declare": false,
                        "start": 142,
                        "end": 219
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
                                "name": "button",
                                "start": 238,
                                "end": 244
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 245,
                                    "end": 253
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "handleClick$",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 255,
                                      "end": 267
                                    },
                                    "start": 254,
                                    "end": 268
                                  },
                                  "start": 245,
                                  "end": 268
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "data-open",
                                    "start": 269,
                                    "end": 278
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "toggleSig",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 280,
                                          "end": 289
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 290,
                                          "end": 295
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 280,
                                        "end": 295
                                      },
                                      "consequent": {
                                        "type": "Literal",
                                        "value": true,
                                        "raw": "true",
                                        "start": 298,
                                        "end": 302
                                      },
                                      "alternate": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "undefined",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 305,
                                        "end": 314
                                      },
                                      "start": 280,
                                      "end": 314
                                    },
                                    "start": 279,
                                    "end": 315
                                  },
                                  "start": 269,
                                  "end": 315
                                }
                              ],
                              "selfClosing": false,
                              "start": 237,
                              "end": 316
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t\tRemoving data-open re-renders\n\t\t\t\t",
                                "raw": "\n\t\t\t\t\tRemoving data-open re-renders\n\t\t\t\t",
                                "start": 316,
                                "end": 356
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "button",
                                "start": 358,
                                "end": 364
                              },
                              "start": 356,
                              "end": 365
                            },
                            "start": 237,
                            "end": 365
                          },
                          "start": 231,
                          "end": 370
                        },
                        "start": 224,
                        "end": 371
                      }
                    ],
                    "start": 97,
                    "end": 375
                  },
                  "id": null,
                  "generator": false,
                  "start": 91,
                  "end": 375
                }
              ],
              "optional": false,
              "start": 80,
              "end": 376
            },
            "definite": false,
            "start": 74,
            "end": 376
          }
        ],
        "declare": false,
        "start": 68,
        "end": 377
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 61,
      "end": 377
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 377
}

```

</details>

## Output

### Module: test.tsx_Cmp_component_handleClick_WawHV3HwS1A.ts (ENTRY POINT)

```ts
import { _captures } from "@qwik.dev/core";
export const Cmp_component_handleClick_WawHV3HwS1A = ()=>{
    const toggleSig = _captures[0];
    toggleSig.value = !toggleSig.value;
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
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "importKind": "value",
          "start": 9,
          "end": 18
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 26,
        "end": 42
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 43
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
              "name": "Cmp_component_handleClick_WawHV3HwS1A",
              "optional": false,
              "typeAnnotation": null,
              "start": 57,
              "end": 94
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
                          "name": "toggleSig",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 113,
                          "end": 122
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_captures",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 125,
                            "end": 134
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 135,
                            "end": 136
                          },
                          "optional": false,
                          "computed": true,
                          "start": 125,
                          "end": 137
                        },
                        "definite": false,
                        "start": 113,
                        "end": 137
                      }
                    ],
                    "declare": false,
                    "start": 107,
                    "end": 138
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "toggleSig",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 143,
                          "end": 152
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "value",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 153,
                          "end": 158
                        },
                        "optional": false,
                        "computed": false,
                        "start": 143,
                        "end": 158
                      },
                      "right": {
                        "type": "UnaryExpression",
                        "operator": "!",
                        "argument": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "toggleSig",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 162,
                            "end": 171
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "value",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 172,
                            "end": 177
                          },
                          "optional": false,
                          "computed": false,
                          "start": 162,
                          "end": 177
                        },
                        "prefix": true,
                        "start": 161,
                        "end": 177
                      },
                      "start": 143,
                      "end": 177
                    },
                    "directive": null,
                    "start": 143,
                    "end": 178
                  }
                ],
                "start": 101,
                "end": 180
              },
              "id": null,
              "generator": false,
              "start": 97,
              "end": 180
            },
            "definite": false,
            "start": 57,
            "end": 180
          }
        ],
        "declare": false,
        "start": 51,
        "end": 181
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 44,
      "end": 181
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 181
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_handleClick_WawHV3HwS1A",
  "entry": null,
  "displayName": "test.tsx_Cmp_component_handleClick",
  "hash": "WawHV3HwS1A",
  "canonicalFilename": "test.tsx_Cmp_component_handleClick_WawHV3HwS1A",
  "path": "",
  "extension": "ts",
  "parent": "Cmp_component_4ryKJTOKjWE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    169,
    221
  ],
  "captureNames": [
    "toggleSig"
  ]
}
```

### Module: test.tsx_Cmp_component_4ryKJTOKjWE.ts (ENTRY POINT)

```ts
import { _fnSignal } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const _hf0 = (p0)=>p0.value ? true : undefined;
const _hf0_str = "p0.value?true:undefined";
const i_WawHV3HwS1A = ()=>import("./test.tsx_Cmp_component_handleClick_WawHV3HwS1A");
export const Cmp_component_4ryKJTOKjWE = ()=>{
    const toggleSig = useSignal(false);
    const handleClick$ = /*#__PURE__*/ qrl(i_WawHV3HwS1A, "Cmp_component_handleClick_WawHV3HwS1A", [
        toggleSig
    ]);
    return /*#__PURE__*/ _jsxSorted("button", null, {
        "q-e:click": handleClick$,
        "data-open": _fnSignal(_hf0, [
            toggleSig
        ], _hf0_str)
    }, "Removing data-open re-renders", 3, "u6_0");
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
            "name": "_fnSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_fnSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "importKind": "value",
          "start": 9,
          "end": 18
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 26,
        "end": 42
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 43
    },
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
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 53,
            "end": 63
          },
          "importKind": "value",
          "start": 53,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 44,
      "end": 88
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
            "start": 98,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 98,
            "end": 101
          },
          "importKind": "value",
          "start": 98,
          "end": 101
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 109,
        "end": 125
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 89,
      "end": 126
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 136,
            "end": 145
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 136,
            "end": 145
          },
          "importKind": "value",
          "start": 136,
          "end": 145
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 153,
        "end": 169
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 127,
      "end": 170
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
            "name": "_hf0",
            "optional": false,
            "typeAnnotation": null,
            "start": 177,
            "end": 181
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
                "name": "p0",
                "optional": false,
                "typeAnnotation": null,
                "start": 185,
                "end": 187
              }
            ],
            "returnType": null,
            "body": {
              "type": "ConditionalExpression",
              "test": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "p0",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 190,
                  "end": 192
                },
                "property": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "value",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 193,
                  "end": 198
                },
                "optional": false,
                "computed": false,
                "start": 190,
                "end": 198
              },
              "consequent": {
                "type": "Literal",
                "value": true,
                "raw": "true",
                "start": 201,
                "end": 205
              },
              "alternate": {
                "type": "Identifier",
                "decorators": [],
                "name": "undefined",
                "optional": false,
                "typeAnnotation": null,
                "start": 208,
                "end": 217
              },
              "start": 190,
              "end": 217
            },
            "id": null,
            "generator": false,
            "start": 184,
            "end": 217
          },
          "definite": false,
          "start": 177,
          "end": 217
        }
      ],
      "declare": false,
      "start": 171,
      "end": 218
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
            "name": "_hf0_str",
            "optional": false,
            "typeAnnotation": null,
            "start": 225,
            "end": 233
          },
          "init": {
            "type": "Literal",
            "value": "p0.value?true:undefined",
            "raw": "\"p0.value?true:undefined\"",
            "start": 236,
            "end": 261
          },
          "definite": false,
          "start": 225,
          "end": 261
        }
      ],
      "declare": false,
      "start": 219,
      "end": 262
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
            "name": "i_WawHV3HwS1A",
            "optional": false,
            "typeAnnotation": null,
            "start": 269,
            "end": 282
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
                "value": "./test.tsx_Cmp_component_handleClick_WawHV3HwS1A",
                "raw": "\"./test.tsx_Cmp_component_handleClick_WawHV3HwS1A\"",
                "start": 296,
                "end": 346
              },
              "options": null,
              "phase": null,
              "start": 289,
              "end": 347
            },
            "id": null,
            "generator": false,
            "start": 285,
            "end": 347
          },
          "definite": false,
          "start": 269,
          "end": 347
        }
      ],
      "declare": false,
      "start": 263,
      "end": 348
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
              "name": "Cmp_component_4ryKJTOKjWE",
              "optional": false,
              "typeAnnotation": null,
              "start": 362,
              "end": 387
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
                          "name": "toggleSig",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 406,
                          "end": 415
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useSignal",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 418,
                            "end": 427
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": false,
                              "raw": "false",
                              "start": 428,
                              "end": 433
                            }
                          ],
                          "optional": false,
                          "start": 418,
                          "end": 434
                        },
                        "definite": false,
                        "start": 406,
                        "end": 434
                      }
                    ],
                    "declare": false,
                    "start": 400,
                    "end": 435
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
                          "name": "handleClick$",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 446,
                          "end": 458
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "qrl",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 475,
                            "end": 478
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "i_WawHV3HwS1A",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 479,
                              "end": 492
                            },
                            {
                              "type": "Literal",
                              "value": "Cmp_component_handleClick_WawHV3HwS1A",
                              "raw": "\"Cmp_component_handleClick_WawHV3HwS1A\"",
                              "start": 494,
                              "end": 533
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "toggleSig",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 545,
                                  "end": 554
                                }
                              ],
                              "start": 535,
                              "end": 560
                            }
                          ],
                          "optional": false,
                          "start": 475,
                          "end": 561
                        },
                        "definite": false,
                        "start": 446,
                        "end": 561
                      }
                    ],
                    "declare": false,
                    "start": 440,
                    "end": 562
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "_jsxSorted",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 588,
                        "end": 598
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "button",
                          "raw": "\"button\"",
                          "start": 599,
                          "end": 607
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 609,
                          "end": 613
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:click",
                                "raw": "\"q-e:click\"",
                                "start": 625,
                                "end": 636
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "handleClick$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 638,
                                "end": 650
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "optional": false,
                              "start": 625,
                              "end": 650
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "data-open",
                                "raw": "\"data-open\"",
                                "start": 660,
                                "end": 671
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "_fnSignal",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 673,
                                  "end": 682
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "_hf0",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 683,
                                    "end": 687
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "toggleSig",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 703,
                                        "end": 712
                                      }
                                    ],
                                    "start": 689,
                                    "end": 722
                                  },
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "_hf0_str",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 724,
                                    "end": 732
                                  }
                                ],
                                "optional": false,
                                "start": 673,
                                "end": 733
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "optional": false,
                              "start": 660,
                              "end": 733
                            }
                          ],
                          "start": 615,
                          "end": 739
                        },
                        {
                          "type": "Literal",
                          "value": "Removing data-open re-renders",
                          "raw": "\"Removing data-open re-renders\"",
                          "start": 741,
                          "end": 772
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 774,
                          "end": 775
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 777,
                          "end": 783
                        }
                      ],
                      "optional": false,
                      "start": 588,
                      "end": 784
                    },
                    "start": 567,
                    "end": 785
                  }
                ],
                "start": 394,
                "end": 787
              },
              "id": null,
              "generator": false,
              "start": 390,
              "end": 787
            },
            "definite": false,
            "start": 362,
            "end": 787
          }
        ],
        "declare": false,
        "start": 356,
        "end": 788
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 349,
      "end": 788
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 788
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_4ryKJTOKjWE",
  "entry": null,
  "displayName": "test.tsx_Cmp_component",
  "hash": "4ryKJTOKjWE",
  "canonicalFilename": "test.tsx_Cmp_component_4ryKJTOKjWE",
  "path": "",
  "extension": "ts",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    95,
    379
  ]
}
```

### Module: test.ts

```ts
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_4ryKJTOKjWE = ()=>import("./test.tsx_Cmp_component_4ryKJTOKjWE");
export const Cmp = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_4ryKJTOKjWE, "Cmp_component_4ryKJTOKjWE"));
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
            "name": "i_4ryKJTOKjWE",
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
                "value": "./test.tsx_Cmp_component_4ryKJTOKjWE",
                "raw": "\"./test.tsx_Cmp_component_4ryKJTOKjWE\"",
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
              "name": "Cmp",
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
                      "name": "i_4ryKJTOKjWE",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "Cmp_component_4ryKJTOKjWE",
                      "raw": "\"Cmp_component_4ryKJTOKjWE\"",
                      "start": 238,
                      "end": 265
                    }
                  ],
                  "optional": false,
                  "start": 219,
                  "end": 266
                }
              ],
              "optional": false,
              "start": 192,
              "end": 267
            },
            "definite": false,
            "start": 172,
            "end": 267
          }
        ],
        "declare": false,
        "start": 166,
        "end": 268
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 159,
      "end": 268
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 268
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-04] Signal Helpers**
- **[CONV-05] Capture Patterns**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**
- **[CONV-14] Hoisted Functions**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| qrl | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| _fnSignal | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| useSignal | test.tsx_Cmp_component_4ryKJTOKjWE.ts | @qwik.dev/core | 1 |
| componentQrl | test.ts | @qwik.dev/core | 1 |
| qrl | test.ts | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
