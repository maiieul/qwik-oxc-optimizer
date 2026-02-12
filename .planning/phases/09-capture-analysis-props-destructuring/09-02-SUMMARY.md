---
phase: 09-capture-analysis-props-destructuring
plan: 02
subsystem: compiler
tags: [oxc, capture-analysis, qrl, scoping, traverse]

# Dependency graph
requires:
  - phase: 09-01
    provides: "Props destructuring transformation (active_props_info, _rawProps rewrite)"
  - phase: 08-02
    provides: "QwikTransform Traverse pipeline with enter/exit hooks, SegmentData, import_rewrite builders"
  - phase: 05
    provides: "CAPTURE-ANALYSIS-MAPPING.md algorithm specification"
provides:
  - "compute_captures() function classifying outer references as captures, imports, or globals"
  - "Stack-based capture tracking in QwikTransform for nested $()-bodies"
  - "qrl()/inlinedQrl() calls include captures array as third argument"
  - "SegmentAnalysis.capture_names metadata for captured variable names"
  - "Props destructuring post-processing: child segment captures use _rawProps instead of individual prop names"
affects: [10-code-move, 11-segment-extraction]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Stack-based capture tracking (Vec of frames) for nested $()-bodies"
    - "Post-processing child segment captures during component$ exit for props destructuring interaction"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/import_rewrite.rs"
    - "crates/qwik-optimizer-oxc/src/types.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "Simplified string-based capture analysis instead of full OXC Scoping API (traverse_mut consumes Scoping)"
  - "Stack-based capture_stack Vec instead of flat fields for correct nested $()-body tracking"
  - "Post-process child segment captures in component$ exit to replace prop aliases with _rawProps"
  - "Only take() active_props_info for component$ exits, not inner $() exits"

patterns-established:
  - "Stack-based capture tracking: push frame on $()-enter, pop on $()-exit, each frame tracks (ident_refs, local_decls)"
  - "Post-processing pattern: modify child segment metadata during parent exit when parent has additional context (props info)"
  - "Traverse hooks enter_identifier_reference and enter_variable_declarator for collecting scope data during traversal"

# Metrics
duration: 18min
completed: 2026-02-11
---

# Phase 9 Plan 02: Capture Analysis Summary

**Stack-based capture analysis detecting cross-boundary variable references in $()-bodies with props destructuring integration and qrl()/inlinedQrl() captures array emission**

## Performance

- **Duration:** ~18 min
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Implemented `compute_captures()` in collector.rs classifying outer references as LocalCapture, ImportReemit, or globals
- Wired capture analysis into QwikTransform with stack-based tracking for nested $()-bodies
- Updated `build_qrl_call()` to accept and emit captures array as third argument
- Added props destructuring post-processing: child segment captures use `_rawProps` instead of individual prop names
- Added `capture_names` field to SegmentAnalysis with `skip_serializing_if` for clean JSON output
- Added 14 new tests (7 unit + 7 integration) covering all capture classification paths

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement compute_captures() in collector.rs** - `58e4c69` (feat)
2. **Task 2: Wire capture analysis into QwikTransform and integration tests** - `740f1cc` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/collector.rs` - Added CaptureAnalysisResult struct, KNOWN_GLOBALS, compute_captures() function, and 7 unit tests
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added capture_stack, enter_identifier_reference/enter_variable_declarator hooks, capture computation in exit_expression, props destructuring post-processing
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - Updated build_qrl_call() to accept captures parameter and emit captures array
- `crates/qwik-optimizer-oxc/src/types.rs` - Added capture_names: Option<Vec<String>> to SegmentAnalysis
- `crates/qwik-optimizer-oxc/src/lib.rs` - Updated segment_data_to_analysis() mapping, added 7 integration tests

## Decisions Made
- **Simplified capture analysis:** Used string-based approach (collect ident names during Traverse, classify after exit) instead of full OXC Scoping API because traverse_mut consumes the Scoping object. This is sufficient for Phase 9 and can be upgraded later.
- **Stack-based tracking:** Changed from flat fields (body_ident_refs, body_local_decls) to a stack (capture_stack: Vec<(Vec<String>, HashSet<String>)>) because nested $()-bodies need independent tracking frames.
- **Props capture post-processing:** Inner $() exits before component$ exit, so capture analysis runs before props destructuring rewrites the AST. Solution: post-process child segment captures during component$ exit to replace individual prop aliases (e.g., "foo") with "_rawProps".
- **active_props_info consumption order:** Only `.take()` active_props_info for component$ exits, not inner $() exits. Previously the inner $() would consume it leaving nothing for the component$.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed flat capture tracking sharing state between nested $()-bodies**
- **Found during:** Task 2 (QwikTransform wiring)
- **Issue:** Flat fields `body_ident_refs` and `body_local_decls` were shared between nested $()-bodies, causing incorrect capture analysis for inner bodies
- **Fix:** Replaced flat fields with stack-based `capture_stack: Vec<(Vec<String>, HashSet<String>)>` where each frame tracks one $()-body independently
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** test_capture_state_variable_inline passes
- **Committed in:** 740f1cc (Task 2 commit)

**2. [Rule 1 - Bug] Fixed active_props_info consumed by inner $() exit instead of component$ exit**
- **Found during:** Task 2 (QwikTransform wiring)
- **Issue:** `self.active_props_info.take()` was called unconditionally for any $()-call exit, so inner $() consumed the props info intended for the component$ exit
- **Fix:** Guard with `is_component_exit` check: only take for component$ exits
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** test_capture_rawprops_segment passes
- **Committed in:** 740f1cc (Task 2 commit)

**3. [Rule 1 - Bug] Fixed FormalParameterRest field access for OXC 0.113**
- **Found during:** Task 2 (collecting arrow param names)
- **Issue:** `rest.argument` doesn't exist; OXC 0.113 wraps BindingRestElement in a `.rest` field
- **Fix:** Use `rest.rest.argument` for FormalParameterRest in both FormalParameters and ObjectPattern/ArrayPattern
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** Build compiles, all tests pass
- **Committed in:** 740f1cc (Task 2 commit)

**4. [Rule 1 - Bug] Fixed incorrect test assertion for segment strategy**
- **Found during:** Task 2 (integration tests)
- **Issue:** Test checked `main_code.contains("_rawProps")` but in segment strategy the component body (containing the inner qrl with captures) is extracted into a separate segment file, not the main module
- **Fix:** Changed assertion to check `componentQrl` presence in main module (segment metadata assertions were already correct)
- **Files modified:** crates/qwik-optimizer-oxc/src/lib.rs
- **Verification:** test_capture_rawprops_segment passes with correct segment metadata
- **Committed in:** 740f1cc (Task 2 commit)

---

**Total deviations:** 4 auto-fixed (4 bugs)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep. Plan's simplified approach worked well.

## Issues Encountered
- OXC 0.113 FormalParameterRest wraps BindingRestElement in `.rest` field (consistent with earlier discovery in Phase 9 Plan 01)
- Nested $()-body capture analysis requires careful ordering: inner exits before outer, so post-processing of child segment metadata must happen during the outer (component$) exit

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Capture analysis complete and verified for both inline and segment strategies
- Props destructuring + capture analysis interaction working correctly (_rawProps post-processing)
- Ready for Phase 10 (code_move) which will generate segment body code using capture_names
- Ready for Phase 11 (segment extraction) which will use capture metadata for segment splitting
- All 92 tests passing (89 unit + 3 integration spec tests)

---
*Phase: 09-capture-analysis-props-destructuring*
*Completed: 2026-02-11*
