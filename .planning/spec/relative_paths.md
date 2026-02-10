# Test: relative_paths

## Test Configuration

**Note:** This test uses `transform_modules` directly (not `test_input!` macro) with two input modules and custom configuration. This is the only snapshot that has NO `==INPUT==` section -- input code is specified directly in `test.rs`.

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |
| Explicit Extensions | true |
| Minify | Simplify (default) |
| Source Maps | true |
| Src Dir | /path/to/app/src/thing |
| Root Dir | /path/to/app/ |

### Input Modules

Two input modules are processed together:

1. `../../node_modules/dep/dist/lib.mjs` -- a pre-transformed dependency module
2. `components/main.tsx` -- the main source component

## Input

### Source Code: lib.mjs (dependency module)

```javascript
import { componentQrl, inlinedQrl, useStore, useLexicalScope } from "@qwik.dev/core";
import { jsx, jsxs } from "@qwik.dev/core/jsx-runtime";
import { state } from './sibling';

const useData = () => {
	return useStore({
		count: 0
	});
}

export const App = /*#__PURE__*/ componentQrl(inlinedQrl(()=>{
	const store = useData();
	return /*#__PURE__*/ jsxs("div", {
		children: [
			/*#__PURE__*/ jsxs("p", {
				children: [
					"Count: ",
					store.count
				]
			}),
			/*#__PURE__*/ jsx("p", {
				children: /*#__PURE__*/ jsx("button", {
					onClick$: inlinedQrl(()=>{
						const [store] = useLexicalScope();
						return store.count++;
					}, "App_component_div_p_button_onClick_8dWUa0cJAr4", [
						store
					]),
					children: "Click"
				})
			})
		]
	});
}, "App_component_AkbU84a8zes"));
```

