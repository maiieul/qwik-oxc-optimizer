# Requirements: Qwik Optimizer -- OXC API Research & Architecture

**Defined:** 2026-02-10
**Core Value:** A complete, SWC-independent behavioral specification of every optimizer transformation so the OXC port can be built from spec, not from reverse-engineering SWC code

## v2.0 Requirements

Requirements for v2.0 OXC API Research & Architecture milestone. Each maps to roadmap phases.

### API Mapping

- [ ] **APIM-01**: Every `$()` extraction pattern mapped to OXC Traverse + AstBuilder APIs with code examples
- [ ] **APIM-02**: Every qrl/inlinedQrl wrapping pattern mapped to OXC expression construction APIs
- [ ] **APIM-03**: Capture analysis algorithm mapped to oxc_semantic Scoping APIs (find_binding, get_resolved_references, scope_ancestors)
- [ ] **APIM-04**: JSX transformation patterns (_jsxSorted, _jsxSplit) mapped to OXC expression replacement APIs
- [ ] **APIM-05**: Import rewriting patterns (remove component$, add componentQrl) mapped to OXC statement mutation APIs
- [ ] **APIM-06**: Multi-module output pattern mapped to AstBuilder Program construction + separate Allocator strategy
- [ ] **APIM-07**: Source map generation mapped to oxc_codegen + oxc_sourcemap APIs with span preservation strategy
- [ ] **APIM-08**: All 14 CONV types cross-referenced to specific OXC API patterns
- [ ] **APIM-09**: Props destructuring and signal optimization patterns mapped to OXC APIs
- [ ] **APIM-10**: Entry strategy, code stripping, and const folding patterns mapped to OXC APIs

### Proof of Concept

- [ ] **POC-01**: Working Rust POC demonstrating OXC Traverse trait for detecting $() call sites in real spec inputs
- [ ] **POC-02**: Working Rust POC demonstrating capture analysis using oxc_semantic against spec files with known capture lists
- [ ] **POC-03**: Working Rust POC demonstrating multi-module output (one input Program split into main + segment Programs)
- [ ] **POC-04**: Working Rust POC demonstrating source map generation for split modules via oxc_codegen

### Architecture

- [ ] **ARCH-01**: Complete crate module layout with dependency ordering between modules
- [ ] **ARCH-02**: Public API design (TransformModulesOptions, TransformOutput, SegmentAnalysis) mapped to OXC internals
- [ ] **ARCH-03**: Data flow specification from parse -> analyze -> emit -> codegen with type signatures
- [ ] **ARCH-04**: Test strategy for validating output against 162 spec files (harness design, comparison approach)
- [ ] **ARCH-05**: Cargo.toml specification with exact dependencies, feature flags, and edition/MSRV

## Future Requirements

### Implementation (v3.0)

- **IMPL-01**: Scaffold oxc-optimizer crate with module layout from ARCH-01
- **IMPL-02**: Implement capture analysis module with all 8 edge cases
- **IMPL-03**: Implement analysis pass (Traverse-based $() detection + collection)
- **IMPL-04**: Implement main module mutation (emit phase)
- **IMPL-05**: Implement segment module construction (multi-output)
- **IMPL-06**: Implement codegen + source maps
- **IMPL-07**: Implement JSX transformation
- **IMPL-08**: Implement advanced features (props destructuring, signals, entry strategy, stripping)
- **IMPL-09**: Pass all 162 spec tests

## Out of Scope

| Feature | Reason |
|---------|--------|
| Writing actual optimizer code | This is a research milestone -- implementation is v3.0 |
| Modifying the 162 spec files | Specs are locked as the source of truth from v1.0 |
| Vite/Rollup plugin integration | Plugin layer untouched until after optimizer port |
| Performance benchmarking | Premature before implementation exists |
| SWC codebase modifications | Read-only against SWC codebase |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| APIM-01 | Phase 4 | Pending |
| APIM-02 | Phase 4 | Pending |
| APIM-03 | Phase 5 | Pending |
| APIM-04 | Phase 6 | Pending |
| APIM-05 | Phase 4 | Pending |
| APIM-06 | Phase 5 | Pending |
| APIM-07 | Phase 6 | Pending |
| APIM-08 | Phase 6 | Pending |
| APIM-09 | Phase 6 | Pending |
| APIM-10 | Phase 6 | Pending |
| POC-01 | Phase 5 | Pending |
| POC-02 | Phase 5 | Pending |
| POC-03 | Phase 5 | Pending |
| POC-04 | Phase 5 | Pending |
| ARCH-01 | Phase 4 | Pending |
| ARCH-02 | Phase 4 | Pending |
| ARCH-03 | Phase 4 | Pending |
| ARCH-04 | Phase 4 | Pending |
| ARCH-05 | Phase 4 | Pending |

**Coverage:**
- v2.0 requirements: 19 total
- Mapped to phases: 19
- Unmapped: 0

---
*Requirements defined: 2026-02-10*
*Last updated: 2026-02-10 after roadmap creation*
