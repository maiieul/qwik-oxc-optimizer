# Test: issue_7216_add_test

## Test Configuration

**Note:** Regression test for issue 7216 -- multiple event handlers and spread props interleaved on a single JSX element. Tests _jsxSplit with mixed spread and event handler ordering.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$ } from '@builder.io/qwik';
export default component$((props) => {
  return (<p 
		onHi$={() => 'hi'} 
		{...props.foo} 
		onHello$={props.helloHandler$} 
		{...props.rest} 
		onVar$={props.onVarHandler$} 
		onConst$={() => 'const'} 
		asd={"1"}
	/>);
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
        "value": "@builder.io/qwik",
        "raw": "'@builder.io/qwik'",
        "start": 27,
        "end": 45
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 46
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
          "start": 62,
          "end": 72
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
                "start": 74,
                "end": 79
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
                          "name": "p",
                          "start": 97,
                          "end": 98
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onHi$",
                              "start": 102,
                              "end": 107
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
                                  "type": "Literal",
                                  "value": "hi",
                                  "raw": "'hi'",
                                  "start": 115,
                                  "end": 119
                                },
                                "id": null,
                                "generator": false,
                                "start": 109,
                                "end": 119
                              },
                              "start": 108,
                              "end": 120
                            },
                            "start": 102,
                            "end": 120
                          },
                          {
                            "type": "JSXSpreadAttribute",
                            "argument": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "props",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 128,
                                "end": 133
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "foo",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 134,
                                "end": 137
                              },
                              "optional": false,
                              "computed": false,
                              "start": 128,
                              "end": 137
                            },
                            "start": 124,
                            "end": 138
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onHello$",
                              "start": 142,
                              "end": 150
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "props",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 152,
                                  "end": 157
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "helloHandler$",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 158,
                                  "end": 171
                                },
                                "optional": false,
                                "computed": false,
                                "start": 152,
                                "end": 171
                              },
                              "start": 151,
                              "end": 172
                            },
                            "start": 142,
                            "end": 172
                          },
                          {
                            "type": "JSXSpreadAttribute",
                            "argument": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "props",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 180,
                                "end": 185
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "rest",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 186,
                                "end": 190
                              },
                              "optional": false,
                              "computed": false,
                              "start": 180,
                              "end": 190
                            },
                            "start": 176,
                            "end": 191
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onVar$",
                              "start": 195,
                              "end": 201
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "props",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 203,
                                  "end": 208
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "onVarHandler$",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 209,
                                  "end": 222
                                },
                                "optional": false,
                                "computed": false,
                                "start": 203,
                                "end": 222
                              },
                              "start": 202,
                              "end": 223
                            },
                            "start": 195,
                            "end": 223
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onConst$",
                              "start": 227,
                              "end": 235
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
                                  "type": "Literal",
                                  "value": "const",
                                  "raw": "'const'",
                                  "start": 243,
                                  "end": 250
                                },
                                "id": null,
                                "generator": false,
                                "start": 237,
                                "end": 250
                              },
                              "start": 236,
                              "end": 251
                            },
                            "start": 227,
                            "end": 251
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "asd",
                              "start": 255,
                              "end": 258
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "Literal",
                                "value": "1",
                                "raw": "\"1\"",
                                "start": 260,
                                "end": 263
                              },
                              "start": 259,
                              "end": 264
                            },
                            "start": 255,
                            "end": 264
                          }
                        ],
                        "selfClosing": true,
                        "start": 96,
                        "end": 268
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 96,
                      "end": 268
                    },
                    "start": 95,
                    "end": 269
                  },
                  "start": 88,
                  "end": 270
                }
              ],
              "start": 84,
              "end": 272
            },
            "id": null,
            "generator": false,
            "start": 73,
            "end": 272
          }
        ],
        "optional": false,
        "start": 62,
        "end": 273
      },
      "exportKind": "value",
      "start": 47,
      "end": 274
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 274
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

### Module: test.tsx_test_component_p_q_e_hi_ttOKZbY46GA.js (ENTRY POINT)

