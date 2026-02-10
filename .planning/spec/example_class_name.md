# Test: example_class_name

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |
| Explicit Extensions | True |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

export const App2 = component$(() => {
	const signal = useSignal();
	const computed = signal.value + 'foo';
	return (
		<>
			<div className="hola"></div>
			<div className={signal.value}></div>
			<div className={signal}></div>
			<div className={computed}></div>

			<Foo className="hola"></Foo>
			<Foo className={signal.value}></Foo>
			<Foo className={signal}></Foo>
			<Foo className={computed}></Foo>
		</>
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
              "name": "App2",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
              "end": 63
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 66,
                "end": 76
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
                              "name": "signal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 92,
                              "end": 98
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useSignal",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 101,
                                "end": 110
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 101,
                              "end": 112
                            },
                            "definite": false,
                            "start": 92,
                            "end": 112
                          }
                        ],
                        "declare": false,
                        "start": 86,
                        "end": 113
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
                              "name": "computed",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 121,
                              "end": 129
                            },
                            "init": {
                              "type": "BinaryExpression",
                              "left": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "signal",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 132,
                                  "end": 138
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "value",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 139,
                                  "end": 144
                                },
                                "optional": false,
                                "computed": false,
                                "start": 132,
                                "end": 144
                              },
                              "operator": "+",
                              "right": {
                                "type": "Literal",
                                "value": "foo",
                                "raw": "'foo'",
                                "start": 147,
                                "end": 152
                              },
                              "start": 132,
                              "end": 152
                            },
                            "definite": false,
                            "start": 121,
                            "end": 152
                          }
                        ],
                        "declare": false,
                        "start": 115,
                        "end": 153
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 166,
                              "end": 168
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 168,
                                "end": 172
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 173,
                                    "end": 176
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 177,
                                        "end": 186
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "hola",
                                        "raw": "\"hola\"",
                                        "start": 187,
                                        "end": 193
                                      },
                                      "start": 177,
                                      "end": 193
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 172,
                                  "end": 194
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 196,
                                    "end": 199
                                  },
                                  "start": 194,
                                  "end": 200
                                },
                                "start": 172,
                                "end": 200
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 200,
                                "end": 204
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 205,
                                    "end": 208
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 209,
                                        "end": 218
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "signal",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 220,
                                            "end": 226
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "value",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 227,
                                            "end": 232
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 220,
                                          "end": 232
                                        },
                                        "start": 219,
                                        "end": 233
                                      },
                                      "start": 209,
                                      "end": 233
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 204,
                                  "end": 234
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 236,
                                    "end": 239
                                  },
                                  "start": 234,
                                  "end": 240
                                },
                                "start": 204,
                                "end": 240
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 240,
                                "end": 244
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 245,
                                    "end": 248
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 249,
                                        "end": 258
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "signal",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 260,
                                          "end": 266
                                        },
                                        "start": 259,
                                        "end": 267
                                      },
                                      "start": 249,
                                      "end": 267
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 244,
                                  "end": 268
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 270,
                                    "end": 273
                                  },
                                  "start": 268,
                                  "end": 274
                                },
                                "start": 244,
                                "end": 274
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 274,
                                "end": 278
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 279,
                                    "end": 282
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 283,
                                        "end": 292
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "computed",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 294,
                                          "end": 302
                                        },
                                        "start": 293,
                                        "end": 303
                                      },
                                      "start": 283,
                                      "end": 303
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 278,
                                  "end": 304
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 306,
                                    "end": 309
                                  },
                                  "start": 304,
                                  "end": 310
                                },
                                "start": 278,
                                "end": 310
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t\t",
                                "raw": "\n\n\t\t\t",
                                "start": 310,
                                "end": 315
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 316,
                                    "end": 319
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 320,
                                        "end": 329
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "hola",
                                        "raw": "\"hola\"",
                                        "start": 330,
                                        "end": 336
                                      },
                                      "start": 320,
                                      "end": 336
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 315,
                                  "end": 337
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 339,
                                    "end": 342
                                  },
                                  "start": 337,
                                  "end": 343
                                },
                                "start": 315,
                                "end": 343
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 343,
                                "end": 347
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 348,
                                    "end": 351
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 352,
                                        "end": 361
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "signal",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 363,
                                            "end": 369
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "value",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 370,
                                            "end": 375
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 363,
                                          "end": 375
                                        },
                                        "start": 362,
                                        "end": 376
                                      },
                                      "start": 352,
                                      "end": 376
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 347,
                                  "end": 377
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 379,
                                    "end": 382
                                  },
                                  "start": 377,
                                  "end": 383
                                },
                                "start": 347,
                                "end": 383
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 383,
                                "end": 387
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 388,
                                    "end": 391
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 392,
                                        "end": 401
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "signal",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 403,
                                          "end": 409
                                        },
                                        "start": 402,
                                        "end": 410
                                      },
                                      "start": 392,
                                      "end": 410
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 387,
                                  "end": 411
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 413,
                                    "end": 416
                                  },
                                  "start": 411,
                                  "end": 417
                                },
                                "start": 387,
                                "end": 417
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 417,
                                "end": 421
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 422,
                                    "end": 425
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "className",
                                        "start": 426,
                                        "end": 435
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "computed",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 437,
                                          "end": 445
                                        },
                                        "start": 436,
                                        "end": 446
                                      },
                                      "start": 426,
                                      "end": 446
                                    }
                                  ],
                                  "selfClosing": false,
                                  "start": 421,
                                  "end": 447
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Foo",
                                    "start": 449,
                                    "end": 452
                                  },
                                  "start": 447,
                                  "end": 453
                                },
                                "start": 421,
                                "end": 453
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 453,
                                "end": 456
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 456,
                              "end": 459
                            },
                            "start": 166,
                            "end": 459
                          },
                          "start": 162,
                          "end": 462
                        },
                        "start": 155,
                        "end": 463
                      }
                    ],
                    "start": 83,
                    "end": 465
                  },
                  "id": null,
                  "generator": false,
                  "start": 77,
                  "end": 465
                }
              ],
              "optional": false,
              "start": 66,
              "end": 466
            },
            "definite": false,
            "start": 59,
            "end": 466
          }
        ],
        "declare": false,
        "start": 53,
        "end": 467
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 467
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 467
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_3yveMqbQ3Fs = ()=>import("./test.tsx_App2_component_3yveMqbQ3Fs.js");
export const App2 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_3yveMqbQ3Fs, "App2_component_3yveMqbQ3Fs"));
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
            "name": "i_3yveMqbQ3Fs",
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
                "value": "./test.tsx_App2_component_3yveMqbQ3Fs.js",
                "raw": "\"./test.tsx_App2_component_3yveMqbQ3Fs.js\"",
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
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "name": "App2",
              "start": 176,
              "end": 180
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 197,
                "end": 209
              },
              "arguments": [
                {
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
                      "name": "i_3yveMqbQ3Fs",
                      "start": 228,
                      "end": 241
                    },
                    {
                      "type": "Literal",
                      "value": "App2_component_3yveMqbQ3Fs",
                      "raw": "\"App2_component_3yveMqbQ3Fs\"",
                      "start": 243,
                      "end": 271
                    }
                  ],
                  "optional": false,
                  "start": 224,
                  "end": 272
                }
              ],
              "optional": false,
              "start": 197,
              "end": 273
            },
            "start": 176,
            "end": 273
          }
        ],
        "start": 170,
        "end": 274
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 163,
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

