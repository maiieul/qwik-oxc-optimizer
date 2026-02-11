# Requirements: Qwik Optimizer — SWC-to-OXC Port

**Defined:** 2026-02-10
**Core Value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests

## v3.0 Requirements

Requirements for the OXC optimizer port. Each maps to roadmap phases.

### Crate Foundation

- [ ] **FOUN-01**: Crate compiles with Cargo.toml matching architecture blueprint dependencies (oxc_parser, oxc_ast, oxc_traverse, oxc_semantic, oxc_codegen, oxc_sourcemap, oxc_span, anyhow, rayon)
- [ ] **FOUN-02**: Module layout matches architecture blueprint (lib.rs, types.rs, errors.rs, words.rs, parse.rs, collector.rs, transform.rs, code_move.rs, entry_strategy.rs, emit.rs, filter_exports.rs)
- [ ] **FOUN-03**: Public API types defined (TransformModulesOptions, TransformOutput, TransformModule, SegmentAnalysis, EntryStrategy, MinifyMode, EmitMode, Diagnostic) with serde serialization
- [ ] **FOUN-04**: `transform_modules()` entry point accepts TransformModulesOptions and returns Result<TransformOutput>

### Core Transforms

- [ ] **CORE-01**: Optimizer detects all `$()` call sites (component$, useTask$, useVisibleTask$, etc.) from `@qwik.dev/core` imports
- [ ] **CORE-02**: Optimizer renames `$`-suffixed callers to `Qrl`-suffixed (component$ → componentQrl, useTask$ → useTaskQrl)
- [ ] **CORE-03**: Optimizer generates `qrl()` calls with correct import identifier and segment export name for segment entry strategy
- [ ] **CORE-04**: Optimizer generates `inlinedQrl()` calls with function body and hash for inline/hoist entry strategies

### Capture Analysis

- [ ] **CAPT-01**: Optimizer detects props destructuring patterns in component$ arrow functions and replaces with `_rawProps` parameter
- [ ] **CAPT-02**: Optimizer generates `_restProps(_rawProps, [...keys])` for rest patterns in destructured props
- [ ] **CAPT-03**: Optimizer identifies variables captured across $() boundaries using scope analysis
- [ ] **CAPT-04**: Optimizer generates capture arrays as third argument to qrl()/inlinedQrl() calls
- [ ] **CAPT-05**: Optimizer generates `_captures[N]` restoration statements in extracted segment bodies

### Segment Extraction

- [ ] **SEGM-01**: Optimizer extracts $() body into separate Program with correct imports and exports
- [ ] **SEGM-02**: Optimizer generates lazy import declarations (`const i_HASH = () => import("./path")`) in main module
- [ ] **SEGM-03**: Optimizer produces correct SegmentAnalysis metadata (hash, entry, canonicalFilename, origin, displayName, captures, ctxKind, ctxName)
- [ ] **SEGM-04**: Optimizer handles all entry strategies (Segment, Inline, Hoist, Single, Component, Smart, Hook)

### JSX Transforms

- [ ] **JSX-01**: Optimizer transforms JSX elements to `_jsxSorted(tag, varProps, constProps, children, flags, key)` calls
- [ ] **JSX-02**: Optimizer transforms JSX elements with spreads to `_jsxSplit(tag, props, children, flags, key)` calls
- [ ] **JSX-03**: Optimizer transforms JSX fragments to `_jsxSorted(Fragment, ...)` with Fragment import
- [ ] **JSX-04**: Optimizer classifies props as const vs var and separates into appropriate argument positions
- [ ] **JSX-05**: Optimizer generates `_wrapProp()` calls for signal.value and store property access in JSX props
- [ ] **JSX-06**: Optimizer generates `_fnSignal()` calls with hoisted helper functions for computed JSX expressions
- [ ] **JSX-07**: Optimizer generates hoisted `_hfN` function declarations and `_hfN_str` string constants at module level
- [ ] **JSX-08**: Optimizer handles `bind:value` and `bind:checked` by generating event handler QRLs with `_val`/`_chk` handlers

