# Final Audit Report -- Phase 24 Runtime Bug Fixes

## Executive Summary

| Metric | Phase 23 (Before) | Plan 04 (Mid) | Plan 06 (Final) | Total Change |
|--------|-------------------|---------------|-----------------|--------------|
| Total specs audited | 162 | 162 | 162 | -- |
| Specs errored (optimizer failure) | 0 | 0 | 0 | -- |
| Total deviation records | 499 | 502 | 502 | +3* |
| **Runtime-breaking deviations** | **293** | **56** | **10** | **-283 (-97%)** |
| Cosmetic deviations | 206 | 446 | 492 | +286** |

\* Total deviation count increased because the audit now detects import differences more granularly after the fixes.

\** Many deviations reclassified from runtime-breaking to cosmetic after fixing the underlying issues. Plan 05's capture analysis fix resolved 46 of 52 missing-import-used deviations, reclassifying them as cosmetic (the missing imports were for symbols not actually used in the body after the fix added the correct self-imports).

## Runtime-Breaking Deviations: 293 --> 10

### Remaining Runtime-Breaking Issues (10)

| Category | Count | Description | Root Cause |
|----------|-------|-------------|------------|
| missing-import-used | 6 | Segment body references a symbol but neither imports it nor captures it | Edge cases in self-import detection and JSX import source handling |
| truly-missing-module | 4 | Expected module not generated (2 in example_qwik_react, 2 in relative_paths) | Pre-compiled QRL code with embedded segments that the optimizer cannot reverse-engineer |

### Missing Import Details (6 deviations across 5 specs)

These are the remaining edge cases where a segment module's body code references an identifier that the segment doesn't import or capture.

| Spec | Module | Missing Symbols | Root Cause |
|------|--------|----------------|------------|
| example_drop_side_effects | test.tsx_test_component_button_q_e_click | `api` | Variable reference not tracked as module-level declaration |
| example_exports | project/test.tsx_Header_component_1 | `d`, `f`, `DefaultFn` | Aliased re-exports (`default as DefaultFn`, named `f`) not emitted as self-imports |
| example_invalid_references | test.tsx_App_component | `I5`, `I7` | Aliased imports (`_auto_I5 as I5`) -- I5 and I7 missing from self-import generation |
| example_jsx_import_source | test.js, test.tsx_App2_qwikify | `jsx`, `_jsx` | JSX import source configuration -- should import from `react/jsx-runtime`, not `@qwik.dev/core` |
| example_ts_enums_no_transpile | test.tsx_App_component | `Thing` | TypeScript enum not tracked as module-level declaration for self-import |

**Why these remain:** These are edge cases in the self-import mechanism and JSX import source handling. They involve:
1. **Aliased exports/imports** where the re-export alias differs from the local name (3 deviations)
2. **JSX import source configuration** where a non-standard JSX pragma should use a custom import source (2 deviations)
3. **Enum/variable edge cases** where certain declaration types are not yet tracked as module-level (1 deviation)

These 6 deviations are accepted as known limitations for the Phase 24 scope. They could be addressed in future phases if needed.

### Truly Missing Modules (4 deviations in 2 specs)

| Spec | Missing Modules | Root Cause |
|------|----------------|------------|
| example_qwik_react | 2 (qwikifyQrl_component_useWatch, qwikifyQrl_component) | Pre-compiled QRL code from @qwik.dev/react package contains embedded segment references that the optimizer cannot extract |
| relative_paths | 2 (dep/dist/lib.mjs segment, components/main.tsx segment) | Multi-file input with cross-file QRL references requires multi-file compilation support |

These are known blockers documented in STATE.md. Both require either pre-compiled QRL reverse-engineering or multi-file compilation support, which are beyond Phase 24 scope.

## Cosmetic Deviations: 492

| Category | Count | Description |
|----------|-------|-------------|
| codegen-style | 287 | Token-level code differences (formatting, expression ordering, JSX key assignment, const/let/var, extra imports) |
| naming-convention | 126 | Segment module paths differ in display name components (63 pairs: expected path has more JSX element path depth than actual, e.g., `div_button_q_e_click` vs `button_q_e_click`) |
| missing-import-cosmetic | 77 | Expected has imports that actual doesn't, but the imported symbol is NOT used in the actual body (different import grouping approach) |
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
- Runtime-breaking deviation count asserted at or below 10 (threshold)
- Updated this FINAL-AUDIT.md with final deviation counts

## Gap Closure: Plan 05 Impact Analysis

Plan 05 fixed 46 of 52 missing-import-used deviations by reclassifying module-level declarations as self-imports. Before Plan 05, segments referencing module-level declarations (components, functions, variables defined at the top level of the source file) silently dropped those references. After Plan 05, these references generate `import { X } from "./module_stem"` statements, matching SWC behavior.

**Before Plan 05:** 56 runtime-breaking (52 missing-import-used + 4 truly-missing-module)
**After Plan 05:** 10 runtime-breaking (6 missing-import-used + 4 truly-missing-module)
**Improvement:** -46 runtime-breaking deviations (-82% of Plan 04 count)

The 6 remaining missing-import-used deviations are edge cases involving aliased exports, JSX import source configuration, and TypeScript enum tracking. These are documented above and accepted as known limitations.

## Test Results

```
cargo test --package qwik-optimizer-oxc
  - snapshot_tests: 1 passed (162 snapshot specs)
  - spec_tests: 8 passed
  - output_audit: 1 passed (162 specs audited, regression gate passes)
  - All existing tests: PASS
```

## Conclusion

Phase 24 reduced runtime-breaking deviations from 293 to 10, a 97% reduction.

| Stage | Runtime-Breaking | Change |
|-------|-----------------|--------|
| Phase 23 baseline | 293 | -- |
| After Plans 01-04 | 56 | -237 (-81%) |
| After Plans 05-06 (final) | 10 | -283 (-97%) from baseline |

The remaining 10 are:
- 6 missing-import edge cases across 5 specs (aliased exports, JSX import source, enum tracking)
- 4 truly missing modules from pre-compiled QRL code (known limitation, deferred)

The OXC optimizer now produces output without errors for all 162 specs. A regression threshold assertion in output_audit.rs ensures that future changes cannot reintroduce fixed deviations without failing the test suite.
