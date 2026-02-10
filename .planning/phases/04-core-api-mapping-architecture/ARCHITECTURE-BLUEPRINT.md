# Architecture Blueprint: qwik-optimizer-oxc

**Date:** 2026-02-10
**Purpose:** Complete structural reference for the qwik-optimizer-oxc crate. A developer reading this document can create the crate skeleton, understand all module boundaries and dependencies, define all public types, trace the data flow, set up the test harness, and have a working Cargo.toml -- without reverse-engineering the SWC optimizer.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| ARCH-01 | Section 1: Crate Module Layout | All 16 modules with dependency ordering |
| ARCH-02 | Section 2: Public API Design | Complete Rust type definitions with serde annotations |
| ARCH-03 | Section 3: Data Flow Specification | Pipeline with type signatures at every stage |
| ARCH-04 | Section 4: Test Harness Design | Spec-based snapshot testing with insta |
| ARCH-05 | Section 5: Cargo.toml Specification | Complete, copy-pasteable Cargo.toml |

**Crate name:** `qwik-optimizer-oxc`

---

## Section 1: Crate Module Layout (ARCH-01)

### Module Inventory

The crate consists of 16 source modules. Each module is documented with its file path, purpose, public exports, dependencies on other modules, and estimated implementation complexity.

---

#### 1. `src/lib.rs` -- Public API Entry Point

**Purpose:** Crate root. Re-exports the public API (`transform_modules`, all public types) and wires together the internal modules. This is the only module external consumers import from.

**Public exports:**
- `pub fn transform_modules(config: TransformModulesOptions) -> Result<TransformOutput, anyhow::Error>`
- Re-exports all public types from `types.rs`

**Dependencies:**
- `types` -- all public type definitions
- `errors` -- Diagnostic creation
- `parse` -- module parsing
- `collector` -- first-pass analysis
- `transform` -- main QwikTransform traversal
- `code_move` -- segment extraction
- `entry_strategy` -- strategy application
- `emit` -- code generation
- `filter_exports` -- export stripping
- `rayon` -- parallel iteration over input modules

**Estimated complexity:** Medium -- orchestration logic that calls each pipeline stage per input module, aggregates results, and handles parallel execution via rayon.

---

#### 2. `src/types.rs` -- Type Definitions

**Purpose:** All public and internal type definitions. This is a pure data module with no logic -- only structs, enums, derives, and serde attributes. Separating types into their own module prevents circular dependencies since every other module can import from `types` without importing logic.

**Public exports:**
- `TransformModulesOptions` -- top-level input configuration
- `TransformModuleInput` -- single input file representation
- `TransformOutput` -- complete result type
- `TransformModule` -- individual module output (main or segment)
- `SegmentAnalysis` -- metadata about an extracted segment
- `EntryStrategy` -- enum (Segment, Inline, Hoist, Single, Component, Smart, Hook)
- `MinifyMode` -- enum (Simplify, None)
- `EmitMode` -- enum (Lib, Prod, Dev)
- `CtxKind` -- enum (EventHandler, Function)
- `Diagnostic` -- error/warning type
- `DiagnosticCategory` -- enum (Error, Warning, SourceError)
- `SourceLocation` -- line/column position

**Internal exports (pub(crate)):**
- `CollectResult` -- output of the collector pass
- `DollarCallSite` -- recorded $-call site with span info
- `ImportInfo` -- recorded import declaration
- `ExportInfo` -- recorded export declaration
- `SegmentData` -- intermediate segment representation during transform
- `TransformOptions` -- per-module options derived from TransformModulesOptions

**Dependencies:** None (leaf module -- pure data definitions)

**Estimated complexity:** Simple -- struct/enum definitions only, no logic.

---

#### 3. `src/errors.rs` -- Diagnostic Types and Error Helpers

**Purpose:** Helper functions for creating `Diagnostic` values with consistent formatting. Centralizes error message templates so the rest of the crate can report errors without constructing Diagnostic structs manually.

