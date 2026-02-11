# Qwik Optimizer — SWC-to-OXC Port

## What This Is

A project to port the Qwik framework's code optimizer from SWC to OXC. The v1.0 milestone produced a complete behavioral specification of all 162 SWC optimizer snapshot tests. The v2.0 milestone mapped every transformation pattern to concrete OXC APIs with Rust code examples, built working proof-of-concept programs, and produced an architectural blueprint. The v3.0 milestone built the complete `qwik-optimizer-oxc` Rust crate implementing all 14 CONV transformation types, validated at 157/162 spec match (96.9%).

## Core Value

A working OXC-based Qwik optimizer crate that passes all 162 spec tests — built from behavioral specs and API mappings, not reverse-engineered from SWC.

## Requirements

### Validated

- ✓ Rust utility crate (oxc-ast-util) parses JS/TS/JSX/TSX and outputs ESTree JSON AST — v1.0
- ✓ 162 markdown spec files exist at `.planning/spec/<test_name>.md` — v1.0
- ✓ Each spec documents input code, output modules, test config, and diagnostics — v1.0
- ✓ Each spec catalogs every convention applied (14 CONV types, zero false negatives) — v1.0
- ✓ Each spec inventories every function call in output (qrl, componentQrl, _jsxSorted, etc.) — v1.0
- ✓ Each spec includes OXC ASTs for input and all output modules — v1.0
- ✓ Specs capture segment metadata (SegmentAnalysis JSON) for extracted segments — v1.0
- ✓ All 162 specs structurally consistent and human-readable — v1.0

- ✓ OXC API mapping for every transformation pattern in the 162 specs — v2.0 (7 mapping documents, 99 Rust code blocks)
- ✓ Deep research on complex patterns: capture analysis, cross-module code movement, source maps — v2.0 (validated with working POCs)
- ✓ Rust library survey for capabilities OXC doesn't provide directly — v2.0 (oxc 0.113 covers all needs)
- ✓ Architectural blueprint for the oxc-optimizer crate — v2.0 (module layout, public API, data flow, Cargo.toml)

- ✓ Crate skeleton with types, errors, module layout, Cargo.toml — v3.0 (16 modules, all blueprint dependencies)
- ✓ All 14 CONV transformation types implemented — v3.0 (dollar detection, QRL wrapping, capture analysis, props destructuring, segment extraction, JSX transforms, signal optimization, PURE annotations, const replacement, dead branch elimination, code stripping, sync$ serialization)
- ✓ Spec-based test harness (162 tests) — v3.0 (157/162 module count match, 250/250 metadata assertions)
- ✓ Source map generation — v3.0 (main and segment modules via OXC codegen)
- ✓ Public API compatibility — v3.0 (TransformModulesOptions, TransformOutput, SegmentAnalysis with serde camelCase)

### Active

## Current Milestone: v4.0 Code Quality Refactor

**Goal:** Refactor the `qwik-optimizer-oxc` crate for maintainability — extract modules, eliminate boilerplate, fix bugs, clean up code style — while maintaining or improving 157/162 spec compliance.

**Target improvements:**
- Extract JSX transform from transform.rs into its own module
- Rewrite const_replace.rs using OXC VisitMut to eliminate ~750 lines of manual AST walking
- Fix minify_expression_string bug (drops spaces between identifiers)
- Remove dead code and duplicate constants
- Strip unnecessary comments, add early returns, flatten nesting
- KNOWN_GLOBALS as HashSet for O(1) lookup
- Consolidate duplicate binding-name collection functions in collector.rs
- Opportunistic spec compliance improvements

### Out of Scope

- Byte-for-byte SWC output matching — the OXC optimizer will produce semantically equivalent output, not identical bytes
- Modifying any existing SWC code — read-only against SWC codebase
- TypeScript plugin layer changes — the TS/Vite/Rollup plugins are untouched until integration milestone
- Modifying the 162 spec files — specs are locked as the source of truth
- Pre-compiled QRL extraction — specs testing extraction of already-compiled `inlinedQrl()` calls are out of optimizer scope

## Context

Shipped v1.0 (162 spec files, 166K lines), v2.0 (7 API mapping docs, 4 working POCs, architecture blueprint, ~16K lines), and v3.0 (complete optimizer crate, 11,758 LOC Rust, 16 source files).
Tech stack: Rust (oxc 0.113 — parser, traverse, semantic, codegen, sourcemap), Python (spec generation/audit scripts), Markdown.

