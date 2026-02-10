# Architecture Research: OXC Optimizer Crate

**Domain:** Qwik optimizer port from SWC to OXC -- crate structure, visitor architecture, multi-output pipeline
**Researched:** 2026-02-10
**Confidence:** MEDIUM-HIGH (OXC APIs verified via docs.rs; multi-output architecture is novel design with no direct precedent in OXC ecosystem)

## Recommended Architecture: Two-Phase Pipeline with Collector + Emitter

Build the OXC optimizer as a single Rust crate (`oxc-optimizer`) with a two-phase pipeline:

1. **Analysis phase** -- traverse the AST once, collecting all `$`-boundary sites, imports, exports, variable bindings, capture relationships, and scope information. Produce a `TransformPlan` that describes every mutation and extraction needed.
2. **Emit phase** -- execute the plan: mutate the main module AST in-place, construct separate `Program` ASTs for each extracted segment, run codegen on all programs.

This decouples "what to do" from "doing it." The SWC optimizer interleaves analysis and mutation in a single `Fold` pass, which works because SWC's fold-by-ownership model naturally returns new ASTs. OXC's `Traverse` trait gives mutable references to nodes in-place, making interleaved collect-and-mutate harder to reason about and more error-prone. A clear phase separation also makes the multi-output requirement manageable: phase 1 identifies which code goes where, phase 2 builds the output ASTs.

## System Overview

```
                            INPUT
    +----------------------------------------------------------+
    |  Source code (String) + TransformModulesOptions           |
    +---------------------------+------------------------------+
                                |
                                v
    +----------------------------------------------------------+
    |  Phase 0: PARSE                                          |
    |                                                          |
    |  oxc_parser -> Program<'a>                               |
    |  oxc_semantic::SemanticBuilder -> Semantic                |
    |    (ScopeTree, SymbolTable, ReferenceTable)              |
    +---------------------------+------------------------------+
                                |
                                v
    +----------------------------------------------------------+
    |  Phase 1: ANALYZE (single Traverse pass)                 |
    |                                                          |
    |  impl Traverse for QwikAnalyzer                          |
    |    - Detect $-boundary call sites (component$, $, etc.)  |
    |    - Collect imports, exports, root declarations          |
    |    - Perform capture analysis per $-boundary              |
    |    - Build scope-aware variable resolution                |
    |    - Classify const vs mutable bindings                  |
    |    - Identify JSX elements needing transformation         |
    |    - Record strip/filter decisions                        |
    |                                                          |
    |  Output: TransformPlan                                   |
    |    - Vec<SegmentPlan> (what to extract)                  |
    |    - Vec<ImportRewrite> (import mutations)               |
    |    - Vec<CallRewrite> (foo$ -> fooQrl mutations)         |
    |    - Vec<JsxTransform> (JSX rewrite instructions)        |
    |    - Vec<StripAction> (exports/code to remove)           |
    |    - CaptureMap (variable -> segment mapping)            |
    +---------------------------+------------------------------+
                                |
                                v
    +----------------------------------------------------------+
    |  Phase 2: EMIT (multiple outputs)                        |
    |                                                          |
    |  2a. Mutate main module AST in-place:                    |
    |      - Rewrite imports (add qrl, remove unused)          |
    |      - Replace $-calls with qrl()/inlinedQrl() calls     |
    |      - Transform JSX to _jsxSorted/_jsxSplit             |
    |      - Insert lazy import declarations                   |
    |      - Apply #__PURE__ annotations                       |
    |      - Strip exports/code per configuration              |
    |                                                          |
    |  2b. Build segment ASTs (one per extracted $-boundary):  |
    |      - Construct new Program per segment                 |
    |      - Add required imports (captured vars, dependencies)|
    |      - Build exported function/expression body           |
    |      - Apply props destructuring rewrites                |
    |                                                          |
    |  2c. Codegen all programs:                               |
    |      - oxc_codegen::Codegen for main module              |
    |      - oxc_codegen::Codegen for each segment             |
    |      - Source maps for each output                       |
    |                                                          |
    |  Output: TransformOutput                                 |
    |    - Vec<TransformModule> (main + segments)              |
    |    - Vec<SegmentAnalysis> (segment metadata)             |
    |    - Vec<Diagnostic> (warnings/errors)                   |
    +----------------------------------------------------------+
```

## Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| **`lib.rs`** (public API) | `transform_modules()` entry point; option validation; orchestrates pipeline | All internal modules |
| **`parse.rs`** | Parse source -> `Program`; run `SemanticBuilder`; produce `Semantic` with scope/symbol tables | `lib.rs`, `analyze.rs` |
| **`analyze.rs`** | `QwikAnalyzer` impl Traverse; single-pass analysis producing `TransformPlan` | `parse.rs` (reads AST), `types.rs` (plan types) |
| **`collector.rs`** | Import/export/root-declaration collection; helper used by analyzer | `analyze.rs` |
| **`capture.rs`** | Capture analysis: determine which variables cross `$` boundaries | `analyze.rs`, `collector.rs` |
| **`emit.rs`** | Execute `TransformPlan` against main AST; coordinate segment building | `analyze.rs` (reads plan), `segment_builder.rs`, `codegen_bridge.rs` |
| **`segment_builder.rs`** | Construct new `Program` ASTs for extracted segments from plan data | `emit.rs`, `types.rs` |
| **`jsx.rs`** | JSX-specific transformation logic (`_jsxSorted`, `_jsxSplit`, signal wrapping) | `analyze.rs`, `emit.rs` |
| **`codegen_bridge.rs`** | Wrap `oxc_codegen::Codegen`; produce code strings + source maps for each Program | `emit.rs` |
| **`entry_strategy.rs`** | Entry strategy logic (segment, inline, smart, hook, component) | `analyze.rs`, `emit.rs` |
| **`strip.rs`** | Export filtering, ctx_name stripping, event handler removal | `analyze.rs`, `emit.rs` |
| **`const_fold.rs`** | Dead code elimination (`if (false)` removal), `isServer`/`isBrowser`/`isDev` replacement | `emit.rs` |
| **`hash.rs`** | Segment hash generation (content-based hash for canonical filenames) | `segment_builder.rs` |
| **`types.rs`** | All shared types: `TransformPlan`, `SegmentPlan`, `SegmentAnalysis`, options, output structs | All modules |
| **`diagnostics.rs`** | Diagnostic creation, error/warning collection | All modules |

## Recommended Crate Layout

```
oxc-optimizer/
+-- Cargo.toml
+-- src/
|   +-- lib.rs                 # Public API: transform_modules(), re-exports
|   +-- types.rs               # TransformModulesOptions, TransformOutput,
|   |                          # SegmentAnalysis, TransformModule, Diagnostic,
|   |                          # TransformPlan, SegmentPlan (internal)
|   +-- parse.rs               # Parse + semantic analysis wrapper
|   +-- analyze.rs             # QwikAnalyzer: impl Traverse, produces TransformPlan
|   +-- collector.rs           # GlobalCollect: imports, exports, root decls
|   +-- capture.rs             # CaptureAnalyzer: cross-$-boundary variable resolution
|   +-- emit.rs                # PlanExecutor: mutates main AST, coordinates segments
|   +-- segment_builder.rs     # Build new Program ASTs for extracted segments
|   +-- jsx.rs                 # JSX transformation (_jsxSorted, _jsxSplit, signals)
|   +-- codegen_bridge.rs      # oxc_codegen wrapper with source map support
|   +-- entry_strategy.rs      # EntryStrategy enum and segment grouping logic
|   +-- strip.rs               # Export/code stripping (server/client filtering)
|   +-- const_fold.rs          # Constant folding, dead code elimination
|   +-- hash.rs                # Content-based hashing for segment names
|   +-- diagnostics.rs         # Diagnostic helpers
|   +-- words.rs               # String constants (BUILDER_IO_QWIK, qrl names, etc.)
+-- tests/
|   +-- spec_runner.rs         # Harness that loads .planning/spec/*.md files
|   +-- spec_parser.rs         # Parse spec markdown into test expectations
|   +-- integration.rs         # Hand-written integration tests
+-- benches/
    +-- transform.rs           # Benchmark against representative inputs
```

### Layout Rationale

**Why mirror the SWC module names where possible** (`collector.rs`, `entry_strategy.rs`, `types.rs`): The 162 spec files reference these concepts by their SWC names. Keeping module names recognizable reduces cognitive load when cross-referencing specs during implementation. Names like `parse.rs`, `collector.rs`, `entry_strategy.rs` map directly to their SWC counterparts.

**Why split `analyze.rs` from `emit.rs`**: The SWC optimizer combines analysis and mutation in `QwikTransform::fold_*`. This made sense for SWC's ownership-based fold model but is the wrong pattern for OXC's mutable-reference traverse. Splitting into analyze (read-only traverse) + emit (targeted mutations) prevents the aliasing complexity that OXC's safety model is designed to avoid.

**Why `segment_builder.rs` is separate from `emit.rs`**: Building a new `Program` from scratch (for an extracted segment) is a fundamentally different operation from mutating an existing AST in-place. Segment building uses `AstBuilder` to construct fresh nodes; emit uses `Traverse` to modify existing nodes. Different tools, different module.

**Why `capture.rs` is separate from `collector.rs`**: Collector gathers module-level facts (imports, exports, declarations). Capture analysis reasons about scope boundaries and determines which variables need to cross `$` boundaries. Collector is a prerequisite for capture analysis, not the same concern.

