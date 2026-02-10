# Test: example_jsx

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { $, component$, h, Fragment } from '@qwik.dev/core';

export const Lightweight = (props) => {
	return (
		<div>
			<>
				<div/>
				<button {...props}/>
			</>
		</div>
	)
};

export const Foo = component$((props) => {
	return $(() => {
		return (
			<div>
				<>
					<div class="class"/>
					<div class="class"></div>
					<div class="class">12</div>
				</>
				<div class="class">
					<Lightweight {...props}/>
				</div>
				<div class="class">
					<div/>
					<div/>
					<div/>
				</div>
				<div class="class">
					{children}
				</div>
			</div>
		)
	});
}, {
	tagName: "my-foo",
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
            "name": "h",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 25
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "h",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 25
          },
          "importKind": "value",
          "start": 24,
          "end": 25
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "Fragment",
            "optional": false,
            "typeAnnotation": null,
            "start": 27,
            "end": 35
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Fragment",
            "optional": false,
            "typeAnnotation": null,
            "start": 27,
            "end": 35
          },
          "importKind": "value",
          "start": 27,
          "end": 35
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 43,
        "end": 59
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 60
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
              "name": "Lightweight",
              "optional": false,
              "typeAnnotation": null,
              "start": 75,
              "end": 86
            },
            "init": {
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
                  "start": 90,
                  "end": 95
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
                            "start": 115,
                            "end": 118
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 114,
                          "end": 119
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "\n\t\t\t",
                            "raw": "\n\t\t\t",
                            "start": 119,
                            "end": 123
                          },
                          {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 123,
                              "end": 125
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t",
                                "raw": "\n\t\t\t\t",
                                "start": 125,
                                "end": 130
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 131,
                                    "end": 134
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": true,
                                  "start": 130,
                                  "end": 136
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 130,
                                "end": 136
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t\t",
                                "raw": "\n\t\t\t\t",
                                "start": 136,
                                "end": 141
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "button",
                                    "start": 142,
                                    "end": 148
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
                                        "start": 153,
                                        "end": 158
                                      },
                                      "start": 149,
                                      "end": 159
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 141,
                                  "end": 161
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 141,
                                "end": 161
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 161,
                                "end": 165
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 165,
                              "end": 168
                            },
                            "start": 123,
                            "end": 168
                          },
                          {
                            "type": "JSXText",
                            "value": "\n\t\t",
                            "raw": "\n\t\t",
                            "start": 168,
                            "end": 171
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 173,
                            "end": 176
                          },
                          "start": 171,
                          "end": 177
                        },
                        "start": 114,
                        "end": 177
                      },
                      "start": 110,
                      "end": 180
                    },
                    "start": 103,
                    "end": 180
                  }
                ],
                "start": 100,
                "end": 182
              },
              "id": null,
              "generator": false,
              "start": 89,
              "end": 182
            },
            "definite": false,
            "start": 75,
            "end": 182
          }
        ],
        "declare": false,
        "start": 69,
        "end": 183
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 62,
      "end": 183
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
              "name": "Foo",
              "optional": false,
              "typeAnnotation": null,
              "start": 198,
              "end": 201
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 204,
                "end": 214
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
                      "start": 216,
                      "end": 221
                    }
                  ],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
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
                            "start": 236,
                            "end": 237
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
                                            "start": 261,
                                            "end": 264
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 260,
                                          "end": 265
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 265,
                                            "end": 270
                                          },
                                          {
                                            "type": "JSXFragment",
                                            "openingFragment": {
                                              "type": "JSXOpeningFragment",
                                              "start": 270,
                                              "end": 272
                                            },
                                            "children": [
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 272,
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
                                                        "name": "class",
                                                        "start": 283,
                                                        "end": 288
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "class",
                                                        "raw": "\"class\"",
                                                        "start": 289,
                                                        "end": 296
                                                      },
                                                      "start": 283,
                                                      "end": 296
                                                    }
                                                  ],
                                                  "selfClosing": true,
                                                  "start": 278,
                                                  "end": 298
                                                },
                                                "children": [],
                                                "closingElement": null,
                                                "start": 278,
                                                "end": 298
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 298,
                                                "end": 304
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 305,
                                                    "end": 308
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [
                                                    {
                                                      "type": "JSXAttribute",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "class",
                                                        "start": 309,
                                                        "end": 314
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "class",
                                                        "raw": "\"class\"",
                                                        "start": 315,
                                                        "end": 322
                                                      },
                                                      "start": 309,
                                                      "end": 322
                                                    }
                                                  ],
                                                  "selfClosing": false,
                                                  "start": 304,
                                                  "end": 323
                                                },
                                                "children": [],
                                                "closingElement": {
                                                  "type": "JSXClosingElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 325,
                                                    "end": 328
                                                  },
                                                  "start": 323,
                                                  "end": 329
                                                },
                                                "start": 304,
                                                "end": 329
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 329,
                                                "end": 335
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 336,
                                                    "end": 339
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [
                                                    {
                                                      "type": "JSXAttribute",
                                                      "name": {
                                                        "type": "JSXIdentifier",
                                                        "name": "class",
                                                        "start": 340,
                                                        "end": 345
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "class",
                                                        "raw": "\"class\"",
                                                        "start": 346,
                                                        "end": 353
                                                      },
                                                      "start": 340,
                                                      "end": 353
                                                    }
                                                  ],
                                                  "selfClosing": false,
                                                  "start": 335,
                                                  "end": 354
                                                },
                                                "children": [
                                                  {
                                                    "type": "JSXText",
                                                    "value": "12",
                                                    "raw": "12",
                                                    "start": 354,
                                                    "end": 356
                                                  }
                                                ],
                                                "closingElement": {
                                                  "type": "JSXClosingElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 358,
                                                    "end": 361
                                                  },
                                                  "start": 356,
                                                  "end": 362
                                                },
                                                "start": 335,
                                                "end": 362
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t",
                                                "raw": "\n\t\t\t\t",
                                                "start": 362,
                                                "end": 367
                                              }
                                            ],
                                            "closingFragment": {
                                              "type": "JSXClosingFragment",
                                              "start": 367,
                                              "end": 370
                                            },
                                            "start": 270,
                                            "end": 370
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 370,
                                            "end": 375
                                          },
                                          {
                                            "type": "JSXElement",
                                            "openingElement": {
                                              "type": "JSXOpeningElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 376,
                                                "end": 379
                                              },
                                              "typeArguments": null,
                                              "attributes": [
                                                {
                                                  "type": "JSXAttribute",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "class",
                                                    "start": 380,
                                                    "end": 385
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "class",
                                                    "raw": "\"class\"",
                                                    "start": 386,
                                                    "end": 393
                                                  },
                                                  "start": 380,
                                                  "end": 393
                                                }
                                              ],
                                              "selfClosing": false,
                                              "start": 375,
                                              "end": 394
                                            },
                                            "children": [
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 394,
                                                "end": 400
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "Lightweight",
                                                    "start": 401,
                                                    "end": 412
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
                                                        "start": 417,
                                                        "end": 422
                                                      },
                                                      "start": 413,
                                                      "end": 423
                                                    }
                                                  ],
                                                  "selfClosing": true,
                                                  "start": 400,
                                                  "end": 425
                                                },
                                                "children": [],
                                                "closingElement": null,
                                                "start": 400,
                                                "end": 425
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t",
                                                "raw": "\n\t\t\t\t",
                                                "start": 425,
                                                "end": 430
                                              }
                                            ],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 432,
                                                "end": 435
                                              },
                                              "start": 430,
                                              "end": 436
                                            },
                                            "start": 375,
                                            "end": 436
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 436,
                                            "end": 441
                                          },
                                          {
                                            "type": "JSXElement",
                                            "openingElement": {
                                              "type": "JSXOpeningElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 442,
                                                "end": 445
                                              },
                                              "typeArguments": null,
                                              "attributes": [
                                                {
                                                  "type": "JSXAttribute",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "class",
                                                    "start": 446,
                                                    "end": 451
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "class",
                                                    "raw": "\"class\"",
                                                    "start": 452,
                                                    "end": 459
                                                  },
                                                  "start": 446,
                                                  "end": 459
                                                }
                                              ],
                                              "selfClosing": false,
                                              "start": 441,
                                              "end": 460
                                            },
                                            "children": [
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 460,
                                                "end": 466
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 467,
                                                    "end": 470
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [],
                                                  "selfClosing": true,
                                                  "start": 466,
                                                  "end": 472
                                                },
                                                "children": [],
                                                "closingElement": null,
                                                "start": 466,
                                                "end": 472
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 472,
                                                "end": 478
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 479,
                                                    "end": 482
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [],
                                                  "selfClosing": true,
                                                  "start": 478,
                                                  "end": 484
                                                },
                                                "children": [],
                                                "closingElement": null,
                                                "start": 478,
                                                "end": 484
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 484,
                                                "end": 490
                                              },
                                              {
                                                "type": "JSXElement",
                                                "openingElement": {
                                                  "type": "JSXOpeningElement",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "div",
                                                    "start": 491,
                                                    "end": 494
                                                  },
                                                  "typeArguments": null,
                                                  "attributes": [],
                                                  "selfClosing": true,
                                                  "start": 490,
                                                  "end": 496
                                                },
                                                "children": [],
                                                "closingElement": null,
                                                "start": 490,
                                                "end": 496
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t",
                                                "raw": "\n\t\t\t\t",
                                                "start": 496,
                                                "end": 501
                                              }
                                            ],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 503,
                                                "end": 506
                                              },
                                              "start": 501,
                                              "end": 507
                                            },
                                            "start": 441,
                                            "end": 507
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 507,
                                            "end": 512
                                          },
                                          {
                                            "type": "JSXElement",
                                            "openingElement": {
                                              "type": "JSXOpeningElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 513,
                                                "end": 516
                                              },
                                              "typeArguments": null,
                                              "attributes": [
                                                {
                                                  "type": "JSXAttribute",
                                                  "name": {
                                                    "type": "JSXIdentifier",
                                                    "name": "class",
                                                    "start": 517,
                                                    "end": 522
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "class",
                                                    "raw": "\"class\"",
                                                    "start": 523,
                                                    "end": 530
                                                  },
                                                  "start": 517,
                                                  "end": 530
                                                }
                                              ],
                                              "selfClosing": false,
                                              "start": 512,
                                              "end": 531
                                            },
                                            "children": [
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t\t",
                                                "raw": "\n\t\t\t\t\t",
                                                "start": 531,
                                                "end": 537
                                              },
                                              {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "children",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 538,
                                                  "end": 546
                                                },
                                                "start": 537,
                                                "end": 547
                                              },
                                              {
                                                "type": "JSXText",
                                                "value": "\n\t\t\t\t",
                                                "raw": "\n\t\t\t\t",
                                                "start": 547,
                                                "end": 552
                                              }
                                            ],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 554,
                                                "end": 557
                                              },
                                              "start": 552,
                                              "end": 558
                                            },
                                            "start": 512,
                                            "end": 558
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t",
                                            "raw": "\n\t\t\t",
                                            "start": 558,
                                            "end": 562
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 564,
                                            "end": 567
                                          },
                                          "start": 562,
                                          "end": 568
                                        },
                                        "start": 260,
                                        "end": 568
                                      },
                                      "start": 255,
                                      "end": 572
                                    },
                                    "start": 248,
                                    "end": 572
                                  }
                                ],
                                "start": 244,
                                "end": 575
                              },
                              "id": null,
                              "generator": false,
                              "start": 238,
                              "end": 575
                            }
                          ],
                          "optional": false,
                          "start": 236,
                          "end": 576
                        },
                        "start": 229,
                        "end": 577
                      }
                    ],
                    "start": 226,
                    "end": 579
                  },
                  "id": null,
                  "generator": false,
                  "start": 215,
                  "end": 579
                },
                {
                  "type": "ObjectExpression",
                  "properties": [
                    {
                      "type": "Property",
                      "kind": "init",
                      "key": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "tagName",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 584,
                        "end": 591
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 593,
                        "end": 601
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 584,
                      "end": 601
                    }
                  ],
                  "start": 581,
                  "end": 604
                }
              ],
              "optional": false,
              "start": 204,
              "end": 605
            },
            "definite": false,
            "start": 198,
            "end": 605
          }
        ],
        "declare": false,
        "start": 192,
        "end": 606
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 185,
      "end": 606
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 606
}
```

</details>

## Output

### Module: `test.js`

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
export const Lightweight = (props)=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null),
        /*#__PURE__*/ _jsxSplit("button", {
            ..._getVarProps(props)
        }, _getConstProps(props), null, 0, null)
    ], 1, "u6_0"), 1, "u6_1");
};
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"), {
    tagName: "my-foo"
});
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
            "name": "_getVarProps",
            "start": 54,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 54,
            "end": 66
          },
          "start": 54,
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
      "start": 45,
      "end": 91
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 101,
            "end": 115
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 101,
            "end": 115
          },
          "start": 101,
          "end": 115
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 123,
        "end": 139
      },
      "phase": null,
      "attributes": [],
      "start": 92,
      "end": 140
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 150,
            "end": 159
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 150,
            "end": 159
          },
          "start": 150,
          "end": 159
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 167,
        "end": 183
      },
      "phase": null,
      "attributes": [],
      "start": 141,
      "end": 184
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 194,
            "end": 206
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 194,
            "end": 206
          },
          "start": 194,
          "end": 206
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 214,
        "end": 230
      },
      "phase": null,
      "attributes": [],
      "start": 185,
      "end": 231
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 241,
            "end": 244
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 241,
            "end": 244
          },
          "start": 241,
          "end": 244
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 252,
        "end": 268
      },
      "phase": null,
      "attributes": [],
      "start": 232,
      "end": 269
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_HTDRsvUbLiE",
            "start": 276,
            "end": 289
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
                "value": "./test.tsx_Foo_component_HTDRsvUbLiE",
                "raw": "\"./test.tsx_Foo_component_HTDRsvUbLiE\"",
                "start": 303,
                "end": 341
              },
              "options": null,
              "phase": null,
              "start": 296,
              "end": 342
            },
            "id": null,
            "generator": false,
            "start": 292,
            "end": 342
          },
          "start": 276,
          "end": 342
        }
      ],
      "start": 270,
      "end": 343
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 353,
            "end": 361
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 365,
            "end": 374
          },
          "start": 353,
          "end": 374
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 382,
        "end": 410
      },
      "phase": null,
      "attributes": [],
      "start": 344,
      "end": 411
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
              "name": "Lightweight",
              "start": 425,
              "end": 436
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 440,
                  "end": 445
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
                        "name": "_jsxSorted",
                        "start": 475,
                        "end": 485
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 486,
                          "end": 491
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 493,
                          "end": 497
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 499,
                          "end": 503
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_jsxSorted",
                            "start": 519,
                            "end": 529
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "_Fragment",
                              "start": 530,
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
                              "value": null,
                              "raw": "null",
                              "start": 547,
                              "end": 551
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 577,
                                    "end": 587
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "div",
                                      "raw": "\"div\"",
                                      "start": 588,
                                      "end": 593
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 595,
                                      "end": 599
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 601,
                                      "end": 605
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 607,
                                      "end": 611
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 613,
                                      "end": 614
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 616,
                                      "end": 620
                                    }
                                  ],
                                  "optional": false,
                                  "start": 577,
                                  "end": 621
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSplit",
                                    "start": 645,
                                    "end": 654
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "button",
                                      "raw": "\"button\"",
                                      "start": 655,
                                      "end": 663
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
                                              "start": 682,
                                              "end": 694
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "props",
                                                "start": 695,
                                                "end": 700
                                              }
                                            ],
                                            "optional": false,
                                            "start": 682,
                                            "end": 701
                                          },
                                          "start": 679,
                                          "end": 701
                                        }
                                      ],
                                      "start": 665,
                                      "end": 711
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_getConstProps",
                                        "start": 713,
                                        "end": 727
                                      },
                                      "arguments": [
                                        {
                                          "type": "Identifier",
                                          "name": "props",
                                          "start": 728,
                                          "end": 733
                                        }
                                      ],
                                      "optional": false,
                                      "start": 713,
                                      "end": 734
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 736,
                                      "end": 740
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 0,
                                      "raw": "0",
                                      "start": 742,
                                      "end": 743
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 745,
                                      "end": 749
                                    }
                                  ],
                                  "optional": false,
                                  "start": 645,
                                  "end": 750
                                }
                              ],
                              "start": 553,
                              "end": 756
                            },
                            {
                              "type": "Literal",
                              "value": 1,
                              "raw": "1",
                              "start": 758,
                              "end": 759
                            },
                            {
                              "type": "Literal",
                              "value": "u6_0",
                              "raw": "\"u6_0\"",
                              "start": 761,
                              "end": 767
                            }
                          ],
                          "optional": false,
                          "start": 519,
                          "end": 768
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 770,
                          "end": 771
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 773,
                          "end": 779
                        }
                      ],
                      "optional": false,
                      "start": 475,
                      "end": 780
                    },
                    "start": 454,
                    "end": 781
                  }
                ],
                "start": 448,
                "end": 783
              },
              "id": null,
              "generator": false,
              "start": 439,
              "end": 783
            },
            "start": 425,
            "end": 783
          }
        ],
        "start": 419,
        "end": 784
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 412,
      "end": 784
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
              "name": "Foo",
              "start": 798,
              "end": 801
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 818,
                "end": 830
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 845,
                    "end": 848
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_HTDRsvUbLiE",
                      "start": 849,
                      "end": 862
                    },
                    {
                      "type": "Literal",
                      "value": "Foo_component_HTDRsvUbLiE",
                      "raw": "\"Foo_component_HTDRsvUbLiE\"",
                      "start": 864,
                      "end": 891
                    }
                  ],
                  "optional": false,
                  "start": 845,
                  "end": 892
                },
                {
                  "type": "ObjectExpression",
                  "properties": [
                    {
                      "type": "Property",
                      "kind": "init",
                      "key": {
                        "type": "Identifier",
                        "name": "tagName",
                        "start": 900,
                        "end": 907
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 909,
                        "end": 917
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "start": 900,
                      "end": 917
                    }
                  ],
                  "start": 894,
                  "end": 919
                }
              ],
              "optional": false,
              "start": 818,
              "end": 920
            },
            "start": 798,
            "end": 920
          }
        ],
        "start": 792,
        "end": 921
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 785,
      "end": 921
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 921
}
```

