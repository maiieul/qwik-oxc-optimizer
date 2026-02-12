# Phase 23: Output Audit - Context

**Gathered:** 2026-02-11
**Status:** Ready for planning

<domain>
## Phase Boundary

Semantic comparison of OXC optimizer output against all 162 spec expected outputs. Classify every deviation as runtime-breaking or cosmetic. Produce a single deviation report. This phase does NOT fix anything — it identifies and documents what needs fixing for Phase 24.

</domain>

<decisions>
## Implementation Decisions

### Classification criteria
- Missing captures: runtime-breaking if the variable is actually referenced in the segment body (Claude evaluates per case — "if it's gonna break, fix it")
- QRL wrapping differences (wrong wrapper function): always runtime-breaking
- Import path differences (wrong path, missing re-export): always runtime-breaking
- Syntax differences (const vs let, structural differences): runtime-breaking — these affect program semantics
- Cosmetic differences (extra parens, reordering that doesn't change semantics): track only if syntactically wrong, otherwise leave alone

### Equivalence rules
- Ignore all whitespace differences (normalize before comparing)
- Import ordering does not matter (same imports in any order = equivalent)
- Extra captures: Claude's discretion — generally cosmetic (unnecessary closure variables don't crash), track but don't classify as runtime-breaking
- const/let/var differences: flag as deviations — these are meaningful semantic differences

### Report format
- Single summary markdown file in phase directory: `.planning/phases/23-output-audit/AUDIT-REPORT.md`
- Executive summary at top with pass/fail counts, category breakdown, severity breakdown
- Detail level at Claude's discretion (balance compact tables with enough diff context to understand each deviation)

### Known deviation handling
- The 5 module count deviations and 16 capture deviations get no special treatment — the full audit covers them naturally
- Module count heuristic (think like a JS engine): missing segments = runtime-breaking (lazy loading breaks), extra segments = investigate case by case
- Audit is a one-time operation, not a permanent test — Claude picks the most direct approach for the tooling

### Review gate
- User reviews and approves the deviation report BEFORE Phase 24 starts fixing anything
- The report must be clear enough for the user to approve/reject classifications

### Claude's Discretion
- Comparison approach (AST-level, normalized text, etc.) — whatever produces accurate results
- Script/tool implementation — one-time tooling, not permanent infrastructure
- Detail level per deviation in the report
- Extra capture classification (generally cosmetic)
- Investigation priority for module count deviations

</decisions>

<specifics>
## Specific Ideas

- "Think about this like you are a JS engine" — evaluate deviations from the perspective of whether a JavaScript runtime would break
- User didn't write the original optimizer, so capture analysis specifics are Claude's domain expertise
- "If it is syntactically wrong fix it. Otherwise if it is semantic and still makes sense then leave it"

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 23-output-audit*
*Context gathered: 2026-02-11*