<details>
<summary>Input AST: lib.mjs (OXC)</summary>

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
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 23,
            "end": 33
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 23,
            "end": 33
          },
          "start": 23,
          "end": 33
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 35,
            "end": 43
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 35,
            "end": 43
          },
          "start": 35,
          "end": 43
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useLexicalScope",
            "start": 45,
            "end": 60
          },
          "local": {
            "type": "Identifier",
            "name": "useLexicalScope",
            "start": 45,
            "end": 60
          },
          "start": 45,
          "end": 60
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 68,
        "end": 84
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 85
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "jsx",
            "start": 95,
            "end": 98
          },
          "local": {
            "type": "Identifier",
            "name": "jsx",
            "start": 95,
            "end": 98
          },
          "start": 95,
          "end": 98
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "jsxs",
            "start": 100,
            "end": 104
          },
          "local": {
            "type": "Identifier",
            "name": "jsxs",
            "start": 100,
            "end": 104
          },
          "start": 100,
          "end": 104
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core/jsx-runtime",
        "raw": "\"@qwik.dev/core/jsx-runtime\"",
        "start": 112,
        "end": 140
      },
      "phase": null,
      "attributes": [],
      "start": 86,
      "end": 141
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "state",
            "start": 151,
            "end": 156
          },
          "local": {
            "type": "Identifier",
            "name": "state",
            "start": 151,
            "end": 156
          },
          "start": 151,
          "end": 156
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./sibling",
        "raw": "'./sibling'",
        "start": 164,
        "end": 175
      },
      "phase": null,
      "attributes": [],
      "start": 142,
      "end": 176
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "useData",
            "start": 183,
            "end": 190
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
                      "start": 208,
                      "end": 216
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
                              "start": 219,
                              "end": 224
                            },
                            "value": {
                              "type": "Literal",
                              "value": 0,
                              "raw": "0",
                              "start": 226,
                              "end": 227
                            },
                            "method": false,
                            "shorthand": false,
                            "computed": false,
                            "start": 219,
                            "end": 227
                          }
                        ],
                        "start": 217,
                        "end": 229
                      }
                    ],
                    "optional": false,
                    "start": 208,
                    "end": 230
                  },
                  "start": 201,
                  "end": 231
                }
              ],
              "start": 199,
              "end": 233
            },
            "id": null,
            "generator": false,
            "start": 193,
            "end": 233
          },
          "start": 183,
          "end": 233
        }
      ],
      "start": 177,
      "end": 233
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
              "start": 247,
              "end": 250
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 267,
                "end": 279
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 280,
                    "end": 290
                  },
                  "arguments": [
                    {
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
                                  "name": "store",
                                  "start": 305,
                                  "end": 310
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useData",
                                    "start": 313,
                                    "end": 320
                                  },
                                  "arguments": [],
                                  "optional": false,
                                  "start": 313,
                                  "end": 322
                                },
                                "start": 305,
                                "end": 322
                              }
                            ],
                            "start": 299,
                            "end": 323
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "jsxs",
                                "start": 347,
                                "end": 351
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 352,
                                  "end": 357
                                },
                                {
                                  "type": "ObjectExpression",
                                  "properties": [
                                    {
                                      "type": "Property",
                                      "kind": "init",
                                      "key": {
                                        "type": "Identifier",
                                        "name": "children",
                                        "start": 361,
                                        "end": 369
                                      },
                                      "value": {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "jsxs",
                                              "start": 387,
                                              "end": 391
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "p",
                                                "raw": "\"p\"",
                                                "start": 392,
                                                "end": 395
                                              },
                                              {
                                                "type": "ObjectExpression",
                                                "properties": [
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Identifier",
                                                      "name": "children",
                                                      "start": 399,
                                                      "end": 407
                                                    },
                                                    "value": {
                                                      "type": "ArrayExpression",
                                                      "elements": [
                                                        {
                                                          "type": "Literal",
                                                          "value": "Count: ",
                                                          "raw": "\"Count: \"",
                                                          "start": 411,
                                                          "end": 420
                                                        },
                                                        {
                                                          "type": "MemberExpression",
                                                          "object": {
                                                            "type": "Identifier",
                                                            "name": "store",
                                                            "start": 422,
                                                            "end": 427
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "name": "count",
                                                            "start": 428,
                                                            "end": 433
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 422,
                                                          "end": 433
                                                        }
                                                      ],
                                                      "start": 409,
                                                      "end": 435
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 399,
                                                    "end": 435
                                                  }
                                                ],
                                                "start": 397,
                                                "end": 437
                                              }
                                            ],
                                            "optional": false,
                                            "start": 387,
                                            "end": 438
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "jsx",
                                              "start": 454,
                                              "end": 457
                                            },
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "p",
                                                "raw": "\"p\"",
                                                "start": 458,
                                                "end": 461
                                              },
                                              {
                                                "type": "ObjectExpression",
                                                "properties": [
                                                  {
                                                    "type": "Property",
                                                    "kind": "init",
                                                    "key": {
                                                      "type": "Identifier",
                                                      "name": "children",
                                                      "start": 465,
                                                      "end": 473
                                                    },
                                                    "value": {
                                                      "type": "CallExpression",
                                                      "callee": {
                                                        "type": "Identifier",
                                                        "name": "jsx",
                                                        "start": 489,
                                                        "end": 492
                                                      },
                                                      "arguments": [
                                                        {
                                                          "type": "Literal",
                                                          "value": "button",
                                                          "raw": "\"button\"",
                                                          "start": 493,
                                                          "end": 501
                                                        },
                                                        {
                                                          "type": "ObjectExpression",
                                                          "properties": [
                                                            {
                                                              "type": "Property",
                                                              "kind": "init",
                                                              "key": {
                                                                "type": "Identifier",
                                                                "name": "onClick$",
                                                                "start": 505,
                                                                "end": 513
                                                              },
                                                              "value": {
                                                                "type": "CallExpression",
                                                                "callee": {
                                                                  "type": "Identifier",
                                                                  "name": "inlinedQrl",
                                                                  "start": 515,
                                                                  "end": 525
                                                                },
                                                                "arguments": [
                                                                  {
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
                                                                                "type": "ArrayPattern",
                                                                                "elements": [
                                                                                  {
                                                                                    "type": "Identifier",
                                                                                    "name": "store",
                                                                                    "start": 539,
                                                                                    "end": 544
                                                                                  }
                                                                                ],
                                                                                "start": 538,
                                                                                "end": 545
                                                                              },
                                                                              "init": {
                                                                                "type": "CallExpression",
                                                                                "callee": {
                                                                                  "type": "Identifier",
                                                                                  "name": "useLexicalScope",
                                                                                  "start": 548,
                                                                                  "end": 563
                                                                                },
                                                                                "arguments": [],
                                                                                "optional": false,
                                                                                "start": 548,
                                                                                "end": 565
                                                                              },
                                                                              "start": 538,
                                                                              "end": 565
                                                                            }
                                                                          ],
                                                                          "start": 532,
                                                                          "end": 566
                                                                        },
                                                                        {
                                                                          "type": "ReturnStatement",
                                                                          "argument": {
                                                                            "type": "UpdateExpression",
                                                                            "operator": "++",
                                                                            "prefix": false,
                                                                            "argument": {
                                                                              "type": "MemberExpression",
                                                                              "object": {
                                                                                "type": "Identifier",
                                                                                "name": "store",
                                                                                "start": 574,
                                                                                "end": 579
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "name": "count",
                                                                                "start": 580,
                                                                                "end": 585
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 574,
                                                                              "end": 585
                                                                            },
                                                                            "start": 574,
                                                                            "end": 587
                                                                          },
                                                                          "start": 567,
                                                                          "end": 588
                                                                        }
                                                                      ],
                                                                      "start": 530,
                                                                      "end": 590
                                                                    },
                                                                    "id": null,
                                                                    "generator": false,
                                                                    "start": 526,
                                                                    "end": 590
                                                                  },
                                                                  {
                                                                    "type": "Literal",
                                                                    "value": "App_component_div_p_button_onClick_8dWUa0cJAr4",
                                                                    "raw": "\"App_component_div_p_button_onClick_8dWUa0cJAr4\"",
                                                                    "start": 592,
                                                                    "end": 640
                                                                  },
                                                                  {
                                                                    "type": "ArrayExpression",
                                                                    "elements": [
                                                                      {
                                                                        "type": "Identifier",
                                                                        "name": "store",
                                                                        "start": 644,
                                                                        "end": 649
                                                                      }
                                                                    ],
                                                                    "start": 642,
                                                                    "end": 651
                                                                  }
                                                                ],
                                                                "optional": false,
                                                                "start": 515,
                                                                "end": 652
                                                              },
                                                              "method": false,
                                                              "shorthand": false,
                                                              "computed": false,
                                                              "start": 505,
                                                              "end": 652
                                                            },
                                                            {
                                                              "type": "Property",
                                                              "kind": "init",
                                                              "key": {
                                                                "type": "Identifier",
                                                                "name": "children",
                                                                "start": 654,
                                                                "end": 662
                                                              },
                                                              "value": {
                                                                "type": "Literal",
                                                                "value": "Click",
                                                                "raw": "\"Click\"",
                                                                "start": 664,
                                                                "end": 671
                                                              },
                                                              "method": false,
                                                              "shorthand": false,
                                                              "computed": false,
                                                              "start": 654,
                                                              "end": 671
                                                            }
                                                          ],
                                                          "start": 503,
                                                          "end": 673
                                                        }
                                                      ],
                                                      "optional": false,
                                                      "start": 489,
                                                      "end": 674
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "start": 465,
                                                    "end": 674
                                                  }
                                                ],
                                                "start": 463,
                                                "end": 676
                                              }
                                            ],
                                            "optional": false,
                                            "start": 454,
                                            "end": 677
                                          }
                                        ],
                                        "start": 371,
                                        "end": 679
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 361,
                                      "end": 679
                                    }
                                  ],
                                  "start": 359,
                                  "end": 681
                                }
                              ],
                              "optional": false,
                              "start": 347,
                              "end": 682
                            },
                            "start": 326,
                            "end": 683
                          }
                        ],
                        "start": 295,
                        "end": 685
                      },
                      "id": null,
                      "generator": false,
                      "start": 291,
                      "end": 685
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_AkbU84a8zes",
                      "raw": "\"App_component_AkbU84a8zes\"",
                      "start": 687,
                      "end": 714
                    }
                  ],
                  "optional": false,
                  "start": 280,
                  "end": 715
                }
              ],
              "optional": false,
              "start": 267,
              "end": 716
            },
            "start": 247,
            "end": 716
          }
        ],
        "start": 241,
        "end": 717
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 234,
      "end": 717
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 718
}
```

</details>

### Source Code: components/main.tsx (main source)

```tsx
import { component$, $ } from '@qwik.dev/core';
import { state } from './sibling';

