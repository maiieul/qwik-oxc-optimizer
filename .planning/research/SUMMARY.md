# Project Research Summary

**Project:** Qwik Optimizer -- OXC Port (v2.0 Implementation)
**Domain:** Rust AST transformer for JavaScript/TypeScript code splitting
**Researched:** 2026-02-10
**Confidence:** HIGH

## Executive Summary

The Qwik optimizer is a JavaScript/TypeScript code transformer that performs code splitting across dollar-sign (`$`) boundaries. It parses source files, extracts lazy-loadable segments marked with `$()`, `component$`, and similar patterns, performs capture analysis to determine which variables cross boundaries, and produces multiple output modules (one main module plus N segment modules) with appropriate QRL wrappers and imports. This is a specialized, performance-critical AST transformation pipeline that differs significantly from standard transpilers.

The recommended approach is to use OXC 0.113 as the foundation (replacing SWC) with a two-phase architecture: **Phase 1 (Analyze)** performs a single read-only traversal to build a transformation plan, and **Phase 2 (Emit)** executes that plan by mutating the main AST and constructing separate segment ASTs. This separation addresses OXC's constraint that semantic analysis must occur before AST mutation, and cleanly handles the multi-output requirement. OXC provides 3x faster parsing than SWC, better-maintained APIs, and comprehensive semantic analysis through `oxc_semantic`.

The critical risks are: (1) arena allocator lifetime management infecting every helper function with `'a` parameters, (2) semantic data invalidation if analysis happens after mutation, (3) building multiple output modules requires careful allocator strategy, and (4) capture analysis has 8+ edge cases that each produce runtime failures if wrong. These are addressed through phase separation, dedicated capture analysis module with comprehensive tests, and establishing coding conventions for lifetime-parameterized functions before implementation begins.

## Key Findings

### Recommended Stack

**Core framework: OXC umbrella crate (v0.113) with Rust edition 2024**

OXC is a modern JavaScript/TypeScript parser and transformer written in Rust, designed as a high-performance successor to SWC. For the Qwik optimizer port, use the `oxc` umbrella crate with feature flags: `semantic` (scope analysis, symbol table, reference resolution for capture analysis), `codegen` (AST-to-JavaScript generation with source maps), and `serialize` (JSON output for segment metadata).

**Core technologies:**
- **oxc (umbrella crate) v0.113 with features ["semantic", "codegen", "serialize"]:** Single dependency providing parser, semantic analyzer, AST traversal, code generator, and source maps. Avoids version-sync issues with 10+ individual sub-crates. All sub-crates versioned in lockstep.
- **oxc_traverse (via semantic feature):** Provides `Traverse` trait for AST mutation with parent context access. Critical for capture analysis where you need to know which scope a reference belongs to. Replaces SWC's `Fold` trait with a mutable-reference model.
- **oxc_semantic (via semantic feature):** Builds scope tree, symbol table, and reference tracking. Provides `Scoping::find_binding()`, `get_resolved_references()`, `scope_ancestors()` for capture analysis. This is the foundation for determining which variables cross `$` boundaries.
- **oxc_codegen (via codegen feature):** Generates JavaScript from transformed ASTs with source map support. Returns `CodegenReturn { code: String, map: Option<SourceMap>, legal_comments }`. Each output module (main + segments) needs independent codegen.
- **serde/serde_json:** JSON serialization for `SegmentAnalysis` metadata and `TransformModulesOptions` deserialization. Required by OXC's `serialize` feature.
- **rayon v1.10:** Parallel file transformation. Each input file gets its own allocator, so arena lifetimes don't conflict across threads. The optimizer processes hundreds of files per build.
- **rustc-hash v2.1 (FxHashMap):** Fast non-cryptographic hash maps for symbol tables and capture sets. Deterministic across platforms (important for reproducible builds). OXC uses this internally.
- **indexmap v2.7:** Insertion-ordered hash maps for deterministic segment ordering. Essential for reproducible builds and stable output.

**Critical version constraint:** OXC 0.113 requires Rust edition 2024 and MSRV 1.91.0. OXC releases weekly, so pin to `0.113` (allow patch versions only).

### Expected Features

