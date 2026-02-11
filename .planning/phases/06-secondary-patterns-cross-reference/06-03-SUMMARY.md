---
phase: 06-secondary-patterns-cross-reference
plan: 03
subsystem: documentation
tags: [oxc, source-maps, pure-annotations, cross-reference, conv-types, span-strategy]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture
    provides: CONV-01, CONV-02, CONV-06 foundational API patterns
  - phase: 05-deep-research-proof-of-concept
    provides: CONV-05 capture analysis, CONV-08 segment extraction, POC-04 span validation
  - phase: 06-secondary-patterns-cross-reference (plans 01-02)
    provides: CONV-03/04/09/10/11/12/13/14 mapping documents
provides:
  - Complete 14 CONV type cross-reference with OXC detection/construction APIs (APIM-08)
  - Source map span strategy for all 50 node types across Phase 4/5/6 (APIM-07)
  - PURE annotation mechanism with two implementation options (CONV-07)
  - Dependency ordering graph and implementation roadmap for v3.0
affects: [v3.0-implementation, oxc-port]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Span preservation: preserve original spans on extracted/moved nodes, SPAN (zero) on constructed nodes"
    - "PURE annotation: two-option strategy (OXC built-in preferred, manual comment fallback)"
    - "Source map per module: each segment and main module gets its own .map file"
    - "Dependency-ordered CONV execution: CONV-11 -> CONV-05 -> CONV-08, CONV-10 -> CONV-09"

key-files:
  created:
    - .planning/phases/06-secondary-patterns-cross-reference/CONV-CROSS-REFERENCE.md
    - .planning/phases/06-secondary-patterns-cross-reference/SOURCE-MAPS-MAPPING.md
  modified: []

key-decisions:
  - "PURE annotation has two options: Option A (OXC built-in, preferred) and Option B (manual comment attachment via Program.comments); open question deferred to v3.0 implementation"
  - "50 node types cataloged across 3 span categories: 42 constructed (SPAN zero), 7 preserved (original span), 1 removed (N/A)"
  - "Implementation roadmap orders 14 CONVs into 9 tiers based on dependency graph and frequency analysis"

patterns-established:
  - "Cross-reference lookup: any CONV type can be found by number with detection API, construction API, spec files, frequency, and document link"
  - "Span decision guide: for any node the optimizer constructs or moves, the span strategy is documented in the span table"

# Metrics
duration: 10min
completed: 2026-02-11
---

# Phase 6 Plan 3: Cross-Reference & Source Maps Summary

**Complete 14 CONV type cross-reference with OXC APIs, 50-entry span strategy table, and PURE annotation mechanism with two implementation options**

## Performance

- **Duration:** 10 min
- **Started:** 2026-02-11T00:33:53Z
- **Completed:** 2026-02-11T00:43:53Z
- **Tasks:** 2
- **Files created:** 2

## Accomplishments
- Compiled all 14 CONV types into a single cross-reference document with detection API, construction API, Rust code snippets, frequency analysis, dependency ordering graph, and implementation roadmap
- Created complete span strategy table covering 50 node types across Phase 4/5/6 with clear preserve-vs-zero rationale for each
- Documented PURE annotation mechanism (CONV-07) with two implementation options and complete annotated-calls list
- Documented 4-step segment source map pipeline with Rust code at each step
- Completed APIM-07 and APIM-08 requirements, finishing Phase 6 and the v2.0 milestone

## Task Commits

Each task was committed atomically:

1. **Task 1: Compile the 14 CONV type cross-reference table (APIM-08)** - `83f5809` (feat)
2. **Task 2: Create source map span strategy and PURE annotation mapping (APIM-07)** - `aba5888` (feat)

## Files Created
- `.planning/phases/06-secondary-patterns-cross-reference/CONV-CROSS-REFERENCE.md` - Master index for all 14 CONV types with OXC detection/construction APIs, frequency analysis, dependency ordering, phase coverage, and implementation roadmap (621 lines)
- `.planning/phases/06-secondary-patterns-cross-reference/SOURCE-MAPS-MAPPING.md` - Source map span strategy for all 50 node types, PURE annotation mechanism (two options), segment/main module source map pipelines, and file naming conventions (531 lines)

## Decisions Made
- PURE annotation has two options: Option A (OXC built-in, preferred) and Option B (manual comment attachment via Program.comments); resolution deferred to v3.0 implementation as originally noted in Phase 4 (04-01)
- 50 node types cataloged across 3 span categories: 42 constructed (SPAN zero), 7 preserved (original span), 1 removed (N/A)
- Implementation roadmap orders 14 CONVs into 9 tiers based on dependency constraints (CONV-11 before CONV-05, CONV-10 before CONV-09) and frequency (universal CONVs first, specialized last)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 6 is complete: all 3 plans executed, all APIM requirements (APIM-04, APIM-07, APIM-08, APIM-09, APIM-10) satisfied
- v2.0 milestone is complete: all 17 plans across 6 phases executed
- Ready for v2.0 milestone audit via `/gsd:audit-milestone`

## Self-Check: PASSED

- FOUND: CONV-CROSS-REFERENCE.md
- FOUND: SOURCE-MAPS-MAPPING.md
- FOUND: 06-03-SUMMARY.md
- FOUND: commit 83f5809
- FOUND: commit aba5888

---
*Phase: 06-secondary-patterns-cross-reference*
*Completed: 2026-02-11*