```javascript
export const test_component_p_q_e_hi_ttOKZbY46GA = ()=>'hi';
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
              "name": "test_component_p_q_e_hi_ttOKZbY46GA",
              "start": 13,
              "end": 48
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [],
              "body": {
                "type": "Literal",
                "value": "hi",
                "raw": "'hi'",
                "start": 55,
                "end": 59
              },
              "id": null,
              "generator": false,
              "start": 51,
              "end": 59
            },
            "start": 13,
            "end": 59
          }
        ],
        "start": 7,
        "end": 60
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 60
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 60
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_p_q_e_hi_ttOKZbY46GA",
  "entry": null,
  "displayName": "test.tsx_test_component_p_q_e_hi",
  "hash": "ttOKZbY46GA",
  "canonicalFilename": "test.tsx_test_component_p_q_e_hi_ttOKZbY46GA",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onHi$",
  "captures": false,
  "loc": [
    111,
    121
  ]
}
```

### Module: test.tsx_test_component_p_q_e_const_e6TYnIxnPLY.js (ENTRY POINT)

```javascript
export const test_component_p_q_e_const_e6TYnIxnPLY = ()=>'const';
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
              "name": "test_component_p_q_e_const_e6TYnIxnPLY",
              "start": 13,
              "end": 51
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [],
              "body": {
                "type": "Literal",
                "value": "const",
                "raw": "'const'",
                "start": 58,
                "end": 65
              },
              "id": null,
              "generator": false,
              "start": 54,
              "end": 65
            },
            "start": 13,
            "end": 65
          }
        ],
        "start": 7,
        "end": 66
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 66
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 66
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "test_component_p_q_e_const_e6TYnIxnPLY",
  "entry": null,
  "displayName": "test.tsx_test_component_p_q_e_const",
  "hash": "e6TYnIxnPLY",
  "canonicalFilename": "test.tsx_test_component_p_q_e_const_e6TYnIxnPLY",
  "path": "",
  "extension": "js",
  "parent": "test_component_LUXeXe0DQrg",
  "ctxKind": "eventHandler",
  "ctxName": "onConst$",
  "captures": false,
  "loc": [
    239,
    252
  ]
}
```

### Module: test.tsx_test_component_LUXeXe0DQrg.js (ENTRY POINT)

