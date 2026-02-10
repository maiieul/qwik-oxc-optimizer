# Test: should_handle_dangerously_set_inner_html

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile Ts | `True` |
| Transpile Jsx | `True` |

## Input

### Source Code

```tsx
import { component$ } from "@qwik.dev/core";
		const Cmp = component$(() => {
			const htmlSignal = useSignal("<h2><span>I'm a signal value!</span></h2>");
			return (
				<div>
					<div>
						<span id="first" dangerouslySetInnerHTML="vanilla HTML here" />
					</div>
					<div>
						<span id="second" dangerouslySetInnerHTML="<h1>I'm an h1!</h1>" class="after" />
					</div>
					<div>
						<span id="third" dangerouslySetInnerHTML={htmlSignal.value} class="after" />
						<button
							onClick$={() =>
								(htmlSignal.value = "<h2><span>I'm a updated signal value!</span></h2>")
							}
						></button>
					</div>
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
      "importKind": "value",
      "start": 0,
      "end": 44
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
            "name": "Cmp",
            "optional": false,
            "typeAnnotation": null,
            "start": 53,
            "end": 56
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "component$",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
              "end": 69
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
                            "name": "htmlSignal",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 87,
                            "end": 97
                          },
                          "init": {
                            "type": "CallExpression",
                            "callee": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "useSignal",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 100,
                              "end": 109
                            },
                            "typeArguments": null,
                            "arguments": [
                              {
                                "type": "Literal",
                                "value": "<h2><span>I'm a signal value!</span></h2>",
                                "raw": "\"<h2><span>I'm a signal value!</span></h2>\"",
                                "start": 110,
                                "end": 153
                              }
                            ],
                            "optional": false,
                            "start": 100,
                            "end": 154
                          },
                          "definite": false,
                          "start": 87,
                          "end": 154
                        }
                      ],
                      "declare": false,
                      "start": 81,
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
                              "start": 173,
                              "end": 176
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 172,
                            "end": 177
                          },
                          "children": [
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t\t",
                              "raw": "\n\t\t\t\t\t",
                              "start": 177,
                              "end": 183
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 184,
                                  "end": 187
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 183,
                                "end": 188
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\t",
                                  "start": 188,
                                  "end": 195
                                },
                                {
                                  "type": "JSXElement",
                                  "openingElement": {
                                    "type": "JSXOpeningElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "span",
                                      "start": 196,
                                      "end": 200
                                    },
                                    "typeArguments": null,
                                    "attributes": [
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "id",
                                          "start": 201,
                                          "end": 203
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "first",
                                          "raw": "\"first\"",
                                          "start": 204,
                                          "end": 211
                                        },
                                        "start": 201,
                                        "end": 211
                                      },
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "dangerouslySetInnerHTML",
                                          "start": 212,
                                          "end": 235
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "vanilla HTML here",
                                          "raw": "\"vanilla HTML here\"",
                                          "start": 236,
                                          "end": 255
                                        },
                                        "start": 212,
                                        "end": 255
                                      }
                                    ],
                                    "selfClosing": true,
                                    "start": 195,
                                    "end": 258
                                  },
                                  "children": [],
                                  "closingElement": null,
                                  "start": 195,
                                  "end": 258
                                },
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t",
                                  "start": 258,
                                  "end": 264
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 266,
                                  "end": 269
                                },
                                "start": 264,
                                "end": 270
                              },
                              "start": 183,
                              "end": 270
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t\t",
                              "raw": "\n\t\t\t\t\t",
                              "start": 270,
                              "end": 276
                            },
                            {
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
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\t",
                                  "start": 281,
                                  "end": 288
                                },
                                {
                                  "type": "JSXElement",
                                  "openingElement": {
                                    "type": "JSXOpeningElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "span",
                                      "start": 289,
                                      "end": 293
                                    },
                                    "typeArguments": null,
                                    "attributes": [
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "id",
                                          "start": 294,
                                          "end": 296
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "second",
                                          "raw": "\"second\"",
                                          "start": 297,
                                          "end": 305
                                        },
                                        "start": 294,
                                        "end": 305
                                      },
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "dangerouslySetInnerHTML",
                                          "start": 306,
                                          "end": 329
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "<h1>I'm an h1!</h1>",
                                          "raw": "\"<h1>I'm an h1!</h1>\"",
                                          "start": 330,
                                          "end": 351
                                        },
                                        "start": 306,
                                        "end": 351
                                      },
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "class",
                                          "start": 352,
                                          "end": 357
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "after",
                                          "raw": "\"after\"",
                                          "start": 358,
                                          "end": 365
                                        },
                                        "start": 352,
                                        "end": 365
                                      }
                                    ],
                                    "selfClosing": true,
                                    "start": 288,
                                    "end": 368
                                  },
                                  "children": [],
                                  "closingElement": null,
                                  "start": 288,
                                  "end": 368
                                },
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t",
                                  "start": 368,
                                  "end": 374
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 376,
                                  "end": 379
                                },
                                "start": 374,
                                "end": 380
                              },
                              "start": 276,
                              "end": 380
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t\t",
                              "raw": "\n\t\t\t\t\t",
                              "start": 380,
                              "end": 386
                            },
                            {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 387,
                                  "end": 390
                                },
                                "typeArguments": null,
                                "attributes": [],
                                "selfClosing": false,
                                "start": 386,
                                "end": 391
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\t",
                                  "start": 391,
                                  "end": 398
                                },
                                {
                                  "type": "JSXElement",
                                  "openingElement": {
                                    "type": "JSXOpeningElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "span",
                                      "start": 399,
                                      "end": 403
                                    },
                                    "typeArguments": null,
                                    "attributes": [
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "id",
                                          "start": 404,
                                          "end": 406
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "third",
                                          "raw": "\"third\"",
                                          "start": 407,
                                          "end": 414
                                        },
                                        "start": 404,
                                        "end": 414
                                      },
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "dangerouslySetInnerHTML",
                                          "start": 415,
                                          "end": 438
                                        },
                                        "value": {
                                          "type": "JSXExpressionContainer",
                                          "expression": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "htmlSignal",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 440,
                                              "end": 450
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "value",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 451,
                                              "end": 456
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 440,
                                            "end": 456
                                          },
                                          "start": 439,
                                          "end": 457
                                        },
                                        "start": 415,
                                        "end": 457
                                      },
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "class",
                                          "start": 458,
                                          "end": 463
                                        },
                                        "value": {
                                          "type": "Literal",
                                          "value": "after",
                                          "raw": "\"after\"",
                                          "start": 464,
                                          "end": 471
                                        },
                                        "start": 458,
                                        "end": 471
                                      }
                                    ],
                                    "selfClosing": true,
                                    "start": 398,
                                    "end": 474
                                  },
                                  "children": [],
                                  "closingElement": null,
                                  "start": 398,
                                  "end": 474
                                },
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t\t",
                                  "start": 474,
                                  "end": 481
                                },
                                {
                                  "type": "JSXElement",
                                  "openingElement": {
                                    "type": "JSXOpeningElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "button",
                                      "start": 482,
                                      "end": 488
                                    },
                                    "typeArguments": null,
                                    "attributes": [
                                      {
                                        "type": "JSXAttribute",
                                        "name": {
                                          "type": "JSXIdentifier",
                                          "name": "onClick$",
                                          "start": 496,
                                          "end": 504
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
                                              "type": "ParenthesizedExpression",
                                              "expression": {
                                                "type": "AssignmentExpression",
                                                "operator": "=",
                                                "left": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "htmlSignal",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 521,
                                                    "end": 531
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "value",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 532,
                                                    "end": 537
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 521,
                                                  "end": 537
                                                },
                                                "right": {
                                                  "type": "Literal",
                                                  "value": "<h2><span>I'm a updated signal value!</span></h2>",
                                                  "raw": "\"<h2><span>I'm a updated signal value!</span></h2>\"",
                                                  "start": 540,
                                                  "end": 591
                                                },
                                                "start": 521,
                                                "end": 591
                                              },
                                              "start": 520,
                                              "end": 592
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 506,
                                            "end": 592
                                          },
                                          "start": 505,
                                          "end": 601
                                        },
                                        "start": 496,
                                        "end": 601
                                      }
                                    ],
                                    "selfClosing": false,
                                    "start": 481,
                                    "end": 609
                                  },
                                  "children": [],
                                  "closingElement": {
                                    "type": "JSXClosingElement",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "button",
                                      "start": 611,
                                      "end": 617
                                    },
                                    "start": 609,
                                    "end": 618
                                  },
                                  "start": 481,
                                  "end": 618
                                },
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t\t\t\t",
                                  "raw": "\n\t\t\t\t\t",
                                  "start": 618,
                                  "end": 624
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 626,
                                  "end": 629
                                },
                                "start": 624,
                                "end": 630
                              },
                              "start": 386,
                              "end": 630
                            },
                            {
                              "type": "JSXText",
                              "value": "\n\t\t\t\t",
                              "raw": "\n\t\t\t\t",
                              "start": 630,
                              "end": 635
                            }
                          ],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "div",
                              "start": 637,
                              "end": 640
                            },
                            "start": 635,
                            "end": 641
                          },
                          "start": 172,
                          "end": 641
                        },
                        "start": 166,
                        "end": 646
                      },
                      "start": 159,
                      "end": 647
                    }
                  ],
                  "start": 76,
                  "end": 651
                },
                "id": null,
                "generator": false,
                "start": 70,
                "end": 651
              }
            ],
            "optional": false,
            "start": 59,
            "end": 652
          },
          "definite": false,
          "start": 53,
          "end": 652
        }
      ],
      "declare": false,
      "start": 47,
      "end": 653
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 653
}
```

