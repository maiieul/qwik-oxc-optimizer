---
phase: 13-source-maps-full-validation
plan: 02
subsystem: testing
tags: [diagnostics, import-alias, core-module, collector, spec-tests]

# Dependency graph
requires:
  - phase: 13-01
    provides: "Source map wiring for spec transforms"
provides:
  - "Diagnostic test categorizing all 83 module count mismatches by failure type"
  - "Import alias detection in collector (local_name -> original_imported_name mapping)"
  - "Broad Qwik module recognition (@builder.io/qwik, @qwik.dev/*, custom core_module)"
  - "Alias-aware $-call detection in QwikTransform"
affects: [13-03, 13-04]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "alias_map HashMap<String,String> in CollectResult for import alias tracking"
    - "CollectContext::is_qwik_core_import() for configurable module matching"
    - "Collector accepts optional core_module parameter for custom framework packages"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/tests/spec_tests.rs"
    - "crates/qwik-optimizer-oxc/src/collector.rs"
    - "crates/qwik-optimizer-oxc/src/types.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "Broadened Qwik module recognition to @qwik.dev/*, @builder.io/qwik-*, excluding sub-paths like /core/build"
  - "alias_map only populated when local_name != imported_name (avoids overhead for non-aliased imports)"
  - "Collector resolves aliases at call-site detection time (not transform time) for consistent display names"
  - "Module count improvement modest (78->79) because 77 of 83 remaining mismatches are JSX event handler extraction, not alias/core_module"

patterns-established:
  - "Diagnostic categorization test: non-asserting test with --nocapture for failure landscape analysis"
  - "Alias resolution at collector level: original imported names propagate to callee_name and display_name"

# Metrics
duration: 8min
completed: 2026-02-11
---

# Phase 13 Plan 02: Diagnostic Categorization & Import Alias/Core Module Fixes Summary

**Diagnostic test categorizing 83 module count mismatches; import alias detection and @builder.io/qwik legacy import recognition added to collector**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-11T17:40:01Z
- **Completed:** 2026-02-11T17:48:26Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Diagnostic test (`test_diagnose_module_count_mismatches`) categorizes all mismatches by failure type: alias, core_module, strip_exports, strip_ctx_name, reg_ctx_name, transpile_only, diagnostics_expected, other
- Import alias detection: `alias_map` in `CollectResult` maps `Component -> component$`, `onRender -> $` etc. Both collector and transform resolve aliases for correct callee_name, ctx_name, and Qrl import names
- Broad Qwik module recognition: `@builder.io/qwik`, `@builder.io/qwik-react`, `@qwik.dev/react`, and custom core_module all recognized for `$`-suffixed import detection
- Module count match improved from 78 to 79 out of 162 (rename_builder_io fixed)
- 8 new unit tests for alias/core_module handling, all passing
- 158 unit tests + 6 spec tests pass with zero regressions

## Task Commits

1. **Task 1: Build diagnostic test for module count mismatches** - `2b09a4b` (feat)
2. **Task 2: Fix import alias detection and core_module handling** - `3f581b6` (feat)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/tests/spec_tests.rs` - Added diagnostic categorization test + updated comments
- `crates/qwik-optimizer-oxc/src/types.rs` - Added `alias_map: HashMap<String, String>` to CollectResult
- `crates/qwik-optimizer-oxc/src/collector.rs` - Alias tracking in collect_import, core_module parameter, broadened Qwik module matching, 8 new tests
- `crates/qwik-optimizer-oxc/src/transform.rs` - Alias resolution in is_dollar_call
- `crates/qwik-optimizer-oxc/src/lib.rs` - Passes config.core_module to collector

## Decisions Made

- **Broad module matching**: Instead of only matching exact core_module string, the collector now recognizes any `@qwik.dev/*` package and `@builder.io/qwik-*` legacy packages. Sub-paths like `@qwik.dev/core/build` are excluded (they don't export $-APIs).
- **Alias map scope**: Only populated for $-suffixed imports where local != imported name. Non-dollar imports (e.g., `useStore as Store`) are not tracked in alias_map since they don't affect $-call detection.
- **Collector-level resolution**: Aliases resolved during call-site detection in the collector (not just in the transform), ensuring `DollarCallSite.callee_name` and `display_name` use original names consistently.

## Deviations from Plan

### Plan expectation vs reality

The plan predicted alias/core_module fixes would improve module count match from ~78 to ~110+ (a ~30 spec improvement). In reality, only 1 spec benefited from these fixes (rename_builder_io: 78->79). The diagnostic revealed that 77 of 83 remaining mismatches are caused by **JSX event handler extraction** (onClick$, onInput$ in JSX attributes create implicit $-calls needing segment extraction), which is a different feature gap entirely.

The alias detection was already partially working before (the collector tracked local names in dollar_imports). The fixes ensure correct ctx_name, Qrl import names, and display names for aliased imports, which is necessary for correctness even if module counts already matched.

The core_module broadening fixed rename_builder_io (which imports from @builder.io/qwik) and will benefit any spec using legacy import paths or @qwik.dev/react.

**Impact on plan targets:** The 100+ module count target requires JSX event handler extraction, which is outside this plan's scope. Plans 03/04 will need to address this gap.

## Issues Encountered

- Module count target (100+) not achievable with alias/core_module fixes alone. The diagnostic categorization clearly shows JSX event handler extraction is the dominant gap, affecting ~70+ specs.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Diagnostic categorization provides clear roadmap for Plans 03/04
- Import alias and core_module infrastructure is correct and tested
- JSX event handler extraction identified as the primary remaining gap for spec coverage
- All 158 unit tests + 6 spec tests + 162/162 spec transforms pass

---
*Phase: 13-source-maps-full-validation*
*Completed: 2026-02-11*
