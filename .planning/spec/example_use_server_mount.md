# Test: example_use_server_mount

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |
| Entry Strategy | Smart |

## Input

### Source Code

```tsx
import { component$, useTask$, useStore, useStyles$ } from '@qwik.dev/core';
import mongo from 'mongodb';
import redis from 'redis';

export const Parent = component$(() => {
	const state = useStore({
		text: ''
	});

	// Double count watch
	useTask$(async () => {
		state.text = await mongo.users();
		redis.set(state.text);
	});

	return (
		<div onClick$={() => console.log('parent')}>
			{state.text}
		</div>
	);
});

export const Child = component$(() => {
	const state = useStore({
		text: ''
	});

	// Double count watch
	useTask$(async () => {
		state.text = await mongo.users();
	});

	return (
		<div onClick$={() => console.log('child')}>
			{state.text}
		</div>
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
            "name": "useTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 29
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 29
          },
          "importKind": "value",
          "start": 21,
          "end": 29
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 39
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 39
          },
          "importKind": "value",
          "start": 31,
          "end": 39
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 41,
            "end": 51
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 41,
            "end": 51
          },
          "importKind": "value",
          "start": 41,
          "end": 51
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 59,
        "end": 75
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 76
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongo",
            "optional": false,
            "typeAnnotation": null,
            "start": 84,
            "end": 89
          },
          "start": 84,
          "end": 89
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mongodb",
        "raw": "'mongodb'",
        "start": 95,
        "end": 104
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 77,
      "end": 105
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "redis",
            "optional": false,
            "typeAnnotation": null,
            "start": 113,
            "end": 118
          },
          "start": 113,
          "end": 118
        }
      ],
      "source": {
        "type": "Literal",
        "value": "redis",
        "raw": "'redis'",
        "start": 124,
        "end": 131
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 106,
      "end": 132
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
              "name": "Parent",
              "optional": false,
              "typeAnnotation": null,
              "start": 147,
              "end": 153
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 156,
                "end": 166
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
                              "name": "state",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 182,
                              "end": 187
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 190,
                                "end": 198
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
                                        "name": "text",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 203,
                                        "end": 207
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "",
                                        "raw": "''",
                                        "start": 209,
                                        "end": 211
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 203,
                                      "end": 211
                                    }
                                  ],
                                  "start": 199,
                                  "end": 214
                                }
                              ],
                              "optional": false,
                              "start": 190,
                              "end": 215
                            },
                            "definite": false,
                            "start": 182,
                            "end": 215
                          }
                        ],
                        "declare": false,
                        "start": 176,
                        "end": 216
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useTask$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 242,
                            "end": 250
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": false,
                              "async": true,
                              "typeParameters": null,
                              "params": [],
                              "returnType": null,
                              "body": {
                                "type": "BlockStatement",
                                "body": [
                                  {
                                    "type": "ExpressionStatement",
                                    "expression": {
                                      "type": "AssignmentExpression",
                                      "operator": "=",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "state",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 267,
                                          "end": 272
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "text",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 273,
                                          "end": 277
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 267,
                                        "end": 277
                                      },
                                      "right": {
                                        "type": "AwaitExpression",
                                        "argument": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "mongo",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 286,
                                              "end": 291
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "users",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 292,
                                              "end": 297
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 286,
                                            "end": 297
                                          },
                                          "typeArguments": null,
                                          "arguments": [],
                                          "optional": false,
                                          "start": 286,
                                          "end": 299
                                        },
                                        "start": 280,
                                        "end": 299
                                      },
                                      "start": 267,
                                      "end": 299
                                    },
                                    "directive": null,
                                    "start": 267,
                                    "end": 300
                                  },
                                  {
                                    "type": "ExpressionStatement",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "redis",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 303,
                                          "end": 308
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "set",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 309,
                                          "end": 312
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 303,
                                        "end": 312
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "state",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 313,
                                            "end": 318
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "text",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 319,
                                            "end": 323
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 313,
                                          "end": 323
                                        }
                                      ],
                                      "optional": false,
                                      "start": 303,
                                      "end": 324
                                    },
                                    "directive": null,
                                    "start": 303,
                                    "end": 325
                                  }
                                ],
                                "start": 263,
                                "end": 328
                              },
                              "id": null,
                              "generator": false,
                              "start": 251,
                              "end": 328
                            }
                          ],
                          "optional": false,
                          "start": 242,
                          "end": 329
                        },
                        "directive": null,
                        "start": 242,
                        "end": 330
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
                                "start": 345,
                                "end": 348
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 349,
                                    "end": 357
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
                                            "name": "console",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 365,
                                            "end": 372
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "log",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 373,
                                            "end": 376
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 365,
                                          "end": 376
                                        },
                                        "typeArguments": null,
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "parent",
                                            "raw": "'parent'",
                                            "start": 377,
                                            "end": 385
                                          }
                                        ],
                                        "optional": false,
                                        "start": 365,
                                        "end": 386
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 359,
                                      "end": 386
                                    },
                                    "start": 358,
                                    "end": 387
                                  },
                                  "start": 349,
                                  "end": 387
                                }
                              ],
                              "selfClosing": false,
                              "start": 344,
                              "end": 388
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 388,
                                "end": 392
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "state",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 393,
                                    "end": 398
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "text",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 399,
                                    "end": 403
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 393,
                                  "end": 403
                                },
                                "start": 392,
                                "end": 404
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 404,
                                "end": 407
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 409,
                                "end": 412
                              },
                              "start": 407,
                              "end": 413
                            },
                            "start": 344,
                            "end": 413
                          },
                          "start": 340,
                          "end": 416
                        },
                        "start": 333,
                        "end": 417
                      }
                    ],
                    "start": 173,
                    "end": 419
                  },
                  "id": null,
                  "generator": false,
                  "start": 167,
                  "end": 419
                }
              ],
              "optional": false,
              "start": 156,
              "end": 420
            },
            "definite": false,
            "start": 147,
            "end": 420
          }
        ],
        "declare": false,
        "start": 141,
        "end": 421
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 134,
      "end": 421
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
              "name": "Child",
              "optional": false,
              "typeAnnotation": null,
              "start": 436,
              "end": 441
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 444,
                "end": 454
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
                              "name": "state",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 470,
                              "end": 475
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 478,
                                "end": 486
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
                                        "name": "text",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 491,
                                        "end": 495
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "",
                                        "raw": "''",
                                        "start": 497,
                                        "end": 499
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 491,
                                      "end": 499
                                    }
                                  ],
                                  "start": 487,
                                  "end": 502
                                }
                              ],
                              "optional": false,
                              "start": 478,
                              "end": 503
                            },
                            "definite": false,
                            "start": 470,
                            "end": 503
                          }
                        ],
                        "declare": false,
                        "start": 464,
                        "end": 504
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useTask$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 530,
                            "end": 538
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "ArrowFunctionExpression",
                              "expression": false,
                              "async": true,
                              "typeParameters": null,
                              "params": [],
                              "returnType": null,
                              "body": {
                                "type": "BlockStatement",
                                "body": [
                                  {
                                    "type": "ExpressionStatement",
                                    "expression": {
                                      "type": "AssignmentExpression",
                                      "operator": "=",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "state",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 555,
                                          "end": 560
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "text",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 561,
                                          "end": 565
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 555,
                                        "end": 565
                                      },
                                      "right": {
                                        "type": "AwaitExpression",
                                        "argument": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "mongo",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 574,
                                              "end": 579
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "users",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 580,
                                              "end": 585
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 574,
                                            "end": 585
                                          },
                                          "typeArguments": null,
                                          "arguments": [],
                                          "optional": false,
                                          "start": 574,
                                          "end": 587
                                        },
                                        "start": 568,
                                        "end": 587
                                      },
                                      "start": 555,
                                      "end": 587
                                    },
                                    "directive": null,
                                    "start": 555,
                                    "end": 588
                                  }
                                ],
                                "start": 551,
                                "end": 591
                              },
                              "id": null,
                              "generator": false,
                              "start": 539,
                              "end": 591
                            }
                          ],
                          "optional": false,
                          "start": 530,
                          "end": 592
                        },
                        "directive": null,
                        "start": 530,
                        "end": 593
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
                                "start": 608,
                                "end": 611
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 612,
                                    "end": 620
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
                                            "name": "console",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 628,
                                            "end": 635
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "log",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 636,
                                            "end": 639
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 628,
                                          "end": 639
                                        },
                                        "typeArguments": null,
                                        "arguments": [
                                          {
                                            "type": "Literal",
                                            "value": "child",
                                            "raw": "'child'",
                                            "start": 640,
                                            "end": 647
                                          }
                                        ],
                                        "optional": false,
                                        "start": 628,
                                        "end": 648
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 622,
                                      "end": 648
                                    },
                                    "start": 621,
                                    "end": 649
                                  },
                                  "start": 612,
                                  "end": 649
                                }
                              ],
                              "selfClosing": false,
                              "start": 607,
                              "end": 650
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 650,
                                "end": 654
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "state",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 655,
                                    "end": 660
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "text",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 661,
                                    "end": 665
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 655,
                                  "end": 665
                                },
                                "start": 654,
                                "end": 666
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 666,
                                "end": 669
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 671,
                                "end": 674
                              },
                              "start": 669,
                              "end": 675
                            },
                            "start": 607,
                            "end": 675
                          },
                          "start": 603,
                          "end": 678
                        },
                        "start": 596,
                        "end": 679
                      }
                    ],
                    "start": 461,
                    "end": 681
                  },
                  "id": null,
                  "generator": false,
                  "start": 455,
                  "end": 681
                }
              ],
              "optional": false,
              "start": 444,
              "end": 682
            },
            "definite": false,
            "start": 436,
            "end": 682
          }
        ],
        "declare": false,
        "start": 430,
        "end": 683
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 423,
      "end": 683
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 683
}
```