## OXC Visitor Architecture vs SWC

### SWC: `VisitMut` / `Fold` Ownership Transfer

```rust
// SWC pattern: take ownership, return modified
impl Fold for QwikTransform {
    fn fold_call_expr(&mut self, node: CallExpr) -> CallExpr {
        // Process children first (they're owned, returned modified)
        let node = node.fold_children_with(self);
        // Then process this node
        self.maybe_rewrite_call(node)
    }
}
```

Key properties:
- `fold_children_with(self)` recurses automatically; children transformed before parent sees them
- Ownership model means you return a *new* node (possibly different type via `Expr` -> different `Expr` variant)
- Single struct (`QwikTransform`) accumulates both analysis state and transformation decisions
- No separate semantic analysis pass; SWC's `SyntaxContext` (hygiene marks) provides scope info inline

### OXC: `Traverse` with Mutable References + `TraverseCtx`

```rust
// OXC pattern: mutable reference to node + context
impl<'a> Traverse<'a> for QwikAnalyzer<'a> {
    fn enter_call_expression(
        &mut self,
        node: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // Read parent context
        if let Some(parent) = ctx.parent() { /* ... */ }
        // Access scoping info
        let scoping = ctx.scoping();
        // Record analysis findings (no mutation yet in analysis phase)
        self.plan.record_dollar_call(node, ctx);
    }

    fn exit_call_expression(
        &mut self,
        node: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // exit_* fires after children have been visited
        // Can mutate node here if needed
    }
}
```

Key properties:
- `enter_*` fires before children; `exit_*` fires after children (explicit control)
- Mutable reference to current node; immutable access to ancestors via `Ancestor` enum
- `TraverseCtx` provides: `ctx.ast` (AstBuilder), `ctx.scoping()` (scope/symbol tables), `ctx.parent()` (ancestor stack)
- Requires `oxc_semantic` analysis before traversal (separate pass)
- Arena allocator means all constructed nodes live in the same allocator scope

### Critical Differences for the Optimizer

| Concern | SWC Approach | OXC Approach |
|---------|-------------|-------------|
| Traversal order | Implicit via `fold_children_with` call position | Explicit via `enter_*` (pre-order) / `exit_*` (post-order) |
| Node replacement | Return different node from fold method | Assign to `*node = new_node` on mutable reference |
| New node creation | Standard heap allocation (`Box::new(...)`) | Arena allocation via `ctx.ast.alloc(...)`, `ctx.ast.vec(...)` |
| Scope resolution | `SyntaxContext` on every identifier (inline hygiene) | `SemanticBuilder` pre-pass: `ScopeTree` + `SymbolTable` |
| Variable identity | `Id = (Atom, SyntaxContext)` equality | `SymbolId` equality from `oxc_semantic` |
| Statement insertion | Return `Vec<Stmt>` from fold (expands parent) | `StatementInjection` or batch post-processing |
| Statement deletion | Return empty vec or `Stmt::Empty` | Filter in `exit_statements` or mark with `Statement::Empty` |
| Multi-output | Build separate `Module` ASTs during fold | Build separate `Program` ASTs after analysis (using `AstBuilder`) |

### Mapping the SWC Optimizer's `fold_*` Methods to OXC

The SWC `QwikTransform` overrides these fold methods. The table shows where each maps in the two-phase OXC architecture:

| SWC Method | What It Does | OXC Phase | OXC Location |
|-----------|-------------|-----------|-------------|
| `fold_module` | Entry point, post-processes segments after folding all children | Analysis: `enter_program` / `exit_program`; Emit: top-level orchestration in `emit.rs` | `analyze.rs`, `emit.rs` |
| `fold_module_items` | Processes all top-level statements, removes/rewrites as needed | Analysis: `enter_statements` / `exit_statements`; Emit: batch statement mutation | `analyze.rs`, `emit.rs` |
| `fold_call_expr` | Detects `$`-boundary calls, rewrites to `qrl()`/`inlinedQrl()` | Analysis: `enter_call_expression` records plan; Emit: node replacement | `analyze.rs`, `emit.rs` |
| `fold_expr` | Signal wrapping, const replacement, general expression rewrites | Analysis: `enter_expression`; Emit: expression mutation | `analyze.rs`, `emit.rs`, `jsx.rs` |
| `fold_var_declarator` | Tracks declarations for scope analysis | Analysis: `enter_variable_declarator` | `analyze.rs`, `collector.rs` |
| `fold_fn_decl` / `fold_arrow` | Tracks function boundaries for capture analysis | Analysis: `enter_function` / `enter_arrow_function_expression` | `analyze.rs`, `capture.rs` |
| `fold_import_decl` | Collects imports, rewrites import sources | Analysis: `enter_import_declaration`; Emit: import rewriting | `collector.rs`, `emit.rs` |
| `fold_export_decl` | Collects exports, filters per strip config | Analysis: `enter_export_named_declaration`; Emit: export filtering | `collector.rs`, `strip.rs` |
| `fold_jsx_element` | JSX transformation to `_jsxSorted`/`_jsxSplit` | Analysis: `enter_jsx_element` records plan; Emit: JSX rewriting | `jsx.rs` |

