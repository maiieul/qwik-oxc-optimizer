# Test: should_move_bind_value_to_var_props

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, useSignal } from "@qwik.dev/core";
import { destructureBindings } from "./destructure-bindings";

export const FieldInput = component$(
  (props) => {
    const initialValues = { value: undefined as string | undefined };
    const rest = destructureBindings(props, initialValues);

    return (
      <input
        {...rest}
        bind:value={finalValue}
		onClick$={() => {
			console.log('clicked');
		}}
      />
    );
  }
);
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "destructureBindings",
            "optional": false,
            "typeAnnotation": null,
            "start": 68,
            "end": 87
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "destructureBindings",
            "optional": false,
            "typeAnnotation": null,
            "start": 68,
            "end": 87
          },
          "importKind": "value",
          "start": 68,
          "end": 87
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./destructure-bindings",
        "raw": "\"./destructure-bindings\"",
        "start": 95,
        "end": 119
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 59,
      "end": 120
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
              "start": 135,
              "end": 145
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 148,
                "end": 158
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
                      "start": 163,
                      "end": 168
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
                              "name": "initialValues",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 185,
                              "end": 198
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
                                    "name": "value",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 203,
                                    "end": 208
                                  },
                                  "value": {
                                    "type": "TSAsExpression",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "undefined",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 210,
                                      "end": 219
                                    },
                                    "typeAnnotation": {
                                      "type": "TSUnionType",
                                      "types": [
                                        {
                                          "type": "TSStringKeyword",
                                          "start": 223,
                                          "end": 229
                                        },
                                        {
                                          "type": "TSUndefinedKeyword",
                                          "start": 232,
                                          "end": 241
                                        }
                                      ],
                                      "start": 223,
                                      "end": 241
                                    },
                                    "start": 210,
                                    "end": 241
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "optional": false,
                                  "start": 203,
                                  "end": 241
                                }
                              ],
                              "start": 201,
                              "end": 243
                            },
                            "definite": false,
                            "start": 185,
                            "end": 243
                          }
                        ],
                        "declare": false,
                        "start": 179,
                        "end": 244
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
                              "name": "rest",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 255,
                              "end": 259
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "destructureBindings",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 262,
                                "end": 281
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "props",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 282,
                                  "end": 287
                                },
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "initialValues",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 289,
                                  "end": 302
                                }
                              ],
                              "optional": false,
                              "start": 262,
                              "end": 303
                            },
                            "definite": false,
                            "start": 255,
                            "end": 303
                          }
                        ],
                        "declare": false,
                        "start": 249,
                        "end": 304
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
                                "start": 326,
                                "end": 331
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXSpreadAttribute",
                                  "argument": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "rest",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 344,
                                    "end": 348
                                  },
                                  "start": 340,
                                  "end": 349
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXNamespacedName",
                                    "namespace": {
                                      "type": "JSXIdentifier",
                                      "name": "bind",
                                      "start": 358,
                                      "end": 362
                                    },
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "value",
                                      "start": 363,
                                      "end": 368
                                    },
                                    "start": 358,
                                    "end": 368
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "finalValue",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 370,
                                      "end": 380
                                    },
                                    "start": 369,
                                    "end": 381
                                  },
                                  "start": 358,
                                  "end": 381
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 384,
                                    "end": 392
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
                                              "type": "CallExpression",
                                              "callee": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "console",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 405,
                                                  "end": 412
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "log",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 413,
                                                  "end": 416
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 405,
                                                "end": 416
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Literal",
                                                  "value": "clicked",
                                                  "raw": "'clicked'",
                                                  "start": 417,
                                                  "end": 426
                                                }
                                              ],
                                              "optional": false,
                                              "start": 405,
                                              "end": 427
                                            },
                                            "directive": null,
                                            "start": 405,
                                            "end": 428
                                          }
                                        ],
                                        "start": 400,
                                        "end": 432
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 394,
                                      "end": 432
                                    },
                                    "start": 393,
                                    "end": 433
                                  },
                                  "start": 384,
                                  "end": 433
                                }
                              ],
                              "selfClosing": true,
                              "start": 325,
                              "end": 442
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 325,
                            "end": 442
                          },
                          "start": 317,
                          "end": 448
                        },
                        "start": 310,
                        "end": 449
                      }
                    ],
                    "start": 173,
                    "end": 453
                  },
                  "id": null,
                  "generator": false,
                  "start": 162,
                  "end": 453
                }
              ],
              "optional": false,
              "start": 148,
              "end": 455
            },
            "definite": false,
            "start": 135,
            "end": 455
          }
        ],
        "declare": false,
        "start": 129,
        "end": 456
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 122,
      "end": 456
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 456
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
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { destructureBindings } from "./destructure-bindings";
import { qrl } from "@qwik.dev/core";
const i_fZgZHYZXt60 = ()=>import("./test.tsx_FieldInput_component_input_q_e_click_fZgZHYZXt60");
export const FieldInput_component_V4XAtJSTRKg = (props)=>{
    const initialValues = {
        value: undefined
    };
    const rest = destructureBindings(props, initialValues);
    return /*#__PURE__*/ _jsxSplit("input", {
        ..._getVarProps(rest),
        ..._getConstProps(rest),
        "bind:value": finalValue
    }, {
        "q-e:click": /*#__PURE__*/ qrl(i_fZgZHYZXt60, "FieldInput_component_input_q_e_click_fZgZHYZXt60")
    }, null, 0, "u6_0");
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
            "name": "_getConstProps",
            "start": 9,
            "end": 23
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 9,
            "end": 23
          },
          "start": 9,
          "end": 23
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 31,
        "end": 47
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 48
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 58,
            "end": 70
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 58,
            "end": 70
          },
          "start": 58,
          "end": 70
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 78,
        "end": 94
      },
      "phase": null,
      "attributes": [],
      "start": 49,
      "end": 95
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 105,
            "end": 114
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 105,
            "end": 114
          },
          "start": 105,
          "end": 114
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 122,
        "end": 138
      },
      "phase": null,
      "attributes": [],
      "start": 96,
      "end": 139
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "destructureBindings",
            "start": 149,
            "end": 168
          },
          "local": {
            "type": "Identifier",
            "name": "destructureBindings",
            "start": 149,
            "end": 168
          },
          "start": 149,
          "end": 168
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./destructure-bindings",
        "raw": "\"./destructure-bindings\"",
        "start": 176,
        "end": 200
      },
      "phase": null,
      "attributes": [],
      "start": 140,
      "end": 201
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 211,
            "end": 214
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 211,
            "end": 214
          },
          "start": 211,
          "end": 214
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 222,
        "end": 238
      },
      "phase": null,
      "attributes": [],
      "start": 202,
      "end": 239
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_fZgZHYZXt60",
            "start": 246,
            "end": 259
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
                "value": "./test.tsx_FieldInput_component_input_q_e_click_fZgZHYZXt60",
                "raw": "\"./test.tsx_FieldInput_component_input_q_e_click_fZgZHYZXt60\"",
                "start": 273,
                "end": 334
              },
              "options": null,
              "phase": null,
              "start": 266,
              "end": 335
            },
            "id": null,
            "generator": false,
            "start": 262,
            "end": 335
          },
          "start": 246,
          "end": 335
        }
      ],
      "start": 240,
      "end": 336
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
              "start": 350,
              "end": 382
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 386,
                  "end": 391
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
                          "name": "initialValues",
                          "start": 406,
                          "end": 419
                        },
                        "init": {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 432,
                                "end": 437
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "undefined",
                                "start": 439,
                                "end": 448
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 432,
                              "end": 448
                            }
                          ],
                          "start": 422,
                          "end": 454
                        },
                        "start": 406,
                        "end": 454
                      }
                    ],
                    "start": 400,
                    "end": 455
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "rest",
                          "start": 466,
                          "end": 470
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "destructureBindings",
                            "start": 473,
                            "end": 492
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "props",
                              "start": 493,
                              "end": 498
                            },
                            {
                              "type": "Identifier",
                              "name": "initialValues",
                              "start": 500,
                              "end": 513
                            }
                          ],
                          "optional": false,
                          "start": 473,
                          "end": 514
                        },
                        "start": 466,
                        "end": 514
                      }
                    ],
                    "start": 460,
                    "end": 515
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSplit",
                        "start": 541,
                        "end": 550
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "input",
                          "raw": "\"input\"",
                          "start": 551,
                          "end": 558
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
                                  "start": 573,
                                  "end": 585
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "rest",
                                    "start": 586,
                                    "end": 590
                                  }
                                ],
                                "optional": false,
                                "start": 573,
                                "end": 591
                              },
                              "start": 570,
                              "end": 591
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_getConstProps",
                                  "start": 604,
                                  "end": 618
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "rest",
                                    "start": 619,
                                    "end": 623
                                  }
                                ],
                                "optional": false,
                                "start": 604,
                                "end": 624
                              },
                              "start": 601,
                              "end": 624
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "bind:value",
                                "raw": "\"bind:value\"",
                                "start": 634,
                                "end": 646
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "finalValue",
                                "start": 648,
                                "end": 658
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 634,
                              "end": 658
                            }
                          ],
                          "start": 560,
                          "end": 664
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
                                "start": 676,
                                "end": 687
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 703,
                                  "end": 706
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_fZgZHYZXt60",
                                    "start": 707,
                                    "end": 720
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "FieldInput_component_input_q_e_click_fZgZHYZXt60",
                                    "raw": "\"FieldInput_component_input_q_e_click_fZgZHYZXt60\"",
                                    "start": 722,
                                    "end": 772
                                  }
                                ],
                                "optional": false,
                                "start": 703,
                                "end": 773
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 676,
                              "end": 773
                            }
                          ],
                          "start": 666,
                          "end": 779
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 781,
                          "end": 785
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 787,
                          "end": 788
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 790,
                          "end": 796
                        }
                      ],
                      "optional": false,
                      "start": 541,
                      "end": 797
                    },
                    "start": 520,
                    "end": 798
                  }
                ],
                "start": 394,
                "end": 800
              },
              "id": null,
              "generator": false,
              "start": 385,
              "end": 800
            },
            "start": 350,
            "end": 800
          }
        ],
        "start": 344,
        "end": 801
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 337,
      "end": 801
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 801
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
    164,
    455
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: `test.tsx_FieldInput_component_input_q_e_click_fZgZHYZXt60.js` (ENTRY POINT)

