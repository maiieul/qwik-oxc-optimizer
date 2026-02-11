---
phase: 08-core-detection-qrl-transforms
plan: 01
subsystem: api
tags: [rust, oxc, parsing, semantic, hash, collector, qwik-optimizer]

# Dependency graph
requires:
  - phase: 07-crate-foundation-test-harness
    provides: Crate skeleton with 16 module stubs, type definitions, and test harness
provides:
  - parse_module() parsing TSX/TS/JSX/JS into OXC Program AST with Scoping via SemanticBuilder
  - collect() first-pass AST analysis finding dollar imports, call sites, imports, and exports
  - compute_segment_hash() deterministic 11-char base64url hashes matching SWC algorithm
  - classify_ctx_kind() and is_qwik_core_import() helper functions in words.rs
affects: [08-02-qrl-transforms, 09-tier2, 10-tier3, all-subsequent-transform-phases]

# Tech tracking
tech-stack:
  added: [base64 (now non-optional)]
  patterns: [recursive AST walk for collector, OXC inherit_variants! handling for JSXExpression, manual Debug impl for types containing OXC Program]

key-files:
  created: []
  modified:
    - crates/qwik-optimizer-oxc/src/parse.rs
    - crates/qwik-optimizer-oxc/src/hash.rs
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/words.rs
    - crates/qwik-optimizer-oxc/Cargo.toml

key-decisions:
  - "base64 made non-optional in Cargo.toml since hash computation needs it unconditionally (source-maps feature kept but no longer gates base64)"
  - "BindingPattern is an enum directly in OXC 0.113 (not a struct with kind field) -- match on BindingPattern::BindingIdentifier"
  - "JSXExpression uses inherit_variants! macro in OXC 0.113 so Expression variants are directly on the enum -- handle CallExpression etc. as JSXExpression::CallExpression"
  - "Manual Debug impl for ParseResult since OXC Program does not derive Debug"
  - "Collector uses two-pass approach: first pass collects all imports (to build dollar_imports set), second pass walks for call sites and exports"

patterns-established:
  - "Collector pattern: recursive walk functions (walk_statement_for_calls, walk_expression_for_calls) rather than OXC Traverse trait, since this is a read-only pass"
  - "JSX handling: dedicated walk_jsx_expression_for_calls function that handles the inherit_variants! pattern"
  - "Display name derivation: var_name + callee_suffix pattern (e.g., Foo_component for component$)"
  - "Hash algorithm: DefaultHasher(scope?, rel_path, display_name) -> u64 -> LE bytes -> base64url -> replace -/_ with 0"

# Metrics
duration: 6min
completed: 2026-02-11
---

# Phase 8 Plan 1: Core Detection and Hash Pipeline Summary

**OXC parser + SemanticBuilder parsing, recursive AST collector finding dollar imports/call sites with display names, and deterministic 11-char segment hash computation**

## Performance

- **Duration:** 6 min
- **Started:** 2026-02-11T03:51:54Z
- **Completed:** 2026-02-11T03:57:34Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments
- parse_module() parses TSX/TS/JSX/JS with source type detection, builds semantic Scoping via SemanticBuilder, reports parse errors as Diagnostics
- collect() performs two-pass AST walk: first collects all imports (building dollar_imports set), then walks for call sites with display name derivation and nesting tracking
- compute_segment_hash() ports the exact SWC algorithm (DefaultHasher + base64url + dash/underscore replacement) producing 11-char alphanumeric hashes
- words.rs enhanced with classify_ctx_kind() (event$ -> EventHandler, all else -> Function) and is_qwik_core_import()
- 48 total tests passing (32 new + 16 existing Phase 7), zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement parse.rs and hash.rs** - `a125b83` (feat)
2. **Task 2: Implement collector.rs and enhance words.rs** - `648307a` (feat)

