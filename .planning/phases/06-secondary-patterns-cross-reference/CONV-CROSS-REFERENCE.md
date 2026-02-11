# Complete 14 CONV Type Cross-Reference (APIM-08)

**Date:** 2026-02-11
**Purpose:** The single-page lookup reference for implementing any of the 14 CONV (convention) types in the OXC-based Qwik optimizer. For each CONV, this document provides the exact OXC detection API, construction API with Rust code snippets, primary spec file references, occurrence frequency, dependency constraints, and a direct link to the detailed mapping document.

**Coverage:** All 14 CONV types across 6 mapping documents from Phases 4, 5, and 6. Zero gaps.

---

## 1. Overview

The Qwik optimizer transforms source code through 14 distinct convention types (CONVs). Each CONV represents a specific transformation pattern -- from detecting `$()` call sites to generating source maps. Together, these 14 patterns cover the complete optimizer behavior observed across all 162 spec files.

This cross-reference serves as the **master index**. For any given CONV type, a developer can look up:

1. **What to detect** -- the OXC Traverse hook and condition
2. **What to construct** -- the OXC AstBuilder API calls with Rust code
3. **Where to find details** -- the exact mapping document and section
4. **How common it is** -- occurrence frequency across spec files
5. **What must run first** -- dependency ordering constraints

---

## 2. Master Cross-Reference Table

### CONV-01: QRL Calls

| Field | Content |
|-------|---------|
| **CONV** | CONV-01: QRL Calls |
| **Detection API** | `enter_call_expression` -> check callee is `$`-suffixed identifier imported from `@qwik.dev/core` via `is_dollar_call()` helper |
| **Construction API** | `build_qrl_call()` for segment strategy; `build_inlined_qrl_call()` for inline/hoist strategy |
| **Primary Spec Files** | `example_1.md`, `example_functional_component.md`, `example_inlined_entry_strategy.md` |
| **Frequency** | 162/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md`, Pattern 1 ($() Extraction) and Pattern 2 (QRL Wrapping) |
| **Phase Mapped** | Phase 4 |
| **Dependencies** | None (foundational, runs first) |

**Construction Snippet (Segment strategy):**
```rust
// qrl(i_HASH, "segment_name_HASH")
let import_ref = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom(import_ident_name));
let name_literal = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(segment_export_name), None);
let mut arguments = ctx.ast.vec_with_capacity(2);
arguments.push(Argument::from(import_ref));
arguments.push(Argument::from(name_literal));
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("qrl"));
ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
```

---

### CONV-02: Dollar-to-QRL Suffix

| Field | Content |
|-------|---------|
| **CONV** | CONV-02: Dollar-to-QRL Suffix |
| **Detection API** | `enter_call_expression` -> check callee ends with `$` and is imported from `@qwik.dev/core` |
| **Construction API** | `dollar_to_qrl_name()` utility: strip trailing `$`, append `Qrl`, then `ctx.ast.expression_identifier_reference()` |
| **Primary Spec Files** | `example_functional_component.md`, `example_derived_signals_cmp.md` |
| **Frequency** | 157/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md`, Pattern 1 (The `dollar_to_qrl_name()` Utility) |
| **Phase Mapped** | Phase 4 |
| **Dependencies** | Concurrent with CONV-01 (same detection pass) |

**Construction Snippet:**
```rust
// component$(...) -> componentQrl(qrl_call)
let qrl_name = dollar_to_qrl_name("component$"); // -> "componentQrl"
let qrl_callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom(&qrl_name));
let mut args = ctx.ast.vec_with_capacity(1);
args.push(Argument::from(replacement));
ctx.ast.expression_call(SPAN, qrl_callee, NONE, args, false)
```

---

### CONV-03: JSX Transforms

