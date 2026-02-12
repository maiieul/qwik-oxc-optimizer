---
phase: 24-runtime-bug-fixes
plan: 08
subsystem: jsx-transform
tags: [jsx, react, jsxImportSource, pragma, code-move, segment]

requires:
  - phase: 24-06
    provides: "Regression gate with runtime-breaking threshold"
provides:
  - "Custom JSX import source extraction and propagation"
  - "React-style _jsx transform when @jsxImportSource pragma present"
  - "Segment modules emit correct JSX runtime import for custom sources"
  - "Tightened regression threshold from 10 to 5"
affects: [24-09, napi-integration]

tech-stack:
  added: []
  patterns:
    - "ImportTracker.custom_jsx_source threads JSX source through transform pipeline"
    - "transform_jsx_element_custom_source for React-style _jsx(tag, {props}) codegen"
    - "code_move.rs custom_jsx_source parameter for segment JSX import emission"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/lib.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/jsx_transform.rs"
    - "crates/qwik-optimizer-oxc/src/code_move.rs"
    - "crates/qwik-optimizer-oxc/tests/output_audit.rs"

key-decisions:
  - "Extract module path from @jsxImportSource pragma via string parsing (not regex)"
  - "Store custom_jsx_import_source as Option<String> on QwikTransform and ImportTracker"
  - "Reuse needs_jsx_sorted flag for custom source (import emission checks custom_jsx_source to decide _jsx vs _jsxSorted)"
  - "Tightened RUNTIME_BREAKING_THRESHOLD from 10 to 5"

patterns-established:
  - "Custom JSX source: stored on both QwikTransform (for event handler skip) and ImportTracker (for JSX codegen)"
  - "React-style JSX: _jsx(tag, {props}) with all props merged, no event handler renaming"

duration: 11min
completed: 2026-02-12
---

# Phase 24 Plan 08: JSX Import Source Propagation Summary

**Custom JSX import source extraction and React-style _jsx codegen, reducing runtime-breaking deviations from 7 to 5**

## Performance

- **Duration:** 11 min
- **Started:** 2026-02-12T07:14:34Z
- **Completed:** 2026-02-12T07:25:33Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Extracted module path from `@jsxImportSource react` pragma and stored as `Option<String>` on QwikTransform
- Implemented `transform_jsx_element_custom_source()` generating React-style `_jsx("tag", {props})` instead of Qwik's `_jsxSorted`
- Both entry module and segment modules now emit `import { jsx as _jsx } from "react/jsx-runtime"` when custom JSX source is set
- Resolved 2 of the remaining missing-import-used deviations in `example_jsx_import_source`
- Tightened regression gate threshold from 10 to 5

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract JSX import source module path and propagate to transform** - `cbe1e66` (feat)
2. **Task 2: Update JSX codegen and segment imports to use custom source** - `8bc5e89` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/lib.rs` - Extract module path from @jsxImportSource pragma
- `crates/qwik-optimizer-oxc/src/transform.rs` - Option<String> field, ImportTracker custom_jsx_source, conditional _jsx import emission
- `crates/qwik-optimizer-oxc/src/jsx_transform.rs` - transform_jsx_element_custom_source() for React-style _jsx codegen
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Custom JSX source parameter for segment import generation
- `crates/qwik-optimizer-oxc/tests/output_audit.rs` - Tightened RUNTIME_BREAKING_THRESHOLD from 10 to 5

## Decisions Made
- Used string parsing (not regex) for extracting module path from @jsxImportSource pragma -- simpler and sufficient since the pragma format is well-defined
- Reused `needs_jsx_sorted` flag to signal that JSX imports are needed, with the import emission logic checking `custom_jsx_source` to decide between `_jsxSorted` from core and `_jsx` from custom source -- avoids adding a separate flag
- React-style JSX transform merges all props into a single object and does NOT rename event handlers (no `q-e:click` renaming) -- matches SWC expected output

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## Next Phase Readiness
- 5 runtime-breaking deviations remain (4 truly-missing-module + 1 missing-import-used)
- The 4 truly-missing-module deviations are structural (pre-compiled QRL reverse-engineering) and accepted
- The 1 remaining missing-import-used is an edge case (aliased exports or enum tracking)
- Ready for Plan 24-09 or Phase 25

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*

## Self-Check: PASSED
- All 5 modified files exist on disk
- Commit cbe1e66 (Task 1) verified in git log
- Commit 8bc5e89 (Task 2) verified in git log
