# Final Audit Report -- Phase 24 Runtime Bug Fixes

## Executive Summary

| Metric | Phase 23 (Before) | Plan 04 (Mid) | Plan 06 (Round 1) | Plan 09 (Round 2) | Plan 11 (Round 3) | Total Change |
|--------|-------------------|---------------|-------------------|-------------------|-------------------|--------------|
| Total specs audited | 162 | 162 | 162 | 162 | 162 | -- |
| Specs errored (optimizer failure) | 0 | 0 | 0 | 0 | 0 | -- |
| Total deviation records | 499 | 502 | 502 | 502 | 501 | +2* |
| **Runtime-breaking deviations** | **293** | **56** | **10** | **5** | **4** | **-289 (-99%)** |
| Cosmetic deviations | 206 | 446 | 492 | 497 | 497 | +291** |

\* Total deviation count increased because the audit now detects import differences more granularly after the fixes.

\** Many deviations reclassified from runtime-breaking to cosmetic after fixing the underlying issues. Plan 05's capture analysis fix resolved 46 of 52 missing-import-used deviations. Plans 07-08 resolved 5 more (pattern defaults, TS enums, default exports, JSX import source).

## Runtime-Breaking Deviations: 293 --> 4

### Remaining Runtime-Breaking Issues (4)

| Category | Count | Description | Root Cause |
|----------|-------|-------------|------------|
| missing-import-used | 0 | -- | All resolved (Plan 10 fixed the last one) |
| truly-missing-module | 4 | Expected module not generated (2 in example_qwik_react, 2 in relative_paths) | Pre-compiled QRL code with embedded segments that the optimizer cannot reverse-engineer |

### Truly Missing Modules (4 deviations in 2 specs)

| Spec | Missing Modules | Root Cause |
|------|----------------|------------|
| example_qwik_react | 2 (qwikifyQrl_component_useWatch, qwikifyQrl_component) | Pre-compiled QRL code from @qwik.dev/react package contains embedded segment references that the optimizer cannot extract |
| relative_paths | 2 (dep/dist/lib.mjs segment, components/main.tsx segment) | Multi-file input with cross-file QRL references requires multi-file compilation support |

These are known architectural limitations documented in STATE.md. Both require either pre-compiled QRL reverse-engineering or multi-file compilation support, which are beyond Phase 24 scope.

## Cosmetic Deviations: 497

| Category | Count | Description |
|----------|-------|-------------|
| codegen-style | 287 | Token-level code differences (formatting, expression ordering, JSX key assignment, const/let/var, extra imports) |
| naming-convention | 126 | Segment module paths differ in display name components (63 pairs: expected path has more JSX element path depth than actual, e.g., `div_button_q_e_click` vs `button_q_e_click`) |
| missing-import-cosmetic | 82 | Expected has imports that actual doesn't, but the imported symbol is NOT used in the actual body (different import grouping approach) |
| module-count-mismatch | 2 | Module count differs (example_qwik_react: expected 5 got 3, relative_paths: expected 4 got 2) |

All cosmetic deviations are acceptable per locked project decision: "Fix only runtime-breaking deviations."

### Naming Convention Pattern

The 63 naming-convention pairs follow a consistent pattern: the expected module path includes the full JSX element hierarchy (e.g., `Foo_component_div_button_q_e_click`) while the actual output uses a shorter path (e.g., `Foo_component_button_q_e_click`). The modules themselves are produced correctly -- only the display name path differs. This affects hash values (since hashes include the display name) but not runtime behavior.

## Fixes Applied in Phase 24

### Plan 01: QRL Extraction and Body Serialization
- Parse-roundtrip approach for JSX lambda serialization
- Fixed TypeScript type annotation handling in body extraction
- Fixed async arrow function position detection

### Plan 02: Import Resolution and Segment Re-emission
- ImportKind enum for default/namespace/named import tracking
- ReemittedImport struct for capture-to-segment import propagation
- Store needed_imports on all segments

### Plan 03: Capture Analysis and JSX Event Handler Codegen
- analyze_lambda_captures() for JSX handler scope analysis
- Nested capture_stack frame merging
- qrl()/inlinedQrl() attribute value replacement in JSX output

### Plan 04: Comprehensive Re-audit and Classification
- Added `_getVarProps`, `_getConstProps`, `_restProps`, `_chk`, `_val` import checks in segment code generation (code_move.rs)
- Added inline strategy segment Qrl-suffixed import propagation (transform.rs)
- Comprehensive re-audit with improved deviation classification methodology
- Accurate runtime-breaking vs cosmetic separation using symbol-in-body verification

### Plan 05: Module-Level Declaration Self-Imports
- Removed `module_level_decls.contains()` skip from `compute_captures()` so module-level declarations are captured
- Added `reclassify_module_level_decl_captures()` to convert module-level decl captures to self-imports (`import { X } from "./module"`)
- Updated JSX event handler capture filtering to allow module_level_decls through parent scope filter
- 20 snapshot files updated with correct self-import statements
- Reduced missing-import-used deviations from 52 to 6 (46 fixed, 88% of import deviations)

### Plan 06: Gap Closure Validation and Regression Gate
- Re-ran full output audit confirming Plan 05's impact
- Added regression threshold assertion (FIX-03) to prevent reintroduction of fixed deviations
- Runtime-breaking deviation count asserted at or below 10 (initial threshold)
- Updated FINAL-AUDIT.md with round 1 deviation counts

### Plan 07: Declaration Collection Edge Cases
- Exhaustive BindingPattern matching (all 4 variants: BindingIdentifier, ObjectPattern, ArrayPattern, AssignmentPattern)
- TSEnumDeclaration tracked as module-level declaration for self-import generation
- Named default export functions/classes added to module_level_decls
- Fixed specs: example_exports (d, f, DefaultFn), example_invalid_references (I5, I7), example_ts_enums_no_transpile (Thing)
- 5 snapshot files updated with correct self-imports

