# Requirements: Qwik Optimizer — OXC Port

**Defined:** 2026-02-11
**Core Value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests

## v4.0 Requirements

Requirements for the code quality refactoring milestone. Each maps to roadmap phases.

### Structure

- [ ] **STRUCT-01**: JSX transformation code extracted from transform.rs into jsx_transform.rs (~1,350 lines)
- [ ] **STRUCT-02**: const_replace.rs rewritten using OXC VisitMut, eliminating manual AST walking boilerplate (~750 line reduction)

### Bug Fixes

- [ ] **BUG-01**: minify_expression_string correctly preserves spaces between identifier characters

### Dead Code

- [ ] **DEAD-01**: errors.rs #![allow(unused)] removed and unused factory functions deleted or used
- [ ] **DEAD-02**: words.rs duplicate constants (BUILDER_IO_QWIK == QWIK_CORE_ID) consolidated
- [ ] **DEAD-03**: collector.rs near-duplicate binding-name collection functions consolidated

### Performance

- [ ] **PERF-01**: KNOWN_GLOBALS uses HashSet for O(1) lookup instead of linear scan

### Style

- [ ] **STYLE-01**: Unnecessary comments stripped throughout the crate
- [ ] **STYLE-02**: Early returns added to flatten nesting where applicable
- [ ] **STYLE-03**: General formatting cleanup (consistent patterns, remove dead branches in match arms)

### Spec Compliance

- [ ] **SPEC-01**: Spec compliance maintained at 157/162 or improved

## Future Requirements

### Integration

- **INTEG-01**: Vite/Rollup plugin integration layer
- **INTEG-02**: Performance benchmarking against SWC optimizer
- **INTEG-03**: Close remaining 5/162 spec gaps (3 parser, 2 out-of-scope)

## Out of Scope

| Feature | Reason |
|---------|--------|
| code_move.rs rewrite to AST construction | Working correctly, parse+codegen roundtrip normalizes output, high risk for marginal gain |
| props_destructuring.rs VisitMut rewrite | Needs TraverseCtx for AST construction which VisitMut doesn't provide |
| argument_to_expression boilerplate removal | Forced by OXC's inherit_variants! macro, no alternative |
| DefaultHasher replacement | Hashes computed per-build, never persisted, toolchain stability irrelevant |
| replace_identifier_in_code string-awareness | Only operates on OXC codegen output (no comments, normalized strings) |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| STRUCT-01 | — | Pending |
| STRUCT-02 | — | Pending |
| BUG-01 | — | Pending |
| DEAD-01 | — | Pending |
| DEAD-02 | — | Pending |
| DEAD-03 | — | Pending |
| PERF-01 | — | Pending |
| STYLE-01 | — | Pending |
| STYLE-02 | — | Pending |
| STYLE-03 | — | Pending |
| SPEC-01 | — | Pending |

**Coverage:**
- v4.0 requirements: 11 total
- Mapped to phases: 0
- Unmapped: 11

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-11 after initial definition*