</details>

## Output

### Module: test.tsx_Parent_component_useTask_gDH1EtUWqBU.js

```javascript
import { _captures } from "@qwik.dev/core";
import mongo from "mongodb";
import redis from "redis";
export const Parent_component_useTask_gDH1EtUWqBU = async ()=>{
    const state = _captures[0];
    state.text = await mongo.users();
    redis.set(state.text);
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
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "mongo",
            "start": 51,
            "end": 56
          },
          "start": 51,
          "end": 56
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mongodb",
        "raw": "\"mongodb\"",
        "start": 62,
        "end": 71
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 72
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "redis",
            "start": 80,
            "end": 85
          },
          "start": 80,
          "end": 85
        }
      ],
      "source": {
        "type": "Literal",
        "value": "redis",
        "raw": "\"redis\"",
        "start": 91,
        "end": 98
      },
      "phase": null,
      "attributes": [],
      "start": 73,
      "end": 99
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
              "name": "Parent_component_useTask_gDH1EtUWqBU",
              "start": 113,
              "end": 149
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": true,
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
                          "name": "state",
                          "start": 174,
                          "end": 179
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 182,
                            "end": 191
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 192,
                            "end": 193
                          },
                          "optional": false,
                          "computed": true,
                          "start": 182,
                          "end": 194
                        },
                        "start": 174,
                        "end": 194
                      }
                    ],
                    "start": 168,
                    "end": 195
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 200,
                          "end": 205
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "text",
                          "start": 206,
                          "end": 210
                        },
                        "optional": false,
                        "computed": false,
                        "start": 200,
                        "end": 210
                      },
                      "right": {
                        "type": "AwaitExpression",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "mongo",
                              "start": 219,
                              "end": 224
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "users",
                              "start": 225,
                              "end": 230
                            },
                            "optional": false,
                            "computed": false,
                            "start": 219,
                            "end": 230
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 219,
                          "end": 232
                        },
                        "start": 213,
                        "end": 232
                      },
                      "start": 200,
                      "end": 232
                    },
                    "start": 200,
                    "end": 233
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "redis",
                          "start": 238,
                          "end": 243
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "set",
                          "start": 244,
                          "end": 247
                        },
                        "optional": false,
                        "computed": false,
                        "start": 238,
                        "end": 247
                      },
                      "arguments": [
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "state",
                            "start": 248,
                            "end": 253
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "text",
                            "start": 254,
                            "end": 258
                          },
                          "optional": false,
                          "computed": false,
                          "start": 248,
                          "end": 258
                        }
                      ],
                      "optional": false,
                      "start": 238,
                      "end": 259
                    },
                    "start": 238,
                    "end": 260
                  }
                ],
                "start": 162,
                "end": 262
              },
              "id": null,
              "generator": false,
              "start": 152,
              "end": 262
            },
            "start": 113,
            "end": 262
          }
        ],
        "start": 107,
        "end": 263
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 100,
      "end": 263
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 263
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_useTask_gDH1EtUWqBU",
  "entry": "test.tsx_entry_Parent",
  "displayName": "test.tsx_Parent_component_useTask",
  "hash": "gDH1EtUWqBU",
  "canonicalFilename": "test.tsx_Parent_component_useTask_gDH1EtUWqBU",
  "path": "",
  "extension": "js",
  "parent": "Parent_component_0TaiDayHrlo",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": true,
  "loc": [
    253,
    330
  ],
  "captureNames": [
    "state"
  ]
}
```

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_0TaiDayHrlo = ()=>import("./test.tsx_Parent_component_0TaiDayHrlo");
const i_9GyF01GDKqw = ()=>import("./test.tsx_Child_component_9GyF01GDKqw");
export const Parent = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_0TaiDayHrlo, "Parent_component_0TaiDayHrlo"));
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_9GyF01GDKqw, "Child_component_9GyF01GDKqw"));
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
            "name": "i_0TaiDayHrlo",
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
                "value": "./test.tsx_Parent_component_0TaiDayHrlo",
                "raw": "\"./test.tsx_Parent_component_0TaiDayHrlo\"",
                "start": 118,
                "end": 159
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 160
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 160
          },
          "start": 91,
          "end": 160
        }
      ],
      "start": 85,
      "end": 161
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_9GyF01GDKqw",
            "start": 168,
            "end": 181
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
                "value": "./test.tsx_Child_component_9GyF01GDKqw",
                "raw": "\"./test.tsx_Child_component_9GyF01GDKqw\"",
                "start": 195,
                "end": 235
              },
              "options": null,
              "phase": null,
              "start": 188,
              "end": 236
            },
            "id": null,
            "generator": false,
            "start": 184,
            "end": 236
          },
          "start": 168,
          "end": 236
        }
      ],
      "start": 162,
      "end": 237
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
              "name": "Parent",
              "start": 251,
              "end": 257
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 274,
                "end": 286
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 301,
                    "end": 304
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_0TaiDayHrlo",
                      "start": 305,
                      "end": 318
                    },
                    {
                      "type": "Literal",
                      "value": "Parent_component_0TaiDayHrlo",
                      "raw": "\"Parent_component_0TaiDayHrlo\"",
                      "start": 320,
                      "end": 350
                    }
                  ],
                  "optional": false,
                  "start": 301,
                  "end": 351
                }
              ],
              "optional": false,
              "start": 274,
              "end": 352
            },
            "start": 251,
            "end": 352
          }
        ],
        "start": 245,
        "end": 353
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 238,
      "end": 353
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
              "name": "Child",
              "start": 367,
              "end": 372
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 389,
                "end": 401
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 416,
                    "end": 419
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_9GyF01GDKqw",
                      "start": 420,
                      "end": 433
                    },
                    {
                      "type": "Literal",
                      "value": "Child_component_9GyF01GDKqw",
                      "raw": "\"Child_component_9GyF01GDKqw\"",
                      "start": 435,
                      "end": 464
                    }
                  ],
                  "optional": false,
                  "start": 416,
                  "end": 465
                }
              ],
              "optional": false,
              "start": 389,
              "end": 466
            },
            "start": 367,
            "end": 466
          }
        ],
        "start": 361,
        "end": 467
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 354,
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

