---
phase: 04-core-api-mapping-architecture
plan: 01
subsystem: api-mapping
tags: [oxc, traverse, astbuilder, qrl, rust, transformation]

# Dependency graph
requires:
  - phase: 01-03 (spec generation)
    provides: "162 spec files with input/output code pairs and segment metadata"
  - phase: 04-RESEARCH
    provides: "OXC API patterns, AstBuilder methods, Traverse trait usage"
provides:
  - "OXC API mapping guide for $-extraction, QRL wrapping, and import rewriting"
  - "Complete Rust code examples for 8 core functions (enter_call_expression, is_dollar_call, build_qrl_call, build_inlined_qrl_call, build_lazy_import_declaration, build_named_import, exit_program import management, dollar_to_qrl_name)"
  - "Before/after code comparisons from 3 spec files (example_1, example_functional_component, example_inlined_entry_strategy)"
affects: [04-02-architecture, phase-05-deep-research, phase-06-secondary-patterns]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Traverse enter_*/exit_* pattern for AST detection and mutation"
    - "AstBuilder (ctx.ast) for constructing replacement expressions"
    - "exit_program hook for deferred import insertion"
    - "take_in(ctx.ast) for moving AST node ownership"

key-files:
  created:
    - ".planning/phases/04-core-api-mapping-architecture/API-MAPPING.md"
  modified: []

key-decisions:
  - "Document both PURE annotation strategies (expression_call_with_pure vs manual comment) as open question"
  - "Import rewriting collects during traversal, builds in exit_program"
  - "Each Qrl-suffixed import gets its own import declaration (matches spec output pattern)"

patterns-established:
  - "Pattern 1: $-extraction via enter_call_expression with DollarCallKind enum"
  - "Pattern 2: QRL wrapping via build_qrl_call/build_inlined_qrl_call builder functions"
  - "Pattern 3: Import management via collect-then-build in exit_program"

# Metrics
duration: 4min
completed: 2026-02-10
---

# Phase 4 Plan 1: OXC API Mapping Guide Summary

**Comprehensive OXC API mapping for $-extraction, QRL wrapping, and import rewriting with complete Rust code examples referencing 3 spec files**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-10T22:43:08Z
- **Completed:** 2026-02-10T22:47:15Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Created API-MAPPING.md covering all three foundational transformation patterns (APIM-01, APIM-02, APIM-05)
- Documented 8 core Rust functions with complete type annotations and use imports
- Provided before/after code comparisons quoted from example_1.md, example_functional_component.md, and example_inlined_entry_strategy.md
- Documented both segment and inline QRL wrapping strategies with spec-derived examples
- Documented import rewriting rules with ordering, including _captures and Qrl-suffixed imports
- Covered cross-cutting concerns: complete OXC pipeline, arena allocator lifetime rules, take_in pattern

## Task Commits

Each task was committed atomically:

1. **Task 1: Create OXC API Mapping Guide** - `44da4cc` (feat)

## Files Created/Modified

- `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md` - Comprehensive OXC API mapping guide for three foundational transformation patterns (1078 lines)

## Decisions Made

- **PURE annotation strategy:** Documented both `expression_call_with_pure` and manual comment attachment as options, recommending Option A with fallback to Option B. Left as open question per research findings.
- **Import management pattern:** Collect needed imports during traversal via ImportTracker struct, build all import declarations in exit_program hook. This matches the OXC anti-pattern guidance (do not modify program.body during traversal).
- **Import declaration granularity:** Each Qrl-suffixed function gets its own `import { X } from "@qwik.dev/core"` declaration rather than combining into a single import. This matches the observed spec file output pattern.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- API-MAPPING.md provides the implementation reference for all three foundational patterns
- Plan 02 (architecture) can now reference API-MAPPING.md for the module layout, public API types, data flow, and test harness design
- The 8 documented Rust functions form the core of the QwikTransform implementation

## Self-Check: PASSED

- [x] API-MAPPING.md exists at `.planning/phases/04-core-api-mapping-architecture/API-MAPPING.md`
- [x] Task commit `44da4cc` exists in git log
- [x] SUMMARY file exists

---
*Phase: 04-core-api-mapping-architecture*
*Completed: 2026-02-10*
