# Test: example_parsed_inlined_qrls

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Prod |
| Transpile TS | False |

## Input

### Source Code

```tsx
import { componentQrl, inlinedQrl, useStore, jsxs, jsx, useLexicalScope } from '@qwik.dev/core';

export const App = /*#__PURE__*/ componentQrl(inlinedQrl(()=>{
	useStyles$(inlinedQrl(STYLES, "STYLES_odz7dfdfdM"));
	useStyles$(inlinedQrl(STYLES, "STYLES_odzdfdfdM"));

	const store = useStore({
		count: 0
	});
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
					}, "App_component_div_p_button_onClick_odz7eidI4GM", [
						store
					]),
					children: "Click"
				})
			})
		]
	});
}, "App_component_Fh88JClhbC0"));

export const STYLES = ".red { color: red; }";
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
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 23,
            "end": 33
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 23,
            "end": 33
          },
          "importKind": "value",
          "start": 23,
          "end": 33
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 35,
            "end": 43
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 35,
            "end": 43
          },
          "importKind": "value",
          "start": 35,
          "end": 43
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsxs",
            "optional": false,
            "typeAnnotation": null,
            "start": 45,
            "end": 49
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsxs",
            "optional": false,
            "typeAnnotation": null,
            "start": 45,
            "end": 49
          },
          "importKind": "value",
          "start": 45,
          "end": 49
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 51,
            "end": 54
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "jsx",
            "optional": false,
            "typeAnnotation": null,
            "start": 51,
            "end": 54
          },
          "importKind": "value",
          "start": 51,
          "end": 54
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLexicalScope",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 71
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLexicalScope",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 71
          },
          "importKind": "value",
          "start": 56,
          "end": 71
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 79,
        "end": 95
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 96
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
              "start": 111,
              "end": 114
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 131,
                "end": 143
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "inlinedQrl",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 144,
                    "end": 154
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
                                "name": "useStyles$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 162,
                                "end": 172
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "inlinedQrl",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 173,
                                    "end": 183
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "STYLES",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 184,
                                      "end": 190
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "STYLES_odz7dfdfdM",
                                      "raw": "\"STYLES_odz7dfdfdM\"",
                                      "start": 192,
                                      "end": 211
                                    }
                                  ],
                                  "optional": false,
                                  "start": 173,
                                  "end": 212
                                }
                              ],
                              "optional": false,
                              "start": 162,
                              "end": 213
                            },
                            "directive": null,
                            "start": 162,
                            "end": 214
                          },
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStyles$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 216,
                                "end": 226
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "inlinedQrl",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 227,
                                    "end": 237
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "STYLES",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 238,
                                      "end": 244
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "STYLES_odzdfdfdM",
                                      "raw": "\"STYLES_odzdfdfdM\"",
                                      "start": 246,
                                      "end": 264
                                    }
                                  ],
                                  "optional": false,
                                  "start": 227,
                                  "end": 265
                                }
                              ],
                              "optional": false,
                              "start": 216,
                              "end": 266
                            },
                            "directive": null,
                            "start": 216,
                            "end": 267
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
                                  "name": "store",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 276,
                                  "end": 281
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "useStore",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 284,
                                    "end": 292
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
                                            "start": 297,
                                            "end": 302
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": 0,
                                            "raw": "0",
                                            "start": 304,
                                            "end": 305
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "optional": false,
                                          "start": 297,
                                          "end": 305
                                        }
                                      ],
                                      "start": 293,
                                      "end": 308
                                    }
                                  ],
                                  "optional": false,
                                  "start": 284,
                                  "end": 309
                                },
                                "definite": false,
                                "start": 276,
                                "end": 309
                              }
                            ],
                            "declare": false,
                            "start": 270,
                            "end": 310
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "jsxs",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 333,
                                "end": 337
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 338,
                                  "end": 343
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
                                        "name": "children",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 349,
                                        "end": 357
                                      },
                                      "value": {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "jsxs",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 378,
                                              "end": 382
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "p",
                                                "raw": "\"p\"",
                                                "start": 383,
                                                "end": 386
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
                                                      "name": "children",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 394,
                                                      "end": 402
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
                                                            "decorators": [],
                                                            "name": "store",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 427,
                                                            "end": 432
                                                          },
                                                          "property": {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "count",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 433,
                                                            "end": 438
                                                          },
                                                          "optional": false,
                                                          "computed": false,
                                                          "start": 427,
                                                          "end": 438
                                                        }
                                                      ],
                                                      "start": 404,
                                                      "end": 444
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "optional": false,
                                                    "start": 394,
                                                    "end": 444
                                                  }
                                                ],
                                                "start": 388,
                                                "end": 449
                                              }
                                            ],
                                            "optional": false,
                                            "start": 378,
                                            "end": 450
                                          },
                                          {
                                            "type": "CallExpression",
                                            "callee": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "jsx",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 469,
                                              "end": 472
                                            },
                                            "typeArguments": null,
                                            "arguments": [
                                              {
                                                "type": "Literal",
                                                "value": "p",
                                                "raw": "\"p\"",
                                                "start": 473,
                                                "end": 476
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
                                                      "name": "children",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 484,
                                                      "end": 492
                                                    },
                                                    "value": {
                                                      "type": "CallExpression",
                                                      "callee": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "jsx",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 508,
                                                        "end": 511
                                                      },
                                                      "typeArguments": null,
                                                      "arguments": [
                                                        {
                                                          "type": "Literal",
                                                          "value": "button",
                                                          "raw": "\"button\"",
                                                          "start": 512,
                                                          "end": 520
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
                                                                "name": "onClick$",
                                                                "optional": false,
                                                                "typeAnnotation": null,
                                                                "start": 529,
                                                                "end": 537
                                                              },
                                                              "value": {
                                                                "type": "CallExpression",
                                                                "callee": {
                                                                  "type": "Identifier",
                                                                  "decorators": [],
                                                                  "name": "inlinedQrl",
                                                                  "optional": false,
                                                                  "typeAnnotation": null,
                                                                  "start": 539,
                                                                  "end": 549
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
                                                                                "type": "ArrayPattern",
                                                                                "decorators": [],
                                                                                "elements": [
                                                                                  {
                                                                                    "type": "Identifier",
                                                                                    "decorators": [],
                                                                                    "name": "store",
                                                                                    "optional": false,
                                                                                    "typeAnnotation": null,
                                                                                    "start": 569,
                                                                                    "end": 574
                                                                                  }
                                                                                ],
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 568,
                                                                                "end": 575
                                                                              },
                                                                              "init": {
                                                                                "type": "CallExpression",
                                                                                "callee": {
                                                                                  "type": "Identifier",
                                                                                  "decorators": [],
                                                                                  "name": "useLexicalScope",
                                                                                  "optional": false,
                                                                                  "typeAnnotation": null,
                                                                                  "start": 578,
                                                                                  "end": 593
                                                                                },
                                                                                "typeArguments": null,
                                                                                "arguments": [],
                                                                                "optional": false,
                                                                                "start": 578,
                                                                                "end": 595
                                                                              },
                                                                              "definite": false,
                                                                              "start": 568,
                                                                              "end": 595
                                                                            }
                                                                          ],
                                                                          "declare": false,
                                                                          "start": 562,
                                                                          "end": 596
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
                                                                                "decorators": [],
                                                                                "name": "store",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 610,
                                                                                "end": 615
                                                                              },
                                                                              "property": {
                                                                                "type": "Identifier",
                                                                                "decorators": [],
                                                                                "name": "count",
                                                                                "optional": false,
                                                                                "typeAnnotation": null,
                                                                                "start": 616,
                                                                                "end": 621
                                                                              },
                                                                              "optional": false,
                                                                              "computed": false,
                                                                              "start": 610,
                                                                              "end": 621
                                                                            },
                                                                            "start": 610,
                                                                            "end": 623
                                                                          },
                                                                          "start": 603,
                                                                          "end": 624
                                                                        }
                                                                      ],
                                                                      "start": 554,
                                                                      "end": 631
                                                                    },
                                                                    "id": null,
                                                                    "generator": false,
                                                                    "start": 550,
                                                                    "end": 631
                                                                  },
                                                                  {
                                                                    "type": "Literal",
                                                                    "value": "App_component_div_p_button_onClick_odz7eidI4GM",
                                                                    "raw": "\"App_component_div_p_button_onClick_odz7eidI4GM\"",
                                                                    "start": 633,
                                                                    "end": 681
                                                                  },
                                                                  {
                                                                    "type": "ArrayExpression",
                                                                    "elements": [
                                                                      {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "store",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 691,
                                                                        "end": 696
                                                                      }
                                                                    ],
                                                                    "start": 683,
                                                                    "end": 703
                                                                  }
                                                                ],
                                                                "optional": false,
                                                                "start": 539,
                                                                "end": 704
                                                              },
                                                              "method": false,
                                                              "shorthand": false,
                                                              "computed": false,
                                                              "optional": false,
                                                              "start": 529,
                                                              "end": 704
                                                            },
                                                            {
                                                              "type": "Property",
                                                              "kind": "init",
                                                              "key": {
                                                                "type": "Identifier",
                                                                "decorators": [],
                                                                "name": "children",
                                                                "optional": false,
                                                                "typeAnnotation": null,
                                                                "start": 711,
                                                                "end": 719
                                                              },
                                                              "value": {
                                                                "type": "Literal",
                                                                "value": "Click",
                                                                "raw": "\"Click\"",
                                                                "start": 721,
                                                                "end": 728
                                                              },
                                                              "method": false,
                                                              "shorthand": false,
                                                              "computed": false,
                                                              "optional": false,
                                                              "start": 711,
                                                              "end": 728
                                                            }
                                                          ],
                                                          "start": 522,
                                                          "end": 734
                                                        }
                                                      ],
                                                      "optional": false,
                                                      "start": 508,
                                                      "end": 735
                                                    },
                                                    "method": false,
                                                    "shorthand": false,
                                                    "computed": false,
                                                    "optional": false,
                                                    "start": 484,
                                                    "end": 735
                                                  }
                                                ],
                                                "start": 478,
                                                "end": 740
                                              }
                                            ],
                                            "optional": false,
                                            "start": 469,
                                            "end": 741
                                          }
                                        ],
                                        "start": 359,
                                        "end": 745
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 349,
                                      "end": 745
                                    }
                                  ],
                                  "start": 345,
                                  "end": 748
                                }
                              ],
                              "optional": false,
                              "start": 333,
                              "end": 749
                            },
                            "start": 312,
                            "end": 750
                          }
                        ],
                        "start": 159,
                        "end": 752
                      },
                      "id": null,
                      "generator": false,
                      "start": 155,
                      "end": 752
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_Fh88JClhbC0",
                      "raw": "\"App_component_Fh88JClhbC0\"",
                      "start": 754,
                      "end": 781
                    }
                  ],
                  "optional": false,
                  "start": 144,
                  "end": 782
                }
              ],
              "optional": false,
              "start": 131,
              "end": 783
            },
            "definite": false,
            "start": 111,
            "end": 783
          }
        ],
        "declare": false,
        "start": 105,
        "end": 784
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 98,
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
              "decorators": [],
              "name": "STYLES",
              "optional": false,
              "typeAnnotation": null,
              "start": 799,
              "end": 805
            },
            "init": {
              "type": "Literal",
              "value": ".red { color: red; }",
              "raw": "\".red { color: red; }\"",
              "start": 808,
              "end": 830
            },
            "definite": false,
            "start": 799,
            "end": 830
          }
        ],
        "declare": false,
        "start": 793,
        "end": 831
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 786,
      "end": 831
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 831
}
```

