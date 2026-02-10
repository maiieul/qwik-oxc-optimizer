# Test: should_not_transform_bind_checked_in_var_props_for_jsx_split

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, useSignal } from "@qwik.dev/core";

export const FieldInput = component$((props) => {
	const input = useSignal(true);

  return (
		<>
			{/* var props */}
			<input
				bind:checked={input}
				{...props}
			/>
			{/* const props */}
			<input
				{...props}
				bind:checked={input}
			/>
		</>
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
        "raw": "\"@qwik.dev/core\"",
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
              "name": "FieldInput",
              "optional": false,
              "typeAnnotation": null,
              "start": 73,
              "end": 83
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 86,
                "end": 96
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "ArrowFunctionExpression",
                  "expression": false,
                  "async": false,
                  "typeParameters": null,
                  "params": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "props",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 98,
                      "end": 103
                    }
                  ],
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
                              "name": "input",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 117,
                              "end": 122
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 125,
                                "end": 134
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": true,
                                  "raw": "true",
                                  "start": 135,
                                  "end": 139
                                }
                              ],
                              "optional": false,
                              "start": 125,
                              "end": 140
                            },
                            "definite": false,
                            "start": 117,
                            "end": 140
                          }
                        ],
                        "declare": false,
                        "start": 111,
                        "end": 141
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 156,
                              "end": 158
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 158,
                                "end": 162
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "JSXEmptyExpression",
                                  "start": 163,
                                  "end": 178
                                },
                                "start": 162,
                                "end": 179
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 179,
                                "end": 183
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 184,
                                    "end": 189
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 194,
                                          "end": 198
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "checked",
                                          "start": 199,
                                          "end": 206
                                        },
                                        "start": 194,
                                        "end": 206
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "input",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 208,
                                          "end": 213
                                        },
                                        "start": 207,
                                        "end": 214
                                      },
                                      "start": 194,
                                      "end": 214
                                    },
                                    {
                                      "type": "JSXSpreadAttribute",
                                      "argument": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "props",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 223,
                                        "end": 228
                                      },
                                      "start": 219,
                                      "end": 229
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 183,
                                  "end": 235
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 183,
                                "end": 235
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 235,
                                "end": 239
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "JSXEmptyExpression",
                                  "start": 240,
                                  "end": 257
                                },
                                "start": 239,
                                "end": 258
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 258,
                                "end": 262
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 263,
                                    "end": 268
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXSpreadAttribute",
                                      "argument": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "props",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 277,
                                        "end": 282
                                      },
                                      "start": 273,
                                      "end": 283
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 288,
                                          "end": 292
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "checked",
                                          "start": 293,
                                          "end": 300
                                        },
                                        "start": 288,
                                        "end": 300
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "input",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 302,
                                          "end": 307
                                        },
                                        "start": 301,
                                        "end": 308
                                      },
                                      "start": 288,
                                      "end": 308
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 262,
                                  "end": 314
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 262,
                                "end": 314
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 314,
                                "end": 317
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 317,
                              "end": 320
                            },
                            "start": 156,
                            "end": 320
                          },
                          "start": 152,
                          "end": 324
                        },
                        "start": 145,
                        "end": 325
                      }
                    ],
                    "start": 108,
                    "end": 327
                  },
                  "id": null,
                  "generator": false,
                  "start": 97,
                  "end": 327
                }
              ],
              "optional": false,
              "start": 86,
              "end": 328
            },
            "definite": false,
            "start": 73,
            "end": 328
          }
        ],
        "declare": false,
        "start": 67,
        "end": 329
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 60,
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

## Output

