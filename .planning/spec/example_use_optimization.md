# Test: example_use_optimization

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile JSX | False |
| Entry Strategy | Inline |
| Transpile TS | True |
| Is Server | Some(false) |

## Input

### Source Code

```tsx
import { $, component$, useTask$ } from '@qwik.dev/core';
import { CONST } from 'const';
export const Works = component$((props) => {
	const {countNested} = useStore({value:{count:0}}).value;
	const countNested2 = countNested;
	const {hello} = countNested2;
	const bye = hello.bye;
	const {ciao} = bye.italian;


	return (
		<div ciao={ciao} >{foo}</div>
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
            "name": "useTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 32
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 32
          },
          "importKind": "value",
          "start": 24,
          "end": 32
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 40,
        "end": 56
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 57
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "CONST",
            "optional": false,
            "typeAnnotation": null,
            "start": 67,
            "end": 72
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "CONST",
            "optional": false,
            "typeAnnotation": null,
            "start": 67,
            "end": 72
          },
          "importKind": "value",
          "start": 67,
          "end": 72
        }
      ],
      "source": {
        "type": "Literal",
        "value": "const",
        "raw": "'const'",
        "start": 80,
        "end": 87
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 58,
      "end": 88
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
              "name": "Works",
              "optional": false,
              "typeAnnotation": null,
              "start": 102,
              "end": 107
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 110,
                "end": 120
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
                      "start": 122,
                      "end": 127
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
                              "type": "ObjectPattern",
                              "decorators": [],
                              "properties": [
                                {
                                  "type": "Property",
                                  "kind": "init",
                                  "key": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "countNested",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 142,
                                    "end": 153
                                  },
                                  "value": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "countNested",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 142,
                                    "end": 153
                                  },
                                  "method": false,
                                  "shorthand": true,
                                  "computed": false,
                                  "optional": false,
                                  "start": 142,
                                  "end": 153
                                }
                              ],
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 141,
                              "end": 154
                            },
                            "init": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "useStore",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 157,
                                  "end": 165
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
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
                                          "start": 167,
                                          "end": 172
                                        },
                                        "value": {
                                          "type": "ObjectExpression",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "count",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 174,
                                                "end": 179
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": 0,
                                                "raw": "0",
                                                "start": 180,
                                                "end": 181
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "optional": false,
                                              "start": 174,
                                              "end": 181
                                            }
                                          ],
                                          "start": 173,
                                          "end": 182
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "optional": false,
                                        "start": 167,
                                        "end": 182
                                      }
                                    ],
                                    "start": 166,
                                    "end": 183
                                  }
                                ],
                                "optional": false,
                                "start": 157,
                                "end": 184
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "value",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 185,
                                "end": 190
                              },
                              "optional": false,
                              "computed": false,
                              "start": 157,
                              "end": 190
                            },
                            "definite": false,
                            "start": 141,
                            "end": 190
                          }
                        ],
                        "declare": false,
                        "start": 135,
                        "end": 191
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
                              "name": "countNested2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 199,
                              "end": 211
                            },
                            "init": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "countNested",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 214,
                              "end": 225
                            },
                            "definite": false,
                            "start": 199,
                            "end": 225
                          }
                        ],
                        "declare": false,
                        "start": 193,
                        "end": 226
                      },
                      {
                        "type": "VariableDeclaration",
                        "kind": "const",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "ObjectPattern",
                              "decorators": [],
                              "properties": [
                                {
                                  "type": "Property",
                                  "kind": "init",
                                  "key": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "hello",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 235,
                                    "end": 240
                                  },
                                  "value": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "hello",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 235,
                                    "end": 240
                                  },
                                  "method": false,
                                  "shorthand": true,
                                  "computed": false,
                                  "optional": false,
                                  "start": 235,
                                  "end": 240
                                }
                              ],
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 234,
                              "end": 241
                            },
                            "init": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "countNested2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 244,
                              "end": 256
                            },
                            "definite": false,
                            "start": 234,
                            "end": 256
                          }
                        ],
                        "declare": false,
                        "start": 228,
                        "end": 257
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
                              "name": "bye",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 265,
                              "end": 268
                            },
                            "init": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "hello",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 271,
                                "end": 276
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "bye",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 277,
                                "end": 280
                              },
                              "optional": false,
                              "computed": false,
                              "start": 271,
                              "end": 280
                            },
                            "definite": false,
                            "start": 265,
                            "end": 280
                          }
                        ],
                        "declare": false,
                        "start": 259,
                        "end": 281
                      },
                      {
                        "type": "VariableDeclaration",
                        "kind": "const",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "ObjectPattern",
                              "decorators": [],
                              "properties": [
                                {
                                  "type": "Property",
                                  "kind": "init",
                                  "key": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ciao",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 290,
                                    "end": 294
                                  },
                                  "value": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ciao",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 290,
                                    "end": 294
                                  },
                                  "method": false,
                                  "shorthand": true,
                                  "computed": false,
                                  "optional": false,
                                  "start": 290,
                                  "end": 294
                                }
                              ],
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 289,
                              "end": 295
                            },
                            "init": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "bye",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 298,
                                "end": 301
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "italian",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 302,
                                "end": 309
                              },
                              "optional": false,
                              "computed": false,
                              "start": 298,
                              "end": 309
                            },
                            "definite": false,
                            "start": 289,
                            "end": 309
                          }
                        ],
                        "declare": false,
                        "start": 283,
                        "end": 310
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
                                "name": "div",
                                "start": 326,
                                "end": 329
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "ciao",
                                    "start": 330,
                                    "end": 334
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "ciao",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 336,
                                      "end": 340
                                    },
                                    "start": 335,
                                    "end": 341
                                  },
                                  "start": 330,
                                  "end": 341
                                }
                              ],
                              "selfClosing": false,
                              "start": 325,
                              "end": 343
                            },
                            "children": [
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "foo",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 344,
                                  "end": 347
                                },
                                "start": 343,
                                "end": 348
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 350,
                                "end": 353
                              },
                              "start": 348,
                              "end": 354
                            },
                            "start": 325,
                            "end": 354
                          },
                          "start": 321,
                          "end": 357
                        },
                        "start": 314,
                        "end": 358
                      }
                    ],
                    "start": 132,
                    "end": 360
                  },
                  "id": null,
                  "generator": false,
                  "start": 121,
                  "end": 360
                }
              ],
              "optional": false,
              "start": 110,
              "end": 361
            },
            "definite": false,
            "start": 102,
            "end": 361
          }
        ],
        "declare": false,
        "start": 96,
        "end": 362
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 89,
      "end": 362
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 362
}
```

