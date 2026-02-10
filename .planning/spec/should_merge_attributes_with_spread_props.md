# Test: should_merge_attributes_with_spread_props

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

		export default component$((props) => {
			return <div {...props} class={[props.class, 'component']} />;
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
                "typeAnnotation": null,
                "start": 75,
                "end": 80
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
                        "name": "div",
                        "start": 98,
                        "end": 101
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
                            "start": 106,
                            "end": 111
                          },
                          "start": 102,
                          "end": 112
                        },
                        {
                          "type": "JSXAttribute",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "class",
                            "start": 113,
                            "end": 118
                          },
                          "value": {
                            "type": "JSXExpressionContainer",
                            "expression": {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "props",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 121,
                                    "end": 126
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "class",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 127,
                                    "end": 132
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 121,
                                  "end": 132
                                },
                                {
                                  "type": "Literal",
                                  "value": "component",
                                  "raw": "'component'",
                                  "start": 134,
                                  "end": 145
                                }
                              ],
                              "start": 120,
                              "end": 146
                            },
                            "start": 119,
                            "end": 147
                          },
                          "start": 113,
                          "end": 147
                        }
                      ],
                      "selfClosing": true,
                      "start": 97,
                      "end": 150
                    },
                    "children": [],
                    "closingElement": null,
                    "start": 97,
                    "end": 150
                  },
                  "start": 90,
                  "end": 151
                }
              ],
              "start": 85,
              "end": 155
            },
            "id": null,
            "generator": false,
            "start": 74,
            "end": 155
          }
        ],
        "optional": false,
        "start": 63,
        "end": 156
      },
      "exportKind": "value",
      "start": 48,
      "end": 157
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 157
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

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _fnSignal } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
const _hf0 = (p0)=>[
        p0.class,
        'component'
    ];
const _hf0_str = '[p0.class,"component"]';
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit("div", {
        ..._getVarProps(props),
        ..._getConstProps(props),
        class: _fnSignal(_hf0, [
            props
        ], _hf0_str)
    }, null, null, 0, "u6_0");
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
            "name": "_fnSignal",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_fnSignal",
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 53,
            "end": 67
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 53,
            "end": 67
          },
          "start": 53,
          "end": 67
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 75,
        "end": 91
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 92
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 102,
            "end": 114
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 102,
            "end": 114
          },
          "start": 102,
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
      "start": 93,
      "end": 139
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 149,
            "end": 158
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 149,
            "end": 158
          },
          "start": 149,
          "end": 158
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 166,
        "end": 182
      },
      "phase": null,
      "attributes": [],
      "start": 140,
      "end": 183
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf0",
            "start": 190,
            "end": 194
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "p0",
                "start": 198,
                "end": 200
              }
            ],
            "body": {
              "type": "ArrayExpression",
              "elements": [
                {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "p0",
                    "start": 213,
                    "end": 215
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "class",
                    "start": 216,
                    "end": 221
                  },
                  "optional": false,
                  "computed": false,
                  "start": 213,
                  "end": 221
                },
                {
                  "type": "Literal",
                  "value": "component",
                  "raw": "'component'",
                  "start": 231,
                  "end": 242
                }
              ],
              "start": 203,
              "end": 248
            },
            "id": null,
            "generator": false,
            "start": 197,
            "end": 248
          },
          "start": 190,
          "end": 248
        }
      ],
      "start": 184,
      "end": 249
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "_hf0_str",
            "start": 256,
            "end": 264
          },
          "init": {
            "type": "Literal",
            "value": "[p0.class,\"component\"]",
            "raw": "'[p0.class,\"component\"]'",
            "start": 267,
            "end": 291
          },
          "start": 256,
          "end": 291
        }
      ],
      "start": 250,
      "end": 292
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
              "start": 306,
              "end": 332
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 336,
                  "end": 341
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
                        "start": 371,
                        "end": 380
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 381,
                          "end": 386
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
                                  "start": 401,
                                  "end": 413
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 414,
                                    "end": 419
                                  }
                                ],
                                "optional": false,
                                "start": 401,
                                "end": 420
                              },
                              "start": 398,
                              "end": 420
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_getConstProps",
                                  "start": 433,
                                  "end": 447
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 448,
                                    "end": 453
                                  }
                                ],
                                "optional": false,
                                "start": 433,
                                "end": 454
                              },
                              "start": 430,
                              "end": 454
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "class",
                                "start": 464,
                                "end": 469
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_fnSignal",
                                  "start": 471,
                                  "end": 480
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "_hf0",
                                    "start": 481,
                                    "end": 485
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "name": "props",
                                        "start": 501,
                                        "end": 506
                                      }
                                    ],
                                    "start": 487,
                                    "end": 516
                                  },
                                  {
                                    "type": "Identifier",
                                    "name": "_hf0_str",
                                    "start": 518,
                                    "end": 526
                                  }
                                ],
                                "optional": false,
                                "start": 471,
                                "end": 527
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 464,
                              "end": 527
                            }
                          ],
                          "start": 388,
                          "end": 533
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 535,
                          "end": 539
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 541,
                          "end": 545
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 547,
                          "end": 548
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 550,
                          "end": 556
                        }
                      ],
                      "optional": false,
                      "start": 371,
                      "end": 557
                    },
                    "start": 350,
                    "end": 558
                  }
                ],
                "start": 344,
                "end": 560
              },
              "id": null,
              "generator": false,
              "start": 335,
              "end": 560
            },
            "start": 306,
            "end": 560
          }
        ],
        "start": 300,
        "end": 561
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 293,
      "end": 561
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 561
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
    159
  ],
  "paramNames": [
    "props"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSplit()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_fnSignal()`, `_getVarProps()`, `_getConstProps()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 3 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)
- **[CONV-14] Hoisted Functions**: 1 hoisted function(s) (`_hfN`/`_hfN_str` pairs) for signal-derived expressions

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSplit` | `test.tsx_test_component_LUXeXe0DQrg.js` | `@qwik.dev/core` | 2 |
| `_fnSignal` | `test.tsx_test_component_LUXeXe0DQrg.js` | `@qwik.dev/core` | 2 |
| `_getVarProps` | `test.tsx_test_component_LUXeXe0DQrg.js` | `@qwik.dev/core` | 2 |
| `_getConstProps` | `test.tsx_test_component_LUXeXe0DQrg.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
