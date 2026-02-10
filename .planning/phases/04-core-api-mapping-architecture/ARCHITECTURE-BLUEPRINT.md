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

---

## Section 2: Public API Design (ARCH-02)

Every public type is defined below with complete Rust struct/enum definitions, including `#[derive]` attributes, `#[serde]` annotations, and field-level documentation. These types must serialize to identical JSON as the SWC optimizer versions, ensuring the TypeScript binding layer (`types.ts`) can consume either backend.

### Wire Compatibility

The OXC optimizer is a drop-in replacement for the SWC optimizer at the FFI boundary. The TypeScript binding layer calls `transform_modules()` with a JSON-serialized `TransformModulesOptions` and receives a JSON-serialized `TransformOutput`. For this to work:

1. **Field names in JSON must match exactly.** Rust uses snake_case; the TypeScript layer expects camelCase for most fields. Use `#[serde(rename_all = "camelCase")]` on structs where the TS binding uses camelCase.
2. **Enum serialization must match.** Use `#[serde(rename_all = "camelCase")]` on enums, and `#[serde(rename = "...")]` on specific variants where needed.
3. **Optional fields must use `Option<T>`.** When the TypeScript type has `field?: T`, the Rust type uses `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`.
4. **Default values** are annotated in comments. The deserializer should use `#[serde(default)]` where appropriate.

### Public Types

#### `TransformModulesOptions` -- Top-Level Input

The complete configuration object passed to `transform_modules()`. Corresponds to the TypeScript `TransformModulesOptions` interface.

```rust
/// Top-level configuration for transforming one or more modules.
///
/// SWC equivalent: TransformModulesOptions in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformModulesOptions {
    /// Root directory for resolving relative paths.
    pub src_dir: String,

    /// Optional root directory override (used for monorepo setups).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_dir: Option<String>,

    /// List of input modules to transform.
    pub input: Vec<TransformModuleInput>,

    /// Whether to generate source maps.
    /// Default: true
    #[serde(default = "default_true")]
    pub source_maps: bool,

    /// Minification mode.
    /// Default: MinifyMode::Simplify
    #[serde(default)]
    pub minify: MinifyMode,

    /// Whether to strip TypeScript type annotations.
    /// Default: false
    #[serde(default)]
    pub transpile_ts: bool,

    /// Whether to transpile JSX to function calls.
    /// Default: false
    #[serde(default)]
    pub transpile_jsx: bool,

    /// Whether to preserve original filenames in output paths.
    /// Default: false
    #[serde(default)]
    pub preserve_filenames: bool,

    /// How to split extracted segments into output modules.
    /// Default: EntryStrategy::Segment
    #[serde(default)]
    pub entry_strategy: EntryStrategy,

    /// Whether to use explicit file extensions in import paths.
    /// Default: false
    #[serde(default)]
    pub explicit_extensions: bool,

    /// Output mode controlling which build target to emit for.
    /// Default: EmitMode::Lib
    #[serde(default)]
    pub mode: EmitMode,

    /// Optional scope prefix for segment names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    /// Override the core module import path (default: "@qwik.dev/core").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_module: Option<String>,

    /// List of export names to strip from output.
    /// Used for server/client-specific builds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_exports: Option<Vec<String>>,

    /// List of ctx names to strip (e.g., strip all "useTask$" segments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strip_ctx_name: Option<Vec<String>>,

    /// Whether to strip event handler registrations.
    /// Default: false
    #[serde(default)]
    pub strip_event_handlers: bool,

    /// List of ctx names to register (for plugin coordination).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reg_ctx_name: Option<Vec<String>>,

    /// Whether this build targets SSR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_server: Option<bool>,
}

fn default_true() -> bool { true }
```

#### `TransformModuleInput` -- Input File Representation

```rust
/// A single input file to transform.
///
/// SWC equivalent: TransformModuleInput in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformModuleInput {
    /// The source code content.
    pub code: String,

    /// The file path (relative to src_dir).
    pub path: String,
}
```

#### `TransformOutput` -- Complete Result

```rust
/// Complete result of transforming one or more modules.
///
/// SWC equivalent: TransformOutput in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOutput {
    /// All output modules: main modules + extracted segments.
    pub modules: Vec<TransformModule>,

    /// Diagnostics (errors, warnings) from transformation.
    pub diagnostics: Vec<Diagnostic>,

    /// Whether any input was TypeScript.
    pub is_type_script: bool,

    /// Whether any input contained JSX.
    pub is_jsx: bool,
}
```

#### `TransformModule` -- Individual Module Output

```rust
/// A single output module (either the transformed main module or an extracted segment).
///
/// SWC equivalent: TransformModule in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformModule {
    /// Output file path (relative to src_dir).
    pub path: String,

    /// Whether this module is an entry point (segment modules are entry points).
    pub is_entry: bool,

    /// The generated JavaScript source code.
    pub code: String,

    /// Optional source map (JSON string, base64-encoded if inline).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,

    /// Segment metadata, present only for extracted segment modules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<SegmentAnalysis>,

    /// Original input file path (before transformation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_path: Option<String>,
}
```

#### `SegmentAnalysis` -- Segment Metadata

