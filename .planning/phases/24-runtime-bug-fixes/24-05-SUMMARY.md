---
phase: 24-runtime-bug-fixes
plan: 05
subsystem: compiler
tags: [oxc, capture-analysis, module-level-decls, self-imports, segment-extraction]

# Dependency graph
requires:
  - phase: 24-runtime-bug-fixes (plan 04)
    provides: "FINAL-AUDIT.md identifying 52 missing-import-used deviations"
provides:
  - "Module-level declaration references in nested segments generate self-imports"
  - "compute_captures() no longer skips module_level_decls"
  - "reclassify_module_level_decl_captures() converts decl captures to needed_imports"
affects: [24-06, segment-extraction, code-move]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Self-import pattern: module-level decls become import { X } from './module' in segments"]

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/tests/snapshots/*.snap (20 files)"

key-decisions:
  - "Module-level declarations reclassified as needed_imports (self-imports) rather than captures, matching SWC behavior"
  - "SWC generates import { X } from './module' for module-level decl references, not _captures[] serialization"

patterns-established:
  - "Self-import pattern: nested segment modules import module-level declarations from parent module via import { X } from './stem'"

# Metrics
duration: 6min
completed: 2026-02-12
---

# Phase 24 Plan 05: Capture Analysis Fix for Module-Level Declarations Summary

**Module-level declaration references in nested segments now generate self-imports (import { X } from "./module") instead of being silently dropped, fixing 22 missing-import-used deviations**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-12T06:42:02Z
- **Completed:** 2026-02-12T06:48:06Z
- **Tasks:** 2
- **Files modified:** 22

## Accomplishments

- Removed the `module_level_decls.contains()` skip in `compute_captures()` so module-level declarations (const, function, class) are no longer silently dropped from capture analysis
- Added `reclassify_module_level_decl_captures()` method that converts module-level declaration captures into self-imports (needed_imports with source `"./module_stem"`), matching SWC optimizer behavior
- Updated JSX event handler capture filtering to allow module_level_decls through the parent scope filter
- 20 snapshot files updated with correct self-import statements (e.g., `import { Header } from "./test"`)
- All 264/264 segment metadata matches, 160/162 module count matches, 0 transform errors

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove module_level_decls skip from compute_captures** - `0b9b9af` (feat)
2. **Task 2: Update transform capture filtering and spec test known deviations** - `c5f11ec` (feat)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/src/collector.rs` - Removed module_level_decls skip from compute_captures(), added test_compute_captures_module_level_decl_captured unit test
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added self_import_source() and reclassify_module_level_decl_captures() methods, updated JSX event handler filter to include module_level_decls, applied reclassification in both JSX event handler and exit_expression capture processing
- `crates/qwik-optimizer-oxc/tests/snapshots/*.snap` (20 files) - Updated expected output with new self-import statements

## Decisions Made

- **Self-import over capture:** The plan assumed module-level declarations should be added as `_captures[]` entries. Investigation of SWC reference output revealed they should be self-imports (`import { X } from "./module"`). This is because the SWC optimizer re-imports module-level declarations from the parent module rather than serializing/restoring them via the capture mechanism. Adjusted implementation accordingly.
- **Reclassification approach:** Rather than changing `compute_captures()` signature to know about module paths, added a post-processing step (`reclassify_module_level_decl_captures()`) that moves module-level decl names from `capture_names` to `needed_imports` with the correct self-import source.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Module-level declarations should be self-imports, not captures**
- **Found during:** Task 2 (Transform capture filtering)
- **Issue:** Plan specified adding module-level declarations as captures, but SWC reference output shows they should be self-imports (`import { X } from "./module"`). Initial implementation caused 3 spec failures (example_8, example_invalid_references, example_strip_exports_used) where captures=true but SWC expected captures=false.
- **Fix:** Added `reclassify_module_level_decl_captures()` to convert module-level decl captures to `needed_imports` (self-imports from parent module). The method extracts module-level decl names from capture_names and creates ImportInfo entries with source `"./module_stem"`.
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** All 264/264 metadata matches pass, 3 previously-failing specs now pass
- **Committed in:** c5f11ec (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - bug fix)
**Impact on plan:** Essential correction. Without reclassification, 3 specs would regress. The self-import approach matches SWC behavior exactly.

## Issues Encountered

None -- all issues were handled inline during implementation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Module-level declaration self-imports working correctly for nested segments
- 30 framework/library import deviations remain (Plan 06 scope: content-based import detection)
- 4 truly-missing-module deviations remain (pre-compiled QRL reverse-engineering, out of Phase 24 scope)
- All regression guards pass (v4.0 baseline maintained)

## Self-Check: PASSED

- FOUND: collector.rs
- FOUND: transform.rs
- FOUND: 24-05-SUMMARY.md
- FOUND: 0b9b9af (Task 1)
- FOUND: c5f11ec (Task 2)

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*
