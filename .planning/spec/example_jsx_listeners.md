# Test: example_jsx_listeners

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | True |
| Transpile Jsx | True |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

export const Foo = component$(() => {

	return $(() => {
		const handler = $(() => console.log('reused'));
		return (
			<div
				onClick$={()=>console.log('onClick$')}
				onDocumentScroll$={()=>console.log('onDocumentScroll')}
				onDocumentScroll$={()=>console.log('onWindowScroll')}

				on-cLick$={()=>console.log('on-cLick$')}
				onDocument-sCroll$={()=>console.log('onDocument-sCroll')}
				onDocument-scroLL$={()=>console.log('onDocument-scroLL')}

				host:onClick$={()=>console.log('host:onClick$')}
				host:onDocumentScroll$={()=>console.log('host:onDocument:scroll')}
				host:onDocumentScroll$={()=>console.log('host:onWindow:scroll')}

				onKeyup$={handler}
				onDocument:keyup$={handler}
				onWindow:keyup$={handler}

				custom$={()=>console.log('custom')}
			/>
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
              "start": 62,
              "end": 65
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 68,
                "end": 78
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
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 96,
                            "end": 97
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
                                          "name": "handler",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 114,
                                          "end": 121
                                        },
                                        "init": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "$",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 124,
                                            "end": 125
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
                                                    "start": 132,
                                                    "end": 139
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "log",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 140,
                                                    "end": 143
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 132,
                                                  "end": 143
                                                },
                                                "typeArguments": null,
                                                "arguments": [
                                                  {
                                                    "type": "Literal",
                                                    "value": "reused",
                                                    "raw": "'reused'",
                                                    "start": 144,
                                                    "end": 152
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 132,
                                                "end": 153
                                              },
                                              "id": null,
                                              "generator": false,
                                              "start": 126,
                                              "end": 153
                                            }
                                          ],
                                          "optional": false,
                                          "start": 124,
                                          "end": 154
                                        },
                                        "definite": false,
                                        "start": 114,
                                        "end": 154
                                      }
                                    ],
                                    "declare": false,
                                    "start": 108,
                                    "end": 155
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
                                            "start": 171,
                                            "end": 174
                                          },
                                          "typeArguments": null,
                                          "attributes": [
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onClick$",
                                                "start": 179,
                                                "end": 187
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
                                                        "start": 193,
                                                        "end": 200
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 201,
                                                        "end": 204
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 193,
                                                      "end": 204
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "onClick$",
                                                        "raw": "'onClick$'",
                                                        "start": 205,
                                                        "end": 215
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 193,
                                                    "end": 216
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 189,
                                                  "end": 216
                                                },
                                                "start": 188,
                                                "end": 217
                                              },
                                              "start": 179,
                                              "end": 217
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onDocumentScroll$",
                                                "start": 222,
                                                "end": 239
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
                                                        "start": 245,
                                                        "end": 252
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 253,
                                                        "end": 256
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 245,
                                                      "end": 256
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "onDocumentScroll",
                                                        "raw": "'onDocumentScroll'",
                                                        "start": 257,
                                                        "end": 275
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 245,
                                                    "end": 276
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 241,
                                                  "end": 276
                                                },
                                                "start": 240,
                                                "end": 277
                                              },
                                              "start": 222,
                                              "end": 277
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onDocumentScroll$",
                                                "start": 282,
                                                "end": 299
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
                                                        "start": 305,
                                                        "end": 312
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 313,
                                                        "end": 316
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 305,
                                                      "end": 316
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "onWindowScroll",
                                                        "raw": "'onWindowScroll'",
                                                        "start": 317,
                                                        "end": 333
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 305,
                                                    "end": 334
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 301,
                                                  "end": 334
                                                },
                                                "start": 300,
                                                "end": 335
                                              },
                                              "start": 282,
                                              "end": 335
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "on-cLick$",
                                                "start": 341,
                                                "end": 350
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
                                                        "start": 356,
                                                        "end": 363
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 364,
                                                        "end": 367
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 356,
                                                      "end": 367
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "on-cLick$",
                                                        "raw": "'on-cLick$'",
                                                        "start": 368,
                                                        "end": 379
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 356,
                                                    "end": 380
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 352,
                                                  "end": 380
                                                },
                                                "start": 351,
                                                "end": 381
                                              },
                                              "start": 341,
                                              "end": 381
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onDocument-sCroll$",
                                                "start": 386,
                                                "end": 404
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
                                                        "start": 410,
                                                        "end": 417
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 418,
                                                        "end": 421
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 410,
                                                      "end": 421
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "onDocument-sCroll",
                                                        "raw": "'onDocument-sCroll'",
                                                        "start": 422,
                                                        "end": 441
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 410,
                                                    "end": 442
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 406,
                                                  "end": 442
                                                },
                                                "start": 405,
                                                "end": 443
                                              },
                                              "start": 386,
                                              "end": 443
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onDocument-scroLL$",
                                                "start": 448,
                                                "end": 466
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
                                                        "start": 472,
                                                        "end": 479
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 480,
                                                        "end": 483
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 472,
                                                      "end": 483
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "onDocument-scroLL",
                                                        "raw": "'onDocument-scroLL'",
                                                        "start": 484,
                                                        "end": 503
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 472,
                                                    "end": 504
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 468,
                                                  "end": 504
                                                },
                                                "start": 467,
                                                "end": 505
                                              },
                                              "start": 448,
                                              "end": 505
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXNamespacedName",
                                                "namespace": {
                                                  "type": "JSXIdentifier",
                                                  "name": "host",
                                                  "start": 511,
                                                  "end": 515
                                                },
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onClick$",
                                                  "start": 516,
                                                  "end": 524
                                                },
                                                "start": 511,
                                                "end": 524
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
                                                        "start": 530,
                                                        "end": 537
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 538,
                                                        "end": 541
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 530,
                                                      "end": 541
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "host:onClick$",
                                                        "raw": "'host:onClick$'",
                                                        "start": 542,
                                                        "end": 557
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 530,
                                                    "end": 558
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 526,
                                                  "end": 558
                                                },
                                                "start": 525,
                                                "end": 559
                                              },
                                              "start": 511,
                                              "end": 559
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXNamespacedName",
                                                "namespace": {
                                                  "type": "JSXIdentifier",
                                                  "name": "host",
                                                  "start": 564,
                                                  "end": 568
                                                },
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onDocumentScroll$",
                                                  "start": 569,
                                                  "end": 586
                                                },
                                                "start": 564,
                                                "end": 586
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
                                                        "start": 592,
                                                        "end": 599
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 600,
                                                        "end": 603
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 592,
                                                      "end": 603
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "host:onDocument:scroll",
                                                        "raw": "'host:onDocument:scroll'",
                                                        "start": 604,
                                                        "end": 628
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 592,
                                                    "end": 629
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 588,
                                                  "end": 629
                                                },
                                                "start": 587,
                                                "end": 630
                                              },
                                              "start": 564,
                                              "end": 630
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXNamespacedName",
                                                "namespace": {
                                                  "type": "JSXIdentifier",
                                                  "name": "host",
                                                  "start": 635,
                                                  "end": 639
                                                },
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onDocumentScroll$",
                                                  "start": 640,
                                                  "end": 657
                                                },
                                                "start": 635,
                                                "end": 657
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
                                                        "start": 663,
                                                        "end": 670
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 671,
                                                        "end": 674
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 663,
                                                      "end": 674
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "host:onWindow:scroll",
                                                        "raw": "'host:onWindow:scroll'",
                                                        "start": 675,
                                                        "end": 697
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 663,
                                                    "end": 698
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 659,
                                                  "end": 698
                                                },
                                                "start": 658,
                                                "end": 699
                                              },
                                              "start": 635,
                                              "end": 699
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onKeyup$",
                                                "start": 705,
                                                "end": 713
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "handler",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 715,
                                                  "end": 722
                                                },
                                                "start": 714,
                                                "end": 723
                                              },
                                              "start": 705,
                                              "end": 723
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXNamespacedName",
                                                "namespace": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onDocument",
                                                  "start": 728,
                                                  "end": 738
                                                },
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "keyup$",
                                                  "start": 739,
                                                  "end": 745
                                                },
                                                "start": 728,
                                                "end": 745
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "handler",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 747,
                                                  "end": 754
                                                },
                                                "start": 746,
                                                "end": 755
                                              },
                                              "start": 728,
                                              "end": 755
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXNamespacedName",
                                                "namespace": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onWindow",
                                                  "start": 760,
                                                  "end": 768
                                                },
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "keyup$",
                                                  "start": 769,
                                                  "end": 775
                                                },
                                                "start": 760,
                                                "end": 775
                                              },
                                              "value": {
                                                "type": "JSXExpressionContainer",
                                                "expression": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "handler",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 777,
                                                  "end": 784
                                                },
                                                "start": 776,
                                                "end": 785
                                              },
                                              "start": 760,
                                              "end": 785
                                            },
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "custom$",
                                                "start": 791,
                                                "end": 798
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
                                                        "start": 804,
                                                        "end": 811
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "log",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 812,
                                                        "end": 815
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 804,
                                                      "end": 815
                                                    },
                                                    "typeArguments": null,
                                                    "arguments": [
                                                      {
                                                        "type": "Literal",
                                                        "value": "custom",
                                                        "raw": "'custom'",
                                                        "start": 816,
                                                        "end": 824
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 804,
                                                    "end": 825
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 800,
                                                  "end": 825
                                                },
                                                "start": 799,
                                                "end": 826
                                              },
                                              "start": 791,
                                              "end": 826
                                            }
                                          ],
                                          "selfClosing": true,
                                          "start": 170,
                                          "end": 832
                                        },
                                        "children": [],
                                        "closingElement": null,
                                        "start": 170,
                                        "end": 832
                                      },
                                      "start": 165,
                                      "end": 836
                                    },
                                    "start": 158,
                                    "end": 836
                                  }
                                ],
                                "start": 104,
                                "end": 839
                              },
                              "id": null,
                              "generator": false,
                              "start": 98,
                              "end": 839
                            }
                          ],
                          "optional": false,
                          "start": 96,
                          "end": 840
                        },
                        "start": 89,
                        "end": 841
                      }
                    ],
                    "start": 85,
                    "end": 843
                  },
                  "id": null,
                  "generator": false,
                  "start": 79,
                  "end": 843
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
                        "start": 848,
                        "end": 855
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 857,
                        "end": 865
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 848,
                      "end": 865
                    }
                  ],
                  "start": 845,
                  "end": 868
                }
              ],
              "optional": false,
              "start": 68,
              "end": 869
            },
            "definite": false,
            "start": 62,
            "end": 869
          }
        ],
        "declare": false,
        "start": 56,
        "end": 870
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 870
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 870
}
```

</details>

## Output

### Module: test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY.js (ENTRY POINT)

```javascript
export const Foo_component_div_host_onDocumentScroll_Zip7mifsjRY = ()=>console.log('host:onDocument:scroll');
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
              "name": "Foo_component_div_host_onDocumentScroll_Zip7mifsjRY",
              "start": 13,
              "end": 64
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
                    "start": 71,
                    "end": 78
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 79,
                    "end": 82
                  },
                  "optional": false,
                  "computed": false,
                  "start": 71,
                  "end": 82
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "host:onDocument:scroll",
                    "raw": "'host:onDocument:scroll'",
                    "start": 83,
                    "end": 107
                  }
                ],
                "optional": false,
                "start": 71,
                "end": 108
              },
              "id": null,
              "generator": false,
              "start": 67,
              "end": 108
            },
            "start": 13,
            "end": 108
          }
        ],
        "start": 7,
        "end": 109
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 109
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 109
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_host_onDocumentScroll_Zip7mifsjRY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_host_onDocumentScroll",
  "hash": "Zip7mifsjRY",
  "canonicalFilename": "test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "host:onDocumentScroll$",
  "captures": false,
  "loc": [
    590,
    631
  ]
}
```

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
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
            "name": "i_HTDRsvUbLiE",
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
                "value": "./test.tsx_Foo_component_HTDRsvUbLiE",
                "raw": "\"./test.tsx_Foo_component_HTDRsvUbLiE\"",
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
          "start": 91,
          "end": 157
        }
      ],
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
              "name": "Foo",
              "start": 172,
              "end": 175
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 192,
                "end": 204
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 219,
                    "end": 222
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_HTDRsvUbLiE",
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "Foo_component_HTDRsvUbLiE",
                      "raw": "\"Foo_component_HTDRsvUbLiE\"",
                      "start": 238,
                      "end": 265
                    }
                  ],
                  "optional": false,
                  "start": 219,
                  "end": 266
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
                        "start": 274,
                        "end": 281
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 283,
                        "end": 291
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "start": 274,
                      "end": 291
                    }
                  ],
                  "start": 268,
                  "end": 293
                }
              ],
              "optional": false,
              "start": 192,
              "end": 294
            },
            "start": 172,
            "end": 294
          }
        ],
        "start": 166,
        "end": 295
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 159,
      "end": 295
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 295
}
```

