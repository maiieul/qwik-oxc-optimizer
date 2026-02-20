# Roadmap: Qwik Optimizer OXC Port

## Overview

This roadmap drives the OXC optimizer from 160/162 snapshot diffs to zero, achieving full parity with the SWC golden reference. The 6 phases follow an initial cascade hypothesis: naming fixes clear the most diff noise first, metadata is simple plumbing, bugs must be fixed before features (missing segments block testing transforms inside them), features fix their own missing imports as side effects, JSX is localized, and import ordering is a clean final pass once all correct imports exist.

**Adaptive replanning:** After each phase completes, reassess the remaining diff landscape. The phase ordering is a starting hypothesis — real diffs may reveal that some later-phase work is trivially fixable earlier, or that phases are entangled differently than expected. Reorder, merge, or split remaining phases based on what the snapshot diffs actually show after each phase lands.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Naming & Display Names** - Fix segment naming to clear cascading diffs across 120+ snapshots
- [x] **Phase 2: Metadata** - Add paramNames and fix path field handling across 90+ snapshots
- [x] **Phase 3: Bugs & Correctness** - Fix TS stripping, missing segments, captures, component options, comments
- [x] **Phase 4: Signal & Props Transforms** - Implement _fnSignal, _wrapProp, props destructuring, QRL hoisting
- [ ] **Phase 5: JSX Keys & Flags** - Fix key generation and immutability flag values
- [ ] **Phase 6: Import Ordering & Cleanup** - Sort imports, merge specifiers, fix paths as final cleanup pass

## Phase Details

### Phase 1: Naming & Display Names
**Goal**: Segment names, display names, and parent fields match SWC output exactly, clearing the most pervasive diff noise across the entire snapshot suite
**Depends on**: Nothing (first phase)
**Requirements**: NAME-01, NAME-02, NAME-03
**Success Criteria** (what must be TRUE):
  1. Event handler segments use original attribute names (e.g. `div_onClick`) not transformed names (`div_q_e_click`) in all snapshot diffs
  2. Display names include all intermediate scope elements (no dropped parent elements like missing `div` between `App_component` and `button`)
  3. Parent field in segment metadata uses segment name with hash format (e.g. `renderHeader_XXXXXXXXXXXX`) not display name string (e.g. `test.tsx_renderHeader`)
  4. Segment filenames derived from corrected names match SWC filenames across all affected snapshots
**Plans**: 2 plans

Plans:
- [x] 01-01-PLAN.md — Port SWC stack_ctxt naming architecture to OXC (escape_sym, segment_stack, dedup counter, JSX event naming)
- [x] 01-02-PLAN.md — Fix default export naming, verify hash computation, clean up dead collector code, comprehensive verification

### Phase 2: Metadata
**Goal**: Segment metadata blocks in all output modules match SWC structure exactly (paramNames present, path fields populated correctly)
**Depends on**: Phase 1 (segment names must be correct before metadata references them)
**Requirements**: META-01, META-02
**Success Criteria** (what must be TRUE):
  1. Every segment's metadata JSON includes a `paramNames` array listing the segment function's parameter names
  2. The `path` field in segment metadata is populated with the file path (not empty), and `canonicalFilename` does not contain the path prefix that belongs in `path`
  3. Metadata-related diff lines are eliminated across the ~90 affected snapshots
**Plans**: 1 plan

Plans:
- [x] 02-01-PLAN.md — Implement paramNames extraction for $() calls and JSX event handlers, fix rel_dir backslash normalization

### Phase 3: Bugs & Correctness
**Goal**: All correctness bugs are fixed -- TypeScript types stripped, all segments extracted, capture ordering correct, component options preserved, comments retained
**Depends on**: Phase 2 (correct names and metadata needed to verify segment output)
**Requirements**: BUG-01, BUG-02, BUG-03, BUG-04, BUG-05, BUG-06
**Success Criteria** (what must be TRUE):
  1. When `transpile_ts=true`, output contains no TypeScript type annotations (no `: Type` in parameter lists, no `interface`/`type` declarations in JS output)
  2. OXC produces the same number of segment files as SWC for every test case (no missing segments)
  3. Capture variable ordering matches SWC for cases where the same variables are captured (genuine ordering bugs fixed); capture content diffs caused by missing Phase 4 transforms (_wrapProp, _fnSignal, props destructuring, q:p injection) are deferred to Phase 4
  4. `componentQrl()` calls include the component options object as second argument when present (e.g. `{ tagName: "my-foo" }`)
  5. Source comments from the original input are preserved in output modules (not stripped)
**Plans**: 3 plans

