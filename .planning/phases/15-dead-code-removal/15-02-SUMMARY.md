---
phase: 15-dead-code-removal
plan: 02
subsystem: code-quality
tags: [rust, dead-code, compiler-warnings, refactor, allow-dead-code]

# Dependency graph
requires:
  - phase: 15-dead-code-removal
    plan: 01
    provides: Dead functions/constants/structs removed; only struct field warnings remain
provides:
  - Zero compiler warnings for qwik-optimizer-oxc crate
  - Consolidated binding-name collection in collector.rs (DEAD-03)
  - Targeted #[allow(dead_code)] annotations on 7 data-model structs
affects: [future phases, all qwik-optimizer-oxc development]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Targeted #[allow(dead_code)] on structs with test-only fields instead of file-level suppression"
    - "Thin delegation wrappers using OXC's Statement::as_declaration() to avoid code duplication"

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/types.rs

key-decisions:
  - "Used Statement::as_declaration() for consolidation -- cleanest delegation approach, avoids cloning"
  - "Applied #[allow(dead_code)] at struct level (not field level) to suppress all field warnings per struct"

patterns-established:
  - "Struct-level dead_code annotation: data-model structs populated by collector/transform but read only in tests get #[allow(dead_code)]"

# Metrics
duration: 3min
completed: 2026-02-11
---

# Phase 15 Plan 02: Collector Consolidation and Warning Elimination Summary

**Consolidated near-duplicate binding-name collection via Statement::as_declaration() delegation and annotated 7 data-model structs to achieve zero compiler warnings**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-11T20:17:47Z
- **Completed:** 2026-02-11T20:20:42Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Reduced collect_statement_decl_names from 20-line duplicate to 3-line delegation wrapper using Statement::as_declaration()
- Added targeted #[allow(dead_code)] to 7 structs: CaptureAnalysisResult, CollectResult, DollarCallSite, ImportInfo, ExportInfo, SegmentData, TransformOptions
- cargo build now produces zero warnings for qwik-optimizer-oxc (down from 7 field warnings)
- No file-level #![allow(unused)] directives anywhere in the crate
- All 158 tests pass (151 unit + 7 spec)

## Task Commits

Each task was committed atomically:

1. **Task 1: Consolidate collector.rs near-duplicate functions (DEAD-03)** - `8e308df` (refactor)
2. **Task 2: Annotate types.rs struct fields to eliminate remaining dead_code warnings** - `4ca9c68` (refactor)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/collector.rs` - Consolidated collect_statement_decl_names to thin delegation wrapper; added #[allow(dead_code)] to CaptureAnalysisResult
- `crates/qwik-optimizer-oxc/src/types.rs` - Added #[allow(dead_code)] to CollectResult, DollarCallSite, ImportInfo, ExportInfo, SegmentData, TransformOptions

## Decisions Made
- Used Statement::as_declaration() for consolidation rather than inlining at call site -- cleaner, preserves function boundary, avoids code churn
- Applied #[allow(dead_code)] at struct level rather than individual field level -- simpler, as multiple fields per struct are affected and all are intentional data-model fields

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 15 (Dead Code Removal) is complete
- Zero compiler warnings for qwik-optimizer-oxc
- All 158 tests pass
- Codebase is clean and ready for next milestone phases

## Self-Check: PASSED

All 3 files verified present. Both commit hashes (8e308df, 4ca9c68) found in git log.

---
*Phase: 15-dead-code-removal*
*Completed: 2026-02-11*