### Module: test.tsx_Child_component_useTask_Oh4n7ZeqJkU.js

```javascript
import { _captures } from "@qwik.dev/core";
import mongo from "mongodb";
export const Child_component_useTask_Oh4n7ZeqJkU = async ()=>{
    const state = _captures[0];
    state.text = await mongo.users();
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
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "mongo",
            "start": 51,
            "end": 56
          },
          "start": 51,
          "end": 56
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mongodb",
        "raw": "\"mongodb\"",
        "start": 62,
        "end": 71
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 72
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
              "name": "Child_component_useTask_Oh4n7ZeqJkU",
              "start": 86,
              "end": 121
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": true,
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
                          "name": "state",
                          "start": 146,
                          "end": 151
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 154,
                            "end": 163
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 164,
                            "end": 165
                          },
                          "optional": false,
                          "computed": true,
                          "start": 154,
                          "end": 166
                        },
                        "start": 146,
                        "end": 166
                      }
                    ],
                    "start": 140,
                    "end": 167
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 172,
                          "end": 177
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "text",
                          "start": 178,
                          "end": 182
                        },
                        "optional": false,
                        "computed": false,
                        "start": 172,
                        "end": 182
                      },
                      "right": {
                        "type": "AwaitExpression",
                        "argument": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "mongo",
                              "start": 191,
                              "end": 196
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "users",
                              "start": 197,
                              "end": 202
                            },
                            "optional": false,
                            "computed": false,
                            "start": 191,
                            "end": 202
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 191,
                          "end": 204
                        },
                        "start": 185,
                        "end": 204
                      },
                      "start": 172,
                      "end": 204
                    },
                    "start": 172,
                    "end": 205
                  }
                ],
                "start": 134,
                "end": 207
              },
              "id": null,
              "generator": false,
              "start": 124,
              "end": 207
            },
            "start": 86,
            "end": 207
          }
        ],
        "start": 80,
        "end": 208
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 73,
      "end": 208
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 208
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Child_component_useTask_Oh4n7ZeqJkU",
  "entry": "test.tsx_entry_Child",
  "displayName": "test.tsx_Child_component_useTask",
  "hash": "Oh4n7ZeqJkU",
  "canonicalFilename": "test.tsx_Child_component_useTask_Oh4n7ZeqJkU",
  "path": "",
  "extension": "js",
  "parent": "Child_component_9GyF01GDKqw",
  "ctxKind": "function",
  "ctxName": "useTask$",
  "captures": true,
  "loc": [
    541,
    593
  ],
  "captureNames": [
    "state"
  ]
}
```

