# Test: should_transform_qrls_in_ternary_expression

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, useSignal } from "@qwik.dev/core";

export const FieldInput = component$(() => {
	const enabled = useSignal(false);
	const input = useSignal("");

  return (
     <input
		id="input"
		onInput$={
		enabled.value
			? $((ev, el) => {
				input.value = el.value;
			})
			: undefined
		}
		onFocus$={() => {
			enabled.value = true;
		}}
	/>
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
                              "name": "enabled",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 112,
                              "end": 119
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 122,
                                "end": 131
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": false,
                                  "raw": "false",
                                  "start": 132,
                                  "end": 137
                                }
                              ],
                              "optional": false,
                              "start": 122,
                              "end": 138
                            },
                            "definite": false,
                            "start": 112,
                            "end": 138
                          }
                        ],
                        "declare": false,
                        "start": 106,
                        "end": 139
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
                              "name": "input",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 147,
                              "end": 152
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 155,
                                "end": 164
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "",
                                  "raw": "\"\"",
                                  "start": 165,
                                  "end": 167
                                }
                              ],
                              "optional": false,
                              "start": 155,
                              "end": 168
                            },
                            "definite": false,
                            "start": 147,
                            "end": 168
                          }
                        ],
                        "declare": false,
                        "start": 141,
                        "end": 169
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
                                "name": "input",
                                "start": 188,
                                "end": 193
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "id",
                                    "start": 196,
                                    "end": 198
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": "input",
                                    "raw": "\"input\"",
                                    "start": 199,
                                    "end": 206
                                  },
                                  "start": 196,
                                  "end": 206
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onInput$",
                                    "start": 209,
                                    "end": 217
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
                                          "name": "enabled",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 222,
                                          "end": 229
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 230,
                                          "end": 235
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 222,
                                        "end": 235
                                      },
                                      "consequent": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "$",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 241,
                                          "end": 242
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
                                                "name": "ev",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 244,
                                                "end": 246
                                              },
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "el",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 248,
                                                "end": 250
                                              }
                                            ],
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
                                                        "name": "input",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 261,
                                                        "end": 266
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 267,
                                                        "end": 272
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 261,
                                                      "end": 272
                                                    },
                                                    "right": {
                                                      "type": "MemberExpression",
                                                      "object": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "el",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 275,
                                                        "end": 277
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "value",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 278,
                                                        "end": 283
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 275,
                                                      "end": 283
                                                    },
                                                    "start": 261,
                                                    "end": 283
                                                  },
                                                  "directive": null,
                                                  "start": 261,
                                                  "end": 284
                                                }
                                              ],
                                              "start": 255,
                                              "end": 289
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 243,
                                            "end": 289
                                          }
                                        ],
                                        "optional": false,
                                        "start": 241,
                                        "end": 290
                                      },
                                      "alternate": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "undefined",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 296,
                                        "end": 305
                                      },
                                      "start": 222,
                                      "end": 305
                                    },
                                    "start": 218,
                                    "end": 309
                                  },
                                  "start": 209,
                                  "end": 309
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onFocus$",
                                    "start": 312,
                                    "end": 320
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
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
                                                  "name": "enabled",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 333,
                                                  "end": 340
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "value",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 341,
                                                  "end": 346
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 333,
                                                "end": 346
                                              },
                                              "right": {
                                                "type": "Literal",
                                                "value": true,
                                                "raw": "true",
                                                "start": 349,
                                                "end": 353
                                              },
                                              "start": 333,
                                              "end": 353
                                            },
                                            "directive": null,
                                            "start": 333,
                                            "end": 354
                                          }
                                        ],
                                        "start": 328,
                                        "end": 358
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 322,
                                      "end": 358
                                    },
                                    "start": 321,
                                    "end": 359
                                  },
                                  "start": 312,
                                  "end": 359
                                }
                              ],
                              "selfClosing": true,
                              "start": 187,
                              "end": 363
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 187,
                            "end": 363
                          },
                          "start": 180,
                          "end": 367
                        },
                        "start": 173,
                        "end": 368
                      }
                    ],
                    "start": 103,
                    "end": 370
                  },
                  "id": null,
                  "generator": false,
                  "start": 97,
                  "end": 370
                }
              ],
              "optional": false,
              "start": 86,
              "end": 371
            },
            "definite": false,
            "start": 73,
            "end": 371
          }
        ],
        "declare": false,
        "start": 67,
        "end": 372
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 60,
      "end": 372
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 372
}
```

</details>

## Output

### Module: `test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const FieldInput_component_input_q_e_focus_Sgf3MDWzexI = ()=>{
    const enabled = _captures[0];
    enabled.value = true;
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
            "name": "_captures",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_captures",
            "start": 9,
            "end": 18
          },
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
              "name": "FieldInput_component_input_q_e_focus_Sgf3MDWzexI",
              "start": 57,
              "end": 105
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
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
                          "name": "enabled",
                          "start": 124,
                          "end": 131
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 134,
                            "end": 143
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 144,
                            "end": 145
                          },
                          "optional": false,
                          "computed": true,
                          "start": 134,
                          "end": 146
                        },
                        "start": 124,
                        "end": 146
                      }
                    ],
                    "start": 118,
                    "end": 147
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
                          "name": "enabled",
                          "start": 152,
                          "end": 159
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 160,
                          "end": 165
                        },
                        "optional": false,
                        "computed": false,
                        "start": 152,
                        "end": 165
                      },
                      "right": {
                        "type": "Literal",
                        "value": true,
                        "raw": "true",
                        "start": 168,
                        "end": 172
                      },
                      "start": 152,
                      "end": 172
                    },
                    "start": 152,
                    "end": 173
                  }
                ],
                "start": 112,
                "end": 175
              },
              "id": null,
              "generator": false,
              "start": 108,
              "end": 175
            },
            "start": 57,
            "end": 175
          }
        ],
        "start": 51,
        "end": 176
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 176
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 176
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "FieldInput_component_input_q_e_focus_Sgf3MDWzexI",
  "entry": null,
  "displayName": "test.tsx_FieldInput_component_input_q_e_focus",
  "hash": "Sgf3MDWzexI",
  "canonicalFilename": "test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI",
  "path": "",
  "extension": "js",
  "parent": "FieldInput_component_V4XAtJSTRKg",
  "ctxKind": "eventHandler",
  "ctxName": "onFocus$",
  "captures": true,
  "loc": [
    324,
    360
  ],
  "captureNames": [
    "enabled"
  ]
}
```

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
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_Sgf3MDWzexI = ()=>import("./test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI");
const i_wqR1xEjZjf4 = ()=>import("./test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4");
export const FieldInput_component_V4XAtJSTRKg = ()=>{
    const enabled = useSignal(false);
    const input = useSignal("");
    return /*#__PURE__*/ _jsxSorted("input", {
        "q-e:input": enabled.value ? /*#__PURE__*/ qrl(i_wqR1xEjZjf4, "FieldInput_component_input_q_e_input_wqR1xEjZjf4", [
            input
        ]) : undefined
    }, {
        id: "input",
        "q-e:focus": /*#__PURE__*/ qrl(i_Sgf3MDWzexI, "FieldInput_component_input_q_e_focus_Sgf3MDWzexI", [
            enabled
        ])
    }, null, 2, "u6_0");
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
            "name": "_jsxSorted",
            "start": 9,
            "end": 19
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
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
        "raw": "\"@qwik.dev/core\"",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 44
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "start": 54,
          "end": 57
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 65,
        "end": 81
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 82
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 92,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 92,
            "end": 101
          },
          "start": 92,
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
      "start": 83,
      "end": 126
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_Sgf3MDWzexI",
            "start": 133,
            "end": 146
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
                "value": "./test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI",
                "raw": "\"./test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI\"",
                "start": 160,
                "end": 221
              },
              "options": null,
              "phase": null,
              "start": 153,
              "end": 222
            },
            "id": null,
            "generator": false,
            "start": 149,
            "end": 222
          },
          "start": 133,
          "end": 222
        }
      ],
      "start": 127,
      "end": 223
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_wqR1xEjZjf4",
            "start": 230,
            "end": 243
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
                "value": "./test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4",
                "raw": "\"./test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4\"",
                "start": 257,
                "end": 318
              },
              "options": null,
              "phase": null,
              "start": 250,
              "end": 319
            },
            "id": null,
            "generator": false,
            "start": 246,
            "end": 319
          },
          "start": 230,
          "end": 319
        }
      ],
      "start": 224,
      "end": 320
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
              "start": 334,
              "end": 366
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [],
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
                          "name": "enabled",
                          "start": 385,
                          "end": 392
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 395,
                            "end": 404
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": false,
                              "raw": "false",
                              "start": 405,
                              "end": 410
                            }
                          ],
                          "optional": false,
                          "start": 395,
                          "end": 411
                        },
                        "start": 385,
                        "end": 411
                      }
                    ],
                    "start": 379,
                    "end": 412
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "input",
                          "start": 423,
                          "end": 428
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 431,
                            "end": 440
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "",
                              "raw": "\"\"",
                              "start": 441,
                              "end": 443
                            }
                          ],
                          "optional": false,
                          "start": 431,
                          "end": 444
                        },
                        "start": 423,
                        "end": 444
                      }
                    ],
                    "start": 417,
                    "end": 445
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 471,
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
                                "value": "q-e:input",
                                "raw": "\"q-e:input\"",
                                "start": 501,
                                "end": 512
                              },
                              "value": {
                                "type": "ConditionalExpression",
                                "test": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "enabled",
                                    "start": 514,
                                    "end": 521
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 522,
                                    "end": 527
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 514,
                                  "end": 527
                                },
                                "consequent": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "qrl",
                                    "start": 544,
                                    "end": 547
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "i_wqR1xEjZjf4",
                                      "start": 548,
                                      "end": 561
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
                                      "raw": "\"FieldInput_component_input_q_e_input_wqR1xEjZjf4\"",
                                      "start": 563,
                                      "end": 613
                                    },
                                    {
                                      "type": "ArrayExpression",
                                      "elements": [
                                        {
                                          "type": "Identifier",
                                          "name": "input",
                                          "start": 629,
                                          "end": 634
                                        }
                                      ],
                                      "start": 615,
                                      "end": 644
                                    }
                                  ],
                                  "optional": false,
                                  "start": 544,
                                  "end": 645
                                },
                                "alternate": {
                                  "type": "Identifier",
                                  "name": "undefined",
                                  "start": 648,
                                  "end": 657
                                },
                                "start": 514,
                                "end": 657
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 501,
                              "end": 657
                            }
                          ],
                          "start": 491,
                          "end": 663
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
                                "start": 675,
                                "end": 677
                              },
                              "value": {
                                "type": "Literal",
                                "value": "input",
                                "raw": "\"input\"",
                                "start": 679,
                                "end": 686
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 675,
                              "end": 686
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:focus",
                                "raw": "\"q-e:focus\"",
                                "start": 696,
                                "end": 707
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 723,
                                  "end": 726
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_Sgf3MDWzexI",
                                    "start": 727,
                                    "end": 740
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "FieldInput_component_input_q_e_focus_Sgf3MDWzexI",
                                    "raw": "\"FieldInput_component_input_q_e_focus_Sgf3MDWzexI\"",
                                    "start": 742,
                                    "end": 792
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "name": "enabled",
                                        "start": 808,
                                        "end": 815
                                      }
                                    ],
                                    "start": 794,
                                    "end": 825
                                  }
                                ],
                                "optional": false,
                                "start": 723,
                                "end": 826
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 696,
                              "end": 826
                            }
                          ],
                          "start": 665,
                          "end": 832
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 834,
                          "end": 838
                        },
                        {
                          "type": "Literal",
                          "value": 2,
                          "raw": "2",
                          "start": 840,
                          "end": 841
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 843,
                          "end": 849
                        }
                      ],
                      "optional": false,
                      "start": 471,
                      "end": 850
                    },
                    "start": 450,
                    "end": 851
                  }
                ],
                "start": 373,
                "end": 853
              },
              "id": null,
              "generator": false,
              "start": 369,
              "end": 853
            },
            "start": 334,
            "end": 853
          }
        ],
        "start": 328,
        "end": 854
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 321,
      "end": 854
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 854
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
    372
  ]
}
```

