# Project Research Summary

**Project:** Qwik Optimizer SWC-to-OXC Port
**Domain:** Compiler transformation spec generation from SWC snapshot tests
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

The Qwik optimizer is a sophisticated JavaScript/TypeScript compiler transformation tool that enables lazy-loading through code-splitting. It parses Qwik components and transforms `$`-suffixed function boundaries into separate loadable modules with capture analysis. The current implementation uses SWC's AST and visitor pattern across 18 Rust files with 162 snapshot tests documenting expected transformations. The goal of this milestone is NOT to port the entire optimizer to OXC yet—it is to generate detailed specification files from the 162 existing snapshot tests that document the transformation behaviors in an implementation-agnostic way.

The recommended approach is to build a Rust CLI tool that parses the snapshot test files, uses `oxc_parser` to generate ASTs for both input and output code, and renders structured markdown specification files. This spec-first approach defers the complex transformation port until we have complete behavioral documentation. The tool will parse snapshot delimiters, extract test configurations from `test.rs`, generate OXC ASTs, catalog transformation conventions, and render one markdown spec per snapshot test.

The highest risks are: (1) accidentally documenting SWC-specific AST implementation details rather than semantic transformations, (2) missing implicit transformation ordering dependencies, and (3) incomplete capture analysis documentation. These are mitigated by focusing the spec on input/output contracts using actual JavaScript code, explicitly documenting traversal order requirements, and exhaustively enumerating capture rules with test coverage for edge cases like shadowed variables and loop iteration variables.

## Key Findings

### Recommended Stack

The spec generation tool should be a single-binary Rust CLI using OXC's umbrella crate with the `serialize` feature. This avoids version-sync headaches across OXC sub-crates while enabling JSON serialization of parsed ASTs. The umbrella crate provides `oxc_parser` (3x faster than SWC), `oxc_allocator` (arena allocator), `oxc_ast` (AST types with richer identifier distinctions), and `oxc_span` (SourceType for language detection).

**Core technologies:**
- **`oxc` v0.112.0 with `serialize` feature**: Single dependency for parser + AST + allocator; enables `serde::Serialize` on all AST types for JSON output
- **`serde_json` v1.x**: JSON serialization of OXC ASTs for spec files; already a dependency in the existing crate
- **`insta` v1.29+ with `json` feature**: Snapshot testing framework already used in the codebase; provides diffing and review workflow

**Why NOT use:** The `full` feature (pulls in codegen, minifier, transformer—unneeded for parsing), individual OXC sub-crates (version sync nightmare), or `ast_visit` feature (transformation traversal not needed for spec generation, only parse + serialize).

