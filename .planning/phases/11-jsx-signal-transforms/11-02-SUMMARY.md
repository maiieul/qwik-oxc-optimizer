---
phase: 11-jsx-signal-transforms
plan: 02
subsystem: transform
tags: [oxc, jsx, signals, fnSignal, wrapProp, bind, reactive]

# Dependency graph
requires:
  - phase: 11-01
    provides: "JSX element/fragment transformation with _jsxSorted, prop classification, children handling"
provides:
  - "_wrapProp signal wrapping for .value access and named props"
  - "_fnSignal computed expression wrapping with hoisted _hfN arrow functions"
  - "is_const_expression for prop var/const classification"
  - "bind:value and bind:checked input binding transforms"
  - "Reactive dependency detection (collect_reactive_deps)"
affects: [phase-12, verification]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "String-based hoisted function insertion (hoisted stmts as code strings injected after imports)"
    - "Reactive dependency analysis via AST walking (collect_reactive_deps)"
    - "Word-boundary-aware identifier replacement for hoisted function body construction"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/import_rewrite.rs"
    - "crates/qwik-optimizer-oxc/src/is_const.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"
    - "crates/qwik-optimizer-oxc/src/code_move.rs"
    - "crates/qwik-optimizer-oxc/src/collector.rs"

key-decisions:
  - "String-based hoisted function approach: store _hfN declarations as code strings and inject into codegen output after imports, rather than building AST nodes (avoids allocator boundary issues between OXC parse allocators)"
  - "Reactive dependency detection uses AST analysis: .value access = signal dep, multi-level property chains = store dep, _rawProps access = props dep, imported identifiers = not reactive"
  - "Signal-wrapped values go into const props (not var), matching Qwik runtime semantics where _wrapProp/_fnSignal handle reactivity tracking"
  - "Segment strategy modules get automatic import detection based on body code content (scans for _jsxSorted, _fnSignal, _wrapProp, _Fragment references)"

patterns-established:
  - "Hoisted function pattern: collect (fn_code, str_code) tuples during JSX transform, insert at module top level in lib.rs or code_move.rs"
  - "Module imports threading: pass module_imports and hoisted_stmts through all JSX transform function signatures"
  - "Bind directive pattern: detect bind:value/bind:checked namespace attrs, expand to value const prop + q-e:input event handler QRL"

# Metrics
duration: 45min
completed: 2025-02-11
---

# Phase 11 Plan 02: Signal & Binding Transforms Summary

**_wrapProp signal wrapping, _fnSignal computed expression hoisting with reactive dependency detection, and bind:value/bind:checked input binding expansion**

## Performance

- **Duration:** ~45 min
- **Tasks:** 3/3
- **Files modified:** 6

## Accomplishments
- signal.value in JSX props becomes _wrapProp(signal), _rawProps.propName becomes _wrapProp(_rawProps, "propName")
- Computed expressions like signal.value + 1 become _fnSignal(_hfN, [signal], _hfN_str) with hoisted const declarations
- bind:value produces value const prop + q-e:input with _val QRL handler; bind:checked uses _chk
- Reactive dependency detection correctly identifies signals (.value), stores (deep chains), props (_rawProps), and excludes imports/globals
- All 162 specs transform without errors, 127 unit tests + 5 spec tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement is_const_expression and _wrapProp signal wrapping** - `772bf0f` (feat)
2. **Task 2: Implement _fnSignal computed expressions with hoisted helpers** - `81d3d6b` (feat)
3. **Task 3: Implement bind:value and bind:checked input binding transforms** - `6e63734` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/is_const.rs` - is_const_expression() for compile-time constant detection in JSX prop classification
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - build_wrap_prop_call() and build_wrap_prop_call_named() for _wrapProp AST construction
- `crates/qwik-optimizer-oxc/src/transform.rs` - Signal wrapping detection (detect_signal_wrap, SignalWrapResult), reactive dep collection (collect_reactive_deps), _fnSignal building (build_fn_signal_wrapping), bind: directive handling, hoisted function storage
- `crates/qwik-optimizer-oxc/src/lib.rs` - Hoisted function code injection after imports in main module, segment module hoisted stmts passthrough, 10 new integration tests
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Segment-specific import detection, hoisted function insertion in segment modules
- `crates/qwik-optimizer-oxc/src/collector.rs` - Made KNOWN_GLOBALS pub(crate) for reactive dep analysis

## Decisions Made
- Used string-based approach for hoisted _hfN declarations instead of AST-level insertion, because OXC allocator boundaries prevent transferring AST nodes between different parse contexts
- Signal-wrapped and _fnSignal-wrapped values go to const props (not var) since the wrapping itself handles reactivity tracking
- Non-wrapping rules: expressions with function calls are never wrapped with _fnSignal (go to var props); mixed reactive + non-reactive non-const expressions also skip wrapping
- Segment strategy modules detect needed imports by scanning body code for framework helper references

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] props_destructuring argument rewriting stub**
- **Found during:** Task 1
- **Issue:** argument_as_expression_mut() was a stub returning None, preventing _rawProps member expression rewriting in _jsxSorted call arguments
- **Fix:** Added rewrite_call_arguments() function that handles Argument variants (SpreadElement, Identifier, compound) via argument_to_expression conversion
- **Files modified:** props_destructuring.rs
- **Committed in:** 772bf0f (Task 1)

**2. [Rule 1 - Bug] WrapPropNamed failing for destructured prop identifiers**
- **Found during:** Task 1
- **Issue:** Code tried to match Expression::StaticMemberExpression for destructured prop values, but they're Expression::Identifier
- **Fix:** Added fallback to build _rawProps identifier when value is not a member expression
- **Files modified:** transform.rs
- **Committed in:** 772bf0f (Task 1)

**3. [Rule 2 - Missing Critical] Segment module imports for JSX helpers**
- **Found during:** Task 2
- **Issue:** Segment strategy modules lacked imports for _jsxSorted, _fnSignal, _wrapProp, _Fragment that are referenced in the serialized body code
- **Fix:** Added body code scanning in code_move to detect and add needed segment imports
- **Files modified:** code_move.rs
- **Committed in:** 81d3d6b (Task 2)

---

**Total deviations:** 3 auto-fixed (1 bug, 1 blocking, 1 missing critical)
**Impact on plan:** All auto-fixes necessary for correctness. No scope creep.

## Issues Encountered
- OXC allocator boundary prevents direct AST node transfer between parse contexts -- solved via string-based hoisted function insertion
- Borrow checker conflict between self.hoisted_function_stmts and self.import_tracker in Traverse impl -- solved by std::mem::take/restore pattern

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- JSX signal transformation pipeline is complete for all CONV-04, CONV-12, CONV-14 conventions
- Ready for verification phase to validate output matching against spec expectations
- Remaining gap: _fnSignal string representation minification is simplified (strips whitespace only, doesn't handle all edge cases)

---
*Phase: 11-jsx-signal-transforms*
*Completed: 2025-02-11*