**Public exports (pub(crate)):**
- `fn create_error(message: &str, span: Span, source: &str) -> Diagnostic`
- `fn create_warning(message: &str, span: Span, source: &str) -> Diagnostic`
- `fn create_source_error(message: &str, span: Span, source: &str) -> Diagnostic`

**Dependencies:**
- `types` -- `Diagnostic`, `DiagnosticCategory`, `SourceLocation`

**Estimated complexity:** Simple -- formatting helpers only.

---

#### 4. `src/words.rs` -- String Constants

**Purpose:** Centralized string constants used across the crate. Prevents typos and provides a single location to update package names or API lists.

**Public exports (pub(crate)):**
- `const BUILDER_IO_QWIK: &str` -- the core module path `"@qwik.dev/core"`
- `const QWIK_CORE_ID: &str` -- alternate import path
- `fn is_known_dollar_api(name: &str) -> bool` -- check if a name is a known $-suffixed Qwik API
- `fn dollar_to_qrl_name(name: &str) -> String` -- convert `component$` to `componentQrl`
- `const KNOWN_DOLLAR_APIS: &[&str]` -- list: `["$", "component$", "useTask$", "useVisibleTask$", "useBrowserVisibleTask$", "useStyles$", "useStylesScoped$", "useOnDocument$", "useOnWindow$", "useOn$", "event$", "eventQrl"]`

**Dependencies:** None (leaf module -- pure constants)

**Estimated complexity:** Simple -- constants and one string manipulation function.

---

#### 5. `src/hash.rs` -- Segment Hash Computation

**Purpose:** Compute the 11-character hash that appears in segment names (e.g., `zBbHWn4e8Cg` in `renderHeader_zBbHWn4e8Cg`). The hash is derived from the segment's display name. This must produce identical hashes to the SWC optimizer for spec compatibility.

**Public exports (pub(crate)):**
- `fn compute_segment_hash(display_name: &str) -> String` -- returns 11-char base64url hash
- `fn format_segment_name(display_name: &str, hash: &str) -> String` -- returns `"{display_name}_{hash}"`

**Dependencies:** None (leaf module -- pure computation, uses `base64` crate)

**Estimated complexity:** Medium -- requires porting the exact hash algorithm from the SWC optimizer to ensure spec file compatibility. The algorithm itself is a pure function (hash + base64 encode), but correctness is critical.

---

#### 6. `src/parse.rs` -- Module Parsing

**Purpose:** Parse a single source file (JS/TS/JSX/TSX) into an OXC `Program` AST. Handles source type detection from filename extension and reports parse errors as `Diagnostic` values.

**Public exports (pub(crate)):**
- `fn parse_module<'a>(allocator: &'a Allocator, source: &str, filename: &str) -> Result<ParseResult<'a>, Vec<Diagnostic>>`

**Internal types:**
- `ParseResult<'a>` -- contains `program: Program<'a>`, `source_type: SourceType`

**Dependencies:**
- `types` -- `Diagnostic` for error reporting
- `errors` -- `create_source_error` for formatting parse errors

**Estimated complexity:** Simple -- thin wrapper around `oxc_parser::Parser`.

---

#### 7. `src/collector.rs` -- First-Pass AST Analysis

**Purpose:** Walk the parsed AST once (before transformation) to collect information needed by the transform pass: which imports come from `@qwik.dev/core`, which of those are `$`-suffixed, where `$()` call sites appear, and what the module exports. This is a read-only pass -- it does not mutate the AST.

**Public exports (pub(crate)):**
- `fn collect<'a>(program: &Program<'a>, scoping: &Scoping) -> CollectResult`

**Dependencies:**
- `types` -- `CollectResult`, `DollarCallSite`, `ImportInfo`, `ExportInfo`
- `words` -- `BUILDER_IO_QWIK`, `is_known_dollar_api` to identify Qwik imports

**Estimated complexity:** Medium -- full AST traversal with pattern matching on import declarations and call expressions. Must correctly identify all $-call sites including nested ones (e.g., `$()` inside JSX attribute expressions).

