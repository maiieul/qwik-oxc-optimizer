# Test: example_optimization_issue_3561

## Test Configuration

**Note:** Optimization fix for issue 3561 -- destructured nested object properties from useStore in inline mode.

| Option | Value |
|--------|-------|
| Entry Strategy | Inline |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | false |
| Is Server | false |

## Input

### Source Code

```tsx
import { component$ } from '@qwik.dev/core';

export const Issue3561 = component$(() => {
	const props = useStore({
		product: {
		currentVariant: {
			variantImage: 'image',
			variantNumber: 'number',
			setContents: 'contents',
		},
		},
	});
	const {
		currentVariant: { variantImage, variantNumber, setContents } = {},
	} = props.product;

	console.log(variantImage, variantNumber, setContents)

	return <p></p>;
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
              "name": "Issue3561",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
              "end": 68
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 71,
                "end": 81
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
                              "name": "props",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 97,
                              "end": 102
                            },
                            "init": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "useStore",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 105,
                                "end": 113
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
                                        "name": "product",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 118,
                                        "end": 125
                                      },
                                      "value": {
                                        "type": "ObjectExpression",
                                        "properties": [
                                          {
                                            "type": "Property",
                                            "kind": "init",
                                            "key": {
                                              "type": "Identifier",
                                              "decorators": [],
                                              "name": "currentVariant",
                                              "optional": false,
                                              "typeAnnotation": null,
                                              "start": 131,
                                              "end": 145
                                            },
                                            "value": {
                                              "type": "ObjectExpression",
                                              "properties": [
                                                {
                                                  "type": "Property",
                                                  "kind": "init",
                                                  "key": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "variantImage",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 152,
                                                    "end": 164
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "image",
                                                    "raw": "'image'",
                                                    "start": 166,
                                                    "end": 173
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "optional": false,
                                                  "start": 152,
                                                  "end": 173
                                                },
                                                {
                                                  "type": "Property",
                                                  "kind": "init",
                                                  "key": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "variantNumber",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 178,
                                                    "end": 191
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "number",
                                                    "raw": "'number'",
                                                    "start": 193,
                                                    "end": 201
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "optional": false,
                                                  "start": 178,
                                                  "end": 201
                                                },
                                                {
                                                  "type": "Property",
                                                  "kind": "init",
                                                  "key": {
                                                    "type": "Identifier",
                                                    "decorators": [],
                                                    "name": "setContents",
                                                    "optional": false,
                                                    "typeAnnotation": null,
                                                    "start": 206,
                                                    "end": 217
                                                  },
                                                  "value": {
                                                    "type": "Literal",
                                                    "value": "contents",
                                                    "raw": "'contents'",
                                                    "start": 219,
                                                    "end": 229
                                                  },
                                                  "method": false,
                                                  "shorthand": false,
                                                  "computed": false,
                                                  "optional": false,
                                                  "start": 206,
                                                  "end": 229
                                                }
                                              ],
                                              "start": 147,
                                              "end": 234
                                            },
                                            "method": false,
                                            "shorthand": false,
                                            "computed": false,
                                            "optional": false,
                                            "start": 131,
                                            "end": 234
                                          }
                                        ],
                                        "start": 127,
                                        "end": 239
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "optional": false,
                                      "start": 118,
                                      "end": 239
                                    }
                                  ],
                                  "start": 114,
                                  "end": 243
                                }
                              ],
                              "optional": false,
                              "start": 105,
                              "end": 244
                            },
                            "definite": false,
                            "start": 97,
                            "end": 244
                          }
                        ],
                        "declare": false,
                        "start": 91,
                        "end": 245
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
                                    "name": "currentVariant",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 257,
                                    "end": 271
                                  },
                                  "value": {
                                    "type": "AssignmentPattern",
                                    "decorators": [],
                                    "left": {
                                      "type": "ObjectPattern",
                                      "decorators": [],
                                      "properties": [
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "variantImage",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 275,
                                            "end": 287
                                          },
                                          "value": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "variantImage",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 275,
                                            "end": 287
                                          },
                                          "method": false,
                                          "shorthand": true,
                                          "computed": false,
                                          "optional": false,
                                          "start": 275,
                                          "end": 287
                                        },
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "variantNumber",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 289,
                                            "end": 302
                                          },
                                          "value": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "variantNumber",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 289,
                                            "end": 302
                                          },
                                          "method": false,
                                          "shorthand": true,
                                          "computed": false,
                                          "optional": false,
                                          "start": 289,
                                          "end": 302
                                        },
                                        {
                                          "type": "Property",
                                          "kind": "init",
                                          "key": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "setContents",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 304,
                                            "end": 315
                                          },
                                          "value": {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "setContents",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 304,
                                            "end": 315
                                          },
                                          "method": false,
                                          "shorthand": true,
                                          "computed": false,
                                          "optional": false,
                                          "start": 304,
                                          "end": 315
                                        }
                                      ],
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 273,
                                      "end": 317
                                    },
                                    "right": {
                                      "type": "ObjectExpression",
                                      "properties": [],
                                      "start": 320,
                                      "end": 322
                                    },
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 273,
                                    "end": 322
                                  },
                                  "method": false,
                                  "shorthand": false,
                                  "computed": false,
                                  "optional": false,
                                  "start": 257,
                                  "end": 322
                                }
                              ],
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 253,
                              "end": 326
                            },
                            "init": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "props",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 329,
                                "end": 334
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "product",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 335,
                                "end": 342
                              },
                              "optional": false,
                              "computed": false,
                              "start": 329,
                              "end": 342
                            },
                            "definite": false,
                            "start": 253,
                            "end": 342
                          }
                        ],
                        "declare": false,
                        "start": 247,
                        "end": 343
                      },
                      {
                        "type": "ExpressionStatement",
                        "expression": {
                          "type": "CallExpression",
                          "callee": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "console",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 346,
                              "end": 353
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 354,
                              "end": 357
                            },
                            "optional": false,
                            "computed": false,
                            "start": 346,
                            "end": 357
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "variantImage",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 358,
                              "end": 370
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "variantNumber",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 372,
                              "end": 385
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "setContents",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 387,
                              "end": 398
                            }
                          ],
                          "optional": false,
                          "start": 346,
                          "end": 399
                        },
                        "directive": null,
                        "start": 346,
                        "end": 399
                      },
                      {
                        "type": "ReturnStatement",
                        "argument": {
                          "type": "JSXElement",
                          "openingElement": {
                            "type": "JSXOpeningElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "p",
                              "start": 410,
                              "end": 411
                            },
                            "typeArguments": null,
                            "attributes": [],
                            "selfClosing": false,
                            "start": 409,
                            "end": 412
                          },
                          "children": [],
                          "closingElement": {
                            "type": "JSXClosingElement",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "p",
                              "start": 414,
                              "end": 415
                            },
                            "start": 412,
                            "end": 416
                          },
                          "start": 409,
                          "end": 416
                        },
                        "start": 402,
                        "end": 417
                      }
                    ],
                    "start": 88,
                    "end": 420
                  },
                  "id": null,
                  "generator": false,
                  "start": 82,
                  "end": 420
                }
              ],
              "optional": false,
              "start": 71,
              "end": 421
            },
            "definite": false,
            "start": 59,
            "end": 421
          }
        ],
        "declare": false,
        "start": 53,
        "end": 422
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 422
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 422
}

```

