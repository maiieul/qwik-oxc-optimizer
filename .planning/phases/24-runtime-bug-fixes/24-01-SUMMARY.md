---
phase: 24-runtime-bug-fixes
plan: 01
subsystem: optimizer
tags: [oxc, qrl, segment-extraction, jsx, codegen, parse-roundtrip]

# Dependency graph
requires:
  - phase: 23-output-audit
    provides: "audit-raw.json with 293 runtime-breaking deviation classifications"
provides:
  - "JSX event handler body serialization via parse-roundtrip approach"
  - "Fixed 3 optimizer-failure spec inputs (example_3, example_component_with_event_listeners_inside_loop, example_immutable_analysis)"
  - "EMPTY_SEGMENT_BODY diagnostic warning for future detection"
  - "4 new integration tests for segment extraction"
affects: [24-runtime-bug-fixes remaining plans, output-audit re-runs]

# Tech tracking
tech-stack:
  added: []
  patterns: [parse-roundtrip for JSX lambda serialization, span-based source extraction]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/lib.rs
    - .planning/spec/example_3.md
    - .planning/spec/example_component_with_event_listeners_inside_loop.md
    - .planning/spec/example_immutable_analysis.md
    - .planning/phases/23-output-audit/audit-raw.json

key-decisions:
  - "Parse-roundtrip approach for JSX lambda serialization: extract source substring by span, re-parse as `var x = <lambda>`, codegen the init expression"
  - "Store source_code as String field on QwikTransform for span-based extraction"
  - "Fixed spec inputs rather than making optimizer tolerant of invalid syntax"

patterns-established:
  - "serialize_jsx_lambda_from_source(): span-based re-parse for serializing JSX attribute lambdas that cannot be directly codegen'd as Expression"
  - "EMPTY_SEGMENT_BODY diagnostic: warn when segment has no body code to catch serialization regressions"

# Metrics
duration: 13min
completed: 2026-02-12
---

# Phase 24 Plan 01: Optimizer Failures and QRL Extraction Summary

**Fixed JSX event handler body serialization via parse-roundtrip, eliminated 3 optimizer-failure specs, and reduced empty-segment deviations from 45 to 0**

## Performance

- **Duration:** 13 min
- **Started:** 2026-02-12T04:53:38Z
- **Completed:** 2026-02-12T05:07:07Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- Eliminated all 3 optimizer-failure specs by fixing invalid input code (syntax errors, placeholder code, missing JSX expression containers)
- Added JSX event handler body serialization to `create_jsx_event_segments_recursive()` using a parse-roundtrip approach that extracts source by span, re-parses, and codegens
- Reduced empty-segment deviations from 45 to 0, meaning all JSX event handler segments now have proper body code
- Added EMPTY_SEGMENT_BODY diagnostic warning and 4 new integration tests for future regression detection
- Reduced unmatched expected modules from 78 to 67 and module count mismatches from 5 to 2

## Task Commits

Each task was committed atomically:

1. **Task 1: Diagnose and fix optimizer-failure specs and QRL extraction pipeline** - `790b668` (feat)
2. **Task 2: Validate QRL extraction fix scope with audit subset** - `5402a54` (feat)

## Files Created/Modified

- `crates/qwik-optimizer-oxc/src/transform.rs` - Added `source_code: String` field to QwikTransform, `serialize_jsx_lambda_from_source()` function, and body serialization call in `create_jsx_event_segments_recursive()`
- `crates/qwik-optimizer-oxc/src/lib.rs` - Pass source code to QwikTransform, added EMPTY_SEGMENT_BODY diagnostic, added 4 integration tests
- `.planning/spec/example_3.md` - Fixed syntax error (extra `)` at end of input)
- `.planning/spec/example_component_with_event_listeners_inside_loop.md` - Replaced placeholder `{...}` with full valid TSX input
- `.planning/spec/example_immutable_analysis.md` - Fixed `[].map(...)` to `{[].map(...)}` (added JSX expression container)
- `.planning/phases/23-output-audit/audit-raw.json` - Regenerated with updated audit results

## Decisions Made

1. **Parse-roundtrip for JSX lambda serialization** - OXC's `inherit_variants!` macro makes JSXExpression and Expression share variant types but have different enum layouts, so direct casting is unsound. Instead, extract lambda source by span, wrap as `var x = <lambda>`, parse with OXC parser, and codegen the init expression. This is safe and handles all lambda patterns (arrow/block/async).

2. **Store source_code on QwikTransform** - Added `source_code: String` field to enable span-based source extraction during traversal. The source is cloned once at construction time.

3. **Fix spec inputs, not the optimizer** - The 3 optimizer-failure specs had genuinely invalid input code (syntax errors, placeholder code). Fixed the spec inputs to match what the SWC-based reference optimizer would have received, rather than making the OXC optimizer more tolerant.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed Diagnostic struct field types**
- **Found during:** Task 1 (adding EMPTY_SEGMENT_BODY diagnostic)
- **Issue:** Used `code: String` and `highlights: Vec<SourceLocation>` but actual Diagnostic struct has `code: Option<String>` and `highlights: Option<Vec<SourceLocation>>`
- **Fix:** Changed to `code: Some("EMPTY_SEGMENT_BODY".to_string())` and `highlights: None`
- **Files modified:** `crates/qwik-optimizer-oxc/src/lib.rs`
- **Verification:** `cargo build` succeeds
- **Committed in:** `790b668` (part of Task 1 commit)

**2. [Rule 1 - Bug] Abandoned unsafe pointer casting approach**
- **Found during:** Task 1 (implementing body serialization)
- **Issue:** Initial approach tried to cast `&ArrowFunctionExpression` to `&Expression` via raw pointer, which is unsound because `Expression::ArrowFunctionExpression(Box<...>)` has different memory layout
- **Fix:** Switched to parse-roundtrip approach using source text spans
- **Files modified:** `crates/qwik-optimizer-oxc/src/transform.rs`
- **Verification:** Safe Rust, no unsafe blocks, all tests pass
- **Committed in:** `790b668` (part of Task 1 commit)

---

**Total deviations:** 2 auto-fixed (2 bugs)
**Impact on plan:** Both were implementation-level issues discovered during development. No scope creep.

## Issues Encountered

- OXC's `inherit_variants!` macro creates enum variants that share inner types between JSXExpression and Expression, but the enum wrappers have different layouts. This prevented direct codegen on JSX attribute values and required the parse-roundtrip approach.
- The 3 optimizer-failure specs each had unique input problems: example_3 had a trailing `});` instead of `};`, example_component_with_event_listeners_inside_loop used placeholder `{...}` in JSX attributes, and example_immutable_analysis had bare `[].map(...)` as JSX child without expression container braces.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Empty-segment deviations eliminated (45 -> 0), providing a clean baseline for remaining fix patterns
- Module count mismatches reduced to 2, unmatched expected modules down to 67
- Remaining runtime-breaking deviations are concentrated in: code generation patterns (import deduplication, PURE annotations), capture analysis, and JSX transform details
- Plans 02-05 of Phase 24 can proceed to address these remaining patterns

## Self-Check: PASSED

- All 6 modified files exist on disk
- Both task commits (790b668, 5402a54) exist in git history
- Key code patterns verified: serialize_jsx_lambda_from_source (2 refs), EMPTY_SEGMENT_BODY (1 ref), source_code field (7 refs)

---
*Phase: 24-runtime-bug-fixes*
*Completed: 2026-02-12*
