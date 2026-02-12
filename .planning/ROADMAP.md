# Roadmap: Qwik Optimizer SWC-to-OXC Port

## Milestones

- v1.0 Spec Generation -- Phases 1-3 (shipped 2026-02-10)
- v2.0 OXC API Research & Architecture -- Phases 4-6 (shipped 2026-02-11)
- v3.0 OXC Optimizer Port -- Phases 7-13 (shipped 2026-02-11)
- v4.0 Code Quality Refactor -- Phases 14-19 (shipped 2026-02-11)
- v5.0 Drop-in Replacement Compliance -- Phases 20-22 (shipped 2026-02-12)
- v6.0 NAPI Integration -- Phases 23-26 (in progress)

## Phases

<details>
<summary>v1.0 Spec Generation (Phases 1-3) -- SHIPPED 2026-02-10</summary>

- [x] Phase 1: OXC AST Utility (1/1 plan) -- completed 2026-02-10
- [x] Phase 2: Generate All Spec Files (6/6 plans) -- completed 2026-02-10
- [x] Phase 3: Verify Completeness (2/2 plans) -- completed 2026-02-10

Full details: `milestones/v1.0-ROADMAP.md`

</details>

<details>
<summary>v2.0 OXC API Research & Architecture (Phases 4-6) -- SHIPPED 2026-02-11</summary>

- [x] Phase 4: Core API Mapping & Architecture (2/2 plans) -- completed 2026-02-10
- [x] Phase 5: Deep Research & Proof of Concept (3/3 plans) -- completed 2026-02-10
- [x] Phase 6: Secondary Patterns & Cross-Reference (3/3 plans) -- completed 2026-02-11

Full details: `milestones/v2.0-ROADMAP.md`

</details>

<details>
<summary>v3.0 OXC Optimizer Port (Phases 7-13) -- SHIPPED 2026-02-11</summary>

- [x] Phase 7: Crate Foundation + Test Harness (2/2 plans) -- completed 2026-02-11
- [x] Phase 8: Core Detection + QRL Transforms (2/2 plans) -- completed 2026-02-10
- [x] Phase 9: Capture Analysis + Props Destructuring (2/2 plans) -- completed 2026-02-10
- [x] Phase 10: Segment Extraction + Codegen (2/2 plans) -- completed 2026-02-11
- [x] Phase 11: JSX + Signal Transforms (2/2 plans) -- completed 2026-02-11
- [x] Phase 12: Annotations + Stripping (2/2 plans) -- completed 2026-02-11
- [x] Phase 13: Source Maps + Full Validation (4/4 plans) -- completed 2026-02-11

Full details: `milestones/v3.0-ROADMAP.md`

</details>

<details>
<summary>v4.0 Code Quality Refactor (Phases 14-19) -- SHIPPED 2026-02-11</summary>

- [x] Phase 14: Style Cleanup (2/2 plans) -- completed 2026-02-11
- [x] Phase 15: Dead Code Removal (2/2 plans) -- completed 2026-02-11
- [x] Phase 16: Targeted Fixes (1/1 plan) -- completed 2026-02-11
- [x] Phase 17: Extract JSX Transform (1/1 plan) -- completed 2026-02-11
- [x] Phase 18: const_replace VisitMut Rewrite (1/1 plan) -- completed 2026-02-11
- [x] Phase 19: Spec Compliance Verification (1/1 plan) -- completed 2026-02-11

Full details: `milestones/v4.0-ROADMAP.md`

</details>

<details>
<summary>v5.0 Drop-in Replacement Compliance (Phases 20-22) -- SHIPPED 2026-02-12</summary>

- [x] Phase 20: Path Resolution (1/1 plan) -- completed 2026-02-12
- [x] Phase 21: Import Correctness (1/1 plan) -- completed 2026-02-12
- [x] Phase 22: Display Names and Annotations (1/1 plan) -- completed 2026-02-12

Full details: `milestones/v5.0-ROADMAP.md`

</details>

### v6.0 NAPI Integration (In Progress)

**Milestone Goal:** Verify semantic output correctness, fix runtime-breaking deviations, and build the NAPI crate so the OXC optimizer can replace SWC in the Qwik build pipeline.

- [x] **Phase 23: Output Audit** - Semantic comparison of OXC output against all 162 spec expected outputs -- completed 2026-02-11
- [ ] **Phase 24: Runtime Bug Fixes** - Fix all runtime-breaking deviations found by the audit
- [ ] **Phase 25: NAPI Crate** - Build `qwik-napi-oxc` with napi-rs exposing `transform_modules`
- [ ] **Phase 26: Integration Validation** - Verify the NAPI binding produces correct results end-to-end

## Phase Details

### Phase 23: Output Audit
**Goal**: Every spec's generated JS output has been semantically compared to its expected output, with deviations classified and documented
**Depends on**: Phase 22 (v5.0 complete)
**Requirements**: AUDIT-01, AUDIT-02, AUDIT-03
**Success Criteria** (what must be TRUE):
  1. A script runs all 162 specs through the OXC optimizer and compares generated JS to spec expected output (not just module count or metadata)
  2. Each deviation is classified as runtime-breaking or cosmetic with documented rationale
  3. A deviation report exists listing every difference with its severity, category (capture, import, QRL, codegen), and affected spec
  4. The 5 known module count deviations and 16 known capture deviations are re-evaluated with actual output comparison
**Plans**: 2 plans