This type is critical for wire compatibility. Every field must match the JSON produced by the SWC optimizer, as the TypeScript layer reads these fields directly.

```rust
/// Metadata about an extracted segment (a lazy-loadable code fragment).
///
/// SWC equivalent: HookAnalysis/SegmentAnalysis in types.ts
///
/// Example JSON (from spec file example_1.md):
/// {
///   "origin": "test.tsx",
///   "name": "renderHeader_zBbHWn4e8Cg",
///   "entry": null,
///   "displayName": "test.tsx_renderHeader",
///   "hash": "zBbHWn4e8Cg",
///   "canonicalFilename": "test.tsx_renderHeader_zBbHWn4e8Cg",
///   "path": "",
///   "extension": "tsx",
///   "parent": null,
///   "ctxKind": "function",
///   "ctxName": "$",
///   "captures": false,
///   "loc": [90, 161]
/// }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SegmentAnalysis {
    /// Source file this segment was extracted from.
    pub origin: String,

    /// Full segment name including hash (e.g., "renderHeader_zBbHWn4e8Cg").
    pub name: String,

    /// Entry point name, if this segment is a named entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,

    /// Human-readable display name (e.g., "test.tsx_renderHeader").
    pub display_name: String,

    /// The 11-character hash (e.g., "zBbHWn4e8Cg").
    pub hash: String,

    /// Canonical filename for the segment module (e.g., "test.tsx_renderHeader_zBbHWn4e8Cg").
    pub canonical_filename: String,

    /// Output path prefix (empty string if same directory).
    pub path: String,

    /// File extension of the output (e.g., "tsx", "ts", "js").
    pub extension: String,

    /// Parent segment name, if this segment is nested inside another.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// Context kind: whether this is an event handler or a function.
    pub ctx_kind: CtxKind,

    /// Context name: the $-suffixed function that created this segment
    /// (e.g., "$", "component$", "useTask$").
    pub ctx_name: String,

    /// Whether this segment captures variables from its enclosing scope.
    pub captures: bool,

    /// Source location as [start_byte, end_byte] of the original $-call.
    pub loc: (u32, u32),
}
```

#### `EntryStrategy` -- Segment Output Strategy

```rust
/// Controls how extracted segments are output.
///
/// SWC equivalent: EntryStrategy in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum EntryStrategy {
    /// Each segment becomes a separate file with a lazy import.
    /// This is the default strategy.
    /// `$(() => {...})` -> separate file + `qrl(()=>import("./file"), "name")`
    Segment,

    /// Segments stay in the same file with inlinedQrl wrappers.
    /// `$(() => {...})` -> `inlinedQrl(() => {...}, "name", [captures])`
    Inline,

    /// Segments are hoisted to the top of the module.
    Hoist,

    /// All segments go into a single output file.
    Single,

    /// Group segments by their parent component.
    Component,

    /// Automatically choose the best strategy based on usage patterns.
    Smart,

    /// Group segments by their hook type.
    Hook,
}

impl Default for EntryStrategy {
    fn default() -> Self {
        EntryStrategy::Segment
    }
}
```

#### `MinifyMode` -- Minification Control

```rust
/// Controls output minification.
///
/// SWC equivalent: MinifyMode in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MinifyMode {
    /// Apply simplification transforms (dead code elimination, constant folding).
    Simplify,

    /// No minification.
    None,
}

impl Default for MinifyMode {
    fn default() -> Self {
        MinifyMode::Simplify
    }
}
```

#### `EmitMode` -- Build Target Mode

```rust
/// Controls the build target output mode.
///
/// SWC equivalent: EmitMode in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EmitMode {
    /// Library mode -- standard output.
    Lib,

    /// Production mode -- optimized output.
    Prod,

    /// Development mode -- includes debug info.
    Dev,
}

impl Default for EmitMode {
    fn default() -> Self {
        EmitMode::Lib
    }
}
```

#### `CtxKind` -- Context Kind

```rust
/// The kind of context that created a segment.
///
/// In spec file JSON, this appears as "eventHandler" or "function".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CtxKind {
    /// Event handler context (e.g., onClick$, onInput$).
    #[serde(rename = "eventHandler")]
    EventHandler,

    /// Function context (e.g., $, component$, useTask$).
    #[serde(rename = "function")]
    Function,
}
```

#### `Diagnostic` -- Error/Warning

```rust
/// A diagnostic message from the transformation process.
///
/// SWC equivalent: Diagnostic in types.ts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// The diagnostic category.
    pub category: DiagnosticCategory,

    /// Machine-readable error code.
    pub code: Option<String>,

    /// File path where the diagnostic originated.
    pub file: String,

    /// Human-readable message.
    pub message: String,

    /// Optional source code highlights.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Vec<SourceLocation>>,

    /// Optional fix suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}
```

#### `DiagnosticCategory`

```rust
/// Severity level of a diagnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DiagnosticCategory {
    /// A hard error that prevents successful transformation.
    Error,

    /// A warning that does not prevent transformation.
    Warning,

    /// An error originating from the source code (e.g., syntax error).
    SourceError,
}
```

#### `SourceLocation`

