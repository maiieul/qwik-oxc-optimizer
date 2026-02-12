---
phase: 13-source-maps-full-validation
plan: 03
subsystem: transform
tags: [strip_exports, jsx-event-handlers, inline-hoist, segment-extraction, oxc]

# Dependency graph
requires:
  - phase: 13-02
    provides: "Diagnostic categorization, import alias detection, core_module handling"
provides:
  - "Working filter_exports implementation replacing todo!() stub"
  - "Inline/Hoist strategy correctly produces only main module (no empty segments)"
  - "JSX event handler segment extraction for $-suffixed attributes"
  - "148/162 module count match (91.4%)"
affects: [13-04, final-validation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "In-place AST mutation for filter_exports (preserves scope IDs)"
    - "JSX event handler pre-scan before transpile_jsx transformation"
    - "Lambda-only extraction: only arrow/function expressions create segments, not identifiers/calls"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/filter_exports.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/collector.rs"

key-decisions:
  - "In-place body mutation for strip_exports (new AST nodes lack scope_id, causing traverse panics)"
  - "Skip segment TransformModule creation for Inline/Hoist strategies (SWC only produces main module)"
  - "JSX event handler segment extraction runs independently of transpile_jsx flag"
  - "Only extract arrow/function expression bodies from $-suffixed attributes (skip identifiers and call expressions)"

patterns-established:
  - "get_jsx_lambda_span: Filter JSX expression values to only arrow/function for segment extraction"
  - "Pre-scan pattern: Create segments from JSX before immutable borrow of collected data"

# Metrics
duration: ~25min
completed: 2026-02-11
---

# Phase 13 Plan 03: strip_exports & JSX event handler extraction Summary

**Working filter_exports replacing todo!() stub, Inline/Hoist module fix (79->109), and JSX event handler segment extraction (109->148/162 module count match)**

## Performance

- **Duration:** ~25 min
- **Started:** 2026-02-11 (continued from prior session)
- **Completed:** 2026-02-11
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Implemented filter_exports.rs: replaces export function/arrow bodies with throw stubs while preserving scope IDs
- Fixed Inline/Hoist strategy to not produce separate segment TransformModules (gain: 79->109)
- Added JSX event handler $-call detection in collector and segment creation in transform (gain: 109->148)
- Module count coverage: 148/162 (91.4%), exceeding 145+ target

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement strip_exports and wire into pipeline** - `5a96674` (feat)
2. **Task 2: Fix inline/hoist module production and add JSX event handler extraction** - `6af736b` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/filter_exports.rs` - Full implementation replacing todo!() stub; in-place body mutation with throw stubs
- `crates/qwik-optimizer-oxc/src/lib.rs` - filter_exports wired into pipeline; Inline/Hoist skip for segment modules
- `crates/qwik-optimizer-oxc/src/collector.rs` - JSX event handler $-call detection (DollarCallSite for $-suffixed attributes)
- `crates/qwik-optimizer-oxc/src/transform.rs` - JSX event handler segment creation (record_jsx_event_segment, create_jsx_event_segments_recursive); pre-scan outside transpile_jsx guard

## Decisions Made
- **In-place AST mutation for filter_exports**: Creating new AST nodes via AstBuilder produces nodes without scope_id, causing panics in traverse_mut walker. Fixed by mutating existing function/arrow bodies in-place, preserving scope IDs.
- **Skip segment TransformModule for Inline/Hoist**: The SWC optimizer only produces the main module for Inline/Hoist strategies. We were incorrectly creating empty-code segment modules. Fixed by adding `continue` for is_inline_like strategies.
- **JSX event pre-scan independent of transpile_jsx**: Specs with transpile_jsx=false still expect JSX event handler segments. Moved pre-scan outside the transpile_jsx guard.
- **Lambda-only extraction**: Only arrow/function expression values in $-suffixed JSX attributes create segments. Identifier references (like `onClick$={handler}`) and call expressions (like `onClick$={sync$(...)}`) are skipped to avoid double-counting.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] JSX event handler segment extraction**
- **Found during:** Task 2
- **Issue:** Plan focused on reg_ctx_name and transpile-only edge cases, but the dominant mismatch category (77/83) was JSX event handler extraction -- $-suffixed JSX attributes creating segments
- **Fix:** Added full JSX event handler detection in collector and segment creation in transform
- **Files modified:** collector.rs, transform.rs
- **Verification:** Module count match jumped from 109 to 148/162
- **Committed in:** 6af736b (Task 2 commit)

**2. [Rule 1 - Bug] Inline/Hoist producing phantom segment modules**
- **Found during:** Task 2
- **Issue:** Inline/Hoist strategies were creating separate empty-code TransformModule entries for each segment, but SWC only produces the main module
- **Fix:** Added `continue` to skip segment module creation when is_inline_like is true
- **Files modified:** lib.rs
- **Verification:** Module count match jumped from 79 to 109/162
- **Committed in:** 6af736b (Task 2 commit)

**3. [Rule 1 - Bug] OXC 0.113 scope_id panic with new AST nodes**
- **Found during:** Task 1
- **Issue:** First filter_exports attempt created new ArrowFunctionExpression nodes via AstBuilder. These lacked scope_id, causing panics in walk_arrow_function_expression during traverse_mut.
- **Fix:** Changed approach to mutate existing node bodies in-place rather than creating new nodes
- **Files modified:** filter_exports.rs
- **Verification:** All tests pass, no panics
- **Committed in:** 5a96674 (Task 1 commit)

---

**Total deviations:** 3 auto-fixed (2 bugs, 1 missing critical functionality)
**Impact on plan:** Deviations were essential for achieving the coverage target. The plan's reg_ctx_name focus was less impactful than the Inline/Hoist fix and JSX event handler extraction which together accounted for 69 additional matches. The reg_ctx_name specs all use Inline/Hoist strategy (1 module expected), so they were automatically fixed by the Inline/Hoist module production fix.

## Issues Encountered
- OXC 0.113 API differences from documentation: binding_pattern is a flat enum, variable_declarator takes 6 args, FormalParameterRest vs BindingRestElement. Resolved by referencing existing import_rewrite.rs patterns.
- Borrow checker conflict in exit_expression: self.collected.module_imports immutable borrow prevented mutable access for create_jsx_event_segments_recursive. Resolved by moving pre-scan before the immutable borrow.

## Remaining Mismatches (14/162)
- 6 specs produce 0 modules (likely parse errors in spec source code)
- 3 specs missing namespace/host: prefixed event handler segments
- 2 specs related to strip_ctx_name/strip_event_handlers (unimplemented)
- 2 specs related to multi-input module handling
- 1 spec over-producing due to non-Qwik $ attribute false positive

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- 148/162 module count match provides strong foundation for final validation
- Remaining 14 mismatches are edge cases (namespace JSX attributes, strip_event_handlers, parse errors)
- Ready for Plan 04 (final validation pass)

## Self-Check: PASSED
- All 4 modified files exist
- Both task commits verified (5a96674, 6af736b)
- SUMMARY.md created at expected path

---
*Phase: 13-source-maps-full-validation*
*Completed: 2026-02-11*
