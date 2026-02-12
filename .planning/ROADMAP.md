# Roadmap: Qwik Optimizer SWC-to-OXC Port

## Milestones

- v1.0 Spec Generation -- Phases 1-3 (shipped 2026-02-10)
- v2.0 OXC API Research & Architecture -- Phases 4-6 (shipped 2026-02-11)
- v3.0 OXC Optimizer Port -- Phases 7-13 (shipped 2026-02-11)
- v4.0 Code Quality Refactor -- Phases 14-19 (shipped 2026-02-11)
- v5.0 Drop-in Replacement Compliance -- Phases 20-22 (in progress)

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

### v5.0 Drop-in Replacement Compliance (In Progress)

**Milestone Goal:** Fix all output compatibility issues so the OXC optimizer can replace the SWC optimizer at runtime.

- [x] **Phase 20: Path Resolution** - Fix canonical filenames, explicit extensions, and output file extensions (completed 2026-02-12)
- [x] **Phase 21: Import Correctness** - Strip consumed $-imports and scope Qrl-suffixed imports to correct modules (completed 2026-02-12)
- [ ] **Phase 22: Display Names and Annotations** - Fix nested segment display names and PURE annotation placement

## Phase Details

### Phase 20: Path Resolution
**Goal**: Segment file paths and import paths match what downstream consumers expect
**Depends on**: Nothing (first v5.0 phase, all path fixes are independent of import/naming fixes)
**Requirements**: PATH-01, PATH-02, PATH-03, PATH-04
**Success Criteria** (what must be TRUE):
  1. Segment filenames contain the source file extension in their origin prefix (e.g., `test.tsx_Header_component_HASH` not `test_Header_component_HASH`)
  2. Lazy `import()` paths in main module output match the actual segment filenames (no mismatch between import path and file)
  3. When `explicit_extensions: true`, lazy import paths end with the file extension (e.g., `import("./seg.tsx")` not `import("./seg")`)
  4. When both `transpile_ts` and `transpile_jsx` are true, output segment files have `.js` extension (not `.jsx`)
**Plans**: 1 plan

Plans:
- [x] 20-01-PLAN.md -- Fix path resolution logic and add tests (PATH-01 through PATH-04)

### Phase 21: Import Correctness
**Goal**: Main module and segment modules contain only the imports they actually need
**Depends on**: Phase 20 (path resolution must be correct before verifying import paths in output)
**Requirements**: IMPORT-01, IMPORT-02
**Success Criteria** (what must be TRUE):
  1. After transformation, consumed `$`-suffixed imports (e.g., `component$`, `useTask$`) do not appear in the main module output
  2. Qrl-suffixed runtime imports (e.g., `useStylesQrl`) appear only in modules where they are actually called, not in the main module when only segments reference them
  3. Main module retains non-consumed imports (imports not related to `$()` extraction remain untouched)
**Plans**: 1 plan

Plans:
- [x] 21-01-PLAN.md -- Fix import stripping and scoping (IMPORT-01 + IMPORT-02)

### Phase 22: Display Names and Annotations
**Goal**: Segment metadata and tree-shaking annotations match SWC optimizer behavior
**Depends on**: Phase 20 (correct segment filenames needed for full validation context)
**Requirements**: NAME-01, PURE-01
**Success Criteria** (what must be TRUE):
  1. Nested segment display names include the full parent context hierarchy (e.g., `App_component_div_onClick` for a click handler inside JSX inside a `component$`)
  2. `/* @__PURE__ */` annotations appear on tree-shakeable calls (`qrl()`, `componentQrl()`, `_jsxSorted()`)
  3. `/* @__PURE__ */` annotations do NOT appear on side-effectful calls (`useStylesQrl()`, `useTaskQrl()`, and similar runtime-invoked hooks)
**Plans**: TBD

Plans:
- [ ] 22-01: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 20 -> 21 -> 22

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
| 22. Display Names and Annotations | v5.0 | 0/? | Not started | - |

---
*Roadmap created: 2026-02-10 (v1.0)*
*Last updated: 2026-02-12 (v5.0 Phase 21 complete)*