```rust
/// A source location for diagnostic highlighting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceLocation {
    /// Starting byte offset.
    pub lo: u32,

    /// Ending byte offset.
    pub hi: u32,

    /// Starting line (1-indexed).
    pub start_line: u32,

    /// Starting column (0-indexed).
    pub start_col: u32,

    /// Ending line (1-indexed).
    pub end_line: u32,

    /// Ending column (0-indexed).
    pub end_col: u32,
}
```

### Internal Types (pub(crate))

These types are used within the crate but not exposed to external consumers.

#### `CollectResult`

```rust
/// Result of the collector pass -- everything discovered about the module
/// before transformation begins.
#[derive(Debug, Clone)]
pub(crate) struct CollectResult {
    /// Set of known $-suffixed imports from @qwik.dev/core.
    /// e.g., {"$", "component$", "useTask$"}
    pub dollar_imports: HashSet<String>,

    /// Located $-call sites with span info.
    pub dollar_calls: Vec<DollarCallSite>,

    /// All import declarations in the module.
    pub module_imports: Vec<ImportInfo>,

    /// All export declarations in the module.
    pub module_exports: Vec<ExportInfo>,
}
```

#### `DollarCallSite`

```rust
/// A located $-call site in the source code.
#[derive(Debug, Clone)]
pub(crate) struct DollarCallSite {
    /// The name of the callee (e.g., "$", "component$").
    pub callee_name: String,

    /// Byte offset span of the entire call expression.
    pub span: (u32, u32),

    /// The display name derived from the lexical context
    /// (e.g., "Header_component" for `const Header = component$(...)`).
    pub display_name: String,

    /// Whether this is a nested $-call (inside another $-call's body).
    pub is_nested: bool,

    /// The parent $-call's display name, if nested.
    pub parent_name: Option<String>,
}
```

#### `ImportInfo`

```rust
/// Recorded import declaration from the source module.
#[derive(Debug, Clone)]
pub(crate) struct ImportInfo {
    /// Import source (e.g., "@qwik.dev/core", "./utils").
    pub source: String,

    /// Named import specifiers (e.g., ["$", "component$", "useStore"]).
    pub specifiers: Vec<String>,

    /// Whether this imports from the Qwik core module.
    pub is_qwik_core: bool,

    /// Byte offset span of the import declaration.
    pub span: (u32, u32),
}
```

#### `ExportInfo`

```rust
/// Recorded export declaration from the source module.
#[derive(Debug, Clone)]
pub(crate) struct ExportInfo {
    /// Exported name (e.g., "renderHeader").
    pub name: String,

    /// Whether this is a re-export (export { X } from '...').
    pub is_reexport: bool,

    /// Byte offset span of the export declaration.
    pub span: (u32, u32),
}
```

#### `SegmentData`

```rust
/// Intermediate segment representation recorded during the transform pass.
/// This gets converted to SegmentAnalysis + a segment Program during code_move.
#[derive(Debug, Clone)]
pub(crate) struct SegmentData {
    /// The display name (e.g., "test.tsx_Header_component").
    pub display_name: String,

    /// The computed hash (e.g., "J4uyIhaBNR4").
    pub hash: String,

    /// The full segment name (e.g., "Header_component_J4uyIhaBNR4").
    pub name: String,

    /// The callee that created this segment (e.g., "$", "component$").
    pub ctx_name: String,

    /// Context kind (function or event handler).
    pub ctx_kind: CtxKind,

    /// Source file origin.
    pub origin: String,

    /// File extension.
    pub extension: String,

    /// Span of the original $-call expression.
    pub span: (u32, u32),

    /// Parent segment name, if nested.
    pub parent: Option<String>,

    /// Whether the segment body captures outer scope variables.
    pub captures: bool,

    /// Names of captured variables (for inlinedQrl's capture array).
    pub capture_names: Vec<String>,

    /// Imports needed by the segment body (e.g., useStore from @qwik.dev/core).
    pub needed_imports: Vec<ImportInfo>,

    /// The extracted function body expression (serialized or stored as reference).
    /// In practice, this will be an index or key into the AST arena.
    pub body_span: (u32, u32),
}
```

#### `TransformOptions`

```rust
/// Per-module options derived from TransformModulesOptions.
/// Passed to individual module transformations.
#[derive(Debug, Clone)]
pub(crate) struct TransformOptions {
    pub src_dir: String,
    pub root_dir: Option<String>,
    pub source_maps: bool,
    pub minify: MinifyMode,
    pub transpile_ts: bool,
    pub transpile_jsx: bool,
    pub preserve_filenames: bool,
    pub entry_strategy: EntryStrategy,
    pub explicit_extensions: bool,
    pub mode: EmitMode,
    pub scope: Option<String>,
    pub core_module: String, // resolved to actual value, not Option
    pub strip_exports: Vec<String>,
    pub strip_ctx_name: Vec<String>,
    pub strip_event_handlers: bool,
    pub reg_ctx_name: Vec<String>,
    pub is_server: bool,
}
```

---

## Section 3: Data Flow Specification (ARCH-03)

### Complete Pipeline Overview