## Handling Multi-Module Output

This is the optimizer's most architecturally unique requirement. A single input produces N output modules (1 main + M segments). Neither SWC nor OXC natively supports this -- the SWC optimizer handles it with manual `Module` construction in `code_move.rs`. The OXC optimizer must do the same.

### Strategy: Collect-Then-Build

During the analysis phase, the analyzer records each `SegmentPlan`:

```rust
struct SegmentPlan {
    // Identity
    name: String,               // e.g. "Foo_component_HTDRsvUbLiE"
    display_name: String,       // e.g. "test.tsx_Foo_component"
    hash: String,               // e.g. "HTDRsvUbLiE"
    canonical_filename: String, // e.g. "test.tsx_Foo_component_HTDRsvUbLiE"
    extension: String,          // "tsx", "ts", "js"

    // What to extract
    body_span: Span,            // Span of the callback body in original AST
    ctx_kind: CtxKind,          // Function or Event
    ctx_name: String,           // "component$", "useStyles$", "$", etc.

    // Dependencies
    captured_vars: Vec<CapturedVar>,   // Variables crossing $ boundary
    required_imports: Vec<ImportSpec>,  // Imports the segment needs
    parent_segment: Option<String>,    // Parent segment name (for nesting)

    // Parameters
    param_names: Vec<String>,          // Original parameter names
    needs_raw_props: bool,             // Whether to rewrite params to _rawProps
}
```

During the emit phase, `segment_builder.rs` takes each `SegmentPlan` and constructs a fresh `Program`:

```rust
fn build_segment_program<'a>(
    plan: &SegmentPlan,
    original_program: &Program<'a>,
    allocator: &'a Allocator,
) -> Program<'a> {
    let ast = AstBuilder::new(allocator);

    // 1. Build import declarations
    let mut body = ast.vec();
    for import in &plan.required_imports {
        body.push(ast.import_declaration(/* ... */));
    }

    // 2. Build the lazy import declarations (for nested segments)
    for child_segment in &plan.child_lazy_imports {
        body.push(build_lazy_import_decl(&ast, child_segment));
    }

    // 3. Build the exported function/expression
    let export = build_segment_export(&ast, plan, original_program);
    body.push(export);

    // 4. Assemble Program
    Program {
        source_type: SourceType::mjs(),
        body,
        // ... other fields
    }
}
```

### Why Not Clone-and-Prune the Original AST

An alternative would be to clone the entire original `Program` and remove what each segment does not need. This is wrong because:

1. **OXC's arena allocator makes cloning expensive**: Every node is arena-allocated. Cloning a Program means allocating an entirely new arena's worth of nodes, then deleting most of them.
2. **Segments are structurally different from the original**: A segment's top-level is an `export const X = (params) => { ... }`. The original has imports, multiple statements, JSX. There is no meaningful structural overlap to preserve.
3. **Source maps are cleaner from built ASTs**: Building fresh ASTs with correct spans allows clean source map generation pointing back to the original source.

### Source Map Strategy for Multiple Outputs

Each output (main module + each segment) needs its own source map. OXC's `Codegen` supports source map generation when `CodegenOptions::source_map_path` is set.

```rust
fn codegen_with_sourcemap<'a>(
    program: &Program<'a>,
    source_text: &str,
    filename: &str,
) -> (String, Option<String>) {
    let codegen = Codegen::new()
        .with_options(CodegenOptions {
            source_map_path: Some(PathBuf::from(filename)),
            ..Default::default()
        })
        .with_source_text(source_text);

    let ret = codegen.build(program);
    let source_map = ret.source_map.map(|sm| sm.to_json_string());
    (ret.code, source_map)
}
```

For segments, the source text and spans must point back to the *original* input file, not to the segment's synthetic code. This requires careful span management: when building segment ASTs with `AstBuilder`, preserve the original spans from the input source where possible (for the extracted function body), and use `SPAN` (zero span) for synthetic constructs (import declarations, export wrapper).

## Pipeline Integration: Parse -> Semantic -> Traverse -> Codegen

The complete pipeline for a single input module:

```rust
pub fn transform_modules(options: &TransformModulesOptions) -> TransformOutput {
    let allocator = Allocator::default();

    // Phase 0: Parse
    let source_type = SourceType::from_path(&options.filename)
        .unwrap_or(SourceType::tsx());
    let parser_ret = Parser::new(&allocator, &options.code, source_type)
        .with_options(ParseOptions::default())
        .parse();

    if parser_ret.panicked {
        return TransformOutput::with_error(/* parse error */);
    }

    let mut program = parser_ret.program;

    // Phase 0b: Semantic analysis (required for scope resolution)
    let semantic_ret = SemanticBuilder::new()
        .with_check_syntax_error(true)
        .build(&program);

    let (symbols, scopes) = semantic_ret.semantic.into_symbol_table_and_scope_tree();

    // Phase 1: Analyze
    let mut analyzer = QwikAnalyzer::new(options, &symbols, &scopes);
    // Run traverse with semantic info
    traverse_mut(&mut analyzer, &allocator, &mut program, symbols, scopes);
    let plan = analyzer.into_plan();

    // Phase 2: Emit
    let mut emitter = PlanExecutor::new(&plan, &allocator);

    // 2a: Mutate main module AST
    emitter.apply_main_module_mutations(&mut program);

    // 2b: Build segment ASTs
    let segment_programs: Vec<(Program, SegmentAnalysis)> = plan.segments
        .iter()
        .map(|seg_plan| emitter.build_segment(seg_plan, &program))
        .collect();

    // 2c: Codegen
    let main_output = codegen_module(&program, &options.code, &options.filename);
    let segment_outputs: Vec<TransformModule> = segment_programs
        .iter()
        .map(|(prog, analysis)| codegen_segment(prog, &options.code, analysis))
        .collect();

    TransformOutput {
        modules: std::iter::once(main_output)
            .chain(segment_outputs)
            .collect(),
        diagnostics: emitter.diagnostics(),
    }
}
```

### Lifetime Management

The OXC arena allocator ties all AST nodes to the `Allocator` lifetime. This affects architecture:

- The `Allocator` must live as long as any `Program` reference
- Segment `Program` ASTs can share the same allocator as the main program (they reference the same lifetime)
- Codegen serializes to `String`, which escapes the arena lifetime -- this is where ownership transfers to the output
- The entire pipeline runs within a single allocator scope; after codegen, the allocator is dropped

This means the pipeline is inherently single-threaded per input file. Parallelism happens at the file level (multiple inputs in parallel via `rayon`), not within a single file's pipeline.

## Public API Boundary

The public API must match the existing SWC optimizer for drop-in replacement:

```rust
// -- Public types (re-exported from lib.rs) --

/// Configuration for transform_modules()
pub struct TransformModulesOptions {
    pub filename: String,
    pub src_dir: String,
    pub code: String,
    pub source_maps: bool,
    pub entry_strategy: EntryStrategy,
    pub minify: MinifyMode,
    pub transpile_ts: bool,
    pub transpile_jsx: bool,
    pub preserve_filenames: bool,
    pub explicit_extensions: bool,
    pub mode: EmitMode,
    pub scope: Option<String>,
    pub core_module: Option<String>,
    pub is_server: Option<bool>,
    pub strip_exports: Option<Vec<String>>,
    pub strip_ctx_name: Option<Vec<String>>,
    pub reg_ctx_name: Option<Vec<String>>,
    pub strip_event_handlers: bool,
}

/// Complete transformation result
pub struct TransformOutput {
    pub modules: Vec<TransformModule>,
    pub diagnostics: Vec<Diagnostic>,
}

/// A single output module (main or segment)
pub struct TransformModule {
    pub path: String,
    pub code: String,
    pub map: Option<String>,  // Source map JSON
    pub is_entry: bool,
    pub segment: Option<SegmentAnalysis>,
}

/// Metadata about an extracted segment
pub struct SegmentAnalysis {
    pub origin: String,
    pub name: String,
    pub entry: Option<String>,
    pub display_name: String,
    pub hash: String,
    pub canonical_filename: String,
    pub path: String,
    pub extension: String,
    pub parent: Option<String>,
    pub ctx_kind: String,
    pub ctx_name: String,
    pub captures: bool,
    pub loc: (u32, u32),
}

/// Entry point
pub fn transform_modules(options: TransformModulesOptions) -> TransformOutput;
```

Everything else is `pub(crate)` internal. The TypeScript plugin layer calls `transform_modules()` through the NAPI/WASM binding and receives JSON-serialized `TransformOutput`. The public API types derive `serde::Serialize` and `serde::Deserialize` for this bridge.

## Test Strategy Against 162 Spec Files

### Spec-Driven Test Harness

The 162 spec files at `.planning/spec/*.md` are the single source of truth. The test strategy parses each spec file, extracts input and expected output, runs the OXC optimizer, and compares results.