export const Local = component$(() => {
	return (
		<div>{state}</div>
	)
});
```

<details>
<summary>Input AST: components/main.tsx (OXC)</summary>

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
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 22
          },
          "importKind": "value",
          "start": 21,
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "state",
            "optional": false,
            "typeAnnotation": null,
            "start": 57,
            "end": 62
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "state",
            "optional": false,
            "typeAnnotation": null,
            "start": 57,
            "end": 62
          },
          "importKind": "value",
          "start": 57,
          "end": 62
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./sibling",
        "raw": "'./sibling'",
        "start": 70,
        "end": 81
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 48,
      "end": 82
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
              "name": "Local",
              "optional": false,
              "typeAnnotation": null,
              "start": 96,
              "end": 101
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 104,
                "end": 114
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
                                "type": "JSXExpressionContainer",
                                "expression": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "state",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 140,
                                  "end": 145
                                },
                                "start": 139,
                                "end": 146
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 148,
                                "end": 151
                              },
                              "start": 146,
                              "end": 152
                            },
                            "start": 134,
                            "end": 152
                          },
                          "start": 132,
                          "end": 154
                        },
                        "start": 125,
                        "end": 154
                      }
                    ],
                    "start": 121,
                    "end": 156
                  },
                  "id": null,
                  "generator": false,
                  "start": 115,
                  "end": 156
                }
              ],
              "optional": false,
              "start": 104,
              "end": 157
            },
            "definite": false,
            "start": 96,
            "end": 157
          }
        ],
        "declare": false,
        "start": 90,
        "end": 158
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 83,
      "end": 158
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 159
}
```