| Field | Content |
|-------|---------|
| **CONV** | CONV-03: JSX Transforms |
| **Detection API** | `enter_jsx_element` / `enter_jsx_fragment` in Traverse, replacement in `exit_expression` matching `Expression::JSXElement` / `Expression::JSXFragment` |
| **Construction API** | `build_jsx_sorted_call()` for standard elements; `build_jsx_split_call()` for spread elements; `build_fragment_import()` for `<>` fragments |
| **Primary Spec Files** | `example_jsx.md`, `example_derived_signals_cmp.md`, `should_destructure_args.md` |
| **Frequency** | 113/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/JSX-TRANSFORMS-MAPPING.md`, Sections 1-10 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | CONCURRENT WITH CONV-04 (signal helpers generated during prop analysis); BEFORE CONV-07 (PURE annotations added after construction) |

**Construction Snippet:**
```rust
// _jsxSorted(tag, varProps, constProps, children, flags, key)
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_jsxSorted"));
let mut arguments = ctx.ast.vec_with_capacity(6);
arguments.push(Argument::from(tag_expr));
arguments.push(Argument::from(var_props.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN))));
arguments.push(Argument::from(const_props.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN))));
arguments.push(Argument::from(children_expr.unwrap_or_else(|| ctx.ast.expression_null_literal(SPAN))));
arguments.push(Argument::from(ctx.ast.expression_numeric_literal(SPAN, flags as f64, None, NumberBase::Decimal)));
arguments.push(Argument::from(key_or_null));
ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
```

---

### CONV-04: Signal Helpers

| Field | Content |
|-------|---------|
| **CONV** | CONV-04: Signal Helpers |
| **Detection API** | During JSX prop value analysis: check for `MemberExpression` with property `"value"` (signal), multi-level property chains (store), computed expressions with reactive sources |
| **Construction API** | `build_wrap_prop_signal()` for `_wrapProp(signal)` (Form 1); `build_wrap_prop_named()` for `_wrapProp(source, "prop")` (Form 2); `build_fn_signal_call()` for `_fnSignal(_hfN, [deps], _hfN_str)` |
| **Primary Spec Files** | `example_derived_signals_cmp.md`, `example_getter_generation.md` |
| **Frequency** | 25/162 (Common) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md`, Sections 2-3 and 5 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | CONCURRENT WITH CONV-03 (generated during JSX prop analysis); BEFORE CONV-14 (hoisted functions referenced by _fnSignal) |

**Construction Snippet (Form 1 -- signal.value):**
```rust
// _wrapProp(signal)
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_wrapProp"));
let mut args = ctx.ast.vec_with_capacity(1);
args.push(Argument::from(signal_expr));
ctx.ast.expression_call(SPAN, callee, NONE, args, false)
```

**Construction Snippet (Form 2 -- named property):**
```rust
// _wrapProp(store, "count")
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_wrapProp"));
let mut args = ctx.ast.vec_with_capacity(2);
args.push(Argument::from(source_expr));
args.push(Argument::from(ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(prop_name), None)));
ctx.ast.expression_call(SPAN, callee, NONE, args, false)
```

---

### CONV-05: Capture Patterns

| Field | Content |
|-------|---------|
| **CONV** | CONV-05: Capture Patterns |
| **Detection API** | `oxc_semantic::Scoping` API: `symbol_ids()` iteration, `symbol_scope_id()` for declaration scope, `scope_ancestors()` for containment check, `get_resolved_references()` for inner-body reference detection |
| **Construction API** | Capture restoration: `ast.expression_member(captures_ident, numeric_index)` producing `_captures[N]`; capture array: `ast.expression_array()` in `qrl()` / `inlinedQrl()` third argument |
| **Primary Spec Files** | `example_multi_capture.md`, `example_strip_server_code.md`, `example_inlined_entry_strategy.md` |
| **Frequency** | 45/162 (Common) |
| **Detailed Mapping** | `.planning/phases/05-deep-research-proof-of-concept/CAPTURE-ANALYSIS-MAPPING.md`, Sections 1-7 |
| **Phase Mapped** | Phase 5 |
| **Dependencies** | AFTER CONV-11 (props destructuring changes variable references); BEFORE CONV-08 (segments need capture lists) |

**Construction Snippet:**
```rust
// const _rawProps = _captures[0];
let captures_ref = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_captures"));
let index_expr = ctx.ast.expression_numeric_literal(SPAN, index as f64, None, NumberBase::Decimal);
let member = ctx.ast.expression_computed_member(SPAN, captures_ref, index_expr, false);
```

