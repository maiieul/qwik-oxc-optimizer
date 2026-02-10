---
phase: 02-generate-all-spec-files
plan: 03
subsystem: spec-generation
tags: [qwik-optimizer, signal-helpers, derived-signals, props-wrapping, hoisted-functions, immutable-analysis, fn-signal, wrap-prop]

# Dependency graph
requires:
  - phase: 01-oxc-ast-utility
    provides: "oxc-ast-util binary for AST generation"
  - phase: 02-generate-all-spec-files (plan 01)
    provides: "Spec template pattern and convention detection methodology"
provides:
  - "27 spec files covering signal helpers, derived signals, props optimization, immutable analysis, and function wrapping"
  - "Complete CONV-04 (signal helpers) documentation across all wrapping/non-wrapping scenarios"
  - "Complete CONV-14 (hoisted functions) documentation with loop and multi-parameter patterns"
  - "Complete CONV-11 (props destructuring) documentation with _rawProps and _restProps patterns"
affects: [03-conventions-document, implementation-phases]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Spec file generation from SWC optimizer snapshots with convention detection"
    - "_fnSignal hoisted function parameter ordering documented across variants"
    - "Wrapping vs non-wrapping decision rules for signal helpers"

key-files:
  created:
    - ".planning/spec/example_props_wrapping_children.md"
    - ".planning/spec/example_props_wrapping_children2.md"
    - ".planning/spec/hoisted_fn_signal_in_loop.md"
    - ".planning/spec/impure_template_fns.md"
    - ".planning/spec/lib_mode_fn_signal.md"
    - ".planning/spec/should_mark_props_as_var_props_for_inner_cmp.md"
    - ".planning/spec/should_not_wrap_fn.md"
    - ".planning/spec/should_not_wrap_ternary_function_operator_with_fn.md"
    - ".planning/spec/should_not_wrap_var_template_string.md"
    - ".planning/spec/should_wrap_inner_inline_component_prop.md"
    - ".planning/spec/should_wrap_logical_expression_in_template.md"
    - ".planning/spec/should_wrap_object_with_fn_signal.md"
    - ".planning/spec/should_wrap_prop_from_destructured_array.md"
  modified: []

key-decisions:
  - "Task 1 files (14) were already committed by prior plan executions -- verified identical content and skipped re-commit"
  - "Task 2 files (13) were the genuinely missing specs that needed generation"
  - "Hoisted function parameter ordering varies by source expression order, not fixed props-first rule"

patterns-established:
  - "Wrapping rules: direct prop/store access wrapped, local variable derived from prop NOT wrapped"
  - "Object literals with only local signals NOT wrapped, mixed prop+local wrapped with _fnSignal"
  - "Destructured props renamed to _rawProps; non-destructured props keep original name"
  - "Function call results never wrapped with _fnSignal regardless of signal arguments"
  - "Inline function components not extracted to separate segments"

# Metrics
duration: 16min
completed: 2026-02-10
---

# Phase 2 Plan 3: Signal Helpers, Derived Signals, and Props Wrapping Spec Generation Summary

**27 spec files documenting signal helper transformations (_fnSignal, _wrapProp, _wrapSignal), hoisted function patterns, props wrapping/non-wrapping decision rules, and immutable analysis optimizations**

## Performance

- **Duration:** 16 min
- **Started:** 2026-02-10T18:31:47Z
- **Completed:** 2026-02-10T18:47:50Z
- **Tasks:** 2
- **Files created:** 13 (14 already existed from prior plan executions)

## Accomplishments
- Generated 13 new spec files covering props wrapping children, hoisted function signals in loops, impure template functions, lib mode, var props marking, and wrapping/non-wrapping decision boundaries
- Verified all 27 spec files from this plan batch exist with complete convention detection
- Documented critical optimizer behaviors: parameter ordering in hoisted functions, wrapping decision rules for mixed expressions, destructured vs non-destructured prop handling

## Task Commits

Each task was committed atomically:

1. **Task 1: Generate spec files for signal and derived signal tests (14 specs)** - No commit (all 14 files already existed from prior plan executions: 33af14f, dffad07, 41b64ef, 495c11a, e74cfb3, b8aeba9)
2. **Task 2: Generate spec files for props wrapping and function wrapping tests (13 specs)** - `5421f7f` (feat)

**Plan metadata:** Pending

## Files Created/Modified

