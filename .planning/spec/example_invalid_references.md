# Test: example_invalid_references

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | Segment (default) |
| Mode | Test (default) |
| Transpile TS | true |
| Transpile JSX | true |

## Input

### Source Code

```tsx
import { $, component$ } from '@qwik.dev/core';

const I1 = 12;
const [I2, {I3, v1: [I4], I5=v2, ...I6}, I7=v3, ...I8] = obj;
function I9() {}
class I10 {}

export const App = component$(({count}) => {
	console.log(I1, I2, I3, I4, I5, I6, I7, I8, I9);
	console.log(itsok, v1, v2, v3, obj);
	return $(() => {
		return (
			<I10></I10>
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
            "name": "I1",
            "optional": false,
            "typeAnnotation": null,
            "start": 55,
            "end": 57
          },
          "init": {
            "type": "Literal",
            "value": 12,
            "raw": "12",
            "start": 60,
            "end": 62
          },
          "definite": false,
          "start": 55,
          "end": 62
        }
      ],
      "declare": false,
      "start": 49,
      "end": 63
    },
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
                "name": "I2",
                "optional": false,
                "typeAnnotation": null,
                "start": 71,
                "end": 73
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
                      "start": 76,
                      "end": 78
                    },
                    "value": {
                      "type": "Identifier",
                      "decorators": [],
                      "name": "I3",
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 76,
                      "end": 78
                    },
                    "method": false,
                    "shorthand": true,
                    "computed": false,
                    "optional": false,
                    "start": 76,
                    "end": 78
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
                      "start": 80,
                      "end": 82
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
                          "start": 85,
                          "end": 87
                        }
                      ],
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 84,
                      "end": 88
                    },
                    "method": false,
                    "shorthand": false,
                    "computed": false,
                    "optional": false,
                    "start": 80,
                    "end": 88
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
                      "start": 90,
                      "end": 92
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
                        "start": 90,
                        "end": 92
                      },
                      "right": {
                        "type": "Identifier",
                        "decorators": [],
                        "name": "v2",
                        "optional": false,
                        "typeAnnotation": null,
                        "start": 93,
                        "end": 95
                      },
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 90,
                      "end": 95
                    },
                    "method": false,
                    "shorthand": true,
                    "computed": false,
                    "optional": false,
                    "start": 90,
                    "end": 95
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
                      "start": 100,
                      "end": 102
                    },
                    "optional": false,
                    "typeAnnotation": null,
                    "value": null,
                    "start": 97,
                    "end": 102
                  }
                ],
                "optional": false,
                "typeAnnotation": null,
                "start": 75,
                "end": 103
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
                  "start": 105,
                  "end": 107
                },
                "right": {
                  "type": "Identifier",
                  "decorators": [],
                  "name": "v3",
                  "optional": false,
                  "typeAnnotation": null,
                  "start": 108,
                  "end": 110
                },
                "optional": false,
                "typeAnnotation": null,
                "start": 105,
                "end": 110
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
                  "start": 115,
                  "end": 117
                },
                "optional": false,
                "typeAnnotation": null,
                "value": null,
                "start": 112,
                "end": 117
              }
            ],
            "optional": false,
            "typeAnnotation": null,
            "start": 70,
            "end": 118
          },
          "init": {
            "type": "Identifier",
            "decorators": [],
            "name": "obj",
            "optional": false,
            "typeAnnotation": null,
            "start": 121,
            "end": 124
          },
          "definite": false,
          "start": 70,
          "end": 124
        }
      ],
      "declare": false,
      "start": 64,
      "end": 125
    },
    {
      "type": "FunctionDeclaration",
      "id": {
        "type": "Identifier",
        "decorators": [],
        "name": "I9",
        "optional": false,
        "typeAnnotation": null,
        "start": 135,
        "end": 137
      },
      "generator": false,
      "async": false,
      "declare": false,
      "typeParameters": null,
      "params": [],
      "returnType": null,
      "body": {
        "type": "BlockStatement",
        "body": [],
        "start": 140,
        "end": 142
      },
      "expression": false,
      "start": 126,
      "end": 142
    },
    {
      "type": "ClassDeclaration",
      "decorators": [],
      "id": {
        "type": "Identifier",
        "decorators": [],
        "name": "I10",
        "optional": false,
        "typeAnnotation": null,
        "start": 149,
        "end": 152
      },
      "typeParameters": null,
      "superClass": null,
      "superTypeArguments": null,
      "implements": [],
      "body": {
        "type": "ClassBody",
        "body": [],
        "start": 153,
        "end": 155
      },
      "abstract": false,
      "declare": false,
      "start": 143,
      "end": 155
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
              "start": 170,
              "end": 173
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 176,
                "end": 186
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
                            "start": 189,
                            "end": 194
                          },
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "count",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 189,
                            "end": 194
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 189,
                          "end": 194
                        }
                      ],
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 188,
                      "end": 195
                    }
                  ],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
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
                              "start": 203,
                              "end": 210
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 211,
                              "end": 214
                            },
                            "optional": false,
                            "computed": false,
                            "start": 203,
                            "end": 214
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I1",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 215,
                              "end": 217
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 219,
                              "end": 221
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I3",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 223,
                              "end": 225
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I4",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 227,
                              "end": 229
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I5",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 231,
                              "end": 233
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I6",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 235,
                              "end": 237
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I7",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 239,
                              "end": 241
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I8",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 243,
                              "end": 245
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "I9",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 247,
                              "end": 249
                            }
                          ],
                          "optional": false,
                          "start": 203,
                          "end": 250
                        },
                        "directive": null,
                        "start": 203,
                        "end": 251
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
                              "start": 253,
                              "end": 260
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "log",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 261,
                              "end": 264
                            },
                            "optional": false,
                            "computed": false,
                            "start": 253,
                            "end": 264
                          },
                          "typeArguments": null,
                          "arguments": [
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "itsok",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 265,
                              "end": 270
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "v1",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 272,
                              "end": 274
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "v2",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 276,
                              "end": 278
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "v3",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 280,
                              "end": 282
                            },
                            {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "obj",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 284,
                              "end": 287
                            }
                          ],
                          "optional": false,
                          "start": 253,
                          "end": 288
                        },
                        "directive": null,
                        "start": 253,
                        "end": 289
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
                            "start": 298,
                            "end": 299
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
                                            "name": "I10",
                                            "start": 323,
                                            "end": 326
                                          },
                                          "typeArguments": null,
                                          "attributes": [],
                                          "selfClosing": false,
                                          "start": 322,
                                          "end": 327
                                        },
                                        "children": [],
                                        "closingElement": {
                                          "type": "JSXClosingElement",
                                          "name": {
                                            "type": "JSXIdentifier",
                                            "name": "I10",
                                            "start": 329,
                                            "end": 332
                                          },
                                          "start": 327,
                                          "end": 333
                                        },
                                        "start": 322,
                                        "end": 333
                                      },
                                      "start": 317,
                                      "end": 337
                                    },
                                    "start": 310,
                                    "end": 337
                                  }
                                ],
                                "start": 306,
                                "end": 340
                              },
                              "id": null,
                              "generator": false,
                              "start": 300,
                              "end": 340
                            }
                          ],
                          "optional": false,
                          "start": 298,
                          "end": 341
                        },
                        "start": 291,
                        "end": 342
                      }
                    ],
                    "start": 200,
                    "end": 344
                  },
                  "id": null,
                  "generator": false,
                  "start": 187,
                  "end": 344
                }
              ],
              "optional": false,
              "start": 176,
              "end": 345
            },
            "definite": false,
            "start": 170,
            "end": 345
          }
        ],
        "declare": false,
        "start": 164,
        "end": 345
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 157,
      "end": 345
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 346
}
```