---

#### 8. `src/transform.rs` -- Main QwikTransform Traverse Implementation

**Purpose:** The core of the optimizer. Implements the `Traverse` trait to walk the AST and apply all Qwik transformations: replace `$()` calls with `qrl()` wrappers, record segments for extraction, rewrite imports, and handle special patterns (component$, useTask$, etc.). This is where all the transformation logic converges.

**Public exports (pub(crate)):**
- `struct QwikTransform` -- the Traverse implementor
- `impl QwikTransform`
  - `fn new(options: &TransformOptions, collected: CollectResult, filename: &str) -> Self`
  - `fn extracted_segments(&self) -> Vec<SegmentData>` -- segments recorded during traversal
  - `fn needed_imports(&self) -> Vec<String>` -- new imports to add (qrl, componentQrl, etc.)
  - `fn diagnostics(&self) -> Vec<Diagnostic>` -- any warnings/errors found

**Dependencies:**
- `types` -- `TransformOptions`, `SegmentData`, `CollectResult`, `DollarCallSite`, `Diagnostic`, `EntryStrategy`
- `collector` -- consumes `CollectResult` (via `types`, not direct module dep)
- `words` -- `dollar_to_qrl_name`, `BUILDER_IO_QWIK`, `is_known_dollar_api`
- `hash` -- `compute_segment_hash`, `format_segment_name`
- `import_rewrite` -- `rewrite_imports` for import transformation logic
- `props_destructuring` -- component props transformation
- `is_const` -- const evaluation during transform
- `const_replace` -- constant inlining during transform

**Estimated complexity:** Complex -- the largest module in the crate. Implements `enter_call_expression`, `exit_expression`, `exit_program`, and potentially other Traverse hooks. Must handle all $-call patterns, capture recording, nested $-calls, and coordinate with import rewriting.

---

#### 9. `src/import_rewrite.rs` -- Import Mutation Logic

**Purpose:** Transform import declarations after the main traversal. Removes `$`-suffixed imports that were consumed by the transform (e.g., removes `component$` from the import specifier list), adds new Qrl-suffixed imports (e.g., `componentQrl`), and adds `qrl` or `inlinedQrl` imports as needed.

**Public exports (pub(crate)):**
- `fn compute_import_changes(original_imports: &[ImportInfo], needed_qrl_names: &[String], needs_qrl: bool, needs_inlined_qrl: bool) -> ImportChanges`
- `struct ImportChanges` -- describes what imports to add/remove/keep

**Dependencies:**
- `types` -- `ImportInfo`, import-related types
- `words` -- `BUILDER_IO_QWIK`, `dollar_to_qrl_name`

**Estimated complexity:** Medium -- logic is straightforward but must match the exact import ordering observed in spec files: new Qrl imports first, then lazy import declarations, then kept original imports.

---

#### 10. `src/code_move.rs` -- Segment Extraction

**Purpose:** After the transform pass identifies segments, this module creates separate `Program` ASTs for each extracted segment. Each segment becomes its own module file containing the extracted function body as an exported const.

**Public exports (pub(crate)):**
- `fn extract_segments<'a>(allocator: &'a Allocator, segments: &[SegmentData], filename: &str, options: &TransformOptions) -> Vec<SegmentProgram<'a>>`

**Internal types:**
- `SegmentProgram<'a>` -- contains `program: Program<'a>`, `path: String`, `analysis: SegmentAnalysis`

**Dependencies:**
- `types` -- `SegmentData`, `SegmentAnalysis`, `TransformOptions`
- `hash` -- segment naming via `format_segment_name`
- `emit` -- may use emit types for output representation

**Estimated complexity:** Complex -- must construct complete, valid `Program` ASTs from extracted function bodies, including any imports the segment body needs (e.g., `useStore` from `@qwik.dev/core`). Must handle capture analysis results to determine what gets imported vs. passed as captures.

---

#### 11. `src/entry_strategy.rs` -- EntryStrategy Application