```rust
// tests/spec_runner.rs

#[test]
fn run_all_specs() {
    let spec_dir = Path::new("../.planning/spec");
    let specs = parse_all_specs(spec_dir);

    let mut passed = 0;
    let mut failed = 0;
    let mut skipped = 0;

    for spec in &specs {
        let result = transform_modules(spec.to_options());
        match compare_output(&result, &spec.expected) {
            SpecResult::Pass => passed += 1,
            SpecResult::Fail(diff) => {
                failed += 1;
                eprintln!("FAIL: {} -- {}", spec.name, diff);
            }
            SpecResult::Skip(reason) => {
                skipped += 1;
                eprintln!("SKIP: {} -- {}", spec.name, reason);
            }
        }
    }

    assert_eq!(failed, 0, "{failed} spec(s) failed out of {}", specs.len());
}
```

### Comparison Strategy

Output comparison is NOT byte-for-byte. The OXC codegen will produce slightly different whitespace and formatting than SWC's codegen. Comparison must be **semantic**:

1. **Parse both expected and actual output** with `oxc_parser`
2. **Compare AST structure** (node types, identifiers, string values) ignoring spans
3. **Compare segment metadata** field-by-field (hashes may differ -- compare structure, not values)
4. **Compare diagnostic presence** (same errors/warnings, different messages are OK)

For hashes specifically: the hash depends on content, so if the codegen produces different whitespace, the hash will differ. The test harness should either:
- Strip hashes from comparison and verify hash format only (11-char alphanumeric)
- Re-hash the actual output and compare structure

### Test Tiers

| Tier | Count | Purpose | When to Run |
|------|-------|---------|-------------|
| **Smoke** | ~10 specs | Core transformations (component$, $, useStyles$, basic JSX) | Every `cargo test` |
| **Full** | All 162 | Complete behavioral verification | CI, pre-merge |
| **Unit** | ~50+ | Individual module tests (collector, capture, jsx, strip) | Every `cargo test` |
| **Benchmark** | ~5 | Performance regression detection | CI nightly |

### Spec Parser Module

A `spec_parser.rs` module reads the markdown files and extracts structured test data:

```rust
struct ParsedSpec {
    name: String,
    config: TestConfig,
    input_code: String,
    expected_modules: Vec<ExpectedModule>,
    expected_diagnostics: Vec<String>,
}

struct ExpectedModule {
    path: String,
    is_entry: bool,
    code: String,
    segment_metadata: Option<serde_json::Value>,
}
```

The spec parser reuses the same delimiter patterns documented in the v1.0 architecture research. It must handle: fenced code blocks (` ```tsx ... ``` `), config tables, segment metadata JSON blocks, and the conventions section (for cross-reference but not used in pass/fail comparison).

## Data Flow Diagram

```
TransformModulesOptions
    |
    v
+-- parse.rs ----+
|  Parser::new() |    +-- Allocator (shared lifetime 'a) --+
|  SemanticBuilder|   |  All AST nodes live here           |
+---------+------+    +------------------------------------+
          |
          v
    Program<'a> + Semantic (ScopeTree, SymbolTable)
          |
          v
+-- analyze.rs --------------------+
|  QwikAnalyzer: impl Traverse     |
|  Single read pass over AST       |
|                                  |
|  Uses:                           |
|   - collector.rs (imports/exports)|
|   - capture.rs (scope analysis)  |
|   - entry_strategy.rs (grouping) |
|   - jsx.rs (JSX detection)       |
|   - strip.rs (filter decisions)  |
+------------------+---------------+
                   |
                   v
             TransformPlan
                   |
                   v
+-- emit.rs --------------------------+
|  PlanExecutor                        |
|                                      |
|  1. Mutate main Program<'a> in-place |
|     (import rewrites, $->qrl, JSX)  |
|                                      |
|  2. Build segment Programs           |  +-- segment_builder.rs --+
|     (AstBuilder constructs new ASTs) |->|  Fresh Program per     |
|                                      |  |  extracted segment     |
|  3. Run Codegen on all Programs      |  +------------------------+
|                                      |
|     +-- codegen_bridge.rs --------+  |
|     |  Codegen::new()             |  |
|     |  .build(&program)           |  |
|     |  -> (code: String, map)     |  |
|     +-----------------------------+  |
+------------------+-------------------+
                   |
                   v
            TransformOutput
            {
              modules: Vec<TransformModule>,
              diagnostics: Vec<Diagnostic>,
            }
```

## Patterns to Follow

### Pattern 1: Plan-Then-Execute (Two-Phase Transform)

**What:** Separate "decide what to do" (analysis) from "do it" (emission). The analyzer reads the AST and builds a data-driven plan. The emitter reads the plan and performs mutations.