</details>

### Module: test.tsx_Foo_component_HTDRsvUbLiE.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
const i_DvU6FitWglY = ()=>import("./test.tsx_Foo_component_1_DvU6FitWglY");
export const Foo_component_HTDRsvUbLiE = ()=>{
    return /*#__PURE__*/ qrl(i_DvU6FitWglY, "Foo_component_1_DvU6FitWglY");
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
              "params": [],
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
                        "start": 186,
                        "end": 189
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_DvU6FitWglY",
                          "start": 190,
                          "end": 203
                        },
                        {
                          "type": "Literal",
                          "value": "Foo_component_1_DvU6FitWglY",
                          "raw": "\"Foo_component_1_DvU6FitWglY\"",
                          "start": 205,
                          "end": 234
                        }
                      ],
                      "optional": false,
                      "start": 186,
                      "end": 235
                    },
                    "start": 165,
                    "end": 236
                  }
                ],
                "start": 159,
                "end": 238
              },
              "id": null,
              "generator": false,
              "start": 155,
              "end": 238
            },
            "start": 127,
            "end": 238
          }
        ],
        "start": 121,
        "end": 239
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 114,
      "end": 239
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 239
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
    81,
    845
  ]
}
```

### Module: test.tsx_Foo_component_div_host_onClick_cPEH970JbEY.js (ENTRY POINT)

```javascript
export const Foo_component_div_host_onClick_cPEH970JbEY = ()=>console.log('host:onClick$');
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
              "name": "Foo_component_div_host_onClick_cPEH970JbEY",
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
                    "value": "host:onClick$",
                    "raw": "'host:onClick$'",
                    "start": 74,
                    "end": 89
                  }
                ],
                "optional": false,
                "start": 62,
                "end": 90
              },
              "id": null,
              "generator": false,
              "start": 58,
              "end": 90
            },
            "start": 13,
            "end": 90
          }
        ],
        "start": 7,
        "end": 91
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 91
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 91
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_host_onClick_cPEH970JbEY",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_host_onClick",
  "hash": "cPEH970JbEY",
  "canonicalFilename": "test.tsx_Foo_component_div_host_onClick_cPEH970JbEY",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "host:onClick$",
  "captures": false,
  "loc": [
    528,
    560
  ]
}
```

### Module: test.tsx_Foo_component_1_DvU6FitWglY.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_0FSbGzUROso = ()=>import("./test.tsx_Foo_component_div_q_e_documentscroll_0FSbGzUROso");
const i_6qyBttefepU = ()=>import("./test.tsx_Foo_component_div_q_e_document_scroll_6qyBttefepU");
const i_Em1LspK7JVg = ()=>import("./test.tsx_Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg");
const i_H10xZtD0e7w = ()=>import("./test.tsx_Foo_component_handler_H10xZtD0e7w");
const i_YEa2A5ADUOg = ()=>import("./test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg");
const i_Zip7mifsjRY = ()=>import("./test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY");
const i_cPEH970JbEY = ()=>import("./test.tsx_Foo_component_div_host_onClick_cPEH970JbEY");
const i_d0Zn04qNgs0 = ()=>import("./test.tsx_Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0");
const i_kX5SiYdz650 = ()=>import("./test.tsx_Foo_component_div_q_e_c_lick_kX5SiYdz650");
const i_pyHnxab17ms = ()=>import("./test.tsx_Foo_component_div_custom_pyHnxab17ms");
const i_wphyTkeintI = ()=>import("./test.tsx_Foo_component_div_q_e_document_scroll_1_wphyTkeintI");
export const Foo_component_1_DvU6FitWglY = ()=>{
    const handler = /*#__PURE__*/ qrl(i_H10xZtD0e7w, "Foo_component_handler_H10xZtD0e7w");
    return /*#__PURE__*/ _jsxSorted("div", null, {
        "q-e:click": /*#__PURE__*/ qrl(i_YEa2A5ADUOg, "Foo_component_div_q_e_click_YEa2A5ADUOg"),
        "q-e:documentscroll": /*#__PURE__*/ qrl(i_0FSbGzUROso, "Foo_component_div_q_e_documentscroll_0FSbGzUROso"),
        "q-e:documentscroll": /*#__PURE__*/ qrl(i_d0Zn04qNgs0, "Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0"),
        "q-e:c-lick": /*#__PURE__*/ qrl(i_kX5SiYdz650, "Foo_component_div_q_e_c_lick_kX5SiYdz650"),
        "q-e:document--scroll": /*#__PURE__*/ qrl(i_6qyBttefepU, "Foo_component_div_q_e_document_scroll_6qyBttefepU"),
        "q-e:document--scroll": /*#__PURE__*/ qrl(i_wphyTkeintI, "Foo_component_div_q_e_document_scroll_1_wphyTkeintI"),
        "host:onClick$": /*#__PURE__*/ qrl(i_cPEH970JbEY, "Foo_component_div_host_onClick_cPEH970JbEY"),
        "host:onDocumentScroll$": /*#__PURE__*/ qrl(i_Zip7mifsjRY, "Foo_component_div_host_onDocumentScroll_Zip7mifsjRY"),
        "host:onDocumentScroll$": /*#__PURE__*/ qrl(i_Em1LspK7JVg, "Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg"),
        "q-e:keyup": handler,
        "q-e:document:keyup": handler,
        "q-e:window:keyup": handler,
        custom$: /*#__PURE__*/ qrl(i_pyHnxab17ms, "Foo_component_div_custom_pyHnxab17ms")
    }, null, 3, "u6_0");
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
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 54,
            "end": 57
          },
          "start": 54,
          "end": 57
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 65,
        "end": 81
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 82
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_0FSbGzUROso",
            "start": 89,
            "end": 102
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
                "value": "./test.tsx_Foo_component_div_q_e_documentscroll_0FSbGzUROso",
                "raw": "\"./test.tsx_Foo_component_div_q_e_documentscroll_0FSbGzUROso\"",
                "start": 116,
                "end": 177
              },
              "options": null,
              "phase": null,
              "start": 109,
              "end": 178
            },
            "id": null,
            "generator": false,
            "start": 105,
            "end": 178
          },
          "start": 89,
          "end": 178
        }
      ],
      "start": 83,
      "end": 179
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_6qyBttefepU",
            "start": 186,
            "end": 199
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
                "value": "./test.tsx_Foo_component_div_q_e_document_scroll_6qyBttefepU",
                "raw": "\"./test.tsx_Foo_component_div_q_e_document_scroll_6qyBttefepU\"",
                "start": 213,
                "end": 275
              },
              "options": null,
              "phase": null,
              "start": 206,
              "end": 276
            },
            "id": null,
            "generator": false,
            "start": 202,
            "end": 276
          },
          "start": 186,
          "end": 276
        }
      ],
      "start": 180,
      "end": 277
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_Em1LspK7JVg",
            "start": 284,
            "end": 297
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
                "value": "./test.tsx_Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg",
                "raw": "\"./test.tsx_Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg\"",
                "start": 311,
                "end": 377
              },
              "options": null,
              "phase": null,
              "start": 304,
              "end": 378
            },
            "id": null,
            "generator": false,
            "start": 300,
            "end": 378
          },
          "start": 284,
          "end": 378
        }
      ],
      "start": 278,
      "end": 379
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_H10xZtD0e7w",
            "start": 386,
            "end": 399
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
                "value": "./test.tsx_Foo_component_handler_H10xZtD0e7w",
                "raw": "\"./test.tsx_Foo_component_handler_H10xZtD0e7w\"",
                "start": 413,
                "end": 459
              },
              "options": null,
              "phase": null,
              "start": 406,
              "end": 460
            },
            "id": null,
            "generator": false,
            "start": 402,
            "end": 460
          },
          "start": 386,
          "end": 460
        }
      ],
      "start": 380,
      "end": 461
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_YEa2A5ADUOg",
            "start": 468,
            "end": 481
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
                "value": "./test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg",
                "raw": "\"./test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg\"",
                "start": 495,
                "end": 547
              },
              "options": null,
              "phase": null,
              "start": 488,
              "end": 548
            },
            "id": null,
            "generator": false,
            "start": 484,
            "end": 548
          },
          "start": 468,
          "end": 548
        }
      ],
      "start": 462,
      "end": 549
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_Zip7mifsjRY",
            "start": 556,
            "end": 569
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
                "value": "./test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY",
                "raw": "\"./test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY\"",
                "start": 583,
                "end": 647
              },
              "options": null,
              "phase": null,
              "start": 576,
              "end": 648
            },
            "id": null,
            "generator": false,
            "start": 572,
            "end": 648
          },
          "start": 556,
          "end": 648
        }
      ],
      "start": 550,
      "end": 649
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_cPEH970JbEY",
            "start": 656,
            "end": 669
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
                "value": "./test.tsx_Foo_component_div_host_onClick_cPEH970JbEY",
                "raw": "\"./test.tsx_Foo_component_div_host_onClick_cPEH970JbEY\"",
                "start": 683,
                "end": 738
              },
              "options": null,
              "phase": null,
              "start": 676,
              "end": 739
            },
            "id": null,
            "generator": false,
            "start": 672,
            "end": 739
          },
          "start": 656,
          "end": 739
        }
      ],
      "start": 650,
      "end": 740
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_d0Zn04qNgs0",
            "start": 747,
            "end": 760
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
                "value": "./test.tsx_Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0",
                "raw": "\"./test.tsx_Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0\"",
                "start": 774,
                "end": 837
              },
              "options": null,
              "phase": null,
              "start": 767,
              "end": 838
            },
            "id": null,
            "generator": false,
            "start": 763,
            "end": 838
          },
          "start": 747,
          "end": 838
        }
      ],
      "start": 741,
      "end": 839
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_kX5SiYdz650",
            "start": 846,
            "end": 859
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
                "value": "./test.tsx_Foo_component_div_q_e_c_lick_kX5SiYdz650",
                "raw": "\"./test.tsx_Foo_component_div_q_e_c_lick_kX5SiYdz650\"",
                "start": 873,
                "end": 926
              },
              "options": null,
              "phase": null,
              "start": 866,
              "end": 927
            },
            "id": null,
            "generator": false,
            "start": 862,
            "end": 927
          },
          "start": 846,
          "end": 927
        }
      ],
      "start": 840,
      "end": 928
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_pyHnxab17ms",
            "start": 935,
            "end": 948
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
                "value": "./test.tsx_Foo_component_div_custom_pyHnxab17ms",
                "raw": "\"./test.tsx_Foo_component_div_custom_pyHnxab17ms\"",
                "start": 962,
                "end": 1011
              },
              "options": null,
              "phase": null,
              "start": 955,
              "end": 1012
            },
            "id": null,
            "generator": false,
            "start": 951,
            "end": 1012
          },
          "start": 935,
          "end": 1012
        }
      ],
      "start": 929,
      "end": 1013
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_wphyTkeintI",
            "start": 1020,
            "end": 1033
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
                "value": "./test.tsx_Foo_component_div_q_e_document_scroll_1_wphyTkeintI",
                "raw": "\"./test.tsx_Foo_component_div_q_e_document_scroll_1_wphyTkeintI\"",
                "start": 1047,
                "end": 1111
              },
              "options": null,
              "phase": null,
              "start": 1040,
              "end": 1112
            },
            "id": null,
            "generator": false,
            "start": 1036,
            "end": 1112
          },
          "start": 1020,
          "end": 1112
        }
      ],
      "start": 1014,
      "end": 1113
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
              "start": 1127,
              "end": 1154
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
                          "name": "handler",
                          "start": 1173,
                          "end": 1180
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 1197,
                            "end": 1200
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_H10xZtD0e7w",
                              "start": 1201,
                              "end": 1214
                            },
                            {
                              "type": "Literal",
                              "value": "Foo_component_handler_H10xZtD0e7w",
                              "raw": "\"Foo_component_handler_H10xZtD0e7w\"",
                              "start": 1216,
                              "end": 1251
                            }
                          ],
                          "optional": false,
                          "start": 1197,
                          "end": 1252
                        },
                        "start": 1173,
                        "end": 1252
                      }
                    ],
                    "start": 1167,
                    "end": 1253
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 1279,
                        "end": 1289
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 1290,
                          "end": 1295
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1297,
                          "end": 1301
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
                                "start": 1313,
                                "end": 1324
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1340,
                                  "end": 1343
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_YEa2A5ADUOg",
                                    "start": 1344,
                                    "end": 1357
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_click_YEa2A5ADUOg",
                                    "raw": "\"Foo_component_div_q_e_click_YEa2A5ADUOg\"",
                                    "start": 1359,
                                    "end": 1400
                                  }
                                ],
                                "optional": false,
                                "start": 1340,
                                "end": 1401
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1313,
                              "end": 1401
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:documentscroll",
                                "raw": "\"q-e:documentscroll\"",
                                "start": 1411,
                                "end": 1431
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1447,
                                  "end": 1450
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_0FSbGzUROso",
                                    "start": 1451,
                                    "end": 1464
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_documentscroll_0FSbGzUROso",
                                    "raw": "\"Foo_component_div_q_e_documentscroll_0FSbGzUROso\"",
                                    "start": 1466,
                                    "end": 1516
                                  }
                                ],
                                "optional": false,
                                "start": 1447,
                                "end": 1517
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1411,
                              "end": 1517
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:documentscroll",
                                "raw": "\"q-e:documentscroll\"",
                                "start": 1527,
                                "end": 1547
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1563,
                                  "end": 1566
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_d0Zn04qNgs0",
                                    "start": 1567,
                                    "end": 1580
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0",
                                    "raw": "\"Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0\"",
                                    "start": 1582,
                                    "end": 1634
                                  }
                                ],
                                "optional": false,
                                "start": 1563,
                                "end": 1635
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1527,
                              "end": 1635
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:c-lick",
                                "raw": "\"q-e:c-lick\"",
                                "start": 1645,
                                "end": 1657
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1673,
                                  "end": 1676
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_kX5SiYdz650",
                                    "start": 1677,
                                    "end": 1690
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_c_lick_kX5SiYdz650",
                                    "raw": "\"Foo_component_div_q_e_c_lick_kX5SiYdz650\"",
                                    "start": 1692,
                                    "end": 1734
                                  }
                                ],
                                "optional": false,
                                "start": 1673,
                                "end": 1735
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1645,
                              "end": 1735
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:document--scroll",
                                "raw": "\"q-e:document--scroll\"",
                                "start": 1745,
                                "end": 1767
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1783,
                                  "end": 1786
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_6qyBttefepU",
                                    "start": 1787,
                                    "end": 1800
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_document_scroll_6qyBttefepU",
                                    "raw": "\"Foo_component_div_q_e_document_scroll_6qyBttefepU\"",
                                    "start": 1802,
                                    "end": 1853
                                  }
                                ],
                                "optional": false,
                                "start": 1783,
                                "end": 1854
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1745,
                              "end": 1854
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:document--scroll",
                                "raw": "\"q-e:document--scroll\"",
                                "start": 1864,
                                "end": 1886
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 1902,
                                  "end": 1905
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_wphyTkeintI",
                                    "start": 1906,
                                    "end": 1919
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_q_e_document_scroll_1_wphyTkeintI",
                                    "raw": "\"Foo_component_div_q_e_document_scroll_1_wphyTkeintI\"",
                                    "start": 1921,
                                    "end": 1974
                                  }
                                ],
                                "optional": false,
                                "start": 1902,
                                "end": 1975
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1864,
                              "end": 1975
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "host:onClick$",
                                "raw": "\"host:onClick$\"",
                                "start": 1985,
                                "end": 2000
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 2016,
                                  "end": 2019
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_cPEH970JbEY",
                                    "start": 2020,
                                    "end": 2033
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_host_onClick_cPEH970JbEY",
                                    "raw": "\"Foo_component_div_host_onClick_cPEH970JbEY\"",
                                    "start": 2035,
                                    "end": 2079
                                  }
                                ],
                                "optional": false,
                                "start": 2016,
                                "end": 2080
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 1985,
                              "end": 2080
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "host:onDocumentScroll$",
                                "raw": "\"host:onDocumentScroll$\"",
                                "start": 2090,
                                "end": 2114
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 2130,
                                  "end": 2133
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_Zip7mifsjRY",
                                    "start": 2134,
                                    "end": 2147
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_host_onDocumentScroll_Zip7mifsjRY",
                                    "raw": "\"Foo_component_div_host_onDocumentScroll_Zip7mifsjRY\"",
                                    "start": 2149,
                                    "end": 2202
                                  }
                                ],
                                "optional": false,
                                "start": 2130,
                                "end": 2203
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2090,
                              "end": 2203
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "host:onDocumentScroll$",
                                "raw": "\"host:onDocumentScroll$\"",
                                "start": 2213,
                                "end": 2237
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 2253,
                                  "end": 2256
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_Em1LspK7JVg",
                                    "start": 2257,
                                    "end": 2270
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg",
                                    "raw": "\"Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg\"",
                                    "start": 2272,
                                    "end": 2327
                                  }
                                ],
                                "optional": false,
                                "start": 2253,
                                "end": 2328
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2213,
                              "end": 2328
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:keyup",
                                "raw": "\"q-e:keyup\"",
                                "start": 2338,
                                "end": 2349
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "handler",
                                "start": 2351,
                                "end": 2358
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2338,
                              "end": 2358
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:document:keyup",
                                "raw": "\"q-e:document:keyup\"",
                                "start": 2368,
                                "end": 2388
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "handler",
                                "start": 2390,
                                "end": 2397
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2368,
                              "end": 2397
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:window:keyup",
                                "raw": "\"q-e:window:keyup\"",
                                "start": 2407,
                                "end": 2425
                              },
                              "value": {
                                "type": "Identifier",
                                "name": "handler",
                                "start": 2427,
                                "end": 2434
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2407,
                              "end": 2434
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "custom$",
                                "start": 2444,
                                "end": 2451
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 2467,
                                  "end": 2470
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_pyHnxab17ms",
                                    "start": 2471,
                                    "end": 2484
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "Foo_component_div_custom_pyHnxab17ms",
                                    "raw": "\"Foo_component_div_custom_pyHnxab17ms\"",
                                    "start": 2486,
                                    "end": 2524
                                  }
                                ],
                                "optional": false,
                                "start": 2467,
                                "end": 2525
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 2444,
                              "end": 2525
                            }
                          ],
                          "start": 1303,
                          "end": 2531
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2533,
                          "end": 2537
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 2539,
                          "end": 2540
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 2542,
                          "end": 2548
                        }
                      ],
                      "optional": false,
                      "start": 1279,
                      "end": 2549
                    },
                    "start": 1258,
                    "end": 2550
                  }
                ],
                "start": 1161,
                "end": 2552
              },
              "id": null,
              "generator": false,
              "start": 1157,
              "end": 2552
            },
            "start": 1127,
            "end": 2552
          }
        ],
        "start": 1121,
        "end": 2553
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 1114,
      "end": 2553
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 2553
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
  "captures": false,
  "loc": [
    100,
    841
  ]
}
```