**Critical API difference:** OXC's arena allocator ties AST lifetime `'a` to the allocator. Unlike SWC's heap-allocated ASTs, OXC AST nodes cannot outlive their allocator. For spec generation this is straightforward (parse, serialize, done), but the future optimizer port will require careful lifetime management.

### Expected Features

The optimizer transforms Qwik code through a sophisticated pipeline. The spec must document all table-stakes features or be incomplete.

**Must have (table stakes):**
- **`$()` extraction into lazy-loadable segments**: The fundamental transformation—extracts closures into separate modules with hash-based names
- **`component$`/`foo$` to `*Qrl` conversion**: Rewrites all `$`-suffixed APIs to `Qrl` wrappers with lazy imports
- **Capture analysis**: Identifies variables captured across `$` boundaries; distinguishes local vs scoped identifiers
- **Segment metadata**: Each segment emits origin, name, hash, captures, displayName, and other metadata for build tools
- **Hash consistency**: Hashes must be identical across EmitMode and EntryStrategy combinations for runtime compatibility
- **JSX transformation**: Converts JSX to `_jsxSorted`/`_jsxSplit` calls with event handler naming (`onClick$` → `q-e:click`)
- **Derived signal optimization**: Wraps signal/store access with `_wrapProp` and computed expressions with `_fnSignal`
- **Props destructuring optimization**: Rewrites `({count}) => ...` to `(_rawProps) => { ... _rawProps.count ... }`
- **Entry strategies**: Seven strategies (Segment/Inline/Hoist/Single/Component/Smart/Hook) determine segment bundling
- **Emit modes**: Four modes (Prod/Dev/Test/Lib) control debug info and transformation depth
- **Code stripping**: `strip_exports`, `strip_ctx_name`, `strip_event_handlers` remove server-only code from client bundles
- **Import management**: Renames `@builder.io/qwik*` to `@qwik.dev/*`, generates synthetic imports, creates lazy import functions

**Should have (competitive):**
- **Custom inlined functions via `wrap()`**: Allows user-defined `$`-suffixed APIs
- **Input binding transformation**: `bind:value` generates two-way signal binding with merged event handlers
- **Synchronous QRL (`sync$`)**: Serializes handlers to strings for perf-critical cases
- **Build constant replacement**: `isServer`/`isBrowser`/`isDev` replaced with literals for dead code elimination

**Defer (v2+):**
- **Windows path normalization**: Infrastructure concern, not transformation spec
- **Parsed QRL passthrough**: Pre-compiled code handling, secondary concern
- **`@jsxImportSource` handling**: React interop edge case

### Architecture Approach

The spec generation pipeline is a deterministic, fail-forward pipeline architecture. Each component transforms data and passes it downstream with no backtracking. The pipeline has five stages: snapshot parser (delimiter-based), test config extractor (regex-based), AST generator (oxc_parser wrapper), convention analyzer (regex pattern detection), and markdown renderer (template-based).

**Major components:**
1. **Snapshot Parser** — Reads `.snap` files, splits on `==INPUT==` / `===== [path] ==` delimiters, extracts input code + output modules + segment metadata + diagnostics
2. **Test Config Extractor** — Parses `test.rs` with regex to extract `TestInput` struct fields per test (entry strategy, emit mode, transpile flags, strip options)
3. **AST Generator** — Parses code strings with `oxc_parser`, serializes to ESTree-compatible JSON using `serde_json::to_string_pretty`
4. **Convention Analyzer** — Walks output code with regex patterns to catalog optimizer conventions: `qrl()` calls, JSX transforms, signal wrapping, capture patterns
5. **Markdown Renderer** — Templates all data into structured markdown with collapsible AST details blocks

**Data flow:** Test name derived from snapshot filename maps to test function in test.rs. Join on test name. Parse input + output modules with oxc. Detect conventions. Render one markdown spec per test.

**Key architectural decisions:**
- Rust CLI, not Python/Node script: Target optimizer is Rust, `oxc_parser` is Rust, consolidating into one tool eliminates shell-out seams
- Fail-forward: If AST parsing fails for a module (some have intentional errors), log warning in spec rather than abort pipeline
- Deterministic output: Sort modules by snapshot order, consistent JSON formatting, no timestamps—spec files are committed to git
- Regex-based convention detection: Full AST walking is overkill for detecting patterns like `qrl(...)` or `_jsxSorted(...)`—string matching is sufficient

### Critical Pitfalls

The research identified seven critical pitfalls that would break the spec or port:

1. **Spec leaks SWC AST shapes instead of capturing semantics**: If the spec describes transformations using SWC-specific node types (`ast::CallExpr`, `SyntaxContext`) rather than semantic operations, the OXC port cannot use it. Mitigation: Describe transformations at three levels—semantic intent, input/output contract (actual JS code), and SWC implementation notes (separated section).

2. **SWC Fold ownership model implicitly shapes transformation order**: SWC's `Fold` trait processes children-then-parent or parent-then-children based on `fold_children_with()` call position. OXC's `Traverse` uses `enter_*`/`exit_*` callbacks. Transformations relying on implicit SWC ordering break silently. Mitigation: Document traversal requirements for every transformation—"children must be transformed BEFORE this rule applies" or "this rule must fire BEFORE children are visited."

3. **Identifier resolution model mismatch (SyntaxContext vs ScopeId/SymbolId)**: SWC identifies variables via `Id = (Atom, SyntaxContext)` with hygiene markers. OXC uses `oxc_semantic` with scope trees and symbol tables. The `id!` macro and `HashMap<Id, ...>` patterns do not translate. Mitigation: Define abstract `ResolvedIdentifier` concept in spec; map to SWC's `Id` and OXC's `SymbolId` in separate appendices.

4. **Capture analysis correctness is highest-risk transformation**: Determining which variables are captured across `$` boundaries is the core optimizer job. Getting it wrong = runtime errors or oversized bundles. The analysis spans multiple modules and handles: local vars, imports, exports, shadowing, hoisting, loop iteration vars, and destructured parameters. Mitigation: Exhaustively enumerate capture rules with test cases for each edge case.

5. **Snapshot tests define behavior but not intent**: The 162 snapshots show input/output pairs but don't explain why output is correct. Whitespace, import order, and source maps differ between SWC and OXC codegen. Which differences are acceptable vs breaking? Mitigation: Classify snapshot assertions into three tiers—semantic contract (MUST match), structural equivalence (AST-equivalent), incidental (may differ).

6. **OXC arena allocator lifetime constraints break SWC patterns**: SWC uses `Box<T>` and `Vec<T>`; OXC uses arena-allocated `Box<'a, T>` tied to allocator lifetime. Cannot store AST nodes outside arena, cannot clone trivially, must thread allocator through helpers. Mitigation: Spec should NOT prescribe AST construction patterns—describe semantically, port uses `AstBuilder` methods.

7. **JSX transformation has hidden mode interactions**: JSX handling is a matrix of modes: EmitMode × transpile_jsx × transpile_ts × is_server × entry_strategy × strip options. Interactions produce edge cases. Mitigation: Define mode matrix table stating which transformations are active in each combination; identify snapshot coverage gaps.

## Implications for Roadmap

Based on research, the milestone should be divided into three phases with clear dependencies. The critical insight is that **getting the data structures right determines everything downstream**—spend time on type definitions before writing parsing code.

### Phase 1: Foundation and Snapshot Parsing

**Rationale:** Types must be defined first because they form the contract between all components. Snapshot parsing can proceed in parallel after types exist and is independently testable. This phase validates we can extract all data from the 162 snapshots before attempting any AST generation or rendering.

**Delivers:**
- `types.rs` with all shared data structures: `ParsedSnapshot`, `OutputModule`, `SegmentMetadata`, `TestConfig`, `Convention` enum
- `snapshot.rs` parsing `.snap` files with delimiter handling: `==INPUT==`, `===== [path] ==`, `== DIAGNOSTICS ==`
- `test_config.rs` extracting `TestInput` fields from `test.rs` via regex
- Unit tests for snapshot parser with known snapshot content
- Verification that test name joining works (snapshot filename maps to test function)

**Addresses features:** Foundational infrastructure for all table-stakes features

**Avoids pitfall:** Establishes the abstraction layer (type definitions) that prevents spec from leaking SWC-specific shapes

**Research flag:** No additional research needed—parsing logic is straightforward delimiter splitting and regex matching

### Phase 2: AST Generation and Convention Detection

**Rationale:** After data extraction works, add AST generation using `oxc_parser` and convention detection. These are independent and can be built in parallel. AST generation validates that OXC can parse all input/output code. Convention detection catalogs transformation patterns visible in output code.

**Delivers:**
- `ast.rs` wrapping `oxc_parser` with `SourceType` inference from file extension
- JSON serialization via `serde_json::to_string_pretty(&ret.program)`
- Handling of parse failures (some outputs have intentional errors) with diagnostics in spec
- `conventions.rs` with regex patterns detecting: `qrl()` calls, `_jsxSorted()`, `_wrapProp()`, `_fnSignal()`, `_captures[N]`, lazy imports, `#__PURE__` annotations
- Verification that ASTs are valid ESTree JSON for all 162 snapshots

**Uses stack:** `oxc` v0.112.0 with `serialize` feature, `serde_json`

**Implements architecture:** AST Generator and Convention Analyzer components

**Avoids pitfall:** Fail-forward pattern handles parse errors without blocking other snapshots; convention detection uses regex (not AST walking) to avoid over-coupling to OXC AST structure

**Research flag:** MEDIUM confidence on exact OXC serialization API—verify during implementation that `serde_json::to_string_pretty` on `Program` produces expected ESTree JSON

### Phase 3: Markdown Rendering and Verification

**Rationale:** Once all data is extracted and analyzed, render structured markdown specs. This is the integration point that assembles everything. After rendering, verify a sample of specs manually and compare against source code to validate completeness.

**Delivers:**
- `renderer.rs` with markdown template rendering
- Spec file structure: test config table, input code, input AST (collapsible), output modules with code/AST/metadata/source map, conventions applied, function call catalog, diagnostics
- `main.rs` CLI orchestration with argument parsing (clap), parallel snapshot processing (optional rayon), error reporting
- 162 markdown specs in `.planning/spec/`
- Manual review of 5-10 representative specs against source code for completeness

**Addresses features:** Documents all table-stakes features through rendered specs

**Implements architecture:** Markdown Renderer component and pipeline orchestration

**Avoids pitfall:** Collapsible AST details blocks prevent spec files from being thousands of lines of JSON; deterministic output produces git-friendly diffs

**Research flag:** No additional research needed—markdown rendering is template-based formatting

### Phase Ordering Rationale

- **Types first** because all components depend on shared data structures; defining these upfront prevents refactoring cascade
- **Snapshot parsing before AST generation** because we must validate we can extract all data from snapshots before attempting to parse code strings
- **AST generation and convention detection in parallel** because they're independent after types exist; AST is input code → JSON, conventions are output code → pattern list
- **Rendering last** because it's the integration point that assembles all upstream outputs; building this too early means integrating with unstable APIs
- **No build-the-optimizer step** because this milestone is spec-only; deferring transformation implementation until specs exist avoids building on unstable foundation

### Research Flags

**Phases needing deeper research during planning:**
- **Phase 2 (AST generation)**: MEDIUM confidence on OXC serialization API—verify `serde_json` on `Program` produces ESTree JSON; may need wrapper or feature flags
- **Phase 3 (verification)**: Need to define what "completeness" means for specs—which aspects of transformations must be documented vs which can be inferred

**Phases with standard patterns (skip research-phase):**
- **Phase 1 (foundation)**: Snapshot file parsing is delimiter splitting; test config extraction is regex matching; both are well-understood
- **Phase 3 (rendering)**: Markdown template rendering is straightforward text formatting

**Post-milestone research needs:**
When this milestone completes, the next milestone (actual OXC optimizer port) will need:
- `/gsd:research-phase` for OXC `Traverse` API patterns and lifetime management
- `/gsd:research-phase` for `oxc_semantic` integration (ScopeId/SymbolId usage)
- Deep dive on mapping SWC's 20+ `fold_*` methods to OXC `enter_`/`exit_` patterns

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | OXC umbrella crate verified on crates.io, docs.rs, GitHub. Parser usage examples in official docs. Serde feature documented. |
| Features | HIGH | All findings derived from source code analysis (18 Rust files, 162 snapshot tests, 5388 lines of test.rs). Primary source, not secondary documentation. |
| Architecture | HIGH | Pipeline architecture is standard for batch processing. Component boundaries validated against existing codebase structure. Snapshot file format verified by examining 6+ snapshots. |
| Pitfalls | HIGH | Based on codebase analysis (SyntaxContext usage, Id type occurrences, fold_children_with patterns), official OXC docs on arena allocation and Traverse, and SWC architecture docs on hygiene. |

**Overall confidence:** HIGH

All research is grounded in primary sources: the existing SWC optimizer codebase, OXC official documentation, and verified crate metadata. The lowest-confidence element is the exact OXC AST serialization API (MEDIUM), which needs implementation-time verification.

### Gaps to Address

**Gap 1: OXC AST Serialization API**
- **Issue:** MEDIUM confidence on whether `serde_json::to_string_pretty(&ret.program)` produces ESTree JSON or if a wrapper/method is needed
- **Resolution:** During Phase 2 implementation, test with a simple code snippet. If direct serialization doesn't work, check for `to_estree_json()` methods or enable additional features. The `serialize` feature is confirmed to add `serde::Serialize` impls, so worst case is wrapping the AST.

**Gap 2: Spec Completeness Definition**
- **Issue:** No objective criteria for when a spec is "complete enough" to support the port
- **Resolution:** During Phase 3 verification, define completeness as: (1) input/output code visible, (2) all configuration options documented, (3) AST available for algorithmic extraction, (4) conventions cataloged. A spec is complete if a non-expert can read it and understand what transformation happened without reading source code.

**Gap 3: Snapshot Tier Classification**
- **Issue:** Need to classify which snapshot assertions are semantic contract vs structural equivalence vs incidental, but this requires domain expertise
- **Resolution:** Defer to post-milestone. This classification matters for the port phase (Milestone 2+) when comparing OXC output to SWC snapshots. For spec generation, document everything and let the port phase decide tiering.

## Sources

### Primary (HIGH confidence)
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/` — 18 Rust source files, codebase analysis
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/test.rs` — 5387 lines, 163 test functions, TestInput struct
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/snapshots/*.snap` — 162 snapshot files, format validation
- [oxc crate on crates.io](https://crates.io/crates/oxc) — v0.112.0, umbrella crate
- [oxc_parser docs.rs](https://docs.rs/oxc_parser) — Parser API, ParserReturn struct
- [oxc_ast docs.rs](https://docs.rs/oxc_ast) — AST types, serialize feature
- [OXC parser usage guide](https://oxc.rs/docs/guide/usage/parser.html) — Official usage documentation
- [OXC parser example on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/examples/parser.rs) — Complete working example

### Secondary (MEDIUM confidence)
- [OXC AST design docs](https://oxc.rs/docs/learn/parser_in_rust/ast) — Arena allocation, memory model
- [OXC Traverse documentation](https://docs.rs/oxc_traverse/latest/oxc_traverse/) — Enter/exit pattern
- [OXC Semantic Analysis](https://oxc.rs/docs/learn/parser_in_rust/semantic_analysis) — ScopeId, SymbolId
- [OXC benchmark: parser comparison](https://github.com/oxc-project/bench-javascript-parser-written-in-rust) — "3x faster than SWC" claim
- [SWC Architecture](https://github.com/swc-project/swc/blob/main/ARCHITECTURE.md) — SyntaxContext, resolver, hygiene

### Tertiary (LOW confidence)
- [Qwik Optimizer Rules](https://qwik.dev/docs/advanced/optimizer/) — User-facing documentation, not implementation details

---
*Research completed: 2026-02-10*
*Ready for roadmap: yes*