</details>

### Module: `test.tsx_Foo_component_HTDRsvUbLiE.js` (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_DvU6FitWglY = ()=>import("./test.tsx_Foo_component_1_DvU6FitWglY");
export const Foo_component_HTDRsvUbLiE = (props)=>{
    return /*#__PURE__*/ qrl(i_DvU6FitWglY, "Foo_component_1_DvU6FitWglY", [
        props
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
            "name": "i_DvU6FitWglY",
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
                "value": "./test.tsx_Foo_component_1_DvU6FitWglY",
                "raw": "\"./test.tsx_Foo_component_1_DvU6FitWglY\"",
                "start": 71,
                "end": 111
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 112
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 112
          },
          "start": 44,
          "end": 112
        }
      ],
      "start": 38,
      "end": 113
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
              "name": "Foo_component_HTDRsvUbLiE",
              "start": 127,
              "end": 152
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 156,
                  "end": 161
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
                        "name": "qrl",
                        "start": 191,
                        "end": 194
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_DvU6FitWglY",
                          "start": 195,
                          "end": 208
                        },
                        {
                          "type": "Literal",
                          "value": "Foo_component_1_DvU6FitWglY",
                          "raw": "\"Foo_component_1_DvU6FitWglY\"",
                          "start": 210,
                          "end": 239
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "props",
                              "start": 251,
                              "end": 256
                            }
                          ],
                          "start": 241,
                          "end": 262
                        }
                      ],
                      "optional": false,
                      "start": 191,
                      "end": 263
                    },
                    "start": 170,
                    "end": 264
                  }
                ],
                "start": 164,
                "end": 266
              },
              "id": null,
              "generator": false,
              "start": 155,
              "end": 266
            },
            "start": 127,
            "end": 266
          }
        ],
        "start": 121,
        "end": 267
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 114,
      "end": 267
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 267
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_HTDRsvUbLiE",
  "entry": null,
  "displayName": "test.tsx_Foo_component",
  "hash": "HTDRsvUbLiE",
  "canonicalFilename": "test.tsx_Foo_component_HTDRsvUbLiE",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    217,
    581
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: `test.tsx_Foo_component_1_DvU6FitWglY.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { Lightweight } from "./test";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
export const Foo_component_1_DvU6FitWglY = ()=>{
    const props = _captures[0];
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
            /*#__PURE__*/ _jsxSorted("div", null, {
                class: "class"
            }, null, 3, null),
            /*#__PURE__*/ _jsxSorted("div", null, {
                class: "class"
            }, null, 3, null),
            /*#__PURE__*/ _jsxSorted("div", null, {
                class: "class"
            }, "12", 3, null)
        ], 3, "u6_2"),
        /*#__PURE__*/ _jsxSorted("div", null, {
            class: "class"
        }, /*#__PURE__*/ _jsxSplit(Lightweight, {
            ..._getVarProps(props)
        }, _getConstProps(props), null, 0, "u6_3"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            class: "class"
        }, [
            /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null),
            /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null),
            /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null)
        ], 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, {
            class: "class"
        }, children, 1, null)
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
            "name": "Lightweight",
            "start": 53,
            "end": 64
          },
          "local": {
            "type": "Identifier",
            "name": "Lightweight",
            "start": 53,
            "end": 64
          },
          "start": 53,
          "end": 64
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 72,
        "end": 80
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 81
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 91,
            "end": 99
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 103,
            "end": 112
          },
          "start": 91,
          "end": 112
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 120,
        "end": 148
      },
      "phase": null,
      "attributes": [],
      "start": 82,
      "end": 149
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 159,
            "end": 173
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 159,
            "end": 173
          },
          "start": 159,
          "end": 173
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 181,
        "end": 197
      },
      "phase": null,
      "attributes": [],
      "start": 150,
      "end": 198
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 208,
            "end": 220
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 208,
            "end": 220
          },
          "start": 208,
          "end": 220
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 228,
        "end": 244
      },
      "phase": null,
      "attributes": [],
      "start": 199,
      "end": 245
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 255,
            "end": 265
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 255,
            "end": 265
          },
          "start": 255,
          "end": 265
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 273,
        "end": 289
      },
      "phase": null,
      "attributes": [],
      "start": 246,
      "end": 290
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 300,
            "end": 309
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 300,
            "end": 309
          },
          "start": 300,
          "end": 309
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 317,
        "end": 333
      },
      "phase": null,
      "attributes": [],
      "start": 291,
      "end": 334
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
              "name": "Foo_component_1_DvU6FitWglY",
              "start": 348,
              "end": 375
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
                          "start": 394,
                          "end": 399
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 402,
                            "end": 411
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 412,
                            "end": 413
                          },
                          "optional": false,
                          "computed": true,
                          "start": 402,
                          "end": 414
                        },
                        "start": 394,
                        "end": 414
                      }
                    ],
                    "start": 388,
                    "end": 415
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 441,
                        "end": 451
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 452,
                          "end": 457
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 459,
                          "end": 463
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 465,
                          "end": 469
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 495,
                                "end": 505
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "_Fragment",
                                  "start": 506,
                                  "end": 515
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 517,
                                  "end": 521
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 523,
                                  "end": 527
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 557,
                                        "end": 567
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 568,
                                          "end": 573
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 575,
                                          "end": 579
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
                                                "start": 599,
                                                "end": 604
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": "class",
                                                "raw": "\"class\"",
                                                "start": 606,
                                                "end": 613
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 599,
                                              "end": 613
                                            }
                                          ],
                                          "start": 581,
                                          "end": 627
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 629,
                                          "end": 633
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 635,
                                          "end": 636
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 638,
                                          "end": 642
                                        }
                                      ],
                                      "optional": false,
                                      "start": 557,
                                      "end": 643
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 671,
                                        "end": 681
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 682,
                                          "end": 687
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 689,
                                          "end": 693
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
                                                "start": 713,
                                                "end": 718
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": "class",
                                                "raw": "\"class\"",
                                                "start": 720,
                                                "end": 727
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 713,
                                              "end": 727
                                            }
                                          ],
                                          "start": 695,
                                          "end": 741
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 743,
                                          "end": 747
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 749,
                                          "end": 750
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 752,
                                          "end": 756
                                        }
                                      ],
                                      "optional": false,
                                      "start": 671,
                                      "end": 757
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 785,
                                        "end": 795
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 796,
                                          "end": 801
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 803,
                                          "end": 807
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
                                                "start": 827,
                                                "end": 832
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": "class",
                                                "raw": "\"class\"",
                                                "start": 834,
                                                "end": 841
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 827,
                                              "end": 841
                                            }
                                          ],
                                          "start": 809,
                                          "end": 855
                                        },
                                        {
                                          "type": "Literal",
                                          "value": "12",
                                          "raw": "\"12\"",
                                          "start": 857,
                                          "end": 861
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 863,
                                          "end": 864
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 866,
                                          "end": 870
                                        }
                                      ],
                                      "optional": false,
                                      "start": 785,
                                      "end": 871
                                    }
                                  ],
                                  "start": 529,
                                  "end": 881
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 883,
                                  "end": 884
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_2",
                                  "raw": "\"u6_2\"",
                                  "start": 886,
                                  "end": 892
                                }
                              ],
                              "optional": false,
                              "start": 495,
                              "end": 893
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 917,
                                "end": 927
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 928,
                                  "end": 933
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 935,
                                  "end": 939
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
                                        "start": 955,
                                        "end": 960
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 962,
                                        "end": 969
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 955,
                                      "end": 969
                                    }
                                  ],
                                  "start": 941,
                                  "end": 979
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSplit",
                                    "start": 995,
                                    "end": 1004
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "Lightweight",
                                      "start": 1005,
                                      "end": 1016
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
                                              "start": 1035,
                                              "end": 1047
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "props",
                                                "start": 1048,
                                                "end": 1053
                                              }
                                            ],
                                            "optional": false,
                                            "start": 1035,
                                            "end": 1054
                                          },
                                          "start": 1032,
                                          "end": 1054
                                        }
                                      ],
                                      "start": 1018,
                                      "end": 1064
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_getConstProps",
                                        "start": 1066,
                                        "end": 1080
                                      },
                                      "arguments": [
                                        {
                                          "type": "Identifier",
                                          "name": "props",
                                          "start": 1081,
                                          "end": 1086
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1066,
                                      "end": 1087
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 1089,
                                      "end": 1093
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 0,
                                      "raw": "0",
                                      "start": 1095,
                                      "end": 1096
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_3",
                                      "raw": "\"u6_3\"",
                                      "start": 1098,
                                      "end": 1104
                                    }
                                  ],
                                  "optional": false,
                                  "start": 995,
                                  "end": 1105
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 1107,
                                  "end": 1108
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1110,
                                  "end": 1114
                                }
                              ],
                              "optional": false,
                              "start": 917,
                              "end": 1115
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1139,
                                "end": 1149
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 1150,
                                  "end": 1155
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1157,
                                  "end": 1161
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
                                        "start": 1177,
                                        "end": 1182
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 1184,
                                        "end": 1191
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1177,
                                      "end": 1191
                                    }
                                  ],
                                  "start": 1163,
                                  "end": 1201
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1231,
                                        "end": 1241
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 1242,
                                          "end": 1247
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1249,
                                          "end": 1253
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1255,
                                          "end": 1259
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1261,
                                          "end": 1265
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1267,
                                          "end": 1268
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1270,
                                          "end": 1274
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1231,
                                      "end": 1275
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1303,
                                        "end": 1313
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 1314,
                                          "end": 1319
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1321,
                                          "end": 1325
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1327,
                                          "end": 1331
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1333,
                                          "end": 1337
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1339,
                                          "end": 1340
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1342,
                                          "end": 1346
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1303,
                                      "end": 1347
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1375,
                                        "end": 1385
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 1386,
                                          "end": 1391
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1393,
                                          "end": 1397
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1399,
                                          "end": 1403
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1405,
                                          "end": 1409
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1411,
                                          "end": 1412
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1414,
                                          "end": 1418
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1375,
                                      "end": 1419
                                    }
                                  ],
                                  "start": 1203,
                                  "end": 1429
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1431,
                                  "end": 1432
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1434,
                                  "end": 1438
                                }
                              ],
                              "optional": false,
                              "start": 1139,
                              "end": 1439
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1463,
                                "end": 1473
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 1474,
                                  "end": 1479
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1481,
                                  "end": 1485
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
                                        "start": 1501,
                                        "end": 1506
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "class",
                                        "raw": "\"class\"",
                                        "start": 1508,
                                        "end": 1515
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1501,
                                      "end": 1515
                                    }
                                  ],
                                  "start": 1487,
                                  "end": 1525
                                },
                                {
                                  "type": "Identifier",
                                  "name": "children",
                                  "start": 1527,
                                  "end": 1535
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 1537,
                                  "end": 1538
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1540,
                                  "end": 1544
                                }
                              ],
                              "optional": false,
                              "start": 1463,
                              "end": 1545
                            }
                          ],
                          "start": 471,
                          "end": 1551
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1553,
                          "end": 1554
                        },
                        {
                          "type": "Literal",
                          "value": "u6_4",
                          "raw": "\"u6_4\"",
                          "start": 1556,
                          "end": 1562
                        }
                      ],
                      "optional": false,
                      "start": 441,
                      "end": 1563
                    },
                    "start": 420,
                    "end": 1564
                  }
                ],
                "start": 382,
                "end": 1566
              },
              "id": null,
              "generator": false,
              "start": 378,
              "end": 1566
            },
            "start": 348,
            "end": 1566
          }
        ],
        "start": 342,
        "end": 1567
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 335,
      "end": 1567
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1567
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_1_DvU6FitWglY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_1",
  "hash": "DvU6FitWglY",
  "canonicalFilename": "test.tsx_Foo_component_1_DvU6FitWglY",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_HTDRsvUbLiE",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    240,
    577
  ],
  "captureNames": [
    "props"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`, `_jsxSplit()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_getVarProps()`, `_getConstProps()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (2 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 19 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.js` | `@qwik.dev/core` | 4 |
| `_jsxSplit` | `test.js` | `@qwik.dev/core` | 2 |
| `_getVarProps` | `test.js` | `@qwik.dev/core` | 2 |
| `_getConstProps` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 12 |
| `_jsxSplit` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 2 |
| `_getVarProps` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 2 |
| `_getConstProps` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 2 |
| `_captures` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
