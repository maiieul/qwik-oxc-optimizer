---
phase: 07-crate-foundation-test-harness
plan: 01
subsystem: api
tags: [rust, oxc, serde, cargo, workspace, qwik-optimizer]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture
    provides: Architecture blueprint with all type definitions and module layout
provides:
  - Compilable qwik-optimizer-oxc crate with all 16 module stubs
  - Complete public type definitions with serde camelCase serialization
  - Stub transform_modules() entry point returning empty TransformOutput
  - Cargo workspace configuration with both crates (POC + new OXC crate)
affects: [07-02, 08-tier1-basic-extraction, 09-tier2-component-pattern, all-subsequent-phases]

# Tech tracking
tech-stack:
  added: [oxc 0.113, oxc_traverse 0.113, serde 1, serde_json 1, anyhow 1, rayon 1, pathdiff 0.2, path-slash 0.2, base64 0.22, insta 1]
  patterns: [cargo workspace with resolver 3, serde rename_all camelCase, tagged enum serialization, feature-gated optional deps]

key-files:
  created:
    - Cargo.toml
    - crates/qwik-optimizer-oxc/Cargo.toml
    - crates/qwik-optimizer-oxc/src/lib.rs
    - crates/qwik-optimizer-oxc/src/types.rs
    - crates/qwik-optimizer-oxc/src/errors.rs
    - crates/qwik-optimizer-oxc/src/words.rs
    - crates/qwik-optimizer-oxc/src/hash.rs
    - crates/qwik-optimizer-oxc/src/parse.rs
    - crates/qwik-optimizer-oxc/src/collector.rs
    - crates/qwik-optimizer-oxc/src/transform.rs
    - crates/qwik-optimizer-oxc/src/import_rewrite.rs
    - crates/qwik-optimizer-oxc/src/code_move.rs
    - crates/qwik-optimizer-oxc/src/entry_strategy.rs
    - crates/qwik-optimizer-oxc/src/emit.rs
    - crates/qwik-optimizer-oxc/src/filter_exports.rs
    - crates/qwik-optimizer-oxc/src/props_destructuring.rs
    - crates/qwik-optimizer-oxc/src/is_const.rs
    - crates/qwik-optimizer-oxc/src/const_replace.rs
  modified: []

key-decisions:
  - "oxc 0.113 parser/traverse are always included (no feature flags); codegen, semantic, serialize are feature-gated"
  - "oxc_traverse added as separate crate dependency (TraverseCtx requires State generic in 0.113)"
  - "errors.rs implemented with actual helper functions instead of todo!() stubs (simple enough to do now)"
  - "entry_strategy.rs implemented should_inline/should_extract (trivial logic, no stub needed)"

patterns-established:
  - "Module stubs use #![allow(unused)] crate-level attribute to suppress dead code warnings"
  - "Internal types use pub(crate) visibility, public types use pub"
  - "All public types derive Serialize, Deserialize with #[serde(rename_all = camelCase)]"
  - "Optional fields use #[serde(skip_serializing_if = Option::is_none)]"
  - "EntryStrategy uses #[serde(tag = type)] for tagged union serialization"
  - "CtxKind variants use explicit #[serde(rename)] for wire format compatibility"

# Metrics
duration: 5min
completed: 2026-02-11
---

# Phase 7 Plan 1: Crate Foundation Summary

**qwik-optimizer-oxc crate skeleton with 16 module stubs, complete serde-annotated public types, and cargo workspace configuration**

## Performance

- **Duration:** 5 min
- **Started:** 2026-02-11T03:00:31Z
- **Completed:** 2026-02-11T03:06:16Z
- **Tasks:** 2
- **Files modified:** 18

## Accomplishments
- Created Cargo workspace with both POC and new OXC optimizer crate as members
- All 16 module files exist with proper pub(crate) stubs and module dependency graph established
- Complete public type definitions (12 public types, 6 internal types) with full serde annotations matching SWC optimizer wire format
- 16 tests passing: serde roundtrip, camelCase field names, tagged enum serialization, loc tuple-as-array, defaults verification
- Stub transform_modules() entry point compiles and returns empty TransformOutput

## Task Commits

Each task was committed atomically:

1. **Task 1: Create workspace Cargo.toml and crate Cargo.toml** - `1e528d4` (chore)
2. **Task 2: Create all 16 module files with types, stubs, and public API** - `97ff3cc` (feat)