**Must have (table stakes):**
- **AST traversal with parent context (oxc_traverse Traverse trait):** The optimizer must walk the entire AST to find `$()` calls, imports, and variable references while maintaining ancestor context for capture analysis. Without parent context, capture analysis is impossible.
- **AST mutation (replace, wrap, remove):** Replace `component$(fn)` with `componentQrl(qrl(...))`, replace JSX with `_jsxSorted()` calls, wrap expressions with QRL wrappers, remove type annotations.
- **AST node construction (AstBuilder):** Build entirely new nodes for `qrl()` calls, lazy import declarations, export declarations for segments, capture arrays, member expressions (`_captures[0]`). Hundreds of node construction calls throughout the optimizer.
- **Semantic analysis (oxc_semantic):** Capture analysis requires scope resolution, symbol table lookups, and reference tracking. Must identify which variables cross `$` boundaries by comparing declaration scopes against reference scopes.
- **Code generation with source maps (oxc_codegen):** Produce JavaScript strings from transformed ASTs with source maps mapping back to original source positions. Critical for debugging lazy-loaded segments.
- **Multi-module output (code splitting):** The optimizer's defining feature. One input file produces 1 main module + N segment modules. No OXC precedent; must invent the pattern.

**Should have (competitive):**
- **Single-pass traversal for all transforms:** OXC's transformer combines TS removal, JSX transform, and lowering in one traversal. The Qwik optimizer could similarly handle `$()` extraction, JSX transform, props destructuring, and signal analysis in one pass if ordering constraints allow.
- **js_to_oxc tool for template generation:** External tool converts JS source to Rust `AstBuilder` code. Dramatically reduces boilerplate for constructing complex output patterns like `qrl()` calls. Use this to generate construction code from templates.
- **BoundIdentifier for safe renaming (TraverseCtx::generate_uid):** When creating new identifiers (`_rawProps`, `_hf0`), use `ctx.generate_uid()` to avoid name collisions with existing bindings. Returns `BoundIdentifier` with correct `SymbolId`.

**Defer (v2+):**
- **TS/JSX transpilation via OXC transformer:** Use OXC's built-in TypeScript and JSX transformers as pre/post passes around the Qwik transform. When `transpile_ts: true`, run OXC's transformer before Qwik analysis to strip types.
- **Diagnostic emission (OxcDiagnostic):** Custom error/warning reporting for "QWIK(x): ..." error messages. Implement once core transforms are stable.

### Architecture Approach

**Two-phase pipeline with collector + emitter:**

The recommended architecture separates analysis from mutation. Phase 1 (Analyze) performs a single read-only `Traverse` pass to collect all `$`-boundary sites, imports, exports, variable bindings, capture relationships, and scope information, producing a `TransformPlan`. Phase 2 (Emit) executes the plan: mutates the main module AST in-place, constructs separate `Program` ASTs for each extracted segment, and runs codegen on all programs.

This decouples "what to do" from "doing it." SWC's `Fold` interleaves analysis and mutation because fold-by-ownership naturally returns new ASTs. OXC's `Traverse` gives mutable references in-place, making interleaved collect-and-mutate error-prone. Phase separation makes multi-output manageable: phase 1 identifies which code goes where, phase 2 builds the output ASTs.

**Major components:**

1. **parse.rs:** Parse source to `Program` via `oxc_parser`, run `SemanticBuilder` to produce `Semantic` with scope/symbol tables. Wraps OXC parsing and semantic analysis with error handling.

2. **analyze.rs (QwikAnalyzer):** Implements `Traverse` trait for single-pass analysis. Detects `$`-boundary call sites, collects imports/exports/root declarations, performs capture analysis per boundary, identifies JSX elements, records strip/filter decisions. Produces `TransformPlan`.

3. **capture.rs (CaptureAnalyzer):** Core algorithm for determining which variables cross `$` boundaries using `oxc_semantic` APIs. For each identifier reference inside a closure, resolves its `SymbolId`, checks if declared in ancestor scope outside the closure. Handles 8+ edge cases: imports (not captured), exports (not captured), loop variables (special handling), shadowing, destructured props, const vs let/var, hoisted functions, TypeScript type-only imports.

4. **emit.rs (PlanExecutor):** Executes `TransformPlan` against main AST. Mutates main module in-place (rewrite imports, replace `$`-calls with `qrl()`/`inlinedQrl()`, transform JSX, insert lazy imports, apply `#__PURE__` annotations, strip exports/code). Coordinates segment building and codegen.

