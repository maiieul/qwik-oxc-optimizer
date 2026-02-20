# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 3 COMPLETE - Ready for Phase 4

## Current Position

Phase: 3 of 6 COMPLETE (Bugs & Correctness)
Plan: 3 of 3 complete in phase 3
Status: Phase complete
Last activity: 2026-02-20 - Completed 03-03-PLAN.md (BUG-05 fixture + BUG-04 segment ordering + BUG-03 capture ordering + triage)

Progress: [██████░░░░] ~60%

## Performance Metrics

**Velocity:**
- Total plans completed: 6
- Average duration: 18min
- Total execution time: 2.1 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 3/3 | 51min | 17min |

**Recent Trend:**
- Last 5 plans: 01-02 (45min), 02-01 (15min), 03-02 (5min), 03-01 (11min), 03-03 (35min)
- Trend: Phase 3 complete. 03-03 took longer due to comprehensive triage of 160 diffs + Python analysis scripts for capture ordering.

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
- [03-03]: Segment sort key: span.0 (source byte offset) not display_name -- matches SWC fold top-down source order
- [03-03]: Capture ordering: SWC uses HashSet->Vec->sort() producing alphabetical order; added capture_names.sort() in compute_captures()
- [03-03]: should_extract_single_qrl_2 dedup suffix naming issue deferred -- bottom-up traverse assigns _1 to wrong segment

### Pending Todos

None.

### Blockers/Concerns

- BUG-01 (TS stripping) RESOLVED -- oxc_transformer with TypeScript-only config, JSX explicitly disabled
- BUG-02 (component options) RESOLVED -- extra argument passthrough for all named $-suffixed calls
- BUG-03 (capture ordering) RESOLVED -- alphabetical sort in compute_captures()
- BUG-04 (segment ordering) RESOLVED -- span-based sort before output iteration
- BUG-05 (test fixture) RESOLVED -- real 1074-line qwik-router bundle
- BUG-06 (source comments) RESOLVED -- temporary Program + build() for comment-preserving segment body codegen
- 160 remaining snapshot diffs are ALL Phase 4/5/6 issues (no Phase 3 bugs remain)
  - Phase 4: _wrapProp (42x), _fnSignal (57x), q:p injection (22x)
  - Phase 5: _jsxSorted imports (181x), Fragment (67x)
  - Phase 6: import ordering (~20x)
- 1 deferred naming issue (should_extract_single_qrl_2) -- dedup suffix on wrong segment due to traverse order

## Session Continuity

Last session: 2026-02-20
Stopped at: Completed 03-03-PLAN.md (Phase 3 complete)
Resume file: None
