---
phase: 02-generate-all-spec-files
plan: 02
subsystem: spec-generation
tags: [jsx, qrl, event-handling, bind, spread-props, convention-detection, oxc-ast]

# Dependency graph
requires:
  - phase: 01-oxc-ast-utility
    provides: "oxc-ast-util binary for AST JSON generation"
provides:
  - "27 spec files covering QRL extraction, JSX transforms, event handling, spread props, and input binding"
  - "gen-spec.py helper script for automated snapshot-to-spec generation"
affects: [02-generate-all-spec-files, 03-verification]

# Tech tracking
tech-stack:
  added: [gen-spec.py (Python 3 snapshot parser + spec generator)]
  patterns: [automated snapshot parsing, convention detection via regex pattern matching, OXC AST generation via subprocess]

key-files:
  created:
    - .planning/spec/should_extract_single_qrl.md
    - .planning/spec/should_extract_single_qrl_2.md
    - .planning/spec/should_extract_single_qrl_with_index.md
    - .planning/spec/should_extract_single_qrl_with_nested_components.md
    - .planning/spec/should_transform_component_with_normal_function.md
    - .planning/spec/should_transform_qrls_in_ternary_expression.md
    - .planning/spec/example_jsx.md
    - .planning/spec/example_jsx_import_source.md
    - .planning/spec/example_jsx_keyed.md
    - .planning/spec/example_jsx_keyed_dev.md
    - .planning/spec/example_jsx_listeners.md
    - .planning/spec/example_spread_jsx.md
    - .planning/spec/example_input_bind.md
    - .planning/spec/example_class_name.md
    - .planning/spec/example_mutable_children.md
    - .planning/spec/special_jsx.md
    - .planning/spec/should_convert_jsx_events.md
    - .planning/spec/should_handle_dangerously_set_inner_html.md
    - .planning/spec/should_merge_attributes_with_spread_props.md
    - .planning/spec/should_merge_attributes_with_spread_props_before_and_after.md
    - .planning/spec/should_merge_bind_checked_and_on_input.md
    - .planning/spec/should_merge_bind_value_and_on_input.md
    - .planning/spec/should_merge_on_input_and_bind_checked.md
    - .planning/spec/should_merge_on_input_and_bind_value.md
    - .planning/spec/should_move_bind_value_to_var_props.md
    - .planning/spec/should_not_transform_bind_checked_in_var_props_for_jsx_split.md
    - .planning/spec/should_not_transform_bind_value_in_var_props_for_jsx_split.md
    - .planning/tools/gen-spec.py
  modified: []

key-decisions:
  - "Created gen-spec.py Python helper for automated snapshot parsing, AST generation, and convention detection rather than manual per-file processing"
  - "Convention detection uses regex patterns on output module code, detecting all 14 CONV types"
  - "special_jsx correctly shows zero conventions since JSX is not transpiled (defaults)"
  - "example_jsx_import_source correctly omits CONV-03 since React JSX runtime handles transforms, not Qwik"

patterns-established:
  - "Snapshot-to-spec pipeline: parse .snap -> extract test config from test.rs -> generate OXC ASTs -> detect conventions -> write markdown"
  - "Convention detection via 14 regex patterns across all output modules (not just main module)"
  - "AST blocks in collapsible <details> elements per SPEC-04/SPEC-06"

# Metrics
duration: 5min
completed: 2026-02-10
---

# Phase 2 Plan 2: JSX, QRL Extraction, Event Handling, and Bind Spec Files Summary

**27 spec files generated for QRL extraction (4), JSX transforms (8), event handling (2), spread props merge (2), input binding (8), and edge cases (3) with automated convention detection via gen-spec.py**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-10T18:32:00Z
- **Completed:** 2026-02-10T18:37:46Z
- **Tasks:** 2
- **Files created:** 28 (27 spec files + 1 helper script)

## Accomplishments
- Generated 27 structured spec files from insta snapshot files, each with test config, input/output code, OXC AST in collapsible blocks, convention detection, function call tables, and diagnostics
- Built gen-spec.py helper that automates the entire snapshot-to-spec pipeline (parsing, AST generation, convention detection, markdown writing)
- Convention detection correctly identifies all applicable conventions across all output modules (not just main module)
- Verified JSX-focused specs detect CONV-03, bind-focused specs detect CONV-12, and edge cases (special_jsx, example_jsx_import_source) correctly report no/different conventions

## Task Commits

Each task was committed atomically:

1. **Task 1: Generate 14 spec files for QRL extraction and JSX transforms** - `dffad07` (feat)
2. **Task 2: Generate 13 spec files for event handling, spread, and bind tests** - `495c11a` (feat)

