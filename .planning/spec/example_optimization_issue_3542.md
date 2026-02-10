# Test: example_optimization_issue_3542

## Test Configuration

**Note:** Optimization fix for issue 3542 -- handling onClick$ event handler with complex arguments in inline mode.

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

export const AtomStatus = component$(({ctx,atom})=>{
	let status = atom.status;
	if(!atom.real) {
		status="WILL-VANISH"
	} else if (JSON.stringify(atom.atom)==JSON.stringify(atom.real)) {
		status="WTFED"
	}
	return (
		<span title={atom.ID} onClick$={(ev)=>atomStatusClick(ctx,ev,[atom])} class={["atom",status,ctx.store[atom.ID]?"selected":null]}>
		</span>
	);
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
              "name": "AtomStatus",
              "optional": false,
              "typeAnnotation": null,
              "start": 59,
              "end": 69
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "decorators": [],
                "name": "component$",
                "optional": false,
                "typeAnnotation": null,
                "start": 72,
                "end": 82
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
                            "name": "ctx",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 85,
                            "end": 88
                          },
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "ctx",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 85,
                            "end": 88
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 85,
                          "end": 88
                        },
                        {
                          "type": "Property",
                          "kind": "init",
                          "key": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "atom",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 89,
                            "end": 93
                          },
                          "value": {
                            "type": "Identifier",
                            "decorators": [],
                            "name": "atom",
                            "optional": false,
                            "typeAnnotation": null,
                            "start": 89,
                            "end": 93
                          },
                          "method": false,
                          "shorthand": true,
                          "computed": false,
                          "optional": false,
                          "start": 89,
                          "end": 93
                        }
                      ],
                      "optional": false,
                      "typeAnnotation": null,
                      "start": 84,
                      "end": 94
                    }
                  ],
                  "returnType": null,
                  "body": {
                    "type": "BlockStatement",
                    "body": [
                      {
                        "type": "VariableDeclaration",
                        "kind": "let",
                        "declarations": [
                          {
                            "type": "VariableDeclarator",
                            "id": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "status",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 104,
                              "end": 110
                            },
                            "init": {
                              "type": "MemberExpression",
                              "object": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "atom",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 113,
                                "end": 117
                              },
                              "property": {
                                "type": "Identifier",
                                "decorators": [],
                                "name": "status",
                                "optional": false,
                                "typeAnnotation": null,
                                "start": 118,
                                "end": 124
                              },
                              "optional": false,
                              "computed": false,
                              "start": 113,
                              "end": 124
                            },
                            "definite": false,
                            "start": 104,
                            "end": 124
                          }
                        ],
                        "declare": false,
                        "start": 100,
                        "end": 125
                      },
                      {
                        "type": "IfStatement",
                        "test": {
                          "type": "UnaryExpression",
                          "operator": "!",
                          "argument": {
                            "type": "MemberExpression",
                            "object": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "atom",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 131,
                              "end": 135
                            },
                            "property": {
                              "type": "Identifier",
                              "decorators": [],
                              "name": "real",
                              "optional": false,
                              "typeAnnotation": null,
                              "start": 136,
                              "end": 140
                            },
                            "optional": false,
                            "computed": false,
                            "start": 131,
                            "end": 140
                          },
                          "prefix": true,
                          "start": 130,
                          "end": 140
                        },
                        "consequent": {
                          "type": "BlockStatement",
                          "body": [
                            {
                              "type": "ExpressionStatement",
                              "expression": {
                                "type": "AssignmentExpression",
                                "operator": "=",
                                "left": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "status",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 146,
                                  "end": 152
                                },
                                "right": {
                                  "type": "Literal",
                                  "value": "WILL-VANISH",
                                  "raw": "\"WILL-VANISH\"",
                                  "start": 153,
                                  "end": 166
                                },
                                "start": 146,
                                "end": 166
                              },
                              "directive": null,
                              "start": 146,
                              "end": 166
                            }
                          ],
                          "start": 142,
                          "end": 169
                        },
                        "alternate": {
                          "type": "IfStatement",
                          "test": {
                            "type": "BinaryExpression",
                            "left": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "JSON",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 179,
                                  "end": 183
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "stringify",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 184,
                                  "end": 193
                                },
                                "optional": false,
                                "computed": false,
                                "start": 179,
                                "end": 193
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "atom",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 194,
                                    "end": 198
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "atom",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 199,
                                    "end": 203
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 194,
                                  "end": 203
                                }
                              ],
                              "optional": false,
                              "start": 179,
                              "end": 204
                            },
                            "operator": "==",
                            "right": {
                              "type": "CallExpression",
                              "callee": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "JSON",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 206,
                                  "end": 210
                                },
                                "property": {
                                  "type": "Identifier",
                                  "decorators": [],
                                  "name": "stringify",
                                  "optional": false,
                                  "typeAnnotation": null,
                                  "start": 211,
                                  "end": 220
                                },
                                "optional": false,
                                "computed": false,
                                "start": 206,
                                "end": 220
                              },
                              "typeArguments": null,
                              "arguments": [
                                {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "atom",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 221,
                                    "end": 225
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "real",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 226,
                                    "end": 230
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 221,
                                  "end": 230
                                }
                              ],
                              "optional": false,
                              "start": 206,
                              "end": 231
                            },
                            "start": 179,
                            "end": 231
                          },
                          "consequent": {
                            "type": "BlockStatement",
                            "body": [
                              {
                                "type": "ExpressionStatement",
                                "expression": {
                                  "type": "AssignmentExpression",
                                  "operator": "=",
                                  "left": {
                                    "type": "Identifier",
                                    "decorators": [],
                                    "name": "status",
                                    "optional": false,
                                    "typeAnnotation": null,
                                    "start": 237,
                                    "end": 243
                                  },
                                  "right": {
                                    "type": "Literal",
                                    "value": "WTFED",
                                    "raw": "\"WTFED\"",
                                    "start": 244,
                                    "end": 251
                                  },
                                  "start": 237,
                                  "end": 251
                                },
                                "directive": null,
                                "start": 237,
                                "end": 251
                              }
                            ],
                            "start": 233,
                            "end": 254
                          },
                          "alternate": null,
                          "start": 175,
                          "end": 254
                        },
                        "start": 127,
                        "end": 254
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
                                "name": "span",
                                "start": 268,
                                "end": 272
                              },
                              "typeArguments": null,
                              "attributes": [
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "title",
                                    "start": 273,
                                    "end": 278
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "atom",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 280,
                                        "end": 284
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "decorators": [],
                                        "name": "ID",
                                        "optional": false,
                                        "typeAnnotation": null,
                                        "start": 285,
                                        "end": 287
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 280,
                                      "end": 287
                                    },
                                    "start": 279,
                                    "end": 288
                                  },
                                  "start": 273,
                                  "end": 288
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "onClick$",
                                    "start": 289,
                                    "end": 297
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
                                          "name": "ev",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 300,
                                          "end": 302
                                        }
                                      ],
                                      "returnType": null,
                                      "body": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "atomStatusClick",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 305,
                                          "end": 320
                                        },
                                        "typeArguments": null,
                                        "arguments": [
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "ctx",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 321,
                                            "end": 324
                                          },
                                          {
                                            "type": "Identifier",
                                            "decorators": [],
                                            "name": "ev",
                                            "optional": false,
                                            "typeAnnotation": null,
                                            "start": 325,
                                            "end": 327
                                          },
                                          {
                                            "type": "ArrayExpression",
                                            "elements": [
                                              {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "atom",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 329,
                                                "end": 333
                                              }
                                            ],
                                            "start": 328,
                                            "end": 334
                                          }
                                        ],
                                        "optional": false,
                                        "start": 305,
                                        "end": 335
                                      },
                                      "id": null,
                                      "generator": false,
                                      "start": 299,
                                      "end": 335
                                    },
                                    "start": 298,
                                    "end": 336
                                  },
                                  "start": 289,
                                  "end": 336
                                },
                                {
                                  "type": "JSXAttribute",
                                  "name": {
                                    "type": "JSXIdentifier",
                                    "name": "class",
                                    "start": 337,
                                    "end": 342
                                  },
                                  "value": {
                                    "type": "JSXExpressionContainer",
                                    "expression": {
                                      "type": "ArrayExpression",
                                      "elements": [
                                        {
                                          "type": "Literal",
                                          "value": "atom",
                                          "raw": "\"atom\"",
                                          "start": 345,
                                          "end": 351
                                        },
                                        {
                                          "type": "Identifier",
                                          "decorators": [],
                                          "name": "status",
                                          "optional": false,
                                          "typeAnnotation": null,
                                          "start": 352,
                                          "end": 358
                                        },
                                        {
                                          "type": "ConditionalExpression",
                                          "test": {
                                            "type": "MemberExpression",
                                            "object": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ctx",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 359,
                                                "end": 362
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "store",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 363,
                                                "end": 368
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 359,
                                              "end": 368
                                            },
                                            "property": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "atom",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 369,
                                                "end": 373
                                              },
                                              "property": {
                                                "type": "Identifier",
                                                "decorators": [],
                                                "name": "ID",
                                                "optional": false,
                                                "typeAnnotation": null,
                                                "start": 374,
                                                "end": 376
                                              },
                                              "optional": false,
                                              "computed": false,
                                              "start": 369,
                                              "end": 376
                                            },
                                            "optional": false,
                                            "computed": true,
                                            "start": 359,
                                            "end": 377
                                          },
                                          "consequent": {
                                            "type": "Literal",
                                            "value": "selected",
                                            "raw": "\"selected\"",
                                            "start": 378,
                                            "end": 388
                                          },
                                          "alternate": {
                                            "type": "Literal",
                                            "value": null,
                                            "raw": "null",
                                            "start": 389,
                                            "end": 393
                                          },
                                          "start": 359,
                                          "end": 393
                                        }
                                      ],
                                      "start": 344,
                                      "end": 394
                                    },
                                    "start": 343,
                                    "end": 395
                                  },
                                  "start": 337,
                                  "end": 395
                                }
                              ],
                              "selfClosing": false,
                              "start": 267,
                              "end": 396
                            },
                            "children": [
                              {
                                "type": "JSXText",
                                "value": "\n\t\t",
                                "raw": "\n\t\t",
                                "start": 396,
                                "end": 399
                              }
                            ],
                            "closingElement": {
                              "type": "JSXClosingElement",
                              "name": {
                                "type": "JSXIdentifier",
                                "name": "span",
                                "start": 401,
                                "end": 405
                              },
                              "start": 399,
                              "end": 406
                            },
                            "start": 267,
                            "end": 406
                          },
                          "start": 263,
                          "end": 409
                        },
                        "start": 256,
                        "end": 410
                      }
                    ],
                    "start": 97,
                    "end": 412
                  },
                  "id": null,
                  "generator": false,
                  "start": 83,
                  "end": 412
                }
              ],
              "optional": false,
              "start": 72,
              "end": 413
            },
            "definite": false,
            "start": 59,
            "end": 413
          }
        ],
        "declare": false,
        "start": 53,
        "end": 413
      },
      "specifiers": [],
      "source": null,
      "exportKind": "value",
      "attributes": [],
      "start": 46,
      "end": 413
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 413
}

