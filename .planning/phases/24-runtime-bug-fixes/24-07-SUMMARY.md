---
phase: 24-runtime-bug-fixes
plan: 07
subsystem: compiler
tags: [oxc, collector, binding-pattern, ts-enum, default-export, self-import]

# Dependency graph
requires:
  - phase: 24-05
    provides: module-level declaration self-import mechanism (reclassify_module_level_decl_captures)
provides:
  - AssignmentPattern handling in binding pattern collection (destructuring defaults)
  - TSEnumDeclaration tracking as module-level declaration
  - Named default export function/class tracking as module-level declarations
  - 5 updated snapshots reflecting correct self-import generation
affects: [24-08, 24-09]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Exhaustive BindingPattern matching (BindingIdentifier, ObjectPattern, ArrayPattern, AssignmentPattern)"
    - "TSEnumDeclaration collection parallel to FunctionDeclaration/ClassDeclaration"

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/tests/snapshots/example_exports.snap
    - crates/qwik-optimizer-oxc/tests/snapshots/example_invalid_references.snap
    - crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums.snap
    - crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums_issue_1341.snap
    - crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums_no_transpile.snap

key-decisions:
  - "Add AssignmentPattern to all three BindingPattern match sites (collector + 2 in transform) for consistency"
  - "Remove unreachable wildcard arms after exhaustive pattern coverage"
  - "Named default export functions/classes added to module_level_decls (not just module_exports)"

patterns-established:
  - "Exhaustive BindingPattern matching: always handle all 4 variants (BindingIdentifier, ObjectPattern, ArrayPattern, AssignmentPattern)"

# Metrics
duration: 6min
completed: 2026-02-12
---

# Phase 24 Plan 07: Declaration Collection Edge Cases Summary

**Fix AssignmentPattern destructuring defaults, TSEnumDeclaration, and named default export collection for self-import generation**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-12T07:14:30Z
- **Completed:** 2026-02-12T07:20:53Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Fixed pattern destructuring defaults (e.g., `I5=v2`, `d=v2`) now collected as module-level declarations via AssignmentPattern handling
- Fixed TypeScript enum declarations (e.g., `export enum Thing`) now tracked as module-level declarations
- Fixed named default exports (e.g., `export default function DefaultFn()`) now added to module_level_decls for self-import generation
- Spec validation improved: 160/162 module count match (was 157), 264/264 metadata match (was 250)
- 5 snapshot files updated with correct self-import statements replacing captures

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix pattern destructuring defaults and TS enum collection** - `c46a8ea` (feat)
2. **Task 2: Update spec tests and snapshot tests for newly-fixed specs** - `e9efbdc` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/collector.rs` - Added AssignmentPattern, TSEnumDeclaration, and default export name handling with 4 new unit tests
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added AssignmentPattern handling in 2 binding pattern match sites, removed unreachable wildcards
- `crates/qwik-optimizer-oxc/tests/snapshots/example_exports.snap` - d, f, DefaultFn now self-imported instead of captured
- `crates/qwik-optimizer-oxc/tests/snapshots/example_invalid_references.snap` - I5, I7 now collected from AssignmentPattern defaults
- `crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums.snap` - TS enum self-import now generated
- `crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums_issue_1341.snap` - TS enum self-import now generated
- `crates/qwik-optimizer-oxc/tests/snapshots/example_ts_enums_no_transpile.snap` - Thing now self-imported in segment

## Decisions Made
- Added AssignmentPattern handling to all three BindingPattern match sites (collector.rs + 2 in transform.rs) for full consistency
- Removed unreachable wildcard `_ => {}` arms after adding AssignmentPattern made pattern matching exhaustive
- Named default export functions/classes are added to `module_level_decls` (enabling self-import generation), not just `module_exports`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Added AssignmentPattern to transform.rs binding pattern matchers**
- **Found during:** Task 1 (collector fixes)
- **Issue:** transform.rs had two additional BindingPattern match sites (`collect_binding_pattern_names` and `collect_binding_names_from_pattern`) that also lacked AssignmentPattern handling
- **Fix:** Added AssignmentPattern arm to both functions in transform.rs
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** All tests pass, no warnings
- **Committed in:** c46a8ea (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Essential for correctness -- same binding pattern bug existed in transform.rs, not just collector.rs. No scope creep.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- 4 of 6 missing-import-used deviations resolved (I5, I7, d, f, DefaultFn, Thing)
- Remaining 2 deviations (JSX import source for example_jsx_import_source) addressed by Plan 24-08
- v4 regression guard passes with improved metrics (160/162 module count, 264/264 metadata)

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*