**Purpose:** Apply the configured `EntryStrategy` to determine how segments are output. The strategy controls whether segments become separate files (Segment), stay inline (Inline), are hoisted, etc.

**Public exports (pub(crate)):**
- `fn apply_strategy(strategy: &EntryStrategy, segments: &[SegmentData]) -> Vec<SegmentOutput>`
- `fn should_inline(strategy: &EntryStrategy) -> bool`
- `fn should_extract(strategy: &EntryStrategy) -> bool`

**Dependencies:**
- `types` -- `EntryStrategy`, `SegmentData`

**Estimated complexity:** Simple -- branching logic based on the strategy enum. The complexity lives in how `transform.rs` and `code_move.rs` use the strategy results, not in this module itself.

---

#### 12. `src/emit.rs` -- Code Generation

**Purpose:** Generate JavaScript source code (and optional source maps) from an OXC `Program` AST. Wraps `oxc_codegen::Codegen` with the crate's options (minification, source maps, etc.) and produces `TransformModule` values.

**Public exports (pub(crate)):**
- `fn emit_module<'a>(program: &Program<'a>, source: &str, options: &EmitOptions) -> EmitResult`
- `struct EmitOptions` -- source_maps, minify, etc.
- `struct EmitResult` -- code: String, map: Option<String>

**Dependencies:**
- `types` -- `MinifyMode`, `TransformModule`

**Estimated complexity:** Simple -- thin wrapper around `oxc_codegen::Codegen`. The main logic is configuring Codegen options and encoding source maps to base64 when requested.

---

#### 13. `src/filter_exports.rs` -- Export Stripping

**Purpose:** Strip exports from the module based on `strip_exports` and `strip_ctx_name` options. Used for server/client mode where certain exports should be removed.

**Public exports (pub(crate)):**
- `fn filter_exports<'a>(program: &mut Program<'a>, strip_exports: &[String], strip_ctx_name: &[String], allocator: &'a Allocator)`

**Dependencies:**
- `types` -- option types
- `words` -- for identifying Qwik-specific exports

**Estimated complexity:** Medium -- must handle various export forms (named, default, re-exports) and correctly remove statements or specifiers.

---

#### 14. `src/props_destructuring.rs` -- Component Props Transformation

**Purpose:** Transform component props destructuring patterns. When a component uses destructured props, the optimizer may need to transform the destructuring to preserve reactivity.

**Public exports (pub(crate)):**
- `fn transform_props<'a>(params: &mut FormalParameters<'a>, body: &mut FunctionBody<'a>, ctx: &mut TraverseCtx<'a>)`

**Dependencies:**
- `types` -- type references

**Estimated complexity:** Medium -- pattern matching on function parameters and body transformations.

---

#### 15. `src/is_const.rs` -- Const Evaluation Utilities

**Purpose:** Determine whether an expression is a compile-time constant. Used by `const_replace.rs` and the transform pass to decide what can be safely inlined.

**Public exports (pub(crate)):**
- `fn is_const_expression(expr: &Expression<'_>) -> bool`

**Dependencies:** None (leaf module -- pure computation on AST nodes)

**Estimated complexity:** Simple -- pattern matching on expression types (literals, template literals with no expressions, etc.).

---

#### 16. `src/const_replace.rs` -- Constant Inlining

**Purpose:** Replace references to compile-time constants with their values. Used for build-time configuration (e.g., `import.meta.env` replacements, `isDev` constants).

**Public exports (pub(crate)):**
- `fn replace_constants<'a>(program: &mut Program<'a>, constants: &HashMap<String, String>, allocator: &'a Allocator)`

**Dependencies:**
- `types` -- type references
- `is_const` -- `is_const_expression` for evaluation

**Estimated complexity:** Simple -- straightforward AST replacement of known identifiers with literal values.

---

### Dependency Graph

