# Test: example_10

## Test Configuration

| Option | Value |
|--------|-------|
| Filename | project/test.tsx |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
const Header = $((decl1, {decl2}, [decl3]) => {

	const hola = ident1.no;
	ident2;
	const a = ident1 + ident3;
	const b = ident1 + ident3;
	ident4(ident5, [ident6], {ident7}, {key: ident8});
	class Some {
		prop = ident9;
		method() {
			return ident10;
		}
	}

	return (
		<div onClick={(ident11) => ident11 + ident12} required={false}/>
	)
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
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "decorators": [],
            "name": "Header",
            "optional": false,
            "typeAnnotation": null,
            "start": 54,
            "end": 60
          },
          "init": {
            "type": "CallExpression",
            "callee": {
              "type": "Identifier",
              "decorators": [],
              "name": "$",
              "optional": false,
              "typeAnnotation": null,
              "start": 63,
              "end": 64
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
                    "name": "decl1",
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 66,
                    "end": 71
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
                          "name": "decl2",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 74,
                          "end": 79
                        },
                        "value": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "decl2",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 74,
                          "end": 79
                        },
                        "method": false,
                        "shorthand": true,
                        "computed": false,
                        "optional": false,
                        "start": 74,
                        "end": 79
                      }
                    ],
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 73,
                    "end": 80
                  },
                  {
                    "type": "ArrayPattern",
                    "decorators": [],
                    "elements": [
                      {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "decl3",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 83,
                        "end": 88
                      }
                    ],
                    "optional": false,
                    "typeAnnotation": null,
                    "start": 82,
                    "end": 89
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
                            "name": "hola",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 104,
                            "end": 108
                          },
                          "init": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident1",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 111,
                              "end": 117
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "no",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 118,
                              "end": 120
                            },
                            "optional": false,
                            "computed": false,
                            "start": 111,
                            "end": 120
                          },
                          "definite": false,
                          "start": 104,
                          "end": 120
                        }
                      ],
                      "declare": false,
                      "start": 98,
                      "end": 121
                    },
                    {
                      "type": "ExpressionStatement",
                      "expression": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "ident2",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 123,
                        "end": 129
                      },
                      "directive": null,
                      "start": 123,
                      "end": 130
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
                            "name": "a",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 138,
                            "end": 139
                          },
                          "init": {
                            "type": "BinaryExpression",
                            "left": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident1",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 142,
                              "end": 148
                            },
                            "operator": "+",
                            "right": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident3",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 151,
                              "end": 157
                            },
                            "start": 142,
                            "end": 157
                          },
                          "definite": false,
                          "start": 138,
                          "end": 157
                        }
                      ],
                      "declare": false,
                      "start": 132,
                      "end": 158
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
                            "name": "b",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 166,
                            "end": 167
                          },
                          "init": {
                            "type": "BinaryExpression",
                            "left": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident1",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 170,
                              "end": 176
                            },
                            "operator": "+",
                            "right": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident3",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 179,
                              "end": 185
                            },
                            "start": 170,
                            "end": 185
                          },
                          "definite": false,
                          "start": 166,
                          "end": 185
                        }
                      ],
                      "declare": false,
                      "start": 160,
                      "end": 186
                    },
                    {
                      "type": "ExpressionStatement",
                      "expression": {
                        "type": "CallExpression",
                        "callee": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident4",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 188,
                          "end": 194
                        },
                        "typeArguments": null,
                        "arguments": [
                          {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "ident5",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 195,
                            "end": 201
                          },
                          {
                            "type": "ArrayExpression",
                            "elements": [
                              {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "ident6",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 204,
                                "end": 210
                              }
                            ],
                            "start": 203,
                            "end": 211
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
                                  "name": "ident7",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 214,
                                  "end": 220
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "ident7",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 214,
                                  "end": 220
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "optional": false,
                                "start": 214,
                                "end": 220
                              }
                            ],
                            "start": 213,
                            "end": 221
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
                                  "name": "key",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 224,
                                  "end": 227
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "ident8",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 229,
                                  "end": 235
                                },
                                "method": false,
                                "shorthand": false,
                                "computed": false,
                                "optional": false,
                                "start": 224,
                                "end": 235
                              }
                            ],
                            "start": 223,
                            "end": 236
                          }
                        ],
                        "optional": false,
                        "start": 188,
                        "end": 237
                      },
                      "directive": null,
                      "start": 188,
                      "end": 238
                    },
                    {
                      "type": "ClassDeclaration",
                      "decorators": [],
                      "id": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "Some",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 246,
                        "end": 250
                      },
                      "typeParameters": null,
                      "superClass": null,
                      "superTypeArguments": null,
                      "implements": [],
                      "body": {
                        "type": "ClassBody",
                        "body": [
                          {
                            "type": "PropertyDefinition",
                            "decorators": [],
                            "key": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "prop",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 255,
                              "end": 259
                            },
                            "typeAnnotation": null,
                            "value": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident9",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 262,
                              "end": 268
                            },
                            "computed": false,
                            "static": false,
                            "declare": false,
                            "override": false,
                            "optional": false,
                            "definite": false,
                            "readonly": false,
                            "accessibility": null,
                            "start": 255,
                            "end": 269
                          },
                          {
                            "type": "MethodDefinition",
                            "decorators": [],
                            "key": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "method",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 272,
                              "end": 278
                            },
                            "value": {
                              "type": "FunctionExpression",
                              "id": null,
                              "generator": false,
                              "async": false,
                              "declare": false,
                              "typeParameters": null,
                              "params": [],
                              "returnType": null,
                              "body": {
                                "type": "BlockStatement",
                                "body": [
                                  {
                                    "type": "ReturnStatement",
                                    "argument": {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "ident10",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 293,
                                      "end": 300
                                    },
                                    "start": 286,
                                    "end": 301
                                  }
                                ],
                                "start": 281,
                                "end": 305
                              },
                              "expression": false,
                              "start": 278,
                              "end": 305
                            },
                            "kind": "method",
                            "computed": false,
                            "static": false,
                            "override": false,
                            "optional": false,
                            "accessibility": null,
                            "start": 272,
                            "end": 305
                          }
                        ],
                        "start": 251,
                        "end": 308
                      },
                      "abstract": false,
                      "declare": false,
                      "start": 240,
                      "end": 308
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
                              "start": 323,
                              "end": 326
                            },
                            "typeArguments": null,
                            "attributes": [
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "onClick",
                                  "start": 327,
                                  "end": 334
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "ArrowFunctionExpression",
                                    "expression": true,
                                    "async": false,
                                    "typeParameters": null,
                                    "params": [
                                      {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "ident11",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 337,
                                        "end": 344
                                      }
                                    ],
                                    "returnType": null,
                                    "body": {
                                      "type": "BinaryExpression",
                                      "left": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "ident11",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 349,
                                        "end": 356
                                      },
                                      "operator": "+",
                                      "right": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "ident12",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 359,
                                        "end": 366
                                      },
                                      "start": 349,
                                      "end": 366
                                    },
                                    "id": null,
                                    "generator": false,
                                    "start": 336,
                                    "end": 366
                                  },
                                  "start": 335,
                                  "end": 367
                                },
                                "start": 327,
                                "end": 367
                              },
                              {
                                "type": "JSXAttribute",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "required",
                                  "start": 368,
                                  "end": 376
                                },
                                "value": {
                                  "type": "JSXExpressionContainer",
                                  "expression": {
                                    "type": "Literal",
                                    "value": false,
                                    "raw": "false",
                                    "start": 378,
                                    "end": 383
                                  },
                                  "start": 377,
                                  "end": 384
                                },
                                "start": 368,
                                "end": 384
                              }
                            ],
                            "selfClosing": true,
                            "start": 322,
                            "end": 386
                          },
                          "children": [],
                          "closingElement": null,
                          "start": 322,
                          "end": 386
                        },
                        "start": 318,
                        "end": 389
                      },
                      "start": 311,
                      "end": 389
                    }
                  ],
                  "start": 94,
                  "end": 391
                },
                "id": null,
                "generator": false,
                "start": 65,
                "end": 391
              }
            ],
            "optional": false,
            "start": 63,
            "end": 392
          },
          "definite": false,
          "start": 54,
          "end": 392
        }
      ],
      "declare": false,
      "start": 48,
      "end": 393
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 393
}
```

</details>

## Output

### Module: project/test.tsx_Header_WlR3xnI6u38.tsx (ENTRY POINT)

```tsx
export const Header_WlR3xnI6u38 = (decl1, { decl2 }, [decl3])=>{
    ident1.no;
    ident2;
    ident1, ident3;
    ident1, ident3;
    ident4(ident5, [
        ident6
    ], {
        ident7
    }, {
        key: ident8
    });
    class Some {
        prop = ident9;
        method() {
            return ident10;
        }
    }
    return <div onClick={(ident11)=>ident11 + ident12} required={false}/>;
};
```

<details>
<summary>Output AST (OXC)</summary>

```json
{
  "type": "Program",
  "body": [
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
              "name": "Header_WlR3xnI6u38",
              "optional": false,
              "typeAnnotation": null,
              "start": 13,
              "end": 31
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
                  "name": "decl1",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 35,
                  "end": 40
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
                        "name": "decl2",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 44,
                        "end": 49
                      },
                      "value": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "decl2",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 44,
                        "end": 49
                      },
                      "method": false,
                      "shorthand": true,
                      "computed": false,
                      "optional": false,
                      "start": 44,
                      "end": 49
                    }
                  ],
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 42,
                  "end": 51
                },
                {
                  "type": "ArrayPattern",
                  "decorators": [],
                  "elements": [
                    {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "decl3",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 54,
                      "end": 59
                    }
                  ],
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 53,
                  "end": 60
                }
              ],
              "returnType": null,
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "MemberExpression",
                      "object": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "ident1",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 69,
                        "end": 75
                      },
                      "property": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "no",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 76,
                        "end": 78
                      },
                      "optional": false,
                      "computed": false,
                      "start": 69,
                      "end": 78
                    },
                    "directive": null,
                    "start": 69,
                    "end": 79
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "ident2",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 84,
                      "end": 90
                    },
                    "directive": null,
                    "start": 84,
                    "end": 91
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "SequenceExpression",
                      "expressions": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident1",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 96,
                          "end": 102
                        },
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident3",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 104,
                          "end": 110
                        }
                      ],
                      "start": 96,
                      "end": 110
                    },
                    "directive": null,
                    "start": 96,
                    "end": 111
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "SequenceExpression",
                      "expressions": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident1",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 116,
                          "end": 122
                        },
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident3",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 124,
                          "end": 130
                        }
                      ],
                      "start": 116,
                      "end": 130
                    },
                    "directive": null,
                    "start": 116,
                    "end": 131
                  },
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "ident4",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 136,
                        "end": 142
                      },
                      "typeArguments": null,
                      "arguments": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "ident5",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 143,
                          "end": 149
                        },
                        {
                          "type": "ArrayExpression",
                          "elements": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "ident6",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 161,
                              "end": 167
                            }
                          ],
                          "start": 151,
                          "end": 173
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
                                "name": "ident7",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 185,
                                "end": 191
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "ident7",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 185,
                                "end": 191
                              },
                              "method": false,
                              "shorthand": true,
                              "computed": false,
                              "optional": false,
                              "start": 185,
                              "end": 191
                            }
                          ],
                          "start": 175,
                          "end": 197
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
                                "name": "key",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 209,
                                "end": 212
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "ident8",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 214,
                                "end": 220
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "optional": false,
                              "start": 209,
                              "end": 220
                            }
                          ],
                          "start": 199,
                          "end": 226
                        }
                      ],
                      "optional": false,
                      "start": 136,
                      "end": 227
                    },
                    "directive": null,
                    "start": 136,
                    "end": 228
                  },
                  {
                    "type": "ClassDeclaration",
                    "decorators": [],
                    "id": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "Some",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 239,
                      "end": 243
                    },
                    "typeParameters": null,
                    "superClass": null,
                    "superTypeArguments": null,
                    "implements": [],
                    "body": {
                      "type": "ClassBody",
                      "body": [
                        {
                          "type": "PropertyDefinition",
                          "decorators": [],
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "prop",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 254,
                            "end": 258
                          },
                          "typeAnnotation": null,
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "ident9",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 261,
                            "end": 267
                          },
                          "computed": false,
                          "static": false,
                          "declare": false,
                          "override": false,
                          "optional": false,
                          "definite": false,
                          "readonly": false,
                          "accessibility": null,
                          "start": 254,
                          "end": 268
                        },
                        {
                          "type": "MethodDefinition",
                          "decorators": [],
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "method",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 277,
                            "end": 283
                          },
                          "value": {
                            "type": "FunctionExpression",
                            "id": null,
                            "generator": false,
                            "async": false,
                            "declare": false,
                            "typeParameters": null,
                            "params": [],
                            "returnType": null,
                            "body": {
                              "type": "BlockStatement",
                              "body": [
                                {
                                  "type": "ReturnStatement",
                                  "argument": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ident10",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 307,
                                    "end": 314
                                  },
                                  "start": 300,
                                  "end": 315
                                }
                              ],
                              "start": 286,
                              "end": 325
                            },
                            "expression": false,
                            "start": 283,
                            "end": 325
                          },
                          "kind": "method",
                          "computed": false,
                          "static": false,
                          "override": false,
                          "optional": false,
                          "accessibility": null,
                          "start": 277,
                          "end": 325
                        }
                      ],
                      "start": 244,
                      "end": 331
                    },
                    "abstract": false,
                    "declare": false,
                    "start": 233,
                    "end": 331
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
                          "start": 344,
                          "end": 347
                        },
                        "typeArguments": null,
                        "attributes": [
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "onClick",
                              "start": 348,
                              "end": 355
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "ArrowFunctionExpression",
                                "expression": true,
                                "async": false,
                                "typeParameters": null,
                                "params": [
                                  {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ident11",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 358,
                                    "end": 365
                                  }
                                ],
                                "returnType": null,
                                "body": {
                                  "type": "BinaryExpression",
                                  "left": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ident11",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 368,
                                    "end": 375
                                  },
                                  "operator": "+",
                                  "right": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "ident12",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 378,
                                    "end": 385
                                  },
                                  "start": 368,
                                  "end": 385
                                },
                                "id": null,
                                "generator": false,
                                "start": 357,
                                "end": 385
                              },
                              "start": 356,
                              "end": 386
                            },
                            "start": 348,
                            "end": 386
                          },
                          {
                            "type": "JSXAttribute",
                            "name": {
                              "type": "JSXIdentifier",
                              "name": "required",
                              "start": 387,
                              "end": 395
                            },
                            "value": {
                              "type": "JSXExpressionContainer",
                              "expression": {
                                "type": "Literal",
                                "value": false,
                                "raw": "false",
                                "start": 397,
                                "end": 402
                              },
                              "start": 396,
                              "end": 403
                            },
                            "start": 387,
                            "end": 403
                          }
                        ],
                        "selfClosing": true,
                        "start": 343,
                        "end": 405
                      },
                      "children": [],
                      "closingElement": null,
                      "start": 343,
                      "end": 405
                    },
                    "start": 336,
                    "end": 406
                  }
                ],
                "start": 63,
                "end": 408
              },
              "id": null,
              "generator": false,
              "start": 34,
              "end": 408
            },
            "definite": false,
            "start": 13,
            "end": 408
          }
        ],
        "declare": false,
        "start": 7,
        "end": 409
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 409
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 409
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "project/test.tsx",
  "name": "Header_WlR3xnI6u38",
  "entry": null,
  "displayName": "test.tsx_Header",
  "hash": "WlR3xnI6u38",
  "canonicalFilename": "test.tsx_Header_WlR3xnI6u38",
  "path": "project",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    67,
    393
  ],
  "paramNames": [
    "decl1",
    "{decl2}",
    "[decl3]"
  ]
}
```

### Module: project/test.tsx

```tsx
import { qrl } from "@qwik.dev/core";
const i_WlR3xnI6u38 = ()=>import("./test.tsx_Header_WlR3xnI6u38");
/*#__PURE__*/ qrl(i_WlR3xnI6u38, "Header_WlR3xnI6u38");
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
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "local": {
            "type": "Identifier",
            "decorators": [],
            "name": "qrl",
            "optional": false,
            "typeAnnotation": null,
            "start": 9,
            "end": 12
          },
          "importKind": "value",
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
      "importKind": "value",
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
            "decorators": [],
            "name": "i_WlR3xnI6u38",
            "optional": false,
            "typeAnnotation": null,
            "start": 44,
            "end": 57
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
                "value": "./test.tsx_Header_WlR3xnI6u38",
                "raw": "\"./test.tsx_Header_WlR3xnI6u38\"",
                "start": 71,
                "end": 102
              },
              "options": null,
              "phase": null,
              "start": 64,
              "end": 103
            },
            "id": null,
            "generator": false,
            "start": 60,
            "end": 103
          },
          "definite": false,
          "start": 44,
          "end": 103
        }
      ],
      "declare": false,
      "start": 38,
      "end": 104
    },
    {
      "type": "ExpressionStatement",
      "expression": {
        "type": "CallExpression",
        "callee": {
          "type": "Identifier",
          "decorators": [],
          "name": "qrl",
          "optional": false,
          "typeAnnotation": null,
          "start": 119,
          "end": 122
        },
        "typeArguments": null,
        "arguments": [
          {
            "type": "Identifier",
            "decorators": [],
            "name": "i_WlR3xnI6u38",
            "optional": false,
            "typeAnnotation": null,
            "start": 123,
            "end": 136
          },
          {
            "type": "Literal",
            "value": "Header_WlR3xnI6u38",
            "raw": "\"Header_WlR3xnI6u38\"",
            "start": 138,
            "end": 158
          }
        ],
        "optional": false,
        "start": 119,
        "end": 159
      },
      "directive": null,
      "start": 119,
      "end": 160
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 160
}
```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**: QRL wrapping via `qrl()` calls to create lazy-loadable references
- **[CONV-06] Lazy Imports**: Dynamic lazy import declarations for code-split segments (1 lazy import(s))
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` annotations for tree-shaking (1 occurrence(s))
- **[CONV-08] Segment Extraction**: Code extracted into 1 separate entry point segment(s)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `qrl` | project/test.tsx | @qwik.dev/core | 1 |

## Diagnostics

```json
[]
```