### Module: test.tsx_Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg.js (ENTRY POINT)

```javascript
export const Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg = ()=>console.log('host:onWindow:scroll');
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
              "name": "Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg",
              "start": 13,
              "end": 66
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
                    "start": 73,
                    "end": 80
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 81,
                    "end": 84
                  },
                  "optional": false,
                  "computed": false,
                  "start": 73,
                  "end": 84
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "host:onWindow:scroll",
                    "raw": "'host:onWindow:scroll'",
                    "start": 85,
                    "end": 107
                  }
                ],
                "optional": false,
                "start": 73,
                "end": 108
              },
              "id": null,
              "generator": false,
              "start": 69,
              "end": 108
            },
            "start": 13,
            "end": 108
          }
        ],
        "start": 7,
        "end": 109
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 109
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 109
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_host_onDocumentScroll_1",
  "hash": "Em1LspK7JVg",
  "canonicalFilename": "test.tsx_Foo_component_div_host_onDocumentScroll_1_Em1LspK7JVg",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "host:onDocumentScroll$",
  "captures": false,
  "loc": [
    661,
    700
  ]
}
```

### Module: test.tsx_Foo_component_div_custom_pyHnxab17ms.js (ENTRY POINT)

```javascript
export const Foo_component_div_custom_pyHnxab17ms = ()=>console.log('custom');
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
              "name": "Foo_component_div_custom_pyHnxab17ms",
              "start": 13,
              "end": 49
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
                    "start": 56,
                    "end": 63
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 64,
                    "end": 67
                  },
                  "optional": false,
                  "computed": false,
                  "start": 56,
                  "end": 67
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "custom",
                    "raw": "'custom'",
                    "start": 68,
                    "end": 76
                  }
                ],
                "optional": false,
                "start": 56,
                "end": 77
              },
              "id": null,
              "generator": false,
              "start": 52,
              "end": 77
            },
            "start": 13,
            "end": 77
          }
        ],
        "start": 7,
        "end": 78
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 78
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 78
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_custom_pyHnxab17ms",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_custom",
  "hash": "pyHnxab17ms",
  "canonicalFilename": "test.tsx_Foo_component_div_custom_pyHnxab17ms",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "custom$",
  "captures": false,
  "loc": [
    802,
    827
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_document_scroll_6qyBttefepU.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_document_scroll_6qyBttefepU = ()=>console.log('onDocument-sCroll');
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
              "name": "Foo_component_div_q_e_document_scroll_6qyBttefepU",
              "start": 13,
              "end": 62
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
                    "start": 69,
                    "end": 76
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 77,
                    "end": 80
                  },
                  "optional": false,
                  "computed": false,
                  "start": 69,
                  "end": 80
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "onDocument-sCroll",
                    "raw": "'onDocument-sCroll'",
                    "start": 81,
                    "end": 100
                  }
                ],
                "optional": false,
                "start": 69,
                "end": 101
              },
              "id": null,
              "generator": false,
              "start": 65,
              "end": 101
            },
            "start": 13,
            "end": 101
          }
        ],
        "start": 7,
        "end": 102
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 102
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 102
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_document_scroll_6qyBttefepU",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_document_scroll",
  "hash": "6qyBttefepU",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_document_scroll_6qyBttefepU",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "onDocument-sCroll$",
  "captures": false,
  "loc": [
    408,
    444
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_c_lick_kX5SiYdz650.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_c_lick_kX5SiYdz650 = ()=>console.log('on-cLick$');
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
              "name": "Foo_component_div_q_e_c_lick_kX5SiYdz650",
              "start": 13,
              "end": 53
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
                    "start": 60,
                    "end": 67
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 68,
                    "end": 71
                  },
                  "optional": false,
                  "computed": false,
                  "start": 60,
                  "end": 71
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "on-cLick$",
                    "raw": "'on-cLick$'",
                    "start": 72,
                    "end": 83
                  }
                ],
                "optional": false,
                "start": 60,
                "end": 84
              },
              "id": null,
              "generator": false,
              "start": 56,
              "end": 84
            },
            "start": 13,
            "end": 84
          }
        ],
        "start": 7,
        "end": 85
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 85
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 85
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_c_lick_kX5SiYdz650",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_c_lick",
  "hash": "kX5SiYdz650",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_c_lick_kX5SiYdz650",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "on-cLick$",
  "captures": false,
  "loc": [
    354,
    382
  ]
}
```

