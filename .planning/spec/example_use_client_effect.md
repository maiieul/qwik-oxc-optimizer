# Test: example_use_client_effect

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';

export const Child = component$(() => {
	const state = useStore({
		count: 0
	});

	// Double count watch
	useBrowserVisibleTask$(() => {
		const timer = setInterval(() => {
		state.count++;
		}, 1000);
		return () => {
		clearInterval(timer);
		}
	});

	return (
		<div>
		{state.count}
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
              "start": 105,
              "end": 110
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 113,
                "end": 123
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
                              "start": 139,
                              "end": 144
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 147,
                                "end": 155
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
                                        "start": 160,
                                        "end": 165
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 167,
                                        "end": 168
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 160,
                                      "end": 168
                                    }
                                  ],
                                  "start": 156,
                                  "end": 171
                                }
                              ],
                              "optional": false,
                              "start": 147,
                              "end": 172
                            },
                            "definite": false,
                            "start": 139,
                            "end": 172
                          }
                        ],
                        "declare": false,
                        "start": 133,
                        "end": 173
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
                            "start": 199,
                            "end": 221
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
                                          "name": "timer",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 238,
                                          "end": 243
                                        },
                                        "init": {
                                          "type": "CallExpression",
                                          "callee": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "setInterval",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 246,
                                            "end": 257
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
                                                      "type": "UpdateExpression",
                                                      "operator": "++",
                                                      "prefix": false,
                                                      "argument": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "state",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 268,
                                                          "end": 273
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "decorators": [],
                                                          "name": "count",
                                                          "optional": false,
                                                          "typeAnnotation": null,
                                                          "start": 274,
                                                          "end": 279
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 268,
                                                        "end": 279
                                                      },
                                                      "start": 268,
                                                      "end": 281
                                                    },
                                                    "directive": null,
                                                    "start": 268,
                                                    "end": 282
                                                  }
                                                ],
                                                "start": 264,
                                                "end": 286
                                              },
                                              "id": null,
                                              "generator": false,
                                              "start": 258,
                                              "end": 286
                                            },
                                            {
                                              "type": "Literal",
                                              "value": 1000,
                                              "raw": "1000",
                                              "start": 288,
                                              "end": 292
                                            }
                                          ],
                                          "optional": false,
                                          "start": 246,
                                          "end": 293
                                        },
                                        "definite": false,
                                        "start": 238,
                                        "end": 293
                                      }
                                    ],
                                    "declare": false,
                                    "start": 232,
                                    "end": 294
                                  },
                                  {
                                    "type": "ReturnStatement",
                                    "argument": {
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
                                                "name": "clearInterval",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 314,
                                                "end": 327
                                              },
                                              "typeArguments": null,
                                              "arguments": [
                                                {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "timer",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 328,
                                                  "end": 333
                                                }
                                              ],
                                              "optional": false,
                                              "start": 314,
                                              "end": 334
                                            },
                                            "directive": null,
                                            "start": 314,
                                            "end": 335
                                          }
                                        ],
                                        "start": 310,
                                        "end": 339
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 304,
                                      "end": 339
                                    },
                                    "start": 297,
                                    "end": 339
                                  }
                                ],
                                "start": 228,
                                "end": 342
                              },
                              "id": null,
                              "generator": false,
                              "start": 222,
                              "end": 342
                            }
                          ],
                          "optional": false,
                          "start": 199,
                          "end": 343
                        },
                        "directive": null,
                        "start": 199,
                        "end": 344
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
                                "start": 359,
                                "end": 362
                              },
                              "typeArguments": null,
                              "attributes": [],
                              "selfClosing": false,
                              "start": 358,
                              "end": 363
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 363,
                                "end": 366
                              },
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
                                    "start": 367,
                                    "end": 372
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "count",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 373,
                                    "end": 378
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 367,
                                  "end": 378
                                },
                                "start": 366,
                                "end": 379
                              },
                              {
                                "type": "JSXText",
                                "value": "\n\t",
                                "raw": "\n\t",
                                "start": 379,
                                "end": 381
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "div",
                                "start": 383,
                                "end": 386
                              },
                              "start": 381,
                              "end": 387
                            },
                            "start": 358,
                            "end": 387
                          },
                          "start": 354,
                          "end": 390
                        },
                        "start": 347,
                        "end": 391
                      }
                    ],
                    "start": 130,
                    "end": 393
                  },
                  "id": null,
                  "generator": false,
                  "start": 124,
                  "end": 393
                }
              ],
              "optional": false,
              "start": 113,
              "end": 394
            },
            "definite": false,
            "start": 105,
            "end": 394
          }
        ],
        "declare": false,
        "start": 99,
        "end": 395
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 92,
      "end": 395
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 395
}
```

</details>

## Output

### Module: test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const Child_component_useBrowserVisibleTask_0IGFPOyJmQA = ()=>{
    const state = _captures[0];
    const timer = setInterval(()=>{
        state.count++;
    }, 1000);
    return ()=>{
        clearInterval(timer);
    };
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
              "name": "Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
              "start": 57,
              "end": 106
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
                          "name": "state",
                          "start": 125,
                          "end": 130
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 133,
                            "end": 142
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 143,
                            "end": 144
                          },
                          "optional": false,
                          "computed": true,
                          "start": 133,
                          "end": 145
                        },
                        "start": 125,
                        "end": 145
                      }
                    ],
                    "start": 119,
                    "end": 146
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "timer",
                          "start": 157,
                          "end": 162
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "setInterval",
                            "start": 165,
                            "end": 176
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
                                    "type": "ExpressionStatement",
                                    "expression": {
                                      "type": "UpdateExpression",
                                      "operator": "++",
                                      "prefix": false,
                                      "argument": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "state",
                                          "start": 191,
                                          "end": 196
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "count",
                                          "start": 197,
                                          "end": 202
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 191,
                                        "end": 202
                                      },
                                      "start": 191,
                                      "end": 204
                                    },
                                    "start": 191,
                                    "end": 205
                                  }
                                ],
                                "start": 181,
                                "end": 211
                              },
                              "id": null,
                              "generator": false,
                              "start": 177,
                              "end": 211
                            },
                            {
                              "type": "Literal",
                              "value": 1000,
                              "raw": "1000",
                              "start": 213,
                              "end": 217
                            }
                          ],
                          "optional": false,
                          "start": 165,
                          "end": 218
                        },
                        "start": 157,
                        "end": 218
                      }
                    ],
                    "start": 151,
                    "end": 219
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "ArrowFunctionExpression",
                      "expression": false,
                      "async": false,
                      "params": [],
                      "body": {
                        "type": "BlockStatement",
                        "body": [
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "name": "clearInterval",
                                "start": 245,
                                "end": 258
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "timer",
                                  "start": 259,
                                  "end": 264
                                }
                              ],
                              "optional": false,
                              "start": 245,
                              "end": 265
                            },
                            "start": 245,
                            "end": 266
                          }
                        ],
                        "start": 235,
                        "end": 272
                      },
                      "id": null,
                      "generator": false,
                      "start": 231,
                      "end": 272
                    },
                    "start": 224,
                    "end": 273
                  }
                ],
                "start": 113,
                "end": 275
              },
              "id": null,
              "generator": false,
              "start": 109,
              "end": 275
            },
            "start": 57,
            "end": 275
          }
        ],
        "start": 51,
        "end": 276
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 276
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 276
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
  "entry": null,
  "displayName": "test.tsx_Child_component_useBrowserVisibleTask",
  "hash": "0IGFPOyJmQA",
  "canonicalFilename": "test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
  "path": "",
  "extension": "js",
  "parent": "Child_component_9GyF01GDKqw",
  "ctxKind": "function",
  "ctxName": "useBrowserVisibleTask$",
  "captures": true,
  "loc": [
    224,
    344
  ],
  "captureNames": [
    "state"
  ]
}
```

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_9GyF01GDKqw = ()=>import("./test.tsx_Child_component_9GyF01GDKqw");
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_9GyF01GDKqw, "Child_component_9GyF01GDKqw"));
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
            "name": "i_9GyF01GDKqw",
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
                "value": "./test.tsx_Child_component_9GyF01GDKqw",
                "raw": "\"./test.tsx_Child_component_9GyF01GDKqw\"",
                "start": 118,
                "end": 158
              },
              "options": null,
              "phase": null,
              "start": 111,
              "end": 159
            },
            "id": null,
            "generator": false,
            "start": 107,
            "end": 159
          },
          "start": 91,
          "end": 159
        }
      ],
      "start": 85,
      "end": 160
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
              "name": "Child",
              "start": 174,
              "end": 179
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 196,
                "end": 208
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 223,
                    "end": 226
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_9GyF01GDKqw",
                      "start": 227,
                      "end": 240
                    },
                    {
                      "type": "Literal",
                      "value": "Child_component_9GyF01GDKqw",
                      "raw": "\"Child_component_9GyF01GDKqw\"",
                      "start": 242,
                      "end": 271
                    }
                  ],
                  "optional": false,
                  "start": 223,
                  "end": 272
                }
              ],
              "optional": false,
              "start": 196,
              "end": 273
            },
            "start": 174,
            "end": 273
          }
        ],
        "start": 168,
        "end": 274
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 161,
      "end": 274
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 274
}
```

</details>

### Module: test.tsx_Child_component_9GyF01GDKqw.js (ENTRY POINT)

```javascript
import { _jsxSorted } from "@qwik.dev/core";
import { _wrapProp } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
import { useBrowserVisibleTaskQrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const i_0IGFPOyJmQA = ()=>import("./test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA");
export const Child_component_9GyF01GDKqw = ()=>{
    const state = useStore({
        count: 0
    });
    // Double count watch
    useBrowserVisibleTaskQrl(/*#__PURE__*/ qrl(i_0IGFPOyJmQA, "Child_component_useBrowserVisibleTask_0IGFPOyJmQA", [
        state
    ]));
    return /*#__PURE__*/ _jsxSorted("div", null, null, _wrapProp(state, "count"), 3, "u6_0");
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
            "name": "useBrowserVisibleTaskQrl",
            "start": 136,
            "end": 160
          },
          "local": {
            "type": "Identifier",
            "name": "useBrowserVisibleTaskQrl",
            "start": 136,
            "end": 160
          },
          "start": 136,
          "end": 160
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 168,
        "end": 184
      },
      "phase": null,
      "attributes": [],
      "start": 127,
      "end": 185
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 195,
            "end": 203
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 195,
            "end": 203
          },
          "start": 195,
          "end": 203
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 211,
        "end": 227
      },
      "phase": null,
      "attributes": [],
      "start": 186,
      "end": 228
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_0IGFPOyJmQA",
            "start": 235,
            "end": 248
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
                "value": "./test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
                "raw": "\"./test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA\"",
                "start": 262,
                "end": 324
              },
              "options": null,
              "phase": null,
              "start": 255,
              "end": 325
            },
            "id": null,
            "generator": false,
            "start": 251,
            "end": 325
          },
          "start": 235,
          "end": 325
        }
      ],
      "start": 229,
      "end": 326
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
              "name": "Child_component_9GyF01GDKqw",
              "start": 340,
              "end": 367
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
                          "name": "state",
                          "start": 386,
                          "end": 391
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 394,
                            "end": 402
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
                                    "start": 413,
                                    "end": 418
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 420,
                                    "end": 421
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 413,
                                  "end": 421
                                }
                              ],
                              "start": 403,
                              "end": 427
                            }
                          ],
                          "optional": false,
                          "start": 394,
                          "end": 428
                        },
                        "start": 386,
                        "end": 428
                      }
                    ],
                    "start": 380,
                    "end": 429
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "useBrowserVisibleTaskQrl",
                        "start": 460,
                        "end": 484
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "qrl",
                            "start": 499,
                            "end": 502
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "i_0IGFPOyJmQA",
                              "start": 503,
                              "end": 516
                            },
                            {
                              "type": "Literal",
                              "value": "Child_component_useBrowserVisibleTask_0IGFPOyJmQA",
                              "raw": "\"Child_component_useBrowserVisibleTask_0IGFPOyJmQA\"",
                              "start": 518,
                              "end": 569
                            },
                            {
                              "type": "ArrayExpression",
                              "elements": [
                                {
                                  "type": "Identifier",
                                  "name": "state",
                                  "start": 581,
                                  "end": 586
                                }
                              ],
                              "start": 571,
                              "end": 592
                            }
                          ],
                          "optional": false,
                          "start": 499,
                          "end": 593
                        }
                      ],
                      "optional": false,
                      "start": 460,
                      "end": 594
                    },
                    "start": 460,
                    "end": 595
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 621,
                        "end": 631
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 632,
                          "end": 637
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 639,
                          "end": 643
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 645,
                          "end": 649
                        },
                        {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "_wrapProp",
                            "start": 651,
                            "end": 660
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 661,
                              "end": 666
                            },
                            {
                              "type": "Literal",
                              "value": "count",
                              "raw": "\"count\"",
                              "start": 668,
                              "end": 675
                            }
                          ],
                          "optional": false,
                          "start": 651,
                          "end": 676
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 678,
                          "end": 679
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 681,
                          "end": 687
                        }
                      ],
                      "optional": false,
                      "start": 621,
                      "end": 688
                    },
                    "start": 600,
                    "end": 689
                  }
                ],
                "start": 374,
                "end": 691
              },
              "id": null,
              "generator": false,
              "start": 370,
              "end": 691
            },
            "start": 340,
            "end": 691
          }
        ],
        "start": 334,
        "end": 692
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 327,
      "end": 692
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 692
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Child_component_9GyF01GDKqw",
  "entry": null,
  "displayName": "test.tsx_Child_component",
  "hash": "9GyF01GDKqw",
  "canonicalFilename": "test.tsx_Child_component_9GyF01GDKqw",
  "path": "",
  "extension": "js",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "component$",
  "captures": false,
  "loc": [
    126,
    395
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`, `useBrowserVisibleTaskQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-04] Signal Helpers**: Reactive signal wrappers: `_wrapProp()`
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (2 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (4 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 2 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `_captures[]` | test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA.js | @qwik.dev/core | 1 |
| `qrl` | test.js | @qwik.dev/core | 1 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `useBrowserVisibleTaskQrl` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `_wrapProp` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |
| `useStore` | test.tsx_Child_component_9GyF01GDKqw.js | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
