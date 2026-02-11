# Source Map Strategy & PURE Annotation Mechanism: OXC API Mapping (APIM-07)

**Date:** 2026-02-11
**OXC Version:** 0.113.0
**Purpose:** A standalone, implementable specification for source map generation across all optimizer-produced modules and the PURE annotation insertion mechanism (CONV-07). A developer reading this document knows the exact span strategy for every AST node type the optimizer constructs or moves, the complete pipeline for generating source maps per segment and main module, the file naming conventions, and how to attach `/*#__PURE__*/` annotations to framework replacement calls.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-07 | Sections 1-6 | Source map generation mapped to oxc_codegen + oxc_sourcemap APIs |
| CONV-07 | Section 5 | PURE annotation insertion mechanism with two implementation options |

---

## 1. Source Map Strategy Overview

### 1.1 Validated Approach (from Phase 5 POC-04)

Phase 5 validated that OXC's codegen pipeline generates correct source maps when:

1. **Preserved spans** are maintained on AST nodes extracted/moved from original source
2. **Zero spans** (`SPAN`) are used on nodes purely constructed by the optimizer
3. **Program span** encompasses all preserved child spans
4. **Source text** is provided to the codegen as the original source content

The core API call:

```rust
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};

let codegen = Codegen::new()
    .with_options(CodegenOptions {
        source_map_path: Some(source_map_path.into()),
        ..Default::default()
    })
    .with_source_text(original_source);

let codegen_result: CodegenReturn = codegen.build(&program);
// codegen_result.code: String    -- generated JavaScript
// codegen_result.map: Option<SourceMap>  -- source map (present when source_map_path is set)
```

### 1.2 Core Principle

**Preserve spans on AST nodes that were extracted/moved from the original source; use `SPAN` (zero span) on nodes that are purely constructed by the optimizer.**

This principle ensures:
- Extracted code (function bodies, expressions) maps back to the original source location
- Constructed wrapper code (imports, QRL calls, `_jsxSorted` wrappers) produces no misleading source map entries
- Debuggers show the original source for business logic while skipping optimizer-generated boilerplate

### 1.3 Key Decisions from Phase 5

| Decision | Source | Implication |
|----------|--------|-------------|
| Program span must encompass all preserved child spans | POC-04 | Use `Span::new(0, source.len() as u32)` for segment Programs |
| Segments with preserved spans require original source text | POC-04 | Pass original source to `.with_source_text()`, not segment text |
| Source maps generated per module | POC-04 | Main module + each segment module gets its own `.map` file |
| SPAN-only nodes produce zero source map entries | POC-04 | Constructed nodes (imports, wrappers) do not pollute mappings |

---

## 2. Complete Span Strategy Table

This table documents the span assignment for **every** node type the optimizer constructs or moves, organized by CONV type and phase origin.

### 2.1 Phase 4 Nodes (CONV-01, CONV-02, CONV-06)

| Node | Span | Rationale |
|------|------|-----------|
| `qrl()` call expression | SPAN (zero) | Constructed replacement for `$()` call |
| `inlinedQrl()` call expression | SPAN (zero) | Constructed replacement for `$()` call |
| `componentQrl` identifier reference | SPAN (zero) | Constructed replacement for `component$` callee |
| `useStylesQrl` (and other `Qrl`-suffixed) identifier | SPAN (zero) | Constructed replacement for dollar-suffixed callee |
| `qrl` / `inlinedQrl` callee identifier | SPAN (zero) | Constructed framework identifier |
| Import argument string literal (`"segment_name_HASH"`) | SPAN (zero) | Constructed hash string |
| Lazy import `const i_HASH = () => import(...)` declaration | SPAN (zero) | Entirely constructed declaration |
| Arrow function in lazy import | SPAN (zero) | Constructed import wrapper |
| `import()` expression inside lazy import | SPAN (zero) | Constructed dynamic import |
| New `import { qrl } from "@qwik.dev/core"` declaration | SPAN (zero) | Constructed import statement |
| New `import { componentQrl } from "@qwik.dev/core"` declaration | SPAN (zero) | Constructed import statement |
| Captures array `[state, count]` in `qrl()` / `inlinedQrl()` | SPAN (zero) | Constructed captures argument |
| `_captures[N]` member expression | SPAN (zero) | Constructed capture restoration |
| `const state = _captures[0]` declaration in segment body | SPAN (zero) | Constructed capture declaration |

