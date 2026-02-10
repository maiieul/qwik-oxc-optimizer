# Test: issue_150

## Test Configuration

**Note:** Regression test for issue 150 -- class object prop with conditional values, capturing useStore and external imports in nested $() segments.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { component$, $ } from '@qwik.dev/core';
import { hola } from 'sdfds';

export const Greeter = component$(() => {
	const stuff = useStore();
	return $(() => {
		return (
			<div
				class={{
					'foo': true,
					'bar': stuff.condition,
					'baz': hola ? 'true' : 'false',
				}}
			/>
		)
	});
});

const d = $(()=>console.log('thing'));
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
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "importKind": "value",
          "start": 21,
          "end": 22
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 30,
        "end": 46
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 47
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "hola",
            "optional": false,
            "typeAnnotation": null,
            "start": 57,
            "end": 61
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "hola",
            "optional": false,
            "typeAnnotation": null,
            "start": 57,
            "end": 61
          },
          "importKind": "value",
          "start": 57,
          "end": 61
        }
      ],
      "source": {
        "type": "Literal",
        "value": "sdfds",
        "raw": "'sdfds'",
        "start": 69,
        "end": 76
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 48,
      "end": 77
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
              "name": "Greeter",
              "optional": false,
              "typeAnnotation": null,
              "start": 92,
              "end": 99
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 102,
                "end": 112
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
                              "name": "stuff",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 128,
                              "end": 133
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 136,
                                "end": 144
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 136,
                              "end": 146
                            },
                            "definite": false,
                            "start": 128,
                            "end": 146
                          }
                        ],
                        "declare": false,
                        "start": 122,
                        "end": 147
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 156,
                            "end": 157
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
                                            "start": 181,
                                            "end": 184
                                          },
                                          "typeArguments": null,
                                          "attributes": [
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "class",
                                                "start": 189,
                                                "end": 194
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "ObjectExpression",
                                                  "properties": [
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Literal",
                                                        "value": "foo",
                                                        "raw": "'foo'",
                                                        "start": 203,
                                                        "end": 208
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": true,
                                                        "raw": "true",
                                                        "start": 210,
                                                        "end": 214
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "optional": false,
                                                      "start": 203,
                                                      "end": 214
                                                    },
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Literal",
                                                        "value": "bar",
                                                        "raw": "'bar'",
                                                        "start": 221,
                                                        "end": 226
                                                      },
                                                      "value": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "stuff",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 228,
                                                          "end": 233
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "condition",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 234,
                                                          "end": 243
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 228,
                                                        "end": 243
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "optional": false,
                                                      "start": 221,
                                                      "end": 243
                                                    },
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Literal",
                                                        "value": "baz",
                                                        "raw": "'baz'",
                                                        "start": 250,
                                                        "end": 255
                                                      },
                                                      "value": {
                                                        "type": "ConditionalExpression",
                                                        "test": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "hola",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 257,
                                                          "end": 261
                                                        },
                                                        "consequent": {
                                                          "type": "Literal",
                                                          "value": "true",
                                                          "raw": "'true'",
                                                          "start": 264,
                                                          "end": 270
                                                        },
                                                        "alternate": {
                                                          "type": "Literal",
                                                          "value": "false",
                                                          "raw": "'false'",
                                                          "start": 273,
                                                          "end": 280
                                                        },
                                                        "start": 257,
                                                        "end": 280
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "optional": false,
                                                      "start": 250,
                                                      "end": 280
                                                    }
                                                  ],
                                                  "start": 196,
                                                  "end": 287
                                                },
                                                "start": 195,
                                                "end": 288
                                              },
                                              "start": 189,
                                              "end": 288
                                            }
                                          ],
                                          "selfClosing": true,
                                          "start": 180,
                                          "end": 294
                                        },
                                        "children": [],
                                        "closingElement": null,
                                        "start": 180,
                                        "end": 294
                                      },
                                      "start": 175,
                                      "end": 298
                                    },
                                    "start": 168,
                                    "end": 298
                                  }
                                ],
                                "start": 164,
                                "end": 301
                              },
                              "id": null,
                              "generator": false,
                              "start": 158,
                              "end": 301
                            }
                          ],
                          "optional": false,
                          "start": 156,
                          "end": 302
                        },
                        "start": 149,
                        "end": 303
                      }
                    ],
                    "start": 119,
                    "end": 305
                  },
                  "id": null,
                  "generator": false,
                  "start": 113,
                  "end": 305
                }
              ],
              "optional": false,
              "start": 102,
              "end": 306
            },
            "definite": false,
            "start": 92,
            "end": 306
          }
        ],
        "declare": false,
        "start": 86,
        "end": 307
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 79,
      "end": 307
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
            "name": "d",
            "optional": false,
            "typeAnnotation": null,
            "start": 315,
            "end": 316
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "$",
              "optional": false,
              "typeAnnotation": null,
              "start": 319,
              "end": 320
            },
            "typeArguments": null,
            "arguments": [
              {
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
                      "name": "console",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 325,
                      "end": 332
                    },
                    "property": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "log",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 333,
                      "end": 336
                    },
                    "optional": false,
                    "computed": false,
                    "start": 325,
                    "end": 336
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Literal",
                      "value": "thing",
                      "raw": "'thing'",
                      "start": 337,
                      "end": 344
                    }
                  ],
                  "optional": false,
                  "start": 325,
                  "end": 345
                },
                "id": null,
                "generator": false,
                "start": 321,
                "end": 345
              }
            ],
            "optional": false,
            "start": 319,
            "end": 346
          },
          "definite": false,
          "start": 315,
          "end": 346
        }
      ],
      "declare": false,
      "start": 309,
      "end": 347
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 347
}

