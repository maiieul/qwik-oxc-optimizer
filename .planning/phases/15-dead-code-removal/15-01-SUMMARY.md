---
phase: 15-dead-code-removal
plan: 01
subsystem: code-quality
tags: [rust, dead-code, compiler-warnings, refactor]

# Dependency graph
requires:
  - phase: 14-code-cleanup
    provides: Consistently formatted source files as baseline
provides:
  - 8 source files cleaned of unused functions, constants, structs, and variables
  - Zero #![allow(unused)] directives in crate source
  - Reduced compiler warnings from 22 to 9 (types.rs field warnings remain for plan 15-02)
affects: [15-02-PLAN, types.rs field cleanup]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "No #![allow(unused)] directives -- all unused code removed instead of suppressed"

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/errors.rs
    - crates/qwik-optimizer-oxc/src/words.rs
    - crates/qwik-optimizer-oxc/src/entry_strategy.rs
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/code_move.rs
    - crates/qwik-optimizer-oxc/src/emit.rs
    - crates/qwik-optimizer-oxc/src/import_rewrite.rs
    - crates/qwik-optimizer-oxc/src/props_destructuring.rs
    - crates/qwik-optimizer-oxc/src/lib.rs

key-decisions:
  - "Kept array_element_as_expression_mut and argument_as_expression_mut helpers in props_destructuring.rs -- plan incorrectly identified them as unused but they are called by rewrite_props_references"
  - "Updated code_move.rs tests to call build_segment_code_with_hoisted directly instead of through deleted wrapper"

patterns-established:
  - "Dead code removal: delete rather than suppress with allow(unused)"

# Metrics
duration: 5min
completed: 2026-02-11
---

# Phase 15 Plan 01: Dead Code Removal Summary

**Deleted 350 lines of unused functions, constants, structs, and variables across 9 source files, eliminating all #![allow(unused)] directives and reducing compiler warnings from 22 to 9**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-11T20:10:29Z
- **Completed:** 2026-02-11T20:15:33Z
- **Tasks:** 2
- **Files modified:** 9

## Accomplishments
- Removed all #![allow(unused)] directives from errors.rs and entry_strategy.rs
- Eliminated duplicate BUILDER_IO_QWIK/QWIK_CORE_ID constants and 3 unused functions from words.rs
- Deleted build_segment_code wrapper, normalize_code, ImportChanges struct, compute_import_changes, rewrite_array_elements, and rewrite_arguments across 4 files
- Fixed unused imports (SegmentData, SourceLocation, MinifyMode, words) and unused variables (ns, spread)
- All 158 tests pass (7 deleted tests were for removed dead code only)

## Task Commits

Each task was committed atomically:

1. **Task 1: Delete unused functions and constants from errors.rs, words.rs, entry_strategy.rs, collector.rs** - `ce76e86` (refactor)
2. **Task 2: Delete unused functions and structs from code_move.rs, emit.rs, import_rewrite.rs, props_destructuring.rs** - `dc5ac87` (refactor)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/errors.rs` - Removed create_error, create_warning; only create_source_error remains
- `crates/qwik-optimizer-oxc/src/words.rs` - Removed 5 items; only dollar_to_qrl_name and classify_ctx_kind remain
- `crates/qwik-optimizer-oxc/src/entry_strategy.rs` - Removed 3 functions; only should_inline remains
- `crates/qwik-optimizer-oxc/src/collector.rs` - Removed unused words import, prefixed unused ns variable
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Removed build_segment_code wrapper, updated tests
- `crates/qwik-optimizer-oxc/src/emit.rs` - Removed minify field from EmitOptions, deleted normalize_code
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - Removed ImportChanges struct, compute_import_changes, and tests
- `crates/qwik-optimizer-oxc/src/props_destructuring.rs` - Removed rewrite_array_elements, rewrite_arguments; prefixed unused spread
- `crates/qwik-optimizer-oxc/src/lib.rs` - Updated EmitOptions construction to remove minify field

## Decisions Made
- Kept array_element_as_expression_mut and argument_as_expression_mut in props_destructuring.rs (plan said delete, but they are used by rewrite_props_references)
- Updated code_move.rs tests to call build_segment_code_with_hoisted directly (cleaner than keeping unused wrapper just for tests)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Kept array_element_as_expression_mut and argument_as_expression_mut helpers**
- **Found during:** Task 2 (props_destructuring.rs cleanup)
- **Issue:** Plan incorrectly identified these helpers as "only used by deleted rewrite_array_elements/rewrite_arguments", but they are also called by rewrite_props_references at lines 158 and 289
- **Fix:** Kept both helpers intact; only deleted rewrite_array_elements and rewrite_arguments
- **Files modified:** crates/qwik-optimizer-oxc/src/props_destructuring.rs
- **Verification:** cargo build shows no errors; cargo test passes all 158 tests
- **Committed in:** dc5ac87 (Task 2 commit)

**2. [Rule 3 - Blocking] Fixed newly-exposed unused imports after deletions**
- **Found during:** Task 1 (post-deletion build)
- **Issue:** Deleting functions exposed previously-suppressed unused imports: SegmentData in entry_strategy.rs, SourceLocation in errors.rs, MinifyMode in emit.rs
- **Fix:** Removed unused imports from each file
- **Files modified:** crates/qwik-optimizer-oxc/src/entry_strategy.rs, crates/qwik-optimizer-oxc/src/errors.rs
- **Verification:** cargo build shows no new warnings for these files
- **Committed in:** ce76e86 (Task 1 commit) and dc5ac87 (Task 2 commit)

**3. [Rule 3 - Blocking] Updated lib.rs EmitOptions construction**
- **Found during:** Task 2 (emit.rs minify field removal)
- **Issue:** lib.rs constructs EmitOptions with minify field that was removed
- **Fix:** Removed minify field from EmitOptions construction in lib.rs
- **Files modified:** crates/qwik-optimizer-oxc/src/lib.rs
- **Verification:** cargo build succeeds
- **Committed in:** dc5ac87 (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (1 bug in plan, 2 blocking)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Dead code (functions, constants, structs) eliminated across all 8 target files
- Remaining 9 warnings are all types.rs field warnings (plan 15-02 will address these)
- All 158 tests pass (151 unit + 7 spec)

## Self-Check: PASSED

All 10 files verified present. Both commit hashes (ce76e86, dc5ac87) found in git log.

---
*Phase: 15-dead-code-removal*
*Completed: 2026-02-11*
