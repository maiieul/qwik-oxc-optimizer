# Qwik Optimizer — SWC-to-OXC Port

## What This Is

A project to port the Qwik framework's code optimizer from SWC to OXC. The v1.0 milestone produced a complete behavioral specification of all 162 SWC optimizer snapshot tests. The v2.0 milestone mapped every transformation pattern to concrete OXC APIs with Rust code examples, built working proof-of-concept programs, and produced an architectural blueprint — so the v3.0 port can proceed with zero guesswork.

## Current Status

v1.0 and v2.0 milestones shipped. Ready for v3.0 (the actual port).

**v2.0 delivered:**
- 7 comprehensive API mapping documents (~16K lines, 99 Rust code blocks)
- 4 working Rust POCs (dollar detection, capture analysis, multi-module output, source maps)
- Architecture blueprint with crate module layout, public API, data flow, and Cargo.toml
- Master cross-reference for all 14 CONV types with dependency ordering and implementation roadmap

## Core Value

A complete, SWC-independent behavioral specification and OXC API mapping of every optimizer transformation so the OXC port can be built from spec and research, not from reverse-engineering SWC code.

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

### Active

(None — next milestone requirements defined via `/gsd:new-milestone`)

### Out of Scope

- Byte-for-byte SWC output matching — the OXC optimizer will produce semantically equivalent output, not identical bytes
- Modifying any existing SWC code — read-only against SWC codebase
- TypeScript plugin layer changes — the TS/Vite/Rollup plugins are untouched until port milestone
- Modifying the 162 spec files — specs are locked as the source of truth

## Context

Shipped v1.0 (162 spec files, 166K lines) and v2.0 (7 API mapping docs, 4 working POCs, architecture blueprint, ~16K lines).
Tech stack: Rust (oxc 0.113 — parser, traverse, semantic, codegen, sourcemap), Python (spec generation/audit scripts), Markdown.

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
| Traverse enter_*/exit_* for detection + mutation | Standard OXC pattern, avoids modifying program.body during iteration | ✓ Good — validated in POC-01/02 |
| exit_program for deferred import insertion | Collect imports during traversal, insert all at end | ✓ Good — avoids iterator invalidation |
| Separate types.rs as universal leaf module | Prevents circular dependencies in crate architecture | ✓ Good — clean module DAG |
| Props destructuring before capture analysis | Destructuring changes variable references that capture analysis reads | ✓ Good — correct ordering validated against specs |
| Shared allocator for POCs, separate for production | Shared is simpler; separate enables parallel codegen per segment | ✓ Good — both approaches validated |
| CONV-10 before CONV-09 execution order | isServer replacement creates dead branches that stripping can then eliminate | ✓ Good — matches SWC behavior |
| Two-option PURE annotation strategy | Option A (OXC built-in) preferred; Option B (manual comment) as fallback | — Pending (decide during v3.0) |
| 9-tier CONV implementation order | Dependency graph + frequency analysis determines build order for v3.0 | — Pending (validated during port) |

---
*Last updated: 2026-02-11 after v2.0 milestone*