</details>

## Output

### Module: test.js

```javascript
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_ckEPmXZlub0 = ()=>import("./test.tsx_App_component_ckEPmXZlub0");
const I1 = 12;
const [I2, { I3, v1: [I4], I5 = v2, ...I6 }, I7 = v3, ...I8] = obj;
function I9() {}
class I10 {
}
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_ckEPmXZlub0, "App_component_ckEPmXZlub0"));
export { I1 as _auto_I1 };
export { I10 as _auto_I10 };
export { I2 as _auto_I2 };
export { I3 as _auto_I3 };
export { I4 as _auto_I4 };
export { I5 as _auto_I5 };
export { I6 as _auto_I6 };
export { I7 as _auto_I7 };
export { I8 as _auto_I8 };
export { I9 as _auto_I9 };
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
      "type": "VariableDeclaration",
      "kind": "const",
      "declarations": [
        {
          "type": "VariableDeclarator",
          "id": {
            "type": "Identifier",
            "name": "I1",
            "start": 165,
            "end": 167
          },
          "init": {
            "type": "Literal",
            "value": 12,
            "raw": "12",
            "start": 170,
            "end": 172
          },
          "start": 165,
          "end": 172
        }
      ],
      "start": 159,
      "end": 173
    },
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
                "name": "I2",
                "start": 181,
                "end": 183
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
                      "start": 187,
                      "end": 189
                    },
                    "value": {
                      "type": "Identifier",
                      "name": "I3",
                      "start": 187,
                      "end": 189
                    },
                    "method": false,
                    "shorthand": true,
                    "computed": false,
                    "start": 187,
                    "end": 189
                  },
                  {
                    "type": "Property",
                    "kind": "init",
                    "key": {
                      "type": "Identifier",
                      "name": "v1",
                      "start": 191,
                      "end": 193
                    },
                    "value": {
                      "type": "ArrayPattern",
                      "elements": [
                        {
                          "type": "Identifier",
                          "name": "I4",
                          "start": 196,
                          "end": 198
                        }
                      ],
                      "start": 195,
                      "end": 199
                    },
                    "method": false,
                    "shorthand": false,
                    "computed": false,
                    "start": 191,
                    "end": 199
                  },
                  {
                    "type": "Property",
                    "kind": "init",
                    "key": {
                      "type": "Identifier",
                      "name": "I5",
                      "start": 201,
                      "end": 203
                    },
                    "value": {
                      "type": "AssignmentPattern",
                      "left": {
                        "type": "Identifier",
                        "name": "I5",
                        "start": 201,
                        "end": 203
                      },
                      "right": {
                        "type": "Identifier",
                        "name": "v2",
                        "start": 206,
                        "end": 208
                      },
                      "start": 201,
                      "end": 208
                    },
                    "method": false,
                    "shorthand": true,
                    "computed": false,
                    "start": 201,
                    "end": 208
                  },
                  {
                    "type": "RestElement",
                    "argument": {
                      "type": "Identifier",
                      "name": "I6",
                      "start": 213,
                      "end": 215
                    },
                    "start": 210,
                    "end": 215
                  }
                ],
                "start": 185,
                "end": 217
              },
              {
                "type": "AssignmentPattern",
                "left": {
                  "type": "Identifier",
                  "name": "I7",
                  "start": 219,
                  "end": 221
                },
                "right": {
                  "type": "Identifier",
                  "name": "v3",
                  "start": 224,
                  "end": 226
                },
                "start": 219,
                "end": 226
              },
              {
                "type": "RestElement",
                "argument": {
                  "type": "Identifier",
                  "name": "I8",
                  "start": 231,
                  "end": 233
                },
                "start": 228,
                "end": 233
              }
            ],
            "start": 180,
            "end": 234
          },
          "init": {
            "type": "Identifier",
            "name": "obj",
            "start": 237,
            "end": 240
          },
          "start": 180,
          "end": 240
        }
      ],
      "start": 174,
      "end": 241
    },
    {
      "type": "FunctionDeclaration",
      "id": {
        "type": "Identifier",
        "name": "I9",
        "start": 251,
        "end": 253
      },
      "generator": false,
      "async": false,
      "params": [],
      "body": {
        "type": "BlockStatement",
        "body": [],
        "start": 256,
        "end": 258
      },
      "expression": false,
      "start": 242,
      "end": 258
    },
    {
      "type": "ClassDeclaration",
      "decorators": [],
      "id": {
        "type": "Identifier",
        "name": "I10",
        "start": 265,
        "end": 268
      },
      "superClass": null,
      "body": {
        "type": "ClassBody",
        "body": [],
        "start": 269,
        "end": 272
      },
      "start": 259,
      "end": 272
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
              "start": 286,
              "end": 289
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 306,
                "end": 318
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "qrl",
                    "start": 333,
                    "end": 336
                  },
                  "arguments": [
                    {
                      "type": "Identifier",
                      "name": "i_ckEPmXZlub0",
                      "start": 337,
                      "end": 350
                    },
                    {
                      "type": "Literal",
                      "value": "App_component_ckEPmXZlub0",
                      "raw": "\"App_component_ckEPmXZlub0\"",
                      "start": 352,
                      "end": 379
                    }
                  ],
                  "optional": false,
                  "start": 333,
                  "end": 380
                }
              ],
              "optional": false,
              "start": 306,
              "end": 381
            },
            "start": 286,
            "end": 381
          }
        ],
        "start": 280,
        "end": 382
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 273,
      "end": 382
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I1",
            "start": 392,
            "end": 394
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I1",
            "start": 398,
            "end": 406
          },
          "start": 392,
          "end": 406
        }
      ],
      "source": null,
      "attributes": [],
      "start": 383,
      "end": 409
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I10",
            "start": 419,
            "end": 422
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I10",
            "start": 426,
            "end": 435
          },
          "start": 419,
          "end": 435
        }
      ],
      "source": null,
      "attributes": [],
      "start": 410,
      "end": 438
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I2",
            "start": 448,
            "end": 450
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I2",
            "start": 454,
            "end": 462
          },
          "start": 448,
          "end": 462
        }
      ],
      "source": null,
      "attributes": [],
      "start": 439,
      "end": 465
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I3",
            "start": 475,
            "end": 477
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I3",
            "start": 481,
            "end": 489
          },
          "start": 475,
          "end": 489
        }
      ],
      "source": null,
      "attributes": [],
      "start": 466,
      "end": 492
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I4",
            "start": 502,
            "end": 504
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I4",
            "start": 508,
            "end": 516
          },
          "start": 502,
          "end": 516
        }
      ],
      "source": null,
      "attributes": [],
      "start": 493,
      "end": 519
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I5",
            "start": 529,
            "end": 531
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I5",
            "start": 535,
            "end": 543
          },
          "start": 529,
          "end": 543
        }
      ],
      "source": null,
      "attributes": [],
      "start": 520,
      "end": 546
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I6",
            "start": 556,
            "end": 558
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I6",
            "start": 562,
            "end": 570
          },
          "start": 556,
          "end": 570
        }
      ],
      "source": null,
      "attributes": [],
      "start": 547,
      "end": 573
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I7",
            "start": 583,
            "end": 585
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I7",
            "start": 589,
            "end": 597
          },
          "start": 583,
          "end": 597
        }
      ],
      "source": null,
      "attributes": [],
      "start": 574,
      "end": 600
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I8",
            "start": 610,
            "end": 612
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I8",
            "start": 616,
            "end": 624
          },
          "start": 610,
          "end": 624
        }
      ],
      "source": null,
      "attributes": [],
      "start": 601,
      "end": 627
    },
    {
      "type": "ExportNamedDeclaration",
      "declaration": null,
      "specifiers": [
        {
          "type": "ExportSpecifier",
          "local": {
            "type": "Identifier",
            "name": "I9",
            "start": 637,
            "end": 639
          },
          "exported": {
            "type": "Identifier",
            "name": "_auto_I9",
            "start": 643,
            "end": 651
          },
          "start": 637,
          "end": 651
        }
      ],
      "source": null,
      "attributes": [],
      "start": 628,
      "end": 654
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 655
}
```

