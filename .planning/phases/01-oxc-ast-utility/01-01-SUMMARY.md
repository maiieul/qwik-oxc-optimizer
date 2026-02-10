---
phase: 01-oxc-ast-utility
plan: 01
subsystem: infra
tags: [rust, oxc, parser, ast, estree, cli]

# Dependency graph
requires: []
provides:
  - "Compiled Rust binary (oxc-ast-util) that parses JS/TS/JSX/TSX code and outputs ESTree JSON AST"
  - "CLI interface: stdin code + extension arg -> stdout JSON"
affects: [02-spec-generation]

# Tech tracking
tech-stack:
  added: [oxc 0.113 (serialize + ast_visit features), Rust edition 2024]
  patterns: [stdin-to-stdout CLI utility, OXC parse-serialize-in-scope pattern, Utf8ToUtf16 span conversion]

key-files:
  created:
    - oxc-ast-util/Cargo.toml
    - oxc-ast-util/src/main.rs
    - .gitignore
  modified: []

key-decisions:
  - "Added ast_visit feature flag to oxc dependency (serialize alone does not re-export Utf8ToUtf16)"
  - "OXC parser panics (ret.panicked=true) on severely malformed input -- utility exits with code 2 for unrecoverable errors, code 0 for recoverable parse errors"

patterns-established:
  - "Parse-serialize-in-scope: allocator, parser, and serialization all in same scope due to arena lifetime"
  - "ESTree JSON via to_pretty_estree_js_json/to_pretty_estree_ts_json (not serde_json)"
  - "Utf8ToUtf16 span conversion before serialization for ESTree compliance"

# Metrics
duration: 3min
completed: 2026-02-10
---

# Phase 1 Plan 1: OXC AST Utility Summary

**Minimal Rust CLI utility parsing JS/TS/JSX/TSX via oxc_parser 0.113, outputting ESTree-compatible JSON AST to stdout**

## Performance

- **Duration:** 3 min
- **Started:** 2026-02-10T18:00:07Z
- **Completed:** 2026-02-10T18:02:57Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Created standalone Rust binary crate with oxc 0.113 (serialize + ast_visit features)
- Implemented complete stdin-to-stdout parse-serialize pipeline: extension arg -> SourceType -> Parser -> Utf8ToUtf16 -> ESTree JSON
- Verified against 5 test cases: JS input, TSX input, recoverable parse errors, markdown safety, and a real 73-statement Qwik optimizer fixture file
- All 4 roadmap success criteria met (INFRA-01, INFRA-02)

## Task Commits

Each task was committed atomically:

1. **Task 1: Create oxc-ast-util Rust crate with parse-to-JSON implementation** - `5424c29` (feat)
2. **Task 2: Verify utility against real Qwik optimizer test code** - No file changes (verification-only task)

## Files Created/Modified
- `oxc-ast-util/Cargo.toml` - Rust crate manifest with oxc 0.113 dependency (serialize + ast_visit features)
- `oxc-ast-util/src/main.rs` - CLI utility: reads code from stdin, parses with OXC, outputs ESTree JSON to stdout (62 lines)
- `.gitignore` - Excludes Rust build artifacts (target/, Cargo.lock)

## Decisions Made
- **Added `ast_visit` feature flag:** The `serialize` feature alone does not re-export `oxc::ast_visit::utf8_to_utf16::Utf8ToUtf16`. Adding `ast_visit` feature was required for the Utf8ToUtf16 span converter.
- **Parser panic handling:** OXC's parser sets `ret.panicked = true` on severely malformed input (e.g., `function { broken`). The utility exits with code 2 for these unrecoverable cases. Recoverable parse errors (e.g., `let await = 5;` in ESM context) produce stderr warnings while still emitting the partial AST on stdout with exit code 0.
- **Edition 2024:** Rust edition 2024 compiled without issues on rustc 1.93.0. No fallback to 2021 needed.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Added `ast_visit` feature flag to oxc dependency**
- **Found during:** Task 1 (crate creation and build)
- **Issue:** Cargo.toml specified `features = ["serialize"]` per plan, but `oxc::ast_visit` module is gated behind the `ast_visit` feature flag, not the `serialize` flag. Build failed with `could not find ast_visit in oxc`.
- **Fix:** Changed features to `["serialize", "ast_visit"]` in Cargo.toml
- **Files modified:** oxc-ast-util/Cargo.toml
- **Verification:** `cargo build --release` succeeded after the fix
- **Committed in:** 5424c29 (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** Necessary for compilation. No scope creep.

## Issues Encountered
- **Test 3 input caused parser panic:** The plan's suggested malformed input (`function { broken syntax here`) triggers `ret.panicked = true` in OXC, which is unrecoverable. Used `let await = 5;` with `.mjs` extension instead to demonstrate recoverable parse errors (stderr warnings + partial AST on stdout). Both code paths (panic exit 2 and recoverable error exit 0) are correctly implemented and verified.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- The compiled binary at `oxc-ast-util/target/release/oxc-ast-util` is ready for Phase 2 (spec generation)
- Phase 2 can invoke: `echo '<code>' | ./oxc-ast-util/target/release/oxc-ast-util <ext>` to generate AST JSON for embedding in spec files
- Supports all extensions: js, jsx, ts, tsx, mjs, cjs, mts, cts

## Self-Check: PASSED

All claimed artifacts verified:
- oxc-ast-util/Cargo.toml: FOUND
- oxc-ast-util/src/main.rs: FOUND
- .gitignore: FOUND
- oxc-ast-util/target/release/oxc-ast-util (binary): FOUND
- Commit 5424c29: FOUND

---
*Phase: 01-oxc-ast-utility*
*Completed: 2026-02-10*
