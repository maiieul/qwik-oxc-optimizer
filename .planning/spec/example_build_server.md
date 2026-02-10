# Test: example_build_server

## Test Configuration

| Option | Value |
|--------|-------|
| Is Server | Some(true) |
| Mode | Prod |

## Input

### Source Code

```tsx
import { component$, useStore, isDev, isServer as isServer2 } from '@qwik.dev/core';
import { isServer, isBrowser as isb } from '@qwik.dev/core/build';
import { mongodb } from 'mondodb';
import { threejs } from 'threejs';

import L from 'leaflet';

export const functionThatNeedsWindow = () => {
  if (isb) {
    console.log('l', L);
    console.log('hey');
    window.alert('hey');
  }
};

export const App = component$(() => {
	useMount$(() => {
		if (isServer) {
			console.log('server', mongodb());
		}
		if (isb) {
			console.log('browser', new threejs());
		}
	});
	return (
		<Cmp>
			{isServer2 && <p>server</p>}
			{isb && <p>server</p>}
		</Cmp>
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
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 29
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
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
            "name": "isDev",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 36
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "isDev",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 36
          },
          "importKind": "value",
          "start": 31,
          "end": 36
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "isServer",
            "optional": false,
            "typeAnnotation": null,
            "start": 38,
            "end": 46
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "isServer2",
            "optional": false,
            "typeAnnotation": null,
            "start": 50,
            "end": 59
          },
          "importKind": "value",
          "start": 38,
          "end": 59
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 67,
        "end": 83
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 84
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "isServer",
            "optional": false,
            "typeAnnotation": null,
            "start": 94,
            "end": 102
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "isServer",
            "optional": false,
            "typeAnnotation": null,
            "start": 94,
            "end": 102
          },
          "importKind": "value",
          "start": 94,
          "end": 102
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "isBrowser",
            "optional": false,
            "typeAnnotation": null,
            "start": 104,
            "end": 113
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "isb",
            "optional": false,
            "typeAnnotation": null,
            "start": 117,
            "end": 120
          },
          "importKind": "value",
          "start": 104,
          "end": 120
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/build",
        "raw": "'@qwik.dev/core/build'",
        "start": 128,
        "end": 150
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 85,
      "end": 151
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 161,
            "end": 168
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 161,
            "end": 168
          },
          "importKind": "value",
          "start": 161,
          "end": 168
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mondodb",
        "raw": "'mondodb'",
        "start": 176,
        "end": 185
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 152,
      "end": 186
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "threejs",
            "optional": false,
            "typeAnnotation": null,
            "start": 196,
            "end": 203
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "threejs",
            "optional": false,
            "typeAnnotation": null,
            "start": 196,
            "end": 203
          },
          "importKind": "value",
          "start": 196,
          "end": 203
        }
      ],
      "source": {
        "type": "Literal",
        "value": "threejs",
        "raw": "'threejs'",
        "start": 211,
        "end": 220
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 187,
      "end": 221
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "L",
            "optional": false,
            "typeAnnotation": null,
            "start": 230,
            "end": 231
          },
          "start": 230,
          "end": 231
        }
      ],
      "source": {
        "type": "Literal",
        "value": "leaflet",
        "raw": "'leaflet'",
        "start": 237,
        "end": 246
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 223,
      "end": 247
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
              "name": "functionThatNeedsWindow",
              "optional": false,
              "typeAnnotation": null,
              "start": 262,
              "end": 285
            },
            "init": {
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
                    "type": "IfStatement",
                    "test": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "isb",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 302,
                      "end": 305
                    },
                    "consequent": {
                      "type": "BlockStatement",
                      "body": [
                        {
                          "type": "ExpressionStatement",
                          "expression": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "console",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 313,
                                "end": 320
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "log",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 321,
                                "end": 324
                              },
                              "optional": false,
                              "computed": false,
                              "start": 313,
                              "end": 324
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "l",
                                "raw": "'l'",
                                "start": 325,
                                "end": 328
                              },
                              {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "L",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 330,
                                "end": 331
                              }
                            ],
                            "optional": false,
                            "start": 313,
                            "end": 332
                          },
                          "directive": null,
                          "start": 313,
                          "end": 333
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
                                "name": "console",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 338,
                                "end": 345
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "log",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 346,
                                "end": 349
                              },
                              "optional": false,
                              "computed": false,
                              "start": 338,
                              "end": 349
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "hey",
                                "raw": "'hey'",
                                "start": 350,
                                "end": 355
                              }
                            ],
                            "optional": false,
                            "start": 338,
                            "end": 356
                          },
                          "directive": null,
                          "start": 338,
                          "end": 357
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
                                "name": "window",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 362,
                                "end": 368
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "alert",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 369,
                                "end": 374
                              },
                              "optional": false,
                              "computed": false,
                              "start": 362,
                              "end": 374
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "hey",
                                "raw": "'hey'",
                                "start": 375,
                                "end": 380
                              }
                            ],
                            "optional": false,
                            "start": 362,
                            "end": 381
                          },
                          "directive": null,
                          "start": 362,
                          "end": 382
                        }
                      ],
                      "start": 307,
                      "end": 386
                    },
                    "alternate": null,
                    "start": 298,
                    "end": 386
                  }
                ],
                "start": 294,
                "end": 388
              },
              "id": null,
              "generator": false,
              "start": 288,
              "end": 388
            },
            "definite": false,
            "start": 262,
            "end": 388
          }
        ],
        "declare": false,
        "start": 256,
        "end": 389
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 249,
      "end": 389
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
              "name": "App",
              "optional": false,
              "typeAnnotation": null,
              "start": 404,
              "end": 407
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 410,
                "end": 420
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
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useMount$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 430,
                            "end": 439
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
                                    "type": "IfStatement",
                                    "test": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "isServer",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 454,
                                      "end": 462
                                    },
                                    "consequent": {
                                      "type": "BlockStatement",
                                      "body": [
                                        {
                                          "type": "ExpressionStatement",
                                          "expression": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "console",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 469,
                                                "end": 476
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 477,
                                                "end": 480
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 469,
                                              "end": 480
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "server",
                                                "raw": "'server'",
                                                "start": 481,
                                                "end": 489
                                              },
                                              {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "mongodb",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 491,
                                                  "end": 498
                                                },
                                                "typeArguments": null,
                                                "arguments": [],
                                                "optional": false,
                                                "start": 491,
                                                "end": 500
                                              }
                                            ],
                                            "optional": false,
                                            "start": 469,
                                            "end": 501
                                          },
                                          "directive": null,
                                          "start": 469,
                                          "end": 502
                                        }
                                      ],
                                      "start": 464,
                                      "end": 506
                                    },
                                    "alternate": null,
                                    "start": 450,
                                    "end": 506
                                  },
                                  {
                                    "type": "IfStatement",
                                    "test": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "isb",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 513,
                                      "end": 516
                                    },
                                    "consequent": {
                                      "type": "BlockStatement",
                                      "body": [
                                        {
                                          "type": "ExpressionStatement",
                                          "expression": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "console",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 523,
                                                "end": 530
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "log",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 531,
                                                "end": 534
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 523,
                                              "end": 534
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "browser",
                                                "raw": "'browser'",
                                                "start": 535,
                                                "end": 544
                                              },
                                              {
                                                "type": "NewExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "threejs",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 550,
                                                  "end": 557
                                                },
                                                "typeArguments": null,
                                                "arguments": [],
                                                "start": 546,
                                                "end": 559
                                              }
                                            ],
                                            "optional": false,
                                            "start": 523,
                                            "end": 560
                                          },
                                          "directive": null,
                                          "start": 523,
                                          "end": 561
                                        }
                                      ],
                                      "start": 518,
                                      "end": 565
                                    },
                                    "alternate": null,
                                    "start": 509,
                                    "end": 565
                                  }
                                ],
                                "start": 446,
                                "end": 568
                              },
                              "id": null,
                              "generator": false,
                              "start": 440,
                              "end": 568
                            }
                          ],
                          "optional": false,
                          "start": 430,
                          "end": 569
                        },
                        "directive": null,
                        "start": 430,
                        "end": 570
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
                                "name": "Cmp",
                                "start": 584,
                                "end": 587
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 583,
                              "end": 588
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 588,
                                "end": 592
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "LogicalExpression",
                                  "left": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "isServer2",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 593,
                                    "end": 602
                                  },
                                  "operator": "&&",
                                  "right": {
                                    "type": "JSXElement",
                                    "openingElement": {
                                      "type": "JSXOpeningElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "p",
                                        "start": 607,
                                        "end": 608
                                      },
                                      "typeArguments": null,
                                      "attributes": [],
                                      "selfClosing": false,
                                      "start": 606,
                                      "end": 609
                                    },
                                    "children": [
                                      {
                                        "type": "JSXText",
                                        "value": "server",
                                        "raw": "server",
                                        "start": 609,
                                        "end": 615
                                      }
                                    ],
                                    "closingElement": {
                                      "type": "JSXClosingElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "p",
                                        "start": 617,
                                        "end": 618
                                      },
                                      "start": 615,
                                      "end": 619
                                    },
                                    "start": 606,
                                    "end": 619
                                  },
                                  "start": 593,
                                  "end": 619
                                },
                                "start": 592,
                                "end": 620
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 620,
                                "end": 624
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "LogicalExpression",
                                  "left": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "isb",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 625,
                                    "end": 628
                                  },
                                  "operator": "&&",
                                  "right": {
                                    "type": "JSXElement",
                                    "openingElement": {
                                      "type": "JSXOpeningElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "p",
                                        "start": 633,
                                        "end": 634
                                      },
                                      "typeArguments": null,
                                      "attributes": [],
                                      "selfClosing": false,
                                      "start": 632,
                                      "end": 635
                                    },
                                    "children": [
                                      {
                                        "type": "JSXText",
                                        "value": "server",
                                        "raw": "server",
                                        "start": 635,
                                        "end": 641
                                      }
                                    ],
                                    "closingElement": {
                                      "type": "JSXClosingElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "p",
                                        "start": 643,
                                        "end": 644
                                      },
                                      "start": 641,
                                      "end": 645
                                    },
                                    "start": 632,
                                    "end": 645
                                  },
                                  "start": 625,
                                  "end": 645
                                },
                                "start": 624,
                                "end": 646
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 646,
                                "end": 649
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Cmp",
                                "start": 651,
                                "end": 654
                              },
                              "start": 649,
                              "end": 655
                            },
                            "start": 583,
                            "end": 655
                          },
                          "start": 579,
                          "end": 658
                        },
                        "start": 572,
                        "end": 659
                      }
                    ],
                    "start": 427,
                    "end": 661
                  },
                  "id": null,
                  "generator": false,
                  "start": 421,
                  "end": 661
                }
              ],
              "optional": false,
              "start": 410,
              "end": 662
            },
            "definite": false,
            "start": 404,
            "end": 662
          }
        ],
        "declare": false,
        "start": 398,
        "end": 663
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 391,
      "end": 663
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 663
}
```

