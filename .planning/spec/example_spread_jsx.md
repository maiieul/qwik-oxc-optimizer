# Test: example_spread_jsx

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';
import { useDocumentHead, useLocation } from '@qwik.dev/router';

/**
 * The RouterHead component is placed inside of the document `<head>` element.
 */
export const RouterHead = component$(() => {
	const head = useDocumentHead();
	const loc = useLocation();

	return (
	<>
		<title>{head.title}</title>

		<link rel="canonical" href={loc.href} />
		<meta name="viewport" content="width=device-width, initial-scale=1.0" />
		<link rel="icon" type="image/svg+xml" href="/favicon.svg" />

		{head.meta.map((m) => (
			<meta {...m} />
		))}

		{head.links.map((l) => (
			<link {...l} key={l.key} />
		))}

		{head.styles.map((s) => (
			<style {...s.props} dangerouslySetInnerHTML={s.style} key={s.key} />
		))}
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useDocumentHead",
            "optional": false,
            "typeAnnotation": null,
            "start": 54,
            "end": 69
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useDocumentHead",
            "optional": false,
            "typeAnnotation": null,
            "start": 54,
            "end": 69
          },
          "importKind": "value",
          "start": 54,
          "end": 69
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLocation",
            "optional": false,
            "typeAnnotation": null,
            "start": 71,
            "end": 82
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLocation",
            "optional": false,
            "typeAnnotation": null,
            "start": 71,
            "end": 82
          },
          "importKind": "value",
          "start": 71,
          "end": 82
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/router",
        "raw": "'@qwik.dev/router'",
        "start": 90,
        "end": 108
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 45,
      "end": 109
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
              "name": "RouterHead",
              "optional": false,
              "typeAnnotation": null,
              "start": 211,
              "end": 221
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 224,
                "end": 234
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
                              "name": "head",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 250,
                              "end": 254
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useDocumentHead",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 257,
                                "end": 272
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 257,
                              "end": 274
                            },
                            "definite": false,
                            "start": 250,
                            "end": 274
                          }
                        ],
                        "declare": false,
                        "start": 244,
                        "end": 275
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
                              "name": "loc",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 283,
                              "end": 286
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useLocation",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 289,
                                "end": 300
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 289,
                              "end": 302
                            },
                            "definite": false,
                            "start": 283,
                            "end": 302
                          }
                        ],
                        "declare": false,
                        "start": 277,
                        "end": 303
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "ParenthesizedExpression",
                          "expression": {
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 316,
                              "end": 318
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 318,
                                "end": 321
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 322,
                                    "end": 327
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 321,
                                  "end": 328
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "head",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 329,
                                        "end": 333
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "title",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 334,
                                        "end": 339
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 329,
                                      "end": 339
                                    },
                                    "start": 328,
                                    "end": 340
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 342,
                                    "end": 347
                                  },
                                  "start": 340,
                                  "end": 348
                                },
                                "start": 321,
                                "end": 348
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t",
                                "raw": "\n\n\t\t",
                                "start": 348,
                                "end": 352
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "link",
                                    "start": 353,
                                    "end": 357
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "rel",
                                        "start": 358,
                                        "end": 361
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "canonical",
                                        "raw": "\"canonical\"",
                                        "start": 362,
                                        "end": 373
                                      },
                                      "start": 358,
                                      "end": 373
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "href",
                                        "start": 374,
                                        "end": 378
                                      },
                                      "value": {
                                        "type": "JSXExpressionContainer",
                                        "expression": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "loc",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 380,
                                            "end": 383
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "href",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 384,
                                            "end": 388
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 380,
                                          "end": 388
                                        },
                                        "start": 379,
                                        "end": 389
                                      },
                                      "start": 374,
                                      "end": 389
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 352,
                                  "end": 392
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 352,
                                "end": 392
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 392,
                                "end": 395
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "meta",
                                    "start": 396,
                                    "end": 400
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "name",
                                        "start": 401,
                                        "end": 405
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "viewport",
                                        "raw": "\"viewport\"",
                                        "start": 406,
                                        "end": 416
                                      },
                                      "start": 401,
                                      "end": 416
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "content",
                                        "start": 417,
                                        "end": 424
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "width=device-width, initial-scale=1.0",
                                        "raw": "\"width=device-width, initial-scale=1.0\"",
                                        "start": 425,
                                        "end": 464
                                      },
                                      "start": 417,
                                      "end": 464
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 395,
                                  "end": 467
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 395,
                                "end": 467
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 467,
                                "end": 470
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "link",
                                    "start": 471,
                                    "end": 475
                                  },
                                  "typeArguments": null,
                                  "attributes": [
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "rel",
                                        "start": 476,
                                        "end": 479
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "icon",
                                        "raw": "\"icon\"",
                                        "start": 480,
                                        "end": 486
                                      },
                                      "start": 476,
                                      "end": 486
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "type",
                                        "start": 487,
                                        "end": 491
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "image/svg+xml",
                                        "raw": "\"image/svg+xml\"",
                                        "start": 492,
                                        "end": 507
                                      },
                                      "start": 487,
                                      "end": 507
                                    },
                                    {
                                      "type": "JSXAttribute",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "href",
                                        "start": 508,
                                        "end": 512
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "/favicon.svg",
                                        "raw": "\"/favicon.svg\"",
                                        "start": 513,
                                        "end": 527
                                      },
                                      "start": 508,
                                      "end": 527
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 470,
                                  "end": 530
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 470,
                                "end": 530
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t",
                                "raw": "\n\n\t\t",
                                "start": 530,
                                "end": 534
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "head",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 535,
                                        "end": 539
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "meta",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 540,
                                        "end": 544
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 535,
                                      "end": 544
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "map",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 545,
                                      "end": 548
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 535,
                                    "end": 548
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "ArrowFunctionExpression",
                                      "expression": true,
                                      "async": false,
                                      "typeParameters": null,
                                      "params": [
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "m",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 550,
                                          "end": 551
                                        }
                                      ],
                                      "returnType": null,
                                      "body": {
                                        "type": "ParenthesizedExpression",
                                        "expression": {
                                          "type": "JSXElement",
                                          "openingElement": {
                                            "type": "JSXOpeningElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "meta",
                                              "start": 562,
                                              "end": 566
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXSpreadAttribute",
                                                "argument": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "m",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 571,
                                                  "end": 572
                                                },
                                                "start": 567,
                                                "end": 573
                                              }
                                            ],
                                            "selfClosing": true,
                                            "start": 561,
                                            "end": 576
                                          },
                                          "children": [],
                                          "closingElement": null,
                                          "start": 561,
                                          "end": 576
                                        },
                                        "start": 556,
                                        "end": 580
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 549,
                                      "end": 580
                                    }
                                  ],
                                  "optional": false,
                                  "start": 535,
                                  "end": 581
                                },
                                "start": 534,
                                "end": 582
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t",
                                "raw": "\n\n\t\t",
                                "start": 582,
                                "end": 586
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "head",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 587,
                                        "end": 591
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "links",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 592,
                                        "end": 597
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 587,
                                      "end": 597
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "map",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 598,
                                      "end": 601
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 587,
                                    "end": 601
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "ArrowFunctionExpression",
                                      "expression": true,
                                      "async": false,
                                      "typeParameters": null,
                                      "params": [
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "l",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 603,
                                          "end": 604
                                        }
                                      ],
                                      "returnType": null,
                                      "body": {
                                        "type": "ParenthesizedExpression",
                                        "expression": {
                                          "type": "JSXElement",
                                          "openingElement": {
                                            "type": "JSXOpeningElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "link",
                                              "start": 615,
                                              "end": 619
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXSpreadAttribute",
                                                "argument": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "l",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 624,
                                                  "end": 625
                                                },
                                                "start": 620,
                                                "end": 626
                                              },
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "key",
                                                  "start": 627,
                                                  "end": 630
                                                },
                                                "value": {
                                                  "type": "JSXExpressionContainer",
                                                  "expression": {
                                                    "type": "MemberExpression",
                                                    "object": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "l",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 632,
                                                      "end": 633
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "key",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 634,
                                                      "end": 637
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 632,
                                                    "end": 637
                                                  },
                                                  "start": 631,
                                                  "end": 638
                                                },
                                                "start": 627,
                                                "end": 638
                                              }
                                            ],
                                            "selfClosing": true,
                                            "start": 614,
                                            "end": 641
                                          },
                                          "children": [],
                                          "closingElement": null,
                                          "start": 614,
                                          "end": 641
                                        },
                                        "start": 609,
                                        "end": 645
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 602,
                                      "end": 645
                                    }
                                  ],
                                  "optional": false,
                                  "start": 587,
                                  "end": 646
                                },
                                "start": 586,
                                "end": 647
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t",
                                "raw": "\n\n\t\t",
                                "start": 647,
                                "end": 651
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "head",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 652,
                                        "end": 656
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "styles",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 657,
                                        "end": 663
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 652,
                                      "end": 663
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "map",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 664,
                                      "end": 667
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 652,
                                    "end": 667
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "ArrowFunctionExpression",
                                      "expression": true,
                                      "async": false,
                                      "typeParameters": null,
                                      "params": [
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "s",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 669,
                                          "end": 670
                                        }
                                      ],
                                      "returnType": null,
                                      "body": {
                                        "type": "ParenthesizedExpression",
                                        "expression": {
                                          "type": "JSXElement",
                                          "openingElement": {
                                            "type": "JSXOpeningElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "style",
                                              "start": 681,
                                              "end": 686
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXSpreadAttribute",
                                                "argument": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "s",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 691,
                                                    "end": 692
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "props",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 693,
                                                    "end": 698
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 691,
                                                  "end": 698
                                                },
                                                "start": 687,
                                                "end": 699
                                              },
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "dangerouslySetInnerHTML",
                                                  "start": 700,
                                                  "end": 723
                                                },
                                                "value": {
                                                  "type": "JSXExpressionContainer",
                                                  "expression": {
                                                    "type": "MemberExpression",
                                                    "object": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "s",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 725,
                                                      "end": 726
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "style",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 727,
                                                      "end": 732
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 725,
                                                    "end": 732
                                                  },
                                                  "start": 724,
                                                  "end": 733
                                                },
                                                "start": 700,
                                                "end": 733
                                              },
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "key",
                                                  "start": 734,
                                                  "end": 737
                                                },
                                                "value": {
                                                  "type": "JSXExpressionContainer",
                                                  "expression": {
                                                    "type": "MemberExpression",
                                                    "object": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "s",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 739,
                                                      "end": 740
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "key",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 741,
                                                      "end": 744
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 739,
                                                    "end": 744
                                                  },
                                                  "start": 738,
                                                  "end": 745
                                                },
                                                "start": 734,
                                                "end": 745
                                              }
                                            ],
                                            "selfClosing": true,
                                            "start": 680,
                                            "end": 748
                                          },
                                          "children": [],
                                          "closingElement": null,
                                          "start": 680,
                                          "end": 748
                                        },
                                        "start": 675,
                                        "end": 752
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 668,
                                      "end": 752
                                    }
                                  ],
                                  "optional": false,
                                  "start": 652,
                                  "end": 753
                                },
                                "start": 651,
                                "end": 754
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t",
                                "raw": "\n\t",
                                "start": 754,
                                "end": 756
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 756,
                              "end": 759
                            },
                            "start": 316,
                            "end": 759
                          },
                          "start": 313,
                          "end": 762
                        },
                        "start": 306,
                        "end": 763
                      }
                    ],
                    "start": 241,
                    "end": 765
                  },
                  "id": null,
                  "generator": false,
                  "start": 235,
                  "end": 765
                }
              ],
              "optional": false,
              "start": 224,
              "end": 766
            },
            "definite": false,
            "start": 211,
            "end": 766
          }
        ],
        "declare": false,
        "start": 205,
        "end": 767
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 198,
      "end": 767
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 767
}
```

</details>

## Output

### Module: `test.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_DPA76mgIou0 = ()=>import("./test.tsx_RouterHead_component_DPA76mgIou0");
/**
 * The RouterHead component is placed inside of the document `<head>` element.
 */ export const RouterHead = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_DPA76mgIou0, "RouterHead_component_DPA76mgIou0"));
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
            "name": "i_DPA76mgIou0",
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
                "value": "./test.tsx_RouterHead_component_DPA76mgIou0",
                "raw": "\"./test.tsx_RouterHead_component_DPA76mgIou0\"",
                "start": 118,
                "end": 163
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 164
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 164
          },
          "start": 91,
          "end": 164
        }
      ],
      "start": 85,
      "end": 165
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
              "name": "RouterHead",
              "start": 266,
              "end": 276
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 293,
                "end": 305
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 320,
                    "end": 323
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_DPA76mgIou0",
                      "start": 324,
                      "end": 337
                    },
                    {
                      "type": "Literal",
                      "value": "RouterHead_component_DPA76mgIou0",
                      "raw": "\"RouterHead_component_DPA76mgIou0\"",
                      "start": 339,
                      "end": 373
                    }
                  ],
                  "optional": false,
                  "start": 320,
                  "end": 374
                }
              ],
              "optional": false,
              "start": 293,
              "end": 375
            },
            "start": 266,
            "end": 375
          }
        ],
        "start": 260,
        "end": 376
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 253,
      "end": 376
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 376
}
```

</details>

### Module: `test.tsx_RouterHead_component_DPA76mgIou0.js` (ENTRY POINT)

```javascript
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { createElement as _createElement } from "@qwik.dev/core";
import { _getConstProps } from "@qwik.dev/core";
import { _getVarProps } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { _jsxSplit } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { useDocumentHead } from "@qwik.dev/router";
import { useLocation } from "@qwik.dev/router";
export const RouterHead_component_DPA76mgIou0 = ()=>{
    const head = useDocumentHead();
    const loc = useLocation();
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("title", null, null, head.title, 1, null),
        /*#__PURE__*/ _jsxSorted("link", null, {
            rel: "canonical",
            href: _wrapProp(loc, "href")
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("meta", null, {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0"
        }, null, 3, null),
        /*#__PURE__*/ _jsxSorted("link", null, {
            rel: "icon",
            type: "image/svg+xml",
            href: "/favicon.svg"
        }, null, 3, null),
        head.meta.map((m)=>/*#__PURE__*/ _jsxSplit("meta", {
                ..._getVarProps(m)
            }, _getConstProps(m), null, 0, "u6_0")),
        head.links.map((l)=>/*#__PURE__*/ _createElement("link", {
                ...l,
                key: l.key
            })),
        head.styles.map((s)=>/*#__PURE__*/ _createElement("style", {
                ...s.props,
                dangerouslySetInnerHTML: s.style,
                key: s.key
            }))
    ], 1, "u6_1");
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
            "name": "createElement",
            "start": 77,
            "end": 90
          },
          "local": {
            "type": "Identifier",
            "name": "_createElement",
            "start": 94,
            "end": 108
          },
          "start": 77,
          "end": 108
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 116,
        "end": 132
      },
      "phase": null,
      "attributes": [],
      "start": 68,
      "end": 133
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 143,
            "end": 157
          },
          "local": {
            "type": "Identifier",
            "name": "_getConstProps",
            "start": 143,
            "end": 157
          },
          "start": 143,
          "end": 157
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 165,
        "end": 181
      },
      "phase": null,
      "attributes": [],
      "start": 134,
      "end": 182
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 192,
            "end": 204
          },
          "local": {
            "type": "Identifier",
            "name": "_getVarProps",
            "start": 192,
            "end": 204
          },
          "start": 192,
          "end": 204
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 212,
        "end": 228
      },
      "phase": null,
      "attributes": [],
      "start": 183,
      "end": 229
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 239,
            "end": 249
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 239,
            "end": 249
          },
          "start": 239,
          "end": 249
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 257,
        "end": 273
      },
      "phase": null,
      "attributes": [],
      "start": 230,
      "end": 274
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 284,
            "end": 293
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSplit",
            "start": 284,
            "end": 293
          },
          "start": 284,
          "end": 293
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 301,
        "end": 317
      },
      "phase": null,
      "attributes": [],
      "start": 275,
      "end": 318
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 328,
            "end": 337
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 328,
            "end": 337
          },
          "start": 328,
          "end": 337
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 345,
        "end": 361
      },
      "phase": null,
      "attributes": [],
      "start": 319,
      "end": 362
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useDocumentHead",
            "start": 372,
            "end": 387
          },
          "local": {
            "type": "Identifier",
            "name": "useDocumentHead",
            "start": 372,
            "end": 387
          },
          "start": 372,
          "end": 387
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/router",
        "raw": "\"@qwik.dev/router\"",
        "start": 395,
        "end": 413
      },
      "phase": null,
      "attributes": [],
      "start": 363,
      "end": 414
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useLocation",
            "start": 424,
            "end": 435
          },
          "local": {
            "type": "Identifier",
            "name": "useLocation",
            "start": 424,
            "end": 435
          },
          "start": 424,
          "end": 435
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/router",
        "raw": "\"@qwik.dev/router\"",
        "start": 443,
        "end": 461
      },
      "phase": null,
      "attributes": [],
      "start": 415,
      "end": 462
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
              "name": "RouterHead_component_DPA76mgIou0",
              "start": 476,
              "end": 508
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
                          "name": "head",
                          "start": 527,
                          "end": 531
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useDocumentHead",
                            "start": 534,
                            "end": 549
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 534,
                          "end": 551
                        },
                        "start": 527,
                        "end": 551
                      }
                    ],
                    "start": 521,
                    "end": 552
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "loc",
                          "start": 563,
                          "end": 566
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useLocation",
                            "start": 569,
                            "end": 580
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 569,
                          "end": 582
                        },
                        "start": 563,
                        "end": 582
                      }
                    ],
                    "start": 557,
                    "end": 583
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 609,
                        "end": 619
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "_Fragment",
                          "start": 620,
                          "end": 629
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 631,
                          "end": 635
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 637,
                          "end": 641
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 667,
                                "end": 677
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "title",
                                  "raw": "\"title\"",
                                  "start": 678,
                                  "end": 685
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 687,
                                  "end": 691
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 693,
                                  "end": 697
                                },
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "head",
                                    "start": 699,
                                    "end": 703
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "title",
                                    "start": 704,
                                    "end": 709
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 699,
                                  "end": 709
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 711,
                                  "end": 712
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 714,
                                  "end": 718
                                }
                              ],
                              "optional": false,
                              "start": 667,
                              "end": 719
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 743,
                                "end": 753
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "link",
                                  "raw": "\"link\"",
                                  "start": 754,
                                  "end": 760
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 762,
                                  "end": 766
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "rel",
                                        "start": 782,
                                        "end": 785
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "canonical",
                                        "raw": "\"canonical\"",
                                        "start": 787,
                                        "end": 798
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 782,
                                      "end": 798
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "href",
                                        "start": 812,
                                        "end": 816
                                      },
                                      "value": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_wrapProp",
                                          "start": 818,
                                          "end": 827
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "loc",
                                            "start": 828,
                                            "end": 831
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "href",
                                            "raw": "\"href\"",
                                            "start": 833,
                                            "end": 839
                                          }
                                        ],
                                        "optional": false,
                                        "start": 818,
                                        "end": 840
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 812,
                                      "end": 840
                                    }
                                  ],
                                  "start": 768,
                                  "end": 850
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 852,
                                  "end": 856
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 858,
                                  "end": 859
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 861,
                                  "end": 865
                                }
                              ],
                              "optional": false,
                              "start": 743,
                              "end": 866
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 890,
                                "end": 900
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "meta",
                                  "raw": "\"meta\"",
                                  "start": 901,
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
                                        "name": "name",
                                        "start": 929,
                                        "end": 933
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "viewport",
                                        "raw": "\"viewport\"",
                                        "start": 935,
                                        "end": 945
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 929,
                                      "end": 945
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "content",
                                        "start": 959,
                                        "end": 966
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "width=device-width, initial-scale=1.0",
                                        "raw": "\"width=device-width, initial-scale=1.0\"",
                                        "start": 968,
                                        "end": 1007
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 959,
                                      "end": 1007
                                    }
                                  ],
                                  "start": 915,
                                  "end": 1017
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1019,
                                  "end": 1023
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1025,
                                  "end": 1026
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1028,
                                  "end": 1032
                                }
                              ],
                              "optional": false,
                              "start": 890,
                              "end": 1033
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1057,
                                "end": 1067
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "link",
                                  "raw": "\"link\"",
                                  "start": 1068,
                                  "end": 1074
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1076,
                                  "end": 1080
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "rel",
                                        "start": 1096,
                                        "end": 1099
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "icon",
                                        "raw": "\"icon\"",
                                        "start": 1101,
                                        "end": 1107
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1096,
                                      "end": 1107
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "type",
                                        "start": 1121,
                                        "end": 1125
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "image/svg+xml",
                                        "raw": "\"image/svg+xml\"",
                                        "start": 1127,
                                        "end": 1142
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1121,
                                      "end": 1142
                                    },
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "href",
                                        "start": 1156,
                                        "end": 1160
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": "/favicon.svg",
                                        "raw": "\"/favicon.svg\"",
                                        "start": 1162,
                                        "end": 1176
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 1156,
                                      "end": 1176
                                    }
                                  ],
                                  "start": 1082,
                                  "end": 1186
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1188,
                                  "end": 1192
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1194,
                                  "end": 1195
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1197,
                                  "end": 1201
                                }
                              ],
                              "optional": false,
                              "start": 1057,
                              "end": 1202
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "head",
                                    "start": 1212,
                                    "end": 1216
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "meta",
                                    "start": 1217,
                                    "end": 1221
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 1212,
                                  "end": 1221
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 1222,
                                  "end": 1225
                                },
                                "optional": false,
                                "computed": false,
                                "start": 1212,
                                "end": 1225
                              },
                              "arguments": [
                                {
                                  "type": "ArrowFunctionExpression",
                                  "expression": true,
                                  "async": false,
                                  "params": [
                                    {
                                      "type": "Identifier",
                                      "name": "m",
                                      "start": 1227,
                                      "end": 1228
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_jsxSplit",
                                      "start": 1245,
                                      "end": 1254
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "meta",
                                        "raw": "\"meta\"",
                                        "start": 1255,
                                        "end": 1261
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
                                                "start": 1284,
                                                "end": 1296
                                              },
                                              "arguments": [
                                                {
                                                  "type": "Identifier",
                                                  "name": "m",
                                                  "start": 1297,
                                                  "end": 1298
                                                }
                                              ],
                                              "optional": false,
                                              "start": 1284,
                                              "end": 1299
                                            },
                                            "start": 1281,
                                            "end": 1299
                                          }
                                        ],
                                        "start": 1263,
                                        "end": 1313
                                      },
                                      {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_getConstProps",
                                          "start": 1315,
                                          "end": 1329
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "m",
                                            "start": 1330,
                                            "end": 1331
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1315,
                                        "end": 1332
                                      },
                                      {
                                        "type": "Literal",
                                        "value": null,
                                        "raw": "null",
                                        "start": 1334,
                                        "end": 1338
                                      },
                                      {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 1340,
                                        "end": 1341
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "u6_0",
                                        "raw": "\"u6_0\"",
                                        "start": 1343,
                                        "end": 1349
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1245,
                                    "end": 1350
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 1226,
                                  "end": 1350
                                }
                              ],
                              "optional": false,
                              "start": 1212,
                              "end": 1351
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "head",
                                    "start": 1361,
                                    "end": 1365
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "links",
                                    "start": 1366,
                                    "end": 1371
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 1361,
                                  "end": 1371
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 1372,
                                  "end": 1375
                                },
                                "optional": false,
                                "computed": false,
                                "start": 1361,
                                "end": 1375
                              },
                              "arguments": [
                                {
                                  "type": "ArrowFunctionExpression",
                                  "expression": true,
                                  "async": false,
                                  "params": [
                                    {
                                      "type": "Identifier",
                                      "name": "l",
                                      "start": 1377,
                                      "end": 1378
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_createElement",
                                      "start": 1395,
                                      "end": 1409
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "link",
                                        "raw": "\"link\"",
                                        "start": 1410,
                                        "end": 1416
                                      },
                                      {
                                        "type": "ObjectExpression",
                                        "properties": [
                                          {
                                            "type": "SpreadElement",
                                            "argument": {
                                              "type": "Identifier",
                                              "name": "l",
                                              "start": 1439,
                                              "end": 1440
                                            },
                                            "start": 1436,
                                            "end": 1440
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "name": "key",
                                              "start": 1458,
                                              "end": 1461
                                            },
                                            "value": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "name": "l",
                                                "start": 1463,
                                                "end": 1464
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "name": "key",
                                                "start": 1465,
                                                "end": 1468
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 1463,
                                              "end": 1468
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1458,
                                            "end": 1468
                                          }
                                        ],
                                        "start": 1418,
                                        "end": 1482
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1395,
                                    "end": 1483
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 1376,
                                  "end": 1483
                                }
                              ],
                              "optional": false,
                              "start": 1361,
                              "end": 1484
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "head",
                                    "start": 1494,
                                    "end": 1498
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "styles",
                                    "start": 1499,
                                    "end": 1505
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 1494,
                                  "end": 1505
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 1506,
                                  "end": 1509
                                },
                                "optional": false,
                                "computed": false,
                                "start": 1494,
                                "end": 1509
                              },
                              "arguments": [
                                {
                                  "type": "ArrowFunctionExpression",
                                  "expression": true,
                                  "async": false,
                                  "params": [
                                    {
                                      "type": "Identifier",
                                      "name": "s",
                                      "start": 1511,
                                      "end": 1512
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_createElement",
                                      "start": 1529,
                                      "end": 1543
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "style",
                                        "raw": "\"style\"",
                                        "start": 1544,
                                        "end": 1551
                                      },
                                      {
                                        "type": "ObjectExpression",
                                        "properties": [
                                          {
                                            "type": "SpreadElement",
                                            "argument": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "name": "s",
                                                "start": 1574,
                                                "end": 1575
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "name": "props",
                                                "start": 1576,
                                                "end": 1581
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 1574,
                                              "end": 1581
                                            },
                                            "start": 1571,
                                            "end": 1581
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "name": "dangerouslySetInnerHTML",
                                              "start": 1599,
                                              "end": 1622
                                            },
                                            "value": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "name": "s",
                                                "start": 1624,
                                                "end": 1625
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "name": "style",
                                                "start": 1626,
                                                "end": 1631
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 1624,
                                              "end": 1631
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1599,
                                            "end": 1631
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "name": "key",
                                              "start": 1649,
                                              "end": 1652
                                            },
                                            "value": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "name": "s",
                                                "start": 1654,
                                                "end": 1655
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "name": "key",
                                                "start": 1656,
                                                "end": 1659
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 1654,
                                              "end": 1659
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1649,
                                            "end": 1659
                                          }
                                        ],
                                        "start": 1553,
                                        "end": 1673
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1529,
                                    "end": 1674
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 1510,
                                  "end": 1674
                                }
                              ],
                              "optional": false,
                              "start": 1494,
                              "end": 1675
                            }
                          ],
                          "start": 643,
                          "end": 1681
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 1683,
                          "end": 1684
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 1686,
                          "end": 1692
                        }
                      ],
                      "optional": false,
                      "start": 609,
                      "end": 1693
                    },
                    "start": 588,
                    "end": 1694
                  }
                ],
                "start": 515,
                "end": 1696
              },
              "id": null,
              "generator": false,
              "start": 511,
              "end": 1696
            },
            "start": 476,
            "end": 1696
          }
        ],
        "start": 470,
        "end": 1697
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 463,
      "end": 1697
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1697
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "RouterHead_component_DPA76mgIou0",
  "entry": null,
  "displayName": "test.tsx_RouterHead_component",
  "hash": "DPA76mgIou0",
  "canonicalFilename": "test.tsx_RouterHead_component_DPA76mgIou0",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    237,
    767
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`, `_jsxSplit()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_wrapProp()`, `_getVarProps()`, `_getConstProps()`
- **[CONV-06] Lazy Imports**: Lazy import declaration (`const i_HASH = () => import(...)`) for deferred module loading (1 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 10 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_RouterHead_component_DPA76mgIou0.js` | `@qwik.dev/core` | 6 |
| `_jsxSplit` | `test.tsx_RouterHead_component_DPA76mgIou0.js` | `@qwik.dev/core` | 2 |
| `_wrapProp` | `test.tsx_RouterHead_component_DPA76mgIou0.js` | `@qwik.dev/core` | 2 |
| `_getVarProps` | `test.tsx_RouterHead_component_DPA76mgIou0.js` | `@qwik.dev/core` | 2 |
| `_getConstProps` | `test.tsx_RouterHead_component_DPA76mgIou0.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
