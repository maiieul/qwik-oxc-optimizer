# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 3 - Bugs & Correctness

## Current Position

Phase: 2 of 6 complete (Metadata)
Plan: 1 of 1 complete in phase 2
Status: Phase 2 complete, ready to plan Phase 3
Last activity: 2026-02-20 - Phase 2 verified and complete

Progress: [███░░░░░░░] ~30%

## Performance Metrics

**Velocity:**
- Total plans completed: 3
- Average duration: 25min
- Total execution time: 1.25 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 ✓ | 60min | 30min |
| 02-metadata | 1/1 ✓ | 15min | 15min |

**Recent Trend:**
- Last 5 plans: 01-01 (15min), 01-02 (45min), 02-01 (15min)
- Trend: Phase 2 was fast -- well-researched, localized changes

*Updated after each plan completion*

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- [Roadmap]: 6-phase cascade -- naming first (clears most noise), imports last (final cleanup after all correct imports exist)
- [Roadmap]: ISSUES.md priority ordering adopted -- naming cascades into everything, bugs before features (missing segments block testing)
- [01-01]: Combined stack_ctxt + JSX event handler naming into single architectural change since they share the same push/pop mechanism
- [01-01]: OXC represents component JSX elements as IdentifierReference (not Identifier) -- must handle both variants
- [01-01]: Kept dollar_call_stack alongside new segment_stack for backward compatibility with finalize_segments matching
- [01-02]: Hash computation: hash on display_name WITHOUT filename prefix, then prepend file_name after (matches SWC lines 358-368)
- [01-02]: Fragment naming: only push when transpile_jsx=true (SWC sees Fragment after JSX transform)
- [01-02]: Raw $() calls: don't push callee name (SWC's handle_qsegment returns before push)
- [01-02]: Prod mode: use s_HASH for segment names in EmitMode::Prod
- [01-cleanup]: Collector display name derivation removed entirely (DollarCallSite, derive_display_name, etc.) -- was dead code never consumed by transform
- [02-01]: 23 remaining paramNames mismatches deferred to Phase 4 -- q:p iteration variable injection (22) + useResource$ _rawProps (1) are transform issues, not metadata extraction
- [02-01]: OXC FormalParameterRest has nested .rest.argument path (different from plan assumption)
- [02-01]: support_windows_paths test had double backslashes vs SWC's single -- fixed

### Pending Todos

None.

### Blockers/Concerns

- BUG-01 (TS stripping) needs research to decide approach: oxc_transformer feature, oxc_isolated_declarations, or manual stripping
- 3 tests have segment ordering diffs (same names, different order) -- traversal order difference between SWC fold and OXC traverse
- 2 tests have missing segments from multi-file inputs -- needs Phase 3 work
- 23 event handler segments need q:p iteration variable injection for full paramNames parity (Phase 4)

## Session Continuity

Last session: 2026-02-20
Stopped at: Phase 2 complete, ready to plan Phase 3
Resume file: None