### Annotations + Stripping

- [ ] **ANNO-01**: Optimizer adds `/*#__PURE__*/` annotations to all framework replacement calls (_jsxSorted, _jsxSplit, componentQrl, qrl, inlinedQrl, _wrapProp, _fnSignal, _qrlSync, _noopQrl, _restProps)
- [ ] **ANNO-02**: Optimizer replaces `isServer`/`isDev`/`isBrowser` with boolean literals based on build mode
- [ ] **ANNO-03**: Optimizer eliminates dead branches after const replacement (if (false) { ... } → removed)
- [ ] **ANNO-04**: Optimizer replaces stripped $() calls with `_noopQrl("s_HASH")` in prod mode
- [ ] **ANNO-05**: Optimizer handles sync$ by generating `_qrlSync(fn, stringified_fn)` with minified function string

### Test Harness

- [ ] **TEST-01**: Test harness parses each of the 162 spec markdown files to extract input code, expected output modules, and expected diagnostics
- [ ] **TEST-02**: Test harness runs oxc-optimizer on each spec's input and compares output semantically against spec expectations
- [ ] **TEST-03**: All 162 spec tests pass with semantically equivalent output

### Codegen

- [ ] **EMIT-01**: Optimizer produces JavaScript output for main module and all extracted segments via OXC codegen
- [ ] **EMIT-02**: Optimizer generates source maps for all output modules
- [ ] **EMIT-03**: Optimizer adds correct import declarations for all framework functions used in output (_jsxSorted, qrl, componentQrl, etc.)

## Future Requirements

### Integration

- **INTG-01**: Drop-in replacement in Qwik framework monorepo (replace SWC optimizer crate)
- **INTG-02**: Vite/Rollup plugin compatibility verification
- **INTG-03**: TypeScript plugin layer integration
- **INTG-04**: Performance benchmarks vs SWC optimizer

## Out of Scope

| Feature | Reason |
|---------|--------|
| Byte-for-byte SWC output matching | Semantically equivalent is sufficient; formatting differences are expected |
| Modifying SWC optimizer code | Read-only reference; we build from specs not from SWC source |
| Modifying 162 spec files | Specs are locked as source of truth from v1.0 |
| Vite/Rollup plugin changes | Separate integration milestone after port ships |
| New optimizer features | Port first, enhance later |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| FOUN-01 | — | Pending |
| FOUN-02 | — | Pending |
| FOUN-03 | — | Pending |
| FOUN-04 | — | Pending |
| CORE-01 | — | Pending |
| CORE-02 | — | Pending |
| CORE-03 | — | Pending |
| CORE-04 | — | Pending |
| CAPT-01 | — | Pending |
| CAPT-02 | — | Pending |
| CAPT-03 | — | Pending |
| CAPT-04 | — | Pending |
| CAPT-05 | — | Pending |
| SEGM-01 | — | Pending |
| SEGM-02 | — | Pending |
| SEGM-03 | — | Pending |
| SEGM-04 | — | Pending |
| JSX-01 | — | Pending |
| JSX-02 | — | Pending |
| JSX-03 | — | Pending |
| JSX-04 | — | Pending |
| JSX-05 | — | Pending |
| JSX-06 | — | Pending |
| JSX-07 | — | Pending |
| JSX-08 | — | Pending |
| ANNO-01 | — | Pending |
| ANNO-02 | — | Pending |
| ANNO-03 | — | Pending |
| ANNO-04 | — | Pending |
| ANNO-05 | — | Pending |
| TEST-01 | — | Pending |
| TEST-02 | — | Pending |
| TEST-03 | — | Pending |
| EMIT-01 | — | Pending |
| EMIT-02 | — | Pending |
| EMIT-03 | — | Pending |

**Coverage:**
- v3.0 requirements: 36 total
- Mapped to phases: 0
- Unmapped: 36 (roadmap pending)

---
*Requirements defined: 2026-02-10*
*Last updated: 2026-02-10 after initial definition*
