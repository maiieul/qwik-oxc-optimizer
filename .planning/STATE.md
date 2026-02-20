# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 6 (imports/cleanup) -- entry module import scoping (06-01) and segment import ordering (06-02) done.

## Current Position

Phase: 6 of 6 (Import Ordering & Cleanup)
Plan: 2 of 3 complete in phase 6 (06-01 entry import scoping + 06-02 segment import ordering done)
Status: In progress
Last activity: 2026-02-20 - Completed 06-01-PLAN.md (entry module import scoping)

Progress: [███████████████] ~97%

## Performance Metrics

**Velocity:**
- Total plans completed: 15
- Average duration: 15min
- Total execution time: 4.2 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 3/3 | 51min | 17min |
| 04-signal-props-transforms | 4/4 | 77min | 19min |
| 05-jsx-keys-flags | 3/3 | 36min | 12min |
| 06-import-ordering-cleanup | 2/3 | 28min | 14min |

**Recent Trend:**
- Last 5 plans: 05-01 (9min), 05-02 (14min), 05-03 (13min), 06-02 (8min), 06-01 (20min)
- Trend: Entry import scoping required deep AST walker for referenced-ident collection, including JSX handling. 8 snapshots fixed.

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
- [05-02]: immutable_function_cmp built in QwikTransform::new() from collected imports (Fragment, RenderOnce, Link, ?jsx/.md)
- [05-02]: jsx_mutable and immutable_function_cmp stored on ImportTracker for jsx_transform.rs access
- [05-02]: WrapPropSignal keeps immutable (SWC is_const=true); WrapPropNamed marks mutable (SWC is_const=false)
- [05-02]: Identifiers/member exprs treated as immutable in children (approximates SWC scope; may miss globals)
- [05-03]: OXC bottom-up traversal requires pre-capture of tracker.jsx_mutable before save/restore in child processing
- [05-03]: contains_mutable_jsx_call scans expression trees for _jsxSorted calls with non-immutable component tags
- [05-03]: Member expressions: mutable by default, immutable only when base object is a known import (matches SWC ConstCollector)
- [05-03]: Remaining 21 flag mismatches are scope-analysis issues (unresolved globals, local mutable bindings)
- [06-02]: _captures import emitted first (before sorted list) -- SWC special case, not sorted with other imports
- [06-02]: Standard Rust string comparison for import sort order -- matches SWC Atom::cmp
- [06-02]: Lazy import declarations moved after all sorted imports -- matches SWC extra_top_items positioning
- [06-01]: Post-hoc referenced-ident filtering in exit_program instead of scope-tracking during traversal (conceptually matches SWC DCE)
- [06-01]: collect_referenced_idents descends into nested function/arrow bodies (correct for inline strategy)
- [06-01]: JSX element names need dedicated walker (JSXIdentifier/JSXElementName separate from Expression::Identifier)
- [06-01]: BTreeMap grouping for specifier merging preserves insertion order (matches SWC original specifier order)
- [06-01]: Side-effect imports (no specifiers) always kept regardless of reference scanning

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
- Gap 2 (flag propagation) RESOLVED -- 05-03 gap closure plan (122->21 mismatches)
- Phase 5 DONE: JSX key generation (05-01), immutability flags (05-02), flag propagation (05-03)
- Post-formatting restored (cherry-picked from sort-format-fix): 2-space indent, object expansion, JSX-aware parsing
- Segment import ordering RESOLVED (06-02): 0 ordering-only diffs, 48 set diffs remain (other phase issues)
- Entry module extra imports RESOLVED (06-01): post-hoc filtering eliminates segment-only imports (8 snapshots fixed, 156->148)
- Remaining snapshot diffs are Phase 6 issues:
  - use*() return value destructuring inlining (2 fixtures)
  - QRL hoisting (deferred from 04-02)
- 1 deferred naming issue (should_extract_single_qrl_2) -- dedup suffix on wrong segment due to traverse order
- Text normalization: trailing spaces in JSX text nodes stripped (e.g., "First " -> "First") -- noted for future fix
- 21 remaining flag mismatches: 10 OXC=1/SWC=3 (over-aggressive mutability), 8 OXC=3/SWC=1 (scope analysis needed for globals/locals), 3 edge cases

## Session Continuity

Last session: 2026-02-20T21:30:10Z
Stopped at: Completed 06-01-PLAN.md (entry module import scoping)
Resume file: None
