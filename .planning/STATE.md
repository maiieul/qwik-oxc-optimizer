# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-11)

**Core value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests
**Current focus:** Phase 24 - Runtime Bug Fixes (v6.0)

## Current Position

Phase: 24 of 26 (Runtime Bug Fixes)
Plan: 7 of 9 in current phase
Status: Executing
Last activity: 2026-02-12 -- Plan 07 complete (declaration collection edge cases fixed)

Progress: [########################......] 24/26 phases (v1.0-v5.0 shipped, v6.0 Phase 24 complete)

## Performance Metrics

**Velocity (v1.0):**
- Total plans completed: 9
- Average duration: 9min
- Total execution time: 1.42 hours

**Velocity (v2.0):**
- Total plans completed: 8
- Average duration: 8min
- Total execution time: ~1.1 hours

**Velocity (v3.0):**
- Total plans completed: 16
- Average duration: 12min
- Total execution time: ~201min

**Velocity (v4.0):**
- Total plans completed: 8
- Average duration: 5min
- Total execution time: ~39min

**Velocity (v5.0):**
- Total plans completed: 3
- Average duration: 7min
- Total execution time: ~21min

**Velocity (v6.0):**

| Phase | Plan | Duration | Tasks | Files |
|-------|------|----------|-------|-------|
| 23    | 01   | 2min     | 1     | 2     |
| 23    | 02   | 3min     | 2     | 1     |
| 24    | 01   | 13min    | 2     | 6     |
| 24    | 02   | 7min     | 2     | 82    |
| 24    | 03   | 20min    | 2     | 52    |
| 24    | 04   | 8min     | 2     | 2     |
| 24    | 05   | 6min     | 2     | 22    |
| 24    | 06   | 4min     | 2     | 2     |
| 24    | 07   | 6min     | 2     | 7     |

## Accumulated Context

### Decisions

Full decision log in PROJECT.md Key Decisions table.
Recent: "Semantic verification before NAPI" and "Fix only runtime-breaking deviations" (approved).
- 23-01: Hash stripping uses last-underscore heuristic for module path matching
- 23-01: Three-tier module matching: exact path, structural (hash-stripped), metadata fallback
- 23-02: 293 runtime-breaking deviations across 140 specs require fixes in Phase 24
- 23-02: 206 cosmetic deviations can be deferred indefinitely
- 23-02: 5 high-priority fix patterns: module generation failures, QRL extraction, code generation, import resolution, capture analysis
- 24-01: Parse-roundtrip approach for JSX lambda serialization (extract source by span, re-parse, codegen)
- 24-01: Store source_code on QwikTransform for span-based extraction during traversal
- 24-01: Fixed spec inputs rather than making optimizer tolerant of invalid syntax
- 24-02: ImportKind enum to distinguish default/namespace/named import specifiers
- 24-02: ReemittedImport struct with kind and alias tracking for capture-to-segment import propagation
- 24-02: Store needed_imports on all segments (top-level and nested)
- 24-03: Merge all capture_stack frames for JSX handler filtering (not just last frame) to support nested $() scopes
- 24-03: Skip capture filtering when capture_stack is empty (bare function handlers like export default)
- 24-03: Pre-transform JSX attribute value replacement before JSX transform runs (avoid invasive signature changes)
- 24-03: Add known_ctxkind_deviations for example_immutable_analysis jSXProp vs eventHandler classification
- 24-04: 56 runtime-breaking deviations remain (52 missing-import-used + 4 truly-missing-module) after Phase 24
- 24-04: 63 naming-convention module path pairs are cosmetic (not truly-missing modules)
- 24-04: Symbol-in-body verification distinguishes runtime-breaking from cosmetic import deviations
- 24-05: Module-level declarations reclassified as needed_imports (self-imports) rather than captures, matching SWC behavior
- 24-05: SWC generates import { X } from './module' for module-level decl references, not _captures[] serialization
- 24-06: Runtime-breaking threshold set to 10 (6 missing-import-used edge cases + 4 truly-missing-module)
- 24-06: Regression gate in output_audit.rs asserts runtime_breaking_count <= RUNTIME_BREAKING_THRESHOLD
- 24-07: Exhaustive BindingPattern matching (all 4 variants) across collector.rs and transform.rs
- 24-07: TSEnumDeclaration tracked as module-level declaration for self-import generation
- 24-07: Named default export functions/classes added to module_level_decls

### Pending Todos

None.

### Blockers/Concerns

- 2 missing-import-used edge cases remain (JSX import source for example_jsx_import_source)
- 4 truly-missing-module deviations remain (pre-compiled QRL, multi-file)
- Module count mismatches at 2 (example_qwik_react, relative_paths require pre-compiled QRL reverse-engineering)

## Session Continuity

Last session: 2026-02-12
Stopped at: Completed 24-07-PLAN.md
Resume file: None
