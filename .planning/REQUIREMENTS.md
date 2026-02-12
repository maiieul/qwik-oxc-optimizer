# Requirements: Qwik Optimizer -- SWC-to-OXC Port

**Defined:** 2026-02-11
**Core Value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests

## v6.0 Requirements

Requirements for NAPI integration milestone. Each maps to roadmap phases.

### Output Verification

- [ ] **AUDIT-01**: Semantic comparison of generated JS output against spec expected output for all 162 specs
- [ ] **AUDIT-02**: Automated classification of each deviation as runtime-breaking or cosmetic
- [ ] **AUDIT-03**: Deviation report documenting all differences with severity and category

### Bug Fixes

- [ ] **FIX-01**: All runtime-breaking capture analysis deviations fixed (missing captures that would crash at runtime)
- [ ] **FIX-02**: All runtime-breaking QRL/import deviations fixed (wrong wrapping, broken imports)
- [ ] **FIX-03**: Test harness updated to validate fixed deviations don't regress

### NAPI Integration

- [ ] **NAPI-01**: `qwik-napi-oxc` crate with napi-rs v2, cdylib output, serde-json feature
- [ ] **NAPI-02**: `transform_modules` function exported with identical name and contract as SWC's `qwik_napi` -- same JS-side calling convention
- [ ] **NAPI-03**: All input/output types serialize/deserialize with camelCase naming, matching SWC NAPI wire format (so `platform.ts` can load OXC binding without code changes)

### Integration Validation

- [ ] **INTG-01**: Node.js test script that calls `transform_modules` through the NAPI binding for all 162 spec inputs
- [ ] **INTG-02**: NAPI round-trip produces same results as direct Rust API invocation

## Future Requirements

### Full Integration

- **FULL-01**: `transform_fs` function for filesystem-based transforms
- **FULL-02**: WASM fallback build for environments without native bindings
- **FULL-03**: Run Qwik's full test suite with OXC optimizer replacing SWC
- **FULL-04**: Cross-platform native bindings (darwin-arm64, darwin-x64, linux-x64, win32-x64)

### Remaining Deviations

- **DEV-01**: Fix cosmetic capture analysis deviations (extra captures that don't break runtime)
- **DEV-02**: Fix diagnostic generation for 3 deviation cases (class capture warnings, invalid segment errors, missing inlined function errors)

## Out of Scope

| Feature | Reason |
|---------|--------|
| Byte-for-byte output matching | Semantic equivalence is sufficient; OXC codegen has different formatting |
| Modifying SWC code | Read-only reference |
| TypeScript plugin changes | NAPI provides the binding surface; TS layer is untouched |
| WASM fallback | Deferred to future milestone |
| Cosmetic deviation fixes | Only fix deviations that break runtime behavior |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| AUDIT-01 | Phase 23 | Pending |
| AUDIT-02 | Phase 23 | Pending |
| AUDIT-03 | Phase 23 | Pending |
| FIX-01 | Phase 24 | Pending |
| FIX-02 | Phase 24 | Pending |
| FIX-03 | Phase 24 | Pending |
| NAPI-01 | Phase 25 | Pending |
| NAPI-02 | Phase 25 | Pending |
| NAPI-03 | Phase 25 | Pending |
| INTG-01 | Phase 26 | Pending |
| INTG-02 | Phase 26 | Pending |

**Coverage:**
- v6.0 requirements: 11 total
- Mapped to phases: 11
- Unmapped: 0

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-11 after roadmap creation*