</details>

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
export const functionThatNeedsWindow = ()=>{};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "s_ckEPmXZlub0"));
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
            "decorators": [],
            "name": "componentQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 21
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "componentQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 21
          },
          "importKind": "value",
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
      "importKind": "value",
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
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
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
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 67,
        "end": 83
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
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
            "decorators": [],
            "name": "i_ckEPmXZlub0",
            "optional": false,
            "typeAnnotation": null,
            "start": 91,
            "end": 104
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": true,
            "async": false,
            "typeParameters": null,
            "params": [],
            "returnType": null,
            "body": {
              "type": "ImportExpression",
              "source": {
                "type": "Literal",
                "value": "./test.tsx_App_component_ckEPmXZlub0",
                "raw": "\"./test.tsx_App_component_ckEPmXZlub0\"",
                "start": 118,
                "end": 156
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 157
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 157
          },
          "definite": false,
          "start": 91,
          "end": 157
        }
      ],
      "declare": false,
      "start": 85,
      "end": 158
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
              "name": "functionThatNeedsWindow",
              "optional": false,
              "typeAnnotation": null,
              "start": 172,
              "end": 195
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "typeParameters": null,
              "params": [],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [],
                "start": 202,
                "end": 204
              },
              "id": null,
              "generator": false,
              "start": 198,
              "end": 204
            },
            "definite": false,
            "start": 172,
            "end": 204
          }
        ],
        "declare": false,
        "start": 166,
        "end": 205
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 159,
      "end": 205
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
              "name": "App",
              "optional": false,
              "typeAnnotation": null,
              "start": 219,
              "end": 222
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 239,
                "end": 251
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "qrl",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 266,
                    "end": 269
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_ckEPmXZlub0",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 270,
                      "end": 283
                    },
                    {
                      "type": "Literal",
                      "value": "s_ckEPmXZlub0",
                      "raw": "\"s_ckEPmXZlub0\"",
                      "start": 285,
                      "end": 300
                    }
                  ],
                  "optional": false,
                  "start": 266,
                  "end": 301
                }
              ],
              "optional": false,
              "start": 239,
              "end": 302
            },
            "definite": false,
            "start": 219,
            "end": 302
          }
        ],
        "declare": false,
        "start": 213,
        "end": 303
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 206,
      "end": 303
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 303
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.tsx (ENTRY POINT)

