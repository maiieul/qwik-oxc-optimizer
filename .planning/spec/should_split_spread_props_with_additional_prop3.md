# Test: should_split_spread_props_with_additional_prop3

## Test Configuration

**Note:** Spread props variant 3: Multiple spreads on a component `<Foo>` with `Math.random()`, `{...props}`, boolean prop `hello`, and `{...globalThis.nothing}`.

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
		import { Foo } from './foo';

		export default component$((props) => {
			return (
				<Foo s={Math.random()} {...props} hello {...globalThis.nothing} />
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "Foo",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Foo",
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
        "value": "./foo",
        "raw": "'./foo'",
        "start": 67,
        "end": 74
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 47,
      "end": 75
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
          "start": 94,
          "end": 104
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
                "start": 106,
                "end": 111
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
                          "name": "Foo",
                          "start": 135,
                          "end": 138
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "s",
                              "start": 139,
                              "end": 140
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "Math",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 142,
                                    "end": 146
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "random",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 147,
                                    "end": 153
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 142,
                                  "end": 153
                                },
                                "typeArguments": null,
                                "arguments": [],
                                "optional": false,
                                "start": 142,
                                "end": 155
                              },
                              "start": 141,
                              "end": 156
                            },
                            "start": 139,
                            "end": 156
                          },
                          {
                            "type": "JSXSpreadAttribute",
                            "argument": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "props",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 161,
                              "end": 166
                            },
                            "start": 157,
                            "end": 167
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "hello",
                              "start": 168,
                              "end": 173
                            },
                            "value": null,
                            "start": 168,
                            "end": 173
                          },
                          {
                            "type": "JSXSpreadAttribute",
                            "argument": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "globalThis",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 178,
                                "end": 188
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "nothing",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 189,
                                "end": 196
                              },
                              "optional": false,
                              "computed": false,
                              "start": 178,
                              "end": 196
                            },
                            "start": 174,
                            "end": 197
                          }
                        ],
                        "selfClosing": true,
                        "start": 134,
                        "end": 200
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 134,
                      "end": 200
                    },
                    "start": 128,
                    "end": 205
                  },
                  "start": 121,
                  "end": 206
                }
              ],
              "start": 116,
              "end": 210
            },
            "id": null,
            "generator": false,
            "start": 105,
            "end": 210
          }
        ],
        "optional": false,
        "start": 94,
        "end": 211
      },
      "exportKind": "value",
      "start": 79,
      "end": 212
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 212
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
import { Foo } from "./foo";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit(Foo, {
        s: Math.random(),
        ..._getVarProps(props),
        ..._getConstProps(props),
        hello: true,
        ...globalThis.nothing
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
            "name": "Foo",
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "name": "Foo",
            "start": 9,
            "end": 12
          },
          "start": 9,
          "end": 12
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./foo",
        "raw": "\"./foo\"",
        "start": 20,
        "end": 27
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 28
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 38,
            "end": 52
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 38,
            "end": 52
          },
          "start": 38,
          "end": 52
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 60,
        "end": 76
      },
      "phase": null,
      "attributes": [],
      "start": 29,
      "end": 77
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 87,
            "end": 99
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 87,
            "end": 99
          },
          "start": 87,
          "end": 99
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 107,
        "end": 123
      },
      "phase": null,
      "attributes": [],
      "start": 78,
      "end": 124
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 134,
            "end": 143
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 134,
            "end": 143
          },
          "start": 134,
          "end": 143
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 151,
        "end": 167
      },
      "phase": null,
      "attributes": [],
      "start": 125,
      "end": 168
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
              "start": 182,
              "end": 208
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 212,
                  "end": 217
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
                        "start": 247,
                        "end": 256
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Foo",
                          "start": 257,
                          "end": 260
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "s",
                                "start": 272,
                                "end": 273
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "Math",
                                    "start": 275,
                                    "end": 279
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "random",
                                    "start": 280,
                                    "end": 286
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 275,
                                  "end": 286
                                },
                                "arguments": [],
                                "optional": false,
                                "start": 275,
                                "end": 288
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 272,
                              "end": 288
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_getVarProps",
                                  "start": 301,
                                  "end": 313
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 314,
                                    "end": 319
                                  }
                                ],
                                "optional": false,
                                "start": 301,
                                "end": 320
                              },
                              "start": 298,
                              "end": 320
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "_getConstProps",
                                  "start": 333,
                                  "end": 347
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 348,
                                    "end": 353
                                  }
                                ],
                                "optional": false,
                                "start": 333,
                                "end": 354
                              },
                              "start": 330,
                              "end": 354
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "hello",
                                "start": 364,
                                "end": 369
                              },
                              "value": {
                                "type": "Literal",
                                "value": true,
                                "raw": "true",
                                "start": 371,
                                "end": 375
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 364,
                              "end": 375
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "globalThis",
                                  "start": 388,
                                  "end": 398
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "nothing",
                                  "start": 399,
                                  "end": 406
                                },
                                "optional": false,
                                "computed": false,
                                "start": 388,
                                "end": 406
                              },
                              "start": 385,
                              "end": 406
                            }
                          ],
                          "start": 262,
                          "end": 412
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 414,
                          "end": 418
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 420,
                          "end": 424
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 426,
                          "end": 427
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 429,
                          "end": 435
                        }
                      ],
                      "optional": false,
                      "start": 247,
                      "end": 436
                    },
                    "start": 226,
                    "end": 437
                  }
                ],
                "start": 220,
                "end": 439
              },
              "id": null,
              "generator": false,
              "start": 211,
              "end": 439
            },
            "start": 182,
            "end": 439
          }
        ],
        "start": 176,
        "end": 440
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 169,
      "end": 440
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 440
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
    109,
    214
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
