# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 4 complete - Signal & Props Transforms (all 3/3 plans done). Ready for Phase 5 planning.

## Current Position

Phase: 4 of 6 (Signal & Props Transforms)
Plan: 3 of 3 complete in phase 4 (04-01 + 04-02 + 04-03 all done)
Status: Phase complete
Last activity: 2026-02-20 - Completed 04-02-PLAN.md (loop tracking + q:p injection)

Progress: [█████████░] ~83%

## Performance Metrics

**Velocity:**
- Total plans completed: 9
- Average duration: 16min
- Total execution time: 2.6 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 3/3 | 51min | 17min |
| 04-signal-props-transforms | 3/3 | 42min | 14min |

**Recent Trend:**
- Last 5 plans: 03-01 (11min), 03-03 (35min), 04-01 (6min), 04-03 (11min), 04-02 (25min)
- Trend: Loop tracking required multiple OXC API fixes (BindingPattern vs BindingPatternKind, expression_array arity, param naming). Clean implementation with q:p injection matching SWC format.

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
- [04-01]: Local variables are co-reactive in _fnSignal: they become deps only when primary reactive sources (signal.value, _rawProps, store chains) exist in same expression
- [04-01]: OXC codegen parentheses stripped from _fnSignal string representation to match SWC format
- [04-02]: in_callback_depth uses u32 counter (not bool) for nested iteration methods (.map inside .map)
- [04-02]: SWC uses "_" for both placeholder params (positions 0 and 1), not "_" and "_1"
- [04-02]: q:p/q:ps keys use string literal format in var_props (colon requires quoting)
- [04-02]: QRL hoisting deferred to Phase 6 -- OXC Traverse hoisted_function_stmts only injects at module top level
- [04-03]: Default expressions serialized to strings during analysis, rebuilt via parse-and-clone during rewrite (avoids arena lifetime issues)
- [04-03]: Import identifiers treated as const for default value checking (matches SWC is_const_expr)
- [04-03]: use*() return value destructuring inlining deferred to Phase 6 (only 2 test fixtures affected)

### Pending Todos

None.

### Blockers/Concerns

- BUG-01 (TS stripping) RESOLVED -- oxc_transformer with TypeScript-only config, JSX explicitly disabled
- BUG-02 (component options) RESOLVED -- extra argument passthrough for all named $-suffixed calls
- BUG-03 (capture ordering) RESOLVED -- alphabetical sort in compute_captures()
- BUG-04 (segment ordering) RESOLVED -- span-based sort before output iteration
- BUG-05 (test fixture) RESOLVED -- real 1074-line qwik-router bundle
- BUG-06 (source comments) RESOLVED -- temporary Program + build() for comment-preserving segment body codegen
- Remaining snapshot diffs are Phase 5/6 issues:
  - Phase 4 DONE: q:p injection (04-02), _wrapProp children wrapping (04-01), _fnSignal children wrapping (04-01), props destructuring defaults + skip cases (04-03)
  - Phase 5: _jsxSorted imports (181x), Fragment (67x)
  - Phase 6: import ordering (~20x), use*() inlining (2x deferred), QRL hoisting (deferred from 04-02)
- 1 deferred naming issue (should_extract_single_qrl_2) -- dedup suffix on wrong segment due to traverse order
- Text normalization: trailing spaces in JSX text nodes stripped (e.g., "First " -> "First") -- noted for future fix
- Non-destructured props parameter (e.g., `(props)` instead of `({fromProps})`) not yet handled for _wrapProp -- may need attention in Phase 5/6
- Pre-existing test failure in destructure_args_colon_props from concurrent 04-01 work (snapshot mismatch in segment file)

## Session Continuity

Last session: 2026-02-20T13:45:00Z
Stopped at: Completed 04-02-PLAN.md (loop tracking + q:p injection) -- Phase 4 complete
Resume file: None
