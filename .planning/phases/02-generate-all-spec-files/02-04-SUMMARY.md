---
phase: 02-generate-all-spec-files
plan: 04
subsystem: documentation
tags: [qwik-optimizer, spec-files, destructuring, captures, dev-mode, prod-mode, code-stripping, props]

# Dependency graph
requires:
  - phase: 01-oxc-ast-utility
    provides: OXC AST JSON parser for convention detection
  - phase: 02-generate-all-spec-files (plans 01-03)
    provides: Spec file template and convention detection patterns
provides:
  - 27 spec files covering destructuring, capture analysis, dev/prod modes, and code stripping
  - Complete CONV-05 (captures), CONV-09 (code stripping), CONV-11 (props destructuring) documentation
affects: [03-write-optimizer-in-oxc]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - Convention detection via regex pattern matching on output modules
    - Snapshot parsing with YAML frontmatter, ==INPUT==, ===== separators

key-files:
  created:
    - .planning/spec/destructure_args_colon_props.md
    - .planning/spec/destructure_args_colon_props2.md
    - .planning/spec/destructure_args_colon_props3.md
    - .planning/spec/destructure_args_inline_cmp_block_stmt.md
    - .planning/spec/destructure_args_inline_cmp_block_stmt2.md
    - .planning/spec/destructure_args_inline_cmp_expr_stmt.md
    - .planning/spec/example_capture_imports.md
    - .planning/spec/example_capturing_fn_class.md
    - .planning/spec/example_multi_capture.md
    - .planning/spec/should_convert_rest_props.md
    - .planning/spec/should_destructure_args.md
    - .planning/spec/should_move_props_related_to_iteration_variables_to_var_props.md
    - .planning/spec/should_wrap_store_expression.md
    - .planning/spec/should_wrap_type_asserted_variables_in_template.md
    - .planning/spec/should_not_generate_conflicting_props_identifiers.md
    - .planning/spec/should_not_move_over_side_effects.md
    - .planning/spec/example_dev_mode.md
    - .planning/spec/example_dev_mode_inlined.md
    - .planning/spec/example_noop_dev_mode.md
    - .planning/spec/example_prod_node.md
    - .planning/spec/example_server_auth.md
    - .planning/spec/example_dead_code.md
    - .planning/spec/example_drop_side_effects.md
    - .planning/spec/example_strip_client_code.md
    - .planning/spec/example_strip_exports_unused.md
    - .planning/spec/example_strip_exports_used.md
    - .planning/spec/example_strip_server_code.md
  modified: []

key-decisions:
  - "Props destructuring specs (CONV-11) document _rawProps, _restProps, _wrapProp patterns across 8 test cases"
  - "Capture analysis specs (CONV-05) document _captures[] restoration across module boundaries"
  - "Dev/prod mode behavioral differences documented: qrlDEV vs qrl, s_HASH naming, inlinedQrlDEV"
  - "Code stripping specs document _noopQrl, _noopQrlDEV, and 'Symbol removed by Qwik Optimizer' throw patterns"

patterns-established:
  - "CONV-11 detection: _rawProps, _restProps( patterns in function parameters"
  - "CONV-05 detection: _captures[ usage for cross-module variable restoration"
  - "CONV-09 detection: _noopQrl/_noopQrlDEV for stripped code, throw 'Symbol removed' for stripped exports"
  - "Dev mode detection: qrlDEV/inlinedQrlDEV with {file, lo, hi, displayName} metadata"
  - "Prod mode detection: s_HASH naming convention, no dev metadata"

# Metrics
duration: 15min
completed: 2026-02-10
---

# Phase 2 Plan 4: Destructuring, Captures, Dev/Prod Modes, and Code Stripping Spec Summary

**27 spec files documenting props destructuring (CONV-11), capture analysis (CONV-05), dev/prod mode variants, and code stripping transformations (CONV-09/CONV-10)**

## Performance

- **Duration:** 15 min
- **Started:** 2026-02-10T18:31:43Z
- **Completed:** 2026-02-10T18:46:00Z
- **Tasks:** 2
- **Files modified:** 27

## Accomplishments

- Generated 14 spec files for destructuring, captures, and props tests covering CONV-04, CONV-05, CONV-11, CONV-12, and CONV-14
- Generated 13 spec files for dev/prod modes and code stripping covering CONV-01 variants, CONV-09, and CONV-10
- Documented key behavioral differences: dev mode (`qrlDEV`, `inlinedQrlDEV`, source metadata) vs prod mode (`s_HASH` naming, no metadata)
- Documented code stripping patterns: `_noopQrl`/`_noopQrlDEV` for server/client stripping, `throw "Symbol removed by Qwik Optimizer"` for export stripping

## Task Commits

Each task was committed atomically:

1. **Task 1: Generate spec files for destructuring, captures, and props tests** - `e74cfb3` (feat)
2. **Task 2: Generate spec files for dev/prod modes and code stripping tests** - `bacd4ca` (feat)