### Module: test.tsx_Parent_component_0TaiDayHrlo.js

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const i_gDH1EtUWqBU = ()=>import("./test.tsx_Parent_component_useTask_gDH1EtUWqBU");
const i_zM9okM0TYrA = ()=>import("./test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA");
export const Parent_component_0TaiDayHrlo = ()=>{
    const state = useStore({
        text: ''
    });
    // Double count watch
    useTaskQrl(/*#__PURE__*/ qrl(i_gDH1EtUWqBU, "Parent_component_useTask_gDH1EtUWqBU", [
        state
    ]));
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_zM9okM0TYrA, "Parent_component_div_q_e_click_zM9okM0TYrA")
    }, _wrapProp(state, "text"), 3, "u6_0");
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
            "name": "_wrapProp",
            "start": 54,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 54,
            "end": 63
          },
          "start": 54,
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
      "start": 45,
      "end": 88
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "start": 98,
          "end": 101
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 109,
        "end": 125
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 126
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "start": 136,
          "end": 144
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 152,
        "end": 168
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 169
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useTaskQrl",
            "start": 179,
            "end": 189
          },
          "local": {
            "type": "Identifier",
            "name": "useTaskQrl",
            "start": 179,
            "end": 189
          },
          "start": 179,
          "end": 189
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 197,
        "end": 213
      },
      "phase": null,
      "attributes": [],
      "start": 170,
      "end": 214
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_gDH1EtUWqBU",
            "start": 221,
            "end": 234
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
                "value": "./test.tsx_Parent_component_useTask_gDH1EtUWqBU",
                "raw": "\"./test.tsx_Parent_component_useTask_gDH1EtUWqBU\"",
                "start": 248,
                "end": 297
              },
              "options": null,
              "phase": null,
              "start": 241,
              "end": 298
            },
            "id": null,
            "generator": false,
            "start": 237,
            "end": 298
          },
          "start": 221,
          "end": 298
        }
      ],
      "start": 215,
      "end": 299
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_zM9okM0TYrA",
            "start": 306,
            "end": 319
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
                "value": "./test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA",
                "raw": "\"./test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA\"",
                "start": 333,
                "end": 388
              },
              "options": null,
              "phase": null,
              "start": 326,
              "end": 389
            },
            "id": null,
            "generator": false,
            "start": 322,
            "end": 389
          },
          "start": 306,
          "end": 389
        }
      ],
      "start": 300,
      "end": 390
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
              "name": "Parent_component_0TaiDayHrlo",
              "start": 404,
              "end": 432
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
                          "name": "state",
                          "start": 451,
                          "end": 456
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 459,
                            "end": 467
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
                                    "name": "text",
                                    "start": 478,
                                    "end": 482
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": "",
                                    "raw": "''",
                                    "start": 484,
                                    "end": 486
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 478,
                                  "end": 486
                                }
                              ],
                              "start": 468,
                              "end": 492
                            }
                          ],
                          "optional": false,
                          "start": 459,
                          "end": 493
                        },
                        "start": 451,
                        "end": 493
                      }
                    ],
                    "start": 445,
                    "end": 494
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "useTaskQrl",
                        "start": 525,
                        "end": 535
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 550,
                            "end": 553
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_gDH1EtUWqBU",
                              "start": 554,
                              "end": 567
                            },
                            {
                              "type": "Literal",
                              "value": "Parent_component_useTask_gDH1EtUWqBU",
                              "raw": "\"Parent_component_useTask_gDH1EtUWqBU\"",
                              "start": 569,
                              "end": 607
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "state",
                                  "start": 619,
                                  "end": 624
                                }
                              ],
                              "start": 609,
                              "end": 630
                            }
                          ],
                          "optional": false,
                          "start": 550,
                          "end": 631
                        }
                      ],
                      "optional": false,
                      "start": 525,
                      "end": 632
                    },
                    "start": 525,
                    "end": 633
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 659,
                        "end": 669
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 670,
                          "end": 675
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 677,
                          "end": 681
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:click",
                                "raw": "\"q-e:click\"",
                                "start": 693,
                                "end": 704
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 720,
                                  "end": 723
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_zM9okM0TYrA",
                                    "start": 724,
                                    "end": 737
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Parent_component_div_q_e_click_zM9okM0TYrA",
                                    "raw": "\"Parent_component_div_q_e_click_zM9okM0TYrA\"",
                                    "start": 739,
                                    "end": 783
                                  }
                                ],
                                "optional": false,
                                "start": 720,
                                "end": 784
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 693,
                              "end": 784
                            }
                          ],
                          "start": 683,
                          "end": 790
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_wrapProp",
                            "start": 792,
                            "end": 801
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 802,
                              "end": 807
                            },
                            {
                              "type": "Literal",
                              "value": "text",
                              "raw": "\"text\"",
                              "start": 809,
                              "end": 815
                            }
                          ],
                          "optional": false,
                          "start": 792,
                          "end": 816
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 818,
                          "end": 819
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 821,
                          "end": 827
                        }
                      ],
                      "optional": false,
                      "start": 659,
                      "end": 828
                    },
                    "start": 638,
                    "end": 829
                  }
                ],
                "start": 439,
                "end": 831
              },
              "id": null,
              "generator": false,
              "start": 435,
              "end": 831
            },
            "start": 404,
            "end": 831
          }
        ],
        "start": 398,
        "end": 832
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 391,
      "end": 832
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 832
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_0TaiDayHrlo",
  "entry": "test.tsx_entry_Parent",
  "displayName": "test.tsx_Parent_component",
  "hash": "0TaiDayHrlo",
  "canonicalFilename": "test.tsx_Parent_component_0TaiDayHrlo",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    169,
    421
  ]
}
```

### Module: test.tsx_Child_component_div_q_e_click_cROa4sult1s.js (ENTRY POINT)

```javascript
export const Child_component_div_q_e_click_cROa4sult1s = ()=>console.log('child');
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
              "name": "Child_component_div_q_e_click_cROa4sult1s",
              "start": 13,
              "end": 54
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
                    "start": 61,
                    "end": 68
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 69,
                    "end": 72
                  },
                  "optional": false,
                  "computed": false,
                  "start": 61,
                  "end": 72
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "child",
                    "raw": "'child'",
                    "start": 73,
                    "end": 80
                  }
                ],
                "optional": false,
                "start": 61,
                "end": 81
              },
              "id": null,
              "generator": false,
              "start": 57,
              "end": 81
            },
            "start": 13,
            "end": 81
          }
        ],
        "start": 7,
        "end": 82
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 82
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 82
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Child_component_div_q_e_click_cROa4sult1s",
  "entry": null,
  "displayName": "test.tsx_Child_component_div_q_e_click",
  "hash": "cROa4sult1s",
  "canonicalFilename": "test.tsx_Child_component_div_q_e_click_cROa4sult1s",
  "path": "",
  "extension": "js",
  "parent": "Child_component_9GyF01GDKqw",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    624,
    650
  ]
}
```

### Module: test.tsx_Child_component_9GyF01GDKqw.js

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
import { useTaskQrl } from "@qwik.dev/core";
const i_Oh4n7ZeqJkU = ()=>import("./test.tsx_Child_component_useTask_Oh4n7ZeqJkU");
const i_cROa4sult1s = ()=>import("./test.tsx_Child_component_div_q_e_click_cROa4sult1s");
export const Child_component_9GyF01GDKqw = ()=>{
    const state = useStore({
        text: ''
    });
    // Double count watch
    useTaskQrl(/*#__PURE__*/ qrl(i_Oh4n7ZeqJkU, "Child_component_useTask_Oh4n7ZeqJkU", [
        state
    ]));
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_cROa4sult1s, "Child_component_div_q_e_click_cROa4sult1s")
    }, _wrapProp(state, "text"), 3, "u6_1");
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
            "name": "_wrapProp",
            "start": 54,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 54,
            "end": 63
          },
          "start": 54,
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
      "start": 45,
      "end": 88
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 98,
            "end": 101
          },
          "start": 98,
          "end": 101
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 109,
        "end": 125
      },
      "phase": null,
      "attributes": [],
      "start": 89,
      "end": 126
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 136,
            "end": 144
          },
          "start": 136,
          "end": 144
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 152,
        "end": 168
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 169
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useTaskQrl",
            "start": 179,
            "end": 189
          },
          "local": {
            "type": "Identifier",
            "name": "useTaskQrl",
            "start": 179,
            "end": 189
          },
          "start": 179,
          "end": 189
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 197,
        "end": 213
      },
      "phase": null,
      "attributes": [],
      "start": 170,
      "end": 214
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_Oh4n7ZeqJkU",
            "start": 221,
            "end": 234
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
                "value": "./test.tsx_Child_component_useTask_Oh4n7ZeqJkU",
                "raw": "\"./test.tsx_Child_component_useTask_Oh4n7ZeqJkU\"",
                "start": 248,
                "end": 296
              },
              "options": null,
              "phase": null,
              "start": 241,
              "end": 297
            },
            "id": null,
            "generator": false,
            "start": 237,
            "end": 297
          },
          "start": 221,
          "end": 297
        }
      ],
      "start": 215,
      "end": 298
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_cROa4sult1s",
            "start": 305,
            "end": 318
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
                "value": "./test.tsx_Child_component_div_q_e_click_cROa4sult1s",
                "raw": "\"./test.tsx_Child_component_div_q_e_click_cROa4sult1s\"",
                "start": 332,
                "end": 386
              },
              "options": null,
              "phase": null,
              "start": 325,
              "end": 387
            },
            "id": null,
            "generator": false,
            "start": 321,
            "end": 387
          },
          "start": 305,
          "end": 387
        }
      ],
      "start": 299,
      "end": 388
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
              "name": "Child_component_9GyF01GDKqw",
              "start": 402,
              "end": 429
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
                          "name": "state",
                          "start": 448,
                          "end": 453
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 456,
                            "end": 464
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
                                    "name": "text",
                                    "start": 475,
                                    "end": 479
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": "",
                                    "raw": "''",
                                    "start": 481,
                                    "end": 483
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 475,
                                  "end": 483
                                }
                              ],
                              "start": 465,
                              "end": 489
                            }
                          ],
                          "optional": false,
                          "start": 456,
                          "end": 490
                        },
                        "start": 448,
                        "end": 490
                      }
                    ],
                    "start": 442,
                    "end": 491
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "useTaskQrl",
                        "start": 522,
                        "end": 532
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 547,
                            "end": 550
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_Oh4n7ZeqJkU",
                              "start": 551,
                              "end": 564
                            },
                            {
                              "type": "Literal",
                              "value": "Child_component_useTask_Oh4n7ZeqJkU",
                              "raw": "\"Child_component_useTask_Oh4n7ZeqJkU\"",
                              "start": 566,
                              "end": 603
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "state",
                                  "start": 615,
                                  "end": 620
                                }
                              ],
                              "start": 605,
                              "end": 626
                            }
                          ],
                          "optional": false,
                          "start": 547,
                          "end": 627
                        }
                      ],
                      "optional": false,
                      "start": 522,
                      "end": 628
                    },
                    "start": 522,
                    "end": 629
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 655,
                        "end": 665
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 666,
                          "end": 671
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 673,
                          "end": 677
                        },
                        {
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:click",
                                "raw": "\"q-e:click\"",
                                "start": 689,
                                "end": 700
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 716,
                                  "end": 719
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_cROa4sult1s",
                                    "start": 720,
                                    "end": 733
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Child_component_div_q_e_click_cROa4sult1s",
                                    "raw": "\"Child_component_div_q_e_click_cROa4sult1s\"",
                                    "start": 735,
                                    "end": 778
                                  }
                                ],
                                "optional": false,
                                "start": 716,
                                "end": 779
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 689,
                              "end": 779
                            }
                          ],
                          "start": 679,
                          "end": 785
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_wrapProp",
                            "start": 787,
                            "end": 796
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 797,
                              "end": 802
                            },
                            {
                              "type": "Literal",
                              "value": "text",
                              "raw": "\"text\"",
                              "start": 804,
                              "end": 810
                            }
                          ],
                          "optional": false,
                          "start": 787,
                          "end": 811
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 813,
                          "end": 814
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 816,
                          "end": 822
                        }
                      ],
                      "optional": false,
                      "start": 655,
                      "end": 823
                    },
                    "start": 634,
                    "end": 824
                  }
                ],
                "start": 436,
                "end": 826
              },
              "id": null,
              "generator": false,
              "start": 432,
              "end": 826
            },
            "start": 402,
            "end": 826
          }
        ],
        "start": 396,
        "end": 827
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 389,
      "end": 827
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 827
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Child_component_9GyF01GDKqw",
  "entry": "test.tsx_entry_Child",
  "displayName": "test.tsx_Child_component",
  "hash": "9GyF01GDKqw",
  "canonicalFilename": "test.tsx_Child_component_9GyF01GDKqw",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    457,
    683
  ]
}
```

