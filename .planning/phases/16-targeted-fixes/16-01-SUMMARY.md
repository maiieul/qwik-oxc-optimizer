---
phase: 16-targeted-fixes
plan: 01
subsystem: optimizer
tags: [rust, hashset, lazylock, minify, bug-fix, performance]

# Dependency graph
requires:
  - phase: 15-dead-code-removal
    provides: Clean codebase with zero warnings
provides:
  - Fixed minify_expression_string preserving spaces between identifier chars
  - KNOWN_GLOBALS as LazyLock<HashSet> with O(1) lookup
  - 3 new unit tests for minify_expression_string
affects: [17-napi-bridge, 18-testing-hardening]

# Tech tracking
tech-stack:
  added: [std::sync::LazyLock]
  patterns: [LazyLock static initialization for const-like HashSet data]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/collector.rs

key-decisions:
  - "Used LazyLock<HashSet> over const phf::Set -- stays in std, no extra dependency"
  - "Removed extra & from HashSet::contains call sites per Borrow trait API"

patterns-established:
  - "LazyLock<HashSet> pattern for large static lookup tables needing O(1) access"

# Metrics
duration: 2min
completed: 2026-02-11
---

# Phase 16 Plan 01: BUG-01 and PERF-01 Targeted Fixes Summary

**Fixed minify_expression_string space-dropping bug and converted KNOWN_GLOBALS to LazyLock<HashSet> for O(1) lookup**

## Performance

- **Duration:** 2 min
- **Started:** 2026-02-11T20:55:00Z
- **Completed:** 2026-02-11T20:57:05Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Fixed BUG-01: minify_expression_string now correctly preserves spaces between identifier characters (was silently dropping them due to empty block body)
- Fixed PERF-01: KNOWN_GLOBALS converted from &[&str] slice (O(n) linear scan) to LazyLock<HashSet<&str>> (O(1) lookup)
- Added 3 unit tests covering space preservation, whitespace collapsing, and string literal handling
- All 161 tests pass with zero regressions, zero compiler warnings

## Task Commits

Each task was committed atomically:

1. **Task 1: Fix minify_expression_string space-between-identifiers bug** - `ff7fa2a` (fix)
2. **Task 2: Convert KNOWN_GLOBALS from slice to LazyLock HashSet** - `a78a45e` (perf)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/src/transform.rs` - Fixed empty block in minify_expression_string to push space; updated 2 KNOWN_GLOBALS call sites for HashSet API; added 3 unit tests
- `crates/qwik-optimizer-oxc/src/collector.rs` - Converted KNOWN_GLOBALS from const &[&str] to static LazyLock<HashSet<&str>>; added LazyLock import; updated 1 call site

## Decisions Made
- Used std::sync::LazyLock rather than a third-party crate (phf, once_cell) -- stays within std, no new dependency needed
- Removed the extra `&` from all three KNOWN_GLOBALS.contains() call sites since HashSet::contains uses the Borrow trait and accepts `&str` directly

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Both cataloged issues (BUG-01, PERF-01) are resolved
- Crate compiles cleanly with zero warnings
- Ready for remaining phase 16 plans or subsequent phases

## Self-Check: PASSED

- FOUND: crates/qwik-optimizer-oxc/src/transform.rs
- FOUND: crates/qwik-optimizer-oxc/src/collector.rs
- FOUND: .planning/phases/16-targeted-fixes/16-01-SUMMARY.md
- FOUND: ff7fa2a (Task 1 commit)
- FOUND: a78a45e (Task 2 commit)

---
*Phase: 16-targeted-fixes*
*Completed: 2026-02-11*