### Module: test.tsx_Foo_component_handler_H10xZtD0e7w.js (ENTRY POINT)

```javascript
export const Foo_component_handler_H10xZtD0e7w = ()=>console.log('reused');
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
              "name": "Foo_component_handler_H10xZtD0e7w",
              "start": 13,
              "end": 46
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
                    "start": 53,
                    "end": 60
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 61,
                    "end": 64
                  },
                  "optional": false,
                  "computed": false,
                  "start": 53,
                  "end": 64
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "reused",
                    "raw": "'reused'",
                    "start": 65,
                    "end": 73
                  }
                ],
                "optional": false,
                "start": 53,
                "end": 74
              },
              "id": null,
              "generator": false,
              "start": 49,
              "end": 74
            },
            "start": 13,
            "end": 74
          }
        ],
        "start": 7,
        "end": 75
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 75
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 75
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_handler_H10xZtD0e7w",
  "entry": null,
  "displayName": "test.tsx_Foo_component_handler",
  "hash": "H10xZtD0e7w",
  "canonicalFilename": "test.tsx_Foo_component_handler_H10xZtD0e7w",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    128,
    155
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_documentscroll_0FSbGzUROso.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_documentscroll_0FSbGzUROso = ()=>console.log('onDocumentScroll');
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
              "name": "Foo_component_div_q_e_documentscroll_0FSbGzUROso",
              "start": 13,
              "end": 61
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
                    "start": 68,
                    "end": 75
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 76,
                    "end": 79
                  },
                  "optional": false,
                  "computed": false,
                  "start": 68,
                  "end": 79
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "onDocumentScroll",
                    "raw": "'onDocumentScroll'",
                    "start": 80,
                    "end": 98
                  }
                ],
                "optional": false,
                "start": 68,
                "end": 99
              },
              "id": null,
              "generator": false,
              "start": 64,
              "end": 99
            },
            "start": 13,
            "end": 99
          }
        ],
        "start": 7,
        "end": 100
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 100
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 100
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_documentscroll_0FSbGzUROso",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_documentscroll",
  "hash": "0FSbGzUROso",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_documentscroll_0FSbGzUROso",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "onDocumentScroll$",
  "captures": false,
  "loc": [
    243,
    278
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0 = ()=>console.log('onWindowScroll');
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
              "name": "Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0",
              "start": 13,
              "end": 63
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
                    "start": 70,
                    "end": 77
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 78,
                    "end": 81
                  },
                  "optional": false,
                  "computed": false,
                  "start": 70,
                  "end": 81
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "onWindowScroll",
                    "raw": "'onWindowScroll'",
                    "start": 82,
                    "end": 98
                  }
                ],
                "optional": false,
                "start": 70,
                "end": 99
              },
              "id": null,
              "generator": false,
              "start": 66,
              "end": 99
            },
            "start": 13,
            "end": 99
          }
        ],
        "start": 7,
        "end": 100
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 100
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 100
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_documentscroll_1",
  "hash": "d0Zn04qNgs0",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_documentscroll_1_d0Zn04qNgs0",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "onDocumentScroll$",
  "captures": false,
  "loc": [
    303,
    336
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_document_scroll_1_wphyTkeintI.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_document_scroll_1_wphyTkeintI = ()=>console.log('onDocument-scroLL');
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
              "name": "Foo_component_div_q_e_document_scroll_1_wphyTkeintI",
              "start": 13,
              "end": 64
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
                    "start": 71,
                    "end": 78
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 79,
                    "end": 82
                  },
                  "optional": false,
                  "computed": false,
                  "start": 71,
                  "end": 82
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "onDocument-scroLL",
                    "raw": "'onDocument-scroLL'",
                    "start": 83,
                    "end": 102
                  }
                ],
                "optional": false,
                "start": 71,
                "end": 103
              },
              "id": null,
              "generator": false,
              "start": 67,
              "end": 103
            },
            "start": 13,
            "end": 103
          }
        ],
        "start": 7,
        "end": 104
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 104
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 104
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_document_scroll_1_wphyTkeintI",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_document_scroll_1",
  "hash": "wphyTkeintI",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_document_scroll_1_wphyTkeintI",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "onDocument-scroLL$",
  "captures": false,
  "loc": [
    470,
    506
  ]
}
```