</details>

## Output

### Module: `test.js`

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_4ryKJTOKjWE = ()=>import("./test.tsx_Cmp_component_4ryKJTOKjWE");
/*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_4ryKJTOKjWE, "Cmp_component_4ryKJTOKjWE"));
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
            "name": "i_4ryKJTOKjWE",
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
                "value": "./test.tsx_Cmp_component_4ryKJTOKjWE",
                "raw": "\"./test.tsx_Cmp_component_4ryKJTOKjWE\"",
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
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "name": "componentQrl",
          "start": 173,
          "end": 185
        },
        "arguments": [
          {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "name": "qrl",
              "start": 200,
              "end": 203
            },
            "arguments": [
              {
                "type": "Identifier",
                "name": "i_4ryKJTOKjWE",
                "start": 204,
                "end": 217
              },
              {
                "type": "Literal",
                "value": "Cmp_component_4ryKJTOKjWE",
                "raw": "\"Cmp_component_4ryKJTOKjWE\"",
                "start": 219,
                "end": 246
              }
            ],
            "optional": false,
            "start": 200,
            "end": 247
          }
        ],
        "optional": false,
        "start": 173,
        "end": 248
      },
      "start": 173,
      "end": 249
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 249
}
```

</details>

### Module: `test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8.js` (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8 = ()=>{
    const htmlSignal = _captures[0];
    return htmlSignal.value = "<h2><span>I'm a updated signal value!</span></h2>";
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
              "name": "Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8",
              "start": 57,
              "end": 107
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
                          "name": "htmlSignal",
                          "start": 126,
                          "end": 136
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 139,
                            "end": 148
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 149,
                            "end": 150
                          },
                          "optional": false,
                          "computed": true,
                          "start": 139,
                          "end": 151
                        },
                        "start": 126,
                        "end": 151
                      }
                    ],
                    "start": 120,
                    "end": 152
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "AssignmentExpression",
                      "operator": "=",
                      "left": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "htmlSignal",
                          "start": 164,
                          "end": 174
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "value",
                          "start": 175,
                          "end": 180
                        },
                        "optional": false,
                        "computed": false,
                        "start": 164,
                        "end": 180
                      },
                      "right": {
                        "type": "Literal",
                        "value": "<h2><span>I'm a updated signal value!</span></h2>",
                        "raw": "\"<h2><span>I'm a updated signal value!</span></h2>\"",
                        "start": 183,
                        "end": 234
                      },
                      "start": 164,
                      "end": 234
                    },
                    "start": 157,
                    "end": 235
                  }
                ],
                "start": 114,
                "end": 237
              },
              "id": null,
              "generator": false,
              "start": 110,
              "end": 237
            },
            "start": 57,
            "end": 237
          }
        ],
        "start": 51,
        "end": 238
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 238
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 238
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8",
  "entry": null,
  "displayName": "test.tsx_Cmp_component_div_div_button_q_e_click",
  "hash": "kOQ2BgBVVS8",
  "canonicalFilename": "test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8",
  "path": "",
  "extension": "js",
  "parent": "Cmp_component_4ryKJTOKjWE",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    510,
    596
  ],
  "captureNames": [
    "htmlSignal"
  ]
}
```

### Module: `test.tsx_Cmp_component_4ryKJTOKjWE.js` (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_kOQ2BgBVVS8 = ()=>import("./test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8");
export const Cmp_component_4ryKJTOKjWE = ()=>{
    const htmlSignal = useSignal("<h2><span>I'm a signal value!</span></h2>");
    return /*#__PURE__*/ _jsxSorted("div", null, null, [
        /*#__PURE__*/ _jsxSorted("div", null, null, /*#__PURE__*/ _jsxSorted("span", null, {
            id: "first",
            dangerouslySetInnerHTML: "vanilla HTML here"
        }, null, 3, null), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, /*#__PURE__*/ _jsxSorted("span", null, {
            id: "second",
            dangerouslySetInnerHTML: "<h1>I'm an h1!</h1>",
            class: "after"
        }, null, 3, null), 3, null),
        /*#__PURE__*/ _jsxSorted("div", null, null, [
            /*#__PURE__*/ _jsxSorted("span", null, {
                id: "third",
                dangerouslySetInnerHTML: _wrapProp(htmlSignal),
                class: "after"
            }, null, 3, null),
            /*#__PURE__*/ _jsxSorted("button", null, {
                "q-e:click": /*#__PURE__*/ qrl(i_kOQ2BgBVVS8, "Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8", [
                    htmlSignal
                ])
            }, null, 3, null)
        ], 3, null)
    ], 3, "u6_0");
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
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_kOQ2BgBVVS8",
            "start": 133,
            "end": 146
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
                "value": "./test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8",
                "raw": "\"./test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8\"",
                "start": 160,
                "end": 223
              },
              "options": null,
              "phase": null,
              "start": 153,
              "end": 224
            },
            "id": null,
            "generator": false,
            "start": 149,
            "end": 224
          },
          "start": 133,
          "end": 224
        }
      ],
      "start": 127,
      "end": 225
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
              "name": "Cmp_component_4ryKJTOKjWE",
              "start": 239,
              "end": 264
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
                          "name": "htmlSignal",
                          "start": 283,
                          "end": 293
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useSignal",
                            "start": 296,
                            "end": 305
                          },
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "<h2><span>I'm a signal value!</span></h2>",
                              "raw": "\"<h2><span>I'm a signal value!</span></h2>\"",
                              "start": 306,
                              "end": 349
                            }
                          ],
                          "optional": false,
                          "start": 296,
                          "end": 350
                        },
                        "start": 283,
                        "end": 350
                      }
                    ],
                    "start": 277,
                    "end": 351
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 377,
                        "end": 387
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 388,
                          "end": 393
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 395,
                          "end": 399
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 401,
                          "end": 405
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 431,
                                "end": 441
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 442,
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
                                      "value": "span",
                                      "raw": "\"span\"",
                                      "start": 486,
                                      "end": 492
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 494,
                                      "end": 498
                                    },
                                    {
                                      "type": "ObjectExpression",
                                      "properties": [
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "id",
                                            "start": 514,
                                            "end": 516
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": "first",
                                            "raw": "\"first\"",
                                            "start": 518,
                                            "end": 525
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 514,
                                          "end": 525
                                        },
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "dangerouslySetInnerHTML",
                                            "start": 539,
                                            "end": 562
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": "vanilla HTML here",
                                            "raw": "\"vanilla HTML here\"",
                                            "start": 564,
                                            "end": 583
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 539,
                                          "end": 583
                                        }
                                      ],
                                      "start": 500,
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
                                      "value": 3,
                                      "raw": "3",
                                      "start": 601,
                                      "end": 602
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 604,
                                      "end": 608
                                    }
                                  ],
                                  "optional": false,
                                  "start": 475,
                                  "end": 609
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 611,
                                  "end": 612
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 614,
                                  "end": 618
                                }
                              ],
                              "optional": false,
                              "start": 431,
                              "end": 619
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 643,
                                "end": 653
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 654,
                                  "end": 659
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 661,
                                  "end": 665
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 667,
                                  "end": 671
                                },
                                {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "_jsxSorted",
                                    "start": 687,
                                    "end": 697
                                  },
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "span",
                                      "raw": "\"span\"",
                                      "start": 698,
                                      "end": 704
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 706,
                                      "end": 710
                                    },
                                    {
                                      "type": "ObjectExpression",
                                      "properties": [
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "id",
                                            "start": 726,
                                            "end": 728
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": "second",
                                            "raw": "\"second\"",
                                            "start": 730,
                                            "end": 738
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 726,
                                          "end": 738
                                        },
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "dangerouslySetInnerHTML",
                                            "start": 752,
                                            "end": 775
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": "<h1>I'm an h1!</h1>",
                                            "raw": "\"<h1>I'm an h1!</h1>\"",
                                            "start": 777,
                                            "end": 798
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 752,
                                          "end": 798
                                        },
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "name": "class",
                                            "start": 812,
                                            "end": 817
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": "after",
                                            "raw": "\"after\"",
                                            "start": 819,
                                            "end": 826
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 812,
                                          "end": 826
                                        }
                                      ],
                                      "start": 712,
                                      "end": 836
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 838,
                                      "end": 842
                                    },
                                    {
                                      "type": "Literal",
                                      "value": 3,
                                      "raw": "3",
                                      "start": 844,
                                      "end": 845
                                    },
                                    {
                                      "type": "Literal",
                                      "value": null,
                                      "raw": "null",
                                      "start": 847,
                                      "end": 851
                                    }
                                  ],
                                  "optional": false,
                                  "start": 687,
                                  "end": 852
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 854,
                                  "end": 855
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 857,
                                  "end": 861
                                }
                              ],
                              "optional": false,
                              "start": 643,
                              "end": 862
                            },
                            {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "_jsxSorted",
                                "start": 886,
                                "end": 896
                              },
                              "arguments": [
                                {
                                  "type": "Literal",
                                  "value": "div",
                                  "raw": "\"div\"",
                                  "start": 897,
                                  "end": 902
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 904,
                                  "end": 908
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 910,
                                  "end": 914
                                },
                                {
                                  "type": "ArrayExpression",
                                  "elements": [
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 944,
                                        "end": 954
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "span",
                                          "raw": "\"span\"",
                                          "start": 955,
                                          "end": 961
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 963,
                                          "end": 967
                                        },
                                        {
                                          "type": "ObjectExpression",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "id",
                                                "start": 987,
                                                "end": 989
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": "third",
                                                "raw": "\"third\"",
                                                "start": 991,
                                                "end": 998
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 987,
                                              "end": 998
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "dangerouslySetInnerHTML",
                                                "start": 1016,
                                                "end": 1039
                                              },
                                              "value": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "_wrapProp",
                                                  "start": 1041,
                                                  "end": 1050
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "htmlSignal",
                                                    "start": 1051,
                                                    "end": 1061
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 1041,
                                                "end": 1062
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 1016,
                                              "end": 1062
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "class",
                                                "start": 1080,
                                                "end": 1085
                                              },
                                              "value": {
                                                "type": "Literal",
                                                "value": "after",
                                                "raw": "\"after\"",
                                                "start": 1087,
                                                "end": 1094
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 1080,
                                              "end": 1094
                                            }
                                          ],
                                          "start": 969,
                                          "end": 1108
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1110,
                                          "end": 1114
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1116,
                                          "end": 1117
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1119,
                                          "end": 1123
                                        }
                                      ],
                                      "optional": false,
                                      "start": 944,
                                      "end": 1124
                                    },
                                    {
                                      "type": "CallExpression",
                                      "callee": {
                                        "type": "Identifier",
                                        "name": "_jsxSorted",
                                        "start": 1152,
                                        "end": 1162
                                      },
                                      "arguments": [
                                        {
                                          "type": "Literal",
                                          "value": "button",
                                          "raw": "\"button\"",
                                          "start": 1163,
                                          "end": 1171
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1173,
                                          "end": 1177
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
                                                "start": 1197,
                                                "end": 1208
                                              },
                                              "value": {
                                                "type": "CallExpression",
                                                "callee": {
                                                  "type": "Identifier",
                                                  "name": "qrl",
                                                  "start": 1224,
                                                  "end": 1227
                                                },
                                                "arguments": [
                                                  {
                                                    "type": "Identifier",
                                                    "name": "i_kOQ2BgBVVS8",
                                                    "start": 1228,
                                                    "end": 1241
                                                  },
                                                  {
                                                    "type": "Literal",
                                                    "value": "Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8",
                                                    "raw": "\"Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8\"",
                                                    "start": 1243,
                                                    "end": 1295
                                                  },
                                                  {
                                                    "type": "ArrayExpression",
                                                    "elements": [
                                                      {
                                                        "type": "Identifier",
                                                        "name": "htmlSignal",
                                                        "start": 1319,
                                                        "end": 1329
                                                      }
                                                    ],
                                                    "start": 1297,
                                                    "end": 1347
                                                  }
                                                ],
                                                "optional": false,
                                                "start": 1224,
                                                "end": 1348
                                              },
                                              "method": false,
                                              "shorthand": false,
                                              "computed": false,
                                              "start": 1197,
                                              "end": 1348
                                            }
                                          ],
                                          "start": 1179,
                                          "end": 1362
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1364,
                                          "end": 1368
                                        },
                                        {
                                          "type": "Literal",
                                          "value": 3,
                                          "raw": "3",
                                          "start": 1370,
                                          "end": 1371
                                        },
                                        {
                                          "type": "Literal",
                                          "value": null,
                                          "raw": "null",
                                          "start": 1373,
                                          "end": 1377
                                        }
                                      ],
                                      "optional": false,
                                      "start": 1152,
                                      "end": 1378
                                    }
                                  ],
                                  "start": 916,
                                  "end": 1388
                                },
                                {
                                  "type": "Literal",
                                  "value": 3,
                                  "raw": "3",
                                  "start": 1390,
                                  "end": 1391
                                },
                                {
                                  "type": "Literal",
                                  "value": null,
                                  "raw": "null",
                                  "start": 1393,
                                  "end": 1397
                                }
                              ],
                              "optional": false,
                              "start": 886,
                              "end": 1398
                            }
                          ],
                          "start": 407,
                          "end": 1404
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 1406,
                          "end": 1407
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 1409,
                          "end": 1415
                        }
                      ],
                      "optional": false,
                      "start": 377,
                      "end": 1416
                    },
                    "start": 356,
                    "end": 1417
                  }
                ],
                "start": 271,
                "end": 1419
              },
              "id": null,
              "generator": false,
              "start": 267,
              "end": 1419
            },
            "start": 239,
            "end": 1419
          }
        ],
        "start": 233,
        "end": 1420
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 226,
      "end": 1420
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1420
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Cmp_component_4ryKJTOKjWE",
  "entry": null,
  "displayName": "test.tsx_Cmp_component",
  "hash": "4ryKJTOKjWE",
  "canonicalFilename": "test.tsx_Cmp_component_4ryKJTOKjWE",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    74,
    655
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Calls**: Output uses `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-Qrl Transform**: Dollar-sign functions converted: `componentQrl()`
- **[CONV-03] JSX Transforms**: JSX transpiled using `_jsxSorted()`
- **[CONV-04] Signal Helpers**: Signal optimization via `_wrapProp()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Multiple lazy import declarations (`const i_HASH = () => import(...)`) for deferred module loading (2 total)
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` tree-shaking hints on 11 call expressions
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point module(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `componentQrl` | `test.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.js` | `@qwik.dev/core` | 2 |
| `_captures` | `test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8.js` | `@qwik.dev/core` | 2 |
| `qrl` | `test.tsx_Cmp_component_4ryKJTOKjWE.js` | `@qwik.dev/core` | 2 |
| `_jsxSorted` | `test.tsx_Cmp_component_4ryKJTOKjWE.js` | `@qwik.dev/core` | 9 |
| `_wrapProp` | `test.tsx_Cmp_component_4ryKJTOKjWE.js` | `@qwik.dev/core` | 2 |
| `useSignal` | `test.tsx_Cmp_component_4ryKJTOKjWE.js` | `-` | 1 |

## Diagnostics

```json
[]
```