```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_n7HuG2hhU0Q = ()=>import("./test.tsx_Greeter_component_n7HuG2hhU0Q");
const i_wKNFJEIQVUA = ()=>import("./test.tsx_d_wKNFJEIQVUA");
export const Greeter = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_n7HuG2hhU0Q, "Greeter_component_n7HuG2hhU0Q"));
/*#__PURE__*/ qrl(i_wKNFJEIQVUA, "d_wKNFJEIQVUA");
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
            "name": "i_n7HuG2hhU0Q",
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
                "value": "./test.tsx_Greeter_component_n7HuG2hhU0Q",
                "raw": "\"./test.tsx_Greeter_component_n7HuG2hhU0Q\"",
                "start": 118,
                "end": 160
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 161
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 161
          },
          "start": 91,
          "end": 161
        }
      ],
      "start": 85,
      "end": 162
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_wKNFJEIQVUA",
            "start": 169,
            "end": 182
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
                "value": "./test.tsx_d_wKNFJEIQVUA",
                "raw": "\"./test.tsx_d_wKNFJEIQVUA\"",
                "start": 196,
                "end": 222
              },
              "options": null,
              "phase": null,
              "start": 189,
              "end": 223
            },
            "id": null,
            "generator": false,
            "start": 185,
            "end": 223
          },
          "start": 169,
          "end": 223
        }
      ],
      "start": 163,
      "end": 224
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
              "name": "Greeter",
              "start": 238,
              "end": 245
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 262,
                "end": 274
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 289,
                    "end": 292
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_n7HuG2hhU0Q",
                      "start": 293,
                      "end": 306
                    },
                    {
                      "type": "Literal",
                      "value": "Greeter_component_n7HuG2hhU0Q",
                      "raw": "\"Greeter_component_n7HuG2hhU0Q\"",
                      "start": 308,
                      "end": 339
                    }
                  ],
                  "optional": false,
                  "start": 289,
                  "end": 340
                }
              ],
              "optional": false,
              "start": 262,
              "end": 341
            },
            "start": 238,
            "end": 341
          }
        ],
        "start": 232,
        "end": 342
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 225,
      "end": 342
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "qrl",
          "start": 357,
          "end": 360
        },
        "arguments": [
          {
            "type": "Identifier",
            "name": "i_wKNFJEIQVUA",
            "start": 361,
            "end": 374
          },
          {
            "type": "Literal",
            "value": "d_wKNFJEIQVUA",
            "raw": "\"d_wKNFJEIQVUA\"",
            "start": 376,
            "end": 391
          }
        ],
        "optional": false,
        "start": 357,
        "end": 392
      },
      "start": 357,
      "end": 393
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 393
}

```

</details>