</details>

## Output

### Module: test.tsx

```tsx
import { _wrapProp } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { componentQrl, inlinedQrl, useStore, useLexicalScope } from '@qwik.dev/core';
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    useStyles$(/*#__PURE__*/ inlinedQrl(STYLES, "s_odz7dfdfdM"));
    useStyles$(/*#__PURE__*/ inlinedQrl(STYLES, "s_odzdfdfdM"));
    const store = useStore({
        count: 0
    });
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("p", null, null, [
            "Count: ",
            _wrapProp(store, "count")
        ], 3, null),
        /*#__PURE__*/ _jsxSorted("p", null, null, /*#__PURE__*/ _jsxSorted("button", {
            "q-e:click": /*#__PURE__*/ inlinedQrl(()=>{
                const [store] = useLexicalScope();
                return store.count++;
            }, "s_odz7eidI4GM", [
                store
            ])
        }, null, "Click", 2, null), 1, null)
    ], 1, "u6_0");
}, "s_Fh88JClhbC0"));
export const STYLES = ".red { color: red; }";
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
            "name": "_wrapProp",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_wrapProp",
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_jsxSorted",
            "optional": false,
            "typeAnnotation": null,
            "start": 53,
            "end": 63
          },
          "importKind": "value",
          "start": 53,
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
      "importKind": "value",
      "start": 44,
      "end": 88
    },
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
            "start": 98,
            "end": 110
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "componentQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 98,
            "end": 110
          },
          "importKind": "value",
          "start": 98,
          "end": 110
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 112,
            "end": 122
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 112,
            "end": 122
          },
          "importKind": "value",
          "start": 112,
          "end": 122
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 124,
            "end": 132
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 124,
            "end": 132
          },
          "importKind": "value",
          "start": 124,
          "end": 132
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLexicalScope",
            "optional": false,
            "typeAnnotation": null,
            "start": 134,
            "end": 149
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useLexicalScope",
            "optional": false,
            "typeAnnotation": null,
            "start": 134,
            "end": 149
          },
          "importKind": "value",
          "start": 134,
          "end": 149
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 157,
        "end": 173
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 89,
      "end": 174
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
              "start": 188,
              "end": 191
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 208,
                "end": 220
              },
              "typeArguments": null,
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "decorators": [],
                    "name": "inlinedQrl",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 235,
                    "end": 245
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
                                "name": "useStyles$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 256,
                                "end": 266
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "inlinedQrl",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 281,
                                    "end": 291
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "STYLES",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 292,
                                      "end": 298
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "s_odz7dfdfdM",
                                      "raw": "\"s_odz7dfdfdM\"",
                                      "start": 300,
                                      "end": 314
                                    }
                                  ],
                                  "optional": false,
                                  "start": 281,
                                  "end": 315
                                }
                              ],
                              "optional": false,
                              "start": 256,
                              "end": 316
                            },
                            "directive": null,
                            "start": 256,
                            "end": 317
                          },
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStyles$",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 322,
                                "end": 332
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "inlinedQrl",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 347,
                                    "end": 357
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "STYLES",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 358,
                                      "end": 364
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "s_odzdfdfdM",
                                      "raw": "\"s_odzdfdfdM\"",
                                      "start": 366,
                                      "end": 379
                                    }
                                  ],
                                  "optional": false,
                                  "start": 347,
                                  "end": 380
                                }
                              ],
                              "optional": false,
                              "start": 322,
                              "end": 381
                            },
                            "directive": null,
                            "start": 322,
                            "end": 382
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
                                  "name": "store",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 393,
                                  "end": 398
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "useStore",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 401,
                                    "end": 409
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
                                            "start": 420,
                                            "end": 425
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": 0,
                                            "raw": "0",
                                            "start": 427,
                                            "end": 428
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "optional": false,
                                          "start": 420,
                                          "end": 428
                                        }
                                      ],
                                      "start": 410,
                                      "end": 434
                                    }
                                  ],
                                  "optional": false,
                                  "start": 401,
                                  "end": 435
                                },
                                "definite": false,
                                "start": 393,
                                "end": 435
                              }
                            ],
                            "declare": false,
                            "start": 387,
                            "end": 436
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "_jsxSorted",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 462,
                                "end": 472
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 473,
                                  "end": 478
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 480,
                                  "end": 484
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 486,
                                  "end": 490
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "_jsxSorted",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 516,
                                        "end": 526
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "p",
                                          "raw": "\"p\"",
                                          "start": 527,
                                          "end": 530
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 532,
                                          "end": 536
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 538,
                                          "end": 542
                                        },
                                        {
                                          "type": "ArrayExpression",
                                          "elements": [
                                            {
                                              "type": "Literal",
                                              "value": "Count: ",
                                              "raw": "\"Count: \"",
                                              "start": 558,
                                              "end": 567
                                            },
                                            {
                                              "type": "CallExpression",
                                              "callee": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "_wrapProp",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 581,
                                                "end": 590
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "store",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 591,
                                                  "end": 596
                                                },
                                                {
                                                  "type": "Literal",
                                                  "value": "count",
                                                  "raw": "\"count\"",
                                                  "start": 598,
                                                  "end": 605
                                                }
                                              ],
                                              "optional": false,
                                              "start": 581,
                                              "end": 606
                                            }
                                          ],
                                          "start": 544,
                                          "end": 616
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 618,
                                          "end": 619
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 621,
                                          "end": 625
                                        }
                                      ],
                                      "optional": false,
                                      "start": 516,
                                      "end": 626
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "_jsxSorted",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 650,
                                        "end": 660
                                      },
                                      "typeArguments": null,
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "p",
                                          "raw": "\"p\"",
                                          "start": 661,
                                          "end": 664
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 666,
                                          "end": 670
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 672,
                                          "end": 676
                                        },
                                        {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "_jsxSorted",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 692,
                                            "end": 702
                                          },
                                          "typeArguments": null,
                                          "arguments": [
                                            {
                                              "type": "Literal",
                                              "value": "button",
                                              "raw": "\"button\"",
                                              "start": 703,
                                              "end": 711
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
                                                    "start": 727,
                                                    "end": 738
                                                  },
                                                  "value": {
                                                    "type": "CallExpression",
                                                    "callee": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "inlinedQrl",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 754,
                                                      "end": 764
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
                                                                    "type": "ArrayPattern",
                                                                    "decorators": [],
                                                                    "elements": [
                                                                      {
                                                                        "type": "Identifier",
                                                                        "decorators": [],
                                                                        "name": "store",
                                                                        "optional": false,
                                                                        "typeAnnotation": null,
                                                                        "start": 794,
                                                                        "end": 799
                                                                      }
                                                                    ],
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 793,
                                                                    "end": 800
                                                                  },
                                                                  "init": {
                                                                    "type": "CallExpression",
                                                                    "callee": {
                                                                      "type": "Identifier",
                                                                      "decorators": [],
                                                                      "name": "useLexicalScope",
                                                                      "optional": false,
                                                                      "typeAnnotation": null,
                                                                      "start": 803,
                                                                      "end": 818
                                                                    },
                                                                    "typeArguments": null,
                                                                    "arguments": [],
                                                                    "optional": false,
                                                                    "start": 803,
                                                                    "end": 820
                                                                  },
                                                                  "definite": false,
                                                                  "start": 793,
                                                                  "end": 820
                                                                }
                                                              ],
                                                              "declare": false,
                                                              "start": 787,
                                                              "end": 821
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
                                                                    "decorators": [],
                                                                    "name": "store",
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 845,
                                                                    "end": 850
                                                                  },
                                                                  "property": {
                                                                    "type": "Identifier",
                                                                    "decorators": [],
                                                                    "name": "count",
                                                                    "optional": false,
                                                                    "typeAnnotation": null,
                                                                    "start": 851,
                                                                    "end": 856
                                                                  },
                                                                  "optional": false,
                                                                  "computed": false,
                                                                  "start": 845,
                                                                  "end": 856
                                                                },
                                                                "start": 845,
                                                                "end": 858
                                                              },
                                                              "start": 838,
                                                              "end": 859
                                                            }
                                                          ],
                                                          "start": 769,
                                                          "end": 873
                                                        },
                                                        "id": null,
                                                        "generator": false,
                                                        "start": 765,
                                                        "end": 873
                                                      },
                                                      {
                                                        "type": "Literal",
                                                        "value": "s_odz7eidI4GM",
                                                        "raw": "\"s_odz7eidI4GM\"",
                                                        "start": 875,
                                                        "end": 890
                                                      },
                                                      {
                                                        "type": "ArrayExpression",
                                                        "elements": [
                                                          {
                                                            "type": "Identifier",
                                                            "decorators": [],
                                                            "name": "store",
                                                            "optional": false,
                                                            "typeAnnotation": null,
                                                            "start": 910,
                                                            "end": 915
                                                          }
                                                        ],
                                                        "start": 892,
                                                        "end": 929
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 754,
                                                    "end": 930
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "optional": false,
                                                  "start": 727,
                                                  "end": 930
                                                }
                                              ],
                                              "start": 713,
                                              "end": 940
                                            },
                                            {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 942,
                                              "end": 946
                                            },
                                            {
                                              "type": "Literal",
                                              "value": "Click",
                                              "raw": "\"Click\"",
                                              "start": 948,
                                              "end": 955
                                            },
                                            {
                                              "type": "Literal",
                                              "value": 2,
                                              "raw": "2",
                                              "start": 957,
                                              "end": 958
                                            },
                                            {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 960,
                                              "end": 964
                                            }
                                          ],
                                          "optional": false,
                                          "start": 692,
                                          "end": 965
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 1,
                                          "raw": "1",
                                          "start": 967,
                                          "end": 968
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 970,
                                          "end": 974
                                        }
                                      ],
                                      "optional": false,
                                      "start": 650,
                                      "end": 975
                                    }
                                  ],
                                  "start": 492,
                                  "end": 981
                                },
                                {
                                  "type": "Literal",
                                  "value": 1,
                                  "raw": "1",
                                  "start": 983,
                                  "end": 984
                                },
                                {
                                  "type": "Literal",
                                  "value": "u6_0",
                                  "raw": "\"u6_0\"",
                                  "start": 986,
                                  "end": 992
                                }
                              ],
                              "optional": false,
                              "start": 462,
                              "end": 993
                            },
                            "start": 441,
                            "end": 994
                          }
                        ],
                        "start": 250,
                        "end": 996
                      },
                      "id": null,
                      "generator": false,
                      "start": 246,
                      "end": 996
                    },
                    {
                      "type": "Literal",
                      "value": "s_Fh88JClhbC0",
                      "raw": "\"s_Fh88JClhbC0\"",
                      "start": 998,
                      "end": 1013
                    }
                  ],
                  "optional": false,
                  "start": 235,
                  "end": 1014
                }
              ],
              "optional": false,
              "start": 208,
              "end": 1015
            },
            "definite": false,
            "start": 188,
            "end": 1015
          }
        ],
        "declare": false,
        "start": 182,
        "end": 1016
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 175,
      "end": 1016
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
              "name": "STYLES",
              "optional": false,
              "typeAnnotation": null,
              "start": 1030,
              "end": 1036
            },
            "init": {
              "type": "Literal",
              "value": ".red { color: red; }",
              "raw": "\".red { color: red; }\"",
              "start": 1039,
              "end": 1061
            },
            "definite": false,
            "start": 1030,
            "end": 1061
          }
        ],
        "declare": false,
        "start": 1024,
        "end": 1062
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 1017,
      "end": 1062
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1062
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (9 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `inlinedQrl` | test.tsx | @qwik.dev/core | 4 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx | @qwik.dev/core | 4 |
| `_wrapProp` | test.tsx | @qwik.dev/core | 1 |
| `useStore` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