---

### CONV-06: Lazy Imports

| Field | Content |
|-------|---------|
| **CONV** | CONV-06: Lazy Imports |
| **Detection API** | Generated during emit phase for Segment/Smart/Component/Hook/Single strategies (not detected from source -- constructed as output) |
| **Construction API** | `build_lazy_import_declaration()`: `ast.declaration_variable(Const, i_ident, ast.expression_arrow_function([], import_call))` |
| **Primary Spec Files** | `example_1.md`, `example_getter_generation.md`, `example_strip_server_code.md` |
| **Frequency** | 107/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md`, Pattern 2 (Complete Rust Code: `build_lazy_import_declaration()`) |
| **Phase Mapped** | Phase 4 |
| **Dependencies** | AFTER CONV-01 (needs detected segment data to build import paths) |

**Construction Snippet:**
```rust
// const i_HASH = () => import("./path_segment_HASH")
let import_source = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(import_path), None);
let import_expr = ctx.ast.expression_import(SPAN, import_source, ctx.ast.vec(), None);
let arrow = ctx.ast.expression_arrow_function(SPAN, true, false, false, NONE, params, NONE, body);
// ... wrap in const declaration
```

---

### CONV-07: PURE Annotations

| Field | Content |
|-------|---------|
| **CONV** | CONV-07: PURE Annotations |
| **Detection API** | Generated for ALL framework call replacements during/after construction (not detected from source) |
| **Construction API** | **Option A (preferred):** `ctx.ast.expression_call_with_pure()` if available in OXC 0.113; **Option B (fallback):** Manual `/*#__PURE__*/` comment insertion via `program.comments.push(Comment { ... })` |
| **Primary Spec Files** | All spec files with framework calls (nearly universal) |
| **Frequency** | 139/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md`, PURE Annotation Strategy section; `.planning/phases/06-secondary-patterns-cross-reference/SOURCE-MAPS-MAPPING.md`, Section 5 |
| **Phase Mapped** | Phase 4/6 (cross-cutting) |
| **Dependencies** | AFTER CONV-03 (annotations added to constructed JSX calls); AFTER CONV-01/CONV-02 (annotations on qrl/componentQrl calls) |

**Construction Snippet (Option B -- manual comment):**
```rust
// /*#__PURE__*/ qrl(...)
// After constructing the call expression, attach a leading comment:
// The exact API depends on OXC version. Comments are stored on Program
// and associated by span. Codegen outputs leading comments before the expression.
let qrl_call = ctx.ast.expression_call(SPAN, callee, NONE, arguments, false);
// program.comments.push(Comment { kind: CommentKind::Block, span, value: "#__PURE__" });
```

**Annotated calls:** `_jsxSorted(...)`, `_jsxSplit(...)`, `componentQrl(...)`, `inlinedQrl(...)`, `qrl(...)`, `_wrapProp(...)`, `_fnSignal(...)`, `_qrlSync(...)`, `_noopQrl(...)`, `_restProps(...)`

---

### CONV-08: Segment Extraction

| Field | Content |
|-------|---------|
| **CONV** | CONV-08: Segment Extraction |
| **Detection API** | `enter_call_expression` -> `$()` detection collects segment boundaries; segment body extracted via `take_in(ctx.ast)` in `exit_expression` |
| **Construction API** | `build_segment_program()`: constructs complete `Program<'a>` per segment via `AstBuilder`, including imports, lazy imports, exported const with body |
| **Primary Spec Files** | All spec files that produce separate segment files (107+ files) |
| **Frequency** | 121/162 (Universal) |
| **Detailed Mapping** | `.planning/phases/05-deep-research-proof-of-concept/MULTI-MODULE-OUTPUT-MAPPING.md`, Sections 3-4 |
| **Phase Mapped** | Phase 5 |
| **Dependencies** | AFTER CONV-01 (detection identifies boundaries); AFTER CONV-05 (captures determine segment imports) |