```
TransformModulesOptions { input: Vec<TransformModuleInput>, ... }
  |
  |  lib.rs: transform_modules()
  |  Derive TransformOptions from TransformModulesOptions
  |
  v
  For each TransformModuleInput (parallel via rayon):
  |
  |  [Stage 1: PARSE]
  |  parse.rs: parse_module(allocator, source, filename)
  |     Input:  source: &str, filename: &str
  |     Output: Result<ParseResult { program: Program<'a>, source_type }, Vec<Diagnostic>>
  |     Module: parse.rs
  |     Side effects: None (pure function)
  |     Can fail: Yes -- syntax errors produce Vec<Diagnostic>
  |
  |  [Stage 2: SEMANTIC ANALYSIS]
  |  SemanticBuilder::new().with_excess_capacity(2.0).build(&program)
  |     Input:  &Program<'a>
  |     Output: SemanticReturn { semantic: Semantic }
  |     Module: OXC (oxc_semantic::SemanticBuilder)
  |     Side effects: Builds scope tree + symbol table
  |     Can fail: No (always succeeds after successful parse)
  |  semantic.into_scoping() -> Scoping
  |
  |  [Stage 3: COLLECT]
  |  collector.rs: collect(&program, &scoping)
  |     Input:  &Program<'a>, &Scoping
  |     Output: CollectResult { dollar_imports, dollar_calls, module_imports, module_exports }
  |     Module: collector.rs
  |     Side effects: None (read-only AST traversal)
  |     Can fail: No
  |
  |  [Stage 4: TRANSFORM]
  |  transform.rs: QwikTransform::new(options, collected, filename)
  |  oxc_traverse::traverse_mut(&mut qwik_transform, &allocator, &mut program, scoping)
  |     Input:  &mut Program<'a>, TransformOptions, CollectResult, Scoping
  |     Output: Mutated program + QwikTransform state containing:
  |             - extracted_segments: Vec<SegmentData>
  |             - needed_imports: Vec<String>
  |             - diagnostics: Vec<Diagnostic>
  |     Module: transform.rs (with import_rewrite.rs, props_destructuring.rs,
  |             is_const.rs, const_replace.rs as helpers)
  |     Side effects: Mutates the AST in-place:
  |       - Replaces $() calls with qrl()/inlinedQrl() wrappers
  |       - Records segment data for extraction
  |       - Rewrites imports (remove $-suffixed, add Qrl-suffixed)
  |       - Adds lazy import declarations
  |       - Applies PURE annotations
  |     Can fail: Yes -- invalid patterns produce Diagnostic warnings
  |
  |  [Stage 5: FILTER EXPORTS] (conditional)
  |  filter_exports.rs: filter_exports(&mut program, strip_exports, strip_ctx_name)
  |     Input:  &mut Program<'a>, &[String], &[String]
  |     Output: Mutated program with stripped exports
  |     Module: filter_exports.rs
  |     Side effects: Removes export statements/specifiers from AST
  |     Can fail: No
  |     Condition: Only runs if strip_exports or strip_ctx_name is non-empty
  |
  |  [Stage 6: EXTRACT SEGMENTS]
  |  code_move.rs: extract_segments(allocator, &segments, filename, &options)
  |     Input:  &Allocator, &[SegmentData], filename, &TransformOptions
  |     Output: Vec<SegmentProgram { program, path, analysis }>
  |     Module: code_move.rs
  |     Side effects: Creates new Program ASTs in the allocator
  |     Can fail: No (data is validated during transform)
  |
  |  [Stage 7: EMIT]
  |  emit.rs: emit_module(program, source, &emit_options)
  |  For main module + each segment:
  |     Input:  &Program<'a>, source: &str, EmitOptions
  |     Output: EmitResult { code: String, map: Option<String> }
  |     Module: emit.rs
  |     Side effects: None (pure code generation)
  |     Can fail: No
  |
  |  [Stage 8: ASSEMBLE]
  |  Construct Vec<TransformModule> from emit results + SegmentAnalysis
  |     Input:  EmitResult (main), Vec<(EmitResult, SegmentAnalysis)> (segments)
  |     Output: Vec<TransformModule>
  |     Module: lib.rs (inline logic)
  |     Side effects: None
  |
  v
  Aggregate: all Vec<TransformModule> + all Vec<Diagnostic>
  |
  v
TransformOutput {
    modules: Vec<TransformModule>,    // main modules + all extracted segments
    diagnostics: Vec<Diagnostic>,     // all errors/warnings
    is_type_script: bool,             // any input was .ts/.tsx
    is_jsx: bool,                     // any input was .jsx/.tsx
}
```

### Stage-by-Stage Type Signatures

#### Stage 1: Parse

```rust
// parse.rs

/// Parse a single source file into an OXC Program AST.
pub(crate) fn parse_module<'a>(
    allocator: &'a Allocator,
    source: &str,
    filename: &str,
) -> Result<ParseResult<'a>, Vec<Diagnostic>>

pub(crate) struct ParseResult<'a> {
    pub program: Program<'a>,
    pub source_type: SourceType,
}
```

