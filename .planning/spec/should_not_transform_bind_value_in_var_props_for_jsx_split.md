# Test: should_not_transform_bind_value_in_var_props_for_jsx_split

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { $, component$, useSignal } from "@qwik.dev/core";

export const FieldInput = component$((props) => {
	const input = useSignal("");

  return (
		<>
			{/* var props */}
			<input
				bind:value={input}
				{...props}
			/>
			{/* const props */}
			<input
				{...props}
				bind:value={input}
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
                                  "value": "",
                                  "raw": "\"\"",
                                  "start": 135,
                                  "end": 137
                                }
                              ],
                              "optional": false,
                              "start": 125,
                              "end": 138
                            },
                            "definite": false,
                            "start": 117,
                            "end": 138
                          }
                        ],
                        "declare": false,
                        "start": 111,
                        "end": 139
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 154,
                              "end": 156
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 156,
                                "end": 160
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "JSXEmptyExpression",
                                  "start": 161,
                                  "end": 176
                                },
                                "start": 160,
                                "end": 177
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 177,
                                "end": 181
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 182,
                                    "end": 187
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
                                          "start": 192,
                                          "end": 196
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "value",
                                          "start": 197,
                                          "end": 202
                                        },
                                        "start": 192,
                                        "end": 202
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "input",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 204,
                                          "end": 209
                                        },
                                        "start": 203,
                                        "end": 210
                                      },
                                      "start": 192,
                                      "end": 210
                                    },
                                    {
                                      "type": "JSXSpreadAttribute",
                                      "argument": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "props",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 219,
                                        "end": 224
                                      },
                                      "start": 215,
                                      "end": 225
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 181,
                                  "end": 231
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 181,
                                "end": 231
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 231,
                                "end": 235
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "JSXEmptyExpression",
                                  "start": 236,
                                  "end": 253
                                },
                                "start": 235,
                                "end": 254
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 254,
                                "end": 258
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "input",
                                    "start": 259,
                                    "end": 264
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
                                        "start": 273,
                                        "end": 278
                                      },
                                      "start": 269,
                                      "end": 279
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXNamespacedName",
                                        "namespace": {
                                          "type": "JSXIdentifier",
                                          "name": "bind",
                                          "start": 284,
                                          "end": 288
                                        },
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "value",
                                          "start": 289,
                                          "end": 294
                                        },
                                        "start": 284,
                                        "end": 294
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "input",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 296,
                                          "end": 301
                                        },
                                        "start": 295,
                                        "end": 302
                                      },
                                      "start": 284,
                                      "end": 302
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 258,
                                  "end": 308
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 258,
                                "end": 308
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 308,
                                "end": 311
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 311,
                              "end": 314
                            },
                            "start": 154,
                            "end": 314
                          },
                          "start": 150,
                          "end": 318
                        },
                        "start": 143,
                        "end": 319
                      }
                    ],
                    "start": 108,
                    "end": 321
                  },
                  "id": null,
                  "generator": false,
                  "start": 97,
                  "end": 321
                }
              ],
              "optional": false,
              "start": 86,
              "end": 322
            },
            "definite": false,
            "start": 73,
            "end": 322
          }
        ],
        "declare": false,
        "start": 67,
        "end": 323
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 60,
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

## Output

### Module: test.js

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

### Module: test.tsx_FieldInput_component_V4XAtJSTRKg.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
export const FieldInput_component_V4XAtJSTRKg = (props)=>{
    const input = useSignal("");
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSplit("input", {
            "bind:value": input,
            ..._getVarProps(props)
        }, _getConstProps(props), null, 0, null),
        /*#__PURE__*/ _jsxSplit("input", {
            ..._getVarProps(props),
            "bind:value": input
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
                              "value": "",
                              "raw": "\"\"",
                              "start": 384,
                              "end": 386
                            }
                          ],
                          "optional": false,
                          "start": 374,
                          "end": 387
                        },
                        "start": 366,
                        "end": 387
                      }
                    ],
                    "start": 360,
                    "end": 388
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 414,
                        "end": 424
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 425,
                          "end": 434
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 436,
                          "end": 440
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 442,
                          "end": 446
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSplit",
                                "start": 472,
                                "end": 481
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 482,
                                  "end": 489
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "bind:value",
                                        "raw": "\"bind:value\"",
                                        "start": 505,
                                        "end": 517
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "input",
                                        "start": 519,
                                        "end": 524
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 505,
                                      "end": 524
                                    },
                                    {
                                      "type": "SpreadElement",
                                      "argument": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_getVarProps",
                                          "start": 541,
                                          "end": 553
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "props",
                                            "start": 554,
                                            "end": 559
                                          }
                                        ],
                                        "optional": false,
                                        "start": 541,
                                        "end": 560
                                      },
                                      "start": 538,
                                      "end": 560
                                    }
                                  ],
                                  "start": 491,
                                  "end": 570
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_getConstProps",
                                    "start": 572,
                                    "end": 586
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "props",
                                      "start": 587,
                                      "end": 592
                                    }
                                  ],
                                  "optional": false,
                                  "start": 572,
                                  "end": 593
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 595,
                                  "end": 599
                                },
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 601,
                                  "end": 602
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 604,
                                  "end": 608
                                }
                              ],
                              "optional": false,
                              "start": 472,
                              "end": 609
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSplit",
                                "start": 633,
                                "end": 642
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "input",
                                  "raw": "\"input\"",
                                  "start": 643,
                                  "end": 650
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
                                          "start": 669,
                                          "end": 681
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "props",
                                            "start": 682,
                                            "end": 687
                                          }
                                        ],
                                        "optional": false,
                                        "start": 669,
                                        "end": 688
                                      },
                                      "start": 666,
                                      "end": 688
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "bind:value",
                                        "raw": "\"bind:value\"",
                                        "start": 702,
                                        "end": 714
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "input",
                                        "start": 716,
                                        "end": 721
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 702,
                                      "end": 721
                                    }
                                  ],
                                  "start": 652,
                                  "end": 731
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_getConstProps",
                                    "start": 733,
                                    "end": 747
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "props",
                                      "start": 748,
                                      "end": 753
                                    }
                                  ],
                                  "optional": false,
                                  "start": 733,
                                  "end": 754
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 756,
                                  "end": 760
                                },
                                {
                                  "type": "Literal",
                                  "value": 0,
                                  "raw": "0",
                                  "start": 762,
                                  "end": 763
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 765,
                                  "end": 769
                                }
                              ],
                              "optional": false,
                              "start": 633,
                              "end": 770
                            }
                          ],
                          "start": 448,
                          "end": 776
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 778,
                          "end": 779
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 781,
                          "end": 787
                        }
                      ],
                      "optional": false,
                      "start": 414,
                      "end": 788
                    },
                    "start": 393,
                    "end": 789
                  }
                ],
                "start": 354,
                "end": 791
              },
              "id": null,
              "generator": false,
              "start": 345,
              "end": 791
            },
            "start": 310,
            "end": 791
          }
        ],
        "start": 304,
        "end": 792
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 297,
      "end": 792
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 792
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
    323
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
