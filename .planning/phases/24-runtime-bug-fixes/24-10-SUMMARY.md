---
phase: 24-runtime-bug-fixes
plan: 10
subsystem: optimizer
tags: [display-name, hash-collision, self-import, capture-analysis, oxc]

requires:
  - phase: 24-09
    provides: "Final audit confirming 5 runtime-breaking deviations (1 missing-import-used + 4 truly-missing-module)"
provides:
  - "Unique display names for nested $() calls via wrapper_callee_name context"
  - "JSX $() call display names in non-$-suffixed attributes (onClick={$(...)})"
  - "Fix for analyze_lambda_captures bailing on semantic parse errors"
  - "Runtime-breaking deviations reduced from 5 to 4 (missing-import-used: 1->0)"
affects: [24-11, 25-napi-crate]

tech-stack:
  added: []
  patterns:
    - "wrapper_callee_name propagation for non-dollar wrapper function display names"
    - "Tolerant parse error handling in capture analysis (proceed on semantic errors)"

key-files:
  modified:
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/tests/output_audit.rs"

key-decisions:
  - "Added wrapper_callee_name to CollectContext for non-dollar wrapper function tracking"
  - "analyze_lambda_captures proceeds on parse errors (only bails on empty body)"
  - "Runtime-breaking threshold tightened from 5 to 4 (0 missing-import-used remaining)"

patterns-established:
  - "wrapper_callee_name: when $() is argument to non-dollar call like component(), derive_display_name appends wrapper name"
  - "JSX attribute $() detection: direct $() calls in non-$-suffixed attributes get JSX event display names"

duration: 9min
completed: 2026-02-12
---

# Phase 24 Plan 10: Display Name Collision and API Import Fixes Summary

**Fixed display name collisions for nested $() calls via wrapper_callee_name context, and fixed api missing-import by tolerating semantic parse errors in capture analysis**

## Performance

- **Duration:** 9 min
- **Started:** 2026-02-12T17:51:27Z
- **Completed:** 2026-02-12T18:00:27Z
- **Tasks:** 3
- **Files modified:** 15

## Accomplishments

- Fixed display name collisions in example_1: three segments (renderHeader, renderHeader_component, renderHeader_div_q_e_click) now have unique names and hashes -- no duplicate const declarations
- Fixed api missing-import in example_drop_side_effects: onClick$ segment now gets `import { api } from "./test"` for module-level declarations from server$() calls
- Runtime-breaking deviations reduced from 5 to 4 (missing-import-used: 1->0)
- Regression threshold tightened from 5 to 4

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix derive_display_name for nested $() calls** - `19310cd` (fix)
2. **Task 2: Fix api missing-import in JSX event handler segments** - `6fa7591` (fix)
3. **Task 3: Update snapshots and spec tests for both fixes** - `af21547` (chore)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/src/collector.rs` - Added wrapper_callee_name to CollectContext, updated derive_display_name, added JSX $() call handling in non-$-suffixed attributes
- `crates/qwik-optimizer-oxc/src/transform.rs` - Fixed analyze_lambda_captures to proceed on semantic parse errors
- `crates/qwik-optimizer-oxc/src/lib.rs` - Added integration test for api self-import with await pattern
- `crates/qwik-optimizer-oxc/tests/output_audit.rs` - Tightened RUNTIME_BREAKING_THRESHOLD from 5 to 4
- 10 snapshot files updated (example_1, example_2, example_3, example_4, example_5, example_7, example_11, example_drop_side_effects, example_qwik_conflict, transform_qrl_in_regular_prop)
- `.planning/phases/23-output-audit/audit-raw.json` - Updated deviation counts

## Decisions Made

1. **wrapper_callee_name approach:** Added a new field to CollectContext rather than modifying derive_display_name's signature. When walking arguments of a non-dollar call like `component(...)`, the callee name is saved so nested `$()` calls can include it in their display name (e.g., `renderHeader_component`).

2. **Tolerant parse error handling:** Changed `analyze_lambda_captures` to only bail on empty program body, not on parse errors. The `await` keyword in a non-async function is a semantic error -- the AST is still well-formed and identifier references can be extracted. This was the root cause of the api missing-import bug.

3. **JSX $() in non-$-suffixed attributes:** Added detection for direct `$()` calls inside non-`$`-suffixed JSX attribute values (e.g., `onClick={$(...)}` vs `onClick$={...}`). These now use the JSX event display name derivation to get proper element/attribute context.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] JSX $() call in non-$-suffixed attributes needed dedicated handling**
- **Found during:** Task 1 (display name collision fix)
- **Issue:** The plan focused on the component() wrapper case, but example_1 also has `onClick={$(...)}` where `$()` is inside a non-$-suffixed attribute. This pattern went through generic walk_jsx_expression_for_calls without JSX context.
- **Fix:** Added detection in walk_jsx_element_for_calls for direct `$()` calls in non-$-suffixed attribute values, using derive_jsx_event_display_name for proper element/attribute context.
- **Files modified:** crates/qwik-optimizer-oxc/src/collector.rs
- **Committed in:** 19310cd (Task 1 commit)

**2. [Rule 1 - Bug] Root cause of api import was parse error bailout, not capture logic**
- **Found during:** Task 2 (api import fix)
- **Issue:** The plan hypothesized multiple potential causes (dollar_imports, module_level_decls, reclassification). The actual root cause was simpler: `analyze_lambda_captures` returned empty results when the lambda `() => await api()` produced a semantic parse error (await in non-async function).
- **Fix:** Changed the bailout condition from `!errors.is_empty()` to `body.is_empty()` so semantic errors don't prevent capture analysis.
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Committed in:** 6fa7591 (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 bugs)
**Impact on plan:** Both were refinements of the planned approach. The first extended the display name fix to cover an additional code pattern. The second simplified the fix by identifying the true root cause.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 162 specs compile without optimizer errors
- Runtime-breaking deviations: 4 (all truly-missing-module -- architectural limitations)
- Missing-import-used deviations: 0 (fully resolved)
- Ready for Plan 11 (if applicable) or Phase 25 (NAPI Crate)

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*