</details>

### Module: test.tsx_App_component_ckEPmXZlub0.js (ENTRY POINT)

```javascript
import { _auto_I1 as I1 } from "./test";
import { _auto_I2 as I2 } from "./test";
import { _auto_I3 as I3 } from "./test";
import { _auto_I4 as I4 } from "./test";
import { _auto_I5 as I5 } from "./test";
import { _auto_I6 as I6 } from "./test";
import { _auto_I7 as I7 } from "./test";
import { _auto_I8 as I8 } from "./test";
import { _auto_I9 as I9 } from "./test";
import { qrl } from "@qwik.dev/core";
const i_w0t0o3QMovU = ()=>import("./test.tsx_App_component_1_w0t0o3QMovU");
export const App_component_ckEPmXZlub0 = (_rawProps)=>{
    console.log(I1, I2, I3, I4, I5, I6, I7, I8, I9);
    console.log(itsok, v1, v2, v3, obj);
    return /*#__PURE__*/ qrl(i_w0t0o3QMovU, "App_component_1_w0t0o3QMovU");
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
            "name": "_auto_I1",
            "start": 9,
            "end": 17
          },
          "local": {
            "type": "Identifier",
            "name": "I1",
            "start": 21,
            "end": 23
          },
          "start": 9,
          "end": 23
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 31,
        "end": 39
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 40
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I2",
            "start": 50,
            "end": 58
          },
          "local": {
            "type": "Identifier",
            "name": "I2",
            "start": 62,
            "end": 64
          },
          "start": 50,
          "end": 64
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 72,
        "end": 80
      },
      "phase": null,
      "attributes": [],
      "start": 41,
      "end": 81
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I3",
            "start": 91,
            "end": 99
          },
          "local": {
            "type": "Identifier",
            "name": "I3",
            "start": 103,
            "end": 105
          },
          "start": 91,
          "end": 105
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 113,
        "end": 121
      },
      "phase": null,
      "attributes": [],
      "start": 82,
      "end": 122
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I4",
            "start": 132,
            "end": 140
          },
          "local": {
            "type": "Identifier",
            "name": "I4",
            "start": 144,
            "end": 146
          },
          "start": 132,
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
      "start": 123,
      "end": 163
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I5",
            "start": 173,
            "end": 181
          },
          "local": {
            "type": "Identifier",
            "name": "I5",
            "start": 185,
            "end": 187
          },
          "start": 173,
          "end": 187
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 195,
        "end": 203
      },
      "phase": null,
      "attributes": [],
      "start": 164,
      "end": 204
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I6",
            "start": 214,
            "end": 222
          },
          "local": {
            "type": "Identifier",
            "name": "I6",
            "start": 226,
            "end": 228
          },
          "start": 214,
          "end": 228
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 236,
        "end": 244
      },
      "phase": null,
      "attributes": [],
      "start": 205,
      "end": 245
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I7",
            "start": 255,
            "end": 263
          },
          "local": {
            "type": "Identifier",
            "name": "I7",
            "start": 267,
            "end": 269
          },
          "start": 255,
          "end": 269
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 277,
        "end": 285
      },
      "phase": null,
      "attributes": [],
      "start": 246,
      "end": 286
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I8",
            "start": 296,
            "end": 304
          },
          "local": {
            "type": "Identifier",
            "name": "I8",
            "start": 308,
            "end": 310
          },
          "start": 296,
          "end": 310
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 318,
        "end": 326
      },
      "phase": null,
      "attributes": [],
      "start": 287,
      "end": 327
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_auto_I9",
            "start": 337,
            "end": 345
          },
          "local": {
            "type": "Identifier",
            "name": "I9",
            "start": 349,
            "end": 351
          },
          "start": 337,
          "end": 351
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 359,
        "end": 367
      },
      "phase": null,
      "attributes": [],
      "start": 328,
      "end": 368
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "qrl",
            "start": 378,
            "end": 381
          },
          "local": {
            "type": "Identifier",
            "name": "qrl",
            "start": 378,
            "end": 381
          },
          "start": 378,
          "end": 381
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 389,
        "end": 405
      },
      "phase": null,
      "attributes": [],
      "start": 369,
      "end": 406
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
            "start": 413,
            "end": 426
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
                "start": 440,
                "end": 480
              },
              "options": null,
              "phase": null,
              "start": 433,
              "end": 481
            },
            "id": null,
            "generator": false,
            "start": 429,
            "end": 481
          },
          "start": 413,
          "end": 481
        }
      ],
      "start": 407,
      "end": 482
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
              "start": 496,
              "end": 521
            },
            "init": {
              "type": "ArrowFunctionExpression",
              "expression": false,
              "async": false,
              "params": [
                {
                  "type": "Identifier",
                  "name": "_rawProps",
                  "start": 525,
                  "end": 534
                }
              ],
              "body": {
                "type": "BlockStatement",
                "body": [
                  {
                    "type": "ExpressionStatement",
                    "expression": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "MemberExpression",
                        "object": {
                          "type": "Identifier",
                          "name": "console",
                          "start": 543,
                          "end": 550
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 551,
                          "end": 554
                        },
                        "optional": false,
                        "computed": false,
                        "start": 543,
                        "end": 554
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "I1",
                          "start": 555,
                          "end": 557
                        },
                        {
                          "type": "Identifier",
                          "name": "I2",
                          "start": 559,
                          "end": 561
                        },
                        {
                          "type": "Identifier",
                          "name": "I3",
                          "start": 563,
                          "end": 565
                        },
                        {
                          "type": "Identifier",
                          "name": "I4",
                          "start": 567,
                          "end": 569
                        },
                        {
                          "type": "Identifier",
                          "name": "I5",
                          "start": 571,
                          "end": 573
                        },
                        {
                          "type": "Identifier",
                          "name": "I6",
                          "start": 575,
                          "end": 577
                        },
                        {
                          "type": "Identifier",
                          "name": "I7",
                          "start": 579,
                          "end": 581
                        },
                        {
                          "type": "Identifier",
                          "name": "I8",
                          "start": 583,
                          "end": 585
                        },
                        {
                          "type": "Identifier",
                          "name": "I9",
                          "start": 587,
                          "end": 589
                        }
                      ],
                      "optional": false,
                      "start": 543,
                      "end": 590
                    },
                    "start": 543,
                    "end": 591
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
                          "start": 596,
                          "end": 603
                        },
                        "property": {
                          "type": "Identifier",
                          "name": "log",
                          "start": 604,
                          "end": 607
                        },
                        "optional": false,
                        "computed": false,
                        "start": 596,
                        "end": 607
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "itsok",
                          "start": 608,
                          "end": 613
                        },
                        {
                          "type": "Identifier",
                          "name": "v1",
                          "start": 615,
                          "end": 617
                        },
                        {
                          "type": "Identifier",
                          "name": "v2",
                          "start": 619,
                          "end": 621
                        },
                        {
                          "type": "Identifier",
                          "name": "v3",
                          "start": 623,
                          "end": 625
                        },
                        {
                          "type": "Identifier",
                          "name": "obj",
                          "start": 627,
                          "end": 630
                        }
                      ],
                      "optional": false,
                      "start": 596,
                      "end": 631
                    },
                    "start": 596,
                    "end": 632
                  },
                  {
                    "type": "ReturnStatement",
                    "argument": {
                      "type": "CallExpression",
                      "callee": {
                        "type": "Identifier",
                        "name": "qrl",
                        "start": 658,
                        "end": 661
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "i_w0t0o3QMovU",
                          "start": 662,
                          "end": 675
                        },
                        {
                          "type": "Literal",
                          "value": "App_component_1_w0t0o3QMovU",
                          "raw": "\"App_component_1_w0t0o3QMovU\"",
                          "start": 677,
                          "end": 706
                        }
                      ],
                      "optional": false,
                      "start": 658,
                      "end": 707
                    },
                    "start": 637,
                    "end": 708
                  }
                ],
                "start": 537,
                "end": 710
              },
              "id": null,
              "generator": false,
              "start": 524,
              "end": 710
            },
            "start": 496,
            "end": 710
          }
        ],
        "start": 490,
        "end": 711
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 483,
      "end": 711
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 712
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
  "loc": [189, 346],
  "paramNames": ["_rawProps"]
}
```

