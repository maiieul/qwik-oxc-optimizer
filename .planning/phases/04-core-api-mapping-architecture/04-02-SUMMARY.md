---
phase: 04-core-api-mapping-architecture
plan: 02
subsystem: architecture
tags: [rust, oxc, cargo, serde, insta, crate-design, module-layout, public-api]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture (plan 01)
    provides: OXC API mapping guide for foundational transformation patterns
  - phase: v1.0 (phases 1-3)
    provides: 162 spec files as behavioral ground truth
provides:
  - Complete crate module layout with 16 modules and dependency ordering
  - Public API type definitions (TransformModulesOptions, TransformOutput, SegmentAnalysis, etc.)
  - 8-stage data flow pipeline specification with type signatures at every transition
  - Test harness design for 162 spec files using insta snapshot testing
  - Copy-pasteable Cargo.toml with feature flags
affects: [phase-05-deep-research-poc, phase-06-secondary-patterns, v3.0-implementation]

# Tech tracking
tech-stack:
  added: [oxc 0.113, serde 1, serde_json 1, anyhow 1, base64 0.22, rayon 1, pathdiff 0.2, path-slash 0.2, insta 1]
  patterns: [arena-allocator-scoped-codegen, traverse-trait-mutation, exit-program-import-rewriting, parallel-rayon-per-module]

key-files:
  created:
    - .planning/phases/04-core-api-mapping-architecture/ARCHITECTURE-BLUEPRINT.md
  modified: []

key-decisions:
  - "Exclude OXC transformer feature flag to avoid babel-compat dependencies; implement import management directly in exit_program hook"
  - "Use optional feature flags for rayon (parallel) and base64 (source-maps) to support WASM targets"
  - "Types in separate types.rs module as universal leaf to prevent circular dependencies"
  - "Progressive 6-tier test ordering from basic $() extraction to edge cases"

patterns-established:
  - "Module dependency DAG: types.rs as universal leaf, lib.rs as universal root"
  - "Wire compatibility: #[serde(rename_all = camelCase)] on all public types for TypeScript binding"
  - "Spec-based testing: parse markdown spec files -> build options -> transform -> snapshot compare"

# Metrics
duration: 6min
completed: 2026-02-10
---

# Phase 4 Plan 2: Architecture Blueprint Summary

**Complete architectural blueprint for qwik-optimizer-oxc crate with 16-module layout, full Rust type definitions with serde annotations, 8-stage data flow pipeline, spec-based test harness, and Cargo.toml**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-10T22:43:11Z
- **Completed:** 2026-02-10T22:49:38Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Documented all 16 crate modules with purpose, public exports, dependencies, and complexity estimates
- Created ASCII dependency graph proving no circular dependencies and a 5-tier build order
- Defined 10 public types and 6 internal types with complete Rust struct/enum definitions including `#[derive]`, `#[serde]` attributes, and field-level documentation
- Specified the complete 8-stage transformation pipeline (parse -> semantic -> collect -> transform -> filter -> extract -> emit -> assemble) with Rust type signatures at every transition
- Designed test harness to parse 162 markdown spec files and compare output using insta snapshots
- Provided copy-pasteable Cargo.toml with justified dependency choices and optional feature flags

## Task Commits

Each task was committed atomically:

1. **Task 1: Create Crate Module Layout and Dependency Ordering (ARCH-01)** - `846b5c6` (feat)
2. **Task 2: Document Public API, Data Flow, Test Harness, and Cargo.toml (ARCH-02 through ARCH-05)** - `f14a511` (feat)

## Files Created/Modified

- `.planning/phases/04-core-api-mapping-architecture/ARCHITECTURE-BLUEPRINT.md` - Complete architectural blueprint (ARCH-01 through ARCH-05) covering module layout, public API types, data flow specification, test harness design, and Cargo.toml

## Decisions Made

1. **Exclude `transformer` OXC feature flag** -- The `transformer` feature pulls in babel-compat layers. Import management is simpler to implement directly in `import_rewrite.rs` and the `exit_program` Traverse hook. This can be revisited if JSX transpilation needs arise.

2. **Optional feature flags for rayon and base64** -- Making `parallel` and `source-maps` optional features (both default-on) supports WASM compilation targets where threads and base64 encoding may not be needed.

3. **Separate `types.rs` as leaf module** -- All type definitions in one module with no dependencies prevents circular imports. Every other module can import from `types.rs` without pulling in logic.

4. **Progressive 6-tier test ordering** -- Start with the 3 simplest spec files (basic $() extraction), build through component$ patterns, captures, inline strategy, JSX, then tackle the remaining 152 edge cases. This validates the pipeline incrementally.

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Architecture blueprint complete and committed. Phase 4 (both plans) is finished.
- Phase 5 (Deep Research & Proof of Concept) can proceed with the architectural context from this blueprint.
- The blueprint's module layout, public API types, and data flow specification provide the structural foundation for building POCs.
- Key open question for Phase 5: the exact hash algorithm from the SWC optimizer must be ported to produce spec-compatible 11-character hashes.

## Self-Check: PASSED

- FOUND: ARCHITECTURE-BLUEPRINT.md
- FOUND: 04-02-SUMMARY.md
- FOUND: commit 846b5c6 (Task 1)
- FOUND: commit f14a511 (Task 2)

---
*Phase: 04-core-api-mapping-architecture*
*Completed: 2026-02-10*
