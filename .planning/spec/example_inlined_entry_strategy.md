# Test: example_inlined_entry_strategy

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |

## Input

### Source Code

```tsx
import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';

export const Child = component$(() => {

	useStyles$('somestring');
	const state = useStore({
		count: 0
	});

	// Double count watch
	useBrowserVisibleTask$(() => {
		state.count = thing.doStuff() + import("./sibling");
	});

	return (
		<div onClick$={() => console.log(mongodb)}>
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
            "name": "useBrowserVisibleTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 43
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useBrowserVisibleTask$",
            "optional": false,
            "typeAnnotation": null,
            "start": 21,
            "end": 43
          },
          "importKind": "value",
          "start": 21,
          "end": 43
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 45,
            "end": 53
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 45,
            "end": 53
          },
          "importKind": "value",
          "start": 45,
          "end": 53
        },
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 55,
            "end": 65
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStyles$",
            "optional": false,
            "typeAnnotation": null,
            "start": 55,
            "end": 65
          },
          "importKind": "value",
          "start": 55,
          "end": 65
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 73,
        "end": 89
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 0,
      "end": 90
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "thing",
            "optional": false,
            "typeAnnotation": null,
            "start": 100,
            "end": 105
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "thing",
            "optional": false,
            "typeAnnotation": null,
            "start": 100,
            "end": 105
          },
          "importKind": "value",
          "start": 100,
          "end": 105
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./sibling",
        "raw": "'./sibling'",
        "start": 113,
        "end": 124
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 91,
      "end": 125
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 133,
            "end": 140
          },
          "start": 133,
          "end": 140
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mongodb",
        "raw": "'mongodb'",
        "start": 146,
        "end": 155
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 126,
      "end": 156
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
              "start": 171,
              "end": 176
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 179,
                "end": 189
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
                            "start": 200,
                            "end": 210
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Literal",
                              "value": "somestring",
                              "raw": "'somestring'",
                              "start": 211,
                              "end": 223
                            }
                          ],
                          "optional": false,
                          "start": 200,
                          "end": 224
                        },
                        "directive": null,
                        "start": 200,
                        "end": 225
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
                              "name": "state",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 233,
                              "end": 238
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 241,
                                "end": 249
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
                                        "start": 254,
                                        "end": 259
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 261,
                                        "end": 262
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 254,
                                      "end": 262
                                    }
                                  ],
                                  "start": 250,
                                  "end": 265
                                }
                              ],
                              "optional": false,
                              "start": 241,
                              "end": 266
                            },
                            "definite": false,
                            "start": 233,
                            "end": 266
                          }
                        ],
                        "declare": false,
                        "start": 227,
                        "end": 267
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "useBrowserVisibleTask$",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 293,
                            "end": 315
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
                                          "start": 326,
                                          "end": 331
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "count",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 332,
                                          "end": 337
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 326,
                                        "end": 337
                                      },
                                      "right": {
                                        "type": "BinaryExpression",
                                        "left": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "thing",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 340,
                                              "end": 345
                                            },
                                            "property": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "doStuff",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 346,
                                              "end": 353
                                            },
                                            "optional": false,
                                            "computed": false,
                                            "start": 340,
                                            "end": 353
                                          },
                                          "typeArguments": null,
                                          "arguments": [],
                                          "optional": false,
                                          "start": 340,
                                          "end": 355
                                        },
                                        "operator": "+",
                                        "right": {
                                          "type": "ImportExpression",
                                          "source": {
                                            "type": "Literal",
                                            "value": "./sibling",
                                            "raw": "\"./sibling\"",
                                            "start": 365,
                                            "end": 376
                                          },
                                          "options": null,
                                          "phase": null,
                                          "start": 358,
                                          "end": 377
                                        },
                                        "start": 340,
                                        "end": 377
                                      },
                                      "start": 326,
                                      "end": 377
                                    },
                                    "directive": null,
                                    "start": 326,
                                    "end": 378
                                  }
                                ],
                                "start": 322,
                                "end": 381
                              },
                              "id": null,
                              "generator": false,
                              "start": 316,
                              "end": 381
                            }
                          ],
                          "optional": false,
                          "start": 293,
                          "end": 382
                        },
                        "directive": null,
                        "start": 293,
                        "end": 383
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
                                "start": 398,
                                "end": 401
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 402,
                                    "end": 410
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
                                            "start": 418,
                                            "end": 425
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "log",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 426,
                                            "end": 429
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 418,
                                          "end": 429
                                        },
                                        "typeArguments": null,
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "mongodb",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 430,
                                            "end": 437
                                          }
                                        ],
                                        "optional": false,
                                        "start": 418,
                                        "end": 438
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 412,
                                      "end": 438
                                    },
                                    "start": 411,
                                    "end": 439
                                  },
                                  "start": 402,
                                  "end": 439
                                }
                              ],
                              "selfClosing": false,
                              "start": 397,
                              "end": 440
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 440,
                                "end": 443
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 445,
                                "end": 448
                              },
                              "start": 443,
                              "end": 449
                            },
                            "start": 397,
                            "end": 449
                          },
                          "start": 393,
                          "end": 452
                        },
                        "start": 386,
                        "end": 453
                      }
                    ],
                    "start": 196,
                    "end": 455
                  },
                  "id": null,
                  "generator": false,
                  "start": 190,
                  "end": 455
                }
              ],
              "optional": false,
              "start": 179,
              "end": 456
            },
            "definite": false,
            "start": 171,
            "end": 456
          }
        ],
        "declare": false,
        "start": 165,
        "end": 457
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 158,
      "end": 457
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 457
}
```