### 2.2 Phase 5 Nodes (CONV-05, CONV-08)

| Node | Span | Rationale |
|------|------|-----------|
| Segment `Program` node | `Span::new(0, source.len())` | Must encompass all preserved child spans |
| Segment `export const NAME = ...` declaration wrapper | SPAN (zero) | Constructed export wrapper |
| Segment `export` keyword | SPAN (zero) | Constructed |
| Segment `const` binding identifier (`NAME_HASH`) | SPAN (zero) | Constructed identifier |
| Body statements inside segment (moved from source) | **Preserve original** | Extracted from original source via `take_in()` |
| Arrow function parameters inside segment | **Preserve original** | Extracted from original `$()` callback |
| Arrow function body inside segment | **Preserve original** | Extracted from original `$()` callback |
| Re-emitted import declarations in segment | SPAN (zero) | Constructed imports for segment module |
| Lazy import declarations in segment (for nested $-calls) | SPAN (zero) | Constructed |

### 2.3 Phase 6 Nodes (CONV-03, CONV-04, CONV-09, CONV-10, CONV-11, CONV-12, CONV-13, CONV-14)

| Node | Span | Rationale |
|------|------|-----------|
| `_jsxSorted(...)` call expression | SPAN (zero) | Constructed JSX replacement |
| `_jsxSplit(...)` call expression | SPAN (zero) | Constructed JSX replacement for spread |
| `_jsxSorted` / `_jsxSplit` callee identifier | SPAN (zero) | Constructed framework identifier |
| Props objects (varProps ObjectExpression) | SPAN (zero) | Newly constructed objects |
| Props objects (constProps ObjectExpression) | SPAN (zero) | Newly constructed objects |
| Original prop VALUE expressions | **Preserve original** | Extracted from JSX attributes |
| Property keys in props objects | SPAN (zero) | Constructed (even if same name as original) |
| `_wrapProp(...)` call expression | SPAN (zero) | Constructed wrapper |
| `_wrapProp` callee identifier | SPAN (zero) | Constructed framework identifier |
| `_fnSignal(...)` call expression | SPAN (zero) | Constructed wrapper |
| `_fnSignal` callee identifier | SPAN (zero) | Constructed framework identifier |
| Hoisted function body expression (`_hfN`) | **Preserve original** | Expression extracted from prop value |
| Hoisted function parameter `p0` | SPAN (zero) | Constructed parameter |
| Hoisted function string `_hfN_str` | SPAN (zero) | Constructed minified string |
| Hoisted function `const _hfN = ...` declaration | SPAN (zero) | Constructed declaration |
| `_noopQrl(...)` call expression | SPAN (zero) | Constructed replacement for stripped code |
| `_noopQrlDEV(...)` call expression | SPAN (zero) | Constructed replacement for stripped code (dev) |
| `_qrlSync(...)` call expression | SPAN (zero) | Constructed replacement for `sync$()` |
| Sync function body inside `_qrlSync` | **Preserve original** | Kept inline from `sync$()` callback |
| Stringified function argument in `_qrlSync` | SPAN (zero) | Constructed string literal |
| `isServer` -> `true`/`false` replacement | SPAN (zero) | Constructed boolean literal |
| `_rawProps` binding identifier | SPAN (zero) | Constructed parameter replacement |
| `_restProps(...)` call expression | SPAN (zero) | Constructed rest-props extraction |
| `_restProps` callee identifier | SPAN (zero) | Constructed framework identifier |
| `_val`/`_chk` handler wrapper in `inlinedQrl()` | SPAN (zero) | Constructed bind directive handler |
| Children array expression | SPAN (zero) | Constructed children encoding |
| Individual child expressions (text, expressions) | **Preserve original** | Extracted from JSX children |
| Flags numeric literal | SPAN (zero) | Constructed flag value |
| Key string literal | SPAN (zero) | Constructed key value |
| Dev info object `{fileName, lineNumber, columnNumber}` | SPAN (zero) | Constructed dev metadata |
| Debug object `{file, lo, hi, displayName}` | SPAN (zero) | Constructed debug metadata |
| `_Fragment` identifier reference | SPAN (zero) | Constructed fragment identifier |
| `Fragment as _Fragment` import declaration | SPAN (zero) | Constructed import |
| `_getVarProps(...)` / `_getConstProps(...)` calls | SPAN (zero) | Constructed spread helpers |
| Dead branch removal (entire `if` statement) | N/A | Removed from AST, no span needed |
| `throw "Symbol removed..."` replacement body | SPAN (zero) | Constructed throw statement |