**Construction Snippet:**
```rust
// export const NAME_HASH = (params) => { body };
let export_stmt = build_segment_export(&segment.export_name, body_fn, &ast);
// Assemble: imports + lazy_imports + export -> Program
let program = ast.program(SPAN, SourceType::mjs(), None, ast.vec(), body);
```

---

### CONV-09: Code Stripping

| Field | Content |
|-------|---------|
| **CONV** | CONV-09: Code Stripping |
| **Detection API** | `enter_call_expression` -> check `ctx_name` (callee without `$`) against `strip_ctx_name` list via `should_strip_call()` |
| **Construction API** | Prod: `build_noop_qrl()` -> `ast.expression_call(_noopQrl_ident, vec![hash_str])`; Dev: `build_noop_qrl_dev()` -> `ast.expression_call(_noopQrlDEV_ident, vec![name_str, debug_obj])` |
| **Primary Spec Files** | `example_strip_server_code.md`, `example_drop_side_effects.md` |
| **Frequency** | 8/162 (Specialized) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md`, Sections 10-14 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | AFTER CONV-10 (const replacement creates dead branches before stripping); AFTER CONV-01 (stripping decisions depend on detected $() ctx_name) |

**Construction Snippet (Prod):**
```rust
// _noopQrl("s_HASH")
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_noopQrl"));
let hash_lit = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(hash), None);
let mut args = ctx.ast.vec_with_capacity(1);
args.push(Argument::from(hash_lit));
ctx.ast.expression_call(SPAN, callee, NONE, args, false)
```

---

### CONV-10: Const Replacement

| Field | Content |
|-------|---------|
| **CONV** | CONV-10: Const Replacement |
| **Detection API** | `enter_identifier_reference` -> check for `isServer` identifier imported from `@qwik.dev/core` |
| **Construction API** | `ctx.ast.expression_boolean_literal(SPAN, is_server_value)` + dead branch elimination via `simplify_dead_branches()` + `try_eval_const_expr()` for static expression folding |
| **Primary Spec Files** | `example_strip_server_code.md`, `example_derived_signals_cmp.md` |
| **Frequency** | 5/162 (Specialized) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md`, Sections 15-16 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | BEFORE CONV-09 (isServer evaluation creates dead branches for subsequent stripping) |

**Construction Snippet:**
```rust
// isServer -> true (server build) or false (client build)
let replacement = ctx.ast.expression_boolean_literal(SPAN, is_server_value);
// Then: if (false) { ... } -> removed by dead branch elimination
```

---

### CONV-11: Props Destructuring

| Field | Content |
|-------|---------|
| **CONV** | CONV-11: Props Destructuring |
| **Detection API** | `enter_arrow_function_expression` -> check parent is `component$()` call + first parameter is `BindingPatternKind::ObjectPattern` |
| **Construction API** | Parameter replacement: `ast.binding_pattern_kind_binding_identifier("_rawProps")`; Rest insertion: `build_rest_props_declaration()`; Reference replacement: `build_wrap_prop_named()` with `_rawProps` source |
| **Primary Spec Files** | `should_destructure_args.md` |
| **Frequency** | 5/162 (Specialized) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md`, Sections 1, 6 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | BEFORE CONV-05 (props destructuring changes variable references that capture analysis reads) |

**Construction Snippet:**
```rust
// ({message, id, count: c, ...rest}) -> (_rawProps)
let binding = ctx.ast.binding_pattern_kind_binding_identifier(SPAN, ctx.ast.atom("_rawProps"));
let pattern = ctx.ast.binding_pattern(binding, None, false);
// Then: const rest = _restProps(_rawProps, ["message", "id", "count"])
```

---

### CONV-12: Input Binding

| Field | Content |
|-------|---------|
| **CONV** | CONV-12: Input Binding |
| **Detection API** | During JSX attribute processing: check for `bind:` prefix on attribute name |
| **Construction API** | `build_bind_value_props()`: generates `"value": signal` + `"q-e:input": inlinedQrl(_val, "_val", [signal])` for `bind:value`; analogous for `bind:checked` with `_chk` handler |
| **Primary Spec Files** | `example_input_bind.md` |
| **Frequency** | 2/162 (Specialized) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md`, Section 7 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | Part of JSX processing pass (concurrent with CONV-03) |