## Files Created/Modified
- `crates/qwik-optimizer-oxc/Cargo.toml` - Made base64 non-optional for hash computation
- `crates/qwik-optimizer-oxc/src/parse.rs` - Module parsing with OXC parser + SemanticBuilder scoping
- `crates/qwik-optimizer-oxc/src/hash.rs` - Segment hash computation with DefaultHasher + base64url
- `crates/qwik-optimizer-oxc/src/collector.rs` - First-pass AST analysis collecting dollar imports, call sites, imports/exports
- `crates/qwik-optimizer-oxc/src/words.rs` - Added ctx_kind classification and qwik core import check helpers

## Decisions Made
- Made base64 non-optional in Cargo.toml since hashing needs it unconditionally. The source-maps feature is preserved for controlling source map generation behavior.
- Used recursive walk functions for the collector instead of OXC Traverse trait. Collector is read-only and does not need mutation, so simpler recursive functions suffice.
- Used two-pass collection: first pass gathers all imports (needed to know which identifiers are dollar imports), second pass walks for call sites and exports.
- OXC 0.113 API adaptations: BindingPattern is a flat enum, JSXExpression uses inherit_variants! (all Expression variants directly on JSXExpression), Program does not implement Debug.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] OXC 0.113 BindingPattern API differs from expected**
- **Found during:** Task 2 (collector.rs implementation)
- **Issue:** BindingPattern in OXC 0.113 is a flat enum, not a struct with `kind` field. No `BindingPatternKind` exists.
- **Fix:** Match directly on `BindingPattern::BindingIdentifier` instead of `pattern.kind`
- **Files modified:** crates/qwik-optimizer-oxc/src/collector.rs
- **Verification:** cargo test passes
- **Committed in:** 648307a

**2. [Rule 3 - Blocking] OXC 0.113 JSXExpression uses inherit_variants! macro**
- **Found during:** Task 2 (collector.rs implementation)
- **Issue:** JSXExpression does not have a `JSXExpression::Expression(expr)` variant. Instead, all Expression variants are directly on JSXExpression via the inherit_variants! macro.
- **Fix:** Created dedicated `walk_jsx_expression_for_calls()` that matches CallExpression and other expression variants directly on JSXExpression
- **Files modified:** crates/qwik-optimizer-oxc/src/collector.rs
- **Verification:** test_collect_example_1_pattern passes with 3 call sites found through JSX attributes
- **Committed in:** 648307a

**3. [Rule 3 - Blocking] OXC Program does not derive Debug**
- **Found during:** Task 1 (parse.rs implementation)
- **Issue:** ParseResult contains OXC Program which does not implement Debug, causing test compilation failure for unwrap_err()
- **Fix:** Added manual Debug impl for ParseResult that shows source_type and program body length
- **Files modified:** crates/qwik-optimizer-oxc/src/parse.rs
- **Verification:** All parse tests compile and pass
- **Committed in:** a125b83

---

**Total deviations:** 3 auto-fixed (3 blocking -- all OXC API mismatches)
**Impact on plan:** All auto-fixes necessary to resolve OXC 0.113 API differences from expected patterns. No scope creep.

## Issues Encountered
None beyond the auto-fixed OXC API deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- parse_module(), collect(), and compute_segment_hash() form the complete detection pipeline needed by Plan 08-02 (QRL transforms)
- The transform pass (Plan 08-02) can now: parse source -> collect dollar imports/call sites -> compute segment hashes -> transform $-calls to QRL wrappers
- Scoping is returned from parse but not yet used in collector (will be needed for capture analysis in Phase 9)

## Self-Check: PASSED

- All 5 modified files verified present on disk
- Both task commits verified in git log (a125b83, 648307a)
- cargo test -p qwik-optimizer-oxc exits 0 (48 tests pass, zero failures)
- cargo build -p qwik-optimizer-oxc exits 0 (warnings only for unused items in unimplemented modules)

---
*Phase: 08-core-detection-qrl-transforms*
*Completed: 2026-02-11*
