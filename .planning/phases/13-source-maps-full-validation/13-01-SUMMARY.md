---
phase: 13-source-maps-full-validation
plan: 01
subsystem: codegen
tags: [source-maps, oxc-codegen, vlq, v3-source-map]

# Dependency graph
requires:
  - phase: 06-secondary-patterns-cross-reference
    provides: SOURCE-MAPS-MAPPING.md with OXC codegen API mapping and span strategy
provides:
  - emit_module() source map generation via OXC codegen source_map_path option
  - emit_segment_with_map() for segment module source maps from re-parsed code
  - Source map piping from emit results to TransformModule.map for main and segment modules
affects: [13-02, 13-03, 13-04]

# Tech tracking
tech-stack:
  added: []
  patterns: [OXC CodegenOptions source_map_path for source map generation, to_json_string() serialization]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/emit.rs
    - crates/qwik-optimizer-oxc/src/lib.rs
    - crates/qwik-optimizer-oxc/src/code_move.rs

key-decisions:
  - "Segment source maps generated from re-parsed string-constructed code (identity-like mappings) since original spans are lost in string construction"
  - "emit_module() takes source_filename parameter to set source map file field"
  - "normalize_code() replaced by emit_segment_with_map() for segment codegen path"

patterns-established:
  - "Source map generation: CodegenOptions { source_map_path } + with_source_text() + to_json_string()"
  - "emit_segment_with_map() parse+codegen pattern for normalized segment code with optional maps"

# Metrics
duration: 3min
completed: 2026-02-11
---

# Phase 13 Plan 01: Source Map Wiring Summary

**OXC source map generation wired into emit_module() and piped to TransformModule.map for both main and segment modules**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-11T17:34:41Z
- **Completed:** 2026-02-11T17:37:56Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- emit_module() now generates v3 source maps via OXC codegen when source_maps option is true
- Main module TransformModule.map contains valid source map JSON with version, mappings, and sources fields
- Segment modules receive source maps from re-parsed code via new emit_segment_with_map() function
- source_maps=false produces map: None on all modules (unchanged behavior)
- All 150 unit tests pass, all 162 specs transform without errors

## Task Commits

Each task was committed atomically:

1. **Task 1: Enable source map generation in emit_module** - `b2ce1ba` (feat)
2. **Task 2: Pipe source maps through lib.rs for main and segment modules** - `7b780d2` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/emit.rs` - Added source_filename parameter, source map generation via CodegenOptions source_map_path + with_source_text() + to_json_string(); added 2 unit tests
- `crates/qwik-optimizer-oxc/src/lib.rs` - Updated emit_module() call with input.path, replaced normalize_code with emit_segment_with_map for segments, added 2 integration tests
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Added emit_segment_with_map() function for segment source map generation via parse+codegen cycle

## Decisions Made
- Segment source maps use re-parsed string-constructed code since original spans are lost during string-based segment code construction. This produces identity-like mappings (useful for debugging column positions within generated lines). Future AST-based segment construction would enable proper source-position mapping.
- normalize_code() replaced entirely by emit_segment_with_map() which handles both normalization and optional source map generation in one pass.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Source map plumbing is complete for main and segment modules
- Ready for Phase 13 Plan 02 (source map content validation and spec integration)
- normalize_code() is now unused (can be removed in a future cleanup)

## Self-Check: PASSED

All files verified present, all commit hashes found in git log.

---
*Phase: 13-source-maps-full-validation*
*Completed: 2026-02-11*