**Construction Snippet:**
```rust
// bind:value -> "value": signal, "q-e:input": inlinedQrl(_val, "_val", [signal])
let _val_ref = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_val"));
let _val_str = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom("_val"), None);
let mut handler_args = ctx.ast.vec_with_capacity(3);
handler_args.push(Argument::from(_val_ref));
handler_args.push(Argument::from(_val_str));
handler_args.push(Argument::from(captures_array));
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("inlinedQrl"));
ctx.ast.expression_call(SPAN, callee, NONE, handler_args, false)
```

---

### CONV-13: Sync$ Serialization

| Field | Content |
|-------|---------|
| **CONV** | CONV-13: Sync$ Serialization |
| **Detection API** | `enter_call_expression` -> check callee is `sync$` identifier imported from `@qwik.dev/core` |
| **Construction API** | `build_qrl_sync_call()`: `ast.expression_call(_qrlSync_ident, vec![fn_expr, stringified_string])`; stringification via `oxc_codegen::Codegen` with `minify: true` |
| **Primary Spec Files** | `example_of_synchronous_qrl.md` |
| **Frequency** | 3/162 (Specialized) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md`, Section 8 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | Part of $() detection pass; NOT extracted to segment (stays inline) |

**Construction Snippet:**
```rust
// _qrlSync(function(event, target) { ... }, "function(event,target){...}")
let callee = ctx.ast.expression_identifier_reference(SPAN, ctx.ast.atom("_qrlSync"));
let string_literal = ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(stringified), None);
let mut args = ctx.ast.vec_with_capacity(2);
args.push(Argument::from(fn_expr));
args.push(Argument::from(string_literal));
ctx.ast.expression_call(SPAN, callee, NONE, args, false)
```

---

### CONV-14: Hoisted Functions

| Field | Content |
|-------|---------|
| **CONV** | CONV-14: Hoisted Functions |
| **Detection API** | Generated during CONV-04 signal optimization when computed expressions are converted to `_fnSignal` |
| **Construction API** | `build_hoisted_function()`: `ast.declaration_variable(Const, _hfN, arrow_fn)` + `build_hoisted_string()`: `ast.declaration_variable(Const, _hfN_str, string_literal)` |
| **Primary Spec Files** | `example_derived_signals_cmp.md`, `example_getter_generation.md` |
| **Frequency** | 17/162 (Common) |
| **Detailed Mapping** | `.planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md`, Section 4 |
| **Phase Mapped** | Phase 6 |
| **Dependencies** | AFTER CONV-04 (hoisted functions are the output of signal optimization) |

**Construction Snippet:**
```rust
// const _hf0 = (p0) => p0.nested.count;
let arrow = ctx.ast.expression_arrow_function(SPAN, true, false, false, NONE, params, NONE, body);
let declarator = ctx.ast.variable_declarator(SPAN, VariableDeclarationKind::Const, pattern, Some(arrow), false);
// const _hf0_str = "p0.nested.count";
let str_declarator = ctx.ast.variable_declarator(SPAN, VariableDeclarationKind::Const, str_pattern, Some(string_value), false);
// Insert both at module top-level, after imports, before exports
```

---

## 3. Frequency Analysis

### 3.1 Occurrence Counts Across 162 Spec Files

| CONV | Name | Count | Category |
|------|------|-------|----------|
| CONV-01 | QRL Calls | 162 | Universal |
| CONV-02 | Dollar-to-QRL Suffix | 157 | Universal |
| CONV-07 | PURE Annotations | 139 | Universal |
| CONV-08 | Segment Extraction | 121 | Universal |
| CONV-03 | JSX Transforms | 113 | Universal |
| CONV-06 | Lazy Imports | 107 | Universal |
| CONV-05 | Capture Patterns | 45 | Common |
| CONV-04 | Signal Helpers | 25 | Common |
| CONV-14 | Hoisted Functions | 17 | Common |
| CONV-09 | Code Stripping | 8 | Specialized |
| CONV-10 | Const Replacement | 5 | Specialized |
| CONV-11 | Props Destructuring | 5 | Specialized |
| CONV-13 | Sync$ Serialization | 3 | Specialized |
| CONV-12 | Input Binding | 2 | Specialized |

### 3.2 Category Definitions

**Universal (>100 occurrences):** These CONVs appear in more than 60% of spec files and represent the core transformation pipeline. Any implementation must handle these first and optimize them for performance.

- CONV-01, CONV-02, CONV-03, CONV-06, CONV-07, CONV-08

**Common (20-100 occurrences):** These CONVs appear in a significant minority of spec files. They must be implemented for correctness but are not as performance-critical.

- CONV-04, CONV-05, CONV-14

**Specialized (<20 occurrences):** These CONVs handle specific features and appear in few spec files. They can be implemented last without affecting the majority of transformations.

- CONV-09, CONV-10, CONV-11, CONV-12, CONV-13

---

## 4. Dependency Ordering

### 4.1 Constraints

The following CONV types have execution ordering dependencies:

```
CONV-11 (Props Destructuring) BEFORE CONV-05 (Capture Analysis)
  Reason: Destructuring changes variable references ({foo} -> _rawProps.foo)
          that capture analysis reads to determine what crosses $() boundaries.

