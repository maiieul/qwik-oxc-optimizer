# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 5 in progress - JSX Keys & Flags (plan 01 complete, key generation fixed).

## Current Position

Phase: 5 of 6 (JSX Keys & Flags)
Plan: 1 of 2 complete in phase 5 (05-01 key generation done)
Status: In progress
Last activity: 2026-02-20 - Completed 05-01-PLAN.md (JSX key prefix and root_jsx_mode)

Progress: [███████████░] ~88%

## Performance Metrics

**Velocity:**
- Total plans completed: 11
- Average duration: 16min
- Total execution time: 3.4 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 3/3 | 51min | 17min |
| 04-signal-props-transforms | 4/4 | 77min | 19min |
| 05-jsx-keys-flags | 1/2 | 9min | 9min |

**Recent Trend:**
- Last 5 plans: 04-01 (6min), 04-03 (11min), 04-02 (25min), 04-04 (35min), 05-01 (9min)
- Trend: Key generation plan was straightforward -- well-defined algorithm from SWC reference, clean parameter threading pattern.

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
- [04-04]: Body destructuring detected separately from parameter destructuring -- two distinct code paths
- [04-04]: Props param name threaded through all JSX transform functions as Option<&str> parameter
- [04-04]: Prop alias origin mapping for _fnSignal: test.value -> [props] dep with p0.test.value hoisted fn
- [04-04]: argument_to_expression was missing MemberExpression variants (pre-existing bug) -- fixed
- [05-01]: Manual base64url encoding (6-bit lookup table) instead of adding base64 crate dependency
- [05-01]: root_jsx_mode hooks added to all 9 SWC-equivalent statement types (function, arrow, for/for-in/for-of, while, do-while, if, block, return)
- [05-01]: is_fn detection: uppercase first char on Identifier/IdentifierReference + MemberExpression match

### Pending Todos

None.

### Blockers/Concerns

- BUG-01 (TS stripping) RESOLVED -- oxc_transformer with TypeScript-only config, JSX explicitly disabled
- BUG-02 (component options) RESOLVED -- extra argument passthrough for all named $-suffixed calls
- BUG-03 (capture ordering) RESOLVED -- alphabetical sort in compute_captures()
- BUG-04 (segment ordering) RESOLVED -- span-based sort before output iteration
- BUG-05 (test fixture) RESOLVED -- real 1074-line qwik-router bundle
- BUG-06 (source comments) RESOLVED -- temporary Program + build() for comment-preserving segment body codegen
- Gap 1 (non-destructured props) RESOLVED -- 04-04 gap closure plan
- Remaining snapshot diffs are Phase 5/6 issues:
  - Phase 5 DONE: JSX key generation (05-01)
  - Phase 5 TODO: immutability flags (05-02)
  - Phase 6: import ordering, use*() inlining (2x deferred), QRL hoisting (deferred from 04-02)
- 1 deferred naming issue (should_extract_single_qrl_2) -- dedup suffix on wrong segment due to traverse order
- Text normalization: trailing spaces in JSX text nodes stripped (e.g., "First " -> "First") -- noted for future fix

## Session Continuity

Last session: 2026-02-20T17:54:47Z
Stopped at: Completed 05-01-PLAN.md (JSX key prefix and root_jsx_mode)
Resume file: None