## Files Created/Modified
- `.planning/spec/should_extract_single_qrl.md` - QRL extraction with hoisted functions and captured variables
- `.planning/spec/should_extract_single_qrl_2.md` - QRL extraction with multiple event handlers in loops
- `.planning/spec/should_extract_single_qrl_with_index.md` - QRL extraction capturing loop index variable
- `.planning/spec/should_extract_single_qrl_with_nested_components.md` - QRL extraction in nested component definitions
- `.planning/spec/should_transform_component_with_normal_function.md` - Function declarations (not arrows) in component$
- `.planning/spec/should_transform_qrls_in_ternary_expression.md` - QRL extraction from ternary conditional expressions
- `.planning/spec/example_jsx.md` - Core JSX transform with _jsxSorted and _jsxSplit
- `.planning/spec/example_jsx_import_source.md` - React JSX pragma handling (non-Qwik JSX)
- `.planning/spec/example_jsx_keyed.md` - JSX with key props
- `.planning/spec/example_jsx_keyed_dev.md` - Dev mode JSX with qrlDEV and custom filename
- `.planning/spec/example_jsx_listeners.md` - Event listener JSX attributes (14 handler segments)
- `.planning/spec/example_spread_jsx.md` - Spread props in JSX with _jsxSorted
- `.planning/spec/example_input_bind.md` - Input binding (Inline strategy, Prod mode) with _val/_chk
- `.planning/spec/example_class_name.md` - className to class conversion with signal helpers
- `.planning/spec/example_mutable_children.md` - Mutable children detection (Hoist strategy)
- `.planning/spec/special_jsx.md` - Edge case: jsx() call without transformation
- `.planning/spec/should_convert_jsx_events.md` - Event name conversion in JSX
- `.planning/spec/should_handle_dangerously_set_inner_html.md` - dangerouslySetInnerHTML prop handling
- `.planning/spec/should_merge_attributes_with_spread_props.md` - Spread props merged with static attributes
- `.planning/spec/should_merge_attributes_with_spread_props_before_and_after.md` - Multiple spread positions
- `.planning/spec/should_merge_bind_checked_and_on_input.md` - bind:checked + onInput$ merge
- `.planning/spec/should_merge_bind_value_and_on_input.md` - bind:value + onInput$ merge
- `.planning/spec/should_merge_on_input_and_bind_checked.md` - onInput$ + bind:checked (reversed order)
- `.planning/spec/should_merge_on_input_and_bind_value.md` - onInput$ + bind:value (reversed order)
- `.planning/spec/should_move_bind_value_to_var_props.md` - bind:value moved to var props with spread
- `.planning/spec/should_not_transform_bind_checked_in_var_props_for_jsx_split.md` - Negative test: bind:checked in var props
- `.planning/spec/should_not_transform_bind_value_in_var_props_for_jsx_split.md` - Negative test: bind:value in var props
- `.planning/tools/gen-spec.py` - Automated snapshot-to-spec generation script

## Decisions Made
- **Created gen-spec.py helper script:** Rather than manually processing each snapshot (which would be error-prone for 27 files), built a Python script that handles snapshot parsing, AST generation via subprocess, convention detection via 14 regex patterns, and structured markdown output. This script is reusable for remaining plan batches.
- **Convention detection covers all output modules:** Patterns are searched across ALL output modules (not just the main module), ensuring conventions that appear only in entry point segments (like _captures, lazy imports) are detected.
- **No false positives on edge cases:** `special_jsx` (no transforms) and `example_jsx_import_source` (React JSX, not Qwik) correctly report minimal/no Qwik conventions.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Created gen-spec.py helper script for batch processing**
- **Found during:** Task 1 (first spec file generation)
- **Issue:** Manually processing 27 snapshot files with inline snapshot parsing, AST generation, convention detection, and markdown generation would be extremely slow and error-prone
- **Fix:** Created `.planning/tools/gen-spec.py` that automates the entire pipeline: snapshot parsing, test config injection, OXC AST generation via subprocess, 14-pattern convention detection, and structured markdown output
- **Files created:** .planning/tools/gen-spec.py
- **Verification:** All 27 specs generated with correct conventions, headings, and AST blocks
- **Committed in:** dffad07 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Essential for efficient batch processing. The script is reusable for remaining plan batches in Phase 2.

## Issues Encountered
None - all snapshots parsed correctly, all ASTs generated without errors, all conventions detected accurately.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- 27 spec files ready for Phase 3 cross-checking and verification
- gen-spec.py helper available for remaining Phase 2 plans (02-03 through 02-06)
- Convention detection patterns validated against edge cases

## Self-Check: PASSED

All claimed artifacts verified:
- 27 spec files: ALL FOUND
- gen-spec.py helper: FOUND
- Commit dffad07 (Task 1): FOUND
- Commit 495c11a (Task 2): FOUND

---
*Phase: 02-generate-all-spec-files*
*Completed: 2026-02-10*
