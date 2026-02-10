---
phase: 05-deep-research-proof-of-concept
plan: 02
subsystem: poc
tags: [oxc, rust, traverse, semantic, scoping, capture-analysis, dollar-detection, poc]

# Dependency graph
requires:
  - phase: 05-01
    provides: "Capture analysis algorithm mapping to OXC Scoping APIs, multi-module output mapping"
provides:
  - "Working POC workspace at poc/ with Cargo.toml and shared utilities"
  - "POC-01: Dollar call site detection using OXC Traverse API"
  - "POC-02: Capture analysis using OXC Scoping APIs with correct capture/import/const classification"
  - "Validated OXC API patterns: scope_id on ArrowFunctionExpression, SymbolFlags::Import, traverse timing"
affects: [05-03, 06-capture-analysis-mapping, 06-multi-module-output-mapping]

# Tech tracking
tech-stack:
  added: [oxc 0.113, oxc_traverse 0.113, serde, serde_json, anyhow, base64]
  patterns:
    - "ArrowFunctionExpression.scope_id.get() for body scope (NOT ctx.current_scope_id() in enter_*)"
    - "pending_dollar_call flag pattern for linking call sites to arrow bodies"
    - "exit_call_expression for expression-arg dollar calls (useStyles$ pattern)"
    - "Scope containment via scoping.scope_ancestors(inner).any(|s| s == outer)"
    - "SymbolFlags::Import for import vs local variable classification"

key-files:
  created:
    - poc/Cargo.toml
    - poc/src/common.rs
    - poc/src/poc_01_detect_dollar.rs
    - poc/src/poc_02_capture_analysis.rs
  modified: []

key-decisions:
  - "Read arrow.scope_id.get() instead of ctx.current_scope_id() -- OXC traverse calls enter_* BEFORE pushing the new scope"
  - "Expression-arg dollar calls (useStyles$) handled via exit_call_expression fallback with parent scope analysis"
  - "Rust edition 2024 works with user's Rust 1.93.0"
  - "oxc_traverse must be a separate dependency -- not re-exported by the oxc umbrella crate"

patterns-established:
  - "POC binary pattern: mod common; use common::{parse_source, build_scoping}; with hardcoded test inputs and assertions"
  - "Traverse timing: enter_* fires BEFORE scope push, exit_* fires AFTER scope pop -- use node.scope_id for accurate scope"
  - "Capture algorithm: iterate all symbols, check scope containment for declaration vs references, classify by SymbolFlags"

# Metrics
duration: ~15min
completed: 2026-02-10
---

# Phase 05 Plan 02: POC Workspace and Capture Analysis Summary

**Working POC Rust programs proving OXC Traverse for dollar detection and OXC Scoping for capture analysis with 7 passing test cases across 4 spec-derived inputs**

## Performance

- **Duration:** ~15 min (continuation from context reset)
- **Started:** 2026-02-10 (previous session for Task 1, this session for Task 2)
- **Completed:** 2026-02-10
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- POC workspace established at poc/ with Cargo.toml, shared common.rs, and 4 binary targets
- POC-01 validates dollar call site detection: correctly finds $(), component$(), useStyles$(), useBrowserVisibleTask$() across 3 test cases
- POC-02 validates capture analysis: correctly classifies local captures (_rawProps, state), import re-emissions (thing, css1-3), and const-inlined values (arg0) across 4 test cases
- Discovered critical OXC traverse timing behavior: enter_* fires BEFORE scope push, requiring arrow.scope_id.get() instead of ctx.current_scope_id()

## Task Commits

Each task was committed atomically:

1. **Task 1: Create POC workspace with Cargo.toml, common.rs, and POC-01** - `1104833` (feat)
2. **Task 2: Implement POC-02 capture analysis using oxc_semantic** - `6d69876` (feat)

## Files Created/Modified
- `poc/Cargo.toml` - POC workspace manifest with oxc 0.113 dependencies and 4 binary targets
- `poc/src/common.rs` - Shared parse_source() and build_scoping() utilities
- `poc/src/poc_01_detect_dollar.rs` - Dollar call site detection using OXC Traverse with 3 test cases
- `poc/src/poc_02_capture_analysis.rs` - Capture analysis using OXC Scoping with 4 test cases

## Decisions Made

1. **Arrow function scope retrieval via node field, not context:** OXC traverse calls `enter_arrow_function_expression` BEFORE pushing the arrow's scope onto the context stack. The correct approach is `arrow.scope_id.get().unwrap()` to read the scope from the AST node directly.

2. **Expression-arg dollar calls via exit_call_expression:** For dollar-suffixed calls like `useStyles$(css1 + css2)` that take expression arguments (not arrow functions), the `pending_dollar_call` flag remains set through the call. The `exit_call_expression` handler catches these and records them with the parent scope as the analysis boundary.

3. **oxc_traverse as separate dependency:** The `oxc` umbrella crate does not re-export `oxc_traverse`. Must be added as `oxc_traverse = { version = "0.113" }` alongside the umbrella crate.