**When:** Always. This is the core architectural pattern.

**Why:** OXC's `Traverse` gives mutable references. Interleaving reads and writes during a single pass is possible but fragile -- you cannot read ahead in the AST while mutating the current node. The plan acts as a communication channel between the read pass and the write pass.

**Example:**

```rust
// Phase 1: Analyze (read-only intent)
struct QwikAnalyzer<'a> {
    plan: TransformPlan,
    collector: GlobalCollect,
    scope_stack: Vec<ScopeInfo>,
    // ...
}

impl<'a> Traverse<'a> for QwikAnalyzer<'a> {
    fn enter_call_expression(&mut self, node: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        if self.is_dollar_call(node) {
            self.plan.add_segment(SegmentPlan::from_call(node, ctx));
        }
    }
}

// Phase 2: Execute (mutate)
struct PlanExecutor<'a> {
    plan: &'a TransformPlan,
    ast: AstBuilder<'a>,
}

impl PlanExecutor<'_> {
    fn apply_main_module_mutations(&self, program: &mut Program) {
        for rewrite in &self.plan.call_rewrites {
            // Targeted mutation at known spans
        }
    }
}
```

### Pattern 2: Scope-Aware Variable Identity via `oxc_semantic`

**What:** Use `SymbolId` from `oxc_semantic` instead of name-based string matching for variable identity. Two identifiers refer to the same binding if and only if they resolve to the same `SymbolId`.

**When:** Always for capture analysis, import tracking, and any cross-scope reasoning.

**Why:** Name-based matching breaks on shadowed variables. The SWC optimizer used `SyntaxContext` for this; OXC uses `SymbolId`.

**Example:**

```rust
fn is_same_binding(
    ref_a: &IdentifierReference,
    ref_b: &IdentifierReference,
    scoping: &Scoping,
) -> bool {
    let sym_a = ref_a.symbol_id();
    let sym_b = ref_b.symbol_id();
    match (sym_a, sym_b) {
        (Some(a), Some(b)) => a == b,
        _ => false, // Unresolved references are never "same"
    }
}
```

### Pattern 3: Fresh AST Construction for Segments

**What:** Build each segment's `Program` from scratch using `AstBuilder` rather than cloning/pruning the original AST.

**When:** Always for segment output modules.

**Why:** Segments have different structure than the original. Building fresh gives precise control over what each segment contains and produces clean source maps.

## Anti-Patterns to Avoid

### Anti-Pattern 1: Single-Pass Analyze-and-Mutate

**What:** Combining analysis and mutation in a single `Traverse` pass, mirroring SWC's `Fold` pattern.

**Why bad:** OXC's mutable reference model means you cannot safely read ahead in the AST while mutating the current node. The SWC `Fold` avoids this because ownership transfer creates new nodes. In OXC, mutating a `CallExpression` while also trying to analyze its children in the same pass creates confusing control flow and hard-to-debug ordering bugs.

**Instead:** Two-phase: analyze (read-only traverse) then emit (targeted mutations + segment construction).

### Anti-Pattern 2: Manual Scope Tracking Instead of `oxc_semantic`

**What:** Maintaining a manual `decl_stack: Vec<Vec<Binding>>` like the SWC optimizer does.

**Why bad:** `oxc_semantic` already builds a complete scope tree, symbol table, and reference table. Manual tracking duplicates this work, introduces bugs, and misses edge cases that semantic analysis handles correctly (e.g., `catch` clause bindings, class field initializers, `for-in`/`for-of` bindings).

**Instead:** Run `SemanticBuilder` after parsing. Use `Scoping::find_binding()`, `SymbolTable::get_scope_id()`, and `ReferenceTable` for all scope queries.

### Anti-Pattern 3: String-Based Code Generation for Segments

**What:** Generating segment JavaScript by string concatenation/template rather than building AST + codegen.

**Why bad:** String-based generation cannot produce source maps, is fragile with escaping, and produces inconsistent formatting. It also cannot be verified by re-parsing.

**Instead:** Build `Program` ASTs via `AstBuilder`, run `Codegen::build()` for each.

### Anti-Pattern 4: Cloning the Original Program for Each Segment

**What:** `program.clone()` then removing everything the segment does not need.

**Why bad:** Arena-allocated ASTs are expensive to clone (requires new allocation for every node). The resulting AST has leftover structure (empty statement slots, orphan scopes) that confuses codegen. And the deletion logic is more complex than construction logic.

**Instead:** Build each segment's `Program` fresh from `SegmentPlan` data.

## Suggested Build Order

The modules have clear dependency ordering. Build in this sequence:

```
Layer 1: Foundation (no internal dependencies)
    types.rs          -- All shared data structures
    words.rs          -- String constants
    hash.rs           -- Hashing utility
    diagnostics.rs    -- Diagnostic helpers
    entry_strategy.rs -- EntryStrategy enum

Layer 2: Parsing (depends on Layer 1)
    parse.rs          -- Parser + SemanticBuilder wrapper

Layer 3: Analysis (depends on Layers 1-2)
    collector.rs      -- Import/export/declaration collection
    capture.rs        -- Capture analysis (depends on collector)
    jsx.rs            -- JSX analysis (detection, not mutation)
    strip.rs          -- Strip decision logic
    analyze.rs        -- QwikAnalyzer combining all analysis

Layer 4: Emission (depends on Layers 1-3)
    segment_builder.rs -- Build segment Program ASTs
    const_fold.rs      -- Dead code elimination
    codegen_bridge.rs  -- Codegen wrapper
    emit.rs            -- PlanExecutor coordinating all emission

Layer 5: Integration (depends on all layers)
    lib.rs             -- Public API, pipeline orchestration
```

**Key dependencies:**
- `analyze.rs` depends on `collector.rs`, `capture.rs`, `jsx.rs`, `strip.rs`, `entry_strategy.rs`
- `emit.rs` depends on `segment_builder.rs`, `const_fold.rs`, `codegen_bridge.rs`
- `lib.rs` depends on `parse.rs`, `analyze.rs`, `emit.rs`
- `capture.rs` depends on `collector.rs` (capture analysis needs import/export knowledge)

**Build Layer 1 first, then 2, then 3 and 4 can partially overlap.** Within Layer 3, `collector.rs` must come before `capture.rs`. Within Layer 4, `segment_builder.rs` and `codegen_bridge.rs` must come before `emit.rs`.

## Scalability Considerations

| Concern | At 100 modules/build | At 10K modules/build | At 100K modules/build |
|---------|---------------------|---------------------|-----------------------|
| Per-file transform time | ~1ms (OXC parser + single traverse + codegen) | Same per file | Same per file |
| Memory per file | One arena allocator per file, dropped after codegen (~1-5MB) | Same | Same |
| Parallelism | `rayon::par_iter` over input files | Essential; memory scales linearly with thread count | Consider batching to limit peak memory |
| Segment count per file | Typically 1-10 | Same | Same |
| Total segment programs | ~500 | ~50K | ~500K -- each is small, total memory is fine |

The optimizer is embarrassingly parallel at the file level. Each file's transform is independent. Memory management is natural: allocate arena, parse, analyze, emit, codegen (serializes to String), drop arena. Peak memory is proportional to the largest single input file, not the total project size.

## Sources

- [oxc_traverse crate](https://docs.rs/oxc_traverse/latest/oxc_traverse/) -- Traverse trait, TraverseCtx, Ancestor system (HIGH confidence)
- [oxc_ast AstBuilder](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) -- Node construction API, arena allocation methods (HIGH confidence)
- [oxc_semantic docs](https://docs.rs/oxc_semantic/latest/oxc_semantic/) -- SemanticBuilder, Scoping, SymbolTable, ScopeTree (HIGH confidence)
- [oxc Codegen docs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) -- CodegenReturn, source map via source_map_path (HIGH confidence)
- [oxc_sourcemap crate](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/) -- SourceMap type, JSON serialization (MEDIUM confidence)
- [OXC statement manipulation discussion](https://github.com/oxc-project/oxc/issues/6993) -- StatementInjection, insert/delete/replace patterns (MEDIUM confidence)
- [OXC AstBuilder move_* discussion](https://github.com/oxc-project/oxc/issues/5359) -- Expression::None, Statement::None for cheap replacement (MEDIUM confidence)
- [OXC transformer blog post](https://oxc.rs/blog/2024-09-29-transformer-alpha) -- Performance benchmarks, architecture overview (HIGH confidence)
- [oxc crate features](https://lib.rs/crates/oxc/features) -- Feature flag reference (HIGH confidence)
- [oxc_ast_builder_impl.rs on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast/src/ast_builder_impl.rs) -- AstBuilder source (HIGH confidence)
- `.planning/codebase/ARCHITECTURE.md` -- SWC optimizer architecture analysis (HIGH confidence, primary source)
- `.planning/codebase/STRUCTURE.md` -- SWC optimizer module layout (HIGH confidence, primary source)
- `.planning/codebase/CONCERNS.md` -- Known issues and fragile areas in SWC optimizer (HIGH confidence)
- `.planning/spec/*.md` -- 162 behavioral specifications (HIGH confidence, ground truth)
- `.planning/research/PITFALLS.md` -- v1.0 pitfalls research on SWC/OXC differences (HIGH confidence)

---
*Architecture research for: OXC optimizer crate structure and pipeline design*
*Researched: 2026-02-10*
