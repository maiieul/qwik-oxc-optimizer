---
phase: 02-generate-all-spec-files
plan: 05
subsystem: documentation
tags: [qwik-optimizer, spec-files, transpile, enums, exports, filenames, integrations, renaming]

# Dependency graph
requires:
  - phase: 01-oxc-ast-utility
    provides: OXC AST JSON generation binary for spec file AST sections
  - phase: 02-generate-all-spec-files (plans 01-04)
    provides: Established spec file template and convention detection patterns
provides:
  - 27 spec files covering transpile options, TypeScript enums, exports/imports, filename preservation, registered context names, and third-party integrations
affects: [03-synthesize-conventions]

# Tech tracking
tech-stack:
  added: []
  patterns: [import-assertion-transform, preserve-filenames, reg-ctx-name, strip-event-handlers, legacy-path-renaming]

key-files:
  created:
    - .planning/spec/example_transpile_jsx_only.md
    - .planning/spec/example_transpile_ts_only.md
    - .planning/spec/example_explicit_ext_no_transpile.md
    - .planning/spec/example_explicit_ext_transpile.md
    - .planning/spec/example_ts_enums.md
    - .planning/spec/example_ts_enums_issue_1341.md
    - .planning/spec/example_ts_enums_no_transpile.md
    - .planning/spec/example_skip_transform.md
    - .planning/spec/example_component_with_event_listeners_inside_loop.md
    - .planning/spec/example_default_export.md
    - .planning/spec/example_default_export_index.md
    - .planning/spec/example_default_export_invalid_ident.md
    - .planning/spec/example_export_issue.md
    - .planning/spec/example_exports.md
    - .planning/spec/example_renamed_exports.md
    - .planning/spec/example_fix_dynamic_import.md
    - .planning/spec/example_import_assertion.md
    - .planning/spec/example_preserve_filenames.md
    - .planning/spec/example_preserve_filenames_segments.md
    - .planning/spec/example_reg_ctx_name_segments.md
    - .planning/spec/example_reg_ctx_name_segments_hoisted.md
    - .planning/spec/example_reg_ctx_name_segments_inlined.md
    - .planning/spec/example_qwik_conflict.md
    - .planning/spec/example_qwik_react.md
    - .planning/spec/example_qwik_react_inline.md
    - .planning/spec/example_qwik_router_inline.md
    - .planning/spec/rename_builder_io.md
  modified: []

key-decisions:
  - "AST sections use structural descriptions rather than full JSON output for manageability"
  - "Convention detection covers all 14 CONV types with pattern matching on output code"
  - "Import assertion assert-to-with keyword modernization documented as its own pattern"

patterns-established:
  - "Preserve filenames: import paths use original filename when preserve_filenames is true"
  - "Registered context names: server$ produces _regSymbol + serverQrl wrapping pattern"
  - "Code stripping: strip_event_handlers replaces non-server handlers with _noopQrl"
  - "Legacy path renaming: @builder.io/qwik -> @qwik.dev/core, @builder.io/qwik-city -> @qwik.dev/router"
  - "Naming conflict resolution: optimizer renames user identifiers that conflict with injected names (e.g., componentQrl1)"

# Metrics
duration: 8min
completed: 2026-02-10
---

# Phase 2 Plan 5: Transpile, Enum, Export, Filename, and Integration Spec Files Summary

**27 spec files documenting transpile flag effects, TypeScript enum handling, export patterns, filename preservation, server registration, naming conflicts, Qwik React/Router integrations, and legacy path renaming**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-10T20:45:00Z
- **Completed:** 2026-02-10T20:55:52Z
- **Tasks:** 2
- **Files created:** 27

## Accomplishments
- 14 spec files for transpile options (jsx-only, ts-only, both, neither), TypeScript enums (exported, non-exported, non-transpiled), skip transform, default exports (standard, index, invalid ident), export edge cases, and event listeners in loops
- 13 spec files for renamed exports, dynamic imports, import assertions, filename preservation (inline + segment), registered context names (3 strategies), naming conflicts, Qwik React (segment + inline), Qwik Router (inline), and legacy path renaming
- All 27 files verified to contain all required template sections

## Task Commits

Each task was committed atomically:

1. **Task 1: Generate spec files for transpile, enum, and export tests (14 specs)** - `b8aeba9` (feat)
2. **Task 2: Generate spec files for filenames, segments, integrations, and renaming tests (13 specs)** - `c904988` (feat)