### Module: `test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const FieldInput_component_input_q_e_input_wqR1xEjZjf4 = (ev, el)=>{
    const input = _captures[0];
    input.value = el.value;
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
            "name": "_captures",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_captures",
            "start": 9,
            "end": 18
          },
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
              "name": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
              "start": 57,
              "end": 105
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "ev",
                  "start": 109,
                  "end": 111
                },
                {
                  "type": "Identifier",
                  "name": "el",
                  "start": 113,
                  "end": 115
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
                          "start": 130,
                          "end": 135
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 138,
                            "end": 147
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 148,
                            "end": 149
                          },
                          "optional": false,
                          "computed": true,
                          "start": 138,
                          "end": 150
                        },
                        "start": 130,
                        "end": 150
                      }
                    ],
                    "start": 124,
                    "end": 151
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
                          "name": "input",
                          "start": 156,
                          "end": 161
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 162,
                          "end": 167
                        },
                        "optional": false,
                        "computed": false,
                        "start": 156,
                        "end": 167
                      },
                      "right": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "el",
                          "start": 170,
                          "end": 172
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 173,
                          "end": 178
                        },
                        "optional": false,
                        "computed": false,
                        "start": 170,
                        "end": 178
                      },
                      "start": 156,
                      "end": 178
                    },
                    "start": 156,
                    "end": 179
                  }
                ],
                "start": 118,
                "end": 181
              },
              "id": null,
              "generator": false,
              "start": 108,
              "end": 181
            },
            "start": 57,
            "end": 181
          }
        ],
        "start": 51,
        "end": 182
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 182
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 182
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
  "entry": null,
  "displayName": "test.tsx_FieldInput_component_input_q_e_input",
  "hash": "wqR1xEjZjf4",
  "canonicalFilename": "test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4",
  "path": "",
  "extension": "js",
  "parent": "FieldInput_component_V4XAtJSTRKg",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    245,
    291
  ],
  "paramNames": [
    "ev",
    "el"
  ],
  "captureNames": [
    "input"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (3 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 5 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_captures` | `test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI.js` | `@qwik.dev/core` | 2 |
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |
| `_jsxSorted` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |
| `_captures` | `test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