Plans:
- [x] 03-01-PLAN.md — Fix component options dropping (BUG-02) and segment comment stripping (BUG-06)
- [x] 03-02-PLAN.md — Implement TypeScript type stripping via oxc_transformer (BUG-01)
- [x] 03-03-PLAN.md — Replace qwik_router_inline fixture (BUG-05), fix segment ordering (BUG-04), fix capture ordering bugs (BUG-03 scoped), triage remaining diffs

### Phase 4: Signal & Props Transforms
**Goal**: Signal reactivity wrappers, props destructuring, and QRL hoisting transforms produce output matching SWC exactly
**Depends on**: Phase 3 (all segments must exist and be correct before testing transforms inside them)
**Requirements**: SIG-01, SIG-02, PROP-01, QRL-01
**Success Criteria** (what must be TRUE):
  1. Derived signals in JSX children are wrapped with `_fnSignal(fn, [deps], "expression")` instead of bare expressions
  2. Signal prop access generates `_wrapProp(obj, "prop")` two-argument form instead of destructuring
  3. Inline component props are destructured via `_restProps` transform matching SWC's pattern
  4. QRL calls are hoisted to variable declarations (e.g. `const _ref = qrl(...)`) and referenced by variable, not inlined at each usage site
  5. Each implemented transform automatically adds its required imports (`_fnSignal`, `_wrapProp`, `_restProps` from `@builder.io/qwik`), reducing IMP-02 (missing imports) diffs as a side effect
**Plans**: 4 plans

Plans:
- [x] 04-01-PLAN.md — Extend JSX children signal wrapping for _wrapProp (named) + _fnSignal (complex reactive expressions)
- [x] 04-02-PLAN.md — Add loop tracking, q:p injection, QRL hoisting, event handler iteration variable transforms
- [x] 04-03-PLAN.md — Fix props destructuring: default values, skip cases, excluded_keys completeness
- [x] 04-04-PLAN.md — [Gap closure] Handle non-destructured (props) parameter for _wrapProp signal wrapping

### Phase 5: JSX Keys & Flags
**Goal**: JSX key values and immutability flags in _jsxSorted/_jsxSplit calls match SWC output exactly
**Depends on**: Phase 4 (signal transforms affect JSX output; must be stable first)
**Requirements**: JSX-01, JSX-02
**Success Criteria** (what must be TRUE):
  1. JSX key values match SWC (correct generated keys like `"u6_0"` instead of `null`, and `null` where SWC uses `null`)
  2. JSX immutability flags match SWC values (correct `0`, `1`, or `2` per element, not blanket `3`)
  3. Key and flag diffs eliminated across the ~80 affected snapshots
**Plans**: 3 plans

Plans:
- [x] 05-01-PLAN.md — Fix JSX key generation: compute key prefix from file hash, implement root_jsx_mode, emit keys only for root elements and component tags
- [x] 05-02-PLAN.md — Fix immutability flags: replace simplistic formula with static_listeners + static_subtree bitfield, track jsx_mutable through children processing
- [x] 05-03-PLAN.md — [Gap closure] Fix mutable propagation from child elements to parents, fix member expression immutability classification

### Phase 6: Import Ordering & Cleanup
**Goal**: Import statements in all output modules match SWC exactly in order, grouping, specifier merging, and path format -- the final cleanup pass to reach 0/162 diffs
**Depends on**: Phase 5 (all imports must exist before sorting; features in phases 3-5 add imports as side effects)
**Requirements**: IMP-01, IMP-02, IMP-03, IMP-04
**Success Criteria** (what must be TRUE):
  1. Import statements appear in the same order as SWC output (consistent sorting algorithm applied)
  2. No missing or extra imports remain in any output module (correct import set per module)
  3. Multiple imports from the same module are merged into a single import statement with combined specifiers
  4. Relative import paths match SWC format exactly (e.g. `./test.tsx_Header_...` not `./project/test.tsx_Header_...`)
  5. All 162 snapshot tests pass with zero diffs against the SWC golden reference
**Plans**: TBD

Plans:
- [ ] 06-01: TBD
- [ ] 06-02: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3 -> 4 -> 5 -> 6
(Subject to reassessment after each phase — see Overview)

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Naming & Display Names | 2/2 | Complete | 2026-02-19 |
| 2. Metadata | 1/1 | Complete | 2026-02-20 |
| 3. Bugs & Correctness | 3/3 | Complete | 2026-02-20 |
| 4. Signal & Props Transforms | 4/4 | Complete | 2026-02-20 |
| 5. JSX Keys & Flags | 3/3 | Complete (gaps remain) | 2026-02-20 |
| 6. Import Ordering & Cleanup | 0/2 | Not started | - |