**Error handling:** If the parser reports errors (`ret.errors` is non-empty or `ret.panicked` is true), convert each OXC parser error to a `Diagnostic` with `DiagnosticCategory::SourceError` and return `Err(diagnostics)`. The caller (`lib.rs`) collects these diagnostics into the final output without aborting the entire batch -- other modules can still be transformed.

#### Stage 2: Semantic Analysis

```rust
// Inline in lib.rs (uses OXC directly)

let semantic_ret = SemanticBuilder::new()
    .with_excess_capacity(2.0)
    .build(&program);
let scoping: Scoping = semantic_ret.semantic.into_scoping();
```

**Note:** `SemanticBuilder` requires a reference to the `Program`. The `Scoping` object is consumed by `traverse_mut` and provides scope/symbol info during traversal.

#### Stage 3: Collect

```rust
// collector.rs

/// Perform first-pass analysis of the parsed module.
pub(crate) fn collect<'a>(
    program: &Program<'a>,
    scoping: &Scoping,
) -> CollectResult
```

**Output details:**

```rust
CollectResult {
    // Which names imported from @qwik.dev/core are $-suffixed
    dollar_imports: HashSet<String>,  // e.g., {"$", "component$"}

    // Each $-call site found in the module
    dollar_calls: Vec<DollarCallSite>,

    // All import declarations (for import rewriting)
    module_imports: Vec<ImportInfo>,

    // All export declarations (for export stripping)
    module_exports: Vec<ExportInfo>,
}
```

#### Stage 4: Transform

```rust
// transform.rs

pub(crate) struct QwikTransform {
    options: TransformOptions,
    collected: CollectResult,
    filename: String,
    segments: Vec<SegmentData>,
    needed_qrl_imports: HashSet<String>,  // e.g., {"componentQrl"}
    needs_qrl: bool,                      // needs `import { qrl }`
    needs_inlined_qrl: bool,              // needs `import { inlinedQrl }`
    diagnostics: Vec<Diagnostic>,
    segment_counter: u32,                 // for ordering
}

impl<'a> Traverse<'a> for QwikTransform {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    );

    fn exit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a>,
    );

    fn exit_program(
        &mut self,
        program: &mut Program<'a>,
        ctx: &mut TraverseCtx<'a>,
    );
}
```

**What the transform does to the AST:**

1. **`enter_call_expression`**: Detect $-call sites. Record segment data. Mark the call for replacement.
2. **`exit_expression`**: Replace marked `$()` calls with `qrl(i_hash, "name")` or `inlinedQrl(fn, "name", [captures])` depending on entry strategy.
3. **`exit_program`**: Rewrite imports -- prepend new import declarations (`componentQrl`, `qrl`), insert lazy import declarations (`const i_hash = ()=>import(...)`), and keep/modify original imports.

**Import ordering in output** (observed from spec files):

```
import { componentQrl } from "@qwik.dev/core";     // 1. New Qrl imports
import { qrl } from "@qwik.dev/core";              // 2. qrl/inlinedQrl import
const i_hash = ()=>import("./segment_path");        // 3. Lazy import declarations
import { $, useStore } from '@qwik.dev/core';       // 4. Original imports (kept, $-suffixed removed)
// ... rest of transformed code                     // 5. Transformed declarations
```

#### Stage 5: Filter Exports (conditional)

```rust
// filter_exports.rs

pub(crate) fn filter_exports<'a>(
    program: &mut Program<'a>,
    strip_exports: &[String],
    strip_ctx_name: &[String],
    allocator: &'a Allocator,
)
```

Only called when `strip_exports` or `strip_ctx_name` is non-empty in the options.

#### Stage 6: Extract Segments

```rust
// code_move.rs

pub(crate) fn extract_segments<'a>(
    allocator: &'a Allocator,
    segments: &[SegmentData],
    filename: &str,
    options: &TransformOptions,
) -> Vec<SegmentProgram<'a>>

pub(crate) struct SegmentProgram<'a> {
    pub program: Program<'a>,
    pub path: String,
    pub analysis: SegmentAnalysis,
}
```

**What each segment Program looks like** (from spec file analysis):

For the Segment strategy, each extracted segment is a standalone module:
```javascript
// Segment with no dependencies:
export const renderHeader_div_onClick_fV2uzAL99u4 = (ctx)=>console.log(ctx);

// Segment with imports from @qwik.dev/core:
import { useStore } from "@qwik.dev/core";
export const Header_component_J4uyIhaBNR4 = ()=>{
    const thing = useStore();
    // ...
};
```

#### Stage 7: Emit

```rust
// emit.rs

pub(crate) fn emit_module<'a>(
    program: &Program<'a>,
    source: &str,
    options: &EmitOptions,
) -> EmitResult

pub(crate) struct EmitOptions {
    pub source_maps: bool,
    pub minify: MinifyMode,
}

pub(crate) struct EmitResult {
    pub code: String,
    pub map: Option<String>,
}
```

#### Stage 8: Assemble (in lib.rs)

```rust
// Inline in lib.rs

fn assemble_output(
    main_emit: EmitResult,
    segment_programs: Vec<(EmitResult, SegmentAnalysis)>,
    filename: &str,
    source_type: SourceType,
    diagnostics: Vec<Diagnostic>,
) -> (Vec<TransformModule>, Vec<Diagnostic>)
```

