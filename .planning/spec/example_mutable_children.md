# Test: example_mutable_children

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Hoist |
| Transpile Ts | True |
| Transpile Jsx | True |
| Explicit Extensions | True |

## Input

### Source Code

```tsx
import { component$, useStore, Slot, Fragment } from '@qwik.dev/core';
import Image from './image.jpg?jsx';

export function Fn1(props: Stuff) {
	return (
		<>
			<div>{prop < 2 ? <p>1</p> : <Stuff>2</Stuff>}</div>
		</>
	);
}

export function Fn2(props: Stuff) {
	return (
		<div>{prop.value && <Stuff></Stuff>}<div></div></div>
	);
}

export function Fn3(props: Stuff) {
	if (prop.value) {
		return (
			<Stuff></Stuff>
		);
	}
	return (
		<div></div>
	);
}

export function Fn4(props: Stuff) {
	if (prop.value) {
		return (
			<div></div>
		);
	}
	return (
		<Stuff></Stuff>
	);
}

export const Arrow = (props: Stuff) => <div>{prop < 2 ? <p>1</p> : <Stuff>2</Stuff>}</div>;

export const AppDynamic1 = component$((props: Stuff) => {
	return (
		<>
			<div>{prop < 2 ? <p>1</p> : <Stuff>2</Stuff>}</div>
		</>
	);
});
export const AppDynamic2 = component$((props: Stuff) => {
	return (
		<div>{prop.value && <Stuff></Stuff>}<div></div></div>
	);
});

export const AppDynamic3 = component$((props: Stuff) => {
	if (prop.value) {
		return (
			<Stuff></Stuff>
		);
	}
	return (
		<div></div>
	);
});

export const AppDynamic4 = component$((props: Stuff) => {
	if (prop.value) {
		return (
			<div></div>
		);
	}
	return (
		<Stuff></Stuff>
	);
});

export const AppStatic = component$((props: Stuff) => {
	return (
		<>
			<div>Static {f ? 1 : 3}</div>
			<div>{prop < 2 ? <p>1</p> : <p>2</p>}</div>

			<div>{prop.value && <div></div>}</div>
			<div>{prop.value && <Fragment><Slot></Slot></Fragment>}</div>
			<div>{prop.value && <><div></div></>}</div>
			<div>{prop.value && <Image/>}</div>
			<div>Static {f ? 1 : 3}</div>
			<div>Static</div>
			<div>Static {props.value}</div>
			<div>Static {stuff()}</div>
			<div>Static {stuff()}</div>
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
            "name": "Slot",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 35
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Slot",
            "optional": false,
            "typeAnnotation": null,
            "start": 31,
            "end": 35
          },
          "importKind": "value",
          "start": 31,
          "end": 35
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "Fragment",
            "optional": false,
            "typeAnnotation": null,
            "start": 37,
            "end": 45
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Fragment",
            "optional": false,
            "typeAnnotation": null,
            "start": 37,
            "end": 45
          },
          "importKind": "value",
          "start": 37,
          "end": 45
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 53,
        "end": 69
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 70
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Image",
            "optional": false,
            "typeAnnotation": null,
            "start": 78,
            "end": 83
          },
          "start": 78,
          "end": 83
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./image.jpg?jsx",
        "raw": "'./image.jpg?jsx'",
        "start": 89,
        "end": 106
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 71,
      "end": 107
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Fn1",
          "optional": false,
          "typeAnnotation": null,
          "start": 125,
          "end": 128
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "Identifier",
            "decorators": [],
            "name": "props",
            "optional": false,
            "typeAnnotation": {
              "type": "TSTypeAnnotation",
              "typeAnnotation": {
                "type": "TSTypeReference",
                "typeName": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "Stuff",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 136,
                  "end": 141
                },
                "typeArguments": null,
                "start": 136,
                "end": 141
              },
              "start": 134,
              "end": 141
            },
            "start": 129,
            "end": 141
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
                  "type": "JSXFragment",
                  "openingFragment": {
                    "type": "JSXOpeningFragment",
                    "start": 157,
                    "end": 159
                  },
                  "children": [
                    {
                      "type": "JSXText",
                      "value": "\n\t\t\t",
                      "raw": "\n\t\t\t",
                      "start": 159,
                      "end": 163
                    },
                    {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 164,
                          "end": 167
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 163,
                        "end": 168
                      },
                      "children": [
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "ConditionalExpression",
                            "test": {
                              "type": "BinaryExpression",
                              "left": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "prop",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 169,
                                "end": 173
                              },
                              "operator": "<",
                              "right": {
                                "type": "Literal",
                                "value": 2,
                                "raw": "2",
                                "start": 176,
                                "end": 177
                              },
                              "start": 169,
                              "end": 177
                            },
                            "consequent": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "p",
                                  "start": 181,
                                  "end": 182
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 180,
                                "end": 183
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "1",
                                  "raw": "1",
                                  "start": 183,
                                  "end": 184
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "p",
                                  "start": 186,
                                  "end": 187
                                },
                                "start": 184,
                                "end": 188
                              },
                              "start": 180,
                              "end": 188
                            },
                            "alternate": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "Stuff",
                                  "start": 192,
                                  "end": 197
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 191,
                                "end": 198
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "2",
                                  "raw": "2",
                                  "start": 198,
                                  "end": 199
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "Stuff",
                                  "start": 201,
                                  "end": 206
                                },
                                "start": 199,
                                "end": 207
                              },
                              "start": 191,
                              "end": 207
                            },
                            "start": 169,
                            "end": 207
                          },
                          "start": 168,
                          "end": 208
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 210,
                          "end": 213
                        },
                        "start": 208,
                        "end": 214
                      },
                      "start": 163,
                      "end": 214
                    },
                    {
                      "type": "JSXText",
                      "value": "\n\t\t",
                      "raw": "\n\t\t",
                      "start": 214,
                      "end": 217
                    }
                  ],
                  "closingFragment": {
                    "type": "JSXClosingFragment",
                    "start": 217,
                    "end": 220
                  },
                  "start": 157,
                  "end": 220
                },
                "start": 153,
                "end": 223
              },
              "start": 146,
              "end": 224
            }
          ],
          "start": 143,
          "end": 226
        },
        "expression": false,
        "start": 116,
        "end": 226
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 109,
      "end": 226
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Fn2",
          "optional": false,
          "typeAnnotation": null,
          "start": 244,
          "end": 247
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "Identifier",
            "decorators": [],
            "name": "props",
            "optional": false,
            "typeAnnotation": {
              "type": "TSTypeAnnotation",
              "typeAnnotation": {
                "type": "TSTypeReference",
                "typeName": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "Stuff",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 255,
                  "end": 260
                },
                "typeArguments": null,
                "start": 255,
                "end": 260
              },
              "start": 253,
              "end": 260
            },
            "start": 248,
            "end": 260
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
                      "start": 277,
                      "end": 280
                    },
                    "typeArguments": null,
                    "attributes": [],
                    "selfClosing": false,
                    "start": 276,
                    "end": 281
                  },
                  "children": [
                    {
                      "type": "JSXExpressionContainer",
                      "expression": {
                        "type": "LogicalExpression",
                        "left": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "prop",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 282,
                            "end": 286
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "value",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 287,
                            "end": 292
                          },
                          "optional": false,
                          "computed": false,
                          "start": 282,
                          "end": 292
                        },
                        "operator": "&&",
                        "right": {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "Stuff",
                              "start": 297,
                              "end": 302
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 296,
                            "end": 303
                          },
                          "children": [],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "Stuff",
                              "start": 305,
                              "end": 310
                            },
                            "start": 303,
                            "end": 311
                          },
                          "start": 296,
                          "end": 311
                        },
                        "start": 282,
                        "end": 311
                      },
                      "start": 281,
                      "end": 312
                    },
                    {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 313,
                          "end": 316
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 312,
                        "end": 317
                      },
                      "children": [],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 319,
                          "end": 322
                        },
                        "start": 317,
                        "end": 323
                      },
                      "start": 312,
                      "end": 323
                    }
                  ],
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
                  "start": 276,
                  "end": 329
                },
                "start": 272,
                "end": 332
              },
              "start": 265,
              "end": 333
            }
          ],
          "start": 262,
          "end": 335
        },
        "expression": false,
        "start": 235,
        "end": 335
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 228,
      "end": 335
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Fn3",
          "optional": false,
          "typeAnnotation": null,
          "start": 353,
          "end": 356
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "Identifier",
            "decorators": [],
            "name": "props",
            "optional": false,
            "typeAnnotation": {
              "type": "TSTypeAnnotation",
              "typeAnnotation": {
                "type": "TSTypeReference",
                "typeName": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "Stuff",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 364,
                  "end": 369
                },
                "typeArguments": null,
                "start": 364,
                "end": 369
              },
              "start": 362,
              "end": 369
            },
            "start": 357,
            "end": 369
          }
        ],
        "returnType": null,
        "body": {
          "type": "BlockStatement",
          "body": [
            {
              "type": "IfStatement",
              "test": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "prop",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 378,
                  "end": 382
                },
                "property": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "value",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 383,
                  "end": 388
                },
                "optional": false,
                "computed": false,
                "start": 378,
                "end": 388
              },
              "consequent": {
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
                            "name": "Stuff",
                            "start": 407,
                            "end": 412
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 406,
                          "end": 413
                        },
                        "children": [],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "Stuff",
                            "start": 415,
                            "end": 420
                          },
                          "start": 413,
                          "end": 421
                        },
                        "start": 406,
                        "end": 421
                      },
                      "start": 401,
                      "end": 425
                    },
                    "start": 394,
                    "end": 426
                  }
                ],
                "start": 390,
                "end": 429
              },
              "alternate": null,
              "start": 374,
              "end": 429
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
                      "start": 443,
                      "end": 446
                    },
                    "typeArguments": null,
                    "attributes": [],
                    "selfClosing": false,
                    "start": 442,
                    "end": 447
                  },
                  "children": [],
                  "closingElement": {
                    "type": "JSXClosingElement",
                    "name": {
                      "type": "JSXIdentifier",
                      "name": "div",
                      "start": 449,
                      "end": 452
                    },
                    "start": 447,
                    "end": 453
                  },
                  "start": 442,
                  "end": 453
                },
                "start": 438,
                "end": 456
              },
              "start": 431,
              "end": 457
            }
          ],
          "start": 371,
          "end": 459
        },
        "expression": false,
        "start": 344,
        "end": 459
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 337,
      "end": 459
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Fn4",
          "optional": false,
          "typeAnnotation": null,
          "start": 477,
          "end": 480
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "Identifier",
            "decorators": [],
            "name": "props",
            "optional": false,
            "typeAnnotation": {
              "type": "TSTypeAnnotation",
              "typeAnnotation": {
                "type": "TSTypeReference",
                "typeName": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "Stuff",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 488,
                  "end": 493
                },
                "typeArguments": null,
                "start": 488,
                "end": 493
              },
              "start": 486,
              "end": 493
            },
            "start": 481,
            "end": 493
          }
        ],
        "returnType": null,
        "body": {
          "type": "BlockStatement",
          "body": [
            {
              "type": "IfStatement",
              "test": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "prop",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 502,
                  "end": 506
                },
                "property": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "value",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 507,
                  "end": 512
                },
                "optional": false,
                "computed": false,
                "start": 502,
                "end": 512
              },
              "consequent": {
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
                            "start": 531,
                            "end": 534
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 530,
                          "end": 535
                        },
                        "children": [],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "div",
                            "start": 537,
                            "end": 540
                          },
                          "start": 535,
                          "end": 541
                        },
                        "start": 530,
                        "end": 541
                      },
                      "start": 525,
                      "end": 545
                    },
                    "start": 518,
                    "end": 546
                  }
                ],
                "start": 514,
                "end": 549
              },
              "alternate": null,
              "start": 498,
              "end": 549
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
                      "name": "Stuff",
                      "start": 563,
                      "end": 568
                    },
                    "typeArguments": null,
                    "attributes": [],
                    "selfClosing": false,
                    "start": 562,
                    "end": 569
                  },
                  "children": [],
                  "closingElement": {
                    "type": "JSXClosingElement",
                    "name": {
                      "type": "JSXIdentifier",
                      "name": "Stuff",
                      "start": 571,
                      "end": 576
                    },
                    "start": 569,
                    "end": 577
                  },
                  "start": 562,
                  "end": 577
                },
                "start": 558,
                "end": 580
              },
              "start": 551,
              "end": 581
            }
          ],
          "start": 495,
          "end": 583
        },
        "expression": false,
        "start": 468,
        "end": 583
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 461,
      "end": 583
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
              "name": "Arrow",
              "optional": false,
              "typeAnnotation": null,
              "start": 598,
              "end": 603
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "typeParameters": null,
              "params": [
                {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "props",
                  "optional": false,
                  "typeAnnotation": {
                    "type": "TSTypeAnnotation",
                    "typeAnnotation": {
                      "type": "TSTypeReference",
                      "typeName": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "Stuff",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 614,
                        "end": 619
                      },
                      "typeArguments": null,
                      "start": 614,
                      "end": 619
                    },
                    "start": 612,
                    "end": 619
                  },
                  "start": 607,
                  "end": 619
                }
              ],
              "returnType": null,
              "body": {
                "type": "JSXElement",
                "openingElement": {
                  "type": "JSXOpeningElement",
                  "name": {
                    "type": "JSXIdentifier",
                    "name": "div",
                    "start": 625,
                    "end": 628
                  },
                  "typeArguments": null,
                  "attributes": [],
                  "selfClosing": false,
                  "start": 624,
                  "end": 629
                },
                "children": [
                  {
                    "type": "JSXExpressionContainer",
                    "expression": {
                      "type": "ConditionalExpression",
                      "test": {
                        "type": "BinaryExpression",
                        "left": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "prop",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 630,
                          "end": 634
                        },
                        "operator": "<",
                        "right": {
                          "type": "Literal",
                          "value": 2,
                          "raw": "2",
                          "start": 637,
                          "end": 638
                        },
                        "start": 630,
                        "end": 638
                      },
                      "consequent": {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "p",
                            "start": 642,
                            "end": 643
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 641,
                          "end": 644
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "1",
                            "raw": "1",
                            "start": 644,
                            "end": 645
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "p",
                            "start": 647,
                            "end": 648
                          },
                          "start": 645,
                          "end": 649
                        },
                        "start": 641,
                        "end": 649
                      },
                      "alternate": {
                        "type": "JSXElement",
                        "openingElement": {
                          "type": "JSXOpeningElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "Stuff",
                            "start": 653,
                            "end": 658
                          },
                          "typeArguments": null,
                          "attributes": [],
                          "selfClosing": false,
                          "start": 652,
                          "end": 659
                        },
                        "children": [
                          {
                            "type": "JSXText",
                            "value": "2",
                            "raw": "2",
                            "start": 659,
                            "end": 660
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "Stuff",
                            "start": 662,
                            "end": 667
                          },
                          "start": 660,
                          "end": 668
                        },
                        "start": 652,
                        "end": 668
                      },
                      "start": 630,
                      "end": 668
                    },
                    "start": 629,
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
                "start": 624,
                "end": 675
              },
              "id": null,
              "generator": false,
              "start": 606,
              "end": 675
            },
            "definite": false,
            "start": 598,
            "end": 675
          }
        ],
        "declare": false,
        "start": 592,
        "end": 676
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 585,
      "end": 676
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
              "name": "AppDynamic1",
              "optional": false,
              "typeAnnotation": null,
              "start": 691,
              "end": 702
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 705,
                "end": 715
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
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 724,
                            "end": 729
                          },
                          "typeArguments": null,
                          "start": 724,
                          "end": 729
                        },
                        "start": 722,
                        "end": 729
                      },
                      "start": 717,
                      "end": 729
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
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 748,
                              "end": 750
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 750,
                                "end": 754
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 755,
                                    "end": 758
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 754,
                                  "end": 759
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "BinaryExpression",
                                        "left": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 760,
                                          "end": 764
                                        },
                                        "operator": "<",
                                        "right": {
                                          "type": "Literal",
                                          "value": 2,
                                          "raw": "2",
                                          "start": 767,
                                          "end": 768
                                        },
                                        "start": 760,
                                        "end": 768
                                      },
                                      "consequent": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 772,
                                            "end": 773
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 771,
                                          "end": 774
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "1",
                                            "raw": "1",
                                            "start": 774,
                                            "end": 775
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 777,
                                            "end": 778
                                          },
                                          "start": 775,
                                          "end": 779
                                        },
                                        "start": 771,
                                        "end": 779
                                      },
                                      "alternate": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "Stuff",
                                            "start": 783,
                                            "end": 788
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 782,
                                          "end": 789
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "2",
                                            "raw": "2",
                                            "start": 789,
                                            "end": 790
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "Stuff",
                                            "start": 792,
                                            "end": 797
                                          },
                                          "start": 790,
                                          "end": 798
                                        },
                                        "start": 782,
                                        "end": 798
                                      },
                                      "start": 760,
                                      "end": 798
                                    },
                                    "start": 759,
                                    "end": 799
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 801,
                                    "end": 804
                                  },
                                  "start": 799,
                                  "end": 805
                                },
                                "start": 754,
                                "end": 805
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 805,
                                "end": 808
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 808,
                              "end": 811
                            },
                            "start": 748,
                            "end": 811
                          },
                          "start": 744,
                          "end": 814
                        },
                        "start": 737,
                        "end": 815
                      }
                    ],
                    "start": 734,
                    "end": 817
                  },
                  "id": null,
                  "generator": false,
                  "start": 716,
                  "end": 817
                }
              ],
              "optional": false,
              "start": 705,
              "end": 818
            },
            "definite": false,
            "start": 691,
            "end": 818
          }
        ],
        "declare": false,
        "start": 685,
        "end": 819
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 678,
      "end": 819
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
              "name": "AppDynamic2",
              "optional": false,
              "typeAnnotation": null,
              "start": 833,
              "end": 844
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 847,
                "end": 857
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
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 866,
                            "end": 871
                          },
                          "typeArguments": null,
                          "start": 866,
                          "end": 871
                        },
                        "start": 864,
                        "end": 871
                      },
                      "start": 859,
                      "end": 871
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
                                "start": 891,
                                "end": 894
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 890,
                              "end": 895
                            },
                            "children": [
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "LogicalExpression",
                                  "left": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "prop",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 896,
                                      "end": 900
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "value",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 901,
                                      "end": 906
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 896,
                                    "end": 906
                                  },
                                  "operator": "&&",
                                  "right": {
                                    "type": "JSXElement",
                                    "openingElement": {
                                      "type": "JSXOpeningElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "Stuff",
                                        "start": 911,
                                        "end": 916
                                      },
                                      "typeArguments": null,
                                      "attributes": [],
                                      "selfClosing": false,
                                      "start": 910,
                                      "end": 917
                                    },
                                    "children": [],
                                    "closingElement": {
                                      "type": "JSXClosingElement",
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "Stuff",
                                        "start": 919,
                                        "end": 924
                                      },
                                      "start": 917,
                                      "end": 925
                                    },
                                    "start": 910,
                                    "end": 925
                                  },
                                  "start": 896,
                                  "end": 925
                                },
                                "start": 895,
                                "end": 926
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 927,
                                    "end": 930
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 926,
                                  "end": 931
                                },
                                "children": [],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 933,
                                    "end": 936
                                  },
                                  "start": 931,
                                  "end": 937
                                },
                                "start": 926,
                                "end": 937
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 939,
                                "end": 942
                              },
                              "start": 937,
                              "end": 943
                            },
                            "start": 890,
                            "end": 943
                          },
                          "start": 886,
                          "end": 946
                        },
                        "start": 879,
                        "end": 947
                      }
                    ],
                    "start": 876,
                    "end": 949
                  },
                  "id": null,
                  "generator": false,
                  "start": 858,
                  "end": 949
                }
              ],
              "optional": false,
              "start": 847,
              "end": 950
            },
            "definite": false,
            "start": 833,
            "end": 950
          }
        ],
        "declare": false,
        "start": 827,
        "end": 951
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 820,
      "end": 951
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
              "name": "AppDynamic3",
              "optional": false,
              "typeAnnotation": null,
              "start": 966,
              "end": 977
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 980,
                "end": 990
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
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 999,
                            "end": 1004
                          },
                          "typeArguments": null,
                          "start": 999,
                          "end": 1004
                        },
                        "start": 997,
                        "end": 1004
                      },
                      "start": 992,
                      "end": 1004
                    }
                  ],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
                      {
                        "type": "IfStatement",
                        "test": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "prop",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1016,
                            "end": 1020
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "value",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1021,
                            "end": 1026
                          },
                          "optional": false,
                          "computed": false,
                          "start": 1016,
                          "end": 1026
                        },
                        "consequent": {
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
                                      "name": "Stuff",
                                      "start": 1045,
                                      "end": 1050
                                    },
                                    "typeArguments": null,
                                    "attributes": [],
                                    "selfClosing": false,
                                    "start": 1044,
                                    "end": 1051
                                  },
                                  "children": [],
                                  "closingElement": {
                                    "type": "JSXClosingElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "Stuff",
                                      "start": 1053,
                                      "end": 1058
                                    },
                                    "start": 1051,
                                    "end": 1059
                                  },
                                  "start": 1044,
                                  "end": 1059
                                },
                                "start": 1039,
                                "end": 1063
                              },
                              "start": 1032,
                              "end": 1064
                            }
                          ],
                          "start": 1028,
                          "end": 1067
                        },
                        "alternate": null,
                        "start": 1012,
                        "end": 1067
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
                                "start": 1081,
                                "end": 1084
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 1080,
                              "end": 1085
                            },
                            "children": [],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 1087,
                                "end": 1090
                              },
                              "start": 1085,
                              "end": 1091
                            },
                            "start": 1080,
                            "end": 1091
                          },
                          "start": 1076,
                          "end": 1094
                        },
                        "start": 1069,
                        "end": 1095
                      }
                    ],
                    "start": 1009,
                    "end": 1097
                  },
                  "id": null,
                  "generator": false,
                  "start": 991,
                  "end": 1097
                }
              ],
              "optional": false,
              "start": 980,
              "end": 1098
            },
            "definite": false,
            "start": 966,
            "end": 1098
          }
        ],
        "declare": false,
        "start": 960,
        "end": 1099
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 953,
      "end": 1099
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
              "name": "AppDynamic4",
              "optional": false,
              "typeAnnotation": null,
              "start": 1114,
              "end": 1125
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 1128,
                "end": 1138
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
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1147,
                            "end": 1152
                          },
                          "typeArguments": null,
                          "start": 1147,
                          "end": 1152
                        },
                        "start": 1145,
                        "end": 1152
                      },
                      "start": 1140,
                      "end": 1152
                    }
                  ],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
                      {
                        "type": "IfStatement",
                        "test": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "prop",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1164,
                            "end": 1168
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "value",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1169,
                            "end": 1174
                          },
                          "optional": false,
                          "computed": false,
                          "start": 1164,
                          "end": 1174
                        },
                        "consequent": {
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
                                      "start": 1193,
                                      "end": 1196
                                    },
                                    "typeArguments": null,
                                    "attributes": [],
                                    "selfClosing": false,
                                    "start": 1192,
                                    "end": 1197
                                  },
                                  "children": [],
                                  "closingElement": {
                                    "type": "JSXClosingElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "div",
                                      "start": 1199,
                                      "end": 1202
                                    },
                                    "start": 1197,
                                    "end": 1203
                                  },
                                  "start": 1192,
                                  "end": 1203
                                },
                                "start": 1187,
                                "end": 1207
                              },
                              "start": 1180,
                              "end": 1208
                            }
                          ],
                          "start": 1176,
                          "end": 1211
                        },
                        "alternate": null,
                        "start": 1160,
                        "end": 1211
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
                                "name": "Stuff",
                                "start": 1225,
                                "end": 1230
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 1224,
                              "end": 1231
                            },
                            "children": [],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "Stuff",
                                "start": 1233,
                                "end": 1238
                              },
                              "start": 1231,
                              "end": 1239
                            },
                            "start": 1224,
                            "end": 1239
                          },
                          "start": 1220,
                          "end": 1242
                        },
                        "start": 1213,
                        "end": 1243
                      }
                    ],
                    "start": 1157,
                    "end": 1245
                  },
                  "id": null,
                  "generator": false,
                  "start": 1139,
                  "end": 1245
                }
              ],
              "optional": false,
              "start": 1128,
              "end": 1246
            },
            "definite": false,
            "start": 1114,
            "end": 1246
          }
        ],
        "declare": false,
        "start": 1108,
        "end": 1247
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 1101,
      "end": 1247
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
              "name": "AppStatic",
              "optional": false,
              "typeAnnotation": null,
              "start": 1262,
              "end": 1271
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 1274,
                "end": 1284
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
                      "typeAnnotation": {
                        "type": "TSTypeAnnotation",
                        "typeAnnotation": {
                          "type": "TSTypeReference",
                          "typeName": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "Stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 1293,
                            "end": 1298
                          },
                          "typeArguments": null,
                          "start": 1293,
                          "end": 1298
                        },
                        "start": 1291,
                        "end": 1298
                      },
                      "start": 1286,
                      "end": 1298
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
                            "type": "JSXFragment",
                            "openingFragment": {
                              "type": "JSXOpeningFragment",
                              "start": 1317,
                              "end": 1319
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1319,
                                "end": 1323
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1324,
                                    "end": 1327
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1323,
                                  "end": 1328
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static ",
                                    "raw": "Static ",
                                    "start": 1328,
                                    "end": 1335
                                  },
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "f",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1336,
                                        "end": 1337
                                      },
                                      "consequent": {
                                        "type": "Literal",
                                        "value": 1,
                                        "raw": "1",
                                        "start": 1340,
                                        "end": 1341
                                      },
                                      "alternate": {
                                        "type": "Literal",
                                        "value": 3,
                                        "raw": "3",
                                        "start": 1344,
                                        "end": 1345
                                      },
                                      "start": 1336,
                                      "end": 1345
                                    },
                                    "start": 1335,
                                    "end": 1346
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1348,
                                    "end": 1351
                                  },
                                  "start": 1346,
                                  "end": 1352
                                },
                                "start": 1323,
                                "end": 1352
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1352,
                                "end": 1356
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1357,
                                    "end": 1360
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1356,
                                  "end": 1361
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "BinaryExpression",
                                        "left": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1362,
                                          "end": 1366
                                        },
                                        "operator": "<",
                                        "right": {
                                          "type": "Literal",
                                          "value": 2,
                                          "raw": "2",
                                          "start": 1369,
                                          "end": 1370
                                        },
                                        "start": 1362,
                                        "end": 1370
                                      },
                                      "consequent": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 1374,
                                            "end": 1375
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 1373,
                                          "end": 1376
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "1",
                                            "raw": "1",
                                            "start": 1376,
                                            "end": 1377
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 1379,
                                            "end": 1380
                                          },
                                          "start": 1377,
                                          "end": 1381
                                        },
                                        "start": 1373,
                                        "end": 1381
                                      },
                                      "alternate": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 1385,
                                            "end": 1386
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 1384,
                                          "end": 1387
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "2",
                                            "raw": "2",
                                            "start": 1387,
                                            "end": 1388
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "p",
                                            "start": 1390,
                                            "end": 1391
                                          },
                                          "start": 1388,
                                          "end": 1392
                                        },
                                        "start": 1384,
                                        "end": 1392
                                      },
                                      "start": 1362,
                                      "end": 1392
                                    },
                                    "start": 1361,
                                    "end": 1393
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1395,
                                    "end": 1398
                                  },
                                  "start": 1393,
                                  "end": 1399
                                },
                                "start": 1356,
                                "end": 1399
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t\t",
                                "raw": "\n\n\t\t\t",
                                "start": 1399,
                                "end": 1404
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1405,
                                    "end": 1408
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1404,
                                  "end": 1409
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "LogicalExpression",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1410,
                                          "end": 1414
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1415,
                                          "end": 1420
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1410,
                                        "end": 1420
                                      },
                                      "operator": "&&",
                                      "right": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 1425,
                                            "end": 1428
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 1424,
                                          "end": 1429
                                        },
                                        "children": [],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 1431,
                                            "end": 1434
                                          },
                                          "start": 1429,
                                          "end": 1435
                                        },
                                        "start": 1424,
                                        "end": 1435
                                      },
                                      "start": 1410,
                                      "end": 1435
                                    },
                                    "start": 1409,
                                    "end": 1436
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1438,
                                    "end": 1441
                                  },
                                  "start": 1436,
                                  "end": 1442
                                },
                                "start": 1404,
                                "end": 1442
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1442,
                                "end": 1446
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1447,
                                    "end": 1450
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1446,
                                  "end": 1451
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "LogicalExpression",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1452,
                                          "end": 1456
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1457,
                                          "end": 1462
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1452,
                                        "end": 1462
                                      },
                                      "operator": "&&",
                                      "right": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "Fragment",
                                            "start": 1467,
                                            "end": 1475
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 1466,
                                          "end": 1476
                                        },
                                        "children": [
                                          {
                                            "type": "JSXElement",
                                            "openingElement": {
                                              "type": "JSXOpeningElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "Slot",
                                                "start": 1477,
                                                "end": 1481
                                              },
                                              "typeArguments": null,
                                              "attributes": [],
                                              "selfClosing": false,
                                              "start": 1476,
                                              "end": 1482
                                            },
                                            "children": [],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "Slot",
                                                "start": 1484,
                                                "end": 1488
                                              },
                                              "start": 1482,
                                              "end": 1489
                                            },
                                            "start": 1476,
                                            "end": 1489
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "Fragment",
                                            "start": 1491,
                                            "end": 1499
                                          },
                                          "start": 1489,
                                          "end": 1500
                                        },
                                        "start": 1466,
                                        "end": 1500
                                      },
                                      "start": 1452,
                                      "end": 1500
                                    },
                                    "start": 1451,
                                    "end": 1501
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1503,
                                    "end": 1506
                                  },
                                  "start": 1501,
                                  "end": 1507
                                },
                                "start": 1446,
                                "end": 1507
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1507,
                                "end": 1511
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1512,
                                    "end": 1515
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1511,
                                  "end": 1516
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "LogicalExpression",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1517,
                                          "end": 1521
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1522,
                                          "end": 1527
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1517,
                                        "end": 1527
                                      },
                                      "operator": "&&",
                                      "right": {
                                        "type": "JSXFragment",
                                        "openingFragment": {
                                          "type": "JSXOpeningFragment",
                                          "start": 1531,
                                          "end": 1533
                                        },
                                        "children": [
                                          {
                                            "type": "JSXElement",
                                            "openingElement": {
                                              "type": "JSXOpeningElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 1534,
                                                "end": 1537
                                              },
                                              "typeArguments": null,
                                              "attributes": [],
                                              "selfClosing": false,
                                              "start": 1533,
                                              "end": 1538
                                            },
                                            "children": [],
                                            "closingElement": {
                                              "type": "JSXClosingElement",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "div",
                                                "start": 1540,
                                                "end": 1543
                                              },
                                              "start": 1538,
                                              "end": 1544
                                            },
                                            "start": 1533,
                                            "end": 1544
                                          }
                                        ],
                                        "closingFragment": {
                                          "type": "JSXClosingFragment",
                                          "start": 1544,
                                          "end": 1547
                                        },
                                        "start": 1531,
                                        "end": 1547
                                      },
                                      "start": 1517,
                                      "end": 1547
                                    },
                                    "start": 1516,
                                    "end": 1548
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1550,
                                    "end": 1553
                                  },
                                  "start": 1548,
                                  "end": 1554
                                },
                                "start": 1511,
                                "end": 1554
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1554,
                                "end": 1558
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1559,
                                    "end": 1562
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1558,
                                  "end": 1563
                                },
                                "children": [
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "LogicalExpression",
                                      "left": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "prop",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1564,
                                          "end": 1568
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "value",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 1569,
                                          "end": 1574
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 1564,
                                        "end": 1574
                                      },
                                      "operator": "&&",
                                      "right": {
                                        "type": "JSXElement",
                                        "openingElement": {
                                          "type": "JSXOpeningElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "Image",
                                            "start": 1579,
                                            "end": 1584
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": true,
                                          "start": 1578,
                                          "end": 1586
                                        },
                                        "children": [],
                                        "closingElement": null,
                                        "start": 1578,
                                        "end": 1586
                                      },
                                      "start": 1564,
                                      "end": 1586
                                    },
                                    "start": 1563,
                                    "end": 1587
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1589,
                                    "end": 1592
                                  },
                                  "start": 1587,
                                  "end": 1593
                                },
                                "start": 1558,
                                "end": 1593
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1593,
                                "end": 1597
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1598,
                                    "end": 1601
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1597,
                                  "end": 1602
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static ",
                                    "raw": "Static ",
                                    "start": 1602,
                                    "end": 1609
                                  },
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ConditionalExpression",
                                      "test": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "f",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1610,
                                        "end": 1611
                                      },
                                      "consequent": {
                                        "type": "Literal",
                                        "value": 1,
                                        "raw": "1",
                                        "start": 1614,
                                        "end": 1615
                                      },
                                      "alternate": {
                                        "type": "Literal",
                                        "value": 3,
                                        "raw": "3",
                                        "start": 1618,
                                        "end": 1619
                                      },
                                      "start": 1610,
                                      "end": 1619
                                    },
                                    "start": 1609,
                                    "end": 1620
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1622,
                                    "end": 1625
                                  },
                                  "start": 1620,
                                  "end": 1626
                                },
                                "start": 1597,
                                "end": 1626
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1626,
                                "end": 1630
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1631,
                                    "end": 1634
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1630,
                                  "end": 1635
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static",
                                    "raw": "Static",
                                    "start": 1635,
                                    "end": 1641
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1643,
                                    "end": 1646
                                  },
                                  "start": 1641,
                                  "end": 1647
                                },
                                "start": 1630,
                                "end": 1647
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1647,
                                "end": 1651
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1652,
                                    "end": 1655
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1651,
                                  "end": 1656
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static ",
                                    "raw": "Static ",
                                    "start": 1656,
                                    "end": 1663
                                  },
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "props",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1664,
                                        "end": 1669
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "value",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1670,
                                        "end": 1675
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 1664,
                                      "end": 1675
                                    },
                                    "start": 1663,
                                    "end": 1676
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1678,
                                    "end": 1681
                                  },
                                  "start": 1676,
                                  "end": 1682
                                },
                                "start": 1651,
                                "end": 1682
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1682,
                                "end": 1686
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1687,
                                    "end": 1690
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1686,
                                  "end": 1691
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static ",
                                    "raw": "Static ",
                                    "start": 1691,
                                    "end": 1698
                                  },
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "stuff",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1699,
                                        "end": 1704
                                      },
                                      "typeArguments": null,
                                      "arguments": [],
                                      "optional": false,
                                      "start": 1699,
                                      "end": 1706
                                    },
                                    "start": 1698,
                                    "end": 1707
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1709,
                                    "end": 1712
                                  },
                                  "start": 1707,
                                  "end": 1713
                                },
                                "start": 1686,
                                "end": 1713
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 1713,
                                "end": 1717
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1718,
                                    "end": 1721
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 1717,
                                  "end": 1722
                                },
                                "children": [
                                  {
                                    "type": "JSXText",
                                    "value": "Static ",
                                    "raw": "Static ",
                                    "start": 1722,
                                    "end": 1729
                                  },
                                  {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "stuff",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 1730,
                                        "end": 1735
                                      },
                                      "typeArguments": null,
                                      "arguments": [],
                                      "optional": false,
                                      "start": 1730,
                                      "end": 1737
                                    },
                                    "start": 1729,
                                    "end": 1738
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "div",
                                    "start": 1740,
                                    "end": 1743
                                  },
                                  "start": 1738,
                                  "end": 1744
                                },
                                "start": 1717,
                                "end": 1744
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 1744,
                                "end": 1747
                              }
                            ],
                            "closingFragment": {
                              "type": "JSXClosingFragment",
                              "start": 1747,
                              "end": 1750
                            },
                            "start": 1317,
                            "end": 1750
                          },
                          "start": 1313,
                          "end": 1753
                        },
                        "start": 1306,
                        "end": 1754
                      }
                    ],
                    "start": 1303,
                    "end": 1756
                  },
                  "id": null,
                  "generator": false,
                  "start": 1285,
                  "end": 1756
                }
              ],
              "optional": false,
              "start": 1274,
              "end": 1757
            },
            "definite": false,
            "start": 1262,
            "end": 1757
          }
        ],
        "declare": false,
        "start": 1256,
        "end": 1758
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 1249,
      "end": 1758
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1758
}
```

