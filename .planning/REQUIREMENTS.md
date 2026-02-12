# Requirements: Qwik Optimizer — SWC-to-OXC Port

**Defined:** 2026-02-11
**Core Value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests

## v5.0 Requirements

Requirements for drop-in replacement compliance. Each maps to roadmap phases.

### Path Resolution

- [ ] **PATH-01**: Canonical filename preserves file extension in origin prefix (`test.tsx_Header_component_HASH` not `test_Header_component_HASH`)
- [ ] **PATH-02**: Lazy import paths in main module match actual segment file paths (no mismatch between `./test_...` and `test.tsx_...`)
- [ ] **PATH-03**: Lazy import paths include file extension when `explicit_extensions: true` (e.g., `import("./seg.tsx")` not `import("./seg")`)
- [ ] **PATH-04**: Output file extension is `.js` (not `.jsx`) when both `transpile_ts` and `transpile_jsx` are true (`.tsx` → `.js`, not `.tsx` → `.jsx`)

### Import Management

- [ ] **IMPORT-01**: Consumed `$`-suffixed imports stripped from main module output (e.g., `import { component$ }` removed after converting to `componentQrl`)
- [ ] **IMPORT-02**: Qrl-suffixed imports (e.g., `useStylesQrl`) only added to modules where they are actually referenced, not to main module when only used in segments

### Naming

- [ ] **NAME-01**: Nested segment display names include full parent context hierarchy (e.g., `App_component_div_onClick` for a `$()` inside a JSX attribute inside a `component$`, not flat `App`)

### Annotations

- [ ] **PURE-01**: PURE annotation (`/* @__PURE__ */`) only applied to tree-shakeable calls (`qrl()`, `componentQrl()`, `_jsxSorted()`), not to side-effectful calls (`useStylesQrl()`, `useTaskQrl()`)

## Future Requirements

None — all identified issues are in v5.0 scope.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Byte-for-byte SWC output matching | Whitespace, hash values, PURE comment format differences are acceptable |
| Capture analysis deviations (16 known) | JSX event handler scope tracking — separate milestone |
| Diagnostic deviations (3 known) | Validation rules for invalid patterns — separate milestone |
| 5 module count deviations | 3 parser limitations + 2 pre-compiled QRL — fundamental differences |
| EmitMode::Test equivalent | Only used in SWC test harness, not needed for production |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| PATH-01 | — | Pending |
| PATH-02 | — | Pending |
| PATH-03 | — | Pending |
| PATH-04 | — | Pending |
| IMPORT-01 | — | Pending |
| IMPORT-02 | — | Pending |
| NAME-01 | — | Pending |
| PURE-01 | — | Pending |

**Coverage:**
- v5.0 requirements: 8 total
- Mapped to phases: 0
- Unmapped: 8 ⚠️

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-11 after initial definition*
