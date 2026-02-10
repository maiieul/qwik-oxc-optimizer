# Qwik Optimizer — SWC-to-OXC Port

## What This Is

A project to port the Qwik framework's code optimizer from SWC to OXC. The first milestone creates a complete specification of every SWC optimizer transformation by documenting all 162+ snapshot tests with their conventions, function calls, and OXC-parsed ASTs. This spec becomes the single source of truth for building the OXC optimizer without ever referencing SWC internals.

## Core Value

A complete, SWC-independent behavioral specification of every optimizer transformation so the OXC port can be built from spec, not from reverse-engineering SWC code.

## Requirements

### Validated

<!-- Shipped and confirmed valuable. -->

(None yet — ship to validate)

### Active

- [ ] Install oxc_parser from cargo as a Rust dependency
- [ ] Create `.planning/spec/` directory with one markdown file per SWC snapshot test (~162 files)
- [ ] Each spec file documents the input source code
- [ ] Each spec file documents the transformed output code (all output modules)
- [ ] Each spec file catalogs every convention applied (hoisted functions, code movement, segment extraction, lazy import generation, etc.)
- [ ] Each spec file inventories every function call in the output (`qrl`, `componentQrl`, `_jsxSorted`, `_jsxSplit`, `_getVarProps`, `_getConstProps`, `_captures`, `_hf`, `qrlDEV`, etc.)
- [ ] Each spec file includes the oxc_parser AST of the input code
- [ ] Each spec file includes the oxc_parser AST of each output module
- [ ] Spec files capture the segment metadata (SegmentAnalysis JSON) for each extracted segment
- [ ] Spec files document test configuration (entry strategy, emit mode, transpile options, etc.)

### Out of Scope

- Byte-for-byte SWC output matching — the OXC optimizer will produce semantically equivalent output, not identical bytes
- Building the OXC optimizer itself — that's a future milestone
- Modifying any existing SWC code — this milestone is read-only against the SWC codebase
- TypeScript plugin layer changes — the TS/Vite/Rollup plugins are untouched in this milestone

## Context

- The existing optimizer lives in `swc-optimizer/core/` as a Rust crate using SWC (`swc_ecmascript`, `swc_common`, `swc_atoms`)
- 163 test functions in `swc-optimizer/core/src/test.rs` produce 162 snapshot files in `swc-optimizer/core/src/snapshots/`
- Each snapshot captures: input code, transformed output modules (with source maps), segment metadata (JSON), and diagnostics
- The optimizer's key transformations: `$()` extraction into lazy-loadable segments, `component$` → `componentQrl` conversion, JSX transformation to `_jsxSorted`/`_jsxSplit` calls, capture analysis, code movement across module boundaries
- The public API (`TransformModulesOptions`, `TransformOutput`, `SegmentAnalysis`) will remain the same in the OXC version
- The internal implementation will be refactored for clarity — not a line-by-line port

## Constraints

- **Parser:** OXC parser (installed via cargo) — this is the target parser for the new optimizer
- **Spec format:** Markdown files in `.planning/spec/` — one per snapshot test, human-readable
- **Read-only:** No modifications to existing SWC optimizer code in this milestone
- **Brownfield:** Existing codebase map in `.planning/codebase/` documents current architecture

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Spec before code | Document all behaviors first so the port isn't a line-by-line translation of SWC patterns | — Pending |
| oxc_parser from cargo | Same language (Rust) as the optimizer, will be reused directly in oxc-optimizer | — Pending |
| Both input and output ASTs | Input ASTs show what to parse; output ASTs show what to generate | — Pending |
| Public API preserved | Drop-in replacement for downstream consumers (Vite/Rollup plugins, TypeScript layer) | — Pending |

---
*Last updated: 2026-02-10 after initialization*
