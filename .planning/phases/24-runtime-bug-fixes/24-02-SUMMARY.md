---
phase: 24-runtime-bug-fixes
plan: 02
subsystem: optimizer
tags: [oxc, imports, segment-extraction, code-move, capture-analysis]

# Dependency graph
requires:
  - phase: 24-runtime-bug-fixes
    provides: "Plan 01 fixed optimizer failures and QRL extraction pipeline"
  - phase: 23-output-audit
    provides: "audit-raw.json identifying 76 import-related runtime-breaking deviations"
provides:
  - "Complete segment import re-emission pipeline for user-code imports"
  - "ImportKind enum distinguishing default, namespace, and named imports"
  - "ReemittedImport struct with full import metadata (kind, alias, source)"
  - "Correct default, namespace, named, and aliased named import emission in segment modules"
affects: [24-runtime-bug-fixes remaining plans, output-audit re-runs]

# Tech tracking
tech-stack:
  added: []
  patterns: [ImportKind-tagged import specifiers, ReemittedImport struct for capture-to-segment import propagation]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/types.rs
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/code_move.rs
    - .planning/phases/23-output-audit/audit-raw.json

key-decisions:
  - "Added ImportKind enum to types.rs to distinguish default/namespace/named import specifiers"
  - "Enriched ReemittedImport struct with kind and imported_name fields for alias tracking"
  - "Store needed_imports on ALL segments (top-level and nested) -- imports are needed regardless of nesting"

patterns-established:
  - "ImportKind tagging: each import specifier now has a parallel kind (Default, Namespace, Named)"
  - "Capture-to-segment import propagation: compute_captures -> ReemittedImport -> seg.needed_imports -> code_move emission"

# Metrics
duration: 7min
completed: 2026-02-12
---

# Phase 24 Plan 02: Segment Import Resolution Summary

**Fixed segment import re-emission pipeline: user-code imports (default, namespace, named, aliased) now correctly emitted in extracted segment modules**

## Performance

- **Duration:** 7 min
- **Started:** 2026-02-12T05:09:24Z
- **Completed:** 2026-02-12T05:17:23Z
- **Tasks:** 2
- **Files modified:** 82 (4 source files + 78 snapshot tests)

## Accomplishments

- Built complete import re-emission pipeline: collector classifies imports by kind, capture analysis tracks reemitted imports with full metadata, transform stores them on segments, code_move emits them
- All four import patterns now correctly emitted in segments: `import dep3 from "dep3/something"` (default), `import * as dep2 from "dep2"` (namespace), `import { foo } from "../state"` (named), `import { bar as bbar } from "../state"` (aliased named)
- example_11 segments now match expected output for all user imports (dep3, dep2, bbar, foo)
- 78 snapshot tests updated to reflect correct import emission across the entire test suite
- Zero test regressions: 172 unit tests, 1 snapshot suite, 7 spec tests all pass (4 pre-existing ctxKind mismatches unchanged)

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix segment import re-emission pipeline** - `6537bed` (feat)
2. **Task 2: Run audit and measure import fix progress** - `d498b85` (feat)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/src/types.rs` - Added `ImportKind` enum (Default/Namespace/Named) and `specifier_kinds: Vec<ImportKind>` field on `ImportInfo`
- `crates/qwik-optimizer-oxc/src/collector.rs` - Added `ReemittedImport` struct, updated `collect_import()` to track specifier kinds, updated `compute_captures()` to use position-based kind lookup and alias tracking
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added `needed_imports` population in `exit_expression()` converting `ReemittedImport` entries to `ImportInfo` entries on `SegmentData`
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Added user-code import emission loop in `build_segment_code_with_hoisted()` handling all three import kinds with alias support
- `.planning/phases/23-output-audit/audit-raw.json` - Regenerated with updated audit results
- 78 snapshot test files updated to include user-code imports in segment modules

## Decisions Made

1. **ImportKind enum instead of boolean flag** - The previous `is_default: bool` approach in `CaptureAnalysisResult` could not distinguish default from namespace imports (both have a single specifier). Added a proper `ImportKind` enum with three variants.

2. **ReemittedImport struct** - Replaced the `(String, String, bool)` tuple in `CaptureAnalysisResult.reemitted_imports` with a named struct carrying `local_name`, `source`, `kind`, and `imported_name` (for aliases). This makes the API self-documenting and extensible.

3. **Needed imports on all segments** - Store `needed_imports` on both top-level and nested segments. Even top-level `$()` calls that don't capture variables still need user imports in their segment modules.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The remaining "missing imports" in the audit (123 total, 74 in segments) are primarily two distinct patterns NOT addressed by this plan: (a) self-imports (`import { Header } from "./test"` where the segment references its own module's exports), and (b) framework imports that are detected by body-code string scanning but only for certain helpers. These are separate fix patterns for Plans 03-04.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Import re-emission pipeline fully operational for user-code imports
- Remaining segment import gaps are concentrated in self-import pattern (module-level declarations used as JSX components)
- Framework import detection in code_move.rs may need expansion for some helpers (_wrapProp, _fnSignal, _restProps in JSX-transpiled segment bodies)
- Plans 03-04 can address code generation patterns, capture analysis, and JSX transform deviations

## Self-Check: PASSED

- All 4 modified source files exist on disk
- Both task commits (6537bed, d498b85) exist in git history
- Key code patterns verified: ImportKind enum, ReemittedImport struct, needed_imports population, user import emission loop

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*