</details>

## Output

### Module: test.jsx

```jsx
import { componentQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const Issue3561 = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    const props = useStore({
        product: {
            currentVariant: {
                variantImage: 'image',
                variantNumber: 'number',
                setContents: 'contents'
            }
        }
    });
    const { currentVariant: { variantImage, variantNumber, setContents } = {} } = props.product;
    console.log(variantImage, variantNumber, setContents);
    return <p></p>;
}, "Issue3561_component_hHTw654BZB8"));
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
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 56,
            "end": 66
          },
          "start": 56,
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
      "start": 47,
      "end": 91
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
              "name": "Issue3561",
              "start": 105,
              "end": 114
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 131,
                "end": 143
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 158,
                    "end": 168
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
                                  "name": "props",
                                  "start": 185,
                                  "end": 190
                                },
                                "init": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "Identifier",
                                    "name": "useStore",
                                    "start": 193,
                                    "end": 201
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
                                            "name": "product",
                                            "start": 212,
                                            "end": 219
                                          },
                                          "value": {
                                            "type": "ObjectExpression",
                                            "properties": [
                                              {
                                                "type": "Property",
                                                "kind": "init",
                                                "key": {
                                                  "type": "Identifier",
                                                  "name": "currentVariant",
                                                  "start": 235,
                                                  "end": 249
                                                },
                                                "value": {
                                                  "type": "ObjectExpression",
                                                  "properties": [
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Identifier",
                                                        "name": "variantImage",
                                                        "start": 269,
                                                        "end": 281
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "image",
                                                        "raw": "'image'",
                                                        "start": 283,
                                                        "end": 290
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "start": 269,
                                                      "end": 290
                                                    },
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Identifier",
                                                        "name": "variantNumber",
                                                        "start": 308,
                                                        "end": 321
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "number",
                                                        "raw": "'number'",
                                                        "start": 323,
                                                        "end": 331
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "start": 308,
                                                      "end": 331
                                                    },
                                                    {
                                                      "type": "Property",
                                                      "kind": "init",
                                                      "key": {
                                                        "type": "Identifier",
                                                        "name": "setContents",
                                                        "start": 349,
                                                        "end": 360
                                                      },
                                                      "value": {
                                                        "type": "Literal",
                                                        "value": "contents",
                                                        "raw": "'contents'",
                                                        "start": 362,
                                                        "end": 372
                                                      },
                                                      "method": false,
                                                      "shorthand": false,
                                                      "computed": false,
                                                      "start": 349,
                                                      "end": 372
                                                    }
                                                  ],
                                                  "start": 251,
                                                  "end": 386
                                                },
                                                "method": false,
                                                "shorthand": false,
                                                "computed": false,
                                                "start": 235,
                                                "end": 386
                                              }
                                            ],
                                            "start": 221,
                                            "end": 396
                                          },
                                          "method": false,
                                          "shorthand": false,
                                          "computed": false,
                                          "start": 212,
                                          "end": 396
                                        }
                                      ],
                                      "start": 202,
                                      "end": 402
                                    }
                                  ],
                                  "optional": false,
                                  "start": 193,
                                  "end": 403
                                },
                                "start": 185,
                                "end": 403
                              }
                            ],
                            "start": 179,
                            "end": 404
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
                                        "name": "currentVariant",
                                        "start": 417,
                                        "end": 431
                                      },
                                      "value": {
                                        "type": "AssignmentPattern",
                                        "left": {
                                          "type": "ObjectPattern",
                                          "properties": [
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "variantImage",
                                                "start": 435,
                                                "end": 447
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "variantImage",
                                                "start": 435,
                                                "end": 447
                                              },
                                              "method": false,
                                              "shorthand": true,
                                              "computed": false,
                                              "start": 435,
                                              "end": 447
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "variantNumber",
                                                "start": 449,
                                                "end": 462
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "variantNumber",
                                                "start": 449,
                                                "end": 462
                                              },
                                              "method": false,
                                              "shorthand": true,
                                              "computed": false,
                                              "start": 449,
                                              "end": 462
                                            },
                                            {
                                              "type": "Property",
                                              "kind": "init",
                                              "key": {
                                                "type": "Identifier",
                                                "name": "setContents",
                                                "start": 464,
                                                "end": 475
                                              },
                                              "value": {
                                                "type": "Identifier",
                                                "name": "setContents",
                                                "start": 464,
                                                "end": 475
                                              },
                                              "method": false,
                                              "shorthand": true,
                                              "computed": false,
                                              "start": 464,
                                              "end": 475
                                            }
                                          ],
                                          "start": 433,
                                          "end": 477
                                        },
                                        "right": {
                                          "type": "ObjectExpression",
                                          "properties": [],
                                          "start": 480,
                                          "end": 482
                                        },
                                        "start": 433,
                                        "end": 482
                                      },
                                      "method": false,
                                      "shorthand": false,
                                      "computed": false,
                                      "start": 417,
                                      "end": 482
                                    }
                                  ],
                                  "start": 415,
                                  "end": 484
                                },
                                "init": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "props",
                                    "start": 487,
                                    "end": 492
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "product",
                                    "start": 493,
                                    "end": 500
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 487,
                                  "end": 500
                                },
                                "start": 415,
                                "end": 500
                              }
                            ],
                            "start": 409,
                            "end": 501
                          },
                          {
                            "type": "ExpressionStatement",
                            "expression": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "name": "console",
                                  "start": 506,
                                  "end": 513
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "log",
                                  "start": 514,
                                  "end": 517
                                },
                                "optional": false,
                                "computed": false,
                                "start": 506,
                                "end": 517
                              },
                              "arguments": [
                                {
                                  "type": "Identifier",
                                  "name": "variantImage",
                                  "start": 518,
                                  "end": 530
                                },
                                {
                                  "type": "Identifier",
                                  "name": "variantNumber",
                                  "start": 532,
                                  "end": 545
                                },
                                {
                                  "type": "Identifier",
                                  "name": "setContents",
                                  "start": 547,
                                  "end": 558
                                }
                              ],
                              "optional": false,
                              "start": 506,
                              "end": 559
                            },
                            "start": 506,
                            "end": 560
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "p",
                                  "start": 573,
                                  "end": 574
                                },
                                "attributes": [],
                                "selfClosing": false,
                                "start": 572,
                                "end": 575
                              },
                              "children": [],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "p",
                                  "start": 577,
                                  "end": 578
                                },
                                "start": 575,
                                "end": 579
                              },
                              "start": 572,
                              "end": 579
                            },
                            "start": 565,
                            "end": 580
                          }
                        ],
                        "start": 173,
                        "end": 582
                      },
                      "id": null,
                      "generator": false,
                      "start": 169,
                      "end": 582
                    },
                    {
                      "type": "Literal",
                      "value": "Issue3561_component_hHTw654BZB8",
                      "raw": "\"Issue3561_component_hHTw654BZB8\"",
                      "start": 584,
                      "end": 617
                    }
                  ],
                  "optional": false,
                  "start": 158,
                  "end": 618
                }
              ],
              "optional": false,
              "start": 131,
              "end": 619
            },
            "start": 105,
            "end": 619
          }
        ],
        "start": 99,
        "end": 620
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 92,
      "end": 620
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 620
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-07] PURE Annotations**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.jsx | @qwik.dev/core | 1 |
| inlinedQrl | test.jsx | @qwik.dev/core | 1 |
| useStore | test.jsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