## Files Created/Modified
- `Cargo.toml` - Workspace root with both crate members, resolver = "3"
- `crates/qwik-optimizer-oxc/Cargo.toml` - Crate config with oxc 0.113, serde, anyhow, rayon, insta
- `crates/qwik-optimizer-oxc/src/lib.rs` - Public API entry point, wires all 16 modules
- `crates/qwik-optimizer-oxc/src/types.rs` - All public and internal type definitions with serde
- `crates/qwik-optimizer-oxc/src/errors.rs` - Diagnostic creation helpers (implemented)
- `crates/qwik-optimizer-oxc/src/words.rs` - String constants and dollar API helpers (implemented)
- `crates/qwik-optimizer-oxc/src/hash.rs` - Segment hash computation (stub + format_segment_name)
- `crates/qwik-optimizer-oxc/src/parse.rs` - Module parsing stub
- `crates/qwik-optimizer-oxc/src/collector.rs` - First-pass AST analysis stub
- `crates/qwik-optimizer-oxc/src/transform.rs` - QwikTransform struct with new/extracted_segments/diagnostics
- `crates/qwik-optimizer-oxc/src/import_rewrite.rs` - Import mutation logic stub
- `crates/qwik-optimizer-oxc/src/code_move.rs` - Segment extraction stub
- `crates/qwik-optimizer-oxc/src/entry_strategy.rs` - Strategy application (implemented)
- `crates/qwik-optimizer-oxc/src/emit.rs` - Code generation stub
- `crates/qwik-optimizer-oxc/src/filter_exports.rs` - Export stripping stub
- `crates/qwik-optimizer-oxc/src/props_destructuring.rs` - Props transformation stub
- `crates/qwik-optimizer-oxc/src/is_const.rs` - Const evaluation stub
- `crates/qwik-optimizer-oxc/src/const_replace.rs` - Constant inlining stub

## Decisions Made
- oxc 0.113's "parser" and "traverse" are not separate features (always included); only "codegen", "semantic", "serialize" need feature flags. Blueprint specified them as features but they don't exist.
- oxc_traverse 0.113 requires a `State` generic parameter on `TraverseCtx<'a, State>`. Stubs updated accordingly.
- Implemented errors.rs, words.rs, and entry_strategy.rs with real logic instead of todo!() stubs since they are simple leaf modules.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed oxc feature flags in Cargo.toml**
- **Found during:** Task 1 (Cargo.toml creation)
- **Issue:** Blueprint specified `parser`, `traverse` as oxc features but oxc 0.113 does not have these feature flags (they are always included)
- **Fix:** Removed `parser` and `traverse` from features list; kept `codegen`, `semantic`, `serialize`. Added `oxc_traverse = "0.113"` as separate dependency.
- **Files modified:** crates/qwik-optimizer-oxc/Cargo.toml
- **Verification:** `cargo check -p qwik-optimizer-oxc` succeeds
- **Committed in:** 97ff3cc (Task 2 commit, since Cargo.toml was updated)

**2. [Rule 3 - Blocking] Fixed TraverseCtx generic parameter**
- **Found during:** Task 2 (module creation)
- **Issue:** `oxc_traverse::TraverseCtx<'a>` requires a `State` generic parameter in version 0.113 (`TraverseCtx<'a, State>`)
- **Fix:** Updated props_destructuring.rs stub to use `TraverseCtx<'a, S>` with generic `S`
- **Files modified:** crates/qwik-optimizer-oxc/src/props_destructuring.rs
- **Verification:** `cargo build -p qwik-optimizer-oxc` succeeds
- **Committed in:** 97ff3cc (Task 2 commit)

---

**Total deviations:** 2 auto-fixed (2 blocking)
**Impact on plan:** Both auto-fixes necessary to resolve API mismatches between blueprint and actual oxc 0.113 crate interface. No scope creep.

## Issues Encountered
None beyond the auto-fixed deviations above.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 16 module stubs ready for implementation in subsequent phases
- Test harness (Plan 07-02) can now build on this crate skeleton
- Module dependency graph is established through imports in each stub
- Public API contract is defined: `transform_modules(TransformModulesOptions) -> Result<TransformOutput>`

## Self-Check: PASSED

- All 18 created files verified present on disk
- Both task commits verified in git log (1e528d4, 97ff3cc)
- cargo build -p qwik-optimizer-oxc exits 0
- cargo test -p qwik-optimizer-oxc exits 0 (16 tests pass)
- cargo check -p qwik-optimizer-poc exits 0 (workspace doesn't break POC)

---
*Phase: 07-crate-foundation-test-harness*
*Completed: 2026-02-11*