### 2.4 Summary Statistics

| Category | Count | Span Type |
|----------|-------|-----------|
| Constructed nodes (use SPAN) | 42 | Zero span -- no source map entry |
| Extracted/moved nodes (preserve original) | 7 | Original span -- maps to source |
| Removed nodes | 1 | N/A |
| **Total** | **50** | |

---

## 3. Segment Source Map Construction

### 3.1 Pipeline Overview

When the optimizer extracts a `$()` body into a segment module, it must generate a source map that maps the extracted code back to its position in the original source file.

**4-Step Pipeline:**

**Step 1: Collect preserved-span nodes from the original source**

During the transform pass, when extracting a `$()` body via `take_in(ctx.ast)`, the moved AST nodes retain their original `Span` values. These spans reference byte offsets in the original source file.

```rust
// In exit_expression, when replacing $() with qrl():
let body_expr = call.arguments[0].take_in(ctx.ast);
// body_expr retains original spans: Span { start: 45, end: 120 }
// These spans reference positions in the original source text
```

**Step 2: Build segment Program with encompassing span and original source text**

The segment `Program` must have a span that encompasses ALL preserved child spans. Use `Span::new(0, source_len)` where `source_len` is the byte length of the original source file.

```rust
use oxc_span::{Span, SourceType};

let program_span = Span::new(0, original_source.len() as u32);

let segment_program = ast.program(
    program_span,           // encompasses all preserved child spans
    SourceType::mjs(),      // segments are always JS modules
    None,                   // no hashbang
    ast.vec(),              // no directives
    body,                   // program body (imports + export with preserved-span body)
);
```

**Step 3: Run codegen with `source_map_path` option**

```rust
use oxc_codegen::{Codegen, CodegenOptions};
use std::path::PathBuf;

let options = CodegenOptions {
    source_map_path: Some(PathBuf::from(output_filename)),
    ..CodegenOptions::default()
};

let codegen_result = Codegen::new()
    .with_options(options)
    .with_source_text(original_source)  // MUST be original source, not segment text
    .build(&segment_program);
```

**Step 4: Extract and serialize the source map**

```rust
if let Some(source_map) = codegen_result.map {
    // Serialize to JSON string for .map file
    let json_string: String = source_map.to_json_string();

    // Or get structured JSON for further processing
    let json_value: oxc_sourcemap::JSONSourceMap = source_map.to_json();
}
```

### 3.2 Critical Constraints

1. **Program span MUST encompass preserved spans**: If a segment body contains a node with `Span { start: 45, end: 120 }`, the Program's span must be at least `Span::new(0, 120)`. Using `Span::new(0, source.len())` is the safe default.

2. **Source text MUST be original**: The `.with_source_text()` call must receive the original source file's content, not the generated segment code. This is because the preserved spans reference byte offsets in the original file.

3. **`source_map_path` determines the `"file"` field**: The path passed to `source_map_path` appears as the `"file"` field in the generated source map JSON. It should match the segment's output `.js` filename.

### 3.3 Source Map Output Structure

The generated source map JSON follows the standard v3 format:

```json
{
    "version": 3,
    "file": "test.tsx_App_component_HASH.js",
    "sourceRoot": "",
    "sources": ["test.tsx_App_component_HASH.js"],
    "sourcesContent": ["<original source text>"],
    "names": [],
    "mappings": "AAAA;AACA;AACA;..."
}
```

The `mappings` field contains VLQ-encoded position mappings. Nodes with preserved spans produce mapping entries; nodes with `SPAN` (zero) produce none.

### 3.4 Reference Implementation

See Phase 5 POC-04 (`poc/src/poc_04_source_maps.rs`) for a validated working example that demonstrates:
- Main module source map: 181-char mappings (many preserved spans)
- SPAN-only segment: 0-char mappings (all constructed, no entries)
- Preserved-span segment: 26-char mappings (body nodes map to original positions)

