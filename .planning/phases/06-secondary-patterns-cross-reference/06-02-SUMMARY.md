---
phase: 06-secondary-patterns-cross-reference
plan: 02
subsystem: api-mapping
tags: [oxc, entry-strategy, code-stripping, const-folding, input-binding, sync-qrl, dev-mode, astbuilder]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture
    provides: "build_qrl_call(), build_inlined_qrl_call(), build_lazy_import_declaration(), build_named_import() from API-MAPPING.md"
  - phase: 05-deep-research-proof-of-concept
    provides: "Capture analysis algorithm (compute_captures), multi-module output pipeline"
provides:
  - "Complete OXC API mapping for all 7 entry strategy variants (Segment, Inline, Hoist, Smart, Component, Hook, Single)"
  - "Code stripping implementation spec: _noopQrl/_noopQrlDEV construction, nested $() preservation, side effect analysis, export stripping"
  - "Const folding implementation spec: isServer replacement, dead branch elimination, static expression evaluator (try_eval_const_expr)"
  - "Input binding (CONV-12) transformation: bind:value/_val, bind:checked/_chk, bind:* passthrough"
  - "Sync$ serialization (CONV-13): _qrlSync construction with oxc_codegen stringification"
  - "Dev mode patterns: qrlDEV, _noopQrlDEV, JSX devInfo object construction"
  - "CONV dependency ordering: CONV-10 before CONV-09"
  - "Complete import requirements table for all secondary patterns"
affects: [06-03-cross-reference-source-maps]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Configuration-driven QRL branching: strategy determines qrl() vs inlinedQrl(), not different APIs"
    - "Dev/Prod branching for all QRL constructs: qrl/qrlDEV, _noopQrl/_noopQrlDEV"
    - "Side effect analysis via oxc_semantic reference tracking after stripping"
    - "Mini constant folder: try_eval_const_expr() for compile-time expression evaluation"
    - "Sync$ stringification via oxc_codegen with minification"

key-files:
  created:
    - ".planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md"
  modified: []

key-decisions:
  - "Entry strategy is pure configuration-driven branching of two existing API patterns (qrl vs inlinedQrl) -- no new AstBuilder patterns needed"
  - "CONV-10 (const replacement) must run before CONV-09 (code stripping) to enable dead branch elimination prior to noop replacement"
  - "Side effect analysis uses post-stripping reference counting via oxc_semantic -- declarations with zero live references after stripping are removed"
  - "Static expression evaluator scoped to literals and simple operations only -- no function calls or complex expressions"

patterns-established:
  - "Dev/Prod branching pattern: every QRL-constructing function takes is_dev flag, selects DEV variant with debug metadata object"
  - "Noop replacement pattern: stripped $-calls become Qrl-suffixed wrapper around _noopQrl(hash)"
  - "Preserved side-effect pattern: IIFEs and standalone calls are never removed during dead code elimination"

# Metrics
duration: 6min
completed: 2026-02-11
---

# Phase 6 Plan 2: Entry Strategy, Code Stripping, and Const Folding Summary

**Complete OXC API mapping for 7 entry strategy variants, _noopQrl stripping with side effect analysis, isServer const folding with dead branch elimination, bind: directives, sync$ serialization, and dev mode patterns -- 18 sections with 36 Rust code examples**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-11T00:23:20Z
- **Completed:** 2026-02-11T00:29:42Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Mapped all 7 entry strategy variants to OXC API calls with configuration-driven branching logic showing that strategies differ only in qrl() vs inlinedQrl() selection
- Documented complete code stripping pipeline: _noopQrl construction, nested $() preservation rules, side effect analysis algorithm using oxc_semantic, and export stripping with throw replacement
- Documented const folding pipeline: isServer boolean replacement, dead branch elimination, and a complete try_eval_const_expr() function handling strings, numbers, booleans, template literals, typeof, ternary, and arithmetic
- Documented input binding (bind:value, bind:checked, bind:*) and sync$ serialization (_qrlSync with stringification) with full OXC construction code
- Documented all 3 dev mode patterns (qrlDEV, _noopQrlDEV, JSX devInfo) with debug metadata object construction
- Established CONV-10 before CONV-09 dependency ordering with rationale

## Task Commits

Each task was committed atomically:

1. **Task 1: Document entry strategy variants and dev mode patterns with OXC APIs** - `fa1e290` (feat)
2. **Task 2: Add code stripping and const folding sections** - `5dd1ca6` (feat)

**Plan metadata:** pending (docs: complete plan)

## Files Created/Modified
- `.planning/phases/06-secondary-patterns-cross-reference/ENTRY-STRIPPING-CONST-MAPPING.md` - Complete OXC API mapping for entry strategies, code stripping, const folding, input binding, sync$, and dev mode (1689 lines, 18 sections, 36 Rust code blocks)

## Decisions Made
- Entry strategy is configuration-driven branching, not different API patterns -- simplifies implementation to a single match on strategy enum
- CONV-10 (isServer replacement) must precede CONV-09 (code stripping) because const evaluation creates dead branches that stripping benefits from
- Side effect analysis after stripping uses oxc_semantic reference counting -- symbols with zero live references (all refs in stripped scopes) are removed
- Static expression evaluator deliberately scoped to literals and simple operations -- function calls and complex expressions return None (not evaluated)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- ENTRY-STRIPPING-CONST-MAPPING.md provides the authoritative API mapping for CONV-09, CONV-10, CONV-12, CONV-13
- Ready for Plan 06-03 (Cross-Reference Table + Source Map Integration) which will compile all CONV types into a single cross-reference and document span strategies for all node types
- All 14 CONV types now have OXC API documentation across Phase 4 (CONV-01, -02, -06), Phase 5 (CONV-05, -08), Phase 6 Plan 1 (CONV-03, -04, -07, -11, -14), and this plan (CONV-09, -10, -12, -13)

## Self-Check: PASSED

- [x] ENTRY-STRIPPING-CONST-MAPPING.md exists (1689 lines)
- [x] 06-02-SUMMARY.md exists
- [x] Commit fa1e290 exists (Task 1)
- [x] Commit 5dd1ca6 exists (Task 2)

---
*Phase: 06-secondary-patterns-cross-reference*
*Completed: 2026-02-11*
