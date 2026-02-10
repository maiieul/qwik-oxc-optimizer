---
phase: 05-deep-research-proof-of-concept
plan: 03
subsystem: compiler
tags: [oxc, astbuilder, codegen, source-maps, multi-module, segment-hash, rust]

# Dependency graph
requires:
  - phase: 05-01
    provides: "Capture analysis and multi-module output API mappings (MULTI-MODULE-OUTPUT-MAPPING.md)"
provides:
  - "POC-03: Working Rust program splitting input into main + segment modules with valid JS output"
  - "POC-04: Working Rust program generating source maps for split modules via oxc_codegen"
  - "Segment hash computation algorithm (DefaultHasher + base64url)"
  - "Span preservation strategy for segment source maps"
affects: [06-secondary-patterns, implementation]

# Tech tracking
tech-stack:
  added: [base64 0.22]
  patterns: [astbuilder-program-construction, codegen-source-map-generation, span-preservation]

key-files:
  created:
    - poc/src/poc_03_multi_module.rs
    - poc/src/poc_04_source_maps.rs
  modified:
    - poc/Cargo.toml

key-decisions:
  - "Segment hash uses DefaultHasher + base64url + replace -/_ with 0; exact values vary by Rust toolchain"
  - "Program span must encompass all child node preserved spans to avoid source map builder panic"
  - "Segments with preserved spans require Program.source_text set to original source"
  - "SPAN-only constructed nodes produce zero source map entries; preserved spans produce entries"

patterns-established:
  - "AstBuilder construction: build_named_import, build_export_const, build_arrow_with_body, build_segment_program helpers"
  - "Source map generation: CodegenOptions { source_map_path } + .with_source_text(source) + result.map.to_json_string()"
  - "Span preservation for segments: wrapper nodes use SPAN, extracted body nodes keep original spans"

# Metrics
duration: 12min
completed: 2026-02-10
---

# Phase 5 Plan 3: Multi-Module Output and Source Maps POC Summary

**POC-03 and POC-04: AstBuilder-constructed segment Programs with valid JS output and source map generation demonstrating span preservation strategy**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-02-10T23:30:00Z
- **Completed:** 2026-02-10T23:42:29Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- POC-03: Splitting one input into main module + 3 segment modules, all producing valid JavaScript via AstBuilder + Codegen
- POC-04: Source map generation for split modules proving span preservation strategy works (preserved spans produce 26-char mappings vs 0-char for SPAN-only)
- Segment hash algorithm validated: deterministic 11-char base64url strings using DefaultHasher (exact match requires same Rust toolchain as SWC WASM build)
- Critical discovery: Program span must encompass preserved child spans, and source_text must be original source
- All four POC binaries (01-04) compile and run in the same workspace

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement POC-03 (multi-module output)** - `c425a16` (feat)
2. **Task 2: Implement POC-04 (source maps for split modules)** - `746bcf3` (feat)

**Plan metadata:** (pending)

## Files Created/Modified

- `poc/src/poc_03_multi_module.rs` - Multi-module output: parse input, construct 3 segment Programs via AstBuilder, codegen all to valid JS, compute segment hashes
- `poc/src/poc_04_source_maps.rs` - Source map generation: main module source maps (181-char mappings), SPAN-only segment (0-char), preserved-span segment (26-char)
- `poc/Cargo.toml` - Added bin entries for poc-03 and poc-04, added base64 dependency

## Decisions Made

1. **Hash algorithm structural validation only** - DefaultHasher produces different hash values across Rust compiler versions. The SWC optimizer WASM was compiled with a specific Rust toolchain, so exact hash matching requires the same toolchain. We validate structure (11 chars, alphanumeric) and determinism instead.

2. **Program span encompasses preserved spans** - When a segment contains nodes with preserved spans from the original source, the Program node's span must cover those positions (e.g., `Span::new(0, source.len() as u32)`), otherwise OXC's source map builder panics with "violated N:M <= 0" error.

3. **Source text required for preserved-span segments** - `ast.program()` takes a `source_text: &'a str` parameter. For segments with preserved spans, this must be the original source text (not empty string) so the source map's sourcesContent includes the original code.

4. **AstBuilder helper function pattern** - Created reusable helpers (`build_named_import`, `build_export_const`, `build_arrow_with_body`, `build_segment_program`) that will serve as reference implementations for the real optimizer.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

1. **Lifetime annotations for AstBuilder string params** - AstBuilder methods accepting strings require `&'a str` (not `&str`) to satisfy `Into<Ident<'a>>` bounds. Fixed by adding proper lifetime annotations to all helper functions.

2. **Source map builder panic on preserved spans** - When constructing a segment Program with SPAN (0:0) but child nodes with non-zero spans (e.g., 133:148 from original source), the source map builder panicked: "violated 145:148 <= 0 for ctx". Fixed by giving the Program `Span::new(0, source.len() as u32)` and passing original source as `source_text`.

3. **Lifetime inference in main()** - Using `None::<Box<'a, ...>>` type annotations in `main()` where `'a` lifetime isn't in scope. Fixed by using `None::<Box<'_, ...>>` instead.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- All 4 POC binaries validated -- Phase 5 core objective (proving OXC can implement the optimizer pipeline) is achieved
- Ready for Phase 6 (secondary patterns and cross-reference) with full confidence in OXC capabilities
- Key patterns established: AstBuilder construction, codegen with source maps, span preservation
- The hash algorithm's toolchain dependency should be documented in Phase 6 for implementors

## Self-Check: PASSED

- [x] poc/src/poc_03_multi_module.rs exists
- [x] poc/src/poc_04_source_maps.rs exists
- [x] poc/Cargo.toml exists
- [x] Commit c425a16 (Task 1: POC-03) found in git log
- [x] Commit 746bcf3 (Task 2: POC-04) found in git log
- [x] All 4 POC binaries compile: `cargo build` succeeds
- [x] POC-03 runs: outputs valid JS for main + 3 segments
- [x] POC-04 runs: outputs JS + source map JSON with mappings

---
*Phase: 05-deep-research-proof-of-concept*
*Completed: 2026-02-10*