### Module: test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg.js (ENTRY POINT)

```javascript
export const Foo_component_div_q_e_click_YEa2A5ADUOg = ()=>console.log('onClick$');
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
              "name": "Foo_component_div_q_e_click_YEa2A5ADUOg",
              "start": 13,
              "end": 52
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
                    "start": 59,
                    "end": 66
                  },
                  "property": {
                    "type": "Identifier",
                    "name": "log",
                    "start": 67,
                    "end": 70
                  },
                  "optional": false,
                  "computed": false,
                  "start": 59,
                  "end": 70
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "onClick$",
                    "raw": "'onClick$'",
                    "start": 71,
                    "end": 81
                  }
                ],
                "optional": false,
                "start": 59,
                "end": 82
              },
              "id": null,
              "generator": false,
              "start": 55,
              "end": 82
            },
            "start": 13,
            "end": 82
          }
        ],
        "start": 7,
        "end": 83
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 0,
      "end": 83
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 83
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Foo_component_div_q_e_click_YEa2A5ADUOg",
  "entry": null,
  "displayName": "test.tsx_Foo_component_div_q_e_click",
  "hash": "YEa2A5ADUOg",
  "canonicalFilename": "test.tsx_Foo_component_div_q_e_click_YEa2A5ADUOg",
  "path": "",
  "extension": "js",
  "parent": "Foo_component_1_DvU6FitWglY",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": false,
  "loc": [
    191,
    218
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (13 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 15 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 13 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Foo_component_HTDRsvUbLiE.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 12 |
| `_jsxSorted` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
