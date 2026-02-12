---
phase: 08-core-detection-qrl-transforms
plan: 02
subsystem: transform
tags: [oxc, traverse, qrl, dollar-detection, import-rewrite, codegen]

# Dependency graph
requires:
  - phase: 08-01
    provides: "parse.rs, collector.rs, hash.rs, words.rs for AST parsing and dollar call collection"
provides:
  - "QwikTransform Traverse implementation with dollar detection and QRL wrapping"
  - "import_rewrite.rs with AST builder functions (build_qrl_call, build_inlined_qrl_call, build_named_import, build_lazy_import_declaration)"
  - "Fully wired transform_modules() pipeline returning real TransformOutput"
  - "emit.rs with OXC Codegen-based code generation"
affects: [09-code-move, 10-capture-analysis, 11-filter-strip, 12-minification, 13-source-maps]

# Tech tracking
tech-stack:
  added: [oxc_traverse, oxc::codegen::Codegen]
  patterns: [Traverse trait implementation, arena string allocation via ctx.ast.atom(), exit_expression for AST replacement, swap pattern for OXC Vec mutation]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/import_rewrite.rs
    - crates/qwik-optimizer-oxc/src/entry_strategy.rs
    - crates/qwik-optimizer-oxc/src/emit.rs
    - crates/qwik-optimizer-oxc/src/lib.rs
    - crates/qwik-optimizer-oxc/tests/spec_tests.rs

key-decisions:
  - "Use exit_expression (not exit_call_expression) for QRL replacement -- gives mutable &mut Expression for full replacement"
  - "Arena string allocation via ctx.ast.atom() for all runtime-constructed strings passed to AstBuilder"
  - "Use std::mem::swap with ctx.ast.vec() instead of std::mem::take for OXC Vec (no Default impl)"
  - "Pending dollar calls tracked by span.start u32 in HashSet for O(1) lookup in exit_expression"
  - "expression_call_with_pure for PURE annotations on qrl/inlinedQrl calls"

patterns-established:
  - "Traverse<'a, ()>: State type is () for QwikTransform (no shared mutable state needed)"
  - "Arena lifetime pattern: ctx.ast.atom(&string) to allocate runtime strings into 'a arena"
  - "Two-phase detection: enter_call_expression records, exit_expression replaces (avoids borrow conflicts)"
  - "allocator.alloc_str() to place source code into arena for tied lifetimes with Program"

# Metrics
duration: 12min
completed: 2026-02-10
---

# Phase 8 Plan 2: QwikTransform Traverse + Pipeline Wiring Summary