---

## 4. Main Module Source Map

### 4.1 Approach

The main (transformed) module retains the original `Program`'s span. Many nodes in the main module are preserved from the original source (unchanged declarations, kept imports, non-dollar identifiers), while replacement nodes use `SPAN`.

```rust
// Main module codegen
let main_result = Codegen::new()
    .with_options(CodegenOptions {
        source_map_path: Some(PathBuf::from(main_filename)),
        ..CodegenOptions::default()
    })
    .with_source_text(original_source)
    .build(&program);  // The mutated original program
```

### 4.2 Span Behavior in Main Module

| Node Type | Span | Source Map Effect |
|-----------|------|-------------------|
| Original import declarations (unchanged) | Preserved | Maps to original position |
| Original variable declarations (unchanged) | Preserved | Maps to original position |
| Original identifier references (unchanged) | Preserved | Maps to original position |
| Replaced `$()` calls -> `qrl()` / `inlinedQrl()` | SPAN (zero) | No mapping entry |
| Replaced `component$` -> `componentQrl` | SPAN (zero) | No mapping entry |
| New `import { qrl }` declarations | SPAN (zero) | No mapping entry |
| Lazy import `const i_HASH = ...` | SPAN (zero) | No mapping entry |
| `/*#__PURE__*/` comments | N/A (comments) | No mapping entry |

### 4.3 Pipeline Position

The main module source map is generated AFTER all transformations are complete, in the same allocator scope:

```rust
// 1. Parse original source -> program
// 2. Run semantic analysis
// 3. Transform pass (replaces $() calls, rewrites imports, etc.)
// 4. Codegen main module with source map  <-- HERE
// 5. Build and codegen each segment
// 6. Return all outputs as String
```

---

## 5. PURE Annotation Mechanism (CONV-07)

### 5.1 Overview

The `/*#__PURE__*/` annotation tells JavaScript bundlers (webpack, Rollup, esbuild) that a function call has no side effects and can be tree-shaken if its return value is unused. The Qwik optimizer applies this annotation to all framework replacement calls.

Frequency: **139/162 spec files** -- nearly universal. Every spec file that produces framework calls (`qrl()`, `componentQrl()`, `_jsxSorted()`, etc.) has PURE annotations.

### 5.2 Annotated Calls (Complete List)

| Call Expression | When Annotated | Example |
|----------------|---------------|---------|
| `qrl(...)` | Always (segment strategy) | `/*#__PURE__*/ qrl(i_HASH, "NAME")` |
| `qrlDEV(...)` | Always (dev mode, segment strategy) | `/*#__PURE__*/ qrlDEV(i_HASH, "NAME", debug)` |
| `inlinedQrl(...)` | Always (inline/hoist strategy) | `/*#__PURE__*/ inlinedQrl(body, "NAME")` |
| `componentQrl(...)` | Always (wraps component$) | `/*#__PURE__*/ componentQrl(qrl_call)` |
| `useStylesQrl(...)` | Always (wraps useStyles$) | `/*#__PURE__*/ useStylesQrl(qrl_call)` |
| Other `*Qrl(...)` calls | Always (wraps any $-suffixed) | `/*#__PURE__*/ useBrowserVisibleTaskQrl(...)` |
| `_jsxSorted(...)` | Always | `/*#__PURE__*/ _jsxSorted(tag, ...)` |
| `_jsxSplit(...)` | Always (spread elements) | `/*#__PURE__*/ _jsxSplit(tag, ...)` |
| `_noopQrl(...)` | Always (stripped code, prod) | `/*#__PURE__*/ _noopQrl("s_HASH")` |
| `_noopQrlDEV(...)` | Always (stripped code, dev) | `/*#__PURE__*/ _noopQrlDEV("NAME", debug)` |

**NOT annotated:** `_wrapProp(...)`, `_fnSignal(...)`, `_restProps(...)`, `_qrlSync(...)`, `_getVarProps(...)`, `_getConstProps(...)` -- these are runtime-essential calls that cannot be tree-shaken.

### 5.3 Option A: OXC Built-in PURE Support (Preferred)

