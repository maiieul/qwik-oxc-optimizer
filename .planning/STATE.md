# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 3 - Bugs & Correctness

## Current Position

Phase: 3 of 6 in progress (Bugs & Correctness)
Plan: 2 of 3 complete in phase 3
Status: In progress
Last activity: 2026-02-20 - Completed 03-01-PLAN.md (BUG-02 component options + BUG-06 comments)

Progress: [█████░░░░░] ~50%

## Performance Metrics

**Velocity:**
- Total plans completed: 5
- Average duration: 18min
- Total execution time: 1.7 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 2/3 | 16min | 8min |

**Recent Trend:**
- Last 5 plans: 01-01 (15min), 01-02 (45min), 02-01 (15min), 03-02 (5min), 03-01 (11min)
- Trend: Phase 3 bugs are fast -- well-researched, targeted changes

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
- [03-01]: Extra args passthrough applies to ALL named $-suffixed calls generically, not just component$
- [03-01]: OXC Codegen::print_expression() doesn't call build_comments() -- used temporary Program + build() for comment-preserving codegen instead
- [03-02]: JsxOptions::disable() required -- OXC TransformOptions default enables JSX plugin which would convert JSX to React format
- [03-02]: Scoping rebuild after transformer: SemanticBuilder::new().with_excess_capacity(2.0).build(&program)

### Pending Todos

None.

### Blockers/Concerns

- BUG-01 (TS stripping) RESOLVED -- oxc_transformer with TypeScript-only config, JSX explicitly disabled
- BUG-02 (component options) RESOLVED -- extra argument passthrough for all named $-suffixed calls
- BUG-06 (source comments) RESOLVED -- temporary Program + build() for comment-preserving segment body codegen
- 3 tests have segment ordering diffs (same names, different order) -- traversal order difference between SWC fold and OXC traverse
- 2 tests have missing segments from multi-file inputs -- needs Phase 3 work
- 23 event handler segments need q:p iteration variable injection for full paramNames parity (Phase 4)
- 160 snapshots changed after TS stripping -- many are cascading improvements, some are Phase 4 symptoms

## Session Continuity

Last session: 2026-02-20
Stopped at: Completed 03-01-PLAN.md
Resume file: None
