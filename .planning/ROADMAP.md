# Roadmap: Qwik Optimizer SWC-to-OXC Port

## Milestones

- v1.0 Spec Generation -- Phases 1-3 (shipped 2026-02-10)
- v2.0 OXC API Research & Architecture -- Phases 4-6 (shipped 2026-02-11)
- v3.0 OXC Optimizer Port -- Phases 7-13 (in progress)

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

### v3.0 OXC Optimizer Port (In Progress)

**Milestone Goal:** Build a complete `qwik-optimizer-oxc` Rust crate implementing all 14 CONV transformation types, validated against the 162 behavioral spec files.

- [x] **Phase 7: Crate Foundation + Test Harness** - Compilable crate skeleton with spec-based test infrastructure -- completed 2026-02-11
- [x] **Phase 8: Core Detection + QRL Transforms** - Dollar call detection and QRL wrapping (CONV-01, CONV-02) -- completed 2026-02-10
- [ ] **Phase 9: Capture Analysis + Props Destructuring** - Scope analysis and props transformation (CONV-11, CONV-05)
- [ ] **Phase 10: Segment Extraction + Codegen** - Multi-module output and lazy imports (CONV-08, CONV-06)
- [ ] **Phase 11: JSX + Signal Transforms** - JSX compilation and signal optimization (CONV-03, CONV-04, CONV-12, CONV-14)
- [ ] **Phase 12: Annotations + Stripping** - PURE annotations, const replacement, code stripping, sync$ (CONV-07, CONV-10, CONV-09, CONV-13)
- [ ] **Phase 13: Source Maps + Full Validation** - Source map generation and 162/162 spec test pass

## Phase Details

### Phase 7: Crate Foundation + Test Harness
**Goal**: Crate compiles with all module stubs and test harness can parse spec files
**Depends on**: Nothing (first v3.0 phase)
**Requirements**: FOUN-01, FOUN-02, FOUN-03, FOUN-04, TEST-01
**Success Criteria** (what must be TRUE):
  1. `cargo build` succeeds for the `qwik-optimizer-oxc` crate with all 16 module files present
  2. `transform_modules()` accepts `TransformModulesOptions` and returns a `TransformOutput` (stub with empty modules/diagnostics)
  3. All public types (`TransformModulesOptions`, `TransformOutput`, `SegmentAnalysis`, etc.) serialize to/from JSON matching the SWC optimizer's wire format
  4. Test harness parses all 162 spec markdown files and extracts input code, expected output modules, and expected diagnostics without errors
**Plans**: 2 plans

Plans:
- [x] 07-01-PLAN.md -- Crate skeleton with workspace Cargo.toml, all 16 module stubs, public types, and stub transform_modules()
- [x] 07-02-PLAN.md -- Spec file parser and test harness validating all 162 specs parse correctly

### Phase 8: Core Detection + QRL Transforms
**Goal**: Optimizer detects all $() call sites and produces qrl()/inlinedQrl() replacements
**Depends on**: Phase 7
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04
**Success Criteria** (what must be TRUE):
  1. Optimizer identifies every `$`-suffixed call site imported from `@qwik.dev/core` in any input module
  2. `component$()` becomes `componentQrl(qrl(...))` and `useTask$()` becomes `useTaskQrl(qrl(...))` (dollar-to-QRL suffix renaming works for all known APIs)
  3. Segment entry strategy produces `qrl(i_HASH, "segment_name")` calls with correct hash and import identifier
  4. Inline/hoist entry strategy produces `inlinedQrl(fn_body, "name_HASH")` calls with function body preserved
**Plans**: 2 plans

Plans:
- [x] 08-01-PLAN.md -- Parse + semantic + collector pipeline (parse.rs, collector.rs, words.rs, hash.rs)
- [x] 08-02-PLAN.md -- QwikTransform traverse with $() detection and QRL wrapping (transform.rs, import_rewrite.rs, entry_strategy.rs, lib.rs)

### Phase 9: Capture Analysis + Props Destructuring
**Goal**: Optimizer correctly identifies captured variables and transforms props destructuring patterns
**Depends on**: Phase 8
**Requirements**: CAPT-01, CAPT-02, CAPT-03, CAPT-04, CAPT-05
**Success Criteria** (what must be TRUE):
  1. Component arrow functions with destructured props are rewritten to use `_rawProps` parameter with individual property access via `_rawProps.key`
  2. Rest patterns in destructured props produce `_restProps(_rawProps, ["key1", "key2", ...])` declarations
  3. Variables referenced inside a `$()` body but declared outside it are identified as captures and listed in the correct order
  4. Capture arrays appear as the third argument to `qrl()`/`inlinedQrl()` calls (e.g., `qrl(i_HASH, "name", [capturedVar1, capturedVar2])`)
  5. Extracted segment bodies contain `const varName = _captures[N];` restoration statements for each captured variable
**Plans**: TBD

Plans:
- [ ] 09-01: Props destructuring transform (props_destructuring.rs, CONV-11)
- [ ] 09-02: Capture analysis via oxc_semantic scoping (CONV-05, capture detection + array generation + restoration)

