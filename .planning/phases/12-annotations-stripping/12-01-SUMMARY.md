---
phase: 12-annotations-stripping
plan: 01
subsystem: transform
tags: [pure-annotations, const-replacement, dead-code-elimination, tree-shaking, oxc]

# Dependency graph
requires:
  - phase: 11-jsx-advanced
    provides: "JSX transform with _jsxSorted/_jsxSplit call construction"
provides:
  - "PURE annotations on all _jsxSorted/_jsxSplit calls for tree-shaking"
  - "Build constant replacement (isServer/isBrowser/isDev -> boolean literals)"
  - "Dead branch elimination after const replacement"
  - "Logical expression simplification (false && x -> false, true && x -> x)"
  - "Build constant import stripping"
affects: [13-final-integration, spec-tests]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Pre-pass AST transformation using standalone AstBuilder (no TraverseCtx)"
    - "Recursive manual walk for identifier replacement with export declaration handling"
    - "Two-pass dead branch elimination: collect actions, rebuild statement vec"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/const_replace.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "const_replace runs as pre-pass before traverse_mut so segment body serialization sees replaced values"
  - "expression_call_with_pure for PURE annotations on _jsxSorted/_jsxSplit (same pattern as qrl/inlinedQrl)"
  - "Build constant imports stripped after replacement to match spec output"
  - "Standalone AstBuilder::new(&allocator) for AST construction outside traverse context"
  - "block.unbox().body.into_iter() pattern for taking ownership of boxed OXC AST nodes"

patterns-established:
  - "Pre-pass transformation: parse -> collect -> const_replace -> traverse_mut -> emit"
  - "Export declaration recursion: Statement::ExportNamedDeclaration wraps Declaration, must be handled explicitly"

# Metrics
duration: 15min
completed: 2026-02-11
---

# Phase 12 Plan 01: PURE Annotations & Const Replacement Summary

**PURE annotations on _jsxSorted/_jsxSplit for tree-shaking, isServer/isBrowser/isDev const replacement with dead branch elimination**

## Performance

- **Duration:** ~15 min (across multiple sessions due to context resets)
- **Started:** 2026-02-11
- **Completed:** 2026-02-11
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- All _jsxSorted and _jsxSplit calls now emit /*#__PURE__*/ annotations for bundler tree-shaking
- Build constants (isServer, isBrowser, isDev) from @qwik.dev/core and @qwik.dev/core/build are replaced with boolean literals based on build configuration
- Dead branches (if(false){...}) are eliminated and true branches are inlined after const replacement
- Logical expressions simplified: false && x -> false, true && x -> x, true || x -> true, false || x -> x
- Import declarations for build constants are stripped from output
- Aliased imports (e.g., import { isServer as myServer }) correctly handled

## Task Commits

Each task was committed atomically:

1. **Task 1: Add PURE annotations to _jsxSorted and _jsxSplit calls** - `c8a83bd` (test)
2. **Task 2: Implement const replacement and dead branch elimination** - `6deec99` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/transform.rs` - expression_call_with_pure on _jsxSorted/_jsxSplit calls
- `crates/qwik-optimizer-oxc/src/const_replace.rs` - Full implementation: identifier replacement, logical simplification, dead branch elimination, import stripping
- `crates/qwik-optimizer-oxc/src/lib.rs` - Integration of const_replace pre-pass + 11 new tests (4 PURE + 7 const replacement)

## Decisions Made
- const_replace runs BEFORE traverse_mut (pre-pass) so that segment body serialization during traverse sees replaced boolean literals instead of original identifiers
- Used AstBuilder::new(&allocator) for AST construction since const_replace runs outside traverse context (no TraverseCtx available)
- Build constant import declarations stripped after replacement to produce clean output matching spec expectations
- OXC Box::unbox() used to take ownership of boxed AST nodes for dead branch elimination

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added build constant import stripping**
- **Found during:** Task 2 (const replacement integration testing)
- **Issue:** After replacing identifiers with boolean literals, the original import declarations (e.g., `import { isServer } from "@qwik.dev/core/build"`) remained in output, causing test failures and not matching spec behavior
- **Fix:** Added `strip_build_constant_imports()` function that removes replaced specifiers from import declarations and removes the entire import if no specifiers remain
- **Files modified:** crates/qwik-optimizer-oxc/src/const_replace.rs
- **Verification:** All 146 unit tests + 5 spec tests pass
- **Committed in:** 6deec99 (Task 2 commit)

**2. [Rule 1 - Bug] Added export declaration handling in recursive walk**
- **Found during:** Task 2 (const replacement integration testing)
- **Issue:** `replace_identifiers_in_statement` did not handle `Statement::ExportNamedDeclaration` or `Statement::ExportDefaultDeclaration`, causing identifiers inside exported declarations to not be replaced
- **Fix:** Added match arms for export declaration variants with new `replace_identifiers_in_declaration` helper, plus corresponding recursion in dead branch elimination and expression simplification
- **Files modified:** crates/qwik-optimizer-oxc/src/const_replace.rs
- **Verification:** All const replacement tests pass
- **Committed in:** 6deec99 (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (1 missing critical, 1 bug)
**Impact on plan:** Both fixes essential for correctness. Import stripping needed for spec compliance. Export handling needed for the recursive walk to reach identifiers in real-world code patterns. No scope creep.

## Issues Encountered
- OXC 0.113 inherit_variants! pattern requires handling Statement::ExportNamedDeclaration as a direct Statement variant (not nested under ModuleDeclaration), which was not initially apparent
- OXC Box::into_inner does not exist; Box::unbox(self) is the correct method for taking ownership

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- PURE annotations and const replacement complete, ready for Phase 13 (Final Integration)
- All 146 unit tests and 5 spec tests pass
- Pipeline order established: parse -> collect -> const_replace -> traverse_mut -> finalize_segments -> emit

## Self-Check: PASSED

- FOUND: crates/qwik-optimizer-oxc/src/const_replace.rs
- FOUND: crates/qwik-optimizer-oxc/src/transform.rs
- FOUND: crates/qwik-optimizer-oxc/src/lib.rs
- FOUND: .planning/phases/12-annotations-stripping/12-01-SUMMARY.md
- FOUND: c8a83bd (Task 1 commit)
- FOUND: 6deec99 (Task 2 commit)
- All 146 unit tests pass
- All 5 spec tests pass

---
*Phase: 12-annotations-stripping*
*Completed: 2026-02-11*