```tsx
import { mongodb } from "mondodb";
export const s_ckEPmXZlub0 = ()=>{
    useMount$(()=>{
        console.log('server', mongodb());
    });
    return <Cmp>
			{<p>server</p>}
			{false}
		</Cmp>;
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
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 16
          },
          "importKind": "value",
          "start": 9,
          "end": 16
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mondodb",
        "raw": "\"mondodb\"",
        "start": 24,
        "end": 33
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 34
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
              "name": "s_ckEPmXZlub0",
              "optional": false,
              "typeAnnotation": null,
              "start": 48,
              "end": 61
            },
            "init": {
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
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "useMount$",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 74,
                        "end": 83
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
                                "type": "ExpressionStatement",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "console",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 98,
                                      "end": 105
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "log",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 106,
                                      "end": 109
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 98,
                                    "end": 109
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "server",
                                      "raw": "'server'",
                                      "start": 110,
                                      "end": 118
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "mongodb",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 120,
                                        "end": 127
                                      },
                                      "typeArguments": null,
                                      "arguments": [],
                                      "optional": false,
                                      "start": 120,
                                      "end": 129
                                    }
                                  ],
                                  "optional": false,
                                  "start": 98,
                                  "end": 130
                                },
                                "directive": null,
                                "start": 98,
                                "end": 131
                              }
                            ],
                            "start": 88,
                            "end": 137
                          },
                          "id": null,
                          "generator": false,
                          "start": 84,
                          "end": 137
                        }
                      ],
                      "optional": false,
                      "start": 74,
                      "end": 138
                    },
                    "directive": null,
                    "start": 74,
                    "end": 139
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Cmp",
                          "start": 152,
                          "end": 155
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 151,
                        "end": 156
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 156,
                          "end": 160
                        },
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "JSXElement",
                            "openingElement": {
                              "type": "JSXOpeningElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "p",
                                "start": 162,
                                "end": 163
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 161,
                              "end": 164
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "server",
                                "raw": "server",
                                "start": 164,
                                "end": 170
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "p",
                                "start": 172,
                                "end": 173
                              },
                              "start": 170,
                              "end": 174
                            },
                            "start": 161,
                            "end": 174
                          },
                          "start": 160,
                          "end": 175
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 175,
                          "end": 179
                        },
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "Literal",
                            "value": false,
                            "raw": "false",
                            "start": 180,
                            "end": 185
                          },
                          "start": 179,
                          "end": 186
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t",
                          "raw": "\n\t\t",
                          "start": 186,
                          "end": 189
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "Cmp",
                          "start": 191,
                          "end": 194
                        },
                        "start": 189,
                        "end": 195
                      },
                      "start": 151,
                      "end": 195
                    },
                    "start": 144,
                    "end": 196
                  }
                ],
                "start": 68,
                "end": 198
              },
              "id": null,
              "generator": false,
              "start": 64,
              "end": 198
            },
            "definite": false,
            "start": 48,
            "end": 198
          }
        ],
        "declare": false,
        "start": 42,
        "end": 199
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 35,
      "end": 199
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 199
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "s_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    423,
    663
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (1 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (2 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point segment(s)
- **[CONV-10] Const Replacement**: `isServer`/`isBrowser`/`isDev` constants replaced with boolean literals based on build target

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.tsx | @qwik.dev/core | 1 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