</details>

## Output

### Module: ../../node_modules/dep/dist/lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4.js (ENTRY POINT)

```javascript
import { useLexicalScope } from "@qwik.dev/core";
export const App_component_div_p_button_onClick_8dWUa0cJAr4 = ()=>{
    const [store] = useLexicalScope();
    return store.count++;
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
            "name": "useLexicalScope",
            "start": 9,
            "end": 24
          },
          "local": {
            "type": "Identifier",
            "name": "useLexicalScope",
            "start": 9,
            "end": 24
          },
          "start": 9,
          "end": 24
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 32,
        "end": 48
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 49
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
              "name": "App_component_div_p_button_onClick_8dWUa0cJAr4",
              "start": 63,
              "end": 109
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
                          "type": "ArrayPattern",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "store",
                              "start": 129,
                              "end": 134
                            }
                          ],
                          "start": 128,
                          "end": 135
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useLexicalScope",
                            "start": 138,
                            "end": 153
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 138,
                          "end": 155
                        },
                        "start": 128,
                        "end": 155
                      }
                    ],
                    "start": 122,
                    "end": 156
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "UpdateExpression",
                      "operator": "++",
                      "prefix": false,
                      "argument": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "store",
                          "start": 168,
                          "end": 173
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 174,
                          "end": 179
                        },
                        "optional": false,
                        "computed": false,
                        "start": 168,
                        "end": 179
                      },
                      "start": 168,
                      "end": 181
                    },
                    "start": 161,
                    "end": 182
                  }
                ],
                "start": 116,
                "end": 184
              },
              "id": null,
              "generator": false,
              "start": 112,
              "end": 184
            },
            "start": 63,
            "end": 184
          }
        ],
        "start": 57,
        "end": 185
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 50,
      "end": 185
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 186
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "../../node_modules/dep/dist/lib.mjs",
  "name": "App_component_div_p_button_onClick_8dWUa0cJAr4",
  "entry": null,
  "displayName": "lib.mjs_App_component_div_p_button_onClick",
  "hash": "8dWUa0cJAr4",
  "canonicalFilename": "lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4",
  "path": "../../node_modules/dep/dist",
  "extension": "js",
  "parent": "App_component_AkbU84a8zes",
  "ctxKind": "function",
  "ctxName": "q-e:click",
  "captures": true,
  "loc": [570, 651],
  "captureNames": ["store"]
}
```

