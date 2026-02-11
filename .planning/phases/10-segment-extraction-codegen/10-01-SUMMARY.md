---
phase: 10-segment-extraction-codegen
plan: 01
subsystem: codegen
tags: [oxc, codegen, segment-extraction, code-move, arrow-serialization]

# Dependency graph
requires:
  - phase: 08-core-detection-qrl-transforms
    provides: "QwikTransform exit_expression with segment recording and qrl replacement"
  - phase: 09-capture-analysis-props-destructuring
    provides: "Capture analysis (capture_names on SegmentData) and props destructuring"
provides:
  - "build_segment_code() in code_move.rs for constructing segment module JavaScript"
  - "Body extraction via Codegen::print_expression in exit_expression"
  - "finalize_segments() for populating child segment lazy imports"
  - "normalize_code() in emit.rs for parse+codegen formatting"
  - "Real segment module code in TransformOutput (no more empty placeholders)"
affects: [10-02-codegen-pipeline, phase-11, spec-test-matching]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "String-based segment code construction (code_move builds JS as string, normalize_code formats)"
    - "Codegen::print_expression + into_source_text for serializing AST expressions to strings"
    - "inject_captures_into_body for capture restoration injection into arrow function strings"
    - "finalize_segments() post-traverse pass for child segment metadata population"

key-files:
  created: []
  modified:
    - "crates/qwik-optimizer-oxc/src/code_move.rs"
    - "crates/qwik-optimizer-oxc/src/transform.rs"
    - "crates/qwik-optimizer-oxc/src/emit.rs"
    - "crates/qwik-optimizer-oxc/src/types.rs"
    - "crates/qwik-optimizer-oxc/src/lib.rs"

key-decisions:
  - "String-based segment code construction instead of AST-based Program construction (simpler, avoids allocator lifetime issues)"
  - "Codegen::print_expression for body serialization (avoids building temp Program inside traverse)"
  - "Extract-and-discard pattern: extract body as Argument, serialize, let it drop since segment strategy replaces entire call"
  - "normalize_code via parse+codegen for consistent formatting of string-constructed segment code"
  - "finalize_segments as post-traverse pass to avoid complex nested mutation during traverse"

patterns-established:
  - "String-based code_move: body serialized during transform, assembled into module string in code_move, normalized by parse+codegen"
  - "Codegen::print_expression + into_source_text for ad-hoc expression serialization"

# Metrics
duration: 6min
completed: 2026-02-11
---

# Phase 10 Plan 01: Segment Extraction + Code Generation Summary

**String-based segment code construction with body extraction via Codegen::print_expression, producing real JavaScript module code for each extracted $() segment**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-11T05:35:40Z
- **Completed:** 2026-02-11T05:41:40Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- Segment modules now produce real JavaScript code instead of empty placeholders
- Body extraction serializes transformed arrow functions (including props destructuring + capture analysis mutations) to strings during exit_expression
- code_move.rs builds complete module strings with _captures imports, qrl imports, lazy imports, capture restoration, and exported body
- Pipeline wired end-to-end: transform -> finalize_segments -> take_body_codes -> build_segment_code -> normalize_code
- 12 new tests (7 unit + 5 integration) all passing, 104 total tests

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement body extraction and build_segment_code** - `69ffae7` (feat)
2. **Task 2: Wire segment code generation into pipeline and add integration tests** - `bbbfcb7` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/types.rs` - Added body_code, child_lazy_imports, needs_qrl_import to SegmentData
- `crates/qwik-optimizer-oxc/src/transform.rs` - Body extraction in exit_expression, segment_body_codes field, take_segment_body_codes(), finalize_segments()
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Complete rewrite: build_segment_code(), inject_captures_into_body(), find_arrow_position(), 7 unit tests
- `crates/qwik-optimizer-oxc/src/emit.rs` - Added normalize_code() for parse+codegen formatting
- `crates/qwik-optimizer-oxc/src/lib.rs` - Wired pipeline with finalize_segments(), code_move, normalize_code; 5 integration tests

## Decisions Made
- **String-based code_move over AST-based Program construction:** The plan suggested both approaches. String-based was chosen because it avoids allocator lifetime issues inside traverse, is simpler to implement, and normalize_code() provides consistent formatting via parse+codegen.
- **Codegen::print_expression over temp Program wrapper:** OXC's Codegen has a print_expression method that serializes an expression without needing a full Program. This avoids the complexity of building a temporary Program with a separate allocator inside the traverse.
- **Extract-and-discard pattern:** For segment strategy, the body is extracted from call.arguments, serialized, then dropped. The entire call expression is replaced by qrl() anyway, so the body is not needed afterward.
- **finalize_segments as post-traverse pass:** Computing child segment metadata (lazy imports, needs_qrl_import) is done after traverse completes, avoiding complex nested mutation during the traversal.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Segment code generation is fully wired and producing real JavaScript
- Ready for Plan 10-02 (additional codegen refinements, if planned)
- Segment modules have correct exports, imports, capture restoration, and lazy imports
- All 104 tests pass including 12 new tests specific to segment code generation

---
*Phase: 10-segment-extraction-codegen*
*Completed: 2026-02-11*
