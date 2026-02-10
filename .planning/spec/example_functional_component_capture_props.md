# Test: example_functional_component_capture_props

## Test Configuration

| Option | Value |
|--------|-------|
| Transpile TS | True |
| Transpile JSX | True |

## Input

### Source Code

```tsx
import { $, component$, useStore } from '@qwik.dev/core';

export const App = component$(({count, rest: [I2, {I3, v1: [I4], I5=v2, ...I6}, I7=v3, ...I8]}) => {
	const state = useStore({count: 0});
	const {rest: [C2, {C3, v1: [C4], C5=v2, ...C6}, C7=v3, ...C8]} = foo();
	return $(() => {
		return (
			<div onClick$={() => state.count += count + total }>
				{I2}{I3}{I4}{I5}{I6}{I7}{I8}
				{C2}{C3}{C4}{C5}{C6}{C7}{C8}
				{v1}{v2}{v3}
			</div>
		)
	});
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
              "name": "App",
              "optional": false,
              "typeAnnotation": null,
              "start": 72,
              "end": 75
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 78,
                "end": 88
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
                      "type": "ObjectPattern",
                      "decorators": [],
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
                            "start": 91,
                            "end": 96
                          },
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "count",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 91,
                            "end": 96
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 91,
                          "end": 96
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "rest",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 98,
                            "end": 102
                          },
                          "value": {
                            "type": "ArrayPattern",
                            "decorators": [],
                            "elements": [
                              {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "I2",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 105,
                                "end": 107
                              },
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
                                      "name": "I3",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 110,
                                      "end": 112
                                    },
                                    "value": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "I3",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 110,
                                      "end": 112
                                    },
                                    "method": false,
                                    "shorthand": true,
                                    "computed": false,
                                    "optional": false,
                                    "start": 110,
                                    "end": 112
                                  },
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "v1",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 114,
                                      "end": 116
                                    },
                                    "value": {
                                      "type": "ArrayPattern",
                                      "decorators": [],
                                      "elements": [
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "I4",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 119,
                                          "end": 121
                                        }
                                      ],
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 118,
                                      "end": 122
                                    },
                                    "method": false,
                                    "shorthand": false,
                                    "computed": false,
                                    "optional": false,
                                    "start": 114,
                                    "end": 122
                                  },
                                  {
                                    "type": "Property",
                                    "kind": "init",
                                    "key": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "I5",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 124,
                                      "end": 126
                                    },
                                    "value": {
                                      "type": "AssignmentPattern",
                                      "decorators": [],
                                      "left": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "I5",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 124,
                                        "end": 126
                                      },
                                      "right": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "v2",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 127,
                                        "end": 129
                                      },
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 124,
                                      "end": 129
                                    },
                                    "method": false,
                                    "shorthand": true,
                                    "computed": false,
                                    "optional": false,
                                    "start": 124,
                                    "end": 129
                                  },
                                  {
                                    "type": "RestElement",
                                    "decorators": [],
                                    "argument": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "I6",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 134,
                                      "end": 136
                                    },
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "value": null,
                                    "start": 131,
                                    "end": 136
                                  }
                                ],
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 109,
                                "end": 137
                              },
                              {
                                "type": "AssignmentPattern",
                                "decorators": [],
                                "left": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "I7",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 139,
                                  "end": 141
                                },
                                "right": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "v3",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 142,
                                  "end": 144
                                },
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 139,
                                "end": 144
                              },
                              {
                                "type": "RestElement",
                                "decorators": [],
                                "argument": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "I8",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 149,
                                  "end": 151
                                },
                                "optional": false,
                                "typeAnnotation": null,
                                "value": null,
                                "start": 146,
                                "end": 151
                              }
                            ],
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 104,
                            "end": 152
                          },
                          "method": false,
                          "shorthand": false,
                          "computed": false,
                          "optional": false,
                          "start": 98,
                          "end": 152
                        }
                      ],
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 90,
                      "end": 153
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
                              "start": 167,
                              "end": 172
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 175,
                                "end": 183
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
                                        "start": 185,
                                        "end": 190
                                      },
                                      "value": {
                                        "type": "Literal",
                                        "value": 0,
                                        "raw": "0",
                                        "start": 192,
                                        "end": 193
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 185,
                                      "end": 193
                                    }
                                  ],
                                  "start": 184,
                                  "end": 194
                                }
                              ],
                              "optional": false,
                              "start": 175,
                              "end": 195
                            },
                            "definite": false,
                            "start": 167,
                            "end": 195
                          }
                        ],
                        "declare": false,
                        "start": 161,
                        "end": 196
                      },
                      {
                        "type": "VariableDeclaration",
                        "kind": "const",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "ObjectPattern",
                              "decorators": [],
                              "properties": [
                                {
                                  "type": "Property",
                                  "kind": "init",
                                  "key": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "rest",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 205,
                                    "end": 209
                                  },
                                  "value": {
                                    "type": "ArrayPattern",
                                    "decorators": [],
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "C2",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 212,
                                        "end": 214
                                      },
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
                                              "name": "C3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 217,
                                              "end": 219
                                            },
                                            "value": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 217,
                                              "end": 219
                                            },
                                            "method": false,
                                            "shorthand": true,
                                            "computed": false,
                                            "optional": false,
                                            "start": 217,
                                            "end": 219
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "v1",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 221,
                                              "end": 223
                                            },
                                            "value": {
                                              "type": "ArrayPattern",
                                              "decorators": [],
                                              "elements": [
                                                {
                                                  "type": "Identifier",
                                                  "decorators": [],
                                                  "name": "C4",
                                                  "optional": false,
                                                  "typeAnnotation": null,
                                                  "start": 226,
                                                  "end": 228
                                                }
                                              ],
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 225,
                                              "end": 229
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "optional": false,
                                            "start": 221,
                                            "end": 229
                                          },
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C5",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 231,
                                              "end": 233
                                            },
                                            "value": {
                                              "type": "AssignmentPattern",
                                              "decorators": [],
                                              "left": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "C5",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 231,
                                                "end": 233
                                              },
                                              "right": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "v2",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 234,
                                                "end": 236
                                              },
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 231,
                                              "end": 236
                                            },
                                            "method": false,
                                            "shorthand": true,
                                            "computed": false,
                                            "optional": false,
                                            "start": 231,
                                            "end": 236
                                          },
                                          {
                                            "type": "RestElement",
                                            "decorators": [],
                                            "argument": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C6",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 241,
                                              "end": 243
                                            },
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "value": null,
                                            "start": 238,
                                            "end": 243
                                          }
                                        ],
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 216,
                                        "end": 244
                                      },
                                      {
                                        "type": "AssignmentPattern",
                                        "decorators": [],
                                        "left": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "C7",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 246,
                                          "end": 248
                                        },
                                        "right": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "v3",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 249,
                                          "end": 251
                                        },
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 246,
                                        "end": 251
                                      },
                                      {
                                        "type": "RestElement",
                                        "decorators": [],
                                        "argument": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "C8",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 256,
                                          "end": 258
                                        },
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "value": null,
                                        "start": 253,
                                        "end": 258
                                      }
                                    ],
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 211,
                                    "end": 259
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "optional": false,
                                  "start": 205,
                                  "end": 259
                                }
                              ],
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 204,
                              "end": 260
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "foo",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 263,
                                "end": 266
                              },
                              "typeArguments": null,
                              "arguments": [],
                              "optional": false,
                              "start": 263,
                              "end": 268
                            },
                            "definite": false,
                            "start": 204,
                            "end": 268
                          }
                        ],
                        "declare": false,
                        "start": 198,
                        "end": 269
                      },
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
                            "start": 278,
                            "end": 279
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
                                            "start": 303,
                                            "end": 306
                                          },
                                          "typeArguments": null,
                                          "attributes": [
                                            {
                                              "type": "JSXAttribute",
                                              "name": {
                                                "type": "JSXIdentifier",
                                                "name": "onClick$",
                                                "start": 307,
                                                "end": 315
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
                                                        "start": 323,
                                                        "end": 328
                                                      },
                                                      "property": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "count",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 329,
                                                        "end": 334
                                                      },
                                                      "optional": false,
                                                      "computed": false,
                                                      "start": 323,
                                                      "end": 334
                                                    },
                                                    "right": {
                                                      "type": "BinaryExpression",
                                                      "left": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "count",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 338,
                                                        "end": 343
                                                      },
                                                      "operator": "+",
                                                      "right": {
                                                        "type": "Identifier",
                                                        "decorators": [],
                                                        "name": "total",
                                                        "optional": false,
                                                        "typeAnnotation": null,
                                                        "start": 346,
                                                        "end": 351
                                                      },
                                                      "start": 338,
                                                      "end": 351
                                                    },
                                                    "start": 323,
                                                    "end": 351
                                                  },
                                                  "id": null,
                                                  "generator": false,
                                                  "start": 317,
                                                  "end": 351
                                                },
                                                "start": 316,
                                                "end": 353
                                              },
                                              "start": 307,
                                              "end": 353
                                            }
                                          ],
                                          "selfClosing": false,
                                          "start": 302,
                                          "end": 354
                                        },
                                        "children": [
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 354,
                                            "end": 359
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I2",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 360,
                                              "end": 362
                                            },
                                            "start": 359,
                                            "end": 363
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 364,
                                              "end": 366
                                            },
                                            "start": 363,
                                            "end": 367
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I4",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 368,
                                              "end": 370
                                            },
                                            "start": 367,
                                            "end": 371
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I5",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 372,
                                              "end": 374
                                            },
                                            "start": 371,
                                            "end": 375
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I6",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 376,
                                              "end": 378
                                            },
                                            "start": 375,
                                            "end": 379
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I7",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 380,
                                              "end": 382
                                            },
                                            "start": 379,
                                            "end": 383
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "I8",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 384,
                                              "end": 386
                                            },
                                            "start": 383,
                                            "end": 387
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 387,
                                            "end": 392
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C2",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 393,
                                              "end": 395
                                            },
                                            "start": 392,
                                            "end": 396
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 397,
                                              "end": 399
                                            },
                                            "start": 396,
                                            "end": 400
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C4",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 401,
                                              "end": 403
                                            },
                                            "start": 400,
                                            "end": 404
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C5",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 405,
                                              "end": 407
                                            },
                                            "start": 404,
                                            "end": 408
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C6",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 409,
                                              "end": 411
                                            },
                                            "start": 408,
                                            "end": 412
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C7",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 413,
                                              "end": 415
                                            },
                                            "start": 412,
                                            "end": 416
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "C8",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 417,
                                              "end": 419
                                            },
                                            "start": 416,
                                            "end": 420
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t\t",
                                            "raw": "\n\t\t\t\t",
                                            "start": 420,
                                            "end": 425
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "v1",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 426,
                                              "end": 428
                                            },
                                            "start": 425,
                                            "end": 429
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "v2",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 430,
                                              "end": 432
                                            },
                                            "start": 429,
                                            "end": 433
                                          },
                                          {
                                            "type": "JSXExpressionContainer",
                                            "expression": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "v3",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 434,
                                              "end": 436
                                            },
                                            "start": 433,
                                            "end": 437
                                          },
                                          {
                                            "type": "JSXText",
                                            "value": "\n\t\t\t",
                                            "raw": "\n\t\t\t",
                                            "start": 437,
                                            "end": 441
                                          }
                                        ],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "div",
                                            "start": 443,
                                            "end": 446
                                          },
                                          "start": 441,
                                          "end": 447
                                        },
                                        "start": 302,
                                        "end": 447
                                      },
                                      "start": 297,
                                      "end": 451
                                    },
                                    "start": 290,
                                    "end": 451
                                  }
                                ],
                                "start": 286,
                                "end": 454
                              },
                              "id": null,
                              "generator": false,
                              "start": 280,
                              "end": 454
                            }
                          ],
                          "optional": false,
                          "start": 278,
                          "end": 455
                        },
                        "start": 271,
                        "end": 456
                      }
                    ],
                    "start": 158,
                    "end": 458
                  },
                  "id": null,
                  "generator": false,
                  "start": 89,
                  "end": 458
                }
              ],
              "optional": false,
              "start": 78,
              "end": 459
            },
            "definite": false,
            "start": 72,
            "end": 459
          }
        ],
        "declare": false,
        "start": 66,
        "end": 459
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 59,
      "end": 459
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 459
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
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
                      "name": "i_ckEPmXZlub0",
                      "start": 223,
                      "end": 236
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 238,
                      "end": 265
                    }
                  ],
                  "optional": false,
                  "start": 219,
                  "end": 266
                }
              ],
              "optional": false,
              "start": 192,
              "end": 267
            },
            "start": 172,
            "end": 267
          }
        ],
        "start": 166,
        "end": 268
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 159,
      "end": 268
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 268
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { qrl } from "@qwik.dev/core";
import { useStore } from "@qwik.dev/core";
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU");
export const App_component_ckEPmXZlub0 = ({ count, rest: [I2, { I3, v1: [I4], I5 = v2, ...I6 }, I7 = v3, ...I8] })=>{
    const state = useStore({
        count: 0
    });
    const { rest: [C2, { C3, v1: [C4], C5 = v2, ...C6 }, C7 = v3, ...C8] } = foo();
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU", [
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        I2,
        I3,
        I4,
        I5,
        I6,
        I7,
        I8,
        count,
        state
    ]);
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
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "useStore",
            "start": 47,
            "end": 55
          },
          "local": {
            "type": "Identifier",
            "name": "useStore",
            "start": 47,
            "end": 55
          },
          "start": 47,
          "end": 55
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 63,
        "end": 79
      },
      "phase": null,
      "attributes": [],
      "start": 38,
      "end": 80
    },
    {
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "i_w0t0o3QMovU",
            "start": 87,
            "end": 100
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
                "value": "./test.tsx_App_component_1_w0t0o3QMovU",
                "raw": "\"./test.tsx_App_component_1_w0t0o3QMovU\"",
                "start": 114,
                "end": 154
              },
              "options": null,
              "phase": null,
              "start": 107,
              "end": 155
            },
            "id": null,
            "generator": false,
            "start": 103,
            "end": 155
          },
          "start": 87,
          "end": 155
        }
      ],
      "start": 81,
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
              "name": "App_component_ckEPmXZlub0",
              "start": 170,
              "end": 195
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "ObjectPattern",
                  "properties": [
                    {
                      "type": "Property",
                      "kind": "init",
                      "key": {
                        "type": "Identifier",
                        "name": "count",
                        "start": 201,
                        "end": 206
                      },
                      "value": {
                        "type": "Identifier",
                        "name": "count",
                        "start": 201,
                        "end": 206
                      },
                      "method": false,
                      "shorthand": true,
                      "computed": false,
                      "start": 201,
                      "end": 206
                    },
                    {
                      "type": "Property",
                      "kind": "init",
                      "key": {
                        "type": "Identifier",
                        "name": "rest",
                        "start": 208,
                        "end": 212
                      },
                      "value": {
                        "type": "ArrayPattern",
                        "elements": [
                          {
                            "type": "Identifier",
                            "name": "I2",
                            "start": 215,
                            "end": 217
                          },
                          {
                            "type": "ObjectPattern",
                            "properties": [
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "name": "I3",
                                  "start": 221,
                                  "end": 223
                                },
                                "value": {
                                  "type": "Identifier",
                                  "name": "I3",
                                  "start": 221,
                                  "end": 223
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "start": 221,
                                "end": 223
                              },
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "name": "v1",
                                  "start": 225,
                                  "end": 227
                                },
                                "value": {
                                  "type": "ArrayPattern",
                                  "elements": [
                                    {
                                      "type": "Identifier",
                                      "name": "I4",
                                      "start": 230,
                                      "end": 232
                                    }
                                  ],
                                  "start": 229,
                                  "end": 233
                                },
                                "method": false,
                                "shorthand": false,
                                "computed": false,
                                "start": 225,
                                "end": 233
                              },
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "name": "I5",
                                  "start": 235,
                                  "end": 237
                                },
                                "value": {
                                  "type": "AssignmentPattern",
                                  "left": {
                                    "type": "Identifier",
                                    "name": "I5",
                                    "start": 235,
                                    "end": 237
                                  },
                                  "right": {
                                    "type": "Identifier",
                                    "name": "v2",
                                    "start": 240,
                                    "end": 242
                                  },
                                  "start": 235,
                                  "end": 242
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "start": 235,
                                "end": 242
                              },
                              {
                                "type": "RestElement",
                                "argument": {
                                  "type": "Identifier",
                                  "name": "I6",
                                  "start": 247,
                                  "end": 249
                                },
                                "start": 244,
                                "end": 249
                              }
                            ],
                            "start": 219,
                            "end": 251
                          },
                          {
                            "type": "AssignmentPattern",
                            "left": {
                              "type": "Identifier",
                              "name": "I7",
                              "start": 253,
                              "end": 255
                            },
                            "right": {
                              "type": "Identifier",
                              "name": "v3",
                              "start": 258,
                              "end": 260
                            },
                            "start": 253,
                            "end": 260
                          },
                          {
                            "type": "RestElement",
                            "argument": {
                              "type": "Identifier",
                              "name": "I8",
                              "start": 265,
                              "end": 267
                            },
                            "start": 262,
                            "end": 267
                          }
                        ],
                        "start": 214,
                        "end": 268
                      },
                      "method": false,
                      "shorthand": false,
                      "computed": false,
                      "start": 208,
                      "end": 268
                    }
                  ],
                  "start": 199,
                  "end": 270
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
                          "start": 285,
                          "end": 290
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "useStore",
                            "start": 293,
                            "end": 301
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
                                    "start": 312,
                                    "end": 317
                                  },
                                  "value": {
                                    "type": "Literal",
                                    "value": 0,
                                    "raw": "0",
                                    "start": 319,
                                    "end": 320
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "start": 312,
                                  "end": 320
                                }
                              ],
                              "start": 302,
                              "end": 326
                            }
                          ],
                          "optional": false,
                          "start": 293,
                          "end": 327
                        },
                        "start": 285,
                        "end": 327
                      }
                    ],
                    "start": 279,
                    "end": 328
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "const",
                    "declarations": [
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "ObjectPattern",
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "name": "rest",
                                "start": 341,
                                "end": 345
                              },
                              "value": {
                                "type": "ArrayPattern",
                                "elements": [
                                  {
                                    "type": "Identifier",
                                    "name": "C2",
                                    "start": 348,
                                    "end": 350
                                  },
                                  {
                                    "type": "ObjectPattern",
                                    "properties": [
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Identifier",
                                          "name": "C3",
                                          "start": 354,
                                          "end": 356
                                        },
                                        "value": {
                                          "type": "Identifier",
                                          "name": "C3",
                                          "start": 354,
                                          "end": 356
                                        },
                                        "method": false,
                                        "shorthand": true,
                                        "computed": false,
                                        "start": 354,
                                        "end": 356
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Identifier",
                                          "name": "v1",
                                          "start": 358,
                                          "end": 360
                                        },
                                        "value": {
                                          "type": "ArrayPattern",
                                          "elements": [
                                            {
                                              "type": "Identifier",
                                              "name": "C4",
                                              "start": 363,
                                              "end": 365
                                            }
                                          ],
                                          "start": 362,
                                          "end": 366
                                        },
                                        "method": false,
                                        "shorthand": false,
                                        "computed": false,
                                        "start": 358,
                                        "end": 366
                                      },
                                      {
                                        "type": "Property",
                                        "kind": "init",
                                        "key": {
                                          "type": "Identifier",
                                          "name": "C5",
                                          "start": 368,
                                          "end": 370
                                        },
                                        "value": {
                                          "type": "AssignmentPattern",
                                          "left": {
                                            "type": "Identifier",
                                            "name": "C5",
                                            "start": 368,
                                            "end": 370
                                          },
                                          "right": {
                                            "type": "Identifier",
                                            "name": "v2",
                                            "start": 373,
                                            "end": 375
                                          },
                                          "start": 368,
                                          "end": 375
                                        },
                                        "method": false,
                                        "shorthand": true,
                                        "computed": false,
                                        "start": 368,
                                        "end": 375
                                      },
                                      {
                                        "type": "RestElement",
                                        "argument": {
                                          "type": "Identifier",
                                          "name": "C6",
                                          "start": 380,
                                          "end": 382
                                        },
                                        "start": 377,
                                        "end": 382
                                      }
                                    ],
                                    "start": 352,
                                    "end": 384
                                  },
                                  {
                                    "type": "AssignmentPattern",
                                    "left": {
                                      "type": "Identifier",
                                      "name": "C7",
                                      "start": 386,
                                      "end": 388
                                    },
                                    "right": {
                                      "type": "Identifier",
                                      "name": "v3",
                                      "start": 391,
                                      "end": 393
                                    },
                                    "start": 386,
                                    "end": 393
                                  },
                                  {
                                    "type": "RestElement",
                                    "argument": {
                                      "type": "Identifier",
                                      "name": "C8",
                                      "start": 398,
                                      "end": 400
                                    },
                                    "start": 395,
                                    "end": 400
                                  }
                                ],
                                "start": 347,
                                "end": 401
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 341,
                              "end": 401
                            }
                          ],
                          "start": 339,
                          "end": 403
                        },
                        "init": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "Identifier",
                            "name": "foo",
                            "start": 406,
                            "end": 409
                          },
                          "arguments": [],
                          "optional": false,
                          "start": 406,
                          "end": 411
                        },
                        "start": 339,
                        "end": 411
                      }
                    ],
                    "start": 333,
                    "end": 412
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "qrl",
                        "start": 438,
                        "end": 441
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_w0t0o3QMovU",
                          "start": 442,
                          "end": 455
                        },
                        {
                          "type": "Literal",
                          "value": "App_component_1_w0t0o3QMovU",
                          "raw": "\"App_component_1_w0t0o3QMovU\"",
                          "start": 457,
                          "end": 486
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "C2",
                              "start": 498,
                              "end": 500
                            },
                            {
                              "type": "Identifier",
                              "name": "C3",
                              "start": 510,
                              "end": 512
                            },
                            {
                              "type": "Identifier",
                              "name": "C4",
                              "start": 522,
                              "end": 524
                            },
                            {
                              "type": "Identifier",
                              "name": "C5",
                              "start": 534,
                              "end": 536
                            },
                            {
                              "type": "Identifier",
                              "name": "C6",
                              "start": 546,
                              "end": 548
                            },
                            {
                              "type": "Identifier",
                              "name": "C7",
                              "start": 558,
                              "end": 560
                            },
                            {
                              "type": "Identifier",
                              "name": "C8",
                              "start": 570,
                              "end": 572
                            },
                            {
                              "type": "Identifier",
                              "name": "I2",
                              "start": 582,
                              "end": 584
                            },
                            {
                              "type": "Identifier",
                              "name": "I3",
                              "start": 594,
                              "end": 596
                            },
                            {
                              "type": "Identifier",
                              "name": "I4",
                              "start": 606,
                              "end": 608
                            },
                            {
                              "type": "Identifier",
                              "name": "I5",
                              "start": 618,
                              "end": 620
                            },
                            {
                              "type": "Identifier",
                              "name": "I6",
                              "start": 630,
                              "end": 632
                            },
                            {
                              "type": "Identifier",
                              "name": "I7",
                              "start": 642,
                              "end": 644
                            },
                            {
                              "type": "Identifier",
                              "name": "I8",
                              "start": 654,
                              "end": 656
                            },
                            {
                              "type": "Identifier",
                              "name": "count",
                              "start": 666,
                              "end": 671
                            },
                            {
                              "type": "Identifier",
                              "name": "state",
                              "start": 681,
                              "end": 686
                            }
                          ],
                          "start": 488,
                          "end": 692
                        }
                      ],
                      "optional": false,
                      "start": 438,
                      "end": 693
                    },
                    "start": 417,
                    "end": 694
                  }
                ],
                "start": 273,
                "end": 696
              },
              "id": null,
              "generator": false,
              "start": 198,
              "end": 696
            },
            "start": 170,
            "end": 696
          }
        ],
        "start": 164,
        "end": 697
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 157,
      "end": 697
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 697
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
    91,
    460
  ],
  "paramNames": [
    "{count, rest: [I2, {I3, v1: [I4], I5}, ...I8]}"
  ]
}
```