- The `qwik-optimizer-oxc` crate at `crates/qwik-optimizer-oxc/` implements all 14 CONV transformation types
- 165 tests pass (158 unit + 7 spec), 157/162 module count match with 250/250 metadata assertions
- 5 known deviations: 3 parser limitations (OXC stricter on invalid source), 2 pre-compiled QRL extraction (out of scope)
- Tech debt: 16 capture analysis deviations (JSX event handler scope tracking), 3 diagnostic deviations, 4 orphaned functions
- The existing SWC optimizer lives in `swc-optimizer/core/` as a read-only reference
- The public API (`TransformModulesOptions`, `TransformOutput`, `SegmentAnalysis`) matches the SWC optimizer's wire format

## Constraints

- **Parser:** OXC parser (installed via cargo) — this is the target parser for the new optimizer
- **Spec format:** Markdown files in `.planning/spec/` — one per snapshot test, human-readable
- **Brownfield:** Existing codebase map in `.planning/codebase/` documents current architecture

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Spec before code | Document all behaviors first so the port isn't a line-by-line translation of SWC patterns | ✓ Good — 162 specs provide clear behavioral contracts |
| oxc_parser from cargo | Same language (Rust) as the optimizer, will be reused directly in oxc-optimizer | ✓ Good — ESTree JSON output clean, parser handles all code variants |
| Both input and output ASTs | Input ASTs show what to parse; output ASTs show what to generate | ✓ Good — collapsible details blocks keep specs readable |
| Public API preserved | Drop-in replacement for downstream consumers (Vite/Rollup plugins, TypeScript layer) | ✓ Good — serde camelCase matches SWC wire format |
| Python for spec generation | Claude generates spec files via Python scripts for consistency and repeatability | ✓ Good — zero false negatives on convention detection |
| Collapsed 8 phases to 3 | Spec generation is documentation, not software engineering; each spec is independent | ✓ Good — simpler roadmap, faster execution |
| Regex convention detection | Pattern matching against all 14 CONV types rather than AST analysis | ✓ Good — 100% accuracy, fast execution |
| oxc ast_visit feature flag | serialize alone does not re-export Utf8ToUtf16; ast_visit feature required | ✓ Good — resolved OXC API quirk |
| Traverse enter_*/exit_* for detection + mutation | Standard OXC pattern, avoids modifying program.body during iteration | ✓ Good — validated in POC and production |
| exit_program for deferred import insertion | Collect imports during traversal, insert all at end | ✓ Good — avoids iterator invalidation |
| Separate types.rs as universal leaf module | Prevents circular dependencies in crate architecture | ✓ Good — clean module DAG |
| Props destructuring before capture analysis | Destructuring changes variable references that capture analysis reads | ✓ Good — correct ordering validated against specs |
| CONV-10 before CONV-09 execution order | isServer replacement creates dead branches that stripping can then eliminate | ✓ Good — matches SWC behavior |
| OXC built-in PURE annotations | expression_call_with_pure for tree-shakeable calls | ✓ Good — cleaner than manual comment injection |
| 9-tier CONV implementation order | Dependency graph + frequency analysis determines build order | ✓ Good — all 14 CONVs implemented in correct order |
| String-based segment code construction | code_move builds JS as string, normalize_code formats via parse+codegen | ✓ Good — simpler than AST construction, identity-like source maps |
| Two-pass collector (recursive walk) | Read-only analysis before mutable traverse; OXC Scoping consumed by traverse_mut | ✓ Good — clean separation of analysis and mutation |
| Stack-based capture analysis | capture_stack Vec for nested $()-body tracking, each frame independent | ✓ Good — handles arbitrary nesting depth |
| Hoisted functions as code strings | Store _hfN declarations as strings, inject after imports in codegen output | ✓ Good — avoids AST allocation during traverse |
| Span-based tracking (HashSet<u32>) | O(1) lookup for stripped segments and pending sync calls | ✓ Good — efficient and simple |
| const_replace as pre-pass | Runs before traverse_mut so segment body serialization sees replaced boolean literals | ✓ Good — correct ordering for dead branch elimination |
| Known deviation sets | 5 module count + 16 capture + 3 diagnostic deviations categorized with rationale | ✓ Good — transparent tracking of limitations |

---
*Last updated: 2026-02-11 after v4.0 milestone started*