### Module: test.tsx_App_component_1_w0t0o3QMovU.js (ENTRY POINT)

```javascript
import { _auto_I10 as I10 } from "./test";
import { _jsxSorted } from "@qwik.dev/core";
export const App_component_1_w0t0o3QMovU = ()=>{
    return /*#__PURE__*/ _jsxSorted(I10, null, null, null, 3, "u6_0");
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
            "name": "_auto_I10",
            "start": 9,
            "end": 18
          },
          "local": {
            "type": "Identifier",
            "name": "I10",
            "start": 22,
            "end": 25
          },
          "start": 9,
          "end": 25
        }
      ],
      "source": {
        "type": "Literal",
        "value": "./test",
        "raw": "\"./test\"",
        "start": 33,
        "end": 41
      },
      "phase": null,
      "attributes": [],
      "start": 0,
      "end": 42
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 52,
            "end": 62
          },
          "local": {
            "type": "Identifier",
            "name": "_jsxSorted",
            "start": 52,
            "end": 62
          },
          "start": 52,
          "end": 62
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 70,
        "end": 86
      },
      "phase": null,
      "attributes": [],
      "start": 43,
      "end": 87
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
              "start": 101,
              "end": 128
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
                        "start": 162,
                        "end": 172
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "name": "I10",
                          "start": 173,
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
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 184,
                          "end": 188
                        },
                        {
                          "type": "Literal",
                          "value": null,
                          "raw": "null",
                          "start": 190,
                          "end": 194
                        },
                        {
                          "type": "Literal",
                          "value": 3,
                          "raw": "3",
                          "start": 196,
                          "end": 197
                        },
                        {
                          "type": "Literal",
                          "value": "u6_0",
                          "raw": "\"u6_0\"",
                          "start": 199,
                          "end": 205
                        }
                      ],
                      "optional": false,
                      "start": 162,
                      "end": 206
                    },
                    "start": 141,
                    "end": 207
                  }
                ],
                "start": 135,
                "end": 209
              },
              "id": null,
              "generator": false,
              "start": 131,
              "end": 209
            },
            "start": 101,
            "end": 209
          }
        ],
        "start": 95,
        "end": 210
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 88,
      "end": 210
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 211
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
  "captures": false,
  "loc": [302, 342]
}
```

