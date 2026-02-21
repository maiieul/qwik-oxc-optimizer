# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-19)

**Core value:** Snapshot parity with the SWC optimizer across all 162 test cases
**Current focus:** Phase 8 in progress. 18/19 plans across 8 phases executed. 138 snapshot files differ with ~3809 diff lines (down from 4405).

## Current Position

Phase: 8 of 9 (JSX Flags & Iteration Variables)
Plan: 1 of 3 complete in phase 8
Status: In progress
Last activity: 2026-02-21 - Completed 08-01-PLAN.md (scope-aware JSX flag classification)

Progress: [██████████████████░] 18/19 plans (95%)

## Performance Metrics

**Velocity:**
- Total plans completed: 18
- Average duration: 15min
- Total execution time: 4.53 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 01-naming | 2/2 | 60min | 30min |
| 02-metadata | 1/1 | 15min | 15min |
| 03-bugs-correctness | 3/3 | 51min | 17min |
| 04-signal-props-transforms | 4/4 | 77min | 19min |
| 05-jsx-keys-flags | 3/3 | 36min | 12min |
| 06-import-ordering-cleanup | 3/3 | 39min | 13min |
| 07-entry-module-emission | 1/1 | 3min | 3min |
| 08-jsx-flags-iteration-variables | 1/3 | 5min | 5min |

**Recent Trend:**
- Last 5 plans: 06-02 (8min), 06-03 (11min), 07-01 (3min), 08-01 (5min)
- Trend: Continued fast execution with targeted scope-aware fixes.

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
- [06-03]: Flush QRL hoists at function/arrow exit when loop_depth == 0 (avoids flushing inside .map() callback arrows)
- [06-03]: Insert hoisted const declarations after variable declarations at function body top (matches SWC positioning)
- [06-03]: Forward iteration order for hoists matches SWC BTreeMap alphabetical ordering
- [06-03]: Lazy imports filtered by referenced-ident analysis in exit_program (critical for correct entry module with hoisted QRLs)
- [07-01]: New is_inline_like_strategy variable before main_code block gates _hf* injection (segment strategy skips entry module injection)
- [07-01]: body_code.contains(var_name) for per-segment _hf* filtering -- extracts var name from "const _hfN = ..." via strip_prefix + split
- [07-01]: _fnSignal import: removed || !hoisted_stmts.is_empty() proxy -- body_code.contains("_fnSignal") is sufficient alone
- [08-01]: const_bindings populated from imports at init + from const declarations via enter_variable_declaration hook
- [08-01]: is_const_expression_with_scope is fully recursive for compound expressions (binary, conditional, template literal, etc.)
- [08-01]: Member expressions in children use const_bindings instead of module_imports scan for consistency

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
- QRL hoisting RESOLVED (06-03): loop-context QRL calls hoisted to enclosing function body (10 more snapshots fixed, 148->138)
- Entry module lazy import ordering RESOLVED (06-03): sorted by hash (matching SWC BTreeMap<Id> key) + filtered by referenced-ident analysis
- Entry module _hf* emission RESOLVED (07-01): conditional injection for segment strategy + per-segment filtering + _fnSignal false-positive fix
- JSX flag scope analysis RESOLVED (08-01): const_bindings scope tracking fixes 29 flag mismatches (21->fewer remaining)
- Remaining 138 snapshot files differ (but with ~596 fewer diff lines after 08-01):
  - Capture list differences (iteration variables in captures, missing/extra captures)
  - _fnSignal hoisting to module level vs segment level
  - q:p / var_props ordering differences
  - JSX flag differences (reduced from 21 to ~5 remaining scope-analysis edge cases)
  - Text normalization (trailing spaces in JSX text nodes)
  - Segment body code differences (_hf naming per-segment counter, _fnSignal usage)
  - Prop classification differences (var_props vs const_props for some expressions)
  - 1 deferred naming issue (should_extract_single_qrl_2 dedup suffix)

## Session Continuity

Last session: 2026-02-21T11:23:42Z
Stopped at: Completed 08-01-PLAN.md (scope-aware JSX flag classification)
Resume file: None