### Module: ../../node_modules/dep/dist/lib.mjs_App_component_AkbU84a8zes.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { _auto_useData as useData } from "./lib.mjs";
const i_8dWUa0cJAr4 = ()=>import("./lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4.js");
export const App_component_AkbU84a8zes = ()=>{
    const store = useData();
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("p", null, null, [
            "Count: ",
            _wrapProp(store, "count")
        ], 3, null),
        /*#__PURE__*/ _jsxSorted("p", null, null, /*#__PURE__*/ _jsxSorted("button", {
            "q-e:click": /*#__PURE__*/ qrl(i_8dWUa0cJAr4, "App_component_div_p_button_onClick_8dWUa0cJAr4", [
                store
            ])
        }, null, "Click", 2, null), 1, null)
    ], 1, "70_0");
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
            "name": "_auto_useData",
            "start": 136,
            "end": 149
          },
          "local": {
            "type": "Identifier",
            "name": "useData",
            "start": 153,
            "end": 160
          },
          "start": 136,
          "end": 160
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./lib.mjs",
        "raw": "\"./lib.mjs\"",
        "start": 168,
        "end": 179
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 180
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_8dWUa0cJAr4",
            "start": 187,
            "end": 200
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
                "value": "./lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4.js",
                "raw": "\"./lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4.js\"",
                "start": 214,
                "end": 275
              },
              "options": null,
              "phase": null,
              "start": 207,
              "end": 276
            },
            "id": null,
            "generator": false,
            "start": 203,
            "end": 276
          },
          "start": 187,
          "end": 276
        }
      ],
      "start": 181,
      "end": 277
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
              "name": "App_component_AkbU84a8zes",
              "start": 291,
              "end": 316
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
                          "name": "store",
                          "start": 335,
                          "end": 340
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useData",
                            "start": 343,
                            "end": 350
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 343,
                          "end": 352
                        },
                        "start": 335,
                        "end": 352
                      }
                    ],
                    "start": 329,
                    "end": 353
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 379,
                        "end": 389
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 390,
                          "end": 395
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 397,
                          "end": 401
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 403,
                          "end": 407
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 433,
                                "end": 443
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "p",
                                  "raw": "\"p\"",
                                  "start": 444,
                                  "end": 447
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 449,
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
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "Literal",
                                      "value": "Count: ",
                                      "raw": "\"Count: \"",
                                      "start": 463,
                                      "end": 472
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_wrapProp",
                                        "start": 474,
                                        "end": 483
                                      },
                                      "arguments": [
                                        {
                                          "type": "Identifier",
                                          "name": "store",
                                          "start": 484,
                                          "end": 489
                                        },
                                        {
                                          "type": "Literal",
                                          "value": "count",
                                          "raw": "\"count\"",
                                          "start": 491,
                                          "end": 498
                                        }
                                      ],
                                      "optional": false,
                                      "start": 474,
                                      "end": 499
                                    }
                                  ],
                                  "start": 461,
                                  "end": 501
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 503,
                                  "end": 504
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 506,
                                  "end": 510
                                }
                              ],
                              "optional": false,
                              "start": 433,
                              "end": 511
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 535,
                                "end": 545
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "p",
                                  "raw": "\"p\"",
                                  "start": 546,
                                  "end": 549
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 551,
                                  "end": 555
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 557,
                                  "end": 561
                                },
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
                                      "value": "button",
                                      "raw": "\"button\"",
                                      "start": 588,
                                      "end": 596
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
                                            "start": 612,
                                            "end": 623
                                          },
                                          "value": {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "name": "qrl",
                                              "start": 639,
                                              "end": 642
                                            },
                                            "arguments": [
                                              {
                                                "type": "Identifier",
                                                "name": "i_8dWUa0cJAr4",
                                                "start": 643,
                                                "end": 656
                                              },
                                              {
                                                "type": "Literal",
                                                "value": "App_component_div_p_button_onClick_8dWUa0cJAr4",
                                                "raw": "\"App_component_div_p_button_onClick_8dWUa0cJAr4\"",
                                                "start": 658,
                                                "end": 706
                                              },
                                              {
                                                "type": "ArrayExpression",
                                                "elements": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "store",
                                                    "start": 710,
                                                    "end": 715
                                                  }
                                                ],
                                                "start": 708,
                                                "end": 717
                                              }
                                            ],
                                            "optional": false,
                                            "start": 639,
                                            "end": 718
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 612,
                                          "end": 718
                                        }
                                      ],
                                      "start": 598,
                                      "end": 728
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 730,
                                      "end": 734
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "Click",
                                      "raw": "\"Click\"",
                                      "start": 736,
                                      "end": 743
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 2,
                                      "raw": "2",
                                      "start": 745,
                                      "end": 746
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 748,
                                      "end": 752
                                    }
                                  ],
                                  "optional": false,
                                  "start": 577,
                                  "end": 753
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 755,
                                  "end": 756
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 758,
                                  "end": 762
                                }
                              ],
                              "optional": false,
                              "start": 535,
                              "end": 763
                            }
                          ],
                          "start": 409,
                          "end": 769
                        },
                        {
                          "type": "Literal",
                          "value": 1,
                          "raw": "1",
                          "start": 771,
                          "end": 772
                        },
                        {
                          "type": "Literal",
                          "value": "70_0",
                          "raw": "\"70_0\"",
                          "start": 774,
                          "end": 780
                        }
                      ],
                      "optional": false,
                      "start": 379,
                      "end": 781
                    },
                    "start": 358,
                    "end": 782
                  }
                ],
                "start": 323,
                "end": 784
              },
              "id": null,
              "generator": false,
              "start": 319,
              "end": 784
            },
            "start": 291,
            "end": 784
          }
        ],
        "start": 285,
        "end": 785
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 278,
      "end": 785
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 786
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "../../node_modules/dep/dist/lib.mjs",
  "name": "App_component_AkbU84a8zes",
  "entry": null,
  "displayName": "lib.mjs_App_component",
  "hash": "AkbU84a8zes",
  "canonicalFilename": "lib.mjs_App_component_AkbU84a8zes",
  "path": "../../node_modules/dep/dist",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [299, 772]
}
```

### Module: ../../node_modules/dep/dist/lib.mjs

```javascript
import { qrl } from "@qwik.dev/core";
const i_AkbU84a8zes = ()=>import("./lib.mjs_App_component_AkbU84a8zes.js");
import { componentQrl, useStore } from "@qwik.dev/core";
const useData = ()=>{
    return useStore({
        count: 0
    });
};
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_AkbU84a8zes, "App_component_AkbU84a8zes"));
export { useData as _auto_useData };
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
            "name": "i_AkbU84a8zes",
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
                "value": "./lib.mjs_App_component_AkbU84a8zes.js",
                "raw": "\"./lib.mjs_App_component_AkbU84a8zes.js\"",
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 123,
            "end": 135
          },
          "local": {
            "type": "Identifier",
            "name": "componentQrl",
            "start": 123,
            "end": 135
          },
          "start": 123,
          "end": 135
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 137,
            "end": 145
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 137,
            "end": 145
          },
          "start": 137,
          "end": 145
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 153,
        "end": 169
      },
      "phase": null,
      "attributes": [],
      "start": 114,
      "end": 170
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "useData",
            "start": 177,
            "end": 184
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
                      "start": 200,
                      "end": 208
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
                              "start": 211,
                              "end": 216
                            },
                            "value": {
                              "type": "Literal",
                              "value": 0,
                              "raw": "0",
                              "start": 218,
                              "end": 219
                            },
                            "method": false,
                            "shorthand": false,
                            "computed": false,
                            "start": 211,
                            "end": 219
                          }
                        ],
                        "start": 209,
                        "end": 221
                      }
                    ],
                    "optional": false,
                    "start": 200,
                    "end": 222
                  },
                  "start": 193,
                  "end": 223
                }
              ],
              "start": 191,
              "end": 225
            },
            "id": null,
            "generator": false,
            "start": 187,
            "end": 225
          },
          "start": 177,
          "end": 225
        }
      ],
      "start": 171,
      "end": 226
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
              "start": 240,
              "end": 243
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 260,
                "end": 272
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 287,
                    "end": 290
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_AkbU84a8zes",
                      "start": 291,
                      "end": 304
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_AkbU84a8zes",
                      "raw": "\"App_component_AkbU84a8zes\"",
                      "start": 306,
                      "end": 333
                    }
                  ],
                  "optional": false,
                  "start": 287,
                  "end": 334
                }
              ],
              "optional": false,
              "start": 260,
              "end": 335
            },
            "start": 240,
            "end": 335
          }
        ],
        "start": 234,
        "end": 336
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 227,
      "end": 336
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "useData",
            "start": 346,
            "end": 353
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_useData",
            "start": 357,
            "end": 370
          },
          "start": 346,
          "end": 370
        }
      ],
      "source": null,
      "attributes": [],
      "start": 337,
      "end": 373
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 374
}
```

</details>

### Module: components/main.tsx_Local_component_jJ0v28bs0p8.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { state } from "./sibling";
export const Local_component_jJ0v28bs0p8 = ()=>{
    return /*#__PURE__*/ _jsxSorted("div", null, null, state, 3, "zp_0");
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
            "name": "state",
            "start": 54,
            "end": 59
          },
          "local": {
            "type": "Identifier",
            "name": "state",
            "start": 54,
            "end": 59
          },
          "start": 54,
          "end": 59
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./sibling",
        "raw": "\"./sibling\"",
        "start": 67,
        "end": 78
      },
      "phase": null,
      "attributes": [],
      "start": 45,
      "end": 79
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
              "name": "Local_component_jJ0v28bs0p8",
              "start": 93,
              "end": 120
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
                        "name": "_jsxSorted",
                        "start": 154,
                        "end": 164
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 165,
                          "end": 170
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 172,
                          "end": 176
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 178,
                          "end": 182
                        },
                        {
                          "type": "Identifier",
                          "name": "state",
                          "start": 184,
                          "end": 189
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 191,
                          "end": 192
                        },
                        {
                          "type": "Literal",
                          "value": "zp_0",
                          "raw": "\"zp_0\"",
                          "start": 194,
                          "end": 200
                        }
                      ],
                      "optional": false,
                      "start": 154,
                      "end": 201
                    },
                    "start": 133,
                    "end": 202
                  }
                ],
                "start": 127,
                "end": 204
              },
              "id": null,
              "generator": false,
              "start": 123,
              "end": 204
            },
            "start": 93,
            "end": 204
          }
        ],
        "start": 87,
        "end": 205
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 80,
      "end": 205
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 206
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "components/main.tsx",
  "name": "Local_component_jJ0v28bs0p8",
  "entry": null,
  "displayName": "main.tsx_Local_component",
  "hash": "jJ0v28bs0p8",
  "canonicalFilename": "main.tsx_Local_component_jJ0v28bs0p8",
  "path": "components",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [118, 161]
}
```

