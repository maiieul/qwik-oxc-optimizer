---
phase: 09-capture-analysis-props-destructuring
plan: 01
subsystem: transform
tags: [oxc, props-destructuring, component$, _rawProps, _restProps, ast-rewrite]

# Dependency graph
requires:
  - phase: 08-core-detection-qrl-transforms
    provides: QwikTransform traverse pass with enter_call_expression/exit_expression hooks
provides:
  - Props destructuring analysis (analyze_props_destructuring)
  - AST rewrite (rewrite_props_references, build_rest_props_declaration)
  - QwikTransform integration for component$ props destructuring
  - param_names field on SegmentData/SegmentAnalysis
affects: [09-capture-analysis-props-destructuring/09-02, 10-code-move-segment-extraction]

# Tech tracking
tech-stack:
  added: []
  patterns: [recursive AST walk for identifier replacement, post-analysis mutation pattern]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/props_destructuring.rs
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/types.rs
    - crates/qwik-optimizer-oxc/src/lib.rs

key-decisions:
  - "Post-analysis mutation pattern: analyze_props_destructuring returns pure data, actual AST mutation happens in QwikTransform exit_expression"
  - "Recursive walk (rewrite_props_references) handles identifier replacement with scope-aware shadowing for nested arrows"
  - "param_names as Option<Vec<String>> on SegmentAnalysis with skip_serializing_if for clean JSON"

patterns-established:
  - "Props destructuring: analyze in enter_call_expression, mutate in exit_expression"
  - "Identifier rewriting via recursive walk with shadowing detection"
  - "Rest props: _restProps(_rawProps, [excluded_keys]) as prepended const declaration"

# Metrics
duration: 7min
completed: 2026-02-11
---

# Phase 9 Plan 1: Props Destructuring Summary

**Component$ props destructuring via analyze_props_destructuring + recursive identifier rewrite to _rawProps.key member expressions with _restProps support**

## Performance

- **Duration:** 7 min
- **Started:** 2026-02-11T04:31:45Z
- **Completed:** 2026-02-11T04:39:21Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Props destructuring detection and analysis for component$ arrow functions (ObjectPattern -> PropsDestructuringInfo)
- AST rewrite: ObjectPattern parameter replaced with _rawProps BindingIdentifier, body references replaced with _rawProps.key member expressions
- Rest pattern support: _restProps(_rawProps, ["key1", "key2"]) prepended to body, _restProps import added
- Renamed props ({count: c}) correctly use original key name (_rawProps.count, not _rawProps.c)
- param_names metadata on SegmentAnalysis for downstream consumers
- Non-component$ dollar calls (useTask$, $) correctly left unchanged

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement props_destructuring.rs with detection and rewrite logic** - `084feff` (feat)
2. **Task 2: Integrate props destructuring into QwikTransform and add integration tests** - `59dd7c2` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/props_destructuring.rs` - Full implementation: analyze_props_destructuring, rewrite_props_references, build_rest_props_declaration, 7 unit tests
- `crates/qwik-optimizer-oxc/src/transform.rs` - QwikTransform integration: active_props_info, enter/exit hooks, _restProps import tracking
- `crates/qwik-optimizer-oxc/src/types.rs` - param_names field on SegmentData and SegmentAnalysis
- `crates/qwik-optimizer-oxc/src/lib.rs` - param_names mapping in segment_data_to_analysis, 8 integration tests

## Decisions Made
- Used post-analysis mutation pattern: analyze_props_destructuring() is pure analysis returning PropsDestructuringInfo, actual AST mutation happens in exit_expression. This avoids fighting the borrow checker during traversal.
- Recursive walk approach for identifier replacement (rewrite_props_references) with scope-aware shadowing detection for nested arrow functions.
- param_names stored as Option<Vec<String>> on SegmentAnalysis with serde skip_serializing_if to keep JSON clean when not present.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Props destructuring is complete and runs before capture analysis
- Plan 09-02 (capture analysis) can now rely on _rawProps identifiers being in scope for component$ bodies
- The _restProps import tracking is ready for code_move (Phase 10) to emit in segment modules
- 78 total tests pass (75 unit + 3 integration spec tests), no regressions

---
*Phase: 09-capture-analysis-props-destructuring*
*Completed: 2026-02-11*