## Conventions Applied

- **[CONV-01] QRL Wrapping**: `qrl()` calls used to wrap lazy imports for segment references
- **[CONV-02] Dollar-to-Qrl Conversion**: `component$` converted to `componentQrl(qrl(...))`, `$()` converted to `qrl()`
- **[CONV-03] JSX Transforms**: `<I10></I10>` transformed to `_jsxSorted(I10, ...)` in nested segment
- **[CONV-06] Lazy Imports**: `const i_HASH = ()=>import("./...")` generated for each segment
- **[CONV-07] PURE Annotations**: `/*#__PURE__*/` on `componentQrl()`, `qrl()`, and `_jsxSorted()` calls
- **[CONV-08] Segment Extraction**: Component body extracted to entry point segment, nested `$()` callback extracted to separate segment. Captured identifiers (I1-I10) re-exported as `_auto_*` aliases
- **[CONV-11] Props Destructuring**: Component parameter destructured as `_rawProps` in segment (original `{count}` destructuring deferred)

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.js | @qwik.dev/core | 1 |
| qrl | test.js | @qwik.dev/core | 1 |
| qrl | test.tsx_App_component_ckEPmXZlub0.js | @qwik.dev/core | 1 |
| _jsxSorted | test.tsx_App_component_1_w0t0o3QMovU.js | @qwik.dev/core | 1 |

## Diagnostics

None (`[]`)
