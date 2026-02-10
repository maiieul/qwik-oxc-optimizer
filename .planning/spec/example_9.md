# Test: example_9

## Test Configuration

All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';
const Header = $((decl1, {decl2}, [decl3]) => {
	const {decl4, key: decl5} = this;
	let [decl6, ...decl7] = stuff;
	const decl8 = 1, decl9;
	function decl10(decl11, {decl12}, [decl13]) {}
	class decl14 {
		method(decl15, {decl16}, [decl17]) {}
	}
	try{}catch(decl18){}
	try{}catch({decl19}){}
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
                            "type": "ObjectPattern",
                            "decorators": [],
                            "properties": [
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "decl4",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 104,
                                  "end": 109
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "decl4",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 104,
                                  "end": 109
                                },
                                "method": false,
                                "shorthand": true,
                                "computed": false,
                                "optional": false,
                                "start": 104,
                                "end": 109
                              },
                              {
                                "type": "Property",
                                "kind": "init",
                                "key": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "key",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 111,
                                  "end": 114
                                },
                                "value": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "decl5",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 116,
                                  "end": 121
                                },
                                "method": false,
                                "shorthand": false,
                                "computed": false,
                                "optional": false,
                                "start": 111,
                                "end": 121
                              }
                            ],
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 103,
                            "end": 122
                          },
                          "init": {
                            "type": "ThisExpression",
                            "start": 125,
                            "end": 129
                          },
                          "definite": false,
                          "start": 103,
                          "end": 129
                        }
                      ],
                      "declare": false,
                      "start": 97,
                      "end": 130
                    },
                    {
                      "type": "VariableDeclaration",
                      "kind": "let",
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
                                "name": "decl6",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 137,
                                "end": 142
                              },
                              {
                                "type": "RestElement",
                                "decorators": [],
                                "argument": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "decl7",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 147,
                                  "end": 152
                                },
                                "optional": false,
                                "typeAnnotation": null,
                                "value": null,
                                "start": 144,
                                "end": 152
                              }
                            ],
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 136,
                            "end": 153
                          },
                          "init": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "stuff",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 156,
                            "end": 161
                          },
                          "definite": false,
                          "start": 136,
                          "end": 161
                        }
                      ],
                      "declare": false,
                      "start": 132,
                      "end": 162
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
                            "name": "decl8",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 170,
                            "end": 175
                          },
                          "init": {
                            "type": "Literal",
                            "value": 1,
                            "raw": "1",
                            "start": 178,
                            "end": 179
                          },
                          "definite": false,
                          "start": 170,
                          "end": 179
                        },
                        {
                          "type": "VariableDeclarator",
                          "id": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "decl9",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 181,
                            "end": 186
                          },
                          "init": null,
                          "definite": false,
                          "start": 181,
                          "end": 186
                        }
                      ],
                      "declare": false,
                      "start": 164,
                      "end": 187
                    },
                    {
                      "type": "FunctionDeclaration",
                      "id": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "decl10",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 198,
                        "end": 204
                      },
                      "generator": false,
                      "async": false,
                      "declare": false,
                      "typeParameters": null,
                      "params": [
                        {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "decl11",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 205,
                          "end": 211
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
                                "name": "decl12",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 214,
                                "end": 220
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl12",
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
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 213,
                          "end": 221
                        },
                        {
                          "type": "ArrayPattern",
                          "decorators": [],
                          "elements": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "decl13",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 224,
                              "end": 230
                            }
                          ],
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 223,
                          "end": 231
                        }
                      ],
                      "returnType": null,
                      "body": {
                        "type": "BlockStatement",
                        "body": [],
                        "start": 233,
                        "end": 235
                      },
                      "expression": false,
                      "start": 189,
                      "end": 235
                    },
                    {
                      "type": "ClassDeclaration",
                      "decorators": [],
                      "id": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "decl14",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 243,
                        "end": 249
                      },
                      "typeParameters": null,
                      "superClass": null,
                      "superTypeArguments": null,
                      "implements": [],
                      "body": {
                        "type": "ClassBody",
                        "body": [
                          {
                            "type": "MethodDefinition",
                            "decorators": [],
                            "key": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "method",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 254,
                              "end": 260
                            },
                            "value": {
                              "type": "FunctionExpression",
                              "id": null,
                              "generator": false,
                              "async": false,
                              "declare": false,
                              "typeParameters": null,
                              "params": [
                                {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "decl15",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 261,
                                  "end": 267
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
                                        "name": "decl16",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 270,
                                        "end": 276
                                      },
                                      "value": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "decl16",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 270,
                                        "end": 276
                                      },
                                      "method": false,
                                      "shorthand": true,
                                      "computed": false,
                                      "optional": false,
                                      "start": 270,
                                      "end": 276
                                    }
                                  ],
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 269,
                                  "end": 277
                                },
                                {
                                  "type": "ArrayPattern",
                                  "decorators": [],
                                  "elements": [
                                    {
                                      "type": "Identifier",
                                      "decorators": [],
                                      "name": "decl17",
                                      "optional": false,
                                      "typeAnnotation": null,
                                      "start": 280,
                                      "end": 286
                                    }
                                  ],
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 279,
                                  "end": 287
                                }
                              ],
                              "returnType": null,
                              "body": {
                                "type": "BlockStatement",
                                "body": [],
                                "start": 289,
                                "end": 291
                              },
                              "expression": false,
                              "start": 260,
                              "end": 291
                            },
                            "kind": "method",
                            "computed": false,
                            "static": false,
                            "override": false,
                            "optional": false,
                            "accessibility": null,
                            "start": 254,
                            "end": 291
                          }
                        ],
                        "start": 250,
                        "end": 294
                      },
                      "abstract": false,
                      "declare": false,
                      "start": 237,
                      "end": 294
                    },
                    {
                      "type": "TryStatement",
                      "block": {
                        "type": "BlockStatement",
                        "body": [],
                        "start": 299,
                        "end": 301
                      },
                      "handler": {
                        "type": "CatchClause",
                        "param": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "decl18",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 307,
                          "end": 313
                        },
                        "body": {
                          "type": "BlockStatement",
                          "body": [],
                          "start": 314,
                          "end": 316
                        },
                        "start": 301,
                        "end": 316
                      },
                      "finalizer": null,
                      "start": 296,
                      "end": 316
                    },
                    {
                      "type": "TryStatement",
                      "block": {
                        "type": "BlockStatement",
                        "body": [],
                        "start": 321,
                        "end": 323
                      },
                      "handler": {
                        "type": "CatchClause",
                        "param": {
                          "type": "ObjectPattern",
                          "decorators": [],
                          "properties": [
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl19",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 330,
                                "end": 336
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl19",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 330,
                                "end": 336
                              },
                              "method": false,
                              "shorthand": true,
                              "computed": false,
                              "optional": false,
                              "start": 330,
                              "end": 336
                            }
                          ],
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 329,
                          "end": 337
                        },
                        "body": {
                          "type": "BlockStatement",
                          "body": [],
                          "start": 338,
                          "end": 340
                        },
                        "start": 323,
                        "end": 340
                      },
                      "finalizer": null,
                      "start": 318,
                      "end": 340
                    }
                  ],
                  "start": 94,
                  "end": 342
                },
                "id": null,
                "generator": false,
                "start": 65,
                "end": 342
              }
            ],
            "optional": false,
            "start": 63,
            "end": 343
          },
          "definite": false,
          "start": 54,
          "end": 343
        }
      ],
      "declare": false,
      "start": 48,
      "end": 344
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 344
}
```

</details>

## Output

### Module: test.tsx_Header_WjUaUQN7Oxg.tsx [ENTRY POINT]

```tsx
export const Header_WjUaUQN7Oxg = (decl1, { decl2 }, [decl3])=>{
    const { decl4, key: decl5 } = this;
    let [decl6, ...decl7] = stuff;
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
              "name": "Header_WjUaUQN7Oxg",
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
                                "name": "decl4",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 77,
                                "end": 82
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl4",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 77,
                                "end": 82
                              },
                              "method": false,
                              "shorthand": true,
                              "computed": false,
                              "optional": false,
                              "start": 77,
                              "end": 82
                            },
                            {
                              "type": "Property",
                              "kind": "init",
                              "key": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "key",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 84,
                                "end": 87
                              },
                              "value": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl5",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 89,
                                "end": 94
                              },
                              "method": false,
                              "shorthand": false,
                              "computed": false,
                              "optional": false,
                              "start": 84,
                              "end": 94
                            }
                          ],
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 75,
                          "end": 96
                        },
                        "init": {
                          "type": "ThisExpression",
                          "start": 99,
                          "end": 103
                        },
                        "definite": false,
                        "start": 75,
                        "end": 103
                      }
                    ],
                    "declare": false,
                    "start": 69,
                    "end": 104
                  },
                  {
                    "type": "VariableDeclaration",
                    "kind": "let",
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
                              "name": "decl6",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 114,
                              "end": 119
                            },
                            {
                              "type": "RestElement",
                              "decorators": [],
                              "argument": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "decl7",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 124,
                                "end": 129
                              },
                              "optional": false,
                              "typeAnnotation": null,
                              "value": null,
                              "start": 121,
                              "end": 129
                            }
                          ],
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 113,
                          "end": 130
                        },
                        "init": {
                          "type": "Identifier",
                          "decorators": [],
                          "name": "stuff",
                          "optional": false,
                          "typeAnnotation": null,
                          "start": 133,
                          "end": 138
                        },
                        "definite": false,
                        "start": 113,
                        "end": 138
                      }
                    ],
                    "declare": false,
                    "start": 109,
                    "end": 139
                  }
                ],
                "start": 63,
                "end": 141
              },
              "id": null,
              "generator": false,
              "start": 34,
              "end": 141
            },
            "definite": false,
            "start": 13,
            "end": 141
          }
        ],
        "declare": false,
        "start": 7,
        "end": 142
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 0,
      "end": 142
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 142
}
```

</details>

#### Segment Metadata

```json
{
  "origin": "test.tsx",
  "name": "Header_WjUaUQN7Oxg",
  "entry": null,
  "displayName": "test.tsx_Header",
  "hash": "WjUaUQN7Oxg",
  "canonicalFilename": "test.tsx_Header_WjUaUQN7Oxg",
  "path": "",
  "extension": "tsx",
  "parent": null,
  "ctxKind": "function",
  "ctxName": "$",
  "captures": false,
  "loc": [
    67,
    344
  ],
  "paramNames": [
    "decl1",
    "{decl2}",
    "[decl3]"
  ]
}
```

### Module: test.tsx

```tsx
import { qrl } from "@qwik.dev/core";
const i_WjUaUQN7Oxg = ()=>import("./test.tsx_Header_WjUaUQN7Oxg");
/*#__PURE__*/ qrl(i_WjUaUQN7Oxg, "Header_WjUaUQN7Oxg");
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
            "name": "i_WjUaUQN7Oxg",
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
                "value": "./test.tsx_Header_WjUaUQN7Oxg",
                "raw": "\"./test.tsx_Header_WjUaUQN7Oxg\"",
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
            "name": "i_WjUaUQN7Oxg",
            "optional": false,
            "typeAnnotation": null,
            "start": 123,
            "end": 136
          },
          {
            "type": "Literal",
            "value": "Header_WjUaUQN7Oxg",
            "raw": "\"Header_WjUaUQN7Oxg\"",
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
| `qrl` | test.tsx | @qwik.dev/core | 1 |

## Diagnostics

No diagnostics.