## Files Created/Modified
- `.planning/spec/example_transpile_jsx_only.md` - JSX-only transpile: output .ts extension, JSX transpiled but TS preserved
- `.planning/spec/example_transpile_ts_only.md` - TS-only transpile: Inline strategy, output .jsx, JSX preserved
- `.planning/spec/example_explicit_ext_no_transpile.md` - Explicit extensions with no transpile: .tsx output with import extensions
- `.planning/spec/example_explicit_ext_transpile.md` - Explicit extensions with both transpile: .js output with .js import extensions
- `.planning/spec/example_ts_enums.md` - Exported enum -> IIFE, enum values constant-folded in segments
- `.planning/spec/example_ts_enums_issue_1341.md` - Non-exported enum -> IIFE expression (not assigned)
- `.planning/spec/example_ts_enums_no_transpile.md` - Enum preserved as-is when transpile_ts: false
- `.planning/spec/example_skip_transform.md` - Aliased imports prevent optimizer recognition
- `.planning/spec/example_component_with_event_listeners_inside_loop.md` - 6 event handlers from loop patterns, _captures, _fnSignal, _hf0
- `.planning/spec/example_default_export.md` - Smart strategy, custom filename with route params
- `.planning/spec/example_default_export_index.md` - Inline strategy, index.tsx named from parent directory
- `.planning/spec/example_default_export_invalid_ident.md` - 404.tsx prefixed with _ for valid identifier
- `.planning/spec/example_export_issue.md` - Non-exported component, re-export alias, _auto_ prefix
- `.planning/spec/example_exports.md` - Comprehensive export patterns: destructured, named, function, class, default
- `.planning/spec/example_renamed_exports.md` - Aliased imports (component$ as Component), _wrapProp for reactive store
- `.planning/spec/example_fix_dynamic_import.md` - Dynamic import() preserved in both main module and segments
- `.planning/spec/example_import_assertion.md` - assert keyword converted to with keyword (modernization)
- `.planning/spec/example_preserve_filenames.md` - Inline strategy: preserve_filenames has minimal visible effect
- `.planning/spec/example_preserve_filenames_segments.md` - Segment strategy: import paths use original filename
- `.planning/spec/example_reg_ctx_name_segments.md` - server$ with _regSymbol, _noopQrl for stripped handlers
- `.planning/spec/example_reg_ctx_name_segments_hoisted.md` - Hoist strategy: segments as top-level vars
- `.planning/spec/example_reg_ctx_name_segments_inlined.md` - Inline strategy with server$ registration
- `.planning/spec/example_qwik_conflict.md` - Naming conflicts: user's componentQrl -> componentQrl1
- `.planning/spec/example_qwik_react.md` - Qwik React integration: pre-compiled input, _auto_filterProps alias
- `.planning/spec/example_qwik_react_inline.md` - Same input, Inline strategy: all code stays inline
- `.planning/spec/example_qwik_router_inline.md` - Full Qwik Router (~1074 lines), Smart+Lib mode
- `.planning/spec/rename_builder_io.md` - Legacy @builder.io/* to @qwik.dev/* path renaming

## Decisions Made
- AST sections use structural descriptions rather than full JSON output (descriptions capture the essential structure while keeping files manageable)
- Convention detection covers all 14 CONV types systematically via pattern matching on output code
- Import assertion `assert` to `with` keyword modernization documented as a distinct transformation pattern

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Recreated 6 missing Task 2 spec files after context reset**
- **Found during:** Task 2 (continuation after context compaction)
- **Issue:** 6 of 13 Task 2 files were written in previous context but not persisted (example_renamed_exports, example_fix_dynamic_import, example_import_assertion, example_preserve_filenames, example_preserve_filenames_segments, example_reg_ctx_name_segments)
- **Fix:** Re-read snapshot files and test configs, regenerated all 6 files
- **Files modified:** 6 spec files in .planning/spec/
- **Verification:** All 13 Task 2 files confirmed present
- **Committed in:** c904988 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking -- context continuation issue)
**Impact on plan:** Auto-fix necessary to complete task after context reset. No scope creep.

## Issues Encountered
- Context compaction during execution caused 6 files to need recreation. Re-read snapshots and test configs to regenerate accurately.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- All 27 spec files from this plan complete
- Combined with plans 01-04 and 06, all spec files for the entire phase should now be generated
- Ready for Phase 3: Synthesize Conventions (aggregate patterns across all spec files)

---
*Phase: 02-generate-all-spec-files*
*Completed: 2026-02-10*
