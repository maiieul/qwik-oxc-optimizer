---
phase: 20-path-resolution
plan: 01
subsystem: transform
tags: [oxc, path-resolution, file-extensions, transpile, segment-extraction]

# Dependency graph
requires:
  - phase: 10-entry-strategy
    provides: "Segment extraction and entry strategy framework"
provides:
  - "Correct canonical filenames with file extension in origin prefix"
  - "Import paths that match actual segment filenames"
  - "Explicit extension support for bundler-compatible import paths"
  - "Correct output extensions for all transpile_ts/transpile_jsx combinations"
affects: [21-snapshot-alignment, 22-spec-compliance]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Match-based extension mapping: (transpile_ts, transpile_jsx, ext) triple"
    - "compute_output_extension helper shared between transform.rs import paths and lib.rs file paths"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"
    - "crates/qwik-optimizer-oxc/tests/snapshots/ (140+ snapshot files)"

key-decisions:
  - "Extension mapping uses match on (transpile_ts, transpile_jsx, ext) triple for clarity"
  - "main_path rewrite triggers on transpile_jsx too, not just transpile_ts"
  - "Snapshots accepted as bulk update since canonical filename format change is correct"

patterns-established:
  - "Extension logic: tsx->jsx (ts only), tsx->ts (jsx only), tsx->js (both), preserve (neither)"
  - "compute_output_extension in transform.rs mirrors output_extension in lib.rs"

# Metrics
duration: 4min
completed: 2026-02-12
---

# Phase 20 Plan 01: Path Resolution Summary

**Fixed 4 path resolution bugs (PATH-01 to PATH-04) in canonical filenames, import paths, explicit extensions, and transpile extension logic**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-12T00:56:05Z
- **Completed:** 2026-02-12T00:59:59Z
- **Tasks:** 2
- **Files modified:** 148

## Accomplishments
- Canonical filenames now include file extension in origin prefix (test.tsx_ not test_)
- Import paths in main module now match segment file paths exactly
- explicit_extensions option correctly appends output extension to lazy import paths
- output_extension correctly handles all 4 transpile combinations (ts, jsx, both, neither)
- 6 new PATH integration tests + updated existing tests and 140+ snapshots

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix path resolution logic (PATH-01 through PATH-04)** - `4b6db05` (fix)
2. **Task 2: Update and add tests for path resolution** - `4251007` (test)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/transform.rs` - Fixed build_canonical_filename, added compute_output_extension, updated build_segment_import_path and finalize_segments
- `crates/qwik-optimizer-oxc/src/lib.rs` - Fixed output_extension (3-arg), updated segment_data_to_analysis, main_path rewrite, added 6 PATH tests
- `crates/qwik-optimizer-oxc/src/const_replace.rs` - Formatting only (cargo fmt)
- `crates/qwik-optimizer-oxc/tests/snapshots/*.snap` - 140+ snapshots updated for canonical filename format

## Decisions Made
- Extension mapping logic uses exhaustive match on (transpile_ts, transpile_jsx, ext) triple for maximum clarity and correctness
- main_path rewrite now triggers when either transpile_ts OR transpile_jsx is true, since transpile_jsx alone can change the file extension (e.g., .tsx -> .ts)
- All snapshot updates accepted since they correctly reflect the PATH-01 canonical filename fix

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] main_path only rewritten when transpile_ts is true**
- **Found during:** Task 1
- **Issue:** The main_path extension rewrite only triggered when `transpile_ts` was true. With `transpile_jsx=true` but `transpile_ts=false`, a .tsx file would keep its original path instead of being rewritten to .ts
- **Fix:** Changed condition from `if transform_options.transpile_ts` to `if transform_options.transpile_ts || transform_options.transpile_jsx`
- **Files modified:** crates/qwik-optimizer-oxc/src/lib.rs
- **Verification:** test_path_04_transpile_jsx_only_ts_extension passes
- **Committed in:** 4b6db05

**2. [Rule 1 - Bug] Snapshot files needed bulk update for canonical filename change**
- **Found during:** Task 2
- **Issue:** 140+ snapshot test files contained the old canonical filename format (without extension prefix). All needed updating.
- **Fix:** Ran `cargo insta test --accept` to accept all updated snapshots
- **Files modified:** crates/qwik-optimizer-oxc/tests/snapshots/*.snap
- **Verification:** All snapshot tests pass
- **Committed in:** 4251007

---

**Total deviations:** 2 auto-fixed (2 bugs)
**Impact on plan:** Both auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All path resolution requirements (PATH-01 through PATH-04) are satisfied
- 160 unit tests + 1 snapshot test + 8 spec tests all pass with zero failures
- Ready for Phase 21 (snapshot alignment) and Phase 22 (spec compliance)

---
*Phase: 20-path-resolution*
*Completed: 2026-02-12*
