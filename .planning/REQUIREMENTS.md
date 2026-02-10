# Requirements: Qwik Optimizer Spec Generation

**Defined:** 2026-02-10
**Core Value:** A complete, SWC-independent behavioral specification of every optimizer transformation so the OXC port can be built from spec

## v1 Requirements

Requirements for 162 spec files documenting every SWC snapshot test. Each maps to roadmap phases.

### Infrastructure

- [ ] **INFRA-01**: Small Rust utility crate exists with `oxc` (serialize feature), `serde`, `serde_json` to parse code and emit AST JSON
- [ ] **INFRA-02**: Utility can accept a code string + file extension, parse with oxc_parser, and output serde JSON AST

### Spec Files

- [ ] **SPEC-01**: One markdown file per snapshot exists at `.planning/spec/<test_name>.md` (162 files total)
- [ ] **SPEC-02**: Each spec file includes test configuration (entry strategy, emit mode, transpile flags, strip options, etc.)
- [ ] **SPEC-03**: Each spec file includes the input source code in a fenced code block
- [ ] **SPEC-04**: Each spec file includes the input code's oxc_parser AST (serde JSON) in a collapsible `<details>` block
- [ ] **SPEC-05**: Each spec file includes each output module's code in a fenced code block with path and entry point flag
- [ ] **SPEC-06**: Each spec file includes each output module's oxc_parser AST (serde JSON) in a collapsible `<details>` block
- [ ] **SPEC-07**: Each spec file includes segment metadata JSON (SegmentAnalysis) for each extracted segment
- [ ] **SPEC-08**: Each spec file includes a "Conventions Applied" section listing every transformation convention observed
- [ ] **SPEC-09**: Each spec file includes a "Function Calls in Output" section cataloging every generated function call (name, module, count)
- [ ] **SPEC-10**: Each spec file includes diagnostics (or "None")

### Conventions Coverage

- [ ] **CONV-01**: Specs identify `qrl()` / `qrlDEV()` / `inlinedQrl()` calls
- [ ] **CONV-02**: Specs identify `component$` → `componentQrl` and other `foo$` → `fooQrl` conversions
- [ ] **CONV-03**: Specs identify JSX transformation calls (`_jsxSorted`, `_jsxSplit`, `_jsxQ`)
- [ ] **CONV-04**: Specs identify signal/reactivity helpers (`_wrapProp`, `_wrapSignal`, `_fnSignal`, `_getVarProps`, `_getConstProps`)
- [ ] **CONV-05**: Specs identify capture patterns (`_captures[N]` references, capture arrays in qrl 3rd arg)
- [ ] **CONV-06**: Specs identify lazy import generation (`const i_HASH = () => import("./...")`)
- [ ] **CONV-07**: Specs identify `#__PURE__` annotations
- [ ] **CONV-08**: Specs identify segment extraction (code moved to separate output module)
- [ ] **CONV-09**: Specs identify code stripping patterns (`strip_exports` stubs, `_noopQrl`)
- [ ] **CONV-10**: Specs identify const replacement (`isServer`/`isBrowser`/`isDev` → literal)
- [ ] **CONV-11**: Specs identify props destructuring optimization (`_rawProps`, `_restProps`)
- [ ] **CONV-12**: Specs identify input binding transformation (`bind:value`, `bind:checked`)
- [ ] **CONV-13**: Specs identify `sync$` serialization patterns
- [ ] **CONV-14**: Specs identify hoisted functions (`_hf` pattern for derived signals)

### Quality

- [ ] **QUAL-01**: Spec files are human-readable without expanding AST details blocks
- [ ] **QUAL-02**: All conventions present in each snapshot are documented (no false negatives)
- [ ] **QUAL-03**: oxc_parser AST generation succeeds for all parseable code (parse errors noted for intentionally broken test cases)
- [ ] **QUAL-04**: All 162 spec files are complete and consistent in structure

## v2 Requirements

Deferred to future release. Tracked but not in current roadmap.

### Enhanced Documentation

- **ENH-01**: Mode matrix table showing which transforms apply in which mode combinations
- **ENH-02**: Snapshot tier classification (semantic contract vs structural equivalence vs incidental)
- **ENH-03**: Cross-reference index linking all spec files with convention summaries

### Port Support

- **PORT-01**: Abstract `ResolvedIdentifier` concept in specs (decoupled from SWC SyntaxContext)
- **PORT-02**: Traversal order annotations on each transformation rule
- **PORT-03**: OXC `AstBuilder` mapping guide for each AST construction pattern

## Out of Scope

| Feature | Reason |
|---------|--------|
| Building the OXC optimizer | Future milestone — this milestone is spec-only |
| Modifying SWC code | Read-only against existing codebase |
| Source map content in specs | Too volatile — note existence only |
| Exact hash value documentation | Hashes depend on content; spec documents hashing properties not values |
| TypeScript plugin layer changes | TS/Vite/Rollup plugins untouched in this milestone |
| Standalone CLI tool | Claude generates specs directly; small Rust utility only for AST generation |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| INFRA-01 | — | Pending |
| INFRA-02 | — | Pending |
| SPEC-01 | — | Pending |
| SPEC-02 | — | Pending |
| SPEC-03 | — | Pending |
| SPEC-04 | — | Pending |
| SPEC-05 | — | Pending |
| SPEC-06 | — | Pending |
| SPEC-07 | — | Pending |
| SPEC-08 | — | Pending |
| SPEC-09 | — | Pending |
| SPEC-10 | — | Pending |
| CONV-01 | — | Pending |
| CONV-02 | — | Pending |
| CONV-03 | — | Pending |
| CONV-04 | — | Pending |
| CONV-05 | — | Pending |
| CONV-06 | — | Pending |
| CONV-07 | — | Pending |
| CONV-08 | — | Pending |
| CONV-09 | — | Pending |
| CONV-10 | — | Pending |
| CONV-11 | — | Pending |
| CONV-12 | — | Pending |
| CONV-13 | — | Pending |
| CONV-14 | — | Pending |
| QUAL-01 | — | Pending |
| QUAL-02 | — | Pending |
| QUAL-03 | — | Pending |
| QUAL-04 | — | Pending |

**Coverage:**
- v1 requirements: 30 total
- Mapped to phases: 0
- Unmapped: 30

---
*Requirements defined: 2026-02-10*
*Last updated: 2026-02-10 after adjustment — removed CLI tool, simplified to direct spec generation*