```
                         lib.rs
                           |
            +--------------+---------------+
            |              |               |
         parse.rs    collector.rs    transform.rs
            |              |               |
            v              v               |
        errors.rs      words.rs            |
            |                         +----+----+------+------+
            v                         |    |    |      |      |
        types.rs                  hash.rs  |  import   |   props_
            ^                              | _rewrite  |   destructuring
            |                              |    .rs    |      .rs
            |                              |    |      |
            +------------------------------+----+      |
                                                    code_move.rs
                                                       |
                                                    emit.rs
                                                       |
                                                    types.rs

        Leaf modules (no internal deps):
          types.rs, words.rs, hash.rs, is_const.rs

        Near-leaf (depend only on types):
          errors.rs, entry_strategy.rs, emit.rs, props_destructuring.rs

        Mid-level:
          parse.rs (-> types, errors)
          collector.rs (-> types, words)
          import_rewrite.rs (-> types, words)
          filter_exports.rs (-> types, words)
          const_replace.rs (-> types, is_const)

        High-level:
          transform.rs (-> types, words, hash, import_rewrite, collector [via types],
                          props_destructuring, is_const, const_replace)
          code_move.rs (-> types, hash, emit)
          lib.rs (-> everything)
```

**Full ASCII dependency graph (edges show "depends on"):**

```
                              +----------+
                              |  lib.rs  |
                              +----+-----+
                                   |
         +-------+-------+--------+--------+-------+--------+
         |       |       |        |        |       |        |
         v       v       v        v        v       v        v
      parse  collect  transform  code   entry   emit    filter
       .rs    or.rs     .rs     _move  _strat   .rs    _exports
         |       |       |       .rs    egy.rs    |       .rs
         |       |       |        |        |      |        |
         v       v       |        v        |      v        v
      errors  words     +---> hash.rs      |   types    words
       .rs     .rs      |        ^         |    .rs      .rs
         |       |      |        |         |     ^        |
         v       v      v        +----+----+     |        v
      types   types   types           |          |     types
       .rs     .rs     .rs         types.rs      |      .rs
                        |                        |
                        +---> import_rewrite.rs  |
                        |         |              |
                        |         v              |
                        |      words.rs          |
                        |      types.rs          |
                        |                        |
                        +---> props_destructuring.rs
                        |         |
                        |         v
                        |      types.rs
                        |
                        +---> is_const.rs (no deps)
                        |
                        +---> const_replace.rs
                                  |
                                  v
                               is_const.rs
                               types.rs
```

### Build Order

Modules are listed from leaf (can be implemented independently, no internal dependencies) to root (requires all prior modules).

**Tier 1 -- Leaf modules (implement first, no internal deps):**
1. `types.rs` -- all data structures
2. `words.rs` -- string constants
3. `hash.rs` -- segment hash computation
4. `is_const.rs` -- const evaluation utilities

**Tier 2 -- Near-leaf modules (depend only on Tier 1):**
5. `errors.rs` -- depends on `types`
6. `entry_strategy.rs` -- depends on `types`
7. `emit.rs` -- depends on `types`
8. `props_destructuring.rs` -- depends on `types`

**Tier 3 -- Mid-level modules (depend on Tier 1-2):**
9. `parse.rs` -- depends on `types`, `errors`
10. `collector.rs` -- depends on `types`, `words`
11. `import_rewrite.rs` -- depends on `types`, `words`
12. `filter_exports.rs` -- depends on `types`, `words`
13. `const_replace.rs` -- depends on `types`, `is_const`

**Tier 4 -- High-level modules (depend on multiple prior tiers):**
14. `code_move.rs` -- depends on `types`, `hash`, `emit`
15. `transform.rs` -- depends on `types`, `words`, `hash`, `import_rewrite`, `props_destructuring`, `is_const`, `const_replace`

**Tier 5 -- Root:**
16. `lib.rs` -- depends on all modules, orchestrates the pipeline

**No circular dependencies exist.** The dependency graph is a DAG: `types.rs` is the universal leaf, `lib.rs` is the universal root, and all edges flow downward from root to leaves.
