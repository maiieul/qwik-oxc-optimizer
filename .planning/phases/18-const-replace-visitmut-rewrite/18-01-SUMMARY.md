---
phase: 18-const-replace-visitmut-rewrite
plan: 01
subsystem: optimizer
tags: [oxc, visit-mut, ast-traversal, dead-branch-elimination, refactor]

requires:
  - phase: 14-code-quality
    provides: "Clean codebase with consistent formatting and import grouping"
provides:
  - "VisitMut-based const_replace.rs with 2 trait impls replacing 16 manual walker functions"
  - "ast_visit feature enabled in oxc dependency"
affects: [const_replace, build-constants, dead-branch-elimination]

tech-stack:
  added: ["oxc::ast_visit::VisitMut"]
  patterns: ["VisitMut trait for AST pre-pass transformations"]

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/const_replace.rs"
    - "crates/qwik-optimizer-oxc/Cargo.toml"

key-decisions:
  - "Used two separate VisitMut impls (ConstReplacer + DeadBranchEliminator) for two-pass approach"
  - "Bottom-up traversal in DeadBranchEliminator (walk children first, then simplify)"

patterns-established:
  - "VisitMut pattern: override visit_expression for expression-level transforms, visit_statements for statement-level transforms"
  - "Non-recursive helpers: extract logic into standalone functions that operate on single nodes, let VisitMut handle recursion"

duration: 4min
completed: 2026-02-11
---

# Phase 18 Plan 01: const_replace VisitMut Rewrite Summary

**Replaced 16 manual recursive walker functions with 2 VisitMut trait impls, reducing const_replace.rs from 860 to 275 lines (68% reduction)**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-11T21:34:37Z
- **Completed:** 2026-02-11T21:39:34Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments
- Rewrote const_replace.rs to use OXC's VisitMut trait for automatic AST traversal
- Eliminated 585 lines of manual match-and-recurse boilerplate (860 -> 275 lines)
- ConstReplacer: single visit_expression override handles all identifier replacement across every AST context (statements, arguments, array elements, JSX, declarations)
- DeadBranchEliminator: visit_expression + visit_statements for bottom-up logical simplification and dead if-statement elimination
- All 154 unit tests + 7 spec tests pass identically with zero behavior change

## Task Commits

Each task was committed atomically:

1. **Task 1: Enable ast_visit feature and rewrite const_replace.rs with VisitMut** - `d4adf30` (refactor)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/Cargo.toml` - Added ast_visit feature to oxc dependency
- `crates/qwik-optimizer-oxc/src/const_replace.rs` - Complete rewrite from 16 manual walker functions to 2 VisitMut trait impls

## Decisions Made
- Used two separate VisitMut impls rather than one combined visitor -- keeps pass 1 (replacement) and pass 2 (elimination) cleanly separated, matching original two-phase approach
- Bottom-up traversal in DeadBranchEliminator: walk_expression before simplify_logical_expression ensures inner expressions are simplified before outer ones are evaluated
- Kept eval_boolean_value, simplify_logical_expression, eliminate_dead_if_statements as standalone non-recursive helper functions -- VisitMut handles the recursion, helpers handle the domain logic

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- const_replace.rs now uses VisitMut pattern which could serve as a template for other AST pre-passes
- Phase 19 (if any) can proceed without blockers

---
*Phase: 18-const-replace-visitmut-rewrite*
*Completed: 2026-02-11*