CONV-01 ($() Detection) BEFORE CONV-08 (Segment Extraction)
  Reason: Detection identifies segment boundaries (which $() calls exist);
          extraction uses those boundaries to build segment Programs.

CONV-01 ($() Detection) BEFORE CONV-09 (Code Stripping)
  Reason: Stripping decisions depend on detected $() call ctx_name
          (e.g., "server" in "server$").

CONV-05 (Capture Analysis) BEFORE CONV-08 (Segment Extraction)
  Reason: Segments need capture lists to build _captures imports
          and _captures[N] restoration statements.

CONV-03 (JSX Transforms) CONCURRENT WITH CONV-04 (Signal Helpers)
  Reason: Signal helpers (_wrapProp, _fnSignal) are generated during
          JSX prop value analysis -- they are part of the same pass.

CONV-03 (JSX Transforms) BEFORE CONV-07 (PURE Annotations)
  Reason: PURE annotations are added to _jsxSorted/_jsxSplit call
          expressions during or after their construction.

CONV-10 (Const Replacement) BEFORE CONV-09 (Code Stripping)
  Reason: isServer evaluation can create additional dead code
          (if (false) { ... }) that stripping then eliminates.
```

### 4.2 Dependency Graph

```
                    CONV-11 (Props Destructuring)
                        |
                        | BEFORE
                        v
CONV-01 ($() Detection) ---+---> CONV-05 (Capture Patterns)
    |                      |         |
    | BEFORE               |         | BEFORE
    v                      |         v
CONV-02 (Dollar Suffix)   |    CONV-08 (Segment Extraction)
                           |         |
                           |         | THEN
                           |         v
                           |    CONV-06 (Lazy Imports)
                           |
                           | BEFORE
                           v
                    CONV-09 (Code Stripping)
                        ^
                        |
                        | BEFORE
                        |
                    CONV-10 (Const Replacement)

    CONV-03 (JSX) <==> CONV-04 (Signals)  [CONCURRENT]
        |                   |
        | BEFORE            | PRODUCES
        v                   v
    CONV-07 (PURE)     CONV-14 (Hoisted Fns)

    CONV-12 (Input Binding)  -- part of CONV-03 JSX pass
    CONV-13 (Sync$)          -- part of CONV-01 $() detection