### Parallel Execution Model

```rust
// lib.rs

use rayon::prelude::*;

pub fn transform_modules(
    config: TransformModulesOptions,
) -> Result<TransformOutput, anyhow::Error> {
    let options = TransformOptions::from(&config);

    let results: Vec<Result<ModuleResult, anyhow::Error>> = config.input
        .par_iter()  // rayon parallel iteration
        .map(|input| transform_single_module(input, &options))
        .collect();

    // Aggregate results
    let mut all_modules = Vec::new();
    let mut all_diagnostics = Vec::new();
    let mut is_type_script = false;
    let mut is_jsx = false;

    for result in results {
        match result {
            Ok(module_result) => {
                all_modules.extend(module_result.modules);
                all_diagnostics.extend(module_result.diagnostics);
                is_type_script |= module_result.is_type_script;
                is_jsx |= module_result.is_jsx;
            }
            Err(e) => {
                all_diagnostics.push(Diagnostic::from_error(e));
            }
        }
    }

    Ok(TransformOutput {
        modules: all_modules,
        diagnostics: all_diagnostics,
        is_type_script,
        is_jsx,
    })
}
```

**Key architectural constraint:** Each `Allocator` is local to its `transform_single_module` call. The `Program<'a>` lifetime is tied to the allocator. All parsing, transformation, and code generation must happen within the same scope. Only `String` values (generated code, source maps) escape the allocator scope.

---

## Section 4: Test Harness Design (ARCH-04)

### Overview

The 162 spec files in `.planning/spec/` are the behavioral ground truth. The test harness parses these markdown files, extracts test configuration/input/output, runs the optimizer, and compares results. This provides 162 regression tests from day one.

### Spec File Format

Each spec file follows this structure (parsed from markdown):

```
# Test: {test_name}

## Test Configuration
| Option | Value |
|--------|-------|
| {key}  | {val} |      <-- TransformModulesOptions overrides

## Input
### Source Code
```tsx
{input_code}                   <-- The source to transform
```

## Output
### Module: {module_path} (ENTRY POINT)   <-- or without "(ENTRY POINT)"
```tsx
{output_code}                  <-- Expected output JavaScript
```
#### Segment Metadata
```json
{segment_json}                 <-- Expected SegmentAnalysis JSON
```

## Diagnostics
```json
{diagnostics_json}             <-- Expected diagnostics (usually [])
```
```

### Spec File Parser

```rust
// tests/spec_parser.rs (or tests/common/mod.rs)

use std::fs;
use std::path::Path;

/// A parsed spec file ready for test execution.
#[derive(Debug)]
pub struct SpecFile {
    /// Test name (e.g., "example_1", "example_functional_component").
    pub name: String,

    /// TransformModulesOptions overrides from the configuration table.
    pub config_overrides: HashMap<String, String>,

    /// Input source code.
    pub input_code: String,

    /// Input filename (derived from test name, defaults to "test.tsx").
    pub input_filename: String,

    /// Expected output modules.
    pub expected_modules: Vec<ExpectedModule>,

    /// Expected diagnostics.
    pub expected_diagnostics: Vec<serde_json::Value>,
}

/// An expected output module from the spec file.
#[derive(Debug)]
pub struct ExpectedModule {
    /// Module path (e.g., "test.tsx", "test.tsx_renderHeader_zBbHWn4e8Cg.tsx").
    pub path: String,

    /// Whether this is an entry point.
    pub is_entry: bool,

    /// Expected output code.
    pub code: String,

    /// Expected segment metadata (if this is a segment module).
    pub segment: Option<serde_json::Value>,
}

/// Parse a spec file from markdown into a SpecFile struct.
pub fn parse_spec_file(path: &Path) -> Result<SpecFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Parse configuration table
    let config_overrides = parse_config_table(&content);

    // Parse input code (from "### Source Code" section)
    let input_code = extract_code_block(&content, "### Source Code")
        .ok_or_else(|| format!("No source code found in {}", name))?;

    // Parse output modules (from "### Module:" sections)
    let expected_modules = parse_output_modules(&content);

    // Parse diagnostics
    let expected_diagnostics = parse_diagnostics(&content);

    Ok(SpecFile {
        name,
        config_overrides,
        input_code,
        input_filename: "test.tsx".to_string(),
        expected_modules,
        expected_diagnostics,
    })
}

/// Build TransformModulesOptions from a spec file.
pub fn build_options(spec: &SpecFile) -> TransformModulesOptions {
    let mut options = TransformModulesOptions::default();
    options.input = vec![TransformModuleInput {
        code: spec.input_code.clone(),
        path: spec.input_filename.clone(),
    }];

    // Apply config overrides
    for (key, value) in &spec.config_overrides {
        match key.to_lowercase().as_str() {
            "minify" => options.minify = parse_minify_mode(value),
            "entry_strategy" | "entrystrategy" => options.entry_strategy = parse_entry_strategy(value),
            "transpile_ts" | "transpilts" => options.transpile_ts = value.parse().unwrap_or(false),
            "transpile_jsx" | "transpilejsx" => options.transpile_jsx = value.parse().unwrap_or(false),
            "mode" => options.mode = parse_emit_mode(value),
            // ... other config keys
            _ => {} // ignore unknown keys
        }
    }

    options
}
```