4. **BindingPattern is the enum directly:** In OXC 0.113, `BindingPattern<'a>` is the enum (not a struct with `.kind` field). Pattern match directly on `BindingPattern::BindingIdentifier(id)`.

5. **BindingIdentifier.symbol_id is Cell<Option<SymbolId>>:** Accessed via `.get()` method on the Cell, not as a function call.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] oxc crate does not have 'parser' feature**
- **Found during:** Task 1 (Cargo.toml setup)
- **Issue:** Plan specified `features = ["parser", "traverse", ...]` but oxc umbrella crate includes parser unconditionally and doesn't have a "parser" feature flag
- **Fix:** Removed "parser" from features list; parser is always available
- **Files modified:** poc/Cargo.toml
- **Verification:** cargo build succeeds
- **Committed in:** 1104833 (Task 1 commit)

**2. [Rule 3 - Blocking] oxc crate does not re-export traverse**
- **Found during:** Task 1 (Cargo.toml setup)
- **Issue:** Plan assumed traverse would be available via oxc umbrella crate feature flag, but it's not re-exported
- **Fix:** Added `oxc_traverse = { version = "0.113" }` as separate dependency
- **Files modified:** poc/Cargo.toml
- **Verification:** cargo build succeeds, traverse_mut works correctly
- **Committed in:** 1104833 (Task 1 commit)

**3. [Rule 1 - Bug] BindingPatternKind doesn't exist in OXC 0.113**
- **Found during:** Task 2 (POC-02 compilation)
- **Issue:** Research code used `BindingPatternKind::BindingIdentifier(id) = &declarator.id.kind` but OXC 0.113 makes BindingPattern the enum directly
- **Fix:** Changed to `BindingPattern::BindingIdentifier(id) = &declarator.id`
- **Files modified:** poc/src/poc_02_capture_analysis.rs
- **Verification:** Compiles and runs correctly
- **Committed in:** 6d69876 (Task 2 commit)

**4. [Rule 1 - Bug] BindingIdentifier.symbol_id is Cell field, not method**
- **Found during:** Task 2 (POC-02 compilation)
- **Issue:** Code used `id.symbol_id()` but symbol_id is a `Cell<Option<SymbolId>>` field
- **Fix:** Changed to `id.symbol_id.get()`
- **Files modified:** poc/src/poc_02_capture_analysis.rs
- **Verification:** Compiles and runs correctly
- **Committed in:** 6d69876 (Task 2 commit)

**5. [Rule 1 - Bug] ctx.current_scope_id() returns parent scope in enter_arrow_function_expression**
- **Found during:** Task 2 (POC-02 runtime)
- **Issue:** OXC traverse calls enter_* BEFORE pushing the new scope. ctx.current_scope_id() in enter_arrow_function_expression returns the enclosing scope, not the arrow's own scope.
- **Fix:** Read scope from `arrow.scope_id.get().unwrap()` (the AST node's scope_id field)
- **Files modified:** poc/src/poc_02_capture_analysis.rs
- **Verification:** All 4 test cases pass with correct scope IDs
- **Committed in:** 6d69876 (Task 2 commit)

**6. [Rule 2 - Missing Critical] Expression-arg dollar calls not tracked**
- **Found during:** Task 2 (POC-02 test case 3)
- **Issue:** useStyles$(css1 + css2) doesn't contain an arrow function, so pending_dollar_call was never consumed, causing useStyles$ bodies to not be recorded
- **Fix:** Added exit_call_expression handler to catch expression-arg dollar calls and compute_captures_for_expr_args for scope-based analysis
- **Files modified:** poc/src/poc_02_capture_analysis.rs
- **Verification:** Test case 3 passes -- CSS imports correctly classified as re-emissions
- **Committed in:** 6d69876 (Task 2 commit)

---

**Total deviations:** 6 auto-fixed (2 blocking, 3 bugs, 1 missing critical)
**Impact on plan:** All fixes necessary for correctness. Research had minor API inaccuracies that were resolved by reading OXC source code directly. The traverse timing discovery (deviation 5) is the most significant finding for future implementation work.

## Issues Encountered

- **OXC API discrepancies from research:** Multiple minor differences between research pseudocode and actual OXC 0.113 APIs (BindingPattern enum vs struct, Cell field vs method, traverse feature not in umbrella crate). Resolved by reading OXC source code directly from cargo registry.
- **Traverse scope timing:** The most impactful finding -- OXC traverse calls enter_* callbacks BEFORE pushing the node's scope. This is by design (allows visitors to see the parent scope) but means body scope must be read from the node, not the context.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- POC workspace is established and ready for plans 05-03 to add POC-03 (multi-module output) and POC-04 (source maps)
- Key OXC API patterns validated and documented for production implementation
- Traverse timing behavior (enter before scope push) must be carried forward to production code

## Self-Check: PASSED

- All 5 referenced files exist on disk
- Both commit hashes (1104833, 6d69876) found in git log
- POC-01 runs with all 3 test cases passing
- POC-02 runs with all 4 test cases passing
- cargo build compiles all binaries without errors

---
*Phase: 05-deep-research-proof-of-concept*
*Completed: 2026-02-10*