```javascript
import { _jsxSplit } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_e6TYnIxnPLY = ()=>import("./test.tsx_test_component_p_q_e_const_e6TYnIxnPLY");
const i_ttOKZbY46GA = ()=>import("./test.tsx_test_component_p_q_e_hi_ttOKZbY46GA");
export const test_component_LUXeXe0DQrg = (props)=>{
    return /*#__PURE__*/ _jsxSplit("p", {
        "q-e:hi": /*#__PURE__*/ qrl(i_ttOKZbY46GA, "test_component_p_q_e_hi_ttOKZbY46GA"),
        ...props.foo,
        "q-e:hello": props.helloHandler$,
        ...props.rest,
        "q-e:var": props.onVarHandler$
    }, {
        "q-e:const": /*#__PURE__*/ qrl(i_e6TYnIxnPLY, "test_component_p_q_e_const_e6TYnIxnPLY"),
        asd: "1"
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
            "name": "_jsxSplit",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
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
            "name": "qrl",
            "start": 53,
            "end": 56
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 53,
            "end": 56
          },
          "start": 53,
          "end": 56
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 64,
        "end": 80
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 81
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_e6TYnIxnPLY",
            "start": 88,
            "end": 101
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
                "value": "./test.tsx_test_component_p_q_e_const_e6TYnIxnPLY",
                "raw": "\"./test.tsx_test_component_p_q_e_const_e6TYnIxnPLY\"",
                "start": 115,
                "end": 166
              },
              "options": null,
              "phase": null,
              "start": 108,
              "end": 167
            },
            "id": null,
            "generator": false,
            "start": 104,
            "end": 167
          },
          "start": 88,
          "end": 167
        }
      ],
      "start": 82,
      "end": 168
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_ttOKZbY46GA",
            "start": 175,
            "end": 188
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
                "value": "./test.tsx_test_component_p_q_e_hi_ttOKZbY46GA",
                "raw": "\"./test.tsx_test_component_p_q_e_hi_ttOKZbY46GA\"",
                "start": 202,
                "end": 250
              },
              "options": null,
              "phase": null,
              "start": 195,
              "end": 251
            },
            "id": null,
            "generator": false,
            "start": 191,
            "end": 251
          },
          "start": 175,
          "end": 251
        }
      ],
      "start": 169,
      "end": 252
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
              "start": 266,
              "end": 292
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 296,
                  "end": 301
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
                        "start": 331,
                        "end": 340
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "p",
                          "raw": "\"p\"",
                          "start": 341,
                          "end": 344
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:hi",
                                "raw": "\"q-e:hi\"",
                                "start": 356,
                                "end": 364
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 380,
                                  "end": 383
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_ttOKZbY46GA",
                                    "start": 384,
                                    "end": 397
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "test_component_p_q_e_hi_ttOKZbY46GA",
                                    "raw": "\"test_component_p_q_e_hi_ttOKZbY46GA\"",
                                    "start": 399,
                                    "end": 436
                                  }
                                ],
                                "optional": false,
                                "start": 380,
                                "end": 437
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 356,
                              "end": 437
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "props",
                                  "start": 450,
                                  "end": 455
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "foo",
                                  "start": 456,
                                  "end": 459
                                },
                                "optional": false,
                                "computed": false,
                                "start": 450,
                                "end": 459
                              },
                              "start": 447,
                              "end": 459
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:hello",
                                "raw": "\"q-e:hello\"",
                                "start": 469,
                                "end": 480
                              },
                              "value": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "props",
                                  "start": 482,
                                  "end": 487
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "helloHandler$",
                                  "start": 488,
                                  "end": 501
                                },
                                "optional": false,
                                "computed": false,
                                "start": 482,
                                "end": 501
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 469,
                              "end": 501
                            },
                            {
                              "type": "SpreadElement",
                              "argument": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "props",
                                  "start": 514,
                                  "end": 519
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "rest",
                                  "start": 520,
                                  "end": 524
                                },
                                "optional": false,
                                "computed": false,
                                "start": 514,
                                "end": 524
                              },
                              "start": 511,
                              "end": 524
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:var",
                                "raw": "\"q-e:var\"",
                                "start": 534,
                                "end": 543
                              },
                              "value": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "props",
                                  "start": 545,
                                  "end": 550
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "onVarHandler$",
                                  "start": 551,
                                  "end": 564
                                },
                                "optional": false,
                                "computed": false,
                                "start": 545,
                                "end": 564
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 534,
                              "end": 564
                            }
                          ],
                          "start": 346,
                          "end": 570
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:const",
                                "raw": "\"q-e:const\"",
                                "start": 582,
                                "end": 593
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 609,
                                  "end": 612
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_e6TYnIxnPLY",
                                    "start": 613,
                                    "end": 626
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "test_component_p_q_e_const_e6TYnIxnPLY",
                                    "raw": "\"test_component_p_q_e_const_e6TYnIxnPLY\"",
                                    "start": 628,
                                    "end": 668
                                  }
                                ],
                                "optional": false,
                                "start": 609,
                                "end": 669
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 582,
                              "end": 669
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "asd",
                                "start": 679,
                                "end": 682
                              },
                              "value": {
                                "type": "Literal",
                                "value": "1",
                                "raw": "\"1\"",
                                "start": 684,
                                "end": 687
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 679,
                              "end": 687
                            }
                          ],
                          "start": 572,
                          "end": 693
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 695,
                          "end": 699
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 701,
                          "end": 702
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 704,
                          "end": 710
                        }
                      ],
                      "optional": false,
                      "start": 331,
                      "end": 711
                    },
                    "start": 310,
                    "end": 712
                  }
                ],
                "start": 304,
                "end": 714
              },
              "id": null,
              "generator": false,
              "start": 295,
              "end": 714
            },
            "start": 266,
            "end": 714
          }
        ],
        "start": 260,
        "end": 715
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 253,
      "end": 715
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 715
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
    75,
    274
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
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 2 |
| _jsxSplit | test.tsx_test_component_LUXeXe0DQrg.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
