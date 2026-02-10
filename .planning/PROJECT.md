# Qwik Optimizer — SWC-to-OXC Port

## What This Is

A project to port the Qwik framework's code optimizer from SWC to OXC. The v1.0 milestone produced a complete behavioral specification of all 162 SWC optimizer snapshot tests — documenting every transformation, convention, function call, and OXC-parsed AST. This spec is the single source of truth for building the OXC optimizer without ever referencing SWC internals.

## Current Milestone: v2.0 OXC API Research & Architecture

**Goal:** Deeply research OXC APIs and Rust libraries to map every spec transformation pattern to concrete implementation approaches, so the port milestone can proceed with zero guesswork.

**Target features:**
- Comprehensive OXC API mapping for all transformation categories
- Deep-dive research on complex patterns (capture analysis, cross-module code movement, source maps)
- Rust library survey for gaps OXC doesn't cover directly
- Architectural blueprint for the optimizer crate

## Core Value

A complete, SWC-independent behavioral specification of every optimizer transformation so the OXC port can be built from spec, not from reverse-engineering SWC code.

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

### Active

- [ ] OXC API mapping for every transformation pattern in the 162 specs
- [ ] Deep research on complex patterns: capture analysis, cross-module code movement, source maps
- [ ] Rust library survey for capabilities OXC doesn't provide directly
- [ ] Architectural blueprint for the oxc-optimizer crate

### Out of Scope

- Byte-for-byte SWC output matching — the OXC optimizer will produce semantically equivalent output, not identical bytes
- Modifying any existing SWC code — read-only against SWC codebase
- TypeScript plugin layer changes — the TS/Vite/Rollup plugins are untouched until port milestone
- Writing actual optimizer code — this milestone is research only, implementation comes in v3.0
- Modifying the 162 spec files — specs are locked as the source of truth

## Context

Shipped v1.0 with 162 spec files, 166K lines of documentation, and a Rust AST utility.
Tech stack: Rust (oxc_parser 0.113), Python (spec generation/audit scripts), Markdown.

- The existing optimizer lives in `swc-optimizer/core/` as a Rust crate using SWC (`swc_ecmascript`, `swc_common`, `swc_atoms`)
- 163 test functions in `swc-optimizer/core/src/test.rs` produce 162 snapshot files in `swc-optimizer/core/src/snapshots/`
- Each snapshot captures: input code, transformed output modules (with source maps), segment metadata (JSON), and diagnostics
- The optimizer's key transformations: `$()` extraction into lazy-loadable segments, `component$` → `componentQrl` conversion, JSX transformation to `_jsxSorted`/`_jsxSplit` calls, capture analysis, code movement across module boundaries
- The public API (`TransformModulesOptions`, `TransformOutput`, `SegmentAnalysis`) will remain the same in the OXC version
- Python generation scripts (generate_specs.py, gen-spec.py) automate spec creation with convention detection for all 14 types
- Audit scripts (audit-specs.py) verify structural consistency and convention completeness

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
| Public API preserved | Drop-in replacement for downstream consumers (Vite/Rollup plugins, TypeScript layer) | — Pending (validated in port milestone) |
| Python for spec generation | Claude generates spec files via Python scripts for consistency and repeatability | ✓ Good — zero false negatives on convention detection |
| Collapsed 8 phases to 3 | Spec generation is documentation, not software engineering; each spec is independent | ✓ Good — simpler roadmap, faster execution |
| Regex convention detection | Pattern matching against all 14 CONV types rather than AST analysis | ✓ Good — 100% accuracy, fast execution |
| oxc ast_visit feature flag | serialize alone does not re-export Utf8ToUtf16; ast_visit feature required | ✓ Good — resolved OXC API quirk |

---
*Last updated: 2026-02-10 after v2.0 milestone start*