```javascript
export const FieldInput_component_input_q_e_click_fZgZHYZXt60 = ()=>{
    console.log('clicked');
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
              "name": "FieldInput_component_input_q_e_click_fZgZHYZXt60",
              "start": 13,
              "end": 61
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
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "console",
                          "start": 74,
                          "end": 81
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 82,
                          "end": 85
                        },
                        "optional": false,
                        "computed": false,
                        "start": 74,
                        "end": 85
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "clicked",
                          "raw": "'clicked'",
                          "start": 86,
                          "end": 95
                        }
                      ],
                      "optional": false,
                      "start": 74,
                      "end": 96
                    },
                    "start": 74,
                    "end": 97
                  }
                ],
                "start": 68,
                "end": 99
              },
              "id": null,
              "generator": false,
              "start": 64,
              "end": 99
            },
            "start": 13,
            "end": 99
          }
        ],
        "start": 7,
        "end": 100
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 100
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 100
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "FieldInput_component_input_q_e_click_fZgZHYZXt60",
  "entry": null,
  "displayName": "test.tsx_FieldInput_component_input_q_e_click",
  "hash": "fZgZHYZXt60",
  "canonicalFilename": "test.tsx_FieldInput_component_input_q_e_click_fZgZHYZXt60",
  "path": "",
  "extension": "js",
  "parent": "FieldInput_component_V4XAtJSTRKg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    396,
    434
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSplit()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_getVarProps()`, `_getConstProps()`
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (2 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 4 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point module(s)
- **[CONV-12] Input Binding**: Input binding via `bind:` directives

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_jsxSplit` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_getVarProps` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_getConstProps` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
