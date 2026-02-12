---
phase: 21-import-correctness
plan: 01
subsystem: transform
tags: [oxc, imports, qrl, segment-extraction, ast-transform]

# Dependency graph
requires:
  - phase: 20-path-resolution
    provides: correct output file paths and extensions
provides:
  - "IMPORT-01: $-suffixed imports stripped from main module output"
  - "IMPORT-02: Qrl-suffixed imports scoped to correct modules (main vs segment)"
  - "Non-dollar Qwik core imports (useStore, etc.) preserved in main module"
  - "Build constant imports (isServer, isBrowser, isDev) excluded from re-emission"
affects: [22-final-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Import stripping via exit_program AST filtering"
    - "Segment-scoped imports via pending_segment_qrl_imports transfer"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/code_move.rs"
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/types.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "Strip ALL Qwik core imports and re-emit only needed specifiers, rather than mutating individual specifiers in OXC arena types"
  - "Use pending_segment_qrl_imports vec to defer nested Qrl import assignment to finalize_segments, avoiding complex mid-traverse bookkeeping"
  - "Exclude build constants (isServer, isBrowser, isDev) from re-emission since const_replace already strips them"
  - "Track specifier_aliases in ImportInfo to correctly re-emit aliased imports (e.g., isServer as myServer)"

patterns-established:
  - "Import filtering: exit_program skips Qwik core imports, re-emits non-dollar specifiers with alias awareness"
  - "Segment-scoped imports: record_segment conditionally routes Qrl imports to main vs parent segment based on dollar_call_stack depth"

# Metrics
duration: 12min
completed: 2026-02-12
---

# Phase 21 Plan 01: Import Correctness Summary

**Strip consumed $-suffixed imports from main module and scope Qrl-suffixed imports to correct segment modules via AST-level import filtering in exit_program**

## Performance

- **Duration:** 12 min
- **Started:** 2026-02-12T01:27:45Z
- **Completed:** 2026-02-12T01:39:45Z
- **Tasks:** 2
- **Files modified:** 5 source files + 150 snapshot files

## Accomplishments
- IMPORT-01: Main module output no longer contains consumed $-suffixed imports (component$, useStyles$, $, etc.)
- IMPORT-02: Qrl-suffixed imports (useStylesQrl, etc.) appear only in the segment modules that reference them, not in the main module
- Non-dollar Qwik core imports (useStore, etc.) correctly preserved in main module output
- 4 new integration tests verify all import correctness behaviors
- All 164 lib tests, 1 snapshot test, and 8 spec tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix import stripping and scoping (IMPORT-01 + IMPORT-02)** - `7160f51` (feat)
2. **Task 2: Add integration tests for IMPORT-01 and IMPORT-02** - `1da9768` (test)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added import stripping in exit_program, conditional Qrl import routing in record_segment, pending_segment_qrl_imports field and finalize_segments transfer
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Added segment_qrl_names emission in build_segment_code_with_hoisted
- `crates/qwik-optimizer-oxc/src/collector.rs` - Added specifier_aliases tracking in collect_import
- `crates/qwik-optimizer-oxc/src/types.rs` - Added segment_qrl_names to SegmentData, specifier_aliases to ImportInfo
- `crates/qwik-optimizer-oxc/src/lib.rs` - Added 4 integration tests for IMPORT-01 and IMPORT-02
- `crates/qwik-optimizer-oxc/tests/snapshots/*.snap` - 150 snapshot files updated to reflect stripped original imports

## Decisions Made
- Strip ALL Qwik core imports and re-emit only needed specifiers: simpler than mutating individual specifiers in OXC arena types
- Use pending_segment_qrl_imports to defer nested Qrl import assignment to finalize_segments
- Exclude build constants from re-emission since const_replace already handles them
- Track specifier_aliases in ImportInfo for correct aliased import re-emission

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed build constant re-emission causing test failure**
- **Found during:** Task 1 (import stripping implementation)
- **Issue:** Re-emitting non-dollar specifiers from Qwik core imports included build constants (isServer, isBrowser, isDev) that had already been stripped by const_replace, causing test_const_replace_aliased_import to fail
- **Fix:** Added BUILD_CONSTANTS exclusion list in exit_program import re-emission; also added specifier_aliases to ImportInfo for correct alias preservation
- **Files modified:** transform.rs, collector.rs, types.rs
- **Verification:** test_const_replace_aliased_import passes
- **Committed in:** 7160f51 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Auto-fix necessary for correctness with existing const_replace behavior. No scope creep.

## Issues Encountered
None - the build constant issue was identified and resolved during implementation.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Import correctness verified for both IMPORT-01 and IMPORT-02
- All 164+ tests pass with zero regressions
- Ready for Phase 22 (final validation)

## Self-Check: PASSED

All files verified to exist. All commit hashes verified in git log.

---
*Phase: 21-import-correctness*
*Completed: 2026-02-12*
