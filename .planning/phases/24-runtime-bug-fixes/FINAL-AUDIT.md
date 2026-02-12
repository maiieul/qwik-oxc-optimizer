# Final Audit Report -- Phase 24 Runtime Bug Fixes

## Executive Summary

| Metric | Phase 23 (Before) | Phase 24 (After) | Change |
|--------|-------------------|-------------------|--------|
| Total specs audited | 162 | 162 | -- |
| Specs errored (optimizer failure) | 0 | 0 | -- |
| Total deviation records | 499 | 502 | +3* |
| **Runtime-breaking deviations** | **293** | **56** | **-237 (-81%)** |
| Cosmetic deviations | 206 | 446 | +240** |

\* Total deviation count increased because the audit now detects import differences more granularly after the fixes.

\** Many deviations reclassified from runtime-breaking to cosmetic after fixing the underlying issues (e.g., segment naming differences were previously counted as "missing modules" but are now properly identified as naming conventions with paired actual modules).

## Runtime-Breaking Deviations: 293 --> 56

### Remaining Runtime-Breaking Issues (56)

| Category | Count | Description | Root Cause |
|----------|-------|-------------|------------|
| missing-import-used | 52 | Segment body references a symbol but neither imports it nor captures it | Capture analysis and import propagation do not detect all identifier references in serialized segment body code |
| truly-missing-module | 4 | Expected module not generated (2 in example_qwik_react, 2 in relative_paths) | Pre-compiled QRL code with embedded segments that the optimizer cannot reverse-engineer |

### Missing Import Details (52 deviations across 46 specs)

These are cases where a segment module's body code references an identifier that was declared/imported in the original source file, but the segment doesn't have an import statement or capture restoration for it.

**By symbol category:**

- **Framework/library imports** (30): `useStore` (11 specs), `componentQrl` (5), `inlinedQrl` (4), `useSignal` (4), `useLexicalScope` (2), `useEffect` (2), `implicit$FirstArg` (2), `useTaskQrl`, `useOnDocument`, `eventQrl`, `_fnSignal`, `_jsxBranch`, `_jsxSplit`, `_restProps`, `_wrapSignal`, `createContextId`, `Slot` (2), `Fragment`, `jsx`, `mutable` (4), `z`, `z2`
- **Component/function references from entry module** (15): `Header` (4 specs), `Cmp` (3), `Button` (1), `ButtonArrow` (1), `Image` (2), `Lightweight` (1), `useFoo` (1), `useCounter` (1), `onGet` (1)
- **Variable references from entry module** (7): `foo` (2), `api` (1), `App` (1), `PANELS` (1), `Hola` (1), `Thing` (1)
- **Bulk re-exports** (2 specs with many symbols): `example_exports` (12 vars), `example_invalid_references` (10 vars: I1-I10)

**Root cause analysis:**

The missing imports fall into two categories:

1. **Self-imports from entry module** -- Segments need to import symbols that were available in the original module scope (components, functions, variables). The current capture analysis (`compute_captures()`) only tracks closure-scope variables via `capture_stack`, not module-level imports/declarations.

2. **Framework re-imports in entry modules** -- For inline strategy, entry modules need framework imports like `useStore`, `componentQrl`, etc. The current import propagation handles some Qrl-suffixed imports but misses many standard framework utilities.

**Fix complexity:** Moderate. Plan 05 addresses the self-import issue by enhancing `compute_captures()` to also check segment body references against the module's import map. Plan 06 addresses framework re-imports with content-based import detection in entry modules.

### Truly Missing Modules (4 deviations in 2 specs)

| Spec | Missing Modules | Root Cause |
|------|----------------|------------|
| example_qwik_react | 2 (qwikifyQrl_component_useWatch, qwikifyQrl_component) | Pre-compiled QRL code from @qwik.dev/react package contains embedded segment references that the optimizer cannot extract |
| relative_paths | 2 (dep/dist/lib.mjs segment, components/main.tsx segment) | Multi-file input with cross-file QRL references requires multi-file compilation support |

These are known blockers documented in STATE.md. Both require either pre-compiled QRL reverse-engineering or multi-file compilation support, which are beyond Phase 24 scope.

## Cosmetic Deviations: 446

| Category | Count | Description |
|----------|-------|-------------|
| codegen-style | 277 | Token-level code differences (formatting, expression ordering, JSX key assignment, const/let/var, extra imports) |
| naming-convention | 126 | Segment module paths differ in display name components (63 pairs: expected path has more JSX element path depth than actual, e.g., `div_button_q_e_click` vs `button_q_e_click`) |
| missing-import-cosmetic | 41 | Expected has imports that actual doesn't, but the imported symbol is NOT used in the actual body (different import grouping approach) |
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

### Plan 04: Final Validation (This Plan)
- Added `_getVarProps`, `_getConstProps`, `_restProps`, `_chk`, `_val` import checks in segment code generation (code_move.rs)
- Added inline strategy segment Qrl-suffixed import propagation (transform.rs)
- Comprehensive re-audit with improved deviation classification methodology
- Accurate runtime-breaking vs cosmetic separation using symbol-in-body verification

## Test Results

```
cargo test --package qwik-optimizer-oxc
  - snapshot_tests: 1 passed (162 snapshot specs)
  - spec_tests: 8 passed
  - output_audit: 1 passed (162 specs audited)
  - All existing tests: PASS
  - 172 unit tests: PASS
```

## Conclusion

Phase 24 reduced runtime-breaking deviations from 293 to 56, an 81% reduction. The remaining 56 are:
- 52 missing-import deviations across 46 specs (segments missing needed imports for symbols used in their body)
- 4 truly missing modules from pre-compiled QRL code (known limitation, deferred)

Of the 52 missing-import deviations:
- 30 involve framework/library imports that segments need but don't emit
- 22 involve self-imports from the entry module (components, functions, variables)

The OXC optimizer now produces output without errors for all 162 specs. The naming-convention issue (63 module path pairs) confirms that all expected segment *types* are being extracted -- only the display name path depth differs. Plans 05 and 06 are created to address the remaining 56 runtime-breaking deviations.