If OXC's `AstBuilder` or `Codegen` supports PURE annotation natively:

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a PURE-annotated call expression using OXC's built-in support.
///
/// This approach relies on OXC's AstBuilder having a method or flag
/// that causes Codegen to emit /*#__PURE__*/ before the call.
///
/// Check OXC 0.113 for:
/// - CallExpression.pure field
/// - AstBuilder::expression_call_with_pure() method
/// - Codegen annotation support via CommentKind
pub fn build_pure_call<'a>(
    callee: Expression<'a>,
    arguments: oxc_allocator::Vec<'a, Argument<'a>>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // If OXC has a pure flag on CallExpression:
    // let call = ctx.ast.expression_call(SPAN, callee, NONE, arguments, false);
    // call.set_pure(true);  // hypothetical API
    // return call;

    // If OXC has expression_call_with_pure:
    // return ctx.ast.expression_call_with_pure(SPAN, callee, NONE, arguments, false);

    // Verify by generating a call and checking Codegen output for /*#__PURE__*/
    todo!("Check OXC 0.113 API for built-in PURE support")
}
```

### 5.4 Option B: Manual Comment Attachment (Fallback)

If OXC does not have built-in PURE support, attach a leading comment manually via the Program's comment list:

```rust
use oxc_ast::ast::*;
use oxc_ast::Comment;
use oxc_span::{Span, SPAN};

/// Attach a /*#__PURE__*/ leading comment to a call expression.
///
/// In OXC 0.113, comments are stored on the Program and associated
/// with AST nodes by their span position. The Codegen emits leading
/// comments that appear before a node's span start.
///
/// This function:
/// 1. Builds the call expression normally
/// 2. Records the call's span position
/// 3. Adds a Block comment "#__PURE__" positioned just before the call
///
/// Arguments:
/// - program_comments: Mutable reference to the Program's comments vec
/// - call_span_start: The byte offset where the call expression starts
///                    in the generated output (or a synthetic position)
pub fn attach_pure_annotation(
    program_comments: &mut Vec<Comment>,
    call_span_start: u32,
) {
    // The comment span should be positioned just before the call expression.
    // OXC's Codegen looks for comments whose span.end <= node.span.start
    // and emits them as leading comments.
    let comment = Comment {
        kind: CommentKind::Block,
        span: Span::new(call_span_start, call_span_start),
        // The content between /* and */ -- OXC adds the delimiters
    };
    program_comments.push(comment);
}

/// Alternative: Build call expression at a synthetic span position
/// and add the PURE comment referencing that position.
///
/// This approach uses a non-zero synthetic span on the call expression
/// so that the comment can be associated with it. The span does not
/// correspond to any position in the original source (it is a constructed
/// node), but it provides an anchor for the comment.
pub fn build_call_with_pure_comment<'a>(
    callee: Expression<'a>,
    arguments: oxc_allocator::Vec<'a, Argument<'a>>,
    pure_comment_position: u32,
    program: &mut Program<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Use a synthetic span at the given position
    let call_span = Span::new(pure_comment_position, pure_comment_position);
    let call = ctx.ast.expression_call(call_span, callee, NONE, arguments, false);

    // Add the PURE comment
    attach_pure_annotation(&mut program.comments, pure_comment_position);

    call
}
```

### 5.5 Recommendation

**Start with Option A**: Check if OXC 0.113 has any of:
- `CallExpression` field for PURE annotation
- `AstBuilder::expression_call` variant that sets PURE flag
- Comment annotation that Codegen recognizes for `/*#__PURE__*/` emission

**Verify**: Build a test program with a PURE-annotated call, run Codegen, check if `/*#__PURE__*/` appears in the output.

**Fall back to Option B** if Option A does not produce the annotation. The manual comment approach is guaranteed to work as long as OXC's Codegen emits leading block comments.

This was identified as an open question in Phase 4 (04-01 decision) and remains open for resolution during v3.0 implementation.

### 5.6 Evidence from Spec Files

Every spec file output shows `/*#__PURE__*/` before framework calls:

```javascript
// example_1.md:
export const renderHeader = /*#__PURE__*/ qrl(i_zBbHWn4e8Cg, "renderHeader_zBbHWn4e8Cg");

// example_functional_component.md:
const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));

// example_derived_signals_cmp.md:
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(App_component_ckEPmXZlub0, "App_component_ckEPmXZlub0"));

