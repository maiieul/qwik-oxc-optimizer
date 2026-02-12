---
phase: 25-napi-crate
plan: 01
subsystem: napi
tags: [napi-rs, cdylib, node-binding, wire-format, serde]

# Dependency graph
requires:
  - phase: 24-runtime-bug-fixes
    provides: "Working OXC optimizer crate with 188 passing tests"
provides:
  - "qwik-napi-oxc crate exposing transform_modules to Node.js via napi-rs v2"
  - "Wire format adapter converting JS string entryStrategy to Rust enum"
  - "Diagnostic scope field matching SWC wire format"
affects: [26-ts-integration, platform-binding]

# Tech tracking
tech-stack:
  added: [napi-rs v2, napi-build, napi-derive]
  patterns: [serde_json::Value NAPI boundary, wire format adapter layer]

key-files:
  created:
    - crates/qwik-napi-oxc/Cargo.toml
    - crates/qwik-napi-oxc/build.rs
    - crates/qwik-napi-oxc/src/lib.rs
  modified:
    - Cargo.toml
    - crates/qwik-optimizer-oxc/src/types.rs
    - crates/qwik-optimizer-oxc/src/errors.rs
    - crates/qwik-optimizer-oxc/src/lib.rs

key-decisions:
  - "serde_json::Value as NAPI boundary type (not typed struct) for flexible JS<->Rust conversion"
  - "Synchronous transform_modules export (platform.ts wraps as Promise at TS layer)"
  - "manualChunks field accepted but ignored (OXC EntryStrategy has no manual variant)"
  - "Diagnostic scope field always 'optimizer' matching SWC wire format"

patterns-established:
  - "Wire format adapter pattern: NapiTransformModulesOptions -> TransformModulesOptions conversion"
  - "String-to-enum conversion for entryStrategy at NAPI boundary"

# Metrics
duration: 4min
completed: 2026-02-12
---

# Phase 25 Plan 01: NAPI Crate Summary

**qwik-napi-oxc crate with napi-rs v2 exposing transform_modules to Node.js, wire format adapter for entryStrategy string-to-enum, and Diagnostic scope field**

## Performance

- **Duration:** 4 min
- **Started:** 2026-02-12T19:06:41Z
- **Completed:** 2026-02-12T19:10:54Z
- **Tasks:** 2
- **Files modified:** 7

## Accomplishments
- Created qwik-napi-oxc crate with cdylib output type and napi-rs v2 dependencies
- Implemented wire format adapter converting JS convertOptions() output to internal OXC types
- Added Diagnostic scope field ("optimizer") matching SWC wire format across all construction sites
- Verified Node.js can load the .node module and call transform_modules with correct output
- Confirmed all 188 existing optimizer tests pass with zero regressions

## Task Commits

Each task was committed atomically:

1. **Task 1: Create qwik-napi-oxc crate with napi-rs v2 and wire format adapter** - `da78792` (feat)
2. **Task 2: Build .node native module and verify it loads** - verification-only (no code changes, build + Node.js smoke test passed)

## Files Created/Modified
- `Cargo.toml` - Added qwik-napi-oxc to workspace members
- `crates/qwik-napi-oxc/Cargo.toml` - NAPI crate config with napi-rs v2, cdylib
- `crates/qwik-napi-oxc/build.rs` - napi-build setup
- `crates/qwik-napi-oxc/src/lib.rs` - transform_modules NAPI export with wire format adapter
- `crates/qwik-optimizer-oxc/src/types.rs` - Added scope field to Diagnostic struct
- `crates/qwik-optimizer-oxc/src/errors.rs` - Added scope to create_source_error
- `crates/qwik-optimizer-oxc/src/lib.rs` - Added scope to inline Diagnostic construction

## Decisions Made
- Used `serde_json::Value` as NAPI boundary type (not a typed struct) because napi-rs's serde-json feature handles JsObject <-> serde_json::Value conversion automatically, matching the SWC binding pattern
- Made transform_modules synchronous in Rust since platform.ts wraps the return at the TS layer as Promise
- manualChunks field is accepted in deserialization but ignored (OXC EntryStrategy enum has no manual variant, SWC's Rust side also ignores it)
- Diagnostic scope field always set to "optimizer" matching SWC wire format (types.ts: `scope: string`)

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- qwik-napi-oxc crate builds as cdylib and loads in Node.js
- transform_modules callable with SWC wire format, producing correct camelCase JSON output
- Ready for Phase 26 TypeScript integration (loadPlatformBinding in platform.ts)

## Self-Check: PASSED

All created files verified present. Commit da78792 verified in git log.

---
*Phase: 25-napi-crate*
*Completed: 2026-02-12*