### Module: test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
export const App_component_div_q_e_click_mi4E1piTWe8 = ()=>{
    const count = _captures[0], state = _captures[1];
    return state.count += count + total;
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
                          "name": "count",
                          "start": 115,
                          "end": 120
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 123,
                            "end": 132
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 133,
                            "end": 134
                          },
                          "optional": false,
                          "computed": true,
                          "start": 123,
                          "end": 135
                        },
                        "start": 115,
                        "end": 135
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 137,
                          "end": 142
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 145,
                            "end": 154
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 155,
                            "end": 156
                          },
                          "optional": false,
                          "computed": true,
                          "start": 145,
                          "end": 157
                        },
                        "start": 137,
                        "end": 157
                      }
                    ],
                    "start": 109,
                    "end": 158
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
                          "start": 170,
                          "end": 175
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 176,
                          "end": 181
                        },
                        "optional": false,
                        "computed": false,
                        "start": 170,
                        "end": 181
                      },
                      "right": {
                        "type": "BinaryExpression",
                        "left": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 185,
                          "end": 190
                        },
                        "operator": "+",
                        "right": {
                          "type": "Identifier",
                          "name": "total",
                          "start": 193,
                          "end": 198
                        },
                        "start": 185,
                        "end": 198
                      },
                      "start": 170,
                      "end": 198
                    },
                    "start": 163,
                    "end": 199
                  }
                ],
                "start": 103,
                "end": 201
              },
              "id": null,
              "generator": false,
              "start": 99,
              "end": 201
            },
            "start": 57,
            "end": 201
          }
        ],
        "start": 51,
        "end": 202
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 44,
      "end": 202
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 202
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
  "parent": "App_component_1_w0t0o3QMovU",
  "ctxKind": "eventHandler",
  "ctxName": "onClick$",
  "captures": true,
  "loc": [
    319,
    353
  ],
  "captureNames": [
    "count",
    "state"
  ]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