**QwikTransform with OXC Traverse for dollar detection and QRL wrapping, plus fully wired transform_modules() pipeline producing real JavaScript output**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-02-10
- **Completed:** 2026-02-10
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- QwikTransform implements `Traverse<'a, ()>` with enter_call_expression for dollar detection, exit_expression for QRL replacement, and exit_program for import insertion
- Segment strategy: `$(() => ...)` becomes `qrl(i_HASH, "name_HASH")` with lazy import constants
- Inline strategy: `$(() => ...)` becomes `inlinedQrl(() => body, "name_HASH")`
- Named calls: `component$(() => ...)` becomes `componentQrl(qrl(...))` or `componentQrl(inlinedQrl(...))`
- Import rewriting adds qrl/inlinedQrl/nameQrl imports from @qwik.dev/core
- Full pipeline wired: parse -> collect -> QwikTransform -> traverse_mut -> Codegen emit
- 63 total tests pass (60 unit + 3 spec tests)

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement QwikTransform Traverse and import_rewrite.rs** - `b02541b` (feat)
2. **Task 2: Wire transform_modules() pipeline and integration tests** - `435a803` (feat)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/src/transform.rs` - QwikTransform with Traverse<'a, ()> implementation: dollar detection, QRL wrapping, import insertion
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - AST builder functions: build_qrl_call, build_inlined_qrl_call, build_named_import, build_lazy_import_declaration, compute_import_changes
- `crates/qwik-optimizer-oxc/src/entry_strategy.rs` - Added should_hoist() and needs_separate_file() helpers
- `crates/qwik-optimizer-oxc/src/emit.rs` - Real code generation using OXC Codegen with source text
- `crates/qwik-optimizer-oxc/src/lib.rs` - Full transform_modules() pipeline with TransformOptions construction, per-input processing loop, segment module building, and 11 integration tests
- `crates/qwik-optimizer-oxc/tests/spec_tests.rs` - Updated test_transform_stub to test_transform_produces_output (verifies real output)

## Decisions Made

1. **exit_expression over exit_call_expression**: The Traverse trait's exit_call_expression only provides `&mut CallExpression`, which cannot be replaced with an arbitrary Expression. Using exit_expression gives `&mut Expression` allowing full replacement via `*expr = new_expr`.

2. **Arena string allocation via ctx.ast.atom()**: OXC AstBuilder methods require strings with arena lifetime 'a. Runtime-constructed strings (like `qrl_name`, `ident_name`) must be allocated into the arena via `ctx.ast.atom(&string)` before use.

3. **std::mem::swap for OXC Vec**: OXC's `oxc::allocator::Vec` does not implement `Default`, so `std::mem::take` cannot be used. Instead, create a new empty vec with `ctx.ast.vec()` and swap.

4. **Pending dollar calls via span.start HashSet**: Track detected dollar calls by their `CallExpression.span.start` value in a `HashSet<u32>` for O(1) lookup in exit_expression.

5. **expression_call_with_pure for PURE annotations**: OXC provides `expression_call_with_pure()` which adds `/*#__PURE__*/` comments to qrl/inlinedQrl calls for tree-shaking.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed OXC API lifetime requirements for string parameters**
- **Found during:** Task 1
- **Issue:** AstBuilder methods like `expression_identifier`, `expression_string_literal`, `binding_pattern_binding_identifier` require strings with arena lifetime 'a. Passing `&str` references to local variables caused E0621/E0597 lifetime errors.
- **Fix:** Used `ctx.ast.atom(&string_value)` to allocate all runtime-constructed strings into the arena before passing to AstBuilder methods. Applied across all functions in import_rewrite.rs and transform.rs.
- **Files modified:** transform.rs, import_rewrite.rs
- **Verification:** `cargo build` succeeds
- **Committed in:** b02541b

**2. [Rule 1 - Bug] Fixed OXC Vec Default trait absence**
- **Found during:** Task 1
- **Issue:** `std::mem::take(&mut program.body)` fails because OXC's `oxc::allocator::Vec` does not implement the `Default` trait.
- **Fix:** Used `let mut old_body = ctx.ast.vec(); std::mem::swap(&mut program.body, &mut old_body);` pattern to extract the body without requiring Default.
- **Files modified:** transform.rs
- **Verification:** `cargo build` succeeds
- **Committed in:** b02541b

**3. [Rule 3 - Blocking] Updated spec_tests test_transform_stub for real pipeline output**
- **Found during:** Task 2
- **Issue:** The existing `test_transform_stub` test asserted that transform_modules returns empty modules, which is no longer true now that the real pipeline is implemented.
- **Fix:** Renamed test to `test_transform_produces_output` and updated assertions to verify the pipeline returns non-empty modules with actual code.
- **Files modified:** tests/spec_tests.rs
- **Verification:** All 63 tests pass
- **Committed in:** 435a803

---

**Total deviations:** 3 auto-fixed (2 bugs, 1 blocking)
**Impact on plan:** All auto-fixes necessary for correctness with OXC 0.113 API. No scope creep.

## Issues Encountered

None beyond the auto-fixed deviations above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 8 complete: the optimizer can now transform Qwik source code with dollar calls into QRL-wrapped output
- Ready for Phase 9 (code_move.rs) to generate segment module code
- Ready for Phase 10 (capture analysis) to detect captured variables in closures
- Segment modules currently have placeholder empty code (Phase 9 will generate actual segment code)
- Source maps deferred to Phase 13 (emit returns None for map)
- Minification deferred to Phase 12 (EmitOptions ignored for now)

## Self-Check: PASSED

- All 7 claimed files: FOUND
- Commit b02541b (Task 1): FOUND
- Commit 435a803 (Task 2): FOUND
- All 63 tests pass (60 unit + 3 spec)

---
*Phase: 08-core-detection-qrl-transforms*
*Completed: 2026-02-10*
