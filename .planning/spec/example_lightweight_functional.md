# Test: example_lightweight_functional

## Test Configuration

| Option | Value |
|--------|-------|
| *(all defaults)* | |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

export const Foo = component$((props) => {
	return (
		<div>
			<Button {...props} />
			<ButtonArrow {...props} />
		</div>
	);
}, {
	tagName: "my-foo",
});

export function Button({text, color}) {
	return (
		<button onColor$={color} onClick$={()=>console.log(text, color)}>{text}</button>
	);
}

export const ButtonArrow = ({text, color}) => {
	return (
		<button onColor$={color} onClick$={()=>console.log(text, color)}>{text}</button>
	);
}
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
                  "params": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "props",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 80,
                      "end": 85
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
                                "start": 105,
                                "end": 108
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 104,
                              "end": 109
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 109,
                                "end": 113
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "Button",
                                    "start": 114,
                                    "end": 120
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
                                        "start": 125,
                                        "end": 130
                                      },
                                      "start": 121,
                                      "end": 131
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 113,
                                  "end": 134
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 113,
                                "end": 134
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 134,
                                "end": 138
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "ButtonArrow",
                                    "start": 139,
                                    "end": 150
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
                                        "start": 155,
                                        "end": 160
                                      },
                                      "start": 151,
                                      "end": 161
                                    }
                                  ],
                                  "selfClosing": true,
                                  "start": 138,
                                  "end": 164
                                },
                                "children": [],
                                "closingElement": null,
                                "start": 138,
                                "end": 164
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 164,
                                "end": 167
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 169,
                                "end": 172
                              },
                              "start": 167,
                              "end": 173
                            },
                            "start": 104,
                            "end": 173
                          },
                          "start": 100,
                          "end": 176
                        },
                        "start": 93,
                        "end": 177
                      }
                    ],
                    "start": 90,
                    "end": 179
                  },
                  "id": null,
                  "generator": false,
                  "start": 79,
                  "end": 179
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
                        "start": 184,
                        "end": 191
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 193,
                        "end": 201
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 184,
                      "end": 201
                    }
                  ],
                  "start": 181,
                  "end": 204
                }
              ],
              "optional": false,
              "start": 68,
              "end": 205
            },
            "definite": false,
            "start": 62,
            "end": 205
          }
        ],
        "declare": false,
        "start": 56,
        "end": 206
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 49,
      "end": 206
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Button",
          "optional": false,
          "typeAnnotation": null,
          "start": 224,
          "end": 230
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "ObjectPattern",
            "decorators": [],
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
                  "start": 232,
                  "end": 236
                },
                "value": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "text",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 232,
                  "end": 236
                },
                "method": false,
                "shorthand": true,
                "computed": false,
                "optional": false,
                "start": 232,
                "end": 236
              },
              {
                "type": "Property",
                "kind": "init",
                "key": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "color",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 238,
                  "end": 243
                },
                "value": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "color",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 238,
                  "end": 243
                },
                "method": false,
                "shorthand": true,
                "computed": false,
                "optional": false,
                "start": 238,
                "end": 243
              }
            ],
            "optional": false,
            "typeAnnotation": null,
            "start": 231,
            "end": 244
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
                      "name": "button",
                      "start": 261,
                      "end": 267
                    },
                    "typeArguments": null,
                    "attributes": [
                      {
                        "type": "JSXAttribute",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "onColor$",
                          "start": 268,
                          "end": 276
                        },
                        "value": {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "color",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 278,
                            "end": 283
                          },
                          "start": 277,
                          "end": 284
                        },
                        "start": 268,
                        "end": 284
                      },
                      {
                        "type": "JSXAttribute",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "onClick$",
                          "start": 285,
                          "end": 293
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
                                  "start": 299,
                                  "end": 306
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "log",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 307,
                                  "end": 310
                                },
                                "optional": false,
                                "computed": false,
                                "start": 299,
                                "end": 310
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "text",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 311,
                                  "end": 315
                                },
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "color",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 317,
                                  "end": 322
                                }
                              ],
                              "optional": false,
                              "start": 299,
                              "end": 323
                            },
                            "id": null,
                            "generator": false,
                            "start": 295,
                            "end": 323
                          },
                          "start": 294,
                          "end": 324
                        },
                        "start": 285,
                        "end": 324
                      }
                    ],
                    "selfClosing": false,
                    "start": 260,
                    "end": 325
                  },
                  "children": [
                    {
                      "type": "JSXExpressionContainer",
                      "expression": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "text",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 326,
                        "end": 330
                      },
                      "start": 325,
                      "end": 331
                    }
                  ],
                  "closingElement": {
                    "type": "JSXClosingElement",
                    "name": {
                      "type": "JSXIdentifier",
                      "name": "button",
                      "start": 333,
                      "end": 339
                    },
                    "start": 331,
                    "end": 340
                  },
                  "start": 260,
                  "end": 340
                },
                "start": 256,
                "end": 343
              },
              "start": 249,
              "end": 344
            }
          ],
          "start": 246,
          "end": 346
        },
        "expression": false,
        "start": 215,
        "end": 346
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 208,
      "end": 346
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
              "name": "ButtonArrow",
              "optional": false,
              "typeAnnotation": null,
              "start": 361,
              "end": 372
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "typeParameters": null,
              "params": [
                {
                  "type": "ObjectPattern",
                  "decorators": [],
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
                        "start": 377,
                        "end": 381
                      },
                      "value": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "text",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 377,
                        "end": 381
                      },
                      "method": false,
                      "shorthand": true,
                      "computed": false,
                      "optional": false,
                      "start": 377,
                      "end": 381
                    },
                    {
                      "type": "Property",
                      "kind": "init",
                      "key": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "color",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 383,
                        "end": 388
                      },
                      "value": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "color",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 383,
                        "end": 388
                      },
                      "method": false,
                      "shorthand": true,
                      "computed": false,
                      "optional": false,
                      "start": 383,
                      "end": 388
                    }
                  ],
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 376,
                  "end": 389
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
                            "name": "button",
                            "start": 409,
                            "end": 415
                          },
                          "typeArguments": null,
                          "attributes": [
                            {
                              "type": "JSXAttribute",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "onColor$",
                                "start": 416,
                                "end": 424
                              },
                              "value": {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "color",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 426,
                                  "end": 431
                                },
                                "start": 425,
                                "end": 432
                              },
                              "start": 416,
                              "end": 432
                            },
                            {
                              "type": "JSXAttribute",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "onClick$",
                                "start": 433,
                                "end": 441
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
                                        "start": 447,
                                        "end": 454
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "log",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 455,
                                        "end": 458
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 447,
                                      "end": 458
                                    },
                                    "typeArguments": null,
                                    "arguments": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "text",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 459,
                                        "end": 463
                                      },
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "color",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 465,
                                        "end": 470
                                      }
                                    ],
                                    "optional": false,
                                    "start": 447,
                                    "end": 471
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 443,
                                  "end": 471
                                },
                                "start": 442,
                                "end": 472
                              },
                              "start": 433,
                              "end": 472
                            }
                          ],
                          "selfClosing": false,
                          "start": 408,
                          "end": 473
                        },
                        "children": [
                          {
                            "type": "JSXExpressionContainer",
                            "expression": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "text",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 474,
                              "end": 478
                            },
                            "start": 473,
                            "end": 479
                          }
                        ],
                        "closingElement": {
                          "type": "JSXClosingElement",
                          "name": {
                            "type": "JSXIdentifier",
                            "name": "button",
                            "start": 481,
                            "end": 487
                          },
                          "start": 479,
                          "end": 488
                        },
                        "start": 408,
                        "end": 488
                      },
                      "start": 404,
                      "end": 491
                    },
                    "start": 397,
                    "end": 492
                  }
                ],
                "start": 394,
                "end": 494
              },
              "id": null,
              "generator": false,
              "start": 375,
              "end": 494
            },
            "definite": false,
            "start": 361,
            "end": 494
          }
        ],
        "declare": false,
        "start": 355,
        "end": 494
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 348,
      "end": 494
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 494
}
```

</details>

## Output

### Module: test.tsx_Foo_component_HTDRsvUbLiE.tsx (ENTRY POINT)

```tsx
import { Button } from "./test";
import { ButtonArrow } from "./test";
export const Foo_component_HTDRsvUbLiE = (props)=>{
    return <div>
			<Button {...props}/>
			<ButtonArrow {...props}/>
		</div>;
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
            "name": "Button",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "Button",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 15
          },
          "importKind": "value",
          "start": 9,
          "end": 15
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 23,
        "end": 31
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 32
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "ButtonArrow",
            "optional": false,
            "typeAnnotation": null,
            "start": 42,
            "end": 53
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "ButtonArrow",
            "optional": false,
            "typeAnnotation": null,
            "start": 42,
            "end": 53
          },
          "importKind": "value",
          "start": 42,
          "end": 53
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 61,
        "end": 69
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 33,
      "end": 70
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
              "name": "Foo_component_HTDRsvUbLiE",
              "optional": false,
              "typeAnnotation": null,
              "start": 84,
              "end": 109
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
                  "start": 113,
                  "end": 118
                }
              ],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 135,
                          "end": 138
                        },
                        "typeArguments": null,
                        "attributes": [],
                        "selfClosing": false,
                        "start": 134,
                        "end": 139
                      },
                      "children": [
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 139,
                          "end": 143
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "Button",
                              "start": 144,
                              "end": 150
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
                                  "start": 155,
                                  "end": 160
                                },
                                "start": 151,
                                "end": 161
                              }
                            ],
                            "selfClosing": true,
                            "start": 143,
                            "end": 163
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 143,
                          "end": 163
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t\t",
                          "raw": "\n\t\t\t",
                          "start": 163,
                          "end": 167
                        },
                        {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "ButtonArrow",
                              "start": 168,
                              "end": 179
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
                                  "start": 184,
                                  "end": 189
                                },
                                "start": 180,
                                "end": 190
                              }
                            ],
                            "selfClosing": true,
                            "start": 167,
                            "end": 192
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 167,
                          "end": 192
                        },
                        {
                          "type": "JSXText",
                          "value": "\n\t\t",
                          "raw": "\n\t\t",
                          "start": 192,
                          "end": 195
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "div",
                          "start": 197,
                          "end": 200
                        },
                        "start": 195,
                        "end": 201
                      },
                      "start": 134,
                      "end": 201
                    },
                    "start": 127,
                    "end": 202
                  }
                ],
                "start": 121,
                "end": 204
              },
              "id": null,
              "generator": false,
              "start": 112,
              "end": 204
            },
            "definite": false,
            "start": 84,
            "end": 204
          }
        ],
        "declare": false,
        "start": 78,
        "end": 205
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 71,
      "end": 205
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 205
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
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    81,
    181
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: test.tsx_Button_button_q_e_click_6YaNiKLqRnQ.tsx (ENTRY POINT)

