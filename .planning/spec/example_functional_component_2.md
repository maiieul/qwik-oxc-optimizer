# Test: example_functional_component_2

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { $, component$, useStore } from '@qwik.dev/core';
export const useCounter = () => {
	return useStore({count: 0});
}

export const STEP = 1;

export const App = component$((props) => {
	const state = useCounter();
	const thing = useStore({thing: 0});
	const STEP_2 = 2;

	const count2 = state.count * 2;
	return (
		<div onClick$={() => state.count+=count2 }>
			<span>{state.count}</span>
			{buttons.map(btn => (
				<button
					onClick$={() => state.count += btn.offset + thing + STEP + STEP_2 + props.step}
				>
					{btn.name}
				</button>
			))}

		</div>

	);
})
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
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 24,
            "end": 32
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
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
              "name": "useCounter",
              "optional": false,
              "typeAnnotation": null,
              "start": 71,
              "end": 81
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
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "useStore",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 100,
                        "end": 108
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
                                "name": "count",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 110,
                                "end": 115
                              },
                              "value": {
                                "type": "Literal",
                                "value": 0,
                                "raw": "0",
                                "start": 117,
                                "end": 118
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "optional": false,
                              "start": 110,
                              "end": 118
                            }
                          ],
                          "start": 109,
                          "end": 119
                        }
                      ],
                      "optional": false,
                      "start": 100,
                      "end": 120
                    },
                    "start": 93,
                    "end": 121
                  }
                ],
                "start": 90,
                "end": 123
              },
              "id": null,
              "generator": false,
              "start": 84,
              "end": 123
            },
            "definite": false,
            "start": 71,
            "end": 123
          }
        ],
        "declare": false,
        "start": 65,
        "end": 123
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 58,
      "end": 123
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
              "name": "STEP",
              "optional": false,
              "typeAnnotation": null,
              "start": 138,
              "end": 142
            },
            "init": {
              "type": "Literal",
              "value": 1,
              "raw": "1",
              "start": 145,
              "end": 146
            },
            "definite": false,
            "start": 138,
            "end": 146
          }
        ],
        "declare": false,
        "start": 132,
        "end": 147
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 125,
      "end": 147
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
              "start": 162,
              "end": 165
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 168,
                "end": 178
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
                      "start": 180,
                      "end": 185
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
                              "type": "Identifier",
                              "decorators": [],
                              "name": "state",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 199,
                              "end": 204
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useCounter",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 207,
                                "end": 217
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 207,
                              "end": 219
                            },
                            "definite": false,
                            "start": 199,
                            "end": 219
                          }
                        ],
                        "declare": false,
                        "start": 193,
                        "end": 220
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
                              "name": "thing",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 228,
                              "end": 233
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 236,
                                "end": 244
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
                                        "name": "thing",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 246,
                                        "end": 251
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 253,
                                        "end": 254
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 246,
                                      "end": 254
                                    }
                                  ],
                                  "start": 245,
                                  "end": 255
                                }
                              ],
                              "optional": false,
                              "start": 236,
                              "end": 256
                            },
                            "definite": false,
                            "start": 228,
                            "end": 256
                          }
                        ],
                        "declare": false,
                        "start": 222,
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
                              "name": "STEP_2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 265,
                              "end": 271
                            },
                            "init": {
                              "type": "Literal",
                              "value": 2,
                              "raw": "2",
                              "start": 274,
                              "end": 275
                            },
                            "definite": false,
                            "start": 265,
                            "end": 275
                          }
                        ],
                        "declare": false,
                        "start": 259,
                        "end": 276
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
                              "name": "count2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 285,
                              "end": 291
                            },
                            "init": {
                              "type": "BinaryExpression",
                              "left": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "state",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 294,
                                  "end": 299
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "count",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 300,
                                  "end": 305
                                },
                                "optional": false,
                                "computed": false,
                                "start": 294,
                                "end": 305
                              },
                              "operator": "*",
                              "right": {
                                "type": "Literal",
                                "value": 2,
                                "raw": "2",
                                "start": 308,
                                "end": 309
                              },
                              "start": 294,
                              "end": 309
                            },
                            "definite": false,
                            "start": 285,
                            "end": 309
                          }
                        ],
                        "declare": false,
                        "start": 279,
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
                                "start": 324,
                                "end": 327
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 328,
                                    "end": 336
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
                                        "type": "AssignmentExpression",
                                        "operator": "+=",
                                        "left": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "state",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 344,
                                            "end": 349
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "count",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 350,
                                            "end": 355
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 344,
                                          "end": 355
                                        },
                                        "right": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "count2",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 357,
                                          "end": 363
                                        },
                                        "start": 344,
                                        "end": 363
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 338,
                                      "end": 363
                                    },
                                    "start": 337,
                                    "end": 365
                                  },
                                  "start": 328,
                                  "end": 365
                                }
                              ],
                              "selfClosing": false,
                              "start": 323,
                              "end": 366
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 366,
                                "end": 370
                              },
                              {
                                "type": "JSXElement",
                                "openingElement": {
                                  "type": "JSXOpeningElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "span",
                                    "start": 371,
                                    "end": 375
                                  },
                                  "typeArguments": null,
                                  "attributes": [],
                                  "selfClosing": false,
                                  "start": 370,
                                  "end": 376
                                },
                                "children": [
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
                                        "start": 377,
                                        "end": 382
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "count",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 383,
                                        "end": 388
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 377,
                                      "end": 388
                                    },
                                    "start": 376,
                                    "end": 389
                                  }
                                ],
                                "closingElement": {
                                  "type": "JSXClosingElement",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "span",
                                    "start": 391,
                                    "end": 395
                                  },
                                  "start": 389,
                                  "end": 396
                                },
                                "start": 370,
                                "end": 396
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t\t\t",
                                "raw": "\n\t\t\t",
                                "start": 396,
                                "end": 400
                              },
                              {
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "buttons",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 401,
                                      "end": 408
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "map",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 409,
                                      "end": 412
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 401,
                                    "end": 412
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
                                          "name": "btn",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 413,
                                          "end": 416
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
                                              "name": "button",
                                              "start": 427,
                                              "end": 433
                                            },
                                            "typeArguments": null,
                                            "attributes": [
                                              {
                                                "type": "JSXAttribute",
                                                "name": {
                                                  "type": "JSXIdentifier",
                                                  "name": "onClick$",
                                                  "start": 439,
                                                  "end": 447
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
                                                      "type": "AssignmentExpression",
                                                      "operator": "+=",
                                                      "left": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "state",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 455,
                                                          "end": 460
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "count",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 461,
                                                          "end": 466
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 455,
                                                        "end": 466
                                                      },
                                                      "right": {
                                                        "type": "BinaryExpression",
                                                        "left": {
                                                          "type": "BinaryExpression",
                                                          "left": {
                                                            "type": "BinaryExpression",
                                                            "left": {
                                                              "type": "BinaryExpression",
                                                              "left": {
                                                                "type": "MemberExpression",
                                                                "object": {
                                                                  "type": "Identifier",
                                                                  "decorators": [],
                                                                  "name": "btn",
                                                                  "optional": false,
                                                                  "typeAnnotation": null,
                                                                  "start": 470,
                                                                  "end": 473
                                                                },
                                                                "property": {
                                                                  "type": "Identifier",
                                                                  "decorators": [],
                                                                  "name": "offset",
                                                                  "optional": false,
                                                                  "typeAnnotation": null,
                                                                  "start": 474,
                                                                  "end": 480
                                                                },
                                                                "optional": false,
                                                                "computed": false,
                                                                "start": 470,
                                                                "end": 480
                                                              },
                                                              "operator": "+",
                                                              "right": {
                                                                "type": "Identifier",
                                                                "decorators": [],
                                                                "name": "thing",
                                                                "optional": false,
                                                                "typeAnnotation": null,
                                                                "start": 483,
                                                                "end": 488
                                                              },
                                                              "start": 470,
                                                              "end": 488
                                                            },
                                                            "operator": "+",
                                                            "right": {
                                                              "type": "Identifier",
                                                              "decorators": [],
                                                              "name": "STEP",
                                                              "optional": false,
                                                              "typeAnnotation": null,
                                                              "start": 491,
                                                              "end": 495
                                                            },
                                                            "start": 470,
                                                            "end": 495
                                                          },
                                                          "operator": "+",
                                                          "right": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "STEP_2",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 498,
                                                            "end": 504
                                                          },
                                                          "start": 470,
                                                          "end": 504
                                                        },
                                                        "operator": "+",
                                                        "right": {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "props",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 507,
                                                            "end": 512
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "step",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 513,
                                                            "end": 517
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 507,
                                                          "end": 517
                                                        },
                                                        "start": 470,
                                                        "end": 517
                                                      },
                                                      "start": 455,
                                                      "end": 517
                                                    },
                                                    "id": null,
                                                    "generator": false,
                                                    "start": 449,
                                                    "end": 517
                                                  },
                                                  "start": 448,
                                                  "end": 518
                                                },
                                                "start": 439,
                                                "end": 518
                                              }
                                            ],
                                            "selfClosing": false,
                                            "start": 426,
                                            "end": 524
                                          },
                                          "children": [
                                            {
                                              "type": "JSXText",
                                              "value": "\n\t\t\t\t\t",
                                              "raw": "\n\t\t\t\t\t",
                                              "start": 524,
                                              "end": 530
                                            },
                                            {
                                              "type": "JSXExpressionContainer",
                                              "expression": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "btn",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 531,
                                                  "end": 534
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "name",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 535,
                                                  "end": 539
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 531,
                                                "end": 539
                                              },
                                              "start": 530,
                                              "end": 540
                                            },
                                            {
                                              "type": "JSXText",
                                              "value": "\n\t\t\t\t",
                                              "raw": "\n\t\t\t\t",
                                              "start": 540,
                                              "end": 545
                                            }
                                          ],
                                          "closingElement": {
                                            "type": "JSXClosingElement",
                                            "name": {
                                              "type": "JSXIdentifier",
                                              "name": "button",
                                              "start": 547,
                                              "end": 553
                                            },
                                            "start": 545,
                                            "end": 554
                                          },
                                          "start": 426,
                                          "end": 554
                                        },
                                        "start": 420,
                                        "end": 559
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 413,
                                      "end": 559
                                    }
                                  ],
                                  "optional": false,
                                  "start": 401,
                                  "end": 560
                                },
                                "start": 400,
                                "end": 561
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\n\t\t",
                                "raw": "\n\n\t\t",
                                "start": 561,
                                "end": 565
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 567,
                                "end": 570
                              },
                              "start": 565,
                              "end": 571
                            },
                            "start": 323,
                            "end": 571
                          },
                          "start": 319,
                          "end": 575
                        },
                        "start": 312,
                        "end": 576
                      }
                    ],
                    "start": 190,
                    "end": 578
                  },
                  "id": null,
                  "generator": false,
                  "start": 179,
                  "end": 578
                }
              ],
              "optional": false,
              "start": 168,
              "end": 579
            },
            "definite": false,
            "start": 162,
            "end": 579
          }
        ],
        "declare": false,
        "start": 156,
        "end": 579
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 149,
      "end": 579
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 579
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
import { useStore } from '@qwik.dev/core';
export const useCounter = ()=>{
    return useStore({
        count: 0
    });
};
export const STEP = 1;
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
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
            "name": "i_ckEPmXZlub0",
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
          "start": 91,
          "end": 157
        }
      ],
      "start": 85,
      "end": 158
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 168,
            "end": 176
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 168,
            "end": 176
          },
          "start": 168,
          "end": 176
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 184,
        "end": 200
      },
      "phase": null,
      "attributes": [],
      "start": 159,
      "end": 201
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
              "name": "useCounter",
              "start": 215,
              "end": 225
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
                        "name": "useStore",
                        "start": 245,
                        "end": 253
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
                                "name": "count",
                                "start": 264,
                                "end": 269
                              },
                              "value": {
                                "type": "Literal",
                                "value": 0,
                                "raw": "0",
                                "start": 271,
                                "end": 272
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 264,
                              "end": 272
                            }
                          ],
                          "start": 254,
                          "end": 278
                        }
                      ],
                      "optional": false,
                      "start": 245,
                      "end": 279
                    },
                    "start": 238,
                    "end": 280
                  }
                ],
                "start": 232,
                "end": 282
              },
              "id": null,
              "generator": false,
              "start": 228,
              "end": 282
            },
            "start": 215,
            "end": 282
          }
        ],
        "start": 209,
        "end": 283
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 202,
      "end": 283
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
              "name": "STEP",
              "start": 297,
              "end": 301
            },
            "init": {
              "type": "Literal",
              "value": 1,
              "raw": "1",
              "start": 304,
              "end": 305
            },
            "start": 297,
            "end": 305
          }
        ],
        "start": 291,
        "end": 306
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 284,
      "end": 306
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
              "name": "App",
              "start": 320,
              "end": 323
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 340,
                "end": 352
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 367,
                    "end": 370
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_ckEPmXZlub0",
                      "start": 371,
                      "end": 384
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 386,
                      "end": 413
                    }
                  ],
                  "optional": false,
                  "start": 367,
                  "end": 414
                }
              ],
              "optional": false,
              "start": 340,
              "end": 415
            },
            "start": 320,
            "end": 415
          }
        ],
        "start": 314,
        "end": 416
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 307,
      "end": 416
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 416
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useCounter } from "./test";
import { useStore } from "@qwik.dev/core";
const i_UB6Fs5a3bd8 = ()=>import("./test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8");
const i_mi4E1piTWe8 = ()=>import("./test.tsx_App_component_div_q_e_click_mi4E1piTWe8");
export const App_component_ckEPmXZlub0 = (props)=>{
    const state = useCounter();
    const thing = useStore({
        thing: 0
    });
    const count2 = state.count * 2;
    const App_component_div_button_q_e_click_UB6Fs5a3bd8 = /*#__PURE__*/ qrl(i_UB6Fs5a3bd8, "App_component_div_button_q_e_click_UB6Fs5a3bd8", [
        props,
        state,
        thing
    ]);
    return /*#__PURE__*/ _jsxSorted("div", {
        "q-e:click": /*#__PURE__*/ qrl(i_mi4E1piTWe8, "App_component_div_q_e_click_mi4E1piTWe8", [
            count2,
            state
        ])
    }, null, [
        /*#__PURE__*/ _jsxSorted("span", null, null, _wrapProp(state, "count"), 3, null),
        buttons.map((btn)=>/*#__PURE__*/ _jsxSorted("button", {
                "q-e:click": App_component_div_button_q_e_click_UB6Fs5a3bd8,
                "q:p": btn
            }, null, _wrapProp(btn, "name"), 0, "u6_0"))
    ], 0, "u6_1");
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
            "name": "useCounter",
            "start": 136,
            "end": 146
          },
          "local": {
            "type": "Identifier",
            "name": "useCounter",
            "start": 136,
            "end": 146
          },
          "start": 136,
          "end": 146
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 154,
        "end": 162
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 163
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 173,
            "end": 181
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 173,
            "end": 181
          },
          "start": 173,
          "end": 181
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 189,
        "end": 205
      },
      "phase": null,
      "attributes": [],
      "start": 164,
      "end": 206
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_UB6Fs5a3bd8",
            "start": 213,
            "end": 226
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
                "value": "./test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8",
                "raw": "\"./test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8\"",
                "start": 240,
                "end": 299
              },
              "options": null,
              "phase": null,
              "start": 233,
              "end": 300
            },
            "id": null,
            "generator": false,
            "start": 229,
            "end": 300
          },
          "start": 213,
          "end": 300
        }
      ],
      "start": 207,
      "end": 301
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_mi4E1piTWe8",
            "start": 308,
            "end": 321
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
                "value": "./test.tsx_App_component_div_q_e_click_mi4E1piTWe8",
                "raw": "\"./test.tsx_App_component_div_q_e_click_mi4E1piTWe8\"",
                "start": 335,
                "end": 387
              },
              "options": null,
              "phase": null,
              "start": 328,
              "end": 388
            },
            "id": null,
            "generator": false,
            "start": 324,
            "end": 388
          },
          "start": 308,
          "end": 388
        }
      ],
      "start": 302,
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
              "name": "App_component_ckEPmXZlub0",
              "start": 403,
              "end": 428
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "props",
                  "start": 432,
                  "end": 437
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
                          "name": "state",
                          "start": 452,
                          "end": 457
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useCounter",
                            "start": 460,
                            "end": 470
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 460,
                          "end": 472
                        },
                        "start": 452,
                        "end": 472
                      }
                    ],
                    "start": 446,
                    "end": 473
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "thing",
                          "start": 484,
                          "end": 489
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 492,
                            "end": 500
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
                                    "name": "thing",
                                    "start": 511,
                                    "end": 516
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 518,
                                    "end": 519
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 511,
                                  "end": 519
                                }
                              ],
                              "start": 501,
                              "end": 525
                            }
                          ],
                          "optional": false,
                          "start": 492,
                          "end": 526
                        },
                        "start": 484,
                        "end": 526
                      }
                    ],
                    "start": 478,
                    "end": 527
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "count2",
                          "start": 538,
                          "end": 544
                        },
                        "init": {
                          "type": "BinaryExpression",
                          "left": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "name": "state",
                              "start": 547,
                              "end": 552
                            },
                            "property": {
                              "type": "Identifier",
                              "name": "count",
                              "start": 553,
                              "end": 558
                            },
                            "optional": false,
                            "computed": false,
                            "start": 547,
                            "end": 558
                          },
                          "operator": "*",
                          "right": {
                            "type": "Literal",
                            "value": 2,
                            "raw": "2",
                            "start": 561,
                            "end": 562
                          },
                          "start": 547,
                          "end": 562
                        },
                        "start": 538,
                        "end": 562
                      }
                    ],
                    "start": 532,
                    "end": 563
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "App_component_div_button_q_e_click_UB6Fs5a3bd8",
                          "start": 574,
                          "end": 620
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 637,
                            "end": 640
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_UB6Fs5a3bd8",
                              "start": 641,
                              "end": 654
                            },
                            {
                              "type": "Literal",
                              "value": "App_component_div_button_q_e_click_UB6Fs5a3bd8",
                              "raw": "\"App_component_div_button_q_e_click_UB6Fs5a3bd8\"",
                              "start": 656,
                              "end": 704
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "props",
                                  "start": 716,
                                  "end": 721
                                },
                                {
                                  "type": "Identifier",
                                  "name": "state",
                                  "start": 731,
                                  "end": 736
                                },
                                {
                                  "type": "Identifier",
                                  "name": "thing",
                                  "start": 746,
                                  "end": 751
                                }
                              ],
                              "start": 706,
                              "end": 757
                            }
                          ],
                          "optional": false,
                          "start": 637,
                          "end": 758
                        },
                        "start": 574,
                        "end": 758
                      }
                    ],
                    "start": 568,
                    "end": 759
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
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
                          "type": "ObjectExpression",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Literal",
                                "value": "q-e:click",
                                "raw": "\"q-e:click\"",
                                "start": 813,
                                "end": 824
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 840,
                                  "end": 843
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_mi4E1piTWe8",
                                    "start": 844,
                                    "end": 857
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "App_component_div_q_e_click_mi4E1piTWe8",
                                    "raw": "\"App_component_div_q_e_click_mi4E1piTWe8\"",
                                    "start": 859,
                                    "end": 900
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "name": "count2",
                                        "start": 916,
                                        "end": 922
                                      },
                                      {
                                        "type": "Identifier",
                                        "name": "state",
                                        "start": 936,
                                        "end": 941
                                      }
                                    ],
                                    "start": 902,
                                    "end": 951
                                  }
                                ],
                                "optional": false,
                                "start": 840,
                                "end": 952
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 813,
                              "end": 952
                            }
                          ],
                          "start": 803,
                          "end": 958
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 960,
                          "end": 964
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 990,
                                "end": 1000
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "span",
                                  "raw": "\"span\"",
                                  "start": 1001,
                                  "end": 1007
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1009,
                                  "end": 1013
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1015,
                                  "end": 1019
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_wrapProp",
                                    "start": 1021,
                                    "end": 1030
                                  },
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "name": "state",
                                      "start": 1031,
                                      "end": 1036
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "count",
                                      "raw": "\"count\"",
                                      "start": 1038,
                                      "end": 1045
                                    }
                                  ],
                                  "optional": false,
                                  "start": 1021,
                                  "end": 1046
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1048,
                                  "end": 1049
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1051,
                                  "end": 1055
                                }
                              ],
                              "optional": false,
                              "start": 990,
                              "end": 1056
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "buttons",
                                  "start": 1066,
                                  "end": 1073
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "map",
                                  "start": 1074,
                                  "end": 1077
                                },
                                "optional": false,
                                "computed": false,
                                "start": 1066,
                                "end": 1077
                              },
                              "arguments": [
                                {
                                  "type": "ArrowFunctionExpression",
                                  "expression": true,
                                  "async": false,
                                  "params": [
                                    {
                                      "type": "Identifier",
                                      "name": "btn",
                                      "start": 1079,
                                      "end": 1082
                                    }
                                  ],
                                  "body": {
                                    "type": "CallExpression",
                                    "callee": {
                                      "type": "Identifier",
                                      "name": "_jsxSorted",
                                      "start": 1099,
                                      "end": 1109
                                    },
                                    "arguments": [
                                      {
                                        "type": "Literal",
                                        "value": "button",
                                        "raw": "\"button\"",
                                        "start": 1110,
                                        "end": 1118
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
                                              "start": 1138,
                                              "end": 1149
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "App_component_div_button_q_e_click_UB6Fs5a3bd8",
                                              "start": 1151,
                                              "end": 1197
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1138,
                                            "end": 1197
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Literal",
                                              "value": "q:p",
                                              "raw": "\"q:p\"",
                                              "start": 1215,
                                              "end": 1220
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "name": "btn",
                                              "start": 1222,
                                              "end": 1225
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "start": 1215,
                                            "end": 1225
                                          }
                                        ],
                                        "start": 1120,
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
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "_wrapProp",
                                          "start": 1247,
                                          "end": 1256
                                        },
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "name": "btn",
                                            "start": 1257,
                                            "end": 1260
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "name",
                                            "raw": "\"name\"",
                                            "start": 1262,
                                            "end": 1268
                                          }
                                        ],
                                        "optional": false,
                                        "start": 1247,
                                        "end": 1269
                                      },
                                      {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 1271,
                                        "end": 1272
                                      },
                                      {
                                        "type": "Literal",
                                        "value": "u6_0",
                                        "raw": "\"u6_0\"",
                                        "start": 1274,
                                        "end": 1280
                                      }
                                    ],
                                    "optional": false,
                                    "start": 1099,
                                    "end": 1281
                                  },
                                  "id": null,
                                  "generator": false,
                                  "start": 1078,
                                  "end": 1281
                                }
                              ],
                              "optional": false,
                              "start": 1066,
                              "end": 1282
                            }
                          ],
                          "start": 966,
                          "end": 1288
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 1290,
                          "end": 1291
                        },
                        {
                          "type": "Literal",
                          "value": "u6_1",
                          "raw": "\"u6_1\"",
                          "start": 1293,
                          "end": 1299
                        }
                      ],
                      "optional": false,
                      "start": 785,
                      "end": 1300
                    },
                    "start": 764,
                    "end": 1301
                  }
                ],
                "start": 440,
                "end": 1303
              },
              "id": null,
              "generator": false,
              "start": 431,
              "end": 1303
            },
            "start": 403,
            "end": 1303
          }
        ],
        "start": 397,
        "end": 1304
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 390,
      "end": 1304
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1304
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_ckEPmXZlub0",
  "entry": null,
  "displayName": "test.tsx_App_component",
  "hash": "ckEPmXZlub0",
  "canonicalFilename": "test.tsx_App_component_ckEPmXZlub0",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    181,
    580
  ],
  "paramNames": [
    "props"
  ]
}
```

### Module: test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { STEP } from "./test";
export const App_component_div_button_q_e_click_UB6Fs5a3bd8 = (_, _1, btn)=>{
    const props = _captures[0], state = _captures[1], thing = _captures[2];
    return state.count += btn.offset + thing + STEP + 2 + props.step;
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
            "name": "STEP",
            "start": 53,
            "end": 57
          },
          "local": {
            "type": "Identifier",
            "name": "STEP",
            "start": 53,
            "end": 57
          },
          "start": 53,
          "end": 57
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 65,
        "end": 73
      },
      "phase": null,
      "attributes": [],
      "start": 44,
      "end": 74
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
              "name": "App_component_div_button_q_e_click_UB6Fs5a3bd8",
              "start": 88,
              "end": 134
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_",
                  "start": 138,
                  "end": 139
                },
                {
                  "type": "Identifier",
                  "name": "_1",
                  "start": 141,
                  "end": 143
                },
                {
                  "type": "Identifier",
                  "name": "btn",
                  "start": 145,
                  "end": 148
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
                          "name": "props",
                          "start": 163,
                          "end": 168
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 171,
                            "end": 180
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 181,
                            "end": 182
                          },
                          "optional": false,
                          "computed": true,
                          "start": 171,
                          "end": 183
                        },
                        "start": 163,
                        "end": 183
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 185,
                          "end": 190
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 193,
                            "end": 202
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 203,
                            "end": 204
                          },
                          "optional": false,
                          "computed": true,
                          "start": 193,
                          "end": 205
                        },
                        "start": 185,
                        "end": 205
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "thing",
                          "start": 207,
                          "end": 212
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 215,
                            "end": 224
                          },
                          "property": {
                            "type": "Literal",
                            "value": 2,
                            "raw": "2",
                            "start": 225,
                            "end": 226
                          },
                          "optional": false,
                          "computed": true,
                          "start": 215,
                          "end": 227
                        },
                        "start": 207,
                        "end": 227
                      }
                    ],
                    "start": 157,
                    "end": 228
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "AssignmentExpression",
                      "operator": "+=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 240,
                          "end": 245
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 246,
                          "end": 251
                        },
                        "optional": false,
                        "computed": false,
                        "start": 240,
                        "end": 251
                      },
                      "right": {
                        "type": "BinaryExpression",
                        "left": {
                          "type": "BinaryExpression",
                          "left": {
                            "type": "BinaryExpression",
                            "left": {
                              "type": "BinaryExpression",
                              "left": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "btn",
                                  "start": 255,
                                  "end": 258
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "offset",
                                  "start": 259,
                                  "end": 265
                                },
                                "optional": false,
                                "computed": false,
                                "start": 255,
                                "end": 265
                              },
                              "operator": "+",
                              "right": {
                                "type": "Identifier",
                                "name": "thing",
                                "start": 268,
                                "end": 273
                              },
                              "start": 255,
                              "end": 273
                            },
                            "operator": "+",
                            "right": {
                              "type": "Identifier",
                              "name": "STEP",
                              "start": 276,
                              "end": 280
                            },
                            "start": 255,
                            "end": 280
                          },
                          "operator": "+",
                          "right": {
                            "type": "Literal",
                            "value": 2,
                            "raw": "2",
                            "start": 283,
                            "end": 284
                          },
                          "start": 255,
                          "end": 284
                        },
                        "operator": "+",
                        "right": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "props",
                            "start": 287,
                            "end": 292
                          },
                          "property": {
                            "type": "Identifier",
                            "name": "step",
                            "start": 293,
                            "end": 297
                          },
                          "optional": false,
                          "computed": false,
                          "start": 287,
                          "end": 297
                        },
                        "start": 255,
                        "end": 297
                      },
                      "start": 240,
                      "end": 297
                    },
                    "start": 233,
                    "end": 298
                  }
                ],
                "start": 151,
                "end": 300
              },
              "id": null,
              "generator": false,
              "start": 137,
              "end": 300
            },
            "start": 88,
            "end": 300
          }
        ],
        "start": 82,
        "end": 301
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 75,
      "end": 301
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 301
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_div_button_q_e_click_UB6Fs5a3bd8",
  "entry": null,
  "displayName": "test.tsx_App_component_div_button_q_e_click",
  "hash": "UB6Fs5a3bd8",
  "canonicalFilename": "test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    451,
    519
  ],
  "paramNames": [
    "_",
    "_",
    "btn"
  ],
  "captureNames": [
    "props",
    "state",
    "thing"
  ]
}
```