5. **segment_builder.rs:** Constructs new `Program` ASTs for extracted segments from plan data. Builds import declarations, export wrappers, function bodies with capture restoration (`const x = _captures[0]`). Each segment is a standalone module.

6. **codegen_bridge.rs:** Wraps `oxc_codegen::Codegen` to produce code strings and source maps for each `Program`. Handles `CodegenOptions` configuration (source_map_path, source text). Returns `TransformModule` with code, map, metadata.

7. **jsx.rs:** JSX-specific transformation logic. Converts JSX elements to `_jsxSorted()`/`_jsxSplit()` calls, applies signal wrapping, handles props optimization. Both analysis (detection) and emission (rewriting).

8. **entry_strategy.rs:** Entry strategy logic (segment, inline, smart, hook, component). Determines how segments are grouped and loaded based on configuration.

### Critical Pitfalls

**Top 5 pitfalls that cause rewrites or runtime failures:**

1. **Arena lifetime infection -- every helper function needs `'a`:** OXC's arena allocator ties all AST nodes to lifetime `'a`. Helper functions that create/return AST nodes must carry this lifetime parameter, reshaping function signatures throughout the codebase. Prevention: Design helper architecture around arena constraints from day one. Separate pure analysis functions (no `'a`) from AST construction functions (require `&AstBuilder<'a>`).

2. **Semantic info invalidation after AST mutation:** `SemanticBuilder` produces scope/symbol data from the original AST. After mutation, all `ScopeId`/`SymbolId` values are stale. Prevention: Run all analysis before any mutation (two-phase architecture). Use `ctx.generate_binding()` and `ctx.create_bound_reference()` for new nodes.

3. **Building new modules requires careful allocator strategy:** Each output `Program<'a>` is tied to an `Allocator`. Cannot build output module AST nodes during input traversal (double-borrow conflict). Prevention: Collect data during traversal (spans, variable names, metadata as plain Rust types). After traversal, build each segment `Program` in a fresh allocator or shared allocator.

4. **Capture analysis scope boundary edge cases:** 8+ distinct cases: imports NOT captured, exports NOT captured, loop variables ARE captured with special handling, shadowed variables use inner binding, destructured props need transformation, const vs let/var affects flags, hoisted functions conditionally captured, TypeScript type-only imports NOT captured. Prevention: Build capture analysis as standalone, heavily-tested module. Test each edge case independently before implementing rest of optimizer.

5. **Source maps for split modules must point to original positions:** When extracting code from line 15 of input and placing at line 1 of segment output, source map must map output line 1 to input line 15. Prevention: Pass original input source text to `Codegen::with_source_text()` for ALL modules (main and segments). Use original spans from moved nodes; use `SPAN` (zero-length) for synthetic constructs.

## Implications for Roadmap

Based on research, the port has clear architectural constraints that dictate phase ordering. OXC's arena allocator and semantic analysis model require a fundamentally different approach than SWC's ownership model. The two-phase (analyze-then-emit) pattern is not optional -- it's required by OXC's safety guarantees.

### Phase 1: Foundation + Parse + Semantic
**Rationale:** Establish project structure, types, and the parsing pipeline before any transformation logic. OXC requires semantic analysis before traversal, making this a hard dependency for all later phases.

**Delivers:**
- Crate structure (`lib.rs`, `types.rs`, `parse.rs`, helper modules)
- Public API types (`TransformModulesOptions`, `TransformOutput`, `TransformModule`, `SegmentAnalysis`)
- Parse wrapper (source -> `Program` -> `Semantic`)
- Test harness that loads 162 spec files

**Stack elements:** `oxc_parser`, `oxc_semantic::SemanticBuilder`, `serde`/`serde_json`, test dependencies (`insta`)

**Avoids:** Pitfall #1 (lifetime infection) by establishing helper function conventions early. Pitfall #3 (semantic invalidation) by making semantic analysis a foundational API.

**Research flag:** No additional research needed. Standard project setup with well-documented APIs.

---

### Phase 2: Capture Analysis (Isolated Module)
**Rationale:** Capture analysis is the most complex and error-prone algorithm in the optimizer. It has 8+ edge cases, each causing runtime failures if wrong. Build and test this independently before any other transform logic.