### Module: `test.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_V4XAtJSTRKg = ()=>import("./test.tsx_FieldInput_component_V4XAtJSTRKg");
export const FieldInput = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_V4XAtJSTRKg, "FieldInput_component_V4XAtJSTRKg"));
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
            "name": "qrl",
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 56,
            "end": 59
          },
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
            "name": "i_V4XAtJSTRKg",
            "start": 91,
            "end": 104
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [],
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_FieldInput_component_V4XAtJSTRKg",
                "raw": "\"./test.tsx_FieldInput_component_V4XAtJSTRKg\"",
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
          "start": 91,
          "end": 164
        }
      ],
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
              "name": "FieldInput",
              "start": 179,
              "end": 189
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 206,
                "end": 218
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 233,
                    "end": 236
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_V4XAtJSTRKg",
                      "start": 237,
                      "end": 250
                    },
                    {
                      "type": "Literal",
                      "value": "FieldInput_component_V4XAtJSTRKg",
                      "raw": "\"FieldInput_component_V4XAtJSTRKg\"",
                      "start": 252,
                      "end": 286
                    }
                  ],
                  "optional": false,
                  "start": 233,
                  "end": 287
                }
              ],
              "optional": false,
              "start": 206,
              "end": 288
            },
            "start": 179,
            "end": 288
          }
        ],
        "start": 173,
        "end": 289
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 166,
      "end": 289
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 289
}
```

</details>

### Module: `test.tsx_FieldInput_component_V4XAtJSTRKg.js` (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
export const FieldInput_component_V4XAtJSTRKg = (props)=>{
    const input = useSignal(true);
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSplit("input", {
            "bind:checked": input,
            ..._getVarProps(props)
        }, _getConstProps(props), null, 0, null),
        /*#__PURE__*/ _jsxSplit("input", {
            ..._getVarProps(props),
            "bind:checked": input
        }, _getConstProps(props), null, 0, null)
    ], 1, "u6_0");
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
            "name": "Fragment",
            "start": 9,
            "end": 17
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 21,
            "end": 30
          },
          "start": 9,
          "end": 30
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 38,
        "end": 66
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 67
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 77,
            "end": 91
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 77,
            "end": 91
          },
          "start": 77,
          "end": 91
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 99,
        "end": 115
      },
      "phase": null,
      "attributes": [],
      "start": 68,
      "end": 116
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 126,
            "end": 138
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 126,
            "end": 138
          },
          "start": 126,
          "end": 138
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 146,
        "end": 162
      },
      "phase": null,
      "attributes": [],
      "start": 117,
      "end": 163
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 173,
            "end": 183
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 173,
            "end": 183
          },
          "start": 173,
          "end": 183
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 191,
        "end": 207
      },
      "phase": null,
      "attributes": [],
      "start": 164,
      "end": 208
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 218,
            "end": 227
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 218,
            "end": 227
          },
          "start": 218,
          "end": 227
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 235,
        "end": 251
      },
      "phase": null,
      "attributes": [],
      "start": 209,
      "end": 252
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 262,
            "end": 271
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 262,
            "end": 271
          },
          "start": 262,
          "end": 271
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 279,
        "end": 295
      },
      "phase": null,
      "attributes": [],
      "start": 253,
      "end": 296
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
              "name": "FieldInput_component_V4XAtJSTRKg",
              "start": 310,
              "end": 342
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 346,
                  "end": 351
                }
              ],
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
                          "name": "input",
                          "start": 366,
                          "end": 371
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 374,
                            "end": 383
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": true,
                              "raw": "true",
                              "start": 384,
                              "end": 388
                            }
                          ],
                          "optional": false,
                          "start": 374,
                          "end": 389
                        },
                        "start": 366,
                        "end": 389
                      }
                    ],
                    "start": 360,
                    "end": 390
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 416,
                        "end": 426
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 427,
                          "end": 436
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 438,
                          "end": 442
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 444,
                          "end": 448
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSplit",
                                "start": 474,
                                "end": 483
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 484,
                                  "end": 491
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "bind:checked",
                                        "raw": "\"bind:checked\"",
                                        "start": 507,
                                        "end": 521
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "input",
                                        "start": 523,
                                        "end": 528
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 507,
                                      "end": 528
                                    },
                                    {
                                      "type": "SpreadElement",
                                      "argument": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_getVarProps",
                                          "start": 545,
                                          "end": 557
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "props",
                                            "start": 558,
                                            "end": 563
                                          }
                                        ],
                                        "optional": false,
                                        "start": 545,
                                        "end": 564
                                      },
                                      "start": 542,
                                      "end": 564
                                    }
                                  ],
                                  "start": 493,
                                  "end": 574
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_getConstProps",
                                    "start": 576,
                                    "end": 590
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "props",
                                      "start": 591,
                                      "end": 596
                                    }
                                  ],
                                  "optional": false,
                                  "start": 576,
                                  "end": 597
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 599,
                                  "end": 603
                                },
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 605,
                                  "end": 606
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 608,
                                  "end": 612
                                }
                              ],
                              "optional": false,
                              "start": 474,
                              "end": 613
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSplit",
                                "start": 637,
                                "end": 646
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 647,
                                  "end": 654
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "SpreadElement",
                                      "argument": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_getVarProps",
                                          "start": 673,
                                          "end": 685
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "props",
                                            "start": 686,
                                            "end": 691
                                          }
                                        ],
                                        "optional": false,
                                        "start": 673,
                                        "end": 692
                                      },
                                      "start": 670,
                                      "end": 692
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "bind:checked",
                                        "raw": "\"bind:checked\"",
                                        "start": 706,
                                        "end": 720
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "input",
                                        "start": 722,
                                        "end": 727
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 706,
                                      "end": 727
                                    }
                                  ],
                                  "start": 656,
                                  "end": 737
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_getConstProps",
                                    "start": 739,
                                    "end": 753
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "props",
                                      "start": 754,
                                      "end": 759
                                    }
                                  ],
                                  "optional": false,
                                  "start": 739,
                                  "end": 760
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 762,
                                  "end": 766
                                },
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 768,
                                  "end": 769
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 771,
                                  "end": 775
                                }
                              ],
                              "optional": false,
                              "start": 637,
                              "end": 776
                            }
                          ],
                          "start": 450,
                          "end": 782
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 784,
                          "end": 785
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 787,
                          "end": 793
                        }
                      ],
                      "optional": false,
                      "start": 416,
                      "end": 794
                    },
                    "start": 395,
                    "end": 795
                  }
                ],
                "start": 354,
                "end": 797
              },
              "id": null,
              "generator": false,
              "start": 345,
              "end": 797
            },
            "start": 310,
            "end": 797
          }
        ],
        "start": 304,
        "end": 798
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 297,
      "end": 798
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 798
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "FieldInput_component_V4XAtJSTRKg",
  "entry": null,
  "displayName": "test.tsx_FieldInput_component",
  "hash": "V4XAtJSTRKg",
  "canonicalFilename": "test.tsx_FieldInput_component_V4XAtJSTRKg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    99,
    329
  ],
  "paramNames": [
    "props"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`, `_jsxSplit()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_getVarProps()`, `_getConstProps()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 5 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)
- **[CONV-12] Input Binding**: Input binding via `bind:` directives

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_jsxSplit` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |
| `_getVarProps` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |
| `_getConstProps` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |
| `useSignal` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
