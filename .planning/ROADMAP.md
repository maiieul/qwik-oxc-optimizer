# Roadmap: Qwik Optimizer SWC-to-OXC Port

## Milestones

- v1.0 Spec Generation -- Phases 1-3 (shipped 2026-02-10)
- v2.0 OXC API Research & Architecture -- Phases 4-6 (in progress)

## Phases

<details>
<summary>v1.0 Spec Generation (Phases 1-3) -- SHIPPED 2026-02-10</summary>

- [x] Phase 1: OXC AST Utility (1/1 plan) -- completed 2026-02-10
- [x] Phase 2: Generate All Spec Files (6/6 plans) -- completed 2026-02-10
- [x] Phase 3: Verify Completeness (2/2 plans) -- completed 2026-02-10

Full details: `milestones/v1.0-ROADMAP.md`

</details>

### v2.0 OXC API Research & Architecture (In Progress)

**Milestone Goal:** Deeply research OXC APIs and Rust libraries to map every spec transformation pattern to concrete implementation approaches, producing API mapping guides, working proof-of-concept Rust programs, and an architectural blueprint -- so the port milestone (v3.0) can proceed with zero guesswork.

- [x] **Phase 4: Core API Mapping & Architecture** - Map foundational transformation patterns to OXC APIs and design the optimizer crate architecture -- completed 2026-02-10
- [x] **Phase 5: Deep Research & Proof of Concept** - Investigate complex patterns (capture analysis, multi-module output) and validate with working Rust programs -- completed 2026-02-10
- [ ] **Phase 6: Secondary Patterns & Cross-Reference** - Map remaining transformation patterns (JSX, source maps, signals, entry strategy) and produce complete CONV cross-reference

## Phase Details

### Phase 4: Core API Mapping & Architecture
**Goal**: Developer has concrete OXC API examples for the three foundational transformation patterns ($-extraction, qrl wrapping, import rewriting) and a complete architectural blueprint for the optimizer crate
**Depends on**: v1.0 (162 spec files as behavioral ground truth)
**Requirements**: APIM-01, APIM-02, APIM-05, ARCH-01, ARCH-02, ARCH-03, ARCH-04, ARCH-05
**Success Criteria** (what must be TRUE):
  1. A document exists showing exactly which OXC Traverse + AstBuilder APIs detect and extract `$()` call sites, with Rust code examples that reference real spec inputs
  2. A document exists showing how to construct `qrl()` / `inlinedQrl()` wrapper expressions using OXC AstBuilder, with code examples mapping to spec output patterns
  3. A document exists showing how to rewrite imports (remove `component$`, add `componentQrl`) using OXC statement mutation APIs, with before/after examples
  4. A complete crate module layout exists with dependency ordering, public API design (TransformModulesOptions, TransformOutput, SegmentAnalysis), data flow specification with type signatures, test harness design, and Cargo.toml specification
**Plans**: 2 plans

Plans:
- [x] 04-01-PLAN.md -- OXC API mapping guide for $-extraction, qrl wrapping, and import rewriting (APIM-01, APIM-02, APIM-05)
- [x] 04-02-PLAN.md -- Architecture blueprint: module layout, public API, data flow, test harness, Cargo.toml (ARCH-01 through ARCH-05)

### Phase 5: Deep Research & Proof of Concept
**Goal**: The hardest research questions (capture analysis, multi-module output) are answered with working Rust programs that run against real spec files and produce correct results
**Depends on**: Phase 4 (architecture provides structural context for POCs)
**Requirements**: APIM-03, APIM-06, POC-01, POC-02, POC-03, POC-04
**Success Criteria** (what must be TRUE):
  1. A document exists mapping the capture analysis algorithm to oxc_semantic Scoping APIs (find_binding, get_resolved_references, scope_ancestors) with examples covering all 8 edge cases
  2. A document exists mapping the multi-module output pattern to AstBuilder Program construction with a concrete allocator strategy (shared vs separate) and code examples
  3. A Rust program compiles and runs, using OXC Traverse to detect `$()` call sites in spec input files and reporting correct locations
  4. A Rust program compiles and runs, using oxc_semantic to perform capture analysis against spec files with known capture lists and producing matching results
  5. A Rust program compiles and runs, splitting one input Program into a main module + segment module(s), producing valid JavaScript output for both via oxc_codegen with source maps
**Plans**: 3 plans

Plans:
- [x] 05-01-PLAN.md -- Capture analysis and multi-module output API mapping documents (APIM-03, APIM-06)
- [x] 05-02-PLAN.md -- POC workspace setup, $() detection POC, and capture analysis POC (POC-01, POC-02)
- [x] 05-03-PLAN.md -- Multi-module output POC and source map generation POC (POC-03, POC-04)

### Phase 6: Secondary Patterns & Cross-Reference
**Goal**: Every remaining transformation pattern is mapped to OXC APIs, and all 14 CONV types have a complete cross-reference to specific API patterns -- achieving full coverage of the 162-spec surface area
**Depends on**: Phase 5 (capture analysis and multi-module patterns inform JSX and advanced feature mappings)
**Requirements**: APIM-04, APIM-07, APIM-08, APIM-09, APIM-10
**Success Criteria** (what must be TRUE):
  1. A document exists mapping JSX transformation patterns (_jsxSorted, _jsxSplit) to OXC expression replacement APIs with code examples
  2. A document exists mapping source map generation to oxc_codegen + oxc_sourcemap APIs with a span preservation strategy for split modules
  3. A cross-reference table exists mapping all 14 CONV types to their specific OXC API patterns, with every CONV type having at least one concrete API mapping
  4. Documents exist mapping props destructuring, signal optimization, entry strategy, code stripping, and const folding patterns to OXC APIs

**Plans**: 3 plans

Plans:
- [ ] 06-01-PLAN.md -- JSX transformation + props destructuring + signal optimization OXC API mapping (APIM-04, APIM-09)
- [ ] 06-02-PLAN.md -- Entry strategy, code stripping, const folding, input binding, sync$, dev mode OXC API mapping (APIM-10)
- [ ] 06-03-PLAN.md -- 14 CONV type cross-reference table + source map span strategy + PURE annotations (APIM-07, APIM-08)

## Progress

**Execution Order:**
Phases execute in numeric order: 4 -> 5 -> 6

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1. OXC AST Utility | v1.0 | 1/1 | Complete | 2026-02-10 |
| 2. Generate All Spec Files | v1.0 | 6/6 | Complete | 2026-02-10 |
| 3. Verify Completeness | v1.0 | 2/2 | Complete | 2026-02-10 |
| 4. Core API Mapping & Architecture | v2.0 | 2/2 | Complete | 2026-02-10 |
| 5. Deep Research & Proof of Concept | v2.0 | 3/3 | Complete | 2026-02-10 |
| 6. Secondary Patterns & Cross-Reference | v2.0 | 0/3 | Not started | - |

---
*Roadmap created: 2026-02-10 (v1.0)*
*Last updated: 2026-02-10 (Phase 6 planned)*
