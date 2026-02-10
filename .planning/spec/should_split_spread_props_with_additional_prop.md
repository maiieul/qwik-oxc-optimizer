# Test: should_split_spread_props_with_additional_prop

## Test Configuration

**Note:** Spread props variant 1: `{...props}` followed by static prop `test="test"`. Tests _jsxSplit with _getVarProps/_getConstProps splitting.

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

		export default component$((props) => {
			return (
				<div {...props} test="test"></div>
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
                    "type": "ParenthesizedExpression",
                    "expression": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 104,
                          "end": 107
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
                              "start": 112,
                              "end": 117
                            },
                            "start": 108,
                            "end": 118
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "test",
                              "start": 119,
                              "end": 123
                            },
                            "value": {
                              "type": "Literal",
                              "value": "test",
                              "raw": "\"test\"",
                              "start": 124,
                              "end": 130
                            },
                            "start": 119,
                            "end": 130
                          }
                        ],
                        "selfClosing": false,
                        "start": 103,
                        "end": 131
                      },
                      "children": [],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 133,
                          "end": 136
                        },
                        "start": 131,
                        "end": 137
                      },
                      "start": 103,
                      "end": 137
                    },
                    "start": 97,
                    "end": 142
                  },
                  "start": 90,
                  "end": 143
                }
              ],
              "start": 85,
              "end": 147
            },
            "id": null,
            "generator": false,
            "start": 74,
            "end": 147
          }
        ],
        "optional": false,
        "start": 63,
        "end": 148
      },
      "exportKind": "value",
      "start": 48,
      "end": 149
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 149
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
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit("div", {
        ..._getVarProps(props)
    }, {
        ..._getConstProps(props),
        test: "test"
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
              "start": 153,
              "end": 179
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 183,
                  "end": 188
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
                        "start": 218,
                        "end": 227
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 228,
                          "end": 233
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
                                  "start": 248,
                                  "end": 260
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 261,
                                    "end": 266
                                  }
                                ],
                                "optional": false,
                                "start": 248,
                                "end": 267
                              },
                              "start": 245,
                              "end": 267
                            }
                          ],
                          "start": 235,
                          "end": 273
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
                                  "name": "_getConstProps",
                                  "start": 288,
                                  "end": 302
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 303,
                                    "end": 308
                                  }
                                ],
                                "optional": false,
                                "start": 288,
                                "end": 309
                              },
                              "start": 285,
                              "end": 309
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "test",
                                "start": 319,
                                "end": 323
                              },
                              "value": {
                                "type": "Literal",
                                "value": "test",
                                "raw": "\"test\"",
                                "start": 325,
                                "end": 331
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 319,
                              "end": 331
                            }
                          ],
                          "start": 275,
                          "end": 337
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 339,
                          "end": 343
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 345,
                          "end": 346
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 348,
                          "end": 354
                        }
                      ],
                      "optional": false,
                      "start": 218,
                      "end": 355
                    },
                    "start": 197,
                    "end": 356
                  }
                ],
                "start": 191,
                "end": 358
              },
              "id": null,
              "generator": false,
              "start": 182,
              "end": 358
            },
            "start": 153,
            "end": 358
          }
        ],
        "start": 147,
        "end": 359
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 140,
      "end": 359
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 359
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
    151
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
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| _jsxSplit | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _getVarProps | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |
| _getConstProps | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