```

### 4.3 Execution Tiers

Based on the dependency graph, the CONVs can be grouped into execution tiers:

| Tier | CONVs | Description |
|------|-------|-------------|
| **Tier 0** | CONV-11 | Pre-pass: Props destructuring (must run before everything else) |
| **Tier 1** | CONV-01, CONV-02, CONV-10 | Detection + const replacement (foundations) |
| **Tier 2** | CONV-03, CONV-04, CONV-05, CONV-12, CONV-13 | Main transform (JSX, signals, captures, input binding, sync$) |
| **Tier 3** | CONV-14, CONV-07 | Post-construction (hoisted functions, PURE annotations) |
| **Tier 4** | CONV-09 | Stripping (after const replacement creates dead branches) |
| **Tier 5** | CONV-08, CONV-06 | Emit (segment extraction + lazy imports) |

---

## 5. Phase Coverage Summary

### 5.1 Phase-to-CONV Mapping

| Phase | CONVs Mapped | Document |
|-------|-------------|----------|
| **Phase 4** (Core API Mapping) | CONV-01, CONV-02, CONV-06 | `API-MAPPING.md` |
| **Phase 5** (Deep Research + POC) | CONV-05, CONV-08 | `CAPTURE-ANALYSIS-MAPPING.md`, `MULTI-MODULE-OUTPUT-MAPPING.md` |
| **Phase 6** (Secondary Patterns) | CONV-03, CONV-04, CONV-07, CONV-09, CONV-10, CONV-11, CONV-12, CONV-13, CONV-14 | `JSX-TRANSFORMS-MAPPING.md`, `PROPS-SIGNALS-MAPPING.md`, `ENTRY-STRIPPING-CONST-MAPPING.md`, `SOURCE-MAPS-MAPPING.md` |

### 5.2 Coverage Verification

| # | CONV Name | Phase | Document | Status |
|---|-----------|-------|----------|--------|
| 1 | QRL Calls | 4 | API-MAPPING.md Pattern 1, 2 | MAPPED |
| 2 | Dollar-to-QRL Suffix | 4 | API-MAPPING.md Pattern 1 | MAPPED |
| 3 | JSX Transforms | 6 | JSX-TRANSFORMS-MAPPING.md | MAPPED |
| 4 | Signal Helpers | 6 | PROPS-SIGNALS-MAPPING.md | MAPPED |
| 5 | Capture Patterns | 5 | CAPTURE-ANALYSIS-MAPPING.md | MAPPED |
| 6 | Lazy Imports | 4 | API-MAPPING.md Pattern 2 | MAPPED |
| 7 | PURE Annotations | 4/6 | API-MAPPING.md + SOURCE-MAPS-MAPPING.md | MAPPED |
| 8 | Segment Extraction | 5 | MULTI-MODULE-OUTPUT-MAPPING.md | MAPPED |
| 9 | Code Stripping | 6 | ENTRY-STRIPPING-CONST-MAPPING.md | MAPPED |
| 10 | Const Replacement | 6 | ENTRY-STRIPPING-CONST-MAPPING.md | MAPPED |
| 11 | Props Destructuring | 6 | PROPS-SIGNALS-MAPPING.md | MAPPED |
| 12 | Input Binding | 6 | ENTRY-STRIPPING-CONST-MAPPING.md | MAPPED |
| 13 | Sync$ Serialization | 6 | ENTRY-STRIPPING-CONST-MAPPING.md | MAPPED |
| 14 | Hoisted Functions | 6 | PROPS-SIGNALS-MAPPING.md | MAPPED |

**All 14 CONVs are mapped. Zero gaps.**

### 5.3 Mapping Document Index

| Document | Path | CONVs Covered | Phase |
|----------|------|---------------|-------|
| API-MAPPING.md | `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md` | CONV-01, CONV-02, CONV-06, CONV-07 (partial) | 4 |
| CAPTURE-ANALYSIS-MAPPING.md | `.planning/phases/05-deep-research-proof-of-concept/CAPTURE-ANALYSIS-MAPPING.md` | CONV-05 | 5 |
| MULTI-MODULE-OUTPUT-MAPPING.md | `.planning/phases/05-deep-research-proof-of-concept/MULTI-MODULE-OUTPUT-MAPPING.md` | CONV-08 | 5 |
| JSX-TRANSFORMS-MAPPING.md | `.planning/phases/06-secondary-patterns-cross-reference/JSX-TRANSFORMS-MAPPING.md` | CONV-03 | 6 |
| PROPS-SIGNALS-MAPPING.md | `.planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md` | CONV-04, CONV-11, CONV-14 | 6 |
| ENTRY-STRIPPING-CONST-MAPPING.md | `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md` | CONV-09, CONV-10, CONV-12, CONV-13 | 6 |
| SOURCE-MAPS-MAPPING.md | `.planning/phases/06-secondary-patterns-cross-reference/SOURCE-MAPS-MAPPING.md` | CONV-07 (complete) | 6 |

---

## 6. Implementation Roadmap

### 6.1 Recommended Implementation Order for v3.0 (OXC Port)

Based on dependency ordering, frequency analysis, and practical considerations:

**Step 1: CONV-01 + CONV-02 (Universal, Foundation)**
- `$()` detection and dollar-to-QRL naming
- These are the foundation everything else builds on
- 162/162 and 157/162 occurrence rate
- Phase 4 API-MAPPING.md provides complete Rust code

**Step 2: CONV-11 (Must Precede Captures)**
- Props destructuring transform (`_rawProps`, `_restProps`)
- Only 5/162 occurrence but must run as a pre-pass before capture analysis
- PROPS-SIGNALS-MAPPING.md Section 1

**Step 3: CONV-05 (Must Precede Extraction)**
- Capture analysis using `oxc_semantic::Scoping`
- 45/162 occurrence rate; critical for correctness
- CAPTURE-ANALYSIS-MAPPING.md provides complete algorithm

**Step 4: CONV-08 + CONV-06 (Segment Extraction + Lazy Imports)**
- Build segment Programs via AstBuilder, generate lazy import declarations
- 121/162 and 107/162 occurrence rates
- MULTI-MODULE-OUTPUT-MAPPING.md + API-MAPPING.md Pattern 2

**Step 5: CONV-03 + CONV-04 + CONV-14 (JSX + Signals + Hoisted Functions)**
- JSX transformation, signal helper generation, hoisted function placement
- These three are tightly coupled and should be implemented together
- 113/162, 25/162, 17/162 occurrence rates
- JSX-TRANSFORMS-MAPPING.md + PROPS-SIGNALS-MAPPING.md Sections 2-4

**Step 6: CONV-07 (PURE Annotations)**
- Add `/*#__PURE__*/` comments to all framework replacement calls
- 139/162 occurrence rate; must run after all call construction is complete
- API-MAPPING.md PURE section + SOURCE-MAPS-MAPPING.md Section 5

**Step 7: CONV-10 (Const Folding)**
- `isServer` replacement and dead branch elimination
- 5/162 occurrence rate but must precede stripping
- ENTRY-STRIPPING-CONST-MAPPING.md Sections 15-16

**Step 8: CONV-09 (Code Stripping)**
- Replace stripped `$()` calls with `_noopQrl()`, dead declaration removal
- 8/162 occurrence rate; runs after const folding
- ENTRY-STRIPPING-CONST-MAPPING.md Sections 10-14

**Step 9: CONV-12 + CONV-13 (Specialized, Last)**
- Input binding (`bind:value`, `bind:checked`) and sync$ serialization
- 2/162 and 3/162 occurrence rates -- lowest priority
- ENTRY-STRIPPING-CONST-MAPPING.md Sections 7-8

### 6.2 Implementation Milestones

| Milestone | CONVs | Spec File Coverage |
|-----------|-------|--------------------|
| **M1: Basic extraction** | CONV-01, CONV-02, CONV-06, CONV-08 | ~107/162 (66%) |
| **M2: Capture analysis** | + CONV-05, CONV-11 | ~112/162 (69%) |
| **M3: JSX transforms** | + CONV-03, CONV-04, CONV-14, CONV-07 | ~152/162 (94%) |
| **M4: Stripping + const** | + CONV-09, CONV-10 | ~157/162 (97%) |
| **M5: Full coverage** | + CONV-12, CONV-13 | 162/162 (100%) |

---

*This document satisfies APIM-08: All 14 CONV types cross-referenced to specific OXC API patterns with detection APIs, construction APIs, Rust code snippets, spec file references, frequency analysis, dependency ordering, phase coverage, and implementation roadmap. Zero gaps across all 162 spec files.*