</details>

## Output

### Module: test.js

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime";
import { Slot, Fragment } from '@qwik.dev/core';
import Image from './image.jpg?jsx';
export function Fn1(props) {
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("div", null, null, prop < 2 ? /*#__PURE__*/ _jsxSorted("p", null, null, "1", 3, "u6_0") : /*#__PURE__*/ _jsxSorted(Stuff, null, null, "2", 3, "u6_1"), 1, null), 1, "u6_2");
}
export function Fn2(props) {
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        prop.value && /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_3"),
        /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null)
    ], 1, "u6_4");
}
export function Fn3(props) {
    if (prop.value) return /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_5");
    return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_6");
}
export function Fn4(props) {
    if (prop.value) return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_7");
    return /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_8");
}
export const Arrow = (props)=>/*#__PURE__*/ _jsxSorted("div", null, null, prop < 2 ? /*#__PURE__*/ _jsxSorted("p", null, null, "1", 3, "u6_9") : /*#__PURE__*/ _jsxSorted(Stuff, null, null, "2", 3, "u6_10"), 1, "u6_11");
const AppDynamic1_component_R00UJ05gbes = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("div", null, null, prop < 2 ? /*#__PURE__*/ _jsxSorted("p", null, null, "1", 3, "u6_12") : /*#__PURE__*/ _jsxSorted(Stuff, null, null, "2", 3, "u6_13"), 1, null), 1, "u6_14");
};
export const AppDynamic1 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(AppDynamic1_component_R00UJ05gbes, "AppDynamic1_component_R00UJ05gbes"));
const AppDynamic2_component_3EY2zm0v00A = (props)=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        prop.value && /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_15"),
        /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null)
    ], 1, "u6_16");
};
export const AppDynamic2 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(AppDynamic2_component_3EY2zm0v00A, "AppDynamic2_component_3EY2zm0v00A"));
const AppDynamic3_component_FVq83NlbTDQ = (props)=>{
    if (prop.value) return /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_17");
    return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_18");
};
export const AppDynamic3 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(AppDynamic3_component_FVq83NlbTDQ, "AppDynamic3_component_FVq83NlbTDQ"));
const AppDynamic4_component_IO0yr8UvWEI = (props)=>{
    if (prop.value) return /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_19");
    return /*#__PURE__*/ _jsxSorted(Stuff, null, null, null, 3, "u6_20");
};
export const AppDynamic4 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(AppDynamic4_component_IO0yr8UvWEI, "AppDynamic4_component_IO0yr8UvWEI"));
const AppStatic_component_gYRXqF3G5nE = (props)=>{
    return /*#__PURE__*/ _jsxSorted(_Fragment, null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "Static ",
            f ? 1 : 3
        ], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, prop < 2 ? /*#__PURE__*/ _jsxSorted("p", null, null, "1", 3, "u6_21") : /*#__PURE__*/ _jsxSorted("p", null, null, "2", 3, "u6_22"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, prop.value && /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, "u6_23"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, prop.value && /*#__PURE__*/ _jsxSorted(Fragment, null, null, /*#__PURE__*/ _jsxSorted(Slot, null, null, null, 3, "u6_24"), 1, "u6_25"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, prop.value && /*#__PURE__*/ _jsxSorted(_Fragment, null, null, /*#__PURE__*/ _jsxSorted("div", null, null, null, 3, null), 3, "u6_26"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, prop.value && /*#__PURE__*/ _jsxSorted(Image, null, null, null, 3, "u6_27"), 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "Static ",
            f ? 1 : 3
        ], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, "Static", 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "Static ",
            _wrapProp(props)
        ], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "Static ",
            stuff()
        ], 1, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            "Static ",
            stuff()
        ], 1, null)
    ], 1, "u6_28");
};
export const AppStatic = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(AppStatic_component_gYRXqF3G5nE, "AppStatic_component_gYRXqF3G5nE"));
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
            "name": "componentQrl",
            "start": 54,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
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
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 101,
            "end": 111
          },
          "start": 101,
          "end": 111
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 119,
        "end": 135
      },
      "phase": null,
      "attributes": [],
      "start": 92,
      "end": 136
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 146,
            "end": 155
          },
          "local": {
            "type": "Identifier",
            "name": "_wrapProp",
            "start": 146,
            "end": 155
          },
          "start": 146,
          "end": 155
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 163,
        "end": 179
      },
      "phase": null,
      "attributes": [],
      "start": 137,
      "end": 180
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 190,
            "end": 198
          },
          "local": {
            "type": "Identifier",
            "name": "_Fragment",
            "start": 202,
            "end": 211
          },
          "start": 190,
          "end": 211
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 219,
        "end": 247
      },
      "phase": null,
      "attributes": [],
      "start": 181,
      "end": 248
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Slot",
            "start": 258,
            "end": 262
          },
          "local": {
            "type": "Identifier",
            "name": "Slot",
            "start": 258,
            "end": 262
          },
          "start": 258,
          "end": 262
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 264,
            "end": 272
          },
          "local": {
            "type": "Identifier",
            "name": "Fragment",
            "start": 264,
            "end": 272
          },
          "start": 264,
          "end": 272
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 280,
        "end": 296
      },
      "phase": null,
      "attributes": [],
      "start": 249,
      "end": 297
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "name": "Image",
            "start": 305,
            "end": 310
          },
          "start": 305,
          "end": 310
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./image.jpg?jsx",
        "raw": "'./image.jpg?jsx'",
        "start": 316,
        "end": 333
      },
      "phase": null,
      "attributes": [],
      "start": 298,
      "end": 334
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "name": "Fn1",
          "start": 351,
          "end": 354
        },
        "generator": false,
        "async": false,
        "params": [
          {
            "type": "Identifier",
            "name": "props",
            "start": 355,
            "end": 360
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
                  "start": 389,
                  "end": 399
                },
                "arguments": [
                  {
                    "type": "Identifier",
                    "name": "_Fragment",
                    "start": 400,
                    "end": 409
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 411,
                    "end": 415
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 417,
                    "end": 421
                  },
                  {
                    "type": "CallExpression",
                    "callee": {
                      "type": "Identifier",
                      "name": "_jsxSorted",
                      "start": 437,
                      "end": 447
                    },
                    "arguments": [
                      {
                        "type": "Literal",
                        "value": "div",
                        "raw": "\"div\"",
                        "start": 448,
                        "end": 453
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 455,
                        "end": 459
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 461,
                        "end": 465
                      },
                      {
                        "type": "ConditionalExpression",
                        "test": {
                          "type": "BinaryExpression",
                          "left": {
                            "type": "Identifier",
                            "name": "prop",
                            "start": 467,
                            "end": 471
                          },
                          "operator": "<",
                          "right": {
                            "type": "Literal",
                            "value": 2,
                            "raw": "2",
                            "start": 474,
                            "end": 475
                          },
                          "start": 467,
                          "end": 475
                        },
                        "consequent": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_jsxSorted",
                            "start": 492,
                            "end": 502
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "p",
                              "raw": "\"p\"",
                              "start": 503,
                              "end": 506
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 508,
                              "end": 512
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 514,
                              "end": 518
                            },
                            {
                              "type": "Literal",
                              "value": "1",
                              "raw": "\"1\"",
                              "start": 520,
                              "end": 523
                            },
                            {
                              "type": "Literal",
                              "value": 3,
                              "raw": "3",
                              "start": 525,
                              "end": 526
                            },
                            {
                              "type": "Literal",
                              "value": "u6_0",
                              "raw": "\"u6_0\"",
                              "start": 528,
                              "end": 534
                            }
                          ],
                          "optional": false,
                          "start": 492,
                          "end": 535
                        },
                        "alternate": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_jsxSorted",
                            "start": 552,
                            "end": 562
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "Stuff",
                              "start": 563,
                              "end": 568
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 570,
                              "end": 574
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 576,
                              "end": 580
                            },
                            {
                              "type": "Literal",
                              "value": "2",
                              "raw": "\"2\"",
                              "start": 582,
                              "end": 585
                            },
                            {
                              "type": "Literal",
                              "value": 3,
                              "raw": "3",
                              "start": 587,
                              "end": 588
                            },
                            {
                              "type": "Literal",
                              "value": "u6_1",
                              "raw": "\"u6_1\"",
                              "start": 590,
                              "end": 596
                            }
                          ],
                          "optional": false,
                          "start": 552,
                          "end": 597
                        },
                        "start": 467,
                        "end": 597
                      },
                      {
                        "type": "Literal",
                        "value": 1,
                        "raw": "1",
                        "start": 599,
                        "end": 600
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 602,
                        "end": 606
                      }
                    ],
                    "optional": false,
                    "start": 437,
                    "end": 607
                  },
                  {
                    "type": "Literal",
                    "value": 1,
                    "raw": "1",
                    "start": 609,
                    "end": 610
                  },
                  {
                    "type": "Literal",
                    "value": "u6_2",
                    "raw": "\"u6_2\"",
                    "start": 612,
                    "end": 618
                  }
                ],
                "optional": false,
                "start": 389,
                "end": 619
              },
              "start": 368,
              "end": 620
            }
          ],
          "start": 362,
          "end": 622
        },
        "expression": false,
        "start": 342,
        "end": 622
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 335,
      "end": 622
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "name": "Fn2",
          "start": 639,
          "end": 642
        },
        "generator": false,
        "async": false,
        "params": [
          {
            "type": "Identifier",
            "name": "props",
            "start": 643,
            "end": 648
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
                  "start": 677,
                  "end": 687
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 688,
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
                    "value": null,
                    "raw": "null",
                    "start": 701,
                    "end": 705
                  },
                  {
                    "type": "ArrayExpression",
                    "elements": [
                      {
                        "type": "LogicalExpression",
                        "left": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "prop",
                            "start": 717,
                            "end": 721
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "value",
                            "start": 722,
                            "end": 727
                          },
                          "optional": false,
                          "computed": false,
                          "start": 717,
                          "end": 727
                        },
                        "operator": "&&",
                        "right": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_jsxSorted",
                            "start": 745,
                            "end": 755
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "Stuff",
                              "start": 756,
                              "end": 761
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 763,
                              "end": 767
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 769,
                              "end": 773
                            },
                            {
                              "type": "Literal",
                              "value": null,
                              "raw": "null",
                              "start": 775,
                              "end": 779
                            },
                            {
                              "type": "Literal",
                              "value": 3,
                              "raw": "3",
                              "start": 781,
                              "end": 782
                            },
                            {
                              "type": "Literal",
                              "value": "u6_3",
                              "raw": "\"u6_3\"",
                              "start": 784,
                              "end": 790
                            }
                          ],
                          "optional": false,
                          "start": 745,
                          "end": 791
                        },
                        "start": 717,
                        "end": 791
                      },
                      {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "name": "_jsxSorted",
                          "start": 815,
                          "end": 825
                        },
                        "arguments": [
                          {
                            "type": "Literal",
                            "value": "div",
                            "raw": "\"div\"",
                            "start": 826,
                            "end": 831
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 833,
                            "end": 837
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 839,
                            "end": 843
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 845,
                            "end": 849
                          },
                          {
                            "type": "Literal",
                            "value": 3,
                            "raw": "3",
                            "start": 851,
                            "end": 852
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 854,
                            "end": 858
                          }
                        ],
                        "optional": false,
                        "start": 815,
                        "end": 859
                      }
                    ],
                    "start": 707,
                    "end": 865
                  },
                  {
                    "type": "Literal",
                    "value": 1,
                    "raw": "1",
                    "start": 867,
                    "end": 868
                  },
                  {
                    "type": "Literal",
                    "value": "u6_4",
                    "raw": "\"u6_4\"",
                    "start": 870,
                    "end": 876
                  }
                ],
                "optional": false,
                "start": 677,
                "end": 877
              },
              "start": 656,
              "end": 878
            }
          ],
          "start": 650,
          "end": 880
        },
        "expression": false,
        "start": 630,
        "end": 880
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 623,
      "end": 880
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "name": "Fn3",
          "start": 897,
          "end": 900
        },
        "generator": false,
        "async": false,
        "params": [
          {
            "type": "Identifier",
            "name": "props",
            "start": 901,
            "end": 906
          }
        ],
        "body": {
          "type": "BlockStatement",
          "body": [
            {
              "type": "IfStatement",
              "test": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "name": "prop",
                  "start": 918,
                  "end": 922
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 923,
                  "end": 928
                },
                "optional": false,
                "computed": false,
                "start": 918,
                "end": 928
              },
              "consequent": {
                "type": "ReturnStatement",
                "argument": {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "_jsxSorted",
                    "start": 951,
                    "end": 961
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "Stuff",
                      "start": 962,
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
                      "value": null,
                      "raw": "null",
                      "start": 975,
                      "end": 979
                    },
                    {
                      "type": "Literal",
                      "value": null,
                      "raw": "null",
                      "start": 981,
                      "end": 985
                    },
                    {
                      "type": "Literal",
                      "value": 3,
                      "raw": "3",
                      "start": 987,
                      "end": 988
                    },
                    {
                      "type": "Literal",
                      "value": "u6_5",
                      "raw": "\"u6_5\"",
                      "start": 990,
                      "end": 996
                    }
                  ],
                  "optional": false,
                  "start": 951,
                  "end": 997
                },
                "start": 930,
                "end": 998
              },
              "alternate": null,
              "start": 914,
              "end": 998
            },
            {
              "type": "ReturnStatement",
              "argument": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "name": "_jsxSorted",
                  "start": 1024,
                  "end": 1034
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 1035,
                    "end": 1040
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1042,
                    "end": 1046
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1048,
                    "end": 1052
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1054,
                    "end": 1058
                  },
                  {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 1060,
                    "end": 1061
                  },
                  {
                    "type": "Literal",
                    "value": "u6_6",
                    "raw": "\"u6_6\"",
                    "start": 1063,
                    "end": 1069
                  }
                ],
                "optional": false,
                "start": 1024,
                "end": 1070
              },
              "start": 1003,
              "end": 1071
            }
          ],
          "start": 908,
          "end": 1073
        },
        "expression": false,
        "start": 888,
        "end": 1073
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 881,
      "end": 1073
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "name": "Fn4",
          "start": 1090,
          "end": 1093
        },
        "generator": false,
        "async": false,
        "params": [
          {
            "type": "Identifier",
            "name": "props",
            "start": 1094,
            "end": 1099
          }
        ],
        "body": {
          "type": "BlockStatement",
          "body": [
            {
              "type": "IfStatement",
              "test": {
                "type": "MemberExpression",
                "object": {
                  "type": "Identifier",
                  "name": "prop",
                  "start": 1111,
                  "end": 1115
                },
                "property": {
                  "type": "Identifier",
                  "name": "value",
                  "start": 1116,
                  "end": 1121
                },
                "optional": false,
                "computed": false,
                "start": 1111,
                "end": 1121
              },
              "consequent": {
                "type": "ReturnStatement",
                "argument": {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "_jsxSorted",
                    "start": 1144,
                    "end": 1154
                  },
                  "arguments": [
                    {
                      "type": "Literal",
                      "value": "div",
                      "raw": "\"div\"",
                      "start": 1155,
                      "end": 1160
                    },
                    {
                      "type": "Literal",
                      "value": null,
                      "raw": "null",
                      "start": 1162,
                      "end": 1166
                    },
                    {
                      "type": "Literal",
                      "value": null,
                      "raw": "null",
                      "start": 1168,
                      "end": 1172
                    },
                    {
                      "type": "Literal",
                      "value": null,
                      "raw": "null",
                      "start": 1174,
                      "end": 1178
                    },
                    {
                      "type": "Literal",
                      "value": 3,
                      "raw": "3",
                      "start": 1180,
                      "end": 1181
                    },
                    {
                      "type": "Literal",
                      "value": "u6_7",
                      "raw": "\"u6_7\"",
                      "start": 1183,
                      "end": 1189
                    }
                  ],
                  "optional": false,
                  "start": 1144,
                  "end": 1190
                },
                "start": 1123,
                "end": 1191
              },
              "alternate": null,
              "start": 1107,
              "end": 1191
            },
            {
              "type": "ReturnStatement",
              "argument": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "name": "_jsxSorted",
                  "start": 1217,
                  "end": 1227
                },
                "arguments": [
                  {
                    "type": "Identifier",
                    "name": "Stuff",
                    "start": 1228,
                    "end": 1233
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1235,
                    "end": 1239
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1241,
                    "end": 1245
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1247,
                    "end": 1251
                  },
                  {
                    "type": "Literal",
                    "value": 3,
                    "raw": "3",
                    "start": 1253,
                    "end": 1254
                  },
                  {
                    "type": "Literal",
                    "value": "u6_8",
                    "raw": "\"u6_8\"",
                    "start": 1256,
                    "end": 1262
                  }
                ],
                "optional": false,
                "start": 1217,
                "end": 1263
              },
              "start": 1196,
              "end": 1264
            }
          ],
          "start": 1101,
          "end": 1266
        },
        "expression": false,
        "start": 1081,
        "end": 1266
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 1074,
      "end": 1266
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
              "name": "Arrow",
              "start": 1280,
              "end": 1285
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": true,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 1289,
                  "end": 1294
                }
              ],
              "body": {
                "type": "CallExpression",
                "callee": {
                  "type": "Identifier",
                  "name": "_jsxSorted",
                  "start": 1311,
                  "end": 1321
                },
                "arguments": [
                  {
                    "type": "Literal",
                    "value": "div",
                    "raw": "\"div\"",
                    "start": 1322,
                    "end": 1327
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1329,
                    "end": 1333
                  },
                  {
                    "type": "Literal",
                    "value": null,
                    "raw": "null",
                    "start": 1335,
                    "end": 1339
                  },
                  {
                    "type": "ConditionalExpression",
                    "test": {
                      "type": "BinaryExpression",
                      "left": {
                        "type": "Identifier",
                        "name": "prop",
                        "start": 1341,
                        "end": 1345
                      },
                      "operator": "<",
                      "right": {
                        "type": "Literal",
                        "value": 2,
                        "raw": "2",
                        "start": 1348,
                        "end": 1349
                      },
                      "start": 1341,
                      "end": 1349
                    },
                    "consequent": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 1366,
                        "end": 1376
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "p",
                          "raw": "\"p\"",
                          "start": 1377,
                          "end": 1380
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1382,
                          "end": 1386
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1388,
                          "end": 1392
                        },
                        {
                          "type": "Literal",
                          "value": "1",
                          "raw": "\"1\"",
                          "start": 1394,
                          "end": 1397
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 1399,
                          "end": 1400
                        },
                        {
                          "type": "Literal",
                          "value": "u6_9",
                          "raw": "\"u6_9\"",
                          "start": 1402,
                          "end": 1408
                        }
                      ],
                      "optional": false,
                      "start": 1366,
                      "end": 1409
                    },
                    "alternate": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 1426,
                        "end": 1436
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Stuff",
                          "start": 1437,
                          "end": 1442
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1444,
                          "end": 1448
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 1450,
                          "end": 1454
                        },
                        {
                          "type": "Literal",
                          "value": "2",
                          "raw": "\"2\"",
                          "start": 1456,
                          "end": 1459
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 1461,
                          "end": 1462
                        },
                        {
                          "type": "Literal",
                          "value": "u6_10",
                          "raw": "\"u6_10\"",
                          "start": 1464,
                          "end": 1471
                        }
                      ],
                      "optional": false,
                      "start": 1426,
                      "end": 1472
                    },
                    "start": 1341,
                    "end": 1472
                  },
                  {
                    "type": "Literal",
                    "value": 1,
                    "raw": "1",
                    "start": 1474,
                    "end": 1475
                  },
                  {
                    "type": "Literal",
                    "value": "u6_11",
                    "raw": "\"u6_11\"",
                    "start": 1477,
                    "end": 1484
                  }
                ],
                "optional": false,
                "start": 1311,
                "end": 1485
              },
              "id": null,
              "generator": false,
              "start": 1288,
              "end": 1485
            },
            "start": 1280,
            "end": 1485
          }
        ],
        "start": 1274,
        "end": 1486
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 1267,
      "end": 1486
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "AppDynamic1_component_R00UJ05gbes",
            "start": 1493,
            "end": 1526
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "props",
                "start": 1530,
                "end": 1535
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
                      "start": 1565,
                      "end": 1575
                    },
                    "arguments": [
                      {
                        "type": "Identifier",
                        "name": "_Fragment",
                        "start": 1576,
                        "end": 1585
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 1587,
                        "end": 1591
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 1593,
                        "end": 1597
                      },
                      {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "name": "_jsxSorted",
                          "start": 1613,
                          "end": 1623
                        },
                        "arguments": [
                          {
                            "type": "Literal",
                            "value": "div",
                            "raw": "\"div\"",
                            "start": 1624,
                            "end": 1629
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 1631,
                            "end": 1635
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 1637,
                            "end": 1641
                          },
                          {
                            "type": "ConditionalExpression",
                            "test": {
                              "type": "BinaryExpression",
                              "left": {
                                "type": "Identifier",
                                "name": "prop",
                                "start": 1643,
                                "end": 1647
                              },
                              "operator": "<",
                              "right": {
                                "type": "Literal",
                                "value": 2,
                                "raw": "2",
                                "start": 1650,
                                "end": 1651
                              },
                              "start": 1643,
                              "end": 1651
                            },
                            "consequent": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1668,
                                "end": 1678
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "p",
                                  "raw": "\"p\"",
                                  "start": 1679,
                                  "end": 1682
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1684,
                                  "end": 1688
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1690,
                                  "end": 1694
                                },
                                {
                                  "type": "Literal",
                                  "value": "1",
                                  "raw": "\"1\"",
                                  "start": 1696,
                                  "end": 1699
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1701,
                                  "end": 1702
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_12",
                                  "raw": "\"u6_12\"",
                                  "start": 1704,
                                  "end": 1711
                                }
                              ],
                              "optional": false,
                              "start": 1668,
                              "end": 1712
                            },
                            "alternate": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 1729,
                                "end": 1739
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Stuff",
                                  "start": 1740,
                                  "end": 1745
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1747,
                                  "end": 1751
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1753,
                                  "end": 1757
                                },
                                {
                                  "type": "Literal",
                                  "value": "2",
                                  "raw": "\"2\"",
                                  "start": 1759,
                                  "end": 1762
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1764,
                                  "end": 1765
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_13",
                                  "raw": "\"u6_13\"",
                                  "start": 1767,
                                  "end": 1774
                                }
                              ],
                              "optional": false,
                              "start": 1729,
                              "end": 1775
                            },
                            "start": 1643,
                            "end": 1775
                          },
                          {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 1777,
                            "end": 1778
                          },
                          {
                            "type": "Literal",
                            "value": null,
                            "raw": "null",
                            "start": 1780,
                            "end": 1784
                          }
                        ],
                        "optional": false,
                        "start": 1613,
                        "end": 1785
                      },
                      {
                        "type": "Literal",
                        "value": 1,
                        "raw": "1",
                        "start": 1787,
                        "end": 1788
                      },
                      {
                        "type": "Literal",
                        "value": "u6_14",
                        "raw": "\"u6_14\"",
                        "start": 1790,
                        "end": 1797
                      }
                    ],
                    "optional": false,
                    "start": 1565,
                    "end": 1798
                  },
                  "start": 1544,
                  "end": 1799
                }
              ],
              "start": 1538,
              "end": 1801
            },
            "id": null,
            "generator": false,
            "start": 1529,
            "end": 1801
          },
          "start": 1493,
          "end": 1801
        }
      ],
      "start": 1487,
      "end": 1802
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
              "name": "AppDynamic1",
              "start": 1816,
              "end": 1827
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 1844,
                "end": 1856
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 1871,
                    "end": 1881
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "AppDynamic1_component_R00UJ05gbes",
                      "start": 1882,
                      "end": 1915
                    },
                    {
                      "type": "Literal",
                      "value": "AppDynamic1_component_R00UJ05gbes",
                      "raw": "\"AppDynamic1_component_R00UJ05gbes\"",
                      "start": 1917,
                      "end": 1952
                    }
                  ],
                  "optional": false,
                  "start": 1871,
                  "end": 1953
                }
              ],
              "optional": false,
              "start": 1844,
              "end": 1954
            },
            "start": 1816,
            "end": 1954
          }
        ],
        "start": 1810,
        "end": 1955
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 1803,
      "end": 1955
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "AppDynamic2_component_3EY2zm0v00A",
            "start": 1962,
            "end": 1995
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "props",
                "start": 1999,
                "end": 2004
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
                      "start": 2034,
                      "end": 2044
                    },
                    "arguments": [
                      {
                        "type": "Literal",
                        "value": "div",
                        "raw": "\"div\"",
                        "start": 2045,
                        "end": 2050
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2052,
                        "end": 2056
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2058,
                        "end": 2062
                      },
                      {
                        "type": "ArrayExpression",
                        "elements": [
                          {
                            "type": "LogicalExpression",
                            "left": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "name": "prop",
                                "start": 2074,
                                "end": 2078
                              },
                              "property": {
                                "type": "Identifier",
                                "name": "value",
                                "start": 2079,
                                "end": 2084
                              },
                              "optional": false,
                              "computed": false,
                              "start": 2074,
                              "end": 2084
                            },
                            "operator": "&&",
                            "right": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 2102,
                                "end": 2112
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "Stuff",
                                  "start": 2113,
                                  "end": 2118
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 2120,
                                  "end": 2124
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 2126,
                                  "end": 2130
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 2132,
                                  "end": 2136
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 2138,
                                  "end": 2139
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_15",
                                  "raw": "\"u6_15\"",
                                  "start": 2141,
                                  "end": 2148
                                }
                              ],
                              "optional": false,
                              "start": 2102,
                              "end": 2149
                            },
                            "start": 2074,
                            "end": 2149
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 2173,
                              "end": 2183
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 2184,
                                "end": 2189
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 2191,
                                "end": 2195
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 2197,
                                "end": 2201
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 2203,
                                "end": 2207
                              },
                              {
                                "type": "Literal",
                                "value": 3,
                                "raw": "3",
                                "start": 2209,
                                "end": 2210
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 2212,
                                "end": 2216
                              }
                            ],
                            "optional": false,
                            "start": 2173,
                            "end": 2217
                          }
                        ],
                        "start": 2064,
                        "end": 2223
                      },
                      {
                        "type": "Literal",
                        "value": 1,
                        "raw": "1",
                        "start": 2225,
                        "end": 2226
                      },
                      {
                        "type": "Literal",
                        "value": "u6_16",
                        "raw": "\"u6_16\"",
                        "start": 2228,
                        "end": 2235
                      }
                    ],
                    "optional": false,
                    "start": 2034,
                    "end": 2236
                  },
                  "start": 2013,
                  "end": 2237
                }
              ],
              "start": 2007,
              "end": 2239
            },
            "id": null,
            "generator": false,
            "start": 1998,
            "end": 2239
          },
          "start": 1962,
          "end": 2239
        }
      ],
      "start": 1956,
      "end": 2240
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
              "name": "AppDynamic2",
              "start": 2254,
              "end": 2265
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 2282,
                "end": 2294
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 2309,
                    "end": 2319
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "AppDynamic2_component_3EY2zm0v00A",
                      "start": 2320,
                      "end": 2353
                    },
                    {
                      "type": "Literal",
                      "value": "AppDynamic2_component_3EY2zm0v00A",
                      "raw": "\"AppDynamic2_component_3EY2zm0v00A\"",
                      "start": 2355,
                      "end": 2390
                    }
                  ],
                  "optional": false,
                  "start": 2309,
                  "end": 2391
                }
              ],
              "optional": false,
              "start": 2282,
              "end": 2392
            },
            "start": 2254,
            "end": 2392
          }
        ],
        "start": 2248,
        "end": 2393
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 2241,
      "end": 2393
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "AppDynamic3_component_FVq83NlbTDQ",
            "start": 2400,
            "end": 2433
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "props",
                "start": 2437,
                "end": 2442
              }
            ],
            "body": {
              "type": "BlockStatement",
              "body": [
                {
                  "type": "IfStatement",
                  "test": {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "name": "prop",
                      "start": 2455,
                      "end": 2459
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "value",
                      "start": 2460,
                      "end": 2465
                    },
                    "optional": false,
                    "computed": false,
                    "start": 2455,
                    "end": 2465
                  },
                  "consequent": {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 2488,
                        "end": 2498
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "Stuff",
                          "start": 2499,
                          "end": 2504
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2506,
                          "end": 2510
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2512,
                          "end": 2516
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2518,
                          "end": 2522
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 2524,
                          "end": 2525
                        },
                        {
                          "type": "Literal",
                          "value": "u6_17",
                          "raw": "\"u6_17\"",
                          "start": 2527,
                          "end": 2534
                        }
                      ],
                      "optional": false,
                      "start": 2488,
                      "end": 2535
                    },
                    "start": 2467,
                    "end": 2536
                  },
                  "alternate": null,
                  "start": 2451,
                  "end": 2536
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "CallExpression",
                    "callee": {
                      "type": "Identifier",
                      "name": "_jsxSorted",
                      "start": 2562,
                      "end": 2572
                    },
                    "arguments": [
                      {
                        "type": "Literal",
                        "value": "div",
                        "raw": "\"div\"",
                        "start": 2573,
                        "end": 2578
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2580,
                        "end": 2584
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2586,
                        "end": 2590
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2592,
                        "end": 2596
                      },
                      {
                        "type": "Literal",
                        "value": 3,
                        "raw": "3",
                        "start": 2598,
                        "end": 2599
                      },
                      {
                        "type": "Literal",
                        "value": "u6_18",
                        "raw": "\"u6_18\"",
                        "start": 2601,
                        "end": 2608
                      }
                    ],
                    "optional": false,
                    "start": 2562,
                    "end": 2609
                  },
                  "start": 2541,
                  "end": 2610
                }
              ],
              "start": 2445,
              "end": 2612
            },
            "id": null,
            "generator": false,
            "start": 2436,
            "end": 2612
          },
          "start": 2400,
          "end": 2612
        }
      ],
      "start": 2394,
      "end": 2613
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
              "name": "AppDynamic3",
              "start": 2627,
              "end": 2638
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 2655,
                "end": 2667
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 2682,
                    "end": 2692
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "AppDynamic3_component_FVq83NlbTDQ",
                      "start": 2693,
                      "end": 2726
                    },
                    {
                      "type": "Literal",
                      "value": "AppDynamic3_component_FVq83NlbTDQ",
                      "raw": "\"AppDynamic3_component_FVq83NlbTDQ\"",
                      "start": 2728,
                      "end": 2763
                    }
                  ],
                  "optional": false,
                  "start": 2682,
                  "end": 2764
                }
              ],
              "optional": false,
              "start": 2655,
              "end": 2765
            },
            "start": 2627,
            "end": 2765
          }
        ],
        "start": 2621,
        "end": 2766
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 2614,
      "end": 2766
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "AppDynamic4_component_IO0yr8UvWEI",
            "start": 2773,
            "end": 2806
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "props",
                "start": 2810,
                "end": 2815
              }
            ],
            "body": {
              "type": "BlockStatement",
              "body": [
                {
                  "type": "IfStatement",
                  "test": {
                    "type": "MemberExpression",
                    "object": {
                      "type": "Identifier",
                      "name": "prop",
                      "start": 2828,
                      "end": 2832
                    },
                    "property": {
                      "type": "Identifier",
                      "name": "value",
                      "start": 2833,
                      "end": 2838
                    },
                    "optional": false,
                    "computed": false,
                    "start": 2828,
                    "end": 2838
                  },
                  "consequent": {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 2861,
                        "end": 2871
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 2872,
                          "end": 2877
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2879,
                          "end": 2883
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2885,
                          "end": 2889
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 2891,
                          "end": 2895
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 2897,
                          "end": 2898
                        },
                        {
                          "type": "Literal",
                          "value": "u6_19",
                          "raw": "\"u6_19\"",
                          "start": 2900,
                          "end": 2907
                        }
                      ],
                      "optional": false,
                      "start": 2861,
                      "end": 2908
                    },
                    "start": 2840,
                    "end": 2909
                  },
                  "alternate": null,
                  "start": 2824,
                  "end": 2909
                },
                {
                  "type": "ReturnStatement",
                  "argument": {
                    "type": "CallExpression",
                    "callee": {
                      "type": "Identifier",
                      "name": "_jsxSorted",
                      "start": 2935,
                      "end": 2945
                    },
                    "arguments": [
                      {
                        "type": "Identifier",
                        "name": "Stuff",
                        "start": 2946,
                        "end": 2951
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2953,
                        "end": 2957
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2959,
                        "end": 2963
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 2965,
                        "end": 2969
                      },
                      {
                        "type": "Literal",
                        "value": 3,
                        "raw": "3",
                        "start": 2971,
                        "end": 2972
                      },
                      {
                        "type": "Literal",
                        "value": "u6_20",
                        "raw": "\"u6_20\"",
                        "start": 2974,
                        "end": 2981
                      }
                    ],
                    "optional": false,
                    "start": 2935,
                    "end": 2982
                  },
                  "start": 2914,
                  "end": 2983
                }
              ],
              "start": 2818,
              "end": 2985
            },
            "id": null,
            "generator": false,
            "start": 2809,
            "end": 2985
          },
          "start": 2773,
          "end": 2985
        }
      ],
      "start": 2767,
      "end": 2986
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
              "name": "AppDynamic4",
              "start": 3000,
              "end": 3011
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 3028,
                "end": 3040
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 3055,
                    "end": 3065
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "AppDynamic4_component_IO0yr8UvWEI",
                      "start": 3066,
                      "end": 3099
                    },
                    {
                      "type": "Literal",
                      "value": "AppDynamic4_component_IO0yr8UvWEI",
                      "raw": "\"AppDynamic4_component_IO0yr8UvWEI\"",
                      "start": 3101,
                      "end": 3136
                    }
                  ],
                  "optional": false,
                  "start": 3055,
                  "end": 3137
                }
              ],
              "optional": false,
              "start": 3028,
              "end": 3138
            },
            "start": 3000,
            "end": 3138
          }
        ],
        "start": 2994,
        "end": 3139
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 2987,
      "end": 3139
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "AppStatic_component_gYRXqF3G5nE",
            "start": 3146,
            "end": 3177
          },
          "init": {
            "type": "ArrowFunctionExpression",
            "expression": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "name": "props",
                "start": 3181,
                "end": 3186
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
                      "start": 3216,
                      "end": 3226
                    },
                    "arguments": [
                      {
                        "type": "Identifier",
                        "name": "_Fragment",
                        "start": 3227,
                        "end": 3236
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 3238,
                        "end": 3242
                      },
                      {
                        "type": "Literal",
                        "value": null,
                        "raw": "null",
                        "start": 3244,
                        "end": 3248
                      },
                      {
                        "type": "ArrayExpression",
                        "elements": [
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 3274,
                              "end": 3284
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 3285,
                                "end": 3290
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3292,
                                "end": 3296
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3298,
                                "end": 3302
                              },
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "Static ",
                                    "raw": "\"Static \"",
                                    "start": 3318,
                                    "end": 3327
                                  },
                                  {
                                    "type": "ConditionalExpression",
                                    "test": {
                                      "type": "Identifier",
                                      "name": "f",
                                      "start": 3341,
                                      "end": 3342
                                    },
                                    "consequent": {
                                      "type": "Literal",
                                      "value": 1,
                                      "raw": "1",
                                      "start": 3345,
                                      "end": 3346
                                    },
                                    "alternate": {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 3349,
                                      "end": 3350
                                    },
                                    "start": 3341,
                                    "end": 3350
                                  }
                                ],
                                "start": 3304,
                                "end": 3360
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 3362,
                                "end": 3363
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3365,
                                "end": 3369
                              }
                            ],
                            "optional": false,
                            "start": 3274,
                            "end": 3370
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 3394,
                              "end": 3404
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 3405,
                                "end": 3410
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3412,
                                "end": 3416
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3418,
                                "end": 3422
                              },
                              {
                                "type": "ConditionalExpression",
                                "test": {
                                  "type": "BinaryExpression",
                                  "left": {
                                    "type": "Identifier",
                                    "name": "prop",
                                    "start": 3424,
                                    "end": 3428
                                  },
                                  "operator": "<",
                                  "right": {
                                    "type": "Literal",
                                    "value": 2,
                                    "raw": "2",
                                    "start": 3431,
                                    "end": 3432
                                  },
                                  "start": 3424,
                                  "end": 3432
                                },
                                "consequent": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 3449,
                                    "end": 3459
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "p",
                                      "raw": "\"p\"",
                                      "start": 3460,
                                      "end": 3463
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3465,
                                      "end": 3469
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3471,
                                      "end": 3475
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "1",
                                      "raw": "\"1\"",
                                      "start": 3477,
                                      "end": 3480
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 3482,
                                      "end": 3483
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_21",
                                      "raw": "\"u6_21\"",
                                      "start": 3485,
                                      "end": 3492
                                    }
                                  ],
                                  "optional": false,
                                  "start": 3449,
                                  "end": 3493
                                },
                                "alternate": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 3510,
                                    "end": 3520
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "p",
                                      "raw": "\"p\"",
                                      "start": 3521,
                                      "end": 3524
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3526,
                                      "end": 3530
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3532,
                                      "end": 3536
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "2",
                                      "raw": "\"2\"",
                                      "start": 3538,
                                      "end": 3541
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 3543,
                                      "end": 3544
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_22",
                                      "raw": "\"u6_22\"",
                                      "start": 3546,
                                      "end": 3553
                                    }
                                  ],
                                  "optional": false,
                                  "start": 3510,
                                  "end": 3554
                                },
                                "start": 3424,
                                "end": 3554
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 3556,
                                "end": 3557
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3559,
                                "end": 3563
                              }
                            ],
                            "optional": false,
                            "start": 3394,
                            "end": 3564
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 3588,
                              "end": 3598
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 3599,
                                "end": 3604
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3606,
                                "end": 3610
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3612,
                                "end": 3616
                              },
                              {
                                "type": "LogicalExpression",
                                "left": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "prop",
                                    "start": 3618,
                                    "end": 3622
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 3623,
                                    "end": 3628
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 3618,
                                  "end": 3628
                                },
                                "operator": "&&",
                                "right": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 3646,
                                    "end": 3656
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "div",
                                      "raw": "\"div\"",
                                      "start": 3657,
                                      "end": 3662
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3664,
                                      "end": 3668
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3670,
                                      "end": 3674
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3676,
                                      "end": 3680
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 3682,
                                      "end": 3683
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_23",
                                      "raw": "\"u6_23\"",
                                      "start": 3685,
                                      "end": 3692
                                    }
                                  ],
                                  "optional": false,
                                  "start": 3646,
                                  "end": 3693
                                },
                                "start": 3618,
                                "end": 3693
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 3695,
                                "end": 3696
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3698,
                                "end": 3702
                              }
                            ],
                            "optional": false,
                            "start": 3588,
                            "end": 3703
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 3727,
                              "end": 3737
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 3738,
                                "end": 3743
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3745,
                                "end": 3749
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3751,
                                "end": 3755
                              },
                              {
                                "type": "LogicalExpression",
                                "left": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "prop",
                                    "start": 3757,
                                    "end": 3761
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 3762,
                                    "end": 3767
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 3757,
                                  "end": 3767
                                },
                                "operator": "&&",
                                "right": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 3785,
                                    "end": 3795
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "Fragment",
                                      "start": 3796,
                                      "end": 3804
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3806,
                                      "end": 3810
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 3812,
                                      "end": 3816
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 3832,
                                        "end": 3842
                                      },
                                      "arguments": [
                                        {
                                          "type": "Identifier",
                                          "name": "Slot",
                                          "start": 3843,
                                          "end": 3847
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 3849,
                                          "end": 3853
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 3855,
                                          "end": 3859
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 3861,
                                          "end": 3865
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 3867,
                                          "end": 3868
                                        },
                                        {
                                          "type": "Literal",
                                          "value": "u6_24",
                                          "raw": "\"u6_24\"",
                                          "start": 3870,
                                          "end": 3877
                                        }
                                      ],
                                      "optional": false,
                                      "start": 3832,
                                      "end": 3878
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 1,
                                      "raw": "1",
                                      "start": 3880,
                                      "end": 3881
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_25",
                                      "raw": "\"u6_25\"",
                                      "start": 3883,
                                      "end": 3890
                                    }
                                  ],
                                  "optional": false,
                                  "start": 3785,
                                  "end": 3891
                                },
                                "start": 3757,
                                "end": 3891
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 3893,
                                "end": 3894
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3896,
                                "end": 3900
                              }
                            ],
                            "optional": false,
                            "start": 3727,
                            "end": 3901
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 3925,
                              "end": 3935
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 3936,
                                "end": 3941
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3943,
                                "end": 3947
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 3949,
                                "end": 3953
                              },
                              {
                                "type": "LogicalExpression",
                                "left": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "prop",
                                    "start": 3955,
                                    "end": 3959
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 3960,
                                    "end": 3965
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 3955,
                                  "end": 3965
                                },
                                "operator": "&&",
                                "right": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 3983,
                                    "end": 3993
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "_Fragment",
                                      "start": 3994,
                                      "end": 4003
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 4005,
                                      "end": 4009
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 4011,
                                      "end": 4015
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 4031,
                                        "end": 4041
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "div",
                                          "raw": "\"div\"",
                                          "start": 4042,
                                          "end": 4047
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 4049,
                                          "end": 4053
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 4055,
                                          "end": 4059
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 4061,
                                          "end": 4065
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 4067,
                                          "end": 4068
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 4070,
                                          "end": 4074
                                        }
                                      ],
                                      "optional": false,
                                      "start": 4031,
                                      "end": 4075
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 4077,
                                      "end": 4078
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_26",
                                      "raw": "\"u6_26\"",
                                      "start": 4080,
                                      "end": 4087
                                    }
                                  ],
                                  "optional": false,
                                  "start": 3983,
                                  "end": 4088
                                },
                                "start": 3955,
                                "end": 4088
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4090,
                                "end": 4091
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4093,
                                "end": 4097
                              }
                            ],
                            "optional": false,
                            "start": 3925,
                            "end": 4098
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4122,
                              "end": 4132
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4133,
                                "end": 4138
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4140,
                                "end": 4144
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4146,
                                "end": 4150
                              },
                              {
                                "type": "LogicalExpression",
                                "left": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "prop",
                                    "start": 4152,
                                    "end": 4156
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "value",
                                    "start": 4157,
                                    "end": 4162
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 4152,
                                  "end": 4162
                                },
                                "operator": "&&",
                                "right": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 4180,
                                    "end": 4190
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "Image",
                                      "start": 4191,
                                      "end": 4196
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 4198,
                                      "end": 4202
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 4204,
                                      "end": 4208
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 4210,
                                      "end": 4214
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 4216,
                                      "end": 4217
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "u6_27",
                                      "raw": "\"u6_27\"",
                                      "start": 4219,
                                      "end": 4226
                                    }
                                  ],
                                  "optional": false,
                                  "start": 4180,
                                  "end": 4227
                                },
                                "start": 4152,
                                "end": 4227
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4229,
                                "end": 4230
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4232,
                                "end": 4236
                              }
                            ],
                            "optional": false,
                            "start": 4122,
                            "end": 4237
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4261,
                              "end": 4271
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4272,
                                "end": 4277
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4279,
                                "end": 4283
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4285,
                                "end": 4289
                              },
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "Static ",
                                    "raw": "\"Static \"",
                                    "start": 4305,
                                    "end": 4314
                                  },
                                  {
                                    "type": "ConditionalExpression",
                                    "test": {
                                      "type": "Identifier",
                                      "name": "f",
                                      "start": 4328,
                                      "end": 4329
                                    },
                                    "consequent": {
                                      "type": "Literal",
                                      "value": 1,
                                      "raw": "1",
                                      "start": 4332,
                                      "end": 4333
                                    },
                                    "alternate": {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 4336,
                                      "end": 4337
                                    },
                                    "start": 4328,
                                    "end": 4337
                                  }
                                ],
                                "start": 4291,
                                "end": 4347
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4349,
                                "end": 4350
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4352,
                                "end": 4356
                              }
                            ],
                            "optional": false,
                            "start": 4261,
                            "end": 4357
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4381,
                              "end": 4391
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4392,
                                "end": 4397
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4399,
                                "end": 4403
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4405,
                                "end": 4409
                              },
                              {
                                "type": "Literal",
                                "value": "Static",
                                "raw": "\"Static\"",
                                "start": 4411,
                                "end": 4419
                              },
                              {
                                "type": "Literal",
                                "value": 3,
                                "raw": "3",
                                "start": 4421,
                                "end": 4422
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4424,
                                "end": 4428
                              }
                            ],
                            "optional": false,
                            "start": 4381,
                            "end": 4429
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4453,
                              "end": 4463
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4464,
                                "end": 4469
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4471,
                                "end": 4475
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4477,
                                "end": 4481
                              },
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "Static ",
                                    "raw": "\"Static \"",
                                    "start": 4497,
                                    "end": 4506
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_wrapProp",
                                      "start": 4520,
                                      "end": 4529
                                    },
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "name": "props",
                                        "start": 4530,
                                        "end": 4535
                                      }
                                    ],
                                    "optional": false,
                                    "start": 4520,
                                    "end": 4536
                                  }
                                ],
                                "start": 4483,
                                "end": 4546
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4548,
                                "end": 4549
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4551,
                                "end": 4555
                              }
                            ],
                            "optional": false,
                            "start": 4453,
                            "end": 4556
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4580,
                              "end": 4590
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4591,
                                "end": 4596
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4598,
                                "end": 4602
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4604,
                                "end": 4608
                              },
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "Static ",
                                    "raw": "\"Static \"",
                                    "start": 4624,
                                    "end": 4633
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "stuff",
                                      "start": 4647,
                                      "end": 4652
                                    },
                                    "arguments": [],
                                    "optional": false,
                                    "start": 4647,
                                    "end": 4654
                                  }
                                ],
                                "start": 4610,
                                "end": 4664
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4666,
                                "end": 4667
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4669,
                                "end": 4673
                              }
                            ],
                            "optional": false,
                            "start": 4580,
                            "end": 4674
                          },
                          {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "name": "_jsxSorted",
                              "start": 4698,
                              "end": 4708
                            },
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "div",
                                "raw": "\"div\"",
                                "start": 4709,
                                "end": 4714
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4716,
                                "end": 4720
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4722,
                                "end": 4726
                              },
                              {
                                "type": "ArrayExpression",
                                "elements": [
                                  {
                                    "type": "Literal",
                                    "value": "Static ",
                                    "raw": "\"Static \"",
                                    "start": 4742,
                                    "end": 4751
                                  },
                                  {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "stuff",
                                      "start": 4765,
                                      "end": 4770
                                    },
                                    "arguments": [],
                                    "optional": false,
                                    "start": 4765,
                                    "end": 4772
                                  }
                                ],
                                "start": 4728,
                                "end": 4782
                              },
                              {
                                "type": "Literal",
                                "value": 1,
                                "raw": "1",
                                "start": 4784,
                                "end": 4785
                              },
                              {
                                "type": "Literal",
                                "value": null,
                                "raw": "null",
                                "start": 4787,
                                "end": 4791
                              }
                            ],
                            "optional": false,
                            "start": 4698,
                            "end": 4792
                          }
                        ],
                        "start": 3250,
                        "end": 4798
                      },
                      {
                        "type": "Literal",
                        "value": 1,
                        "raw": "1",
                        "start": 4800,
                        "end": 4801
                      },
                      {
                        "type": "Literal",
                        "value": "u6_28",
                        "raw": "\"u6_28\"",
                        "start": 4803,
                        "end": 4810
                      }
                    ],
                    "optional": false,
                    "start": 3216,
                    "end": 4811
                  },
                  "start": 3195,
                  "end": 4812
                }
              ],
              "start": 3189,
              "end": 4814
            },
            "id": null,
            "generator": false,
            "start": 3180,
            "end": 4814
          },
          "start": 3146,
          "end": 4814
        }
      ],
      "start": 3140,
      "end": 4815
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
              "name": "AppStatic",
              "start": 4829,
              "end": 4838
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 4855,
                "end": 4867
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 4882,
                    "end": 4892
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "AppStatic_component_gYRXqF3G5nE",
                      "start": 4893,
                      "end": 4924
                    },
                    {
                      "type": "Literal",
                      "value": "AppStatic_component_gYRXqF3G5nE",
                      "raw": "\"AppStatic_component_gYRXqF3G5nE\"",
                      "start": 4926,
                      "end": 4959
                    }
                  ],
                  "optional": false,
                  "start": 4882,
                  "end": 4960
                }
              ],
              "optional": false,
              "start": 4855,
              "end": 4961
            },
            "start": 4829,
            "end": 4961
          }
        ],
        "start": 4823,
        "end": 4962
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 4816,
      "end": 4962
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 4962
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_wrapProp()`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 55 call expressions

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 6 |
| `inlinedQrl` | `test.js` | `@qwik.dev/core` | 6 |
| `_jsxSorted` | `test.js` | `@qwik.dev/core` | 46 |
| `_wrapProp` | `test.js` | `@qwik.dev/core` | 2 |

## Diagnostics

```json
[]
```