Plans:
- [x] 23-01-PLAN.md -- Build and run output audit comparison tool (Rust integration test comparing all 162 specs)
- [x] 23-02-PLAN.md -- Classify deviations and write AUDIT-REPORT.md (user review gate)

### Phase 24: Runtime Bug Fixes
**Goal**: All runtime-breaking deviations are fixed so the OXC optimizer produces semantically correct output for every spec
**Depends on**: Phase 23 (audit results needed to know what to fix)
**Requirements**: FIX-01, FIX-02, FIX-03
**Success Criteria** (what must be TRUE):
  1. Every capture analysis deviation classified as runtime-breaking in the audit is fixed (missing captures that would crash at runtime)
  2. Every QRL wrapping and import deviation classified as runtime-breaking is fixed (wrong wrapper function, broken import paths, missing re-exports)
  3. Re-running the audit script after fixes shows zero runtime-breaking deviations
  4. Test harness includes assertions for each fixed deviation that prevent regression
**Plans**: TBD

Plans:
- [ ] 24-01: TBD
- [ ] 24-02: TBD

### Phase 25: NAPI Crate
**Goal**: A `qwik-napi-oxc` crate exists that exposes `transform_modules` to Node.js with the same calling convention as the SWC NAPI binding
**Depends on**: Phase 24 (fixes should land before building integration layer)
**Requirements**: NAPI-01, NAPI-02, NAPI-03
**Success Criteria** (what must be TRUE):
  1. `qwik-napi-oxc` crate builds as a cdylib with napi-rs v2 and produces a loadable `.node` native module
  2. `transform_modules` is callable from Node.js with the same function name and argument shape as SWC's `qwik_napi` binding
  3. All input types (`TransformModulesOptions`) and output types (`TransformOutput`, `SegmentAnalysis`) serialize with camelCase naming matching the SWC wire format
  4. `platform.ts` (Qwik's binding loader) can load the OXC `.node` file without code changes to the TypeScript layer
**Plans**: TBD

Plans:
- [ ] 25-01: TBD
- [ ] 25-02: TBD

### Phase 26: Integration Validation
**Goal**: The NAPI binding is verified to produce identical results to the direct Rust API across all 162 specs
**Depends on**: Phase 24 (fixes complete), Phase 25 (NAPI crate complete)
**Requirements**: INTG-01, INTG-02
**Success Criteria** (what must be TRUE):
  1. A Node.js test script calls `transform_modules` through the NAPI binding for all 162 spec inputs and collects results
  2. NAPI round-trip output (JS -> NAPI -> Rust -> NAPI -> JS) matches direct Rust API output for every spec (zero serialization drift)
  3. The test script can be run as a single command and reports pass/fail for each spec
**Plans**: TBD

Plans:
- [ ] 26-01: TBD
- [ ] 26-02: TBD

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. OXC AST Utility | v1.0 | 1/1 | Complete | 2026-02-10 |
| 2. Generate All Spec Files | v1.0 | 6/6 | Complete | 2026-02-10 |
| 3. Verify Completeness | v1.0 | 2/2 | Complete | 2026-02-10 |
| 4. Core API Mapping & Architecture | v2.0 | 2/2 | Complete | 2026-02-10 |
| 5. Deep Research & Proof of Concept | v2.0 | 3/3 | Complete | 2026-02-10 |
| 6. Secondary Patterns & Cross-Reference | v2.0 | 3/3 | Complete | 2026-02-11 |
| 7. Crate Foundation + Test Harness | v3.0 | 2/2 | Complete | 2026-02-11 |
| 8. Core Detection + QRL Transforms | v3.0 | 2/2 | Complete | 2026-02-10 |
| 9. Capture Analysis + Props Destructuring | v3.0 | 2/2 | Complete | 2026-02-10 |
| 10. Segment Extraction + Codegen | v3.0 | 2/2 | Complete | 2026-02-11 |
| 11. JSX + Signal Transforms | v3.0 | 2/2 | Complete | 2026-02-11 |
| 12. Annotations + Stripping | v3.0 | 2/2 | Complete | 2026-02-11 |
| 13. Source Maps + Full Validation | v3.0 | 4/4 | Complete | 2026-02-11 |
| 14. Style Cleanup | v4.0 | 2/2 | Complete | 2026-02-11 |
| 15. Dead Code Removal | v4.0 | 2/2 | Complete | 2026-02-11 |
| 16. Targeted Fixes | v4.0 | 1/1 | Complete | 2026-02-11 |
| 17. Extract JSX Transform | v4.0 | 1/1 | Complete | 2026-02-11 |
| 18. const_replace VisitMut Rewrite | v4.0 | 1/1 | Complete | 2026-02-11 |
| 19. Spec Compliance Verification | v4.0 | 1/1 | Complete | 2026-02-11 |
| 20. Path Resolution | v5.0 | 1/1 | Complete | 2026-02-12 |
| 21. Import Correctness | v5.0 | 1/1 | Complete | 2026-02-12 |
| 22. Display Names and Annotations | v5.0 | 1/1 | Complete | 2026-02-12 |
| 23. Output Audit | v6.0 | 2/2 | Complete | 2026-02-11 |
| 24. Runtime Bug Fixes | v6.0 | 0/TBD | Not started | - |
| 25. NAPI Crate | v6.0 | 0/TBD | Not started | - |
| 26. Integration Validation | v6.0 | 0/TBD | Not started | - |

---
*Roadmap created: 2026-02-10 (v1.0)*
*Last updated: 2026-02-11 (Phase 23 complete)*