### Module: test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA.js (ENTRY POINT)

```javascript
export const Parent_component_div_q_e_click_zM9okM0TYrA = ()=>console.log('parent');
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
              "name": "Parent_component_div_q_e_click_zM9okM0TYrA",
              "start": 13,
              "end": 55
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
                    "start": 62,
                    "end": 69
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 70,
                    "end": 73
                  },
                  "optional": false,
                  "computed": false,
                  "start": 62,
                  "end": 73
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "parent",
                    "raw": "'parent'",
                    "start": 74,
                    "end": 82
                  }
                ],
                "optional": false,
                "start": 62,
                "end": 83
              },
              "id": null,
              "generator": false,
              "start": 58,
              "end": 83
            },
            "start": 13,
            "end": 83
          }
        ],
        "start": 7,
        "end": 84
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 84
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 84
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Parent_component_div_q_e_click_zM9okM0TYrA",
  "entry": null,
  "displayName": "test.tsx_Parent_component_div_q_e_click",
  "hash": "zM9okM0TYrA",
  "canonicalFilename": "test.tsx_Parent_component_div_q_e_click_zM9okM0TYrA",
  "path": "",
  "extension": "js",
  "parent": "Parent_component_0TaiDayHrlo",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    361,
    388
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`, `useTaskQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (6 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (10 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_captures[]` | test.tsx_Parent_component_useTask_gDH1EtUWqBU.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 2 |
| `componentQrl` | test.js | @qwik.dev/core | 2 |
| `_captures[]` | test.tsx_Child_component_useTask_Oh4n7ZeqJkU.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_Parent_component_0TaiDayHrlo.js | @qwik.dev/core | 2 |
| `useTaskQrl` | test.tsx_Parent_component_0TaiDayHrlo.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx_Parent_component_0TaiDayHrlo.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.tsx_Parent_component_0TaiDayHrlo.js | @qwik.dev/core | 1 |
| `useStore` | test.tsx_Parent_component_0TaiDayHrlo.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 2 |
| `useTaskQrl` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `useStore` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
