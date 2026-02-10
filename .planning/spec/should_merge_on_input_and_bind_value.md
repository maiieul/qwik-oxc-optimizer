# Test: should_merge_on_input_and_bind_value

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { component$, useSignal } from "@qwik.dev/core";

export const FieldInput = component$(() => {
  const localValue = useSignal("");

  return (
    <input
      bind:value={localValue}
      onInput$={() => {
        console.log("test");
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
                                  "value": "",
                                  "raw": "\"\"",
                                  "start": 133,
                                  "end": 135
                                }
                              ],
                              "optional": false,
                              "start": 123,
                              "end": 136
                            },
                            "definite": false,
                            "start": 110,
                            "end": 136
                          }
                        ],
                        "declare": false,
                        "start": 104,
                        "end": 137
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
                                "start": 155,
                                "end": 160
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
                                      "start": 167,
                                      "end": 171
                                    },
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "value",
                                      "start": 172,
                                      "end": 177
                                    },
                                    "start": 167,
                                    "end": 177
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "localValue",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 179,
                                      "end": 189
                                    },
                                    "start": 178,
                                    "end": 190
                                  },
                                  "start": 167,
                                  "end": 190
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onInput$",
                                    "start": 197,
                                    "end": 205
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
                                                  "start": 223,
                                                  "end": 230
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "log",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 231,
                                                  "end": 234
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 223,
                                                "end": 234
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Literal",
                                                  "value": "test",
                                                  "raw": "\"test\"",
                                                  "start": 235,
                                                  "end": 241
                                                }
                                              ],
                                              "optional": false,
                                              "start": 223,
                                              "end": 242
                                            },
                                            "directive": null,
                                            "start": 223,
                                            "end": 243
                                          }
                                        ],
                                        "start": 213,
                                        "end": 251
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 207,
                                      "end": 251
                                    },
                                    "start": 206,
                                    "end": 252
                                  },
                                  "start": 197,
                                  "end": 252
                                }
                              ],
                              "selfClosing": true,
                              "start": 154,
                              "end": 259
                            },
                            "children": [],
                            "closingElement": null,
                            "start": 154,
                            "end": 259
                          },
                          "start": 148,
                          "end": 263
                        },
                        "start": 141,
                        "end": 264
                      }
                    ],
                    "start": 100,
                    "end": 266
                  },
                  "id": null,
                  "generator": false,
                  "start": 94,
                  "end": 266
                }
              ],
              "optional": false,
              "start": 83,
              "end": 267
            },
            "definite": false,
            "start": 70,
            "end": 267
          }
        ],
        "declare": false,
        "start": 64,
        "end": 268
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 57,
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
import { _jsxSorted } from "@qwik.dev/core";
import { _val } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useSignal } from "@qwik.dev/core";
const i_wqR1xEjZjf4 = ()=>import("./test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4");
export const FieldInput_component_V4XAtJSTRKg = ()=>{
    const localValue = useSignal("");
    return /*#__PURE__*/ _jsxSorted("input", null, {
        "value": localValue,
        "q-e:input": [
            inlinedQrl(_val, "_val", [
                localValue
            ]),
            /*#__PURE__*/ qrl(i_wqR1xEjZjf4, "FieldInput_component_input_q_e_input_wqR1xEjZjf4")
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
            "name": "_val",
            "start": 54,
            "end": 58
          },
          "local": {
            "type": "Identifier",
            "name": "_val",
            "start": 54,
            "end": 58
          },
          "start": 54,
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
      "start": 45,
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
                              "value": "",
                              "raw": "\"\"",
                              "start": 395,
                              "end": 397
                            }
                          ],
                          "optional": false,
                          "start": 385,
                          "end": 398
                        },
                        "start": 372,
                        "end": 398
                      }
                    ],
                    "start": 366,
                    "end": 399
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 425,
                        "end": 435
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "input",
                          "raw": "\"input\"",
                          "start": 436,
                          "end": 443
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 445,
                          "end": 449
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "value",
                                "raw": "\"value\"",
                                "start": 461,
                                "end": 468
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "localValue",
                                "start": 470,
                                "end": 480
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 461,
                              "end": 480
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:input",
                                "raw": "\"q-e:input\"",
                                "start": 490,
                                "end": 501
                              },
                              "value": {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "inlinedQrl",
                                      "start": 517,
                                      "end": 527
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "_val",
                                        "start": 528,
                                        "end": 532
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "_val",
                                        "raw": "\"_val\"",
                                        "start": 534,
                                        "end": 540
                                      },
                                      {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Identifier",
                                            "name": "localValue",
                                            "start": 560,
                                            "end": 570
                                          }
                                        ],
                                        "start": 542,
                                        "end": 584
                                      }
                                    ],
                                    "optional": false,
                                    "start": 517,
                                    "end": 585
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "qrl",
                                      "start": 613,
                                      "end": 616
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "i_wqR1xEjZjf4",
                                        "start": 617,
                                        "end": 630
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "FieldInput_component_input_q_e_input_wqR1xEjZjf4",
                                        "raw": "\"FieldInput_component_input_q_e_input_wqR1xEjZjf4\"",
                                        "start": 632,
                                        "end": 682
                                      }
                                    ],
                                    "optional": false,
                                    "start": 613,
                                    "end": 683
                                  }
                                ],
                                "start": 503,
                                "end": 693
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 490,
                              "end": 693
                            }
                          ],
                          "start": 451,
                          "end": 699
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 701,
                          "end": 705
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 707,
                          "end": 708
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 710,
                          "end": 716
                        }
                      ],
                      "optional": false,
                      "start": 425,
                      "end": 717
                    },
                    "start": 404,
                    "end": 718
                  }
                ],
                "start": 360,
                "end": 720
              },
              "id": null,
              "generator": false,
              "start": 356,
              "end": 720
            },
            "start": 321,
            "end": 720
          }
        ],
        "start": 315,
        "end": 721
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 308,
      "end": 721
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 721
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
    268
  ]
}
```

### Module: test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4.js (ENTRY POINT)

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
    209,
    253
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
- **[CONV-12] Input Binding**: Input binding via `_val` handler

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `inlinedQrl` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 2 |
| `_val` | `test.tsx_FieldInput_component_V4XAtJSTRKg.js` | `@qwik.dev/core` | 3 |

## Diagnostics

```json
[]
```
