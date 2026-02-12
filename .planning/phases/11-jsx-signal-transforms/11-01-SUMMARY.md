---
phase: 11-jsx-signal-transforms
plan: 01
subsystem: transform
tags: [jsx, oxc, _jsxSorted, _jsxSplit, Fragment, event-handlers, q-e]

# Dependency graph
requires:
  - phase: 10-segment-extraction-codegen
    provides: "Segment extraction and entry strategy routing for $()-call bodies"
  - phase: 06-secondary-patterns-cross-reference
    provides: "JSX-TRANSFORMS-MAPPING.md reference document with Rust pseudocode"
provides:
  - "JSX element -> _jsxSorted() call transformation with var/const prop classification"
  - "JSX element with spreads -> _jsxSplit() call transformation"
  - "JSX fragment -> _jsxSorted(_Fragment, ...) with aliased import from jsx-runtime"
  - "Event handler attribute renaming (onClick$ -> q-e:click, document:onFocus$ -> q-e:document:focus)"
  - "Children encoding (null, single, array) and flags computation (0=spread, 1=container, 3=leaf)"
  - "Auto-key generation (u6_N pattern) for sibling elements"
  - "build_aliased_import() helper for aliased imports"
affects: [11-02-signal-transforms, jsx-dev-mode]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Recursive JSX transformation via free functions (not Traverse hooks) because JSXChild::Element doesn't trigger walk_expression"
    - "JSXExpression -> Expression exhaustive conversion for all inherit_variants members"
    - "Prop classification: string/numeric/boolean/null/template-no-expressions = const, everything else = var"
    - "Event handler naming: strip on/onDocument/onWindow prefix, lowercase event name, add q-e:/q-e:document:/q-e:window: prefix"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/import_rewrite.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "Recursive free functions for JSX transform instead of Traverse hooks (JSXChild::Element calls walk_jsx_element, not walk_expression)"
  - "Handle JSX in exit_expression for top-level JSXElement/JSXFragment expressions, recurse for children"
  - "Auto-key generation for elements with content/props/children (u6_N pattern); null key for empty self-closing elements"
  - "Event handlers go to const props; non-const attribute values go to var props"
  - "Exhaustive JSXExpression->Expression match covering all 39 inherited variants (no catch-all)"

patterns-established:
  - "JSX recursive transform: transform_jsx_element_inner / transform_jsx_fragment_inner / transform_jsx_children"
  - "ImportTracker JSX flags: needs_jsx_sorted, needs_jsx_split, needs_fragment, needs_get_var_props, needs_get_const_props"

# Metrics
duration: 15min
completed: 2026-02-11
---

# Phase 11 Plan 01: JSX Element Transformation Summary

**JSX elements/fragments converted to _jsxSorted/_jsxSplit calls with var/const prop split, event handler q-e: renaming, and Fragment aliased import from jsx-runtime**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-02-11
- **Completed:** 2026-02-11
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments
- JSX elements become _jsxSorted(tag, varProps, constProps, children, flags, key) calls when transpile_jsx=true
- JSX elements with spread attributes become _jsxSplit() calls using _getVarProps/_getConstProps
- JSX fragments become _jsxSorted(_Fragment, null, null, children, flags, key) with Fragment imported from @qwik.dev/core/jsx-runtime
- Event handler attributes (onClick$, onDocument:scroll$, window:onClick$) renamed to q-e: prefixed const props
- Props correctly classified: string/numeric/boolean/null literals to const, identifiers/member expressions/calls to var
- Children correctly encoded: null (no children), single expression, or array
- All 162 specs transform without errors, 116 unit tests pass (9 new JSX integration tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement JSX element and fragment transformation** - `9c1a706` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/transform.rs` - Added ~730 lines: JSX element/fragment transformation logic with prop classification, event handler renaming, children encoding, flags computation, auto-key generation, and ImportTracker JSX fields
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - Added build_aliased_import() for `import { Fragment as _Fragment } from "@qwik.dev/core/jsx-runtime"`
- `crates/qwik-optimizer-oxc/src/lib.rs` - Added 9 JSX integration tests: basic element, fragment, spread/split, event handler, const props, no-transform flag, key attribute, children encoding, self-closing

## Decisions Made
- **Recursive free functions instead of Traverse hooks:** OXC's `JSXChild::Element` calls `walk_jsx_element` (not `walk_expression`), so `exit_expression` only fires for top-level JSX. Children are recursively transformed within the parent's handler using free functions that take ownership.
- **Exhaustive JSXExpression match:** All 39 inherited Expression variants are explicitly mapped (no catch-all). This prevents silently dropping member expressions, computed properties, etc. into `undefined`.
- **Auto-key generation:** Elements with content (children, props) get auto-generated keys (u6_N); empty self-closing elements get `null` key.
- **Event handler naming convention:** onClick$ -> q-e:click, onDocumentScroll$ -> q-e:document:scroll, window:onClick$ -> q-e:window:click. Host prefix is deprecated and maps to plain q-e: (no prefix).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed JSXExpression to Expression conversion dropping member expressions**
- **Found during:** Task 1 (JSX element transformation)
- **Issue:** The initial `jsx_expression_to_expression` function had a catch-all `_ => undefined` branch that silently converted `StaticMemberExpression` (e.g., `props.id`), `ComputedMemberExpression`, `PrivateFieldExpression`, and 8 other Expression variants to `undefined` instead of preserving them.
- **Fix:** Replaced catch-all with exhaustive match covering all 39 JSXExpression variants (including Super, PrivateInExpression, all 3 MemberExpression variants, and 6 TypeScript expression variants).
- **Files modified:** crates/qwik-optimizer-oxc/src/transform.rs
- **Verification:** `id={props.id}` now correctly outputs `id: props.id` instead of `id: undefined`
- **Committed in:** 9c1a706 (part of task commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Critical correctness fix. Without it, any JSX attribute value using member expressions (props.foo, obj.bar, etc.) would silently become undefined.

## Issues Encountered
None beyond the auto-fixed bug above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- JSX transformation foundation complete, ready for signal optimization (Plan 11-02)
- Signal transforms will build on the prop classification infrastructure to detect signal accesses in JSX attribute values
- All specs continue to transform without errors

## Self-Check: PASSED

- [x] transform.rs exists with JSX transformation code
- [x] import_rewrite.rs exists with build_aliased_import
- [x] lib.rs exists with 9 JSX integration tests
- [x] 11-01-SUMMARY.md exists
- [x] Commit 9c1a706 exists in git log

---
*Phase: 11-jsx-signal-transforms*
*Completed: 2026-02-11*