**Delivers:**
- `capture.rs` module implementing capture analysis algorithm
- Uses `oxc_semantic` APIs: `Scoping::find_binding()`, `get_resolved_references()`, `scope_ancestors()`, `symbol_declaration()`
- Unit tests for all 8 edge cases (imports, exports, loops, shadowing, destructuring, const/let/var, hoisted functions, TypeScript types)
- Integration tests against spec files: `example_multi_capture.md`, `example_capture_imports.md`, `example_functional_component_capture_props.md`, `example_component_with_event_listeners_inside_loop.md`

**Addresses:** Pitfall #4 (capture edge cases) by comprehensive testing before integration.

**Research flag:** Needs research. Capture analysis for loop variables and destructured props has subtle interactions with scope rules. May need deep dive into OXC's scope tree API during implementation.

---

### Phase 3: Analysis Pass (Traversal + Detection)
**Rationale:** With capture analysis complete, build the read-only traversal pass that detects all `$`-boundary sites, collects imports/exports, and produces a `TransformPlan`. This is the "what to do" phase.

**Delivers:**
- `analyze.rs` implementing `Traverse` trait with `enter_*`/`exit_*` methods
- `collector.rs` for import/export/declaration collection
- `TransformPlan` data structure (internal type, not public API)
- `SegmentPlan` records for each extracted segment
- Detection logic for `$()`, `component$`, `useStyles$`, etc.

**Stack elements:** `oxc_traverse` (Traverse trait, TraverseCtx), `rustc-hash` (FxHashMap), `indexmap` (deterministic ordering)

**Avoids:** Pitfall #2 (semantic invalidation) by keeping this phase read-only. No AST mutations yet.

**Research flag:** No additional research needed. `Traverse` pattern is well-documented.

---

### Phase 4: Main Module Mutation (Emit Core)
**Rationale:** Execute the transformation plan against the main module AST. Replace `$`-calls with `qrl()` wrappers, rewrite imports, insert lazy import declarations, apply `#__PURE__` annotations.

**Delivers:**
- `emit.rs` implementing `PlanExecutor` that mutates main AST based on `TransformPlan`
- Import rewriting (remove `component$` import, add `componentQrl` + `qrl` imports)
- Expression replacement (`component$(fn)` -> `componentQrl(qrl(lazy_import, name))`)
- Lazy import declaration insertion (`const i_HASH = () => import("./segment")`)
- `#__PURE__` annotation attachment via `program.comments`

**Addresses:** Pitfall #7 (statement insertion) by using deferred insertion pattern (`exit_program` callback).

**Avoids:** Pitfall #2 (move_expression garbage) by using `std::mem::replace` for atomic swap.

**Research flag:** No additional research needed. Standard mutation patterns.

---

### Phase 5: Segment Module Construction
**Rationale:** Build separate `Program` ASTs for each extracted segment from `SegmentPlan` data. This is the multi-output capability that defines the optimizer.

**Delivers:**
- `segment_builder.rs` constructing fresh `Program` per segment via `AstBuilder`
- Import hoisting (re-import what segment needs)
- Export wrapping (`export const SegmentName = (params) => { ... }`)
- Capture restoration (`const x = _captures[0]`)
- One `Program` per segment, ready for codegen

**Stack elements:** `oxc_ast::AstBuilder`, allocator per segment strategy

**Addresses:** Pitfall #3 (multi-allocator) by using post-traversal segment building with fresh allocators.

**Research flag:** Needs research. Multi-output pattern has no OXC precedent. May need experimentation with allocator sharing vs separate allocators per segment.

---

### Phase 6: Codegen + Source Maps
**Rationale:** Serialize all `Program` ASTs (main + segments) to JavaScript strings with source maps. This is the final output stage.

**Delivers:**
- `codegen_bridge.rs` wrapping `oxc_codegen::Codegen`
- Source map generation via `CodegenOptions { source_map_path }`
- `TransformModule` assembly with code, map, metadata
- Complete `TransformOutput` with all modules and diagnostics

**Stack elements:** `oxc_codegen::Codegen`, `oxc_sourcemap::SourceMap`, `base64` for source map encoding

**Addresses:** Pitfall #5 (source map positions) by passing original source text to `Codegen::with_source_text()`.