### Task 2 (13 new files)
- `.planning/spec/example_props_wrapping_children.md` - Destructured props in children array; _rawProps renaming; mixed wrapping rules for object literals
- `.planning/spec/example_props_wrapping_children2.md` - Non-destructured props in children; different hoisted fn parameter order than children variant
- `.planning/spec/hoisted_fn_signal_in_loop.md` - _fnSignal usage inside .map() loops with loop variable as signal source (CONV-14 key test)
- `.planning/spec/impure_template_fns.md` - Impure function calls NOT wrapped; _auto_ prefix export pattern for cross-segment locals
- `.planning/spec/lib_mode_fn_signal.md` - transpile_jsx without transpile_ts produces .ts extensions; _wrapProp single-arg form
- `.planning/spec/should_mark_props_as_var_props_for_inner_cmp.md` - useResource$/useResourceQrl; multiple component segments; Image component var props
- `.planning/spec/should_not_wrap_fn.md` - Function call results not wrapped with _fnSignal
- `.planning/spec/should_not_wrap_ternary_function_operator_with_fn.md` - Ternary with function calls in both branches not wrapped
- `.planning/spec/should_not_wrap_var_template_string.md` - Template literals mixing function calls + signal.value not wrapped; useComputed$/useComputedQrl
- `.planning/spec/should_wrap_inner_inline_component_prop.md` - Inline function components not extracted; _wrapProp applied at both call site and definition
- `.planning/spec/should_wrap_logical_expression_in_template.md` - Logical OR signal expression wrapped with both signals as dependencies
- `.planning/spec/should_wrap_object_with_fn_signal.md` - Direct props.x.y access wrapped vs local variable derived from prop not wrapped
- `.planning/spec/should_wrap_prop_from_destructured_array.md` - Store tracking through array/object destructuring; _fnSignal reuse with same accessor pattern

### Task 1 (14 files already existed)
- `.planning/spec/should_not_transform_events_on_non_elements.md`
- `.planning/spec/should_split_spread_props.md`
- `.planning/spec/should_transform_event_names_without_jsx_transpile.md`
- `.planning/spec/example_derived_signals_children.md`
- `.planning/spec/example_derived_signals_cmp.md`
- `.planning/spec/example_derived_signals_complext_children.md`
- `.planning/spec/example_derived_signals_div.md`
- `.planning/spec/example_derived_signals_multiple_children.md`
- `.planning/spec/example_getter_generation.md`
- `.planning/spec/example_immutable_analysis.md`
- `.planning/spec/example_immutable_function_components.md`
- `.planning/spec/example_props_optimization.md`
- `.planning/spec/example_props_wrapping.md`
- `.planning/spec/example_props_wrapping2.md`

## Decisions Made
- Task 1's 14 files were already committed identically by prior plan executions (plans 01, 02, 04, 05, 06 had overlapping file coverage). Verified zero diff and skipped re-commit.
- Hoisted function parameter ordering varies by source expression order (e.g., `fromLocal + fromProps` puts `fromLocal` as p0 in children2 but `_rawProps` as p0 in children variant due to destructuring renaming).
- Objects containing only local signals are never wrapped with `_fnSignal`, while mixed prop+local objects are.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Task 1 files already committed by prior plan executions**
- **Found during:** Task 1 (signal and derived signal spec generation)
- **Issue:** All 14 Task 1 files were already committed by plans 01, 02, 04, 05, and 06. Git add/commit returned "nothing added to commit".
- **Fix:** Verified files existed with identical content (zero git diff), documented the overlap, and proceeded to Task 2.
- **Files modified:** None (pre-existing)
- **Verification:** `git log --oneline -- .planning/spec/<file>` confirmed each file had a prior commit
- **Committed in:** N/A (pre-existing)

**2. [Rule 3 - Blocking] Two Task 2 files lost during context compaction**
- **Found during:** Task 2 verification (continuation after context reset)
- **Issue:** `example_props_wrapping_children.md` and `example_props_wrapping_children2.md` were not written to disk before context compaction
- **Fix:** Re-read snapshots and test configs, generated both files from scratch
- **Files modified:** `.planning/spec/example_props_wrapping_children.md`, `.planning/spec/example_props_wrapping_children2.md`
- **Verification:** Both files exist with complete content matching snapshot output
- **Committed in:** 5421f7f (part of Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** No scope creep. Deviation 1 was inherent overlap from parallel plan execution. Deviation 2 was context window limitation handled by regeneration.

## Issues Encountered
- Prior plan executions (02-01 through 02-06) had overlapping file coverage with this plan's Task 1 files. The plans were designed for parallel wave execution but were run sequentially, causing earlier plans to commit files listed in later plans.
- Context compaction during initial execution lost 2 files that needed regeneration in continuation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 27 spec files from this batch are complete and committed
- Signal helper conventions (CONV-04) fully documented across wrapping and non-wrapping scenarios
- Hoisted function patterns (CONV-14) documented with loop, multi-parameter, and parameter ordering edge cases
- Ready for conventions document generation (Phase 3) or remaining plan summaries (04, 05)

## Self-Check: PASSED

All 27 spec files verified present. Commit 5421f7f verified in git log. Summary file exists.

---
*Phase: 02-generate-all-spec-files*
*Completed: 2026-02-10*