### Module: test.tsx_d_wKNFJEIQVUA.js (ENTRY POINT)

```javascript
export const d_wKNFJEIQVUA = ()=>console.log('thing');
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
              "name": "d_wKNFJEIQVUA",
              "start": 13,
              "end": 26
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "MemberExpression",
                  "object": {
                    "type": "Identifier",
                    "name": "console",
                    "start": 33,
                    "end": 40
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 41,
                    "end": 44
                  },
                  "optional": false,
                  "computed": false,
                  "start": 33,
                  "end": 44
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "thing",
                    "raw": "'thing'",
                    "start": 45,
                    "end": 52
                  }
                ],
                "optional": false,
                "start": 33,
                "end": 53
              },
              "id": null,
              "generator": false,
              "start": 29,
              "end": 53
            },
            "start": 13,
            "end": 53
          }
        ],
        "start": 7,
        "end": 54
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 54
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 54
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "d_wKNFJEIQVUA",
  "entry": null,
  "displayName": "test.tsx_d",
  "hash": "wKNFJEIQVUA",
  "canonicalFilename": "test.tsx_d_wKNFJEIQVUA",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    323,
    347
  ]
}
```

### Module: test.tsx_Greeter_component_1_krCndSwhX4U.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { hola } from "sdfds";
export const Greeter_component_1_krCndSwhX4U = ()=>{
    const stuff = _captures[0];
    return /*#__PURE__*/ _jsxSorted("div", {
        class: {
            'foo': true,
            'bar': stuff.condition,
            'baz': hola ? 'true' : 'false'
        }
    }, null, null, 3, "u6_0");
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "start": 53,
          "end": 63
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 71,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 88
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "hola",
            "start": 98,
            "end": 102
          },
          "local": {
            "type": "Identifier",
            "name": "hola",
            "start": 98,
            "end": 102
          },
          "start": 98,
          "end": 102
        }
      ],
      "source": {
        "type": "Literal",
        "value": "sdfds",
        "raw": "\"sdfds\"",
        "start": 110,
        "end": 117
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 118
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
              "name": "Greeter_component_1_krCndSwhX4U",
              "start": 132,
              "end": 163
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
                          "name": "stuff",
                          "start": 182,
                          "end": 187
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 190,
                            "end": 199
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 200,
                            "end": 201
                          },
                          "optional": false,
                          "computed": true,
                          "start": 190,
                          "end": 202
                        },
                        "start": 182,
                        "end": 202
                      }
                    ],
                    "start": 176,
                    "end": 203
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 229,
                        "end": 239
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 240,
                          "end": 245
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "class",
                                "start": 257,
                                "end": 262
                              },
                              "value": {
                                "type": "ObjectExpression",
                                "properties": [
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Literal",
                                      "value": "foo",
                                      "raw": "'foo'",
                                      "start": 278,
                                      "end": 283
                                    },
                                    "value": {
                                      "type": "Literal",
                                      "value": true,
                                      "raw": "true",
                                      "start": 285,
                                      "end": 289
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "start": 278,
                                    "end": 289
                                  },
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Literal",
                                      "value": "bar",
                                      "raw": "'bar'",
                                      "start": 303,
                                      "end": 308
                                    },
                                    "value": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "name": "stuff",
                                        "start": 310,
                                        "end": 315
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "condition",
                                        "start": 316,
                                        "end": 325
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 310,
                                      "end": 325
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "start": 303,
                                    "end": 325
                                  },
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Literal",
                                      "value": "baz",
                                      "raw": "'baz'",
                                      "start": 339,
                                      "end": 344
                                    },
                                    "value": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "Identifier",
                                        "name": "hola",
                                        "start": 346,
                                        "end": 350
                                      },
                                      "consequent": {
                                        "type": "Literal",
                                        "value": "true",
                                        "raw": "'true'",
                                        "start": 353,
                                        "end": 359
                                      },
                                      "alternate": {
                                        "type": "Literal",
                                        "value": "false",
                                        "raw": "'false'",
                                        "start": 362,
                                        "end": 369
                                      },
                                      "start": 346,
                                      "end": 369
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "start": 339,
                                    "end": 369
                                  }
                                ],
                                "start": 264,
                                "end": 379
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 257,
                              "end": 379
                            }
                          ],
                          "start": 247,
                          "end": 385
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 387,
                          "end": 391
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 393,
                          "end": 397
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 399,
                          "end": 400
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 402,
                          "end": 408
                        }
                      ],
                      "optional": false,
                      "start": 229,
                      "end": 409
                    },
                    "start": 208,
                    "end": 410
                  }
                ],
                "start": 170,
                "end": 412
              },
              "id": null,
              "generator": false,
              "start": 166,
              "end": 412
            },
            "start": 132,
            "end": 412
          }
        ],
        "start": 126,
        "end": 413
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 119,
      "end": 413
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 413
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_1_krCndSwhX4U",
  "entry": null,
  "displayName": "test.tsx_Greeter_component_1",
  "hash": "krCndSwhX4U",
  "canonicalFilename": "test.tsx_Greeter_component_1_krCndSwhX4U",
  "path": "",
  "extension": "js",
  "parent": "Greeter_component_n7HuG2hhU0Q",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    160,
    303
  ],
  "captureNames": [
    "stuff"
  ]
}
```

### Module: test.tsx_Greeter_component_n7HuG2hhU0Q.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_krCndSwhX4U = ()=>import("./test.tsx_Greeter_component_1_krCndSwhX4U");
export const Greeter_component_n7HuG2hhU0Q = ()=>{
    const stuff = useStore();
    return /*#__PURE__*/ qrl(i_krCndSwhX4U, "Greeter_component_1_krCndSwhX4U", [
        stuff
    ]);
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
            "name": "qrl",
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 9,
            "end": 12
          },
          "start": 9,
          "end": 12
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 20,
        "end": 36
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 37
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_krCndSwhX4U",
            "start": 44,
            "end": 57
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
                "value": "./test.tsx_Greeter_component_1_krCndSwhX4U",
                "raw": "\"./test.tsx_Greeter_component_1_krCndSwhX4U\"",
                "start": 71,
                "end": 115
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 116
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 116
          },
          "start": 44,
          "end": 116
        }
      ],
      "start": 38,
      "end": 117
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
              "name": "Greeter_component_n7HuG2hhU0Q",
              "start": 131,
              "end": 160
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
                          "name": "stuff",
                          "start": 179,
                          "end": 184
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 187,
                            "end": 195
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 187,
                          "end": 197
                        },
                        "start": 179,
                        "end": 197
                      }
                    ],
                    "start": 173,
                    "end": 198
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "qrl",
                        "start": 224,
                        "end": 227
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_krCndSwhX4U",
                          "start": 228,
                          "end": 241
                        },
                        {
                          "type": "Literal",
                          "value": "Greeter_component_1_krCndSwhX4U",
                          "raw": "\"Greeter_component_1_krCndSwhX4U\"",
                          "start": 243,
                          "end": 276
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "stuff",
                              "start": 288,
                              "end": 293
                            }
                          ],
                          "start": 278,
                          "end": 299
                        }
                      ],
                      "optional": false,
                      "start": 224,
                      "end": 300
                    },
                    "start": 203,
                    "end": 301
                  }
                ],
                "start": 167,
                "end": 303
              },
              "id": null,
              "generator": false,
              "start": 163,
              "end": 303
            },
            "start": 131,
            "end": 303
          }
        ],
        "start": 125,
        "end": 304
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 118,
      "end": 304
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 304
}

```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Greeter_component_n7HuG2hhU0Q",
  "entry": null,
  "displayName": "test.tsx_Greeter_component",
  "hash": "n7HuG2hhU0Q",
  "canonicalFilename": "test.tsx_Greeter_component_n7HuG2hhU0Q",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    115,
    307
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-03] JSX Transforms**
- **[CONV-05] Capture Patterns**
- **[CONV-06] Lazy Imports**
- **[CONV-07] PURE Annotations**
- **[CONV-08] Segment Extraction**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 2 |
| _jsxSorted | test.tsx_Greeter_component_1_krCndSwhX4U.js | @qwik.dev/core | 1 |
| qrl | test.tsx_Greeter_component_n7HuG2hhU0Q.js | @qwik.dev/core | 1 |
| useStore | test.tsx_Greeter_component_n7HuG2hhU0Q.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