### Module: test.tsx_App2_component_3yveMqbQ3Fs.js (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
export const App2_component_3yveMqbQ3Fs = ()=>{
    const signal = useSignal();
    const computed = signal.value + 'foo';
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, {
            "class": "hola"
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            "class": _wrapProp(signal)
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            "class": signal
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("div", {
            "class": computed
        }, null, null, 3, null),
        /*#__PURE__*/ _jsxSorted(Foo, null, {
            className: "hola"
        }, null, 3, "u6_0"),
        /*#__PURE__*/ _jsxSorted(Foo, null, {
            className: _wrapProp(signal)
        }, null, 3, "u6_1"),
        /*#__PURE__*/ _jsxSorted(Foo, null, {
            className: signal
        }, null, 3, "u6_2"),
        /*#__PURE__*/ _jsxSorted(Foo, {
            className: computed
        }, null, null, 3, "u6_3")
    ], 1, "u6_4");
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
            "name": "Fragment",
            "start": 9,
            "end": 17
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 21,
            "end": 30
          },
          "start": 9,
          "end": 30
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 38,
        "end": 66
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 67
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 77,
            "end": 87
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 77,
            "end": 87
          },
          "start": 77,
          "end": 87
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 95,
        "end": 111
      },
      "phase": null,
      "attributes": [],
      "start": 68,
      "end": 112
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 122,
            "end": 131
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 122,
            "end": 131
          },
          "start": 122,
          "end": 131
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 139,
        "end": 155
      },
      "phase": null,
      "attributes": [],
      "start": 113,
      "end": 156
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
              "name": "App2_component_3yveMqbQ3Fs",
              "start": 170,
              "end": 196
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
                          "name": "signal",
                          "start": 215,
                          "end": 221
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 224,
                            "end": 233
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 224,
                          "end": 235
                        },
                        "start": 215,
                        "end": 235
                      }
                    ],
                    "start": 209,
                    "end": 236
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "computed",
                          "start": 247,
                          "end": 255
                        },
                        "init": {
                          "type": "BinaryExpression",
                          "left": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "signal",
                              "start": 258,
                              "end": 264
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "value",
                              "start": 265,
                              "end": 270
                            },
                            "optional": false,
                            "computed": false,
                            "start": 258,
                            "end": 270
                          },
                          "operator": "+",
                          "right": {
                            "type": "Literal",
                            "value": "foo",
                            "raw": "'foo'",
                            "start": 273,
                            "end": 278
                          },
                          "start": 258,
                          "end": 278
                        },
                        "start": 247,
                        "end": 278
                      }
                    ],
                    "start": 241,
                    "end": 279
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 305,
                        "end": 315
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 316,
                          "end": 325
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 327,
                          "end": 331
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 333,
                          "end": 337
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 363,
                                "end": 373
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 374,
                                  "end": 379
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 381,
                                  "end": 385
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 401,
                                        "end": 408
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "hola",
                                        "raw": "\"hola\"",
                                        "start": 410,
                                        "end": 416
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 401,
                                      "end": 416
                                    }
                                  ],
                                  "start": 387,
                                  "end": 426
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 428,
                                  "end": 432
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 434,
                                  "end": 435
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 437,
                                  "end": 441
                                }
                              ],
                              "optional": false,
                              "start": 363,
                              "end": 442
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 466,
                                "end": 476
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 477,
                                  "end": 482
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 484,
                                  "end": 488
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 504,
                                        "end": 511
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_wrapProp",
                                          "start": 513,
                                          "end": 522
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "signal",
                                            "start": 523,
                                            "end": 529
                                          }
                                        ],
                                        "optional": false,
                                        "start": 513,
                                        "end": 530
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 504,
                                      "end": 530
                                    }
                                  ],
                                  "start": 490,
                                  "end": 540
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 542,
                                  "end": 546
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 548,
                                  "end": 549
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 551,
                                  "end": 555
                                }
                              ],
                              "optional": false,
                              "start": 466,
                              "end": 556
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 580,
                                "end": 590
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 591,
                                  "end": 596
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 598,
                                  "end": 602
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 618,
                                        "end": 625
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "signal",
                                        "start": 627,
                                        "end": 633
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 618,
                                      "end": 633
                                    }
                                  ],
                                  "start": 604,
                                  "end": 643
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 645,
                                  "end": 649
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 651,
                                  "end": 652
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 654,
                                  "end": 658
                                }
                              ],
                              "optional": false,
                              "start": 580,
                              "end": 659
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 683,
                                "end": 693
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 694,
                                  "end": 699
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 715,
                                        "end": 722
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "computed",
                                        "start": 724,
                                        "end": 732
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 715,
                                      "end": 732
                                    }
                                  ],
                                  "start": 701,
                                  "end": 742
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 744,
                                  "end": 748
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 750,
                                  "end": 754
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 756,
                                  "end": 757
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 759,
                                  "end": 763
                                }
                              ],
                              "optional": false,
                              "start": 683,
                              "end": 764
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 788,
                                "end": 798
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Foo",
                                  "start": 799,
                                  "end": 802
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 804,
                                  "end": 808
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "className",
                                        "start": 824,
                                        "end": 833
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "hola",
                                        "raw": "\"hola\"",
                                        "start": 835,
                                        "end": 841
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 824,
                                      "end": 841
                                    }
                                  ],
                                  "start": 810,
                                  "end": 851
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 853,
                                  "end": 857
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 859,
                                  "end": 860
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_0",
                                  "raw": "\"u6_0\"",
                                  "start": 862,
                                  "end": 868
                                }
                              ],
                              "optional": false,
                              "start": 788,
                              "end": 869
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 893,
                                "end": 903
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Foo",
                                  "start": 904,
                                  "end": 907
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 909,
                                  "end": 913
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "className",
                                        "start": 929,
                                        "end": 938
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_wrapProp",
                                          "start": 940,
                                          "end": 949
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "signal",
                                            "start": 950,
                                            "end": 956
                                          }
                                        ],
                                        "optional": false,
                                        "start": 940,
                                        "end": 957
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 929,
                                      "end": 957
                                    }
                                  ],
                                  "start": 915,
                                  "end": 967
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 969,
                                  "end": 973
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 975,
                                  "end": 976
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_1",
                                  "raw": "\"u6_1\"",
                                  "start": 978,
                                  "end": 984
                                }
                              ],
                              "optional": false,
                              "start": 893,
                              "end": 985
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1009,
                                "end": 1019
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Foo",
                                  "start": 1020,
                                  "end": 1023
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1025,
                                  "end": 1029
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "className",
                                        "start": 1045,
                                        "end": 1054
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "signal",
                                        "start": 1056,
                                        "end": 1062
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1045,
                                      "end": 1062
                                    }
                                  ],
                                  "start": 1031,
                                  "end": 1072
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1074,
                                  "end": 1078
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1080,
                                  "end": 1081
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_2",
                                  "raw": "\"u6_2\"",
                                  "start": 1083,
                                  "end": 1089
                                }
                              ],
                              "optional": false,
                              "start": 1009,
                              "end": 1090
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1114,
                                "end": 1124
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Foo",
                                  "start": 1125,
                                  "end": 1128
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "className",
                                        "start": 1144,
                                        "end": 1153
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "name": "computed",
                                        "start": 1155,
                                        "end": 1163
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1144,
                                      "end": 1163
                                    }
                                  ],
                                  "start": 1130,
                                  "end": 1173
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1175,
                                  "end": 1179
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1181,
                                  "end": 1185
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1187,
                                  "end": 1188
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_3",
                                  "raw": "\"u6_3\"",
                                  "start": 1190,
                                  "end": 1196
                                }
                              ],
                              "optional": false,
                              "start": 1114,
                              "end": 1197
                            }
                          ],
                          "start": 339,
                          "end": 1203
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1205,
                          "end": 1206
                        },
                        {
                          "type": "Literal",
                          "value": "u6_4",
                          "raw": "\"u6_4\"",
                          "start": 1208,
                          "end": 1214
                        }
                      ],
                      "optional": false,
                      "start": 305,
                      "end": 1215
                    },
                    "start": 284,
                    "end": 1216
                  }
                ],
                "start": 203,
                "end": 1218
              },
              "id": null,
              "generator": false,
              "start": 199,
              "end": 1218
            },
            "start": 170,
            "end": 1218
          }
        ],
        "start": 164,
        "end": 1219
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 157,
      "end": 1219
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1219
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App2_component_3yveMqbQ3Fs",
  "entry": null,
  "displayName": "test.tsx_App2_component",
  "hash": "3yveMqbQ3Fs",
  "canonicalFilename": "test.tsx_App2_component_3yveMqbQ3Fs",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    79,
    467
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_wrapProp()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 11 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_App2_component_3yveMqbQ3Fs.js` | `@qwik.dev/core` | 10 |
| `_wrapProp` | `test.tsx_App2_component_3yveMqbQ3Fs.js` | `@qwik.dev/core` | 3 |
| `useSignal` | `test.tsx_App2_component_3yveMqbQ3Fs.js` | `-` | 1 |

## Diagnostics

```json
[]
```