### Module: test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_q_e_click_mi4E1piTWe8 = ()=>{
    const count2 = _captures[0], state = _captures[1];
    return state.count += count2;
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
      "type": "ExportNamedDeclaration",
      "declaration": {
        "type": "VariableDeclaration",
        "kind": "const",
        "declarations": [
          {
            "type": "VariableDeclarator",
            "id": {
              "type": "Identifier",
              "name": "App_component_div_q_e_click_mi4E1piTWe8",
              "start": 57,
              "end": 96
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
                          "name": "count2",
                          "start": 115,
                          "end": 121
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 124,
                            "end": 133
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 134,
                            "end": 135
                          },
                          "optional": false,
                          "computed": true,
                          "start": 124,
                          "end": 136
                        },
                        "start": 115,
                        "end": 136
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 138,
                          "end": 143
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 146,
                            "end": 155
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 156,
                            "end": 157
                          },
                          "optional": false,
                          "computed": true,
                          "start": 146,
                          "end": 158
                        },
                        "start": 138,
                        "end": 158
                      }
                    ],
                    "start": 109,
                    "end": 159
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "AssignmentExpression",
                      "operator": "+=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 171,
                          "end": 176
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 177,
                          "end": 182
                        },
                        "optional": false,
                        "computed": false,
                        "start": 171,
                        "end": 182
                      },
                      "right": {
                        "type": "Identifier",
                        "name": "count2",
                        "start": 186,
                        "end": 192
                      },
                      "start": 171,
                      "end": 192
                    },
                    "start": 164,
                    "end": 193
                  }
                ],
                "start": 103,
                "end": 195
              },
              "id": null,
              "generator": false,
              "start": 99,
              "end": 195
            },
            "start": 57,
            "end": 195
          }
        ],
        "start": 51,
        "end": 196
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 196
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 196
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_div_q_e_click_mi4E1piTWe8",
  "entry": null,
  "displayName": "test.tsx_App_component_div_q_e_click",
  "hash": "mi4E1piTWe8",
  "canonicalFilename": "test.tsx_App_component_div_q_e_click_mi4E1piTWe8",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    340,
    365
  ],
  "captureNames": [
    "count2",
    "state"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (7 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.js | @qwik.dev/core | 1 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `useStore` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |
| `_jsxSorted` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 3 |
| `_wrapProp` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 2 |
| `useStore` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8.js | @qwik.dev/core | 3 |
| `_captures[]` | test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js | @qwik.dev/core | 2 |

## Diagnostics

```json
[]
```
