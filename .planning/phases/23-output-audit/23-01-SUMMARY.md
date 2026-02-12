---
phase: 23-output-audit
plan: 01
subsystem: testing
tags: [audit, spec-comparison, output-validation, json]

# Dependency graph
requires:
  - phase: 22-v5-ship
    provides: "All 162 specs passing module count and metadata validation"
provides:
  - "audit-raw.json with structured deviation data for all 162 specs"
  - "output_audit.rs integration test for running full output comparison"
affects: [23-02-classification, 24-deviation-fixes]

# Tech tracking
tech-stack:
  added: []
  patterns: ["normalize-then-compare for whitespace-insensitive code comparison", "structural path matching with hash stripping for module pairing"]

key-files:
  created:
    - "crates/qwik-optimizer-oxc/tests/output_audit.rs"
    - ".planning/phases/23-output-audit/audit-raw.json"
  modified: []

key-decisions:
  - "Hash stripping uses last-underscore heuristic to remove 3+ alphanumeric hash suffixes from segment paths"
  - "Module matching falls back through exact path, structural (hash-stripped), and display_name/ctx_name metadata"
  - "Diff summaries detect const/let/var, import, QRL wrapper, and useLexicalScope differences specifically"

patterns-established:
  - "normalize_code: collapse whitespace, sort imports alphabetically, join as single space-separated string"
  - "strip_hash_from_path: rfind last underscore before extension, verify alphanumeric hash candidate"

# Metrics
duration: 2min
completed: 2026-02-12
---

# Phase 23 Plan 01: Output Audit Summary

**Full output comparison of all 162 specs against OXC optimizer producing 499 structured deviation records in audit-raw.json**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-12T04:11:22Z
- **Completed:** 2026-02-12T04:13:43Z
- **Tasks:** 1
- **Files modified:** 2

## Accomplishments
- Built output_audit.rs integration test running all 162 specs through the OXC optimizer and comparing JS output module-by-module
- Implemented module matching with exact path, structural (hash-stripped), and metadata-based fallback strategies
- Implemented code normalization: whitespace collapsing, import sorting, preserving const/let/var differences per user decision
- Generated audit-raw.json with 499 deviations: 5 module count mismatches, 359 code deviations, 135 unmatched modules across 161 specs

## Task Commits

Each task was committed atomically:

1. **Task 1: Build the output audit comparison test** - `31c6417` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/tests/output_audit.rs` - Rust integration test that runs all 162 specs and compares JS output module-by-module
- `.planning/phases/23-output-audit/audit-raw.json` - Structured JSON with all 499 deviations from the audit run

## Decisions Made
- Used a three-tier module matching strategy: exact path match first, then structural match with hash stripping, then metadata-based fallback using display_name and ctx_name
- Hash stripping identifies the hash as the last underscore-delimited alphanumeric segment (3+ chars) before the file extension
- Diff summary generator specifically detects const/let/var differences, import differences, QRL wrapper count changes, and useLexicalScope differences before falling back to general token comparison

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
- Two Rust lifetime errors in the initial implementation: a closure returning references with mismatched lifetimes (const_let_var HashMap keys) and a closure-based extract_fn function. Fixed by using owned Strings in the HashMap and converting the closure to a named function.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- audit-raw.json is ready for Plan 02 to classify all 499 deviations and produce the AUDIT-REPORT.md
- Key audit statistics: 1 spec matched perfectly, 161 specs have deviations, 0 transform errors
- The 5 known module count deviations are naturally recorded without special treatment

## Self-Check: PASSED

- FOUND: crates/qwik-optimizer-oxc/tests/output_audit.rs
- FOUND: .planning/phases/23-output-audit/audit-raw.json
- FOUND: .planning/phases/23-output-audit/23-01-SUMMARY.md
- FOUND: commit 31c6417

---
*Phase: 23-output-audit*
*Completed: 2026-02-12*