**Avoids:** Pitfall #8 (pure comments) by explicitly adding `#__PURE__` comments to `program.comments` for new nodes.

**Research flag:** No additional research needed. `Codegen` API is well-documented.

---

### Phase 7: JSX Transformation
**Rationale:** Transform JSX elements to `_jsxSorted()`/`_jsxSplit()` calls. This is a self-contained feature that can be added once core transform is working.

**Delivers:**
- `jsx.rs` implementing JSX detection (in analyze phase) and rewriting (in emit phase)
- `_jsxSorted()` call construction for static JSX
- `_jsxSplit()` for dynamic JSX with signal props
- Signal wrapping for reactive props

**Addresses:** Features from spec files: `example_jsx.md`, `example_jsx_props.md`

**Research flag:** Needs research. JSX transformation has complex interaction with props destructuring and signal analysis. May need deep dive during implementation.

---

### Phase 8: Advanced Features (Props Destructuring, Signals, Entry Strategy)
**Rationale:** Optimization features that enhance output quality but are not required for basic functionality.

**Delivers:**
- Props destructuring optimization (rewrite parameters to `_rawProps` access)
- Derived signal optimization (`_wrapProp`, `_fnSignal`)
- Entry strategy implementation (segment, inline, smart, hook, component)
- Code stripping (`strip_exports`, `strip_ctx_name`, `strip_event_handlers`)
- Const folding (`if (false)` removal, `isServer`/`isBrowser`/`isDev` replacement)

**Addresses:** Competitive features that improve bundle size and runtime performance.

**Research flag:** Entry strategy and signal optimization are complex. Needs research during implementation.

---

### Phase Ordering Rationale

**Dependency-driven:**
- Phase 1 (parse/semantic) is a hard prerequisite for all phases. OXC requires `SemanticBuilder` output before `Traverse`.
- Phase 2 (capture analysis) must be independent and tested before Phase 3 uses it.
- Phase 3 (analysis) produces data consumed by Phase 4-5 (emit/build segments).
- Phase 6 (codegen) depends on Phases 4-5 producing complete ASTs.
- Phase 7-8 (JSX, advanced) are feature additions layered on top of working core.

**Risk mitigation:**
- Building capture analysis first (Phase 2) isolates the highest-risk algorithm for independent testing.
- Splitting analysis (Phase 3) from emit (Phase 4-5) prevents semantic invalidation bugs.
- Deferring JSX and advanced features (Phases 7-8) allows core functionality to stabilize first.

**OXC constraints:**
- Two-phase (analyze-then-emit) is required by OXC's semantic analysis model.
- Multi-allocator strategy (Phase 5) must be resolved before segment building.
- Source map handling (Phase 6) requires understanding of span preservation during AST construction.

### Research Flags

**Phases needing deeper research during planning:**
- **Phase 2 (Capture Analysis):** Loop variable capture and destructured props have subtle scope interactions. OXC's scope tree API needs exploration.
- **Phase 5 (Segment Building):** Multi-output pattern has no precedent in OXC ecosystem. Need to experiment with allocator sharing strategies.
- **Phase 7 (JSX Transformation):** Interaction between JSX rewriting, props destructuring, and signal wrapping is complex. Needs design work.
- **Phase 8 (Advanced Features):** Entry strategy grouping logic and signal optimization have sparse documentation. May need SWC source code study.

**Phases with standard patterns (skip research-phase):**
- **Phase 1 (Foundation):** Standard Rust project setup with well-documented OXC APIs.
- **Phase 3 (Analysis Pass):** `Traverse` trait pattern is standard and well-documented.
- **Phase 4 (Main Module Mutation):** Expression replacement and import rewriting follow established patterns.
- **Phase 6 (Codegen):** `Codegen` API is straightforward with clear examples.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | OXC 0.113 APIs verified via docs.rs, crates.io, GitHub source. Feature flags confirmed. Version compatibility validated. All dependencies have stable APIs. |
| Features | MEDIUM-HIGH | OXC APIs for table-stakes features (Traverse, semantic, codegen) are confirmed. Multi-output pattern is novel but implementable. Some advanced features (signal optimization, entry strategy) lack detailed OXC examples. |
| Architecture | MEDIUM-HIGH | Two-phase (analyze-then-emit) pattern is sound and addresses OXC constraints. Multi-allocator strategy for segment building needs implementation validation but is architecturally viable. Component boundaries are clear. |
| Pitfalls | HIGH | All critical pitfalls verified via OXC source code, GitHub issues, and official documentation. Edge cases (lifetime infection, semantic invalidation, capture analysis) have clear prevention strategies. Phase-specific warnings map directly to implementation stages. |

