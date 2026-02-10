# Test: should_split_spread_props_with_additional_prop4

## Test Configuration

**Note:** Spread props variant 4: Spread followed by onClick$ event handler that references spread props. Tests interaction of spread + event handler extraction.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

		export default component$((props: any) => {
			return <button {...props} onClick$={() => props.onClick$()}></button>;
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
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 27,
        "end": 43
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 44
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "decorators": [],
          "name": "component$",
          "optional": false,
          "typeAnnotation": null,
          "start": 63,
          "end": 73
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
                "typeAnnotation": {
                  "type": "TSTypeAnnotation",
                  "typeAnnotation": {
                    "type": "TSAnyKeyword",
                    "start": 82,
                    "end": 85
                  },
                  "start": 80,
                  "end": 85
                },
                "start": 75,
                "end": 85
              }
            ],
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
                        "name": "button",
                        "start": 103,
                        "end": 109
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
                            "start": 114,
                            "end": 119
                          },
                          "start": 110,
                          "end": 120
                        },
                        {
                          "type": "JSXAttribute",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "onClick$",
                            "start": 121,
                            "end": 129
                          },
                          "value": {
                            "type": "JSXExpressionContainer",
                            "expression": {
                              "type": "ArrowFunctionExpression",
                              "expression": true,
                              "async": false,
                              "typeParameters": null,
                              "params": [],
                              "returnType": null,
                              "body": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "props",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 137,
                                    "end": 142
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "onClick$",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 143,
                                    "end": 151
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 137,
                                  "end": 151
                                },
                                "typeArguments": null,
                                "arguments": [],
                                "optional": false,
                                "start": 137,
                                "end": 153
                              },
                              "id": null,
                              "generator": false,
                              "start": 131,
                              "end": 153
                            },
                            "start": 130,
                            "end": 154
                          },
                          "start": 121,
                          "end": 154
                        }
                      ],
                      "selfClosing": false,
                      "start": 102,
                      "end": 155
                    },
                    "children": [],
                    "closingElement": {
                      "type": "JSXClosingElement",
                      "name": {
                        "type": "JSXIdentifier",
                        "name": "button",
                        "start": 157,
                        "end": 163
                      },
                      "start": 155,
                      "end": 164
                    },
                    "start": 102,
                    "end": 164
                  },
                  "start": 95,
                  "end": 165
                }
              ],
              "start": 90,
              "end": 169
            },
            "id": null,
            "generator": false,
            "start": 74,
            "end": 169
          }
        ],
        "optional": false,
        "start": 63,
        "end": 170
      },
      "exportKind": "value",
      "start": 48,
      "end": 171
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 171
}

```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_LUXeXe0DQrg = ()=>import("./test.tsx_test_component_LUXeXe0DQrg");
export default /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_LUXeXe0DQrg, "test_component_LUXeXe0DQrg"));
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
            "name": "i_LUXeXe0DQrg",
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
                "value": "./test.tsx_test_component_LUXeXe0DQrg",
                "raw": "\"./test.tsx_test_component_LUXeXe0DQrg\"",
                "start": 118,
                "end": 157
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 158
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 158
          },
          "start": 91,
          "end": 158
        }
      ],
      "start": 85,
      "end": 159
    },
    {
      "type": "ExportDefaultDeclaration",
      "declaration": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 189,
          "end": 201
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 216,
              "end": 219
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_LUXeXe0DQrg",
                "start": 220,
                "end": 233
              },
              {
                "type": "Literal",
                "value": "test_component_LUXeXe0DQrg",
                "raw": "\"test_component_LUXeXe0DQrg\"",
                "start": 235,
                "end": 263
              }
            ],
            "optional": false,
            "start": 216,
            "end": 264
          }
        ],
        "optional": false,
        "start": 189,
        "end": 265
      },
      "start": 160,
      "end": 266
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 266
}

```

</details>