### Module: components/main.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_jJ0v28bs0p8 = ()=>import("./main.tsx_Local_component_jJ0v28bs0p8.js");
export const Local = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_jJ0v28bs0p8, "Local_component_jJ0v28bs0p8"));
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
            "name": "i_jJ0v28bs0p8",
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
                "value": "./main.tsx_Local_component_jJ0v28bs0p8.js",
                "raw": "\"./main.tsx_Local_component_jJ0v28bs0p8.js\"",
                "start": 118,
                "end": 161
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 162
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 162
          },
          "start": 91,
          "end": 162
        }
      ],
      "start": 85,
      "end": 163
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
              "name": "Local",
              "start": 177,
              "end": 182
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 199,
                "end": 211
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 226,
                    "end": 229
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_jJ0v28bs0p8",
                      "start": 230,
                      "end": 243
                    },
                    {
                      "type": "Literal",
                      "value": "Local_component_jJ0v28bs0p8",
                      "raw": "\"Local_component_jJ0v28bs0p8\"",
                      "start": 245,
                      "end": 274
                    }
                  ],
                  "optional": false,
                  "start": 226,
                  "end": 275
                }
              ],
              "optional": false,
              "start": 199,
              "end": 276
            },
            "start": 177,
            "end": 276
          }
        ],
        "start": 171,
        "end": 277
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 164,
      "end": 277
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 278
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: `qrl()` calls wrap lazy imports with segment name identifiers in both dependency and main module outputs
- **[CONV-02] Dollar-to-Qrl Conversion**: `component$` converted to `componentQrl(qrl(...))` in both lib.mjs and components/main.js outputs
- **[CONV-03] JSX Transforms**: JSX transformed to `_jsxSorted()` calls in entry point segments (e.g., `_jsxSorted("div", ...)`)
- **[CONV-04] Signal Helpers**: `_wrapProp(store, "count")` used for reactive store property access in entry point module
- **[CONV-05] Capture Patterns**: `useLexicalScope()` used in onClick handler segment to capture `store` variable. Capture array passed as 3rd arg to `qrl()`
- **[CONV-06] Lazy Imports**: Lazy import constants generated: `const i_HASH = ()=>import("./...")` for each extracted segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations on `componentQrl()`, `qrl()`, and `_jsxSorted()` calls throughout
- **[CONV-08] Segment Extraction**: Component body and event handler extracted to separate entry point modules (4 entry point segments total)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| useLexicalScope | lib.mjs_App_..._onClick_8dWUa0cJAr4.js | @qwik.dev/core | 1 |
| _jsxSorted | lib.mjs_App_component_AkbU84a8zes.js | @qwik.dev/core | 4 |
| _wrapProp | lib.mjs_App_component_AkbU84a8zes.js | @qwik.dev/core | 1 |
| qrl | lib.mjs_App_component_AkbU84a8zes.js | @qwik.dev/core | 1 |
| componentQrl | lib.mjs (main) | @qwik.dev/core | 1 |
| qrl | lib.mjs (main) | @qwik.dev/core | 1 |
| useStore | lib.mjs (main) | @qwik.dev/core | 1 |
| _jsxSorted | main.tsx_Local_component_jJ0v28bs0p8.js | @qwik.dev/core | 1 |
| componentQrl | components/main.js | @qwik.dev/core | 1 |
| qrl | components/main.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