**Overall confidence:** MEDIUM-HIGH

The recommended stack (OXC 0.113) and core patterns (two-phase pipeline, capture analysis algorithm, multi-output strategy) are well-founded and verified. The primary uncertainty is in implementation details for novel patterns (multi-output module generation, allocator management) where no OXC precedent exists. These are addressable through experimentation during Phases 5-6.

### Gaps to Address

**During architecture design (before Phase 1):**
- **Allocator sharing strategy:** Decide whether to use one shared allocator for all output modules or separate allocators per segment. Trade-offs: shared = simpler lifetime management, separate = cleaner memory isolation. Validate with small prototype.
- **Helper function conventions:** Establish coding standard for lifetime-parameterized functions (`<'a>`) vs lifetime-free functions. Prevents architectural drift and lifetime infection spreading uncontrolled.

**During Phase 2 (Capture Analysis):**
- **Loop variable capture semantics:** OXC's scope tree representation of `for-i`, `for-in`, `for-of` loop scopes needs investigation. Spec file `example_component_with_event_listeners_inside_loop.md` shows expected behavior, but OXC API mapping unclear.
- **TypeScript type-only import detection:** Verify how `oxc_semantic` flags type-only imports (`import type { X }`) vs value imports. Critical for capture analysis to avoid capturing type-only symbols.

**During Phase 5 (Segment Building):**
- **Span preservation for source maps:** When building segment ASTs from scratch, determine which nodes use original spans (extracted code) vs `SPAN` (synthetic wrappers). Validate that source maps map correctly with mixed span sources.

**During Phase 7 (JSX Transformation):**
- **JSX props optimization interaction:** Clarify how props destructuring optimization affects JSX transformation. Spec files show examples, but the order of transforms (JSX first or props first) affects output.

## Sources

### Primary (HIGH confidence)
- [oxc crate v0.113.0 on crates.io](https://crates.io/crates/oxc) -- Umbrella crate, feature flags, version compatibility
- [oxc_traverse docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/) -- Traverse trait, TraverseCtx, Ancestor system
- [oxc_semantic docs.rs](https://docs.rs/oxc_semantic/latest/oxc_semantic/) -- SemanticBuilder, Scoping API, symbol/scope/reference tables
- [oxc_codegen docs.rs](https://docs.rs/oxc/latest/oxc/codegen/) -- Codegen API, CodegenReturn, source map generation
- [oxc_ast AstBuilder docs.rs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) -- Node construction methods
- [oxc_sourcemap docs.rs](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/) -- SourceMap, ConcatSourceMapBuilder
- [OXC GitHub repository](https://github.com/oxc-project/oxc) -- Source code for transformer examples (jsx_impl.rs, annotations.rs)
- [OXC Cargo.toml](https://github.com/oxc-project/oxc/blob/main/Cargo.toml) -- MSRV 1.91.0, edition 2024
- 162 spec files in `.planning/spec/` -- Behavioral ground truth for all transformations

### Secondary (MEDIUM confidence)
- [GitHub issue #5359: Statement replacement patterns](https://github.com/oxc-project/oxc/issues/5359) -- `move_expression`, NullExpression, arena allocation overhead
- [GitHub issue #4767: Statement insertion during traversal](https://github.com/oxc-project/oxc/issues/4767) -- Deferred insertion pattern, exit_statements
- [GitHub discussion #2704: Transformer bottom-up implementation](https://github.com/oxc-project/oxc/discussions/2704) -- Semantic synchronization challenges
- [OXC Transformer Alpha announcement](https://oxc.rs/blog/2024-09-29-transformer-alpha) -- Performance characteristics, architecture overview
- [js_to_oxc tool](https://github.com/KermanX/js_to_oxc) -- JS-to-AstBuilder code generation utility

### Tertiary (LOW confidence)
- Multi-output module generation pattern -- No OXC precedent; pattern must be invented and validated

---

*Research completed: 2026-02-10*
*Ready for roadmap: yes*
