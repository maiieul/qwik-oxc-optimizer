# Requirements: Qwik Optimizer OXC Port

**Defined:** 2026-02-19
**Core Value:** Snapshot parity with the SWC optimizer across all 162 test cases

## v1 Requirements

Requirements for milestone v1.0 — Full Snapshot Parity. Each maps to roadmap phases.

### Naming

- [ ] **NAME-01**: Segment names use original attribute names for event handlers (`div_onClick` not `div_q_e_click`)
- [ ] **NAME-02**: Display names include all intermediate scope elements (no dropped context)
- [ ] **NAME-03**: Parent field uses segment name with hash format (not display name string)

### Metadata

- [ ] **META-01**: Segment metadata includes `paramNames` array
- [ ] **META-02**: Path field handling matches SWC (`path` field populated, not `canonicalFilename` prefix)

### Bugs & Correctness

- [ ] **BUG-01**: TypeScript type annotations stripped when `transpile_ts=true`
- [ ] **BUG-02**: Component options object preserved in `componentQrl()` second argument
- [ ] **BUG-03**: Captured variables match SWC (correct vars, correct order)
- [ ] **BUG-04**: All segments extracted (no missing segment files)
- [ ] **BUG-05**: `qwik_router_inline` complex transformation handled correctly
- [ ] **BUG-06**: Source comments preserved in output

### Signal & Props Transforms

- [ ] **SIG-01**: `_fnSignal` wrapper emitted for derived signals in JSX children
- [ ] **SIG-02**: `_wrapProp(obj, "prop")` 2-argument form generated
- [ ] **PROP-01**: Inline component props destructuring and `_restProps` transformed
- [ ] **QRL-01**: QRL calls hoisted to variables (not inlined at usage site)

### JSX

- [ ] **JSX-01**: JSX key generation matches SWC values
- [ ] **JSX-02**: JSX immutability flags match SWC values

### Imports

- [ ] **IMP-01**: Import statement ordering matches SWC
- [ ] **IMP-02**: No missing or extra imports (correct import set per module)
- [ ] **IMP-03**: Import specifiers from same module merged into single import
- [ ] **IMP-04**: Relative import paths match SWC format

## v2 Requirements

Deferred to future milestones. Tracked but not in current roadmap.

### Diagnostics

- **DIAG-01**: Custom QWIK(x) error/warning messages emitted for invalid patterns
- **DIAG-02**: Source locations included in diagnostic output

### Performance

- **PERF-01**: Parallel file transformation via rayon
- **PERF-02**: Benchmark suite comparing OXC vs SWC optimizer throughput

### Transpilation

- **TRANS-01**: Full TS/JSX transpilation via OXC transformer (beyond type stripping)

## Out of Scope

| Feature | Reason |
|---------|--------|
| Entry strategy variations beyond snapshot coverage | Match SWC behavior only; novel strategies are v2+ |
| Source map accuracy validation | Snapshot tests don't cover source maps; defer to integration testing |
| NAPI/Node.js bindings | Infrastructure concern, not optimizer correctness |
| Qwik v2 optimizer changes | This port targets current SWC behavior |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| NAME-01 | — | Pending |
| NAME-02 | — | Pending |
| NAME-03 | — | Pending |
| META-01 | — | Pending |
| META-02 | — | Pending |
| BUG-01 | — | Pending |
| BUG-02 | — | Pending |
| BUG-03 | — | Pending |
| BUG-04 | — | Pending |
| BUG-05 | — | Pending |
| BUG-06 | — | Pending |
| SIG-01 | — | Pending |
| SIG-02 | — | Pending |
| PROP-01 | — | Pending |
| QRL-01 | — | Pending |
| JSX-01 | — | Pending |
| JSX-02 | — | Pending |
| IMP-01 | — | Pending |
| IMP-02 | — | Pending |
| IMP-03 | — | Pending |
| IMP-04 | — | Pending |

**Coverage:**
- v1 requirements: 21 total
- Mapped to phases: 0
- Unmapped: 21 ⚠️

---
*Requirements defined: 2026-02-19*
*Last updated: 2026-02-19 after initial definition*