```

</details>

## Output

### Module: test.jsx

```jsx
import { componentQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
export const AtomStatus = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl((_rawProps)=>{
    let status = _rawProps.atom.status;
    if (!_rawProps.atom.real) status = "WILL-VANISH";
    else if (JSON.stringify(_rawProps.atom.atom) == JSON.stringify(_rawProps.atom.real)) status = "WTFED";
    return <span title={_rawProps.atom.ID} q-e:click={/*#__PURE__*/ inlinedQrl((ev)=>{
        const _rawProps = _captures[0];
        return atomStatusClick(_rawProps.ctx, ev, [
            _rawProps.atom
        ]);
    }, "AtomStatus_component_span_q_e_click_0yqKAycyBF0", [
        _rawProps
    ])} class={[
        "atom",
        status,
        _rawProps.ctx.store[_rawProps.atom.ID] ? "selected" : null
    ]}>
		</span>;
}, "AtomStatus_component_hdwpoUtydSA"));
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
            "name": "_captures",
            "start": 56,
            "end": 65
          },
          "local": {
            "type": "Identifier",
            "name": "_captures",
            "start": 56,
            "end": 65
          },
          "start": 56,
          "end": 65
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 73,
        "end": 89
      },
      "phase": null,
      "attributes": [],
      "start": 47,
      "end": 90
    },
    {
      "type": "ImportDeclaration",
      "specifiers": [
        {
          "type": "ImportSpecifier",
          "imported": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 100,
            "end": 110
          },
          "local": {
            "type": "Identifier",
            "name": "inlinedQrl",
            "start": 100,
            "end": 110
          },
          "start": 100,
          "end": 110
        }
      ],
      "source": {
        "type": "Literal",
        "value": "@qwik.dev/core",
        "raw": "\"@qwik.dev/core\"",
        "start": 118,
        "end": 134
      },
      "phase": null,
      "attributes": [],
      "start": 91,
      "end": 135
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
              "name": "AtomStatus",
              "start": 149,
              "end": 159
            },
            "init": {
              "type": "CallExpression",
              "callee": {
                "type": "Identifier",
                "name": "componentQrl",
                "start": 176,
                "end": 188
              },
              "arguments": [
                {
                  "type": "CallExpression",
                  "callee": {
                    "type": "Identifier",
                    "name": "inlinedQrl",
                    "start": 203,
                    "end": 213
                  },
                  "arguments": [
                    {
                      "type": "ArrowFunctionExpression",
                      "expression": false,
                      "async": false,
                      "params": [
                        {
                          "type": "Identifier",
                          "name": "_rawProps",
                          "start": 215,
                          "end": 224
                        }
                      ],
                      "body": {
                        "type": "BlockStatement",
                        "body": [
                          {
                            "type": "VariableDeclaration",
                            "kind": "let",
                            "declarations": [
                              {
                                "type": "VariableDeclarator",
                                "id": {
                                  "type": "Identifier",
                                  "name": "status",
                                  "start": 237,
                                  "end": 243
                                },
                                "init": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "name": "_rawProps",
                                      "start": 246,
                                      "end": 255
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "atom",
                                      "start": 256,
                                      "end": 260
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 246,
                                    "end": 260
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "status",
                                    "start": 261,
                                    "end": 267
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 246,
                                  "end": 267
                                },
                                "start": 237,
                                "end": 267
                              }
                            ],
                            "start": 233,
                            "end": 268
                          },
                          {
                            "type": "IfStatement",
                            "test": {
                              "type": "UnaryExpression",
                              "operator": "!",
                              "argument": {
                                "type": "MemberExpression",
                                "object": {
                                  "type": "MemberExpression",
                                  "object": {
                                    "type": "Identifier",
                                    "name": "_rawProps",
                                    "start": 278,
                                    "end": 287
                                  },
                                  "property": {
                                    "type": "Identifier",
                                    "name": "atom",
                                    "start": 288,
                                    "end": 292
                                  },
                                  "optional": false,
                                  "computed": false,
                                  "start": 278,
                                  "end": 292
                                },
                                "property": {
                                  "type": "Identifier",
                                  "name": "real",
                                  "start": 293,
                                  "end": 297
                                },
                                "optional": false,
                                "computed": false,
                                "start": 278,
                                "end": 297
                              },
                              "prefix": true,
                              "start": 277,
                              "end": 297
                            },
                            "consequent": {
                              "type": "ExpressionStatement",
                              "expression": {
                                "type": "AssignmentExpression",
                                "operator": "=",
                                "left": {
                                  "type": "Identifier",
                                  "name": "status",
                                  "start": 299,
                                  "end": 305
                                },
                                "right": {
                                  "type": "Literal",
                                  "value": "WILL-VANISH",
                                  "raw": "\"WILL-VANISH\"",
                                  "start": 308,
                                  "end": 321
                                },
                                "start": 299,
                                "end": 321
                              },
                              "start": 299,
                              "end": 322
                            },
                            "alternate": {
                              "type": "IfStatement",
                              "test": {
                                "type": "BinaryExpression",
                                "left": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "name": "JSON",
                                      "start": 336,
                                      "end": 340
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "stringify",
                                      "start": 341,
                                      "end": 350
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 336,
                                    "end": 350
                                  },
                                  "arguments": [
                                    {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "_rawProps",
                                          "start": 351,
                                          "end": 360
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "atom",
                                          "start": 361,
                                          "end": 365
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 351,
                                        "end": 365
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "atom",
                                        "start": 366,
                                        "end": 370
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 351,
                                      "end": 370
                                    }
                                  ],
                                  "optional": false,
                                  "start": 336,
                                  "end": 371
                                },
                                "operator": "==",
                                "right": {
                                  "type": "CallExpression",
                                  "callee": {
                                    "type": "MemberExpression",
                                    "object": {
                                      "type": "Identifier",
                                      "name": "JSON",
                                      "start": 375,
                                      "end": 379
                                    },
                                    "property": {
                                      "type": "Identifier",
                                      "name": "stringify",
                                      "start": 380,
                                      "end": 389
                                    },
                                    "optional": false,
                                    "computed": false,
                                    "start": 375,
                                    "end": 389
                                  },
                                  "arguments": [
                                    {
                                      "type": "MemberExpression",
                                      "object": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "Identifier",
                                          "name": "_rawProps",
                                          "start": 390,
                                          "end": 399
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "atom",
                                          "start": 400,
                                          "end": 404
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 390,
                                        "end": 404
                                      },
                                      "property": {
                                        "type": "Identifier",
                                        "name": "real",
                                        "start": 405,
                                        "end": 409
                                      },
                                      "optional": false,
                                      "computed": false,
                                      "start": 390,
                                      "end": 409
                                    }
                                  ],
                                  "optional": false,
                                  "start": 375,
                                  "end": 410
                                },
                                "start": 336,
                                "end": 410
                              },
                              "consequent": {
                                "type": "ExpressionStatement",
                                "expression": {
                                  "type": "AssignmentExpression",
                                  "operator": "=",
                                  "left": {
                                    "type": "Identifier",
                                    "name": "status",
                                    "start": 412,
                                    "end": 418
                                  },
                                  "right": {
                                    "type": "Literal",
                                    "value": "WTFED",
                                    "raw": "\"WTFED\"",
                                    "start": 421,
                                    "end": 428
                                  },
                                  "start": 412,
                                  "end": 428
                                },
                                "start": 412,
                                "end": 429
                              },
                              "alternate": null,
                              "start": 332,
                              "end": 429
                            },
                            "start": 273,
                            "end": 429
                          },
                          {
                            "type": "ReturnStatement",
                            "argument": {
                              "type": "JSXElement",
                              "openingElement": {
                                "type": "JSXOpeningElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "span",
                                  "start": 442,
                                  "end": 446
                                },
                                "attributes": [
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "title",
                                      "start": 447,
                                      "end": 452
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "MemberExpression",
                                        "object": {
                                          "type": "MemberExpression",
                                          "object": {
                                            "type": "Identifier",
                                            "name": "_rawProps",
                                            "start": 454,
                                            "end": 463
                                          },
                                          "property": {
                                            "type": "Identifier",
                                            "name": "atom",
                                            "start": 464,
                                            "end": 468
                                          },
                                          "optional": false,
                                          "computed": false,
                                          "start": 454,
                                          "end": 468
                                        },
                                        "property": {
                                          "type": "Identifier",
                                          "name": "ID",
                                          "start": 469,
                                          "end": 471
                                        },
                                        "optional": false,
                                        "computed": false,
                                        "start": 454,
                                        "end": 471
                                      },
                                      "start": 453,
                                      "end": 472
                                    },
                                    "start": 447,
                                    "end": 472
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXNamespacedName",
                                      "namespace": {
                                        "type": "JSXIdentifier",
                                        "name": "q-e",
                                        "start": 473,
                                        "end": 476
                                      },
                                      "name": {
                                        "type": "JSXIdentifier",
                                        "name": "click",
                                        "start": 477,
                                        "end": 482
                                      },
                                      "start": 473,
                                      "end": 482
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "CallExpression",
                                        "callee": {
                                          "type": "Identifier",
                                          "name": "inlinedQrl",
                                          "start": 498,
                                          "end": 508
                                        },
                                        "arguments": [
                                          {
                                            "type": "ArrowFunctionExpression",
                                            "expression": false,
                                            "async": false,
                                            "params": [
                                              {
                                                "type": "Identifier",
                                                "name": "ev",
                                                "start": 510,
                                                "end": 512
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
                                                        "name": "_rawProps",
                                                        "start": 531,
                                                        "end": 540
                                                      },
                                                      "init": {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "Identifier",
                                                          "name": "_captures",
                                                          "start": 543,
                                                          "end": 552
                                                        },
                                                        "property": {
                                                          "type": "Literal",
                                                          "value": 0,
                                                          "raw": "0",
                                                          "start": 553,
                                                          "end": 554
                                                        },
                                                        "optional": false,
                                                        "computed": true,
                                                        "start": 543,
                                                        "end": 555
                                                      },
                                                      "start": 531,
                                                      "end": 555
                                                    }
                                                  ],
                                                  "start": 525,
                                                  "end": 556
                                                },
                                                {
                                                  "type": "ReturnStatement",
                                                  "argument": {
                                                    "type": "CallExpression",
                                                    "callee": {
                                                      "type": "Identifier",
                                                      "name": "atomStatusClick",
                                                      "start": 572,
                                                      "end": 587
                                                    },
                                                    "arguments": [
                                                      {
                                                        "type": "MemberExpression",
                                                        "object": {
                                                          "type": "Identifier",
                                                          "name": "_rawProps",
                                                          "start": 588,
                                                          "end": 597
                                                        },
                                                        "property": {
                                                          "type": "Identifier",
                                                          "name": "ctx",
                                                          "start": 598,
                                                          "end": 601
                                                        },
                                                        "optional": false,
                                                        "computed": false,
                                                        "start": 588,
                                                        "end": 601
                                                      },
                                                      {
                                                        "type": "Identifier",
                                                        "name": "ev",
                                                        "start": 603,
                                                        "end": 605
                                                      },
                                                      {
                                                        "type": "ArrayExpression",
                                                        "elements": [
                                                          {
                                                            "type": "MemberExpression",
                                                            "object": {
                                                              "type": "Identifier",
                                                              "name": "_rawProps",
                                                              "start": 621,
                                                              "end": 630
                                                            },
                                                            "property": {
                                                              "type": "Identifier",
                                                              "name": "atom",
                                                              "start": 631,
                                                              "end": 635
                                                            },
                                                            "optional": false,
                                                            "computed": false,
                                                            "start": 621,
                                                            "end": 635
                                                          }
                                                        ],
                                                        "start": 607,
                                                        "end": 645
                                                      }
                                                    ],
                                                    "optional": false,
                                                    "start": 572,
                                                    "end": 646
                                                  },
                                                  "start": 565,
                                                  "end": 647
                                                }
                                              ],
                                              "start": 515,
                                              "end": 653
                                            },
                                            "id": null,
                                            "generator": false,
                                            "start": 509,
                                            "end": 653
                                          },
                                          {
                                            "type": "Literal",
                                            "value": "AtomStatus_component_span_q_e_click_0yqKAycyBF0",
                                            "raw": "\"AtomStatus_component_span_q_e_click_0yqKAycyBF0\"",
                                            "start": 655,
                                            "end": 704
                                          },
                                          {
                                            "type": "ArrayExpression",
                                            "elements": [
                                              {
                                                "type": "Identifier",
                                                "name": "_rawProps",
                                                "start": 716,
                                                "end": 725
                                              }
                                            ],
                                            "start": 706,
                                            "end": 731
                                          }
                                        ],
                                        "optional": false,
                                        "start": 498,
                                        "end": 732
                                      },
                                      "start": 483,
                                      "end": 733
                                    },
                                    "start": 473,
                                    "end": 733
                                  },
                                  {
                                    "type": "JSXAttribute",
                                    "name": {
                                      "type": "JSXIdentifier",
                                      "name": "class",
                                      "start": 734,
                                      "end": 739
                                    },
                                    "value": {
                                      "type": "JSXExpressionContainer",
                                      "expression": {
                                        "type": "ArrayExpression",
                                        "elements": [
                                          {
                                            "type": "Literal",
                                            "value": "atom",
                                            "raw": "\"atom\"",
                                            "start": 751,
                                            "end": 757
                                          },
                                          {
                                            "type": "Identifier",
                                            "name": "status",
                                            "start": 767,
                                            "end": 773
                                          },
                                          {
                                            "type": "ConditionalExpression",
                                            "test": {
                                              "type": "MemberExpression",
                                              "object": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "name": "_rawProps",
                                                    "start": 783,
                                                    "end": 792
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "name": "ctx",
                                                    "start": 793,
                                                    "end": 796
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 783,
                                                  "end": 796
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "name": "store",
                                                  "start": 797,
                                                  "end": 802
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 783,
                                                "end": 802
                                              },
                                              "property": {
                                                "type": "MemberExpression",
                                                "object": {
                                                  "type": "MemberExpression",
                                                  "object": {
                                                    "type": "Identifier",
                                                    "name": "_rawProps",
                                                    "start": 803,
                                                    "end": 812
                                                  },
                                                  "property": {
                                                    "type": "Identifier",
                                                    "name": "atom",
                                                    "start": 813,
                                                    "end": 817
                                                  },
                                                  "optional": false,
                                                  "computed": false,
                                                  "start": 803,
                                                  "end": 817
                                                },
                                                "property": {
                                                  "type": "Identifier",
                                                  "name": "ID",
                                                  "start": 818,
                                                  "end": 820
                                                },
                                                "optional": false,
                                                "computed": false,
                                                "start": 803,
                                                "end": 820
                                              },
                                              "optional": false,
                                              "computed": true,
                                              "start": 783,
                                              "end": 821
                                            },
                                            "consequent": {
                                              "type": "Literal",
                                              "value": "selected",
                                              "raw": "\"selected\"",
                                              "start": 824,
                                              "end": 834
                                            },
                                            "alternate": {
                                              "type": "Literal",
                                              "value": null,
                                              "raw": "null",
                                              "start": 837,
                                              "end": 841
                                            },
                                            "start": 783,
                                            "end": 841
                                          }
                                        ],
                                        "start": 741,
                                        "end": 847
                                      },
                                      "start": 740,
                                      "end": 848
                                    },
                                    "start": 734,
                                    "end": 848
                                  }
                                ],
                                "selfClosing": false,
                                "start": 441,
                                "end": 849
                              },
                              "children": [
                                {
                                  "type": "JSXText",
                                  "value": "\n\t\t",
                                  "raw": "\n\t\t",
                                  "start": 849,
                                  "end": 852
                                }
                              ],
                              "closingElement": {
                                "type": "JSXClosingElement",
                                "name": {
                                  "type": "JSXIdentifier",
                                  "name": "span",
                                  "start": 854,
                                  "end": 858
                                },
                                "start": 852,
                                "end": 859
                              },
                              "start": 441,
                              "end": 859
                            },
                            "start": 434,
                            "end": 860
                          }
                        ],
                        "start": 227,
                        "end": 862
                      },
                      "id": null,
                      "generator": false,
                      "start": 214,
                      "end": 862
                    },
                    {
                      "type": "Literal",
                      "value": "AtomStatus_component_hdwpoUtydSA",
                      "raw": "\"AtomStatus_component_hdwpoUtydSA\"",
                      "start": 864,
                      "end": 898
                    }
                  ],
                  "optional": false,
                  "start": 203,
                  "end": 899
                }
              ],
              "optional": false,
              "start": 176,
              "end": 900
            },
            "start": 149,
            "end": 900
          }
        ],
        "start": 143,
        "end": 901
      },
      "specifiers": [],
      "source": null,
      "attributes": [],
      "start": 136,
      "end": 901
    }
  ],
  "sourceType": "module",
  "hashbang": null,
  "start": 0,
  "end": 901
}

```

</details>

## Conventions Applied

- **[CONV-01] QRL Wrapping**
- **[CONV-02] Dollar-to-Qrl Conversion**
- **[CONV-05] Capture Patterns**
- **[CONV-07] PURE Annotations**
- **[CONV-11] Props Destructuring**

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| componentQrl | test.jsx | @qwik.dev/core | 1 |
| inlinedQrl | test.jsx | @qwik.dev/core | 2 |

## Diagnostics

None (`[]`)