### Phase 10: Segment Extraction + Codegen
**Goal**: Optimizer produces separate module files for extracted segments with correct imports, exports, and lazy loading declarations
**Depends on**: Phase 9
**Requirements**: SEGM-01, SEGM-02, SEGM-03, SEGM-04, EMIT-01, EMIT-03
**Success Criteria** (what must be TRUE):
  1. Each `$()` body is extracted into a standalone `Program` with correct `export const NAME_HASH = ...` and necessary imports from `@qwik.dev/core`
  2. Main module contains lazy import declarations (`const i_HASH = () => import("./path")`) for every extracted segment
  3. `SegmentAnalysis` metadata is produced for each segment with correct hash, canonicalFilename, displayName, origin, ctxKind, ctxName, and captures fields
  4. All entry strategies (Segment, Inline, Hoist, Single, Component, Smart, Hook) produce the correct output shape
  5. OXC codegen emits valid JavaScript for both main modules and extracted segments, with correct import declarations for all framework functions used
**Plans**: TBD

Plans:
- [ ] 10-01: Segment Program construction (code_move.rs, CONV-08) and lazy import generation (CONV-06)
- [ ] 10-02: Code emission pipeline (emit.rs, filter_exports.rs) and entry strategy variants (SEGM-04)

### Phase 11: JSX + Signal Transforms
**Goal**: Optimizer transforms JSX elements to _jsxSorted/_jsxSplit calls with signal optimization and hoisted helpers
**Depends on**: Phase 10
**Requirements**: JSX-01, JSX-02, JSX-03, JSX-04, JSX-05, JSX-06, JSX-07, JSX-08
**Success Criteria** (what must be TRUE):
  1. JSX elements become `_jsxSorted(tag, varProps, constProps, children, flags, key)` calls with props correctly classified as const vs var
  2. JSX elements with spread attributes become `_jsxSplit(tag, props, children, flags, key)` calls
  3. `signal.value` in JSX props becomes `_wrapProp(signal)`, store property access becomes `_wrapProp(store, "prop")`, and computed expressions become `_fnSignal(_hfN, [deps], _hfN_str)` calls
  4. Hoisted `_hfN` function declarations and `_hfN_str` string constants are inserted at module top level
  5. `bind:value` and `bind:checked` JSX attributes produce the correct event handler QRLs with `_val`/`_chk` handlers
**Plans**: TBD

Plans:
- [ ] 11-01: JSX element transformation (_jsxSorted, _jsxSplit, fragments, prop classification)
- [ ] 11-02: Signal optimization (_wrapProp, _fnSignal, hoisted functions) and input binding (bind:value, bind:checked)

### Phase 12: Annotations + Stripping
**Goal**: Optimizer adds PURE annotations, replaces build constants, strips dead code, and handles sync$ serialization
**Depends on**: Phase 11
**Requirements**: ANNO-01, ANNO-02, ANNO-03, ANNO-04, ANNO-05
**Success Criteria** (what must be TRUE):
  1. Every framework replacement call (_jsxSorted, componentQrl, qrl, inlinedQrl, _wrapProp, _fnSignal, _qrlSync, _noopQrl, _restProps) has a `/*#__PURE__*/` annotation in codegen output
  2. `isServer`, `isDev`, `isBrowser` identifiers are replaced with boolean literals matching the build mode, and resulting dead branches (`if (false) { ... }`) are eliminated
  3. Stripped `$()` calls (matching `strip_ctx_name`) become `_noopQrl("s_HASH")` in prod mode
  4. `sync$()` calls become `_qrlSync(fn, "minified_fn_string")` with the function body serialized as a minified string
**Plans**: TBD

Plans:
- [ ] 12-01: PURE annotation mechanism and const replacement with dead branch elimination (is_const.rs, const_replace.rs, CONV-07, CONV-10)
- [ ] 12-02: Code stripping (_noopQrl) and sync$ serialization (_qrlSync) (CONV-09, CONV-13)

### Phase 13: Source Maps + Full Validation
**Goal**: All 162 spec tests pass with source map generation
**Depends on**: Phase 12
**Requirements**: EMIT-02, TEST-02, TEST-03
**Success Criteria** (what must be TRUE):
  1. Optimizer generates source maps for all output modules (main and segments) that map generated positions back to original source locations
  2. Test harness runs the optimizer on all 162 spec inputs and compares output semantically against spec expectations
  3. All 162 spec tests pass -- every module path, segment metadata field, and diagnostic matches the behavioral specification
**Plans**: TBD

Plans:
- [ ] 13-01: Source map generation for main and segment modules (EMIT-02)
- [ ] 13-02: Full 162-spec validation pass and fix remaining failures (TEST-02, TEST-03)

## Progress

**Execution Order:**
Phases execute in numeric order: 7 -> 8 -> 9 -> 10 -> 11 -> 12 -> 13

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
| 9. Capture Analysis + Props Destructuring | v3.0 | 0/2 | Not started | - |
| 10. Segment Extraction + Codegen | v3.0 | 0/2 | Not started | - |
| 11. JSX + Signal Transforms | v3.0 | 0/2 | Not started | - |
| 12. Annotations + Stripping | v3.0 | 0/2 | Not started | - |
| 13. Source Maps + Full Validation | v3.0 | 0/2 | Not started | - |

---
*Roadmap created: 2026-02-10 (v1.0)*
*Last updated: 2026-02-10 (Phase 8 complete)*
