---
phase: 03-verify-completeness
plan: 01
subsystem: testing
tags: [python, audit, normalization, markdown, spec-files]

# Dependency graph
requires:
  - phase: 02-generate-all-spec-files
    provides: "162 spec files in .planning/spec/"
provides:
  - "audit-specs.py: two-pass auditor for structural consistency + convention completeness"
  - "fix-specs.py: deterministic normalizer for all cosmetic inconsistencies"
  - "All 162 spec files structurally normalized"
affects: [03-02-PLAN, future-spec-generation]

# Tech tracking
tech-stack:
  added: [python3-audit-scripts]
  patterns: [two-pass-audit, deterministic-normalization, key-behavior-preservation]

key-files:
  created:
    - ".planning/tools/audit-specs.py"
    - ".planning/tools/fix-specs.py"
    - ".planning/phases/03-verify-completeness/audit-report-initial.txt"
  modified:
    - ".planning/spec/*.md (134 of 162 files normalized)"

key-decisions:
  - "Key behavior baseline is 8 files (not 65 as research estimated)"
  - "NO_DETAILS (6 files) and MISSING_CONV (3 files) deferred to Plan 02"
  - "Noconv normalizer preserves Key behavior notes in same Conventions section"

patterns-established:
  - "Two-pass audit: structural checks (Pass 1) separated from convention checks (Pass 2)"
  - "Safety guard: Key behavior count comparison before/after prevents data loss"

# Metrics
duration: 5min
completed: 2026-02-10
---

# Phase 3 Plan 01: Spec Audit and Normalization Summary

**Two-pass Python auditor and deterministic normalizer fix 9 categories of structural inconsistency across 134 of 162 spec files**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-10T19:26:39Z
- **Completed:** 2026-02-10T19:31:26Z
- **Tasks:** 2
- **Files modified:** 136 (2 scripts created, 134 spec files normalized)

## Accomplishments
- Built audit-specs.py with 10 structural checks and 14-convention completeness detection
- Built fix-specs.py with 7 deterministic normalization passes and Key behavior safety guard
- Normalized 134 spec files: resolved all backtick module paths, bracket entry points, prose configs, config backticks, extra annotations, non-standard diagnostics, and non-standard no-conventions text
- Key behavior notes preserved in all 8 files (verified by count match)

## Task Commits

Each task was committed atomically:

1. **Task 1: Build audit script and run initial audit** - `b90afd6` (feat)
2. **Task 2: Build fix script, run it, and verify zero structural issues** - `56dbfa2` (feat)

## Files Created/Modified
- `.planning/tools/audit-specs.py` - Two-pass auditor: structural consistency (10 checks) + convention completeness (14 CONV patterns)
- `.planning/tools/fix-specs.py` - Deterministic normalizer: 7 transformation categories with Key behavior safety guard
- `.planning/phases/03-verify-completeness/audit-report-initial.txt` - Initial audit baseline (140 files with issues)
- `.planning/spec/*.md` - 134 spec files normalized (525 insertions, 343 deletions)

## Decisions Made
- Key behavior baseline is 8 files (research estimated 65 -- actual count verified via grep)
- 6 files without `<details>` blocks are Plan 02 scope (not structural normalization)
- 3 files with missing conventions are Plan 02 scope (convention completeness)
- Noconv text normalizer splits on paragraphs to preserve Key behavior notes in the same section

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed noconv normalizer destroying Key behavior notes**
- **Found during:** Task 2 (fix script first run)
- **Issue:** `normalize_noconv_text()` replaced the entire Conventions Applied section content between `## Conventions Applied` and `## Function Calls in Output`, which included Key behavior notes in `should_not_move_over_side_effects.md`
- **Fix:** Changed to split section by paragraphs and only replace the first paragraph (the "None" text), preserving subsequent Key behavior paragraphs
- **Files modified:** `.planning/tools/fix-specs.py`
- **Verification:** Re-run confirmed Key behavior count preserved (8 files before and after)
- **Committed in:** 56dbfa2 (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - Bug)
**Impact on plan:** Essential fix for data preservation. No scope creep.

## Issues Encountered
None beyond the deviation noted above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Audit infrastructure ready for Plan 02 (convention completeness fixes)
- 6 files need `<details>` blocks added (Plan 02)
- 3 files need missing conventions documented (Plan 02)
- All structural normalization complete -- Plan 02 can focus solely on content completeness

---
*Phase: 03-verify-completeness*
*Completed: 2026-02-10*
