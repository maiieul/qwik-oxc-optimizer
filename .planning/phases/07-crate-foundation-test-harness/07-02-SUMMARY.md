---
phase: 07-crate-foundation-test-harness
plan: 02
subsystem: testing
tags: [rust, markdown-parser, spec-tests, qwik-optimizer, test-harness]

# Dependency graph
requires:
  - phase: 07-crate-foundation-test-harness
    plan: 01
    provides: Compilable qwik-optimizer-oxc crate with public types and stub transform_modules()
provides:
  - Spec file parser that extracts test data from 162 behavioral spec markdown files
  - Test runner validating all 162 specs parse with zero failures
  - build_options() that creates TransformModulesOptions from parsed specs
  - Stub test confirming transform_modules entry point works
affects: [08-tier1-basic-extraction, 09-tier2-component-pattern, all-subsequent-phases]

# Tech tracking
tech-stack:
  added: [serde_json (for spec metadata parsing)]
  patterns: [line-by-line markdown parsing without regex, integration tests via tests/ directory]

key-files:
  created:
    - crates/qwik-optimizer-oxc/tests/spec_parser.rs
    - crates/qwik-optimizer-oxc/tests/spec_tests.rs
  modified: []

key-decisions:
  - "Case-insensitive config key matching to handle spec variations (Transpile TS vs Transpile Ts)"
  - "Relaxed entry point segment metadata assertion: some specs omit metadata for brevity"
  - "Test mode maps to EmitMode::Lib (specs mark Test as default, our default is Lib)"
  - "Inline segment metadata format parsed into serde_json::Value for completeness"
  - "Filename config override applied to input path in build_options()"

patterns-established:
  - "Integration tests live in crates/qwik-optimizer-oxc/tests/ as separate modules"
  - "spec_parser is a module imported by spec_tests via mod spec_parser"
  - "Spec directory resolved from CARGO_MANIFEST_DIR at compile time"

# Metrics
duration: 6min
completed: 2026-02-11
---

# Phase 7 Plan 2: Test Harness Summary

**Spec file parser and test runner for 162 behavioral markdown specs: extracts input code, expected output modules, segment metadata, diagnostics, and config overrides with zero parse failures**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-11T03:09:22Z
- **Completed:** 2026-02-11T03:15:50Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- All 162 spec markdown files parse successfully with zero failures
- 439 output modules extracted (261 entry points, 269 with segment metadata)
- 149 specs with config overrides correctly parsed, 3 specs with non-empty diagnostics
- build_options() produces serializable TransformModulesOptions for all 162 specs
- Stub transform_modules() entry point confirmed working via test_transform_stub

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement spec file parser (spec_parser.rs)** - `9310ee5` (feat)
2. **Task 2: Create test runner and validate all 162 specs parse** - `676e8dc` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/tests/spec_parser.rs` - Markdown spec file parser with parse_spec_file(), build_options(), spec_dir(), load_all_specs()
- `crates/qwik-optimizer-oxc/tests/spec_tests.rs` - Integration test runner with test_parse_all_specs, test_build_options, test_transform_stub

## Decisions Made
- **Case-insensitive config keys**: Some spec files use "Transpile Ts" while others use "Transpile TS". Normalized all keys to lowercase for matching.
- **Relaxed segment metadata assertion**: Not all entry-point modules have segment metadata in every spec file. Some specs omit metadata for brevity (e.g., example_prod_node, example_qwik_conflict). The assertion now validates that at least one module per spec has code, rather than requiring all entry points to have metadata.
- **EmitMode::Test mapped to EmitMode::Lib**: 75 specs use "Test" or "Test (default)" as the Mode value. Since our EmitMode enum doesn't have a Test variant and specs mark it as "(default)", it maps to the default Lib mode.
- **Inline segment metadata**: Some specs (example_component_with_event_listeners_inside_loop) use `#### Segment Metadata (captures: true, captureNames: ["cart"])` format without a JSON code block. These are parsed into serde_json::Value objects.
- **Filename config override**: 10 specs override the input filename via a "Filename" config table entry. This is applied in build_options() to set the correct input path.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed case-insensitive config key matching**
- **Found during:** Task 2 (running test_build_options)
- **Issue:** Spec files use both "Transpile TS" and "Transpile Ts" (and similarly for JSX). The match statement used exact string matching.
- **Fix:** Normalized all config keys to lowercase before matching
- **Files modified:** crates/qwik-optimizer-oxc/tests/spec_parser.rs
- **Verification:** All 162 specs parse without unknown key warnings
- **Committed in:** 9310ee5 (handled in initial implementation)

**2. [Rule 1 - Bug] Fixed module parsing for specs without code blocks**
- **Found during:** Task 2 (running test_parse_all_specs)
- **Issue:** Some entry-point modules in specs (example_qwik_conflict) have only descriptive text instead of code blocks. The parser expected every module to have a code block.
- **Fix:** Added extract_module_code_block() that returns empty string when no code block exists before the next section boundary
- **Files modified:** crates/qwik-optimizer-oxc/tests/spec_parser.rs
- **Verification:** All 162 specs parse, modules with descriptions handled gracefully
- **Committed in:** 9310ee5 (handled in initial implementation)

**3. [Rule 1 - Bug] Fixed inline segment metadata parsing**
- **Found during:** Task 2 (running test_parse_all_specs)
- **Issue:** Some specs use `#### Segment Metadata (captures: true, ...)` format with metadata in the heading, not a JSON code block
- **Fix:** Added parse_inline_segment_metadata() to extract key-value pairs from inline format
- **Files modified:** crates/qwik-optimizer-oxc/tests/spec_parser.rs
- **Verification:** All 162 specs parse, inline metadata extracted correctly
- **Committed in:** 9310ee5 (handled in initial implementation)

**4. [Rule 2 - Missing Critical] Added Filename and Dev Path config key handling**
- **Found during:** Task 2 (running test_build_options)
- **Issue:** 10 specs use "Filename" config to override the input file path, 1 spec uses "Dev Path". These were not in the original key list.
- **Fix:** Added case-insensitive matching for "filename" and "dev path" keys; Filename overrides the input path in build_options()
- **Files modified:** crates/qwik-optimizer-oxc/tests/spec_parser.rs
- **Verification:** No unknown key warnings for any of the 162 specs
- **Committed in:** 9310ee5 (handled in initial implementation)

---

**Total deviations:** 4 auto-fixed (3 bugs, 1 missing critical)
**Impact on plan:** All auto-fixes necessary for handling real spec format variations. No scope creep -- these are edge cases in the actual spec data.

## Issues Encountered
None beyond the auto-fixed deviations above. The iterative build-test-fix cycle caught and resolved all issues within the single task execution.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Test harness is complete and ready for Phase 8 (Tier 1: Basic Extraction)
- When transform_modules() starts producing real output, the parsed specs provide expected values for comparison
- Zero friction for adding output comparison tests: load specs, run optimizer, compare modules
- The public API contract is validated: `transform_modules(TransformModulesOptions) -> Result<TransformOutput>`

## Self-Check: PASSED

- All 2 created files verified present on disk
- Both task commits verified in git log (9310ee5, 676e8dc)
- cargo test -p qwik-optimizer-oxc --test spec_tests exits 0 (3 tests pass)
- 162 spec files parsed with zero failures

---
*Phase: 07-crate-foundation-test-harness*
*Completed: 2026-02-11*