## Files Created/Modified

### Task 1: Destructuring, Captures, and Props (14 files)
- `.planning/spec/should_wrap_store_expression.md` - CONV-04 (_fnSignal), CONV-14 (hoisted functions with _hf0)
- `.planning/spec/should_wrap_type_asserted_variables_in_template.md` - CONV-04 (_wrapProp sees through type assertions)
- `.planning/spec/destructure_args_colon_props.md` - CONV-11/CONV-12 (bind:value colon-prop destructuring)
- `.planning/spec/destructure_args_colon_props2.md` - CONV-11/CONV-12 (bind:value with useSignal)
- `.planning/spec/destructure_args_colon_props3.md` - CONV-11/CONV-12 (_restProps with bind:value)
- `.planning/spec/destructure_args_inline_cmp_block_stmt.md` - CONV-11 (_rawProps for inline components)
- `.planning/spec/destructure_args_inline_cmp_block_stmt2.md` - CONV-11 (named props, body destructuring removed)
- `.planning/spec/destructure_args_inline_cmp_expr_stmt.md` - CONV-11 (expression-bodied arrow)
- `.planning/spec/example_capture_imports.md` - CONV-05 (CSS import captures via re-import)
- `.planning/spec/example_capturing_fn_class.md` - C02 diagnostics for non-serializable captures
- `.planning/spec/example_multi_capture.md` - CONV-05 (_captures[0] for _rawProps), .jsx output
- `.planning/spec/should_convert_rest_props.md` - CONV-11 (_restProps with no exclusions)
- `.planning/spec/should_destructure_args.md` - CONV-11 (complex: _rawProps, _restProps, _wrapProp, _jsxSplit, _getVarProps, _getConstProps)
- `.planning/spec/should_move_props_related_to_iteration_variables_to_var_props.md` - CONV-04 (_fnSignal for iteration variables)

### Task 2: Dev/Prod Modes and Code Stripping (13 files)
- `.planning/spec/should_not_generate_conflicting_props_identifiers.md` - Hoist strategy, _rawProps vs props naming
- `.planning/spec/should_not_move_over_side_effects.md` - No conventions; side-effect ordering preserved
- `.planning/spec/example_dev_mode.md` - CONV-01 Dev variant (qrlDEV with source metadata)
- `.planning/spec/example_dev_mode_inlined.md` - CONV-01 Dev+Inline variant (inlinedQrlDEV)
- `.planning/spec/example_noop_dev_mode.md` - CONV-09 (_noopQrlDEV), custom dev_path, strip_event_handlers
- `.planning/spec/example_prod_node.md` - Prod mode (s_HASH naming, no dev metadata, .tsx output)
- `.planning/spec/example_server_auth.md` - Third-party $ functions (serverAuth$, auth$ from @auth/qwik)
- `.planning/spec/example_dead_code.md` - Dead code elimination (if(false) removed, deps tree-shaken)
- `.planning/spec/example_drop_side_effects.md` - Side effect dropping (server$ stripped, IIFEs preserved)
- `.planning/spec/example_strip_client_code.md` - Client code stripping (Inline + strip_event_handlers + strip_ctx_name)
- `.planning/spec/example_strip_exports_unused.md` - Export stripping (throw "Symbol removed")
- `.planning/spec/example_strip_exports_used.md` - Export stripping with cross-segment reference
- `.planning/spec/example_strip_server_code.md` - Server code stripping (Prod + strip_ctx_name, nested $ preserved)

## Decisions Made

- Props destructuring specs (CONV-11) document `_rawProps`, `_restProps`, `_wrapProp` patterns across 8 test cases
- Capture analysis specs (CONV-05) document `_captures[]` restoration across module boundaries
- Dev/prod mode behavioral differences documented: `qrlDEV` vs `qrl`, `s_HASH` naming, `inlinedQrlDEV`
- Code stripping specs document `_noopQrl`, `_noopQrlDEV`, and `throw "Symbol removed by Qwik Optimizer"` patterns

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- Parallel Write tool calls for Task 2 resulted in some files not being persisted to disk despite success responses. Detected via post-write verification and re-created the 8 missing files in smaller batches. All 27 files verified present before commit.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- 27 spec files from this plan complete the destructuring, captures, dev/prod, and code stripping test documentation
- Combined with plans 01-03 and 05-06, provides comprehensive spec coverage for Phase 3 (OXC optimizer implementation)
- Key convention patterns (CONV-05, CONV-09, CONV-10, CONV-11) now fully documented with input/output examples

## Self-Check: PASSED

- 27/27 spec files: FOUND
- Commit e74cfb3 (Task 1): FOUND
- Commit bacd4ca (Task 2): FOUND
- Summary file 02-04-SUMMARY.md: FOUND

---
*Phase: 02-generate-all-spec-files*
*Plan: 04*
*Completed: 2026-02-10*