</details>

## Output

### Module: test.jsx

```jsx
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const Works = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((props)=>{
    const store = useStore({
        value: {
            count: 0
        }
    });
    return <div ciao={store.value.countNested.hello.bye.italian.ciao}>{foo}</div>;
}, "Works_component_t45qL4vNGv0"));
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
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "start": 56,
          "end": 66
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 74,
        "end": 90
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 91
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
              "name": "Works",
              "start": 105,
              "end": 110
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 127,
                "end": 139
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 154,
                    "end": 164
                  },
                  "arguments": [
                    {
                      "type": "ArrowFunctionExpression",
                      "expression": false,
                      "async": false,
                      "params": [
                        {
                          "type": "Identifier",
                          "name": "props",
                          "start": 166,
                          "end": 171
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
                                  "name": "store",
                                  "start": 186,
                                  "end": 191
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useStore",
                                    "start": 194,
                                    "end": 202
                                  },
                                  "arguments": [
                                    {
                                      "type": "ObjectExpression",
                                      "properties": [
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "value",
                                            "start": 213,
                                            "end": 218
                                          },
                                          "value": {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "count",
                                                  "start": 234,
                                                  "end": 239
                                                },
                                                "value": {
                                                  "type": "Literal",
                                                  "value": 0,
                                                  "raw": "0",
                                                  "start": 241,
                                                  "end": 242
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 234,
                                                "end": 242
                                              }
                                            ],
                                            "start": 220,
                                            "end": 252
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 213,
                                          "end": 252
                                        }
                                      ],
                                      "start": 203,
                                      "end": 258
                                    }
                                  ],
                                  "optional": false,
                                  "start": 194,
                                  "end": 259
                                },
                                "start": 186,
                                "end": 259
                              }
                            ],
                            "start": 180,
                            "end": 260
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 273,
                                  "end": 276
                                },
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "ciao",
                                      "start": 277,
                                      "end": 281
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "name": "store",
                                                    "start": 283,
                                                    "end": 288
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "name": "value",
                                                    "start": 289,
                                                    "end": 294
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 283,
                                                  "end": 294
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "name": "countNested",
                                                  "start": 295,
                                                  "end": 306
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 283,
                                                "end": 306
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "name": "hello",
                                                "start": 307,
                                                "end": 312
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 283,
                                              "end": 312
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "name": "bye",
                                              "start": 313,
                                              "end": 316
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 283,
                                            "end": 316
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "italian",
                                            "start": 317,
                                            "end": 324
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 283,
                                          "end": 324
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "ciao",
                                          "start": 325,
                                          "end": 329
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 283,
                                        "end": 329
                                      },
                                      "start": 282,
                                      "end": 330
                                    },
                                    "start": 277,
                                    "end": 330
                                  }
                                ],
                                "selfClosing": false,
                                "start": 272,
                                "end": 331
                              },
                              "children": [
                                {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "Identifier",
                                    "name": "foo",
                                    "start": 332,
                                    "end": 335
                                  },
                                  "start": 331,
                                  "end": 336
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 338,
                                  "end": 341
                                },
                                "start": 336,
                                "end": 342
                              },
                              "start": 272,
                              "end": 342
                            },
                            "start": 265,
                            "end": 343
                          }
                        ],
                        "start": 174,
                        "end": 345
                      },
                      "id": null,
                      "generator": false,
                      "start": 165,
                      "end": 345
                    },
                    {
                      "type": "Literal",
                      "value": "Works_component_t45qL4vNGv0",
                      "raw": "\"Works_component_t45qL4vNGv0\"",
                      "start": 347,
                      "end": 376
                    }
                  ],
                  "optional": false,
                  "start": 154,
                  "end": 377
                }
              ],
              "optional": false,
              "start": 127,
              "end": 378
            },
            "start": 105,
            "end": 378
          }
        ],
        "start": 99,
        "end": 379
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 92,
      "end": 379
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 379
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (2 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `inlinedQrl` | test.jsx | @qwik.dev/core | 1 |
| `componentQrl` | test.jsx | @qwik.dev/core | 1 |
| `useStore` | test.jsx | local/scope | 1 |

## Diagnostics

```json
[]
```