// example_strip_server_code.md:
serverStuffQrl(/*#__PURE__*/ _noopQrl("s_r1qAHX7Opp0"));
```

---

## 6. Source Map File Naming

### 6.1 Naming Convention

| Module Type | JS Output File | Source Map File |
|-------------|---------------|-----------------|
| Main module | `{filename}` (e.g., `test.tsx`) | `{filename}.map` (e.g., `test.tsx.map`) |
| Segment module | `{filename}_{segment_name}_{hash}.js` | `{filename}_{segment_name}_{hash}.js.map` |

### 6.2 Examples

| Input File | Segment | Output JS | Output Map |
|-----------|---------|-----------|------------|
| `test.tsx` | (main) | `test.tsx` | `test.tsx.map` |
| `test.tsx` | `renderHeader_zBbHWn4e8Cg` | `test.tsx_renderHeader_zBbHWn4e8Cg.js` | `test.tsx_renderHeader_zBbHWn4e8Cg.js.map` |
| `test.tsx` | `App_component_ckEPmXZlub0` | `test.tsx_App_component_ckEPmXZlub0.js` | `test.tsx_App_component_ckEPmXZlub0.js.map` |
| `test.tsx` | Prod: `s_ckEPmXZlub0` | `test.tsx_s_ckEPmXZlub0.js` | `test.tsx_s_ckEPmXZlub0.js.map` |

### 6.3 source_map_path Configuration

The `source_map_path` passed to `CodegenOptions` should match the output `.js` filename. This value appears as the `"file"` field in the generated source map JSON:

```rust
let output_filename = format!("{}_{}.js", filename, segment_name);
let options = CodegenOptions {
    source_map_path: Some(PathBuf::from(&output_filename)),
    ..CodegenOptions::default()
};
```

### 6.4 Inline Source Maps (Optional)

For development builds, source maps can be inlined as a base64-encoded data URL in the generated JavaScript:

```rust
let map_json = source_map.to_json_string();
let encoded = base64::engine::general_purpose::STANDARD.encode(map_json.as_bytes());
let inline_comment = format!(
    "\n//# sourceMappingURL=data:application/json;base64,{}",
    encoded
);
let code_with_inline_map = format!("{}{}", codegen_result.code, inline_comment);
```

For production builds, use external `.map` files with a `//# sourceMappingURL=` comment pointing to the file path.

---

## 7. Import Management

No new imports are required for source map generation. The source map pipeline is entirely handled by the codegen infrastructure:

- `oxc_codegen::Codegen` -- code generation with source map support
- `oxc_codegen::CodegenOptions` -- configuration including `source_map_path`
- `oxc_codegen::CodegenReturn` -- result containing `code` and optional `map`
- `oxc_sourcemap::SourceMap` -- source map type with `to_json_string()` serialization

For PURE annotations, no runtime imports are needed. The `/*#__PURE__*/` annotation is a comment (not a function call) and requires no framework import.

---

## 8. Cross-References

| Topic | Document | Relevance |
|-------|----------|-----------|
| Phase 5 POC-04 validation | `05-03-SUMMARY.md` | Validated span preservation strategy with working Rust code |
| Codegen pipeline architecture | `MULTI-MODULE-OUTPUT-MAPPING.md` Section 6 | Main module and segment codegen pipeline |
| Allocator lifetime rules | `API-MAPPING.md` Arena Allocator Lifetime Rules | All codegen must happen in allocator scope |
| PURE annotation open question | `API-MAPPING.md` PURE Annotation Strategy | Phase 4 decision to verify during implementation |
| Segment hash computation | `MULTI-MODULE-OUTPUT-MAPPING.md` Section 5 | Hash values used in file naming |
| AstBuilder construction helpers | `MULTI-MODULE-OUTPUT-MAPPING.md` Section 4 | `build_segment_program()` and related helpers |

---

*This document satisfies APIM-07: Source map generation mapped to oxc_codegen + oxc_sourcemap APIs with complete span strategy table for all node types (50 entries across Phase 4/5/6), PURE annotation mechanism (CONV-07) with two implementation options (built-in and manual comment), segment and main module source map pipelines (4-step process), file naming conventions, and cross-references to all related mapping documents.*