### Test Runner Structure

The test harness uses a combination of `insta` snapshot testing and direct assertion.

```rust
// tests/spec_tests.rs

use std::path::PathBuf;

mod spec_parser;

/// Get the path to the spec files directory.
fn spec_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Spec files are at workspace_root/.planning/spec/
    // Adjust path based on crate location relative to workspace root
    manifest_dir.join("..").join(".planning").join("spec")
}

/// Load all spec files from the spec directory.
fn load_all_specs() -> Vec<spec_parser::SpecFile> {
    let dir = spec_dir();
    let mut specs = Vec::new();

    for entry in std::fs::read_dir(&dir).expect("Failed to read spec dir") {
        let entry = entry.expect("Failed to read dir entry");
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            match spec_parser::parse_spec_file(&path) {
                Ok(spec) => specs.push(spec),
                Err(e) => eprintln!("Warning: Failed to parse {}: {}", path.display(), e),
            }
        }
    }

    specs.sort_by(|a, b| a.name.cmp(&b.name));
    specs
}

#[cfg(test)]
mod tests {
    use super::*;
    use qwik_optimizer_oxc::transform_modules;

    #[test]
    fn test_all_specs() {
        let specs = load_all_specs();
        assert!(!specs.is_empty(), "No spec files found");

        let mut failures = Vec::new();

        for spec in &specs {
            let options = spec_parser::build_options(spec);
            let result = match transform_modules(options) {
                Ok(r) => r,
                Err(e) => {
                    failures.push(format!("{}: transform failed: {}", spec.name, e));
                    continue;
                }
            };

            // Check module count
            if result.modules.len() != spec.expected_modules.len() {
                failures.push(format!(
                    "{}: expected {} modules, got {}",
                    spec.name,
                    spec.expected_modules.len(),
                    result.modules.len()
                ));
                continue;
            }

            // Compare each module
            for expected in &spec.expected_modules {
                let actual = result.modules.iter()
                    .find(|m| m.path == expected.path);

                match actual {
                    None => {
                        failures.push(format!(
                            "{}: missing output module '{}'",
                            spec.name, expected.path
                        ));
                    }
                    Some(actual) => {
                        // Snapshot test the code output
                        insta::assert_snapshot!(
                            format!("{}__{}", spec.name, expected.path.replace('/', "_")),
                            actual.code.clone()
                        );

                        // Compare segment metadata if present
                        if let (Some(expected_seg), Some(actual_seg)) =
                            (&expected.segment, &actual.segment)
                        {
                            let actual_json = serde_json::to_value(actual_seg).unwrap();
                            assert_eq!(
                                expected_seg, &actual_json,
                                "{}: segment metadata mismatch for {}",
                                spec.name, expected.path
                            );
                        }
                    }
                }
            }
        }

        if !failures.is_empty() {
            panic!(
                "Spec test failures ({}/{}):\n{}",
                failures.len(),
                specs.len(),
                failures.join("\n")
            );
        }
    }
}
```

### Progressive Testing Order

Start with the simplest spec files and build up incrementally. Each tier introduces new transformation patterns that depend on the prior tier working.

**Tier 1 -- Basic $() extraction (3 specs):**
Target: `parse.rs`, `collector.rs`, `hash.rs`, `emit.rs`, basic `transform.rs`
- `example_1.md` -- Raw `$()` with nested JSX onClick handler
- `example_2.md` -- Multiple `$()` extractions in one module
- `example_3.md` -- Export patterns with `$()` extraction

**Tier 2 -- component$ pattern (2 specs):**
Target: `import_rewrite.rs`, dollar-to-qrl naming in `words.rs`
- `example_functional_component.md` -- `component$` with `useStore` import hoisting
- `example_functional_component_2.md` -- Multiple components in one file

**Tier 3 -- Capture analysis (2 specs):**
Target: Capture detection in `collector.rs` and `transform.rs`
- `example_capture_imports.md` -- Captures with CSS imports
- `example_multi_capture.md` -- Multiple captures in one segment

**Tier 4 -- Inline entry strategy (1 spec):**
Target: `entry_strategy.rs`, `inlinedQrl` path in `transform.rs`
- `example_inlined_entry_strategy.md` -- Inline strategy with captures and `inlinedQrl`

**Tier 5 -- JSX transformation (2 specs):**
Target: JSX-specific transform paths
- `example_jsx.md` -- JSX elements with `_jsxSorted`/`_jsxSplit`
- `example_jsx_keyed.md` -- Keyed JSX children

**Tier 6 -- Edge cases and remaining patterns (152 specs):**
Target: All remaining modules, full coverage
- All other spec files, run via the test_all_specs harness
- Categorized by: export stripping, props destructuring, signals, const replacement, error diagnostics, and various option combinations

### Spec File Location and Configuration

Spec files live at `{workspace_root}/.planning/spec/*.md` (162 files). The test harness resolves this path using `CARGO_MANIFEST_DIR`:

```rust
fn spec_dir() -> PathBuf {
    // If the crate is at workspace_root/crates/qwik-optimizer-oxc/
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("..").join("..").join(".planning").join("spec")
}
```

**Alternative:** Set a `QWIK_SPEC_DIR` environment variable in `.cargo/config.toml`:
```toml
[env]
QWIK_SPEC_DIR = { value = ".planning/spec", relative = true }
```

### Comparison Strategy

1. **Code comparison** -- Use `insta::assert_snapshot!` for the generated JavaScript. This gives human-readable diffs when output changes. After initial snapshot creation, `cargo insta review` provides an interactive review workflow.

2. **Segment metadata comparison** -- JSON-serialize `SegmentAnalysis` and compare field-by-field with `assert_eq!`. Hash values must match exactly. The `loc` field (source span) is compared as `(u32, u32)`.

3. **Module count comparison** -- Verify the same number of output modules (main + segments) as the spec file.

4. **Import verification** -- Check that the same imports appear in the same output modules. This is covered implicitly by the code snapshot comparison.

5. **Diagnostic comparison** -- Compare the diagnostics JSON array. Most spec files have `[]` (no diagnostics).

---

## Section 5: Cargo.toml Specification (ARCH-05)

### Complete Cargo.toml

This Cargo.toml can be copied directly into a new crate and will compile. All version pins are verified against crates.io as of 2026-02-10.

```toml
[package]
name = "qwik-optimizer-oxc"
version = "0.1.0"
edition = "2024"
description = "Qwik optimizer using OXC for code transformation"
license = "MIT"

[features]
default = ["parallel", "source-maps"]

# Enable parallel module transformation via rayon.
# Disable for single-threaded environments (WASM).
parallel = ["dep:rayon"]

# Enable source map generation.
source-maps = ["dep:base64"]

[dependencies]
# OXC umbrella crate -- provides parser, AST, traverse, codegen, semantic analysis.
# NOTE: The "transformer" feature is intentionally excluded. It pulls in babel-compat
# layers we don't need. Import management is implemented directly in import_rewrite.rs
# and the exit_program hook.
oxc = { version = "0.113", features = [
    "parser",       # oxc_parser for JS/TS/JSX/TSX parsing
    "traverse",     # oxc_traverse for Traverse trait + TraverseCtx
    "codegen",      # oxc_codegen for AST -> JavaScript output
    "semantic",     # oxc_semantic for SemanticBuilder (scope/symbol analysis)
    "serialize",    # ESTree JSON serialization (for debugging/testing)
] }

# Serialization for TransformOutput, SegmentAnalysis, and all public types.
serde = { version = "1", features = ["derive"] }

# JSON encoding/decoding for transform options and output.
serde_json = "1"

# Error handling with context chains.
anyhow = "1"

# Source map encoding (base64).
# Only included when "source-maps" feature is active.
base64 = { version = "0.22", optional = true }

# Parallel module transformation.
# Only included when "parallel" feature is active.
rayon = { version = "1", optional = true }

# Relative path computation for import paths between segments.
pathdiff = "0.2"

# Cross-platform path normalization (consistent / separators).
path-slash = "0.2"

[dev-dependencies]
# Snapshot testing for comparing output against 162 spec files.
insta = { version = "1", features = ["json"] }
```

### Dependency Justification

| Dependency | Purpose | Why Required |
|------------|---------|--------------|
| `oxc` | Parser, AST, Traverse, Codegen, Semantic | Core transformation infrastructure -- replaces SWC |
| `serde` + `serde_json` | JSON serialization | FFI boundary with TypeScript layer requires JSON |
| `anyhow` | Error handling | Propagate errors with context from parse/transform/emit |
| `base64` | Source map encoding | Source maps are base64-encoded in the JSON output |
| `rayon` | Parallel iteration | `transform_modules` processes N files; parallelism matches SWC optimizer |
| `pathdiff` | Relative paths | Import paths between main module and segments are relative |
| `path-slash` | Path normalization | Windows compatibility for consistent `/` in paths |
| `insta` | Snapshot testing | Compare generated code against 162 spec files |

### OXC Feature Flag Decision

The `transformer` feature is **excluded** by design. The research (open question #5) identified that:

1. The `transformer` feature pulls in `oxc_transformer`, which includes babel-compatibility layers, TypeScript stripping, JSX transformation, and module import management via `ModuleImportsStore` and `StatementInjectorStore`.
2. We only need `ModuleImportsStore`-like functionality (adding imports during traversal), which we implement directly in `import_rewrite.rs` and the `exit_program` hook -- simpler and avoids the babel-compat dependency tree.
3. If JSX transpilation is needed (when `transpile_jsx: true`), we can add the `transformer` feature later without changing the crate architecture.

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `parallel` | on | Enables `rayon` for parallel multi-file transforms. Disable for WASM targets. |
| `source-maps` | on | Enables `base64` for source map generation. Disable to reduce binary size if source maps are not needed. |

### Minimum Supported Rust Version

Rust edition 2024 requires rustc 1.85+. This matches the OXC umbrella crate's MSRV.
