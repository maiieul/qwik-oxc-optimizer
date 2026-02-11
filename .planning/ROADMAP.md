# Roadmap: Qwik Optimizer SWC-to-OXC Port

## Milestones

- v1.0 Spec Generation -- Phases 1-3 (shipped 2026-02-10)
- v2.0 OXC API Research & Architecture -- Phases 4-6 (shipped 2026-02-11)
- v3.0 OXC Optimizer Port -- Phases 7-13 (shipped 2026-02-11)
- v4.0 Code Quality Refactor -- Phases 14-19 (in progress)

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

### v4.0 Code Quality Refactor (In Progress)

**Milestone Goal:** Refactor the qwik-optimizer-oxc crate for maintainability -- clean style, remove dead code, fix bugs, extract modules, rewrite boilerplate -- while maintaining or improving 157/162 spec compliance.

- [x] **Phase 14: Style Cleanup** - Strip unnecessary comments, add early returns, normalize formatting across the crate -- completed 2026-02-11
- [x] **Phase 15: Dead Code Removal** - Eliminate unused error factories, consolidate duplicate constants and functions -- completed 2026-02-11
- [x] **Phase 16: Targeted Fixes** - Fix minify_expression_string bug and convert KNOWN_GLOBALS to HashSet -- completed 2026-02-11
- [x] **Phase 17: Extract JSX Transform** - Move JSX transformation code from transform.rs into its own module -- completed 2026-02-11
- [x] **Phase 18: const_replace VisitMut Rewrite** - Replace manual AST walking with OXC VisitMut pattern -- completed 2026-02-11
- [ ] **Phase 19: Spec Compliance Verification** - Confirm all changes preserve or improve 157/162 spec match

## Phase Details

### Phase 14: Style Cleanup
**Goal**: Codebase reads cleanly -- no stale comments, minimal nesting, consistent formatting patterns
**Depends on**: Phase 13 (v3.0 complete)
**Requirements**: STYLE-01, STYLE-02, STYLE-03
**Success Criteria** (what must be TRUE):
  1. No comments that merely restate what the code does (e.g., "// check if X" before `if x`) remain in the crate
  2. Functions with deeply nested if/else chains use early returns to keep the happy path at the top indentation level
  3. Match arms with unreachable dead branches are cleaned up, formatting is consistent across all 16 source files
  4. All 165 tests still pass after style changes (zero regressions)
**Plans**: 2 plans

Plans:
- [x] 14-01-PLAN.md -- Strip unnecessary comments and add early returns across 8 high-comment source files
- [x] 14-02-PLAN.md -- Normalize formatting and clean match arms across all 16 source files

### Phase 15: Dead Code Removal
**Goal**: No unused code, no duplicate definitions -- every function and constant earns its place
**Depends on**: Phase 14
**Requirements**: DEAD-01, DEAD-02, DEAD-03
**Success Criteria** (what must be TRUE):
  1. errors.rs has no `#![allow(unused)]` attribute, and every public function in errors.rs is called somewhere in the crate
  2. words.rs has a single constant for the Qwik core package identifier (no BUILDER_IO_QWIK / QWIK_CORE_ID duplication)
  3. collector.rs has one canonical function for collecting binding names from patterns (near-duplicates consolidated)
  4. `cargo build` produces no unused warnings without allow-unused attributes suppressing them
**Plans**: 2 plans

Plans:
- [x] 15-01-PLAN.md -- Delete unused functions/constants from errors.rs, words.rs, entry_strategy.rs, and 4 other source files
- [x] 15-02-PLAN.md -- Consolidate collector.rs binding-name functions and annotate types.rs data-model fields

### Phase 16: Targeted Fixes
**Goal**: Known bug fixed and known performance bottleneck addressed
**Depends on**: Phase 15
**Requirements**: BUG-01, PERF-01
**Success Criteria** (what must be TRUE):
  1. `minify_expression_string("a b")` preserves the space between identifiers (does not produce `"ab"`)
  2. KNOWN_GLOBALS is a `HashSet` (or equivalent O(1) lookup structure), not a linear-scan array/slice
  3. All 165 tests still pass after these changes
**Plans**: 1 plan

Plans:
- [x] 16-01-PLAN.md -- Fix minify_expression_string space bug and convert KNOWN_GLOBALS to LazyLock HashSet

### Phase 17: Extract JSX Transform
**Goal**: JSX transformation logic lives in its own module, and transform.rs is shorter and focused
**Depends on**: Phase 16
**Requirements**: STRUCT-01
**Success Criteria** (what must be TRUE):
  1. A new `jsx_transform.rs` module exists containing all JSX-specific transformation code (~1,350 lines extracted from transform.rs)
  2. transform.rs delegates to jsx_transform.rs for JSX node handling -- no JSX logic duplicated between the two
  3. All 165 tests still pass after extraction (pure refactor, zero behavior change)
**Plans**: 1 plan

Plans:
- [x] 17-01-PLAN.md -- Extract JSX transformation free functions into jsx_transform.rs and wire up delegation

### Phase 18: const_replace VisitMut Rewrite
**Goal**: const_replace.rs uses OXC's VisitMut pattern instead of manual recursive AST walking
**Depends on**: Phase 17
**Requirements**: STRUCT-02
**Success Criteria** (what must be TRUE):
  1. const_replace.rs implements `VisitMut` trait instead of manual match-and-recurse functions
  2. Net line count reduction of at least 500 lines compared to current const_replace.rs
  3. All 165 tests still pass -- identical transformation behavior to the manual implementation
  4. The `isServer`/`isBrowser`/`isDev` replacement and dead branch elimination produce the same output as before
**Plans**: 1 plan

Plans:
- [x] 18-01-PLAN.md -- Enable ast_visit feature and rewrite const_replace.rs with VisitMut (includes test validation)

### Phase 19: Spec Compliance Verification
**Goal**: All refactoring confirmed to preserve (or improve) spec compliance
**Depends on**: Phase 18
**Requirements**: SPEC-01
**Success Criteria** (what must be TRUE):
  1. Spec test harness reports 157/162 or better module count match
  2. All 250 metadata assertions pass
  3. No new test failures introduced compared to v3.0 baseline
**Plans**: 1 plan

Plans:
- [ ] 19-01-PLAN.md -- Full spec compliance run and regression comparison

## Progress

**Execution Order:**
Phases execute in numeric order: 14 -> 15 -> 16 -> 17 -> 18 -> 19

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
| 19. Spec Compliance Verification | v4.0 | 0/1 | Not started | - |

---
*Roadmap created: 2026-02-10 (v1.0)*
*Last updated: 2026-02-11 (Phase 18 complete)*
