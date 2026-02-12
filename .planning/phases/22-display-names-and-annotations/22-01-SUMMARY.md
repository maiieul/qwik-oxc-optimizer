---
phase: 22-display-names-and-annotations
plan: 01
subsystem: transform
tags: [oxc, display-names, pure-annotations, tree-shaking, qwik]

# Dependency graph
requires:
  - phase: 21-import-correctness
    provides: "Correct import stripping and scoping"
provides:
  - "Function scope prefix tracking in collector for nested display names"
  - "Selective PURE annotation on tree-shakeable wrappers only (componentQrl)"
  - "Integration tests verifying NAME-01 and PURE-01 compliance"
affects: []

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "scope_prefix field in CollectContext for composing function hierarchy into display names"
    - "is_tree_shakeable_dollar_call() guard for selective PURE annotation placement"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"
    - "crates/qwik-optimizer-oxc/tests/snapshots/*.snap (28 files)"

key-decisions:
  - "scope_prefix composes with existing prefix for deeply nested function declarations"
  - "Only component$ is tree-shakeable; all other Qrl wrappers are side-effectful"

patterns-established:
  - "scope_prefix: function declaration names tracked as context for nested dollar calls"
  - "Selective PURE: is_tree_shakeable_dollar_call() controls annotation placement"

# Metrics
duration: 5min
completed: 2026-02-12
---

# Phase 22 Plan 01: Display Names and PURE Annotations Summary

**Function scope prefix tracking for nested display names and selective PURE annotation on componentQrl only**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-12T02:20:01Z
- **Completed:** 2026-02-12T02:25:51Z
- **Tasks:** 2
- **Files modified:** 31

## Accomplishments
- Nested segment display names now include full parent context hierarchy (e.g., `App_Header_component` instead of `s_0`)
- PURE annotation applied only to `componentQrl()` -- `useStylesQrl`, `useTaskQrl`, and other side-effectful wrappers no longer get PURE
- 4 new integration tests verify both NAME-01 and PURE-01 behaviors
- All 168 tests pass (164 existing + 4 new), metadata match 250/250, zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix display names and PURE annotations (NAME-01 + PURE-01)** - `61e3177` (feat)
2. **Task 2: Add integration tests for NAME-01 and PURE-01** - `2b348b1` (test)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/collector.rs` - Added scope_prefix field, function body walking, and scope-aware display name derivation
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added is_tree_shakeable_dollar_call() and conditional PURE annotation
- `crates/qwik-optimizer-oxc/src/lib.rs` - 4 new integration tests for NAME-01 and PURE-01
- `crates/qwik-optimizer-oxc/tests/snapshots/*.snap` - 28 snapshot files updated with corrected display names and PURE placement

## Decisions Made
- **scope_prefix composition:** When entering nested function declarations, scope_prefix composes with any existing prefix (e.g., function inside function) to support arbitrary nesting depth.
- **Tree-shakeability:** Only `component$` produces a tree-shakeable wrapper (`componentQrl`). All other Qrl wrappers (`useStylesQrl`, `useTaskQrl`, `useVisibleTaskQrl`, `serverStuffQrl`, `serverLoaderQrl`, `useResourceQrl`, etc.) are side-effectful and must not have PURE annotations to prevent incorrect removal by bundlers.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Phase 22 (final v5.0 phase) complete
- All v5.0 compliance metrics maintained: 157/162 module count match, 250/250 metadata match, 0 transform errors
- The optimizer is now feature-complete for v5.0 Drop-in Replacement Compliance

---
*Phase: 22-display-names-and-annotations*
*Completed: 2026-02-12*