import { _captures } from "@qwik.dev/core";
import { _jsxSorted } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_mi4E1piTWe8 = ()=>import("./test.tsx_App_component_div_q_e_click_mi4E1piTWe8");
export const App_component_1_w0t0o3QMovU = ()=>{
    const C2 = _captures[0], C3 = _captures[1], C4 = _captures[2], C5 = _captures[3], C6 = _captures[4], C7 = _captures[5], C8 = _captures[6], I2 = _captures[7], I3 = _captures[8], I4 = _captures[9], I5 = _captures[10], I6 = _captures[11], I7 = _captures[12], I8 = _captures[13], count = _captures[14], state = _captures[15];
    return /*#__PURE__*/ _jsxSorted("div", {
        "q-e:click": /*#__PURE__*/ qrl(i_mi4E1piTWe8, "App_component_div_q_e_click_mi4E1piTWe8", [
            count,
            state
        ])
    }, null, [
        I2,
        I3,
        I4,
        I5,
        I6,
        I7,
        I8,
        C2,
        C3,
        C4,
        C5,
        C6,
        C7,
        C8,
        v1,
        v2,
        v3
    ], 0, "u6_0");
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
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 53,
            "end": 63
          },
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
            "name": "i_mi4E1piTWe8",
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
                "value": "./test.tsx_App_component_div_q_e_click_mi4E1piTWe8",
                "raw": "\"./test.tsx_App_component_div_q_e_click_mi4E1piTWe8\"",
                "start": 160,
                "end": 212
              },
              "options": null,
              "phase": null,
              "start": 153,
              "end": 213
            },
            "id": null,
            "generator": false,
            "start": 149,
            "end": 213
          },
          "start": 133,
          "end": 213
        }
      ],
      "start": 127,
      "end": 214
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
              "name": "App_component_1_w0t0o3QMovU",
              "start": 228,
              "end": 255
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
                          "name": "C2",
                          "start": 274,
                          "end": 276
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 279,
                            "end": 288
                          },
                          "property": {
                            "type": "Literal",
                            "value": 0,
                            "raw": "0",
                            "start": 289,
                            "end": 290
                          },
                          "optional": false,
                          "computed": true,
                          "start": 279,
                          "end": 291
                        },
                        "start": 274,
                        "end": 291
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C3",
                          "start": 293,
                          "end": 295
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 298,
                            "end": 307
                          },
                          "property": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 308,
                            "end": 309
                          },
                          "optional": false,
                          "computed": true,
                          "start": 298,
                          "end": 310
                        },
                        "start": 293,
                        "end": 310
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C4",
                          "start": 312,
                          "end": 314
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 317,
                            "end": 326
                          },
                          "property": {
                            "type": "Literal",
                            "value": 2,
                            "raw": "2",
                            "start": 327,
                            "end": 328
                          },
                          "optional": false,
                          "computed": true,
                          "start": 317,
                          "end": 329
                        },
                        "start": 312,
                        "end": 329
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C5",
                          "start": 331,
                          "end": 333
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 336,
                            "end": 345
                          },
                          "property": {
                            "type": "Literal",
                            "value": 3,
                            "raw": "3",
                            "start": 346,
                            "end": 347
                          },
                          "optional": false,
                          "computed": true,
                          "start": 336,
                          "end": 348
                        },
                        "start": 331,
                        "end": 348
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C6",
                          "start": 350,
                          "end": 352
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 355,
                            "end": 364
                          },
                          "property": {
                            "type": "Literal",
                            "value": 4,
                            "raw": "4",
                            "start": 365,
                            "end": 366
                          },
                          "optional": false,
                          "computed": true,
                          "start": 355,
                          "end": 367
                        },
                        "start": 350,
                        "end": 367
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C7",
                          "start": 369,
                          "end": 371
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 374,
                            "end": 383
                          },
                          "property": {
                            "type": "Literal",
                            "value": 5,
                            "raw": "5",
                            "start": 384,
                            "end": 385
                          },
                          "optional": false,
                          "computed": true,
                          "start": 374,
                          "end": 386
                        },
                        "start": 369,
                        "end": 386
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "C8",
                          "start": 388,
                          "end": 390
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 393,
                            "end": 402
                          },
                          "property": {
                            "type": "Literal",
                            "value": 6,
                            "raw": "6",
                            "start": 403,
                            "end": 404
                          },
                          "optional": false,
                          "computed": true,
                          "start": 393,
                          "end": 405
                        },
                        "start": 388,
                        "end": 405
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I2",
                          "start": 407,
                          "end": 409
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 412,
                            "end": 421
                          },
                          "property": {
                            "type": "Literal",
                            "value": 7,
                            "raw": "7",
                            "start": 422,
                            "end": 423
                          },
                          "optional": false,
                          "computed": true,
                          "start": 412,
                          "end": 424
                        },
                        "start": 407,
                        "end": 424
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I3",
                          "start": 426,
                          "end": 428
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 431,
                            "end": 440
                          },
                          "property": {
                            "type": "Literal",
                            "value": 8,
                            "raw": "8",
                            "start": 441,
                            "end": 442
                          },
                          "optional": false,
                          "computed": true,
                          "start": 431,
                          "end": 443
                        },
                        "start": 426,
                        "end": 443
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I4",
                          "start": 445,
                          "end": 447
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 450,
                            "end": 459
                          },
                          "property": {
                            "type": "Literal",
                            "value": 9,
                            "raw": "9",
                            "start": 460,
                            "end": 461
                          },
                          "optional": false,
                          "computed": true,
                          "start": 450,
                          "end": 462
                        },
                        "start": 445,
                        "end": 462
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I5",
                          "start": 464,
                          "end": 466
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 469,
                            "end": 478
                          },
                          "property": {
                            "type": "Literal",
                            "value": 10,
                            "raw": "10",
                            "start": 479,
                            "end": 481
                          },
                          "optional": false,
                          "computed": true,
                          "start": 469,
                          "end": 482
                        },
                        "start": 464,
                        "end": 482
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I6",
                          "start": 484,
                          "end": 486
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 489,
                            "end": 498
                          },
                          "property": {
                            "type": "Literal",
                            "value": 11,
                            "raw": "11",
                            "start": 499,
                            "end": 501
                          },
                          "optional": false,
                          "computed": true,
                          "start": 489,
                          "end": 502
                        },
                        "start": 484,
                        "end": 502
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I7",
                          "start": 504,
                          "end": 506
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 509,
                            "end": 518
                          },
                          "property": {
                            "type": "Literal",
                            "value": 12,
                            "raw": "12",
                            "start": 519,
                            "end": 521
                          },
                          "optional": false,
                          "computed": true,
                          "start": 509,
                          "end": 522
                        },
                        "start": 504,
                        "end": 522
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "I8",
                          "start": 524,
                          "end": 526
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 529,
                            "end": 538
                          },
                          "property": {
                            "type": "Literal",
                            "value": 13,
                            "raw": "13",
                            "start": 539,
                            "end": 541
                          },
                          "optional": false,
                          "computed": true,
                          "start": 529,
                          "end": 542
                        },
                        "start": 524,
                        "end": 542
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "count",
                          "start": 544,
                          "end": 549
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 552,
                            "end": 561
                          },
                          "property": {
                            "type": "Literal",
                            "value": 14,
                            "raw": "14",
                            "start": 562,
                            "end": 564
                          },
                          "optional": false,
                          "computed": true,
                          "start": 552,
                          "end": 565
                        },
                        "start": 544,
                        "end": 565
                      },
                      {
                        "type": "VariableDeclarator",
                        "id": {
                          "type": "Identifier",
                          "name": "state",
                          "start": 567,
                          "end": 572
                        },
                        "init": {
                          "type": "MemberExpression",
                          "object": {
                            "type": "Identifier",
                            "name": "_captures",
                            "start": 575,
                            "end": 584
                          },
                          "property": {
                            "type": "Literal",
                            "value": 15,
                            "raw": "15",
                            "start": 585,
                            "end": 587
                          },
                          "optional": false,
                          "computed": true,
                          "start": 575,
                          "end": 588
                        },
                        "start": 567,
                        "end": 588
                      }
                    ],
                    "start": 268,
                    "end": 589
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "_jsxSorted",
                        "start": 615,
                        "end": 625
                      },
                      "arguments": [
                        {
                          "type": "Literal",
                          "value": "div",
                          "raw": "\"div\"",
                          "start": 626,
                          "end": 631
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
                                "start": 643,
                                "end": 654
                              },
                              "value": {
                                "type": "CallExpression",
                                "callee": {
                                  "type": "Identifier",
                                  "name": "qrl",
                                  "start": 670,
                                  "end": 673
                                },
                                "arguments": [
                                  {
                                    "type": "Identifier",
                                    "name": "i_mi4E1piTWe8",
                                    "start": 674,
                                    "end": 687
                                  },
                                  {
                                    "type": "Literal",
                                    "value": "App_component_div_q_e_click_mi4E1piTWe8",
                                    "raw": "\"App_component_div_q_e_click_mi4E1piTWe8\"",
                                    "start": 689,
                                    "end": 730
                                  },
                                  {
                                    "type": "ArrayExpression",
                                    "elements": [
                                      {
                                        "type": "Identifier",
                                        "name": "count",
                                        "start": 746,
                                        "end": 751
                                      },
                                      {
                                        "type": "Identifier",
                                        "name": "state",
                                        "start": 765,
                                        "end": 770
                                      }
                                    ],
                                    "start": 732,
                                    "end": 780
                                  }
                                ],
                                "optional": false,
                                "start": 670,
                                "end": 781
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "start": 643,
                              "end": 781
                            }
                          ],
                          "start": 633,
                          "end": 787
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 789,
                          "end": 793
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "name": "I2",
                              "start": 805,
                              "end": 807
                            },
                            {
                              "type": "Identifier",
                              "name": "I3",
                              "start": 817,
                              "end": 819
                            },
                            {
                              "type": "Identifier",
                              "name": "I4",
                              "start": 829,
                              "end": 831
                            },
                            {
                              "type": "Identifier",
                              "name": "I5",
                              "start": 841,
                              "end": 843
                            },
                            {
                              "type": "Identifier",
                              "name": "I6",
                              "start": 853,
                              "end": 855
                            },
                            {
                              "type": "Identifier",
                              "name": "I7",
                              "start": 865,
                              "end": 867
                            },
                            {
                              "type": "Identifier",
                              "name": "I8",
                              "start": 877,
                              "end": 879
                            },
                            {
                              "type": "Identifier",
                              "name": "C2",
                              "start": 889,
                              "end": 891
                            },
                            {
                              "type": "Identifier",
                              "name": "C3",
                              "start": 901,
                              "end": 903
                            },
                            {
                              "type": "Identifier",
                              "name": "C4",
                              "start": 913,
                              "end": 915
                            },
                            {
                              "type": "Identifier",
                              "name": "C5",
                              "start": 925,
                              "end": 927
                            },
                            {
                              "type": "Identifier",
                              "name": "C6",
                              "start": 937,
                              "end": 939
                            },
                            {
                              "type": "Identifier",
                              "name": "C7",
                              "start": 949,
                              "end": 951
                            },
                            {
                              "type": "Identifier",
                              "name": "C8",
                              "start": 961,
                              "end": 963
                            },
                            {
                              "type": "Identifier",
                              "name": "v1",
                              "start": 973,
                              "end": 975
                            },
                            {
                              "type": "Identifier",
                              "name": "v2",
                              "start": 985,
                              "end": 987
                            },
                            {
                              "type": "Identifier",
                              "name": "v3",
                              "start": 997,
                              "end": 999
                            }
                          ],
                          "start": 795,
                          "end": 1005
                        },
                        {
                          "type": "Literal",
                          "value": 0,
                          "raw": "0",
                          "start": 1007,
                          "end": 1008
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 1010,
                          "end": 1016
                        }
                      ],
                      "optional": false,
                      "start": 615,
                      "end": 1017
                    },
                    "start": 594,
                    "end": 1018
                  }
                ],
                "start": 262,
                "end": 1020
              },
              "id": null,
              "generator": false,
              "start": 258,
              "end": 1020
            },
            "start": 228,
            "end": 1020
          }
        ],
        "start": 222,
        "end": 1021
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 215,
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

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "App_component_1_w0t0o3QMovU",
  "entry": null,
  "displayName": "test.tsx_App_component_1",
  "hash": "w0t0o3QMovU",
  "canonicalFilename": "test.tsx_App_component_1_w0t0o3QMovU",
  "path": "",
  "extension": "js",
  "parent": "App_component_ckEPmXZlub0",
  "ctxKind": "function",
  "ctxName": "$",
  "captures": true,
  "loc": [
    282,
    456
  ],
  "captureNames": [
    "C2",
    "C3",
    "C4",
    "C5",
    "C6",
    "C7",
    "C8",
    "I2",
    "I3",
    "I4",
    "I5",
    "I6",
    "I7",
    "I8",
    "count",
    "state"
  ]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-02] Dollar-to-QRL Suffix**: `$`-suffixed APIs transformed to Qrl variants: `componentQrl`
- **[CONV-03] JSX Transforms**: JSX elements compiled to `_jsxSorted()` function calls
- **[CONV-05] Capture Patterns**: Captured variables accessed via `_captures[]` array in extracted segments
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (3 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (5 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 3 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | test.js | @qwik.dev/core | 1 |
| `componentQrl` | test.js | @qwik.dev/core | 1 |
| `qrl` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| `useStore` | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js | @qwik.dev/core | 2 |
| `qrl` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |
| `_jsxSorted` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |
| `_captures[]` | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 16 |

## Diagnostics

```json
[]
```
