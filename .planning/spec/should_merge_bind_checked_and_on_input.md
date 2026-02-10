# Test: should_merge_bind_checked_and_on_input

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { component$, useSignal } from "@qwik.dev/core";

export const FieldInput = component$(() => {
  const localValue = useSignal(false);

  return (
    <input
      onInput$={() => {
        console.log("test");
      }}
      bind:checked={localValue}
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
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 30
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useSignal",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 30
          },
          "importKind": "value",
          "start": 21,
          "end": 30
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 38,
        "end": 54
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 55
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
              "start": 70,
              "end": 80
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 83,
                "end": 93
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
                              "name": "localValue",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 110,
                              "end": 120
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 123,
                                "end": 132
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": false,
                                  "raw": "false",
                                  "start": 133,
                                  "end": 138
                                }
                              ],
                              "optional": false,
                              "start": 123,
                              "end": 139
                            },
                            "definite": false,
                            "start": 110,
                            "end": 139
                          }
                        ],
                        "declare": false,
                        "start": 104,
                        "end": 140
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
                                "start": 158,
                                "end": 163
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onInput$",
                                    "start": 170,
                                    "end": 178
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
                                                  "start": 196,
                                                  "end": 203
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "log",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 204,
                                                  "end": 207
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 196,
                                                "end": 207
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Literal",
                                                  "value": "test",
                                                  "raw": "\"test\"",
                                                  "start": 208,
                                                  "end": 214
                                                }
                                              ],
                                              "optional": false,
                                              "start": 196,
                                              "end": 215
                                            },
                                            "directive": null,
                                            "start": 196,
                                            "end": 216
                                          }
                                        ],
                                        "start": 186,
                                        "end": 224
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 180,
                                      "end": 224
                                    },
                                    "start": 179,
                                    "end": 225
                                  },
                                  "start": 170,
                                  "end": 225
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXNamespacedName",
                                    "namespace": {
                                      "type": "JSXIdentifier",
                                      "name": "bind",
                                      "start": 232,
                                      "end": 236
                                    },
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "checked",
                                      "start": 237,
                                      "end": 244
                                    },
                                    "start": 232,
                                    "end": 244
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "localValue",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 246,
                                      "end": 256
                                    },
                                    "start": 245,
                                    "end": 257
                                  },
                                  "start": 232,
                                  "end": 257
                                }
                              ],
                              "selfClosing": true,
                              "start": 157,
                              "end": 264
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 157,
                            "end": 264
                          },
                          "start": 151,
                          "end": 268
                        },
                        "start": 144,
                        "end": 269
                      }
                    ],
                    "start": 100,
                    "end": 271
                  },
                  "id": null,
                  "generator": false,
                  "start": 94,
                  "end": 271
                }
              ],
              "optional": false,
              "start": 83,
              "end": 272
            },
            "definite": false,
            "start": 70,
            "end": 272
          }
        ],
        "declare": false,
        "start": 64,
        "end": 273
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 57,
      "end": 273
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 273
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
import { _chk } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_wqR1xEjZjf4 = ()=>import("./test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4");
export const FieldInput_component_V4XAtJSTRKg = ()=>{
    const localValue = useSignal(false);
    return /*#__PURE__*/ _jsxSorted("input", null, {
        "checked": localValue,
        "q-e:input": [
            /*#__PURE__*/ qrl(i_wqR1xEjZjf4, "FieldInput_component_input_q_e_input_wqR1xEjZjf4"),
            inlinedQrl(_chk, "_chk", [
                localValue
            ])
        ]
    }, null, 3, "u6_0");
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
            "name": "_chk",
            "start": 9,
            "end": 13
          },
          "local": {
            "type": "Identifier",
            "name": "_chk",
            "start": 9,
            "end": 13
          },
          "start": 9,
          "end": 13
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 21,
        "end": 37
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 38
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 48,
            "end": 58
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 48,
            "end": 58
          },
          "start": 48,
          "end": 58
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 66,
        "end": 82
      },
      "phase": null,
      "attributes": [],
      "start": 39,
      "end": 83
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 93,
            "end": 103
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 93,
            "end": 103
          },
          "start": 93,
          "end": 103
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 111,
        "end": 127
      },
      "phase": null,
      "attributes": [],
      "start": 84,
      "end": 128
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 138,
            "end": 141
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 138,
            "end": 141
          },
          "start": 138,
          "end": 141
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 149,
        "end": 165
      },
      "phase": null,
      "attributes": [],
      "start": 129,
      "end": 166
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 176,
            "end": 185
          },
          "local": {
            "type": "Identifier",
            "name": "useSignal",
            "start": 176,
            "end": 185
          },
          "start": 176,
          "end": 185
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 193,
        "end": 209
      },
      "phase": null,
      "attributes": [],
      "start": 167,
      "end": 210
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
            "start": 217,
            "end": 230
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
                "start": 244,
                "end": 305
              },
              "options": null,
              "phase": null,
              "start": 237,
              "end": 306
            },
            "id": null,
            "generator": false,
            "start": 233,
            "end": 306
          },
          "start": 217,
          "end": 306
        }
      ],
      "start": 211,
      "end": 307
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
              "start": 321,
              "end": 353
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
                          "name": "localValue",
                          "start": 372,
                          "end": 382
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 385,
                            "end": 394
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": false,
                              "raw": "false",
                              "start": 395,
                              "end": 400
                            }
                          ],
                          "optional": false,
                          "start": 385,
                          "end": 401
                        },
                        "start": 372,
                        "end": 401
                      }
                    ],
                    "start": 366,
                    "end": 402
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 428,
                        "end": 438
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "input",
                          "raw": "\"input\"",
                          "start": 439,
                          "end": 446
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 448,
                          "end": 452
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "checked",
                                "raw": "\"checked\"",
                                "start": 464,
                                "end": 473
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "localValue",
                                "start": 475,
                                "end": 485
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 464,
                              "end": 485
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:input",
                                "raw": "\"q-e:input\"",
                                "start": 495,
                                "end": 506
                              },
                              "value": {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "qrl",
                                      "start": 536,
                                      "end": 539
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "i_wqR1xEjZjf4",
                                        "start": 540,
                                        "end": 553
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
                                        "raw": "\"FieldInput_component_input_q_e_input_wqR1xEjZjf4\"",
                                        "start": 555,
                                        "end": 605
                                      }
                                    ],
                                    "optional": false,
                                    "start": 536,
                                    "end": 606
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "inlinedQrl",
                                      "start": 620,
                                      "end": 630
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "_chk",
                                        "start": 631,
                                        "end": 635
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "_chk",
                                        "raw": "\"_chk\"",
                                        "start": 637,
                                        "end": 643
                                      },
                                      {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Identifier",
                                            "name": "localValue",
                                            "start": 663,
                                            "end": 673
                                          }
                                        ],
                                        "start": 645,
                                        "end": 687
                                      }
                                    ],
                                    "optional": false,
                                    "start": 620,
                                    "end": 688
                                  }
                                ],
                                "start": 508,
                                "end": 698
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 495,
                              "end": 698
                            }
                          ],
                          "start": 454,
                          "end": 704
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 706,
                          "end": 710
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 712,
                          "end": 713
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 715,
                          "end": 721
                        }
                      ],
                      "optional": false,
                      "start": 428,
                      "end": 722
                    },
                    "start": 407,
                    "end": 723
                  }
                ],
                "start": 360,
                "end": 725
              },
              "id": null,
              "generator": false,
              "start": 356,
              "end": 725
            },
            "start": 321,
            "end": 725
          }
        ],
        "start": 315,
        "end": 726
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 308,
      "end": 726
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 726
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
    96,
    273
  ]
}
```

### Module: `test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4.js` (ENTRY POINT)

```javascript
export const FieldInput_component_input_q_e_input_wqR1xEjZjf4 = ()=>{
    console.log("test");
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
              "name": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
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
                          "value": "test",
                          "raw": "\"test\"",
                          "start": 86,
                          "end": 92
                        }
                      ],
                      "optional": false,
                      "start": 74,
                      "end": 93
                    },
                    "start": 74,
                    "end": 94
                  }
                ],
                "start": 68,
                "end": 96
              },
              "id": null,
              "generator": false,
              "start": 64,
              "end": 96
            },
            "start": 13,
            "end": 96
          }
        ],
        "start": 7,
        "end": 97
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 97
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 97
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
  "ctxKind": "eventHandler",
  "ctxName": "onInput$",
  "captures": false,
  "loc": [
    182,
    226
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls, `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (2 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 4 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point module(s)
- **[CONV-12] Input Binding**: Input binding via `_chk` handler

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `inlinedQrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_chk` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |

## Diagnostics

```json
[]
```