```tsx
import { _captures } from "@qwik.dev/core";
export const Button_button_q_e_click_6YaNiKLqRnQ = ()=>{
    const color = _captures[0], text = _captures[1];
    return console.log(text, color);
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
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "importKind": "value",
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
      "importKind": "value",
      "start": 0,
      "end": 43
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
              "name": "Button_button_q_e_click_6YaNiKLqRnQ",
              "optional": false,
              "typeAnnotation": null,
              "start": 57,
              "end": 92
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
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "color",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 111,
                          "end": 116
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_captures",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 119,
                            "end": 128
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 129,
                            "end": 130
                          },
                          "optional": false,
                          "computed": true,
                          "start": 119,
                          "end": 131
                        },
                        "definite": false,
                        "start": 111,
                        "end": 131
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "text",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 133,
                          "end": 137
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_captures",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 140,
                            "end": 149
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 150,
                            "end": 151
                          },
                          "optional": false,
                          "computed": true,
                          "start": 140,
                          "end": 152
                        },
                        "definite": false,
                        "start": 133,
                        "end": 152
                      }
                    ],
                    "declare": false,
                    "start": 105,
                    "end": 153
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "console",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 165,
                          "end": 172
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "log",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 173,
                          "end": 176
                        },
                        "optional": false,
                        "computed": false,
                        "start": 165,
                        "end": 176
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "text",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 177,
                          "end": 181
                        },
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "color",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 183,
                          "end": 188
                        }
                      ],
                      "optional": false,
                      "start": 165,
                      "end": 189
                    },
                    "start": 158,
                    "end": 190
                  }
                ],
                "start": 99,
                "end": 192
              },
              "id": null,
              "generator": false,
              "start": 95,
              "end": 192
            },
            "definite": false,
            "start": 57,
            "end": 192
          }
        ],
        "declare": false,
        "start": 51,
        "end": 193
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 44,
      "end": 193
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 193
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Button_button_q_e_click_6YaNiKLqRnQ",
  "entry": null,
  "displayName": "test.tsx_Button_button_q_e_click",
  "hash": "6YaNiKLqRnQ",
  "canonicalFilename": "test.tsx_Button_button_q_e_click_6YaNiKLqRnQ",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    297,
    325
  ],
  "captureNames": [
    "color",
    "text"
  ]
}
```

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_6YaNiKLqRnQ = ()=>import("./test.tsx_Button_button_q_e_click_6YaNiKLqRnQ");
const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE");
const i_rEE0GCaea7M = ()=>import("./test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M");
export const Foo = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HTDRsvUbLiE, "Foo_component_HTDRsvUbLiE"), {
    tagName: "my-foo"
});
export function Button({ text, color }) {
    return <button q-e:color={color} q-e:click={/*#__PURE__*/ qrl(i_6YaNiKLqRnQ, "Button_button_q_e_click_6YaNiKLqRnQ", [
        color,
        text
    ])}>{text}</button>;
}
export const ButtonArrow = (_rawProps)=>{
    return <button q-e:color={_rawProps.color} q-e:click={/*#__PURE__*/ qrl(i_rEE0GCaea7M, "ButtonArrow_button_q_e_click_rEE0GCaea7M", [
        _rawProps
    ])}>{_rawProps.text}</button>;
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
            "name": "i_6YaNiKLqRnQ",
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
                "value": "./test.tsx_Button_button_q_e_click_6YaNiKLqRnQ",
                "raw": "\"./test.tsx_Button_button_q_e_click_6YaNiKLqRnQ\"",
                "start": 118,
                "end": 166
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 167
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 167
          },
          "definite": false,
          "start": 91,
          "end": 167
        }
      ],
      "declare": false,
      "start": 85,
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
            "decorators": [],
            "name": "i_HTDRsvUbLiE",
            "optional": false,
            "typeAnnotation": null,
            "start": 175,
            "end": 188
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
                "value": "./test.tsx_Foo_component_HTDRsvUbLiE",
                "raw": "\"./test.tsx_Foo_component_HTDRsvUbLiE\"",
                "start": 202,
                "end": 240
              },
              "options": null,
              "phase": null,
              "start": 195,
              "end": 241
            },
            "id": null,
            "generator": false,
            "start": 191,
            "end": 241
          },
          "definite": false,
          "start": 175,
          "end": 241
        }
      ],
      "declare": false,
      "start": 169,
      "end": 242
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
            "name": "i_rEE0GCaea7M",
            "optional": false,
            "typeAnnotation": null,
            "start": 249,
            "end": 262
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
                "value": "./test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M",
                "raw": "\"./test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M\"",
                "start": 276,
                "end": 329
              },
              "options": null,
              "phase": null,
              "start": 269,
              "end": 330
            },
            "id": null,
            "generator": false,
            "start": 265,
            "end": 330
          },
          "definite": false,
          "start": 249,
          "end": 330
        }
      ],
      "declare": false,
      "start": 243,
      "end": 331
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
              "start": 345,
              "end": 348
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 365,
                "end": 377
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
                    "start": 392,
                    "end": 395
                  },
                  "typeArguments": null,
                  "arguments": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "i_HTDRsvUbLiE",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 396,
                      "end": 409
                    },
                    {
                      "type": "Literal",
                      "value": "Foo_component_HTDRsvUbLiE",
                      "raw": "\"Foo_component_HTDRsvUbLiE\"",
                      "start": 411,
                      "end": 438
                    }
                  ],
                  "optional": false,
                  "start": 392,
                  "end": 439
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
                        "start": 447,
                        "end": 454
                      },
                      "value": {
                        "type": "Literal",
                        "value": "my-foo",
                        "raw": "\"my-foo\"",
                        "start": 456,
                        "end": 464
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "optional": false,
                      "start": 447,
                      "end": 464
                    }
                  ],
                  "start": 441,
                  "end": 466
                }
              ],
              "optional": false,
              "start": 365,
              "end": 467
            },
            "definite": false,
            "start": 345,
            "end": 467
          }
        ],
        "declare": false,
        "start": 339,
        "end": 468
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 332,
      "end": 468
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "FunctionDeclaration",
        "id": {
          "type": "Identifier",
          "decorators": [],
          "name": "Button",
          "optional": false,
          "typeAnnotation": null,
          "start": 485,
          "end": 491
        },
        "generator": false,
        "async": false,
        "declare": false,
        "typeParameters": null,
        "params": [
          {
            "type": "ObjectPattern",
            "decorators": [],
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
                  "start": 494,
                  "end": 498
                },
                "value": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "text",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 494,
                  "end": 498
                },
                "method": false,
                "shorthand": true,
                "computed": false,
                "optional": false,
                "start": 494,
                "end": 498
              },
              {
                "type": "Property",
                "kind": "init",
                "key": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "color",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 500,
                  "end": 505
                },
                "value": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "color",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 500,
                  "end": 505
                },
                "method": false,
                "shorthand": true,
                "computed": false,
                "optional": false,
                "start": 500,
                "end": 505
              }
            ],
            "optional": false,
            "typeAnnotation": null,
            "start": 492,
            "end": 507
          }
        ],
        "returnType": null,
        "body": {
          "type": "BlockStatement",
          "body": [
            {
              "type": "ReturnStatement",
              "argument": {
                "type": "JSXElement",
                "openingElement": {
                  "type": "JSXOpeningElement",
                  "name": {
                    "type": "JSXIdentifier",
                    "name": "button",
                    "start": 523,
                    "end": 529
                  },
                  "typeArguments": null,
                  "attributes": [
                    {
                      "type": "JSXAttribute",
                      "name": {
                        "type": "JSXNamespacedName",
                        "namespace": {
                          "type": "JSXIdentifier",
                          "name": "q-e",
                          "start": 530,
                          "end": 533
                        },
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "color",
                          "start": 534,
                          "end": 539
                        },
                        "start": 530,
                        "end": 539
                      },
                      "value": {
                        "type": "JSXExpressionContainer",
                        "expression": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "color",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 541,
                          "end": 546
                        },
                        "start": 540,
                        "end": 547
                      },
                      "start": 530,
                      "end": 547
                    },
                    {
                      "type": "JSXAttribute",
                      "name": {
                        "type": "JSXNamespacedName",
                        "namespace": {
                          "type": "JSXIdentifier",
                          "name": "q-e",
                          "start": 548,
                          "end": 551
                        },
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "click",
                          "start": 552,
                          "end": 557
                        },
                        "start": 548,
                        "end": 557
                      },
                      "value": {
                        "type": "JSXExpressionContainer",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "qrl",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 573,
                            "end": 576
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "i_6YaNiKLqRnQ",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 577,
                              "end": 590
                            },
                            {
                              "type": "Literal",
                              "value": "Button_button_q_e_click_6YaNiKLqRnQ",
                              "raw": "\"Button_button_q_e_click_6YaNiKLqRnQ\"",
                              "start": 592,
                              "end": 629
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "color",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 641,
                                  "end": 646
                                },
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "text",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 656,
                                  "end": 660
                                }
                              ],
                              "start": 631,
                              "end": 666
                            }
                          ],
                          "optional": false,
                          "start": 573,
                          "end": 667
                        },
                        "start": 558,
                        "end": 668
                      },
                      "start": 548,
                      "end": 668
                    }
                  ],
                  "selfClosing": false,
                  "start": 522,
                  "end": 669
                },
                "children": [
                  {
                    "type": "JSXExpressionContainer",
                    "expression": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "text",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 670,
                      "end": 674
                    },
                    "start": 669,
                    "end": 675
                  }
                ],
                "closingElement": {
                  "type": "JSXClosingElement",
                  "name": {
                    "type": "JSXIdentifier",
                    "name": "button",
                    "start": 677,
                    "end": 683
                  },
                  "start": 675,
                  "end": 684
                },
                "start": 522,
                "end": 684
              },
              "start": 515,
              "end": 685
            }
          ],
          "start": 509,
          "end": 687
        },
        "expression": false,
        "start": 476,
        "end": 687
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 469,
      "end": 687
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
              "name": "ButtonArrow",
              "optional": false,
              "typeAnnotation": null,
              "start": 701,
              "end": 712
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
                  "name": "_rawProps",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 716,
                  "end": 725
                }
              ],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "JSXElement",
                      "openingElement": {
                        "type": "JSXOpeningElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "button",
                          "start": 742,
                          "end": 748
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXNamespacedName",
                              "namespace": {
                                "type": "JSXIdentifier",
                                "name": "q-e",
                                "start": 749,
                                "end": 752
                              },
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "color",
                                "start": 753,
                                "end": 758
                              },
                              "start": 749,
                              "end": 758
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "_rawProps",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 760,
                                  "end": 769
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "color",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 770,
                                  "end": 775
                                },
                                "optional": false,
                                "computed": false,
                                "start": 760,
                                "end": 775
                              },
                              "start": 759,
                              "end": 776
                            },
                            "start": 749,
                            "end": 776
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXNamespacedName",
                              "namespace": {
                                "type": "JSXIdentifier",
                                "name": "q-e",
                                "start": 777,
                                "end": 780
                              },
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "click",
                                "start": 781,
                                "end": 786
                              },
                              "start": 777,
                              "end": 786
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "qrl",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 802,
                                  "end": 805
                                },
                                "typeArguments": null,
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "i_rEE0GCaea7M",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 806,
                                    "end": 819
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "ButtonArrow_button_q_e_click_rEE0GCaea7M",
                                    "raw": "\"ButtonArrow_button_q_e_click_rEE0GCaea7M\"",
                                    "start": 821,
                                    "end": 863
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "_rawProps",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 875,
                                        "end": 884
                                      }
                                    ],
                                    "start": 865,
                                    "end": 890
                                  }
                                ],
                                "optional": false,
                                "start": 802,
                                "end": 891
                              },
                              "start": 787,
                              "end": 892
                            },
                            "start": 777,
                            "end": 892
                          }
                        ],
                        "selfClosing": false,
                        "start": 741,
                        "end": 893
                      },
                      "children": [
                        {
                          "type": "JSXExpressionContainer",
                          "expression": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "_rawProps",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 894,
                              "end": 903
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "text",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 904,
                              "end": 908
                            },
                            "optional": false,
                            "computed": false,
                            "start": 894,
                            "end": 908
                          },
                          "start": 893,
                          "end": 909
                        }
                      ],
                      "closingElement": {
                        "type": "JSXClosingElement",
                        "name": {
                          "type": "JSXIdentifier",
                          "name": "button",
                          "start": 911,
                          "end": 917
                        },
                        "start": 909,
                        "end": 918
                      },
                      "start": 741,
                      "end": 918
                    },
                    "start": 734,
                    "end": 919
                  }
                ],
                "start": 728,
                "end": 921
              },
              "id": null,
              "generator": false,
              "start": 715,
              "end": 921
            },
            "definite": false,
            "start": 701,
            "end": 921
          }
        ],
        "declare": false,
        "start": 695,
        "end": 922
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 688,
      "end": 922
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 922
}
```

</details>

### Module: test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M.tsx (ENTRY POINT)

```tsx
import { _captures } from "@qwik.dev/core";
export const ButtonArrow_button_q_e_click_rEE0GCaea7M = ()=>{
    const _rawProps = _captures[0];
    return console.log(_rawProps.text, _rawProps.color);
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
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "importKind": "value",
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
      "importKind": "value",
      "start": 0,
      "end": 43
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
              "name": "ButtonArrow_button_q_e_click_rEE0GCaea7M",
              "optional": false,
              "typeAnnotation": null,
              "start": 57,
              "end": 97
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
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "_rawProps",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 116,
                          "end": 125
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_captures",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 128,
                            "end": 137
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 138,
                            "end": 139
                          },
                          "optional": false,
                          "computed": true,
                          "start": 128,
                          "end": 140
                        },
                        "definite": false,
                        "start": 116,
                        "end": 140
                      }
                    ],
                    "declare": false,
                    "start": 110,
                    "end": 141
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "console",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 153,
                          "end": 160
                        },
                        "property": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "log",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 161,
                          "end": 164
                        },
                        "optional": false,
                        "computed": false,
                        "start": 153,
                        "end": 164
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_rawProps",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 165,
                            "end": 174
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "text",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 175,
                            "end": 179
                          },
                          "optional": false,
                          "computed": false,
                          "start": 165,
                          "end": 179
                        },
                        {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "_rawProps",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 181,
                            "end": 190
                          },
                          "property": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "color",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 191,
                            "end": 196
                          },
                          "optional": false,
                          "computed": false,
                          "start": 181,
                          "end": 196
                        }
                      ],
                      "optional": false,
                      "start": 153,
                      "end": 197
                    },
                    "start": 146,
                    "end": 198
                  }
                ],
                "start": 104,
                "end": 200
              },
              "id": null,
              "generator": false,
              "start": 100,
              "end": 200
            },
            "definite": false,
            "start": 57,
            "end": 200
          }
        ],
        "declare": false,
        "start": 51,
        "end": 201
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 44,
      "end": 201
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 201
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "ButtonArrow_button_q_e_click_rEE0GCaea7M",
  "entry": null,
  "displayName": "test.tsx_ButtonArrow_button_q_e_click",
  "hash": "rEE0GCaea7M",
  "canonicalFilename": "test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    445,
    473
  ],
  "captureNames": [
    "_rawProps"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (4 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point segment(s)
- **[CONV-11] Props Destructuring**: Arrow function component props converted to `_rawProps` pattern

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_captures[]` | test.tsx_Button_button_q_e_click_6YaNiKLqRnQ.tsx | @qwik.dev/core | 2 |
| `qrl` | test.tsx | @qwik.dev/core | 3 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