</details>

## Output

### Module: test.tsx

```tsx
import { componentQrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { useBrowserVisibleTaskQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { useStore } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    useStylesQrl(/*#__PURE__*/ inlinedQrl('somestring', "Child_component_useStyles_qBZTuFM0160"));
    const state = useStore({
        count: 0
    });
    // Double count watch
    useBrowserVisibleTaskQrl(/*#__PURE__*/ inlinedQrl(()=>{
        const state = _captures[0];
        state.count = thing.doStuff() + import("./sibling");
    }, "Child_component_useBrowserVisibleTask_0IGFPOyJmQA", [
        state
    ]));
    return <div q-e:click={/*#__PURE__*/ inlinedQrl(()=>console.log(mongodb), "Child_component_div_q_e_click_cROa4sult1s")}>
		</div>;
}, "Child_component_9GyF01GDKqw"));
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
            "name": "useStylesQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 68
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStylesQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 56,
            "end": 68
          },
          "importKind": "value",
          "start": 56,
          "end": 68
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 76,
        "end": 92
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 47,
      "end": 93
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 103,
            "end": 113
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "inlinedQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 103,
            "end": 113
          },
          "importKind": "value",
          "start": 103,
          "end": 113
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 121,
        "end": 137
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 94,
      "end": 138
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useBrowserVisibleTaskQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 148,
            "end": 172
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useBrowserVisibleTaskQrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 148,
            "end": 172
          },
          "importKind": "value",
          "start": 148,
          "end": 172
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 180,
        "end": 196
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 139,
      "end": 197
    },
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
            "start": 207,
            "end": 216
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "_captures",
            "optional": false,
            "typeAnnotation": null,
            "start": 207,
            "end": 216
          },
          "importKind": "value",
          "start": 207,
          "end": 216
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 224,
        "end": 240
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 198,
      "end": 241
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 251,
            "end": 259
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "useStore",
            "optional": false,
            "typeAnnotation": null,
            "start": 251,
            "end": 259
          },
          "importKind": "value",
          "start": 251,
          "end": 259
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "'@qwik.dev/core'",
        "start": 267,
        "end": 283
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 242,
      "end": 284
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "decorators": [],
            "name": "thing",
            "optional": false,
            "typeAnnotation": null,
            "start": 294,
            "end": 299
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "thing",
            "optional": false,
            "typeAnnotation": null,
            "start": 294,
            "end": 299
          },
          "importKind": "value",
          "start": 294,
          "end": 299
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./sibling",
        "raw": "'./sibling'",
        "start": 307,
        "end": 318
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 285,
      "end": 319
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportDefaultSpecifier",
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "mongodb",
            "optional": false,
            "typeAnnotation": null,
            "start": 327,
            "end": 334
          },
          "start": 327,
          "end": 334
        }
      ],
      "source": {
        "type": "Literal",
        "value": "mongodb",
        "raw": "'mongodb'",
        "start": 340,
        "end": 349
      },
      "phase": null,
      "attributes": [],
      "importKind": "value",
      "start": 320,
      "end": 350
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
              "start": 364,
              "end": 369
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "componentQrl",
                "optional": false,
                "typeAnnotation": null,
                "start": 386,
                "end": 398
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
                    "start": 413,
                    "end": 423
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
                                "name": "useStylesQrl",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 434,
                                "end": 446
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
                                    "start": 461,
                                    "end": 471
                                  },
                                  "typeArguments": null,
                                  "arguments": [
                                    {
                                      "type": "Literal",
                                      "value": "somestring",
                                      "raw": "'somestring'",
                                      "start": 472,
                                      "end": 484
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "Child_component_useStyles_qBZTuFM0160",
                                      "raw": "\"Child_component_useStyles_qBZTuFM0160\"",
                                      "start": 486,
                                      "end": 525
                                    }
                                  ],
                                  "optional": false,
                                  "start": 461,
                                  "end": 526
                                }
                              ],
                              "optional": false,
                              "start": 434,
                              "end": 527
                            },
                            "directive": null,
                            "start": 434,
                            "end": 528
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
                                  "name": "state",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 539,
                                  "end": 544
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "useStore",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 547,
                                    "end": 555
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
                                            "start": 566,
                                            "end": 571
                                          },
                                          "value": {
                                            "type": "Literal",
                                            "value": 0,
                                            "raw": "0",
                                            "start": 573,
                                            "end": 574
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "optional": false,
                                          "start": 566,
                                          "end": 574
                                        }
                                      ],
                                      "start": 556,
                                      "end": 580
                                    }
                                  ],
                                  "optional": false,
                                  "start": 547,
                                  "end": 581
                                },
                                "definite": false,
                                "start": 539,
                                "end": 581
                              }
                            ],
                            "declare": false,
                            "start": 533,
                            "end": 582
                          },
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useBrowserVisibleTaskQrl",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 613,
                                "end": 637
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
                                    "start": 652,
                                    "end": 662
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
                                                  "start": 683,
                                                  "end": 688
                                                },
                                                "init": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "_captures",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 691,
                                                    "end": 700
                                                  },
                                                  "property": {
                                                    "type": "Literal",
                                                    "value": 0,
                                                    "raw": "0",
                                                    "start": 701,
                                                    "end": 702
                                                  },
                                                  "optional": false,
                                                  "computed": true,
                                                  "start": 691,
                                                  "end": 703
                                                },
                                                "definite": false,
                                                "start": 683,
                                                "end": 703
                                              }
                                            ],
                                            "declare": false,
                                            "start": 677,
                                            "end": 704
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
                                                  "decorators": [],
                                                  "name": "state",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 713,
                                                  "end": 718
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "count",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 719,
                                                  "end": 724
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 713,
                                                "end": 724
                                              },
                                              "right": {
                                                "type": "BinaryExpression",
                                                "left": {
                                                  "type": "CallExpression",
                                                  "callee": {
                                                    "type": "MemberExpression",
                                                    "object": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "thing",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 727,
                                                      "end": 732
                                                    },
                                                    "property": {
                                                      "type": "Identifier",
                                                      "decorators": [],
                                                      "name": "doStuff",
                                                      "optional": false,
                                                      "typeAnnotation": null,
                                                      "start": 733,
                                                      "end": 740
                                                    },
                                                    "optional": false,
                                                    "computed": false,
                                                    "start": 727,
                                                    "end": 740
                                                  },
                                                  "typeArguments": null,
                                                  "arguments": [],
                                                  "optional": false,
                                                  "start": 727,
                                                  "end": 742
                                                },
                                                "operator": "+",
                                                "right": {
                                                  "type": "ImportExpression",
                                                  "source": {
                                                    "type": "Literal",
                                                    "value": "./sibling",
                                                    "raw": "\"./sibling\"",
                                                    "start": 752,
                                                    "end": 763
                                                  },
                                                  "options": null,
                                                  "phase": null,
                                                  "start": 745,
                                                  "end": 764
                                                },
                                                "start": 727,
                                                "end": 764
                                              },
                                              "start": 713,
                                              "end": 764
                                            },
                                            "directive": null,
                                            "start": 713,
                                            "end": 765
                                          }
                                        ],
                                        "start": 667,
                                        "end": 771
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 663,
                                      "end": 771
                                    },
                                    {
                                      "type": "Literal",
                                      "value": "Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
                                      "raw": "\"Child_component_useBrowserVisibleTask_0IGFPOyJmQA\"",
                                      "start": 773,
                                      "end": 824
                                    },
                                    {
                                      "type": "ArrayExpression",
                                      "elements": [
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "state",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 836,
                                          "end": 841
                                        }
                                      ],
                                      "start": 826,
                                      "end": 847
                                    }
                                  ],
                                  "optional": false,
                                  "start": 652,
                                  "end": 848
                                }
                              ],
                              "optional": false,
                              "start": 613,
                              "end": 849
                            },
                            "directive": null,
                            "start": 613,
                            "end": 850
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 863,
                                  "end": 866
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
                                        "start": 867,
                                        "end": 870
                                      },
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "click",
                                        "start": 871,
                                        "end": 876
                                      },
                                      "start": 867,
                                      "end": 876
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "inlinedQrl",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 892,
                                          "end": 902
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
                                                  "start": 907,
                                                  "end": 914
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "log",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 915,
                                                  "end": 918
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 907,
                                                "end": 918
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "mongodb",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 919,
                                                  "end": 926
                                                }
                                              ],
                                              "optional": false,
                                              "start": 907,
                                              "end": 927
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 903,
                                            "end": 927
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "Child_component_div_q_e_click_cROa4sult1s",
                                            "raw": "\"Child_component_div_q_e_click_cROa4sult1s\"",
                                            "start": 929,
                                            "end": 972
                                          }
                                        ],
                                        "optional": false,
                                        "start": 892,
                                        "end": 973
                                      },
                                      "start": 877,
                                      "end": 974
                                    },
                                    "start": 867,
                                    "end": 974
                                  }
                                ],
                                "selfClosing": false,
                                "start": 862,
                                "end": 975
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t",
                                  "raw": "\n\t\t",
                                  "start": 975,
                                  "end": 978
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "div",
                                  "start": 980,
                                  "end": 983
                                },
                                "start": 978,
                                "end": 984
                              },
                              "start": 862,
                              "end": 984
                            },
                            "start": 855,
                            "end": 985
                          }
                        ],
                        "start": 428,
                        "end": 987
                      },
                      "id": null,
                      "generator": false,
                      "start": 424,
                      "end": 987
                    },
                    {
                      "type": "Literal",
                      "value": "Child_component_9GyF01GDKqw",
                      "raw": "\"Child_component_9GyF01GDKqw\"",
                      "start": 989,
                      "end": 1018
                    }
                  ],
                  "optional": false,
                  "start": 413,
                  "end": 1019
                }
              ],
              "optional": false,
              "start": 386,
              "end": 1020
            },
            "definite": false,
            "start": 364,
            "end": 1020
          }
        ],
        "declare": false,
        "start": 358,
        "end": 1021
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 351,
      "end": 1021
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 1021
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `inlinedQrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`, `useStylesQrl`, `useBrowserVisibleTaskQrl`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (5 occurrence(s))

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `inlinedQrl` | test.tsx | @qwik.dev/core | 4 |
| `componentQrl` | test.tsx | @qwik.dev/core | 1 |
| `useStylesQrl` | test.tsx | @qwik.dev/core | 1 |
| `useBrowserVisibleTaskQrl` | test.tsx | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx | @qwik.dev/core | 1 |
| `useStore` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

No diagnostics.