### Module: test.tsx_test_component_button_q_e_click_qwSL5gM03T4.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const test_component_button_q_e_click_qwSL5gM03T4 = ()=>{
    const props = _captures[0];
    return props.onClick$();
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
              "name": "test_component_button_q_e_click_qwSL5gM03T4",
              "start": 57,
              "end": 100
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
                          "name": "props",
                          "start": 119,
                          "end": 124
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 127,
                            "end": 136
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 137,
                            "end": 138
                          },
                          "optional": false,
                          "computed": true,
                          "start": 127,
                          "end": 139
                        },
                        "start": 119,
                        "end": 139
                      }
                    ],
                    "start": 113,
                    "end": 140
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "props",
                          "start": 152,
                          "end": 157
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "onClick$",
                          "start": 158,
                          "end": 166
                        },
                        "optional": false,
                        "computed": false,
                        "start": 152,
                        "end": 166
                      },
                      "arguments": [],
                      "optional": false,
                      "start": 152,
                      "end": 168
                    },
                    "start": 145,
                    "end": 169
                  }
                ],
                "start": 107,
                "end": 171
              },
              "id": null,
              "generator": false,
              "start": 103,
              "end": 171
            },
            "start": 57,
            "end": 171
          }
        ],
        "start": 51,
        "end": 172
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
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

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_button_q_e_click_qwSL5gM03T4",
  "entry": null,
  "displayName": "test.tsx_test_component_button_q_e_click",
  "hash": "qwSL5gM03T4",
  "canonicalFilename": "test.tsx_test_component_button_q_e_click_qwSL5gM03T4",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    135,
    157
  ],
  "captureNames": [
    "props"
  ]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_qwSL5gM03T4 = ()=>import("./test.tsx_test_component_button_q_e_click_qwSL5gM03T4");
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit("button", {
        ..._getVarProps(props),
        "q-e:click": /*#__PURE__*/ qrl(i_qwSL5gM03T4, "test_component_button_q_e_click_qwSL5gM03T4", [
            props
        ])
    }, _getConstProps(props), null, 0, "u6_0");
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
            "name": "qrl",
            "start": 149,
            "end": 152
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 149,
            "end": 152
          },
          "start": 149,
          "end": 152
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 160,
        "end": 176
      },
      "phase": null,
      "attributes": [],
      "start": 140,
      "end": 177
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_qwSL5gM03T4",
            "start": 184,
            "end": 197
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
                "value": "./test.tsx_test_component_button_q_e_click_qwSL5gM03T4",
                "raw": "\"./test.tsx_test_component_button_q_e_click_qwSL5gM03T4\"",
                "start": 211,
                "end": 267
              },
              "options": null,
              "phase": null,
              "start": 204,
              "end": 268
            },
            "id": null,
            "generator": false,
            "start": 200,
            "end": 268
          },
          "start": 184,
          "end": 268
        }
      ],
      "start": 178,
      "end": 269
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
              "name": "test_component_LUXeXe0DQrg",
              "start": 283,
              "end": 309
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 313,
                  "end": 318
                }
              ],
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSplit",
                        "start": 348,
                        "end": 357
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "button",
                          "raw": "\"button\"",
                          "start": 358,
                          "end": 366
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
                                  "start": 381,
                                  "end": 393
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 394,
                                    "end": 399
                                  }
                                ],
                                "optional": false,
                                "start": 381,
                                "end": 400
                              },
                              "start": 378,
                              "end": 400
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:click",
                                "raw": "\"q-e:click\"",
                                "start": 410,
                                "end": 421
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 437,
                                  "end": 440
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_qwSL5gM03T4",
                                    "start": 441,
                                    "end": 454
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "test_component_button_q_e_click_qwSL5gM03T4",
                                    "raw": "\"test_component_button_q_e_click_qwSL5gM03T4\"",
                                    "start": 456,
                                    "end": 501
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "name": "props",
                                        "start": 517,
                                        "end": 522
                                      }
                                    ],
                                    "start": 503,
                                    "end": 532
                                  }
                                ],
                                "optional": false,
                                "start": 437,
                                "end": 533
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 410,
                              "end": 533
                            }
                          ],
                          "start": 368,
                          "end": 539
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_getConstProps",
                            "start": 541,
                            "end": 555
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "props",
                              "start": 556,
                              "end": 561
                            }
                          ],
                          "optional": false,
                          "start": 541,
                          "end": 562
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 564,
                          "end": 568
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 570,
                          "end": 571
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 573,
                          "end": 579
                        }
                      ],
                      "optional": false,
                      "start": 348,
                      "end": 580
                    },
                    "start": 327,
                    "end": 581
                  }
                ],
                "start": 321,
                "end": 583
              },
              "id": null,
              "generator": false,
              "start": 312,
              "end": 583
            },
            "start": 283,
            "end": 583
          }
        ],
        "start": 277,
        "end": 584
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 270,
      "end": 584
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 584
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_LUXeXe0DQrg",
  "entry": null,
  "displayName": "test.tsx_test_component",
  "hash": "LUXeXe0DQrg",
  "canonicalFilename": "test.tsx_test_component_LUXeXe0DQrg",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    78,
    173
  ],
  "paramNames": [
    "props"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-04] Signal Helpers**
- **[CONV-05] Capture Patterns**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _jsxSplit | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _getVarProps | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _getConstProps | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