### Plan 08: JSX Import Source Propagation
- Custom JSX import source extraction from @jsxImportSource pragma
- React-style `_jsx(tag, {props})` codegen for custom JSX sources
- Segment modules emit correct JSX runtime import for custom sources
- Fixed specs: example_jsx_import_source (jsx, _jsx)
- Tightened regression threshold from 10 to 5

## Gap Closure Round 1: Plan 05 Impact Analysis

Plan 05 fixed 46 of 52 missing-import-used deviations by reclassifying module-level declarations as self-imports. Before Plan 05, segments referencing module-level declarations (components, functions, variables defined at the top level of the source file) silently dropped those references. After Plan 05, these references generate `import { X } from "./module_stem"` statements, matching SWC behavior.

**Before Plan 05:** 56 runtime-breaking (52 missing-import-used + 4 truly-missing-module)
**After Plan 05:** 10 runtime-breaking (6 missing-import-used + 4 truly-missing-module)
**Improvement:** -46 runtime-breaking deviations (-82% of Plan 04 count)

## Gap Closure Round 2: Plans 07-08 Impact Analysis (5 fixed)

Plans 07 and 08 targeted the 6 remaining missing-import-used deviations identified in Plan 06's audit:

| Fix | Plan | Specs Fixed | Symbols | Approach |
|-----|------|-------------|---------|----------|
| AssignmentPattern defaults | 07 | example_invalid_references | I5, I7 | Exhaustive BindingPattern matching in collector.rs and transform.rs |
| TSEnumDeclaration | 07 | example_ts_enums_no_transpile | Thing | Added TSEnumDeclaration to module-level declaration collector |
| Named default exports | 07 | example_exports | d, f, DefaultFn | Track `export default function Foo(){}` in module_level_decls |
| JSX import source | 08 | example_jsx_import_source | jsx, _jsx | Custom JSX source extraction and React-style codegen |

**Before Plans 07-08:** 10 runtime-breaking (6 missing-import-used + 4 truly-missing-module)
**After Plans 07-08:** 5 runtime-breaking (1 missing-import-used + 4 truly-missing-module)
**Improvement:** -5 runtime-breaking deviations (5 of 6 missing-import-used resolved)

**Remaining edge case (at the time):** `example_drop_side_effects` -- the `api` symbol defined via `export const api = server$(...)` was a module-level export assigned from a QRL wrapper function call. This was fixed in Plan 10 (see Round 3 below).

## Gap Closure Round 3: Plan 10 Impact Analysis (1 fixed)

Plan 10 targeted two issues: display name collisions and the last missing-import-used deviation.

| Fix | Plan | Specs Fixed | Symbols | Approach |
|-----|------|-------------|---------|----------|
| Display name collisions | 10 | example_1 (and others) | renderHeader, renderHeader_component, etc. | wrapper_callee_name context for non-dollar wrapper function display names |
| api missing-import | 10 | example_drop_side_effects | api | Tolerant parse error handling in analyze_lambda_captures (await in non-async is semantic, not structural) |

**Before Plan 10:** 5 runtime-breaking (1 missing-import-used + 4 truly-missing-module)
**After Plan 10:** 4 runtime-breaking (0 missing-import-used + 4 truly-missing-module)
**Improvement:** -1 runtime-breaking deviation (last missing-import-used resolved)

**Root cause of api fix:** The `analyze_lambda_captures` function bailed out on *any* parse error, including semantic errors like `await` in a non-async function. The lambda body `() => await api()` produced a semantic parse error, causing capture analysis to return empty results. The fix changed the bailout condition from "any parse errors" to "empty program body" -- semantic errors produce a valid AST and identifier references can still be extracted.

**Display name collision fix:** Added `wrapper_callee_name` to `CollectContext` so that when `$()` is an argument to a non-dollar function like `component(...)`, the wrapper function name is included in the display name. This eliminated duplicate segment names (e.g., three segments all named `renderHeader` in example_1 now have unique names: `renderHeader`, `renderHeader_component`, `renderHeader_div_q_e_click`).

## Test Results

```
cargo test --package qwik-optimizer-oxc
  - unit tests: 178 passed
  - snapshot_tests: 1 passed (162 snapshot specs)
  - spec_tests: 8 passed
  - output_audit: 1 passed (162 specs audited, regression gate: 4 <= 4)
  - All tests: PASS
```

## Conclusion

Phase 24 reduced runtime-breaking deviations from 293 to 4, a 99% reduction across 11 plans.

| Stage | Runtime-Breaking | Change |
|-------|-----------------|--------|
| Phase 23 baseline | 293 | -- |
| After Plans 01-04 | 56 | -237 (-81%) |
| After Plans 05-06 (round 1) | 10 | -283 (-97%) from baseline |
| After Plans 07-09 (round 2) | 5 | -288 (-98%) from baseline |
| After Plans 10-11 (round 3) | 4 | -289 (-99%) from baseline |

The remaining 4 are all truly-missing-module deviations (architectural limitations):
- 2 in example_qwik_react (pre-compiled QRL code from @qwik.dev/react)
- 2 in relative_paths (multi-file cross-module QRL references)

All missing-import-used deviations have been fully resolved (0 remaining). The OXC optimizer now produces output without runtime errors for all 162 specs except the 4 accepted architectural edge cases. A regression threshold assertion (`RUNTIME_BREAKING_THRESHOLD = 4`) in output_audit.rs ensures that future changes cannot reintroduce fixed deviations without failing the test suite.
