---
phase: 05-deep-research-proof-of-concept
plan: 01
subsystem: api-mapping
tags: [oxc, scoping, captures, astbuilder, codegen, source-maps, hash, rust]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture (plan 01)
    provides: "OXC API mapping for $-extraction, QRL wrapping, import rewriting"
  - phase: 04-core-api-mapping-architecture (plan 02)
    provides: "Architecture blueprint with module layout, public API types, data flow specification"
  - phase: 05-RESEARCH
    provides: "OXC Scoping API research, capture analysis algorithm, multi-module construction patterns"
provides:
  - "APIM-03: Capture analysis algorithm mapped to OXC Scoping APIs with 4-type classification and 8 edge cases"
  - "APIM-06: Multi-module output mapped to AstBuilder Program construction with allocator strategy, hash algorithm, codegen pipeline"
  - "Complete compute_captures() function specification with Rust code"
  - "Complete build_segment_program() function specification with Rust code"
  - "Segment hash computation algorithm (DefaultHasher + base64url) with 8 verification hashes from spec files"
affects: [05-02-poc-dollar-detection-capture-analysis, 05-03-poc-multi-module-source-maps, phase-06-secondary-patterns]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "4-type capture classification: ImportReemit, ConstInline, LocalCapture, InvalidCapture"
    - "Shared allocator strategy for multi-module construction in POC"
    - "Span preservation rules: original spans on extracted nodes, SPAN on constructed nodes"
    - "Segment hash: DefaultHasher(scope, rel_path, display_name) -> base64url -> replace -/_ with 0"

key-files:
  created:
    - ".planning/phases/05-deep-research-proof-of-concept/CAPTURE-ANALYSIS-MAPPING.md"
    - ".planning/phases/05-deep-research-proof-of-concept/MULTI-MODULE-OUTPUT-MAPPING.md"
  modified: []

key-decisions:
  - "Shared allocator recommended for POC; separate allocators for production parallel codegen"
  - "Reference scope identification via traversal-time mapping (record ref_id -> scope_id during traverse_mut)"
  - "Capture ordering uses encounter-order during AST traversal of the body"
  - "Props destructuring pre-transform must run BEFORE capture analysis"

patterns-established:
  - "Pattern 1: compute_captures() iterates all symbols, checks scope containment, classifies by SymbolFlags"
  - "Pattern 2: build_segment_program() assembles Program from imports + lazy imports + exported body"
  - "Pattern 3: Codegen with source_map_path for segment source map generation"
  - "Pattern 4: Const literal detection via VariableDeclarationKind::Const + primitive literal initializer"

# Metrics
duration: 8min
completed: 2026-02-10
---

# Phase 5 Plan 1: API Mapping Research Summary

**Capture analysis algorithm with 4-type classification mapped to OXC Scoping APIs across 8 edge cases, and multi-module output construction mapped to AstBuilder with shared allocator strategy, segment hash algorithm, and codegen source map pipeline**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-10T23:19:23Z
- **Completed:** 2026-02-10T23:27:03Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- Created CAPTURE-ANALYSIS-MAPPING.md (919 lines) covering the complete capture analysis algorithm with OXC Scoping API mapping, 4-type classification logic (ImportReemit, ConstInline, LocalCapture, InvalidCapture), and all 8 edge cases with code examples from 4 spec files
- Created MULTI-MODULE-OUTPUT-MAPPING.md (1079 lines) covering multi-module Program construction via AstBuilder, shared vs separate allocator comparison, segment hash algorithm, codegen pipeline with source maps, and a complete end-to-end example tracing example_1.md through all stages
- Documented 10+ OXC Scoping API methods in a reference table with signatures and purpose
- Documented 20+ AstBuilder methods in a reference table for segment construction
- Provided segment hash verification table with 8 known hashes from spec files

## Task Commits

Each task was committed atomically:

1. **Task 1: Create capture analysis API mapping document (APIM-03)** - `bd24f46` (feat)
2. **Task 2: Create multi-module output API mapping document (APIM-06)** - `f769f86` (feat)

## Files Created/Modified

- `.planning/phases/05-deep-research-proof-of-concept/CAPTURE-ANALYSIS-MAPPING.md` - Complete capture analysis algorithm specification with OXC Scoping API mapping, 4-type classification, 8 edge cases (919 lines)
- `.planning/phases/05-deep-research-proof-of-concept/MULTI-MODULE-OUTPUT-MAPPING.md` - Complete multi-module output specification with AstBuilder APIs, allocator strategy, hash algorithm, codegen pipeline, end-to-end example (1079 lines)

## Decisions Made

1. **Shared allocator for POC** -- Use a single Allocator for main module and all segments during POC. Simpler lifetime management. Production can switch to separate allocators per segment for parallel codegen via rayon.

2. **Reference scope identification via traversal mapping** -- Since it is unclear whether OXC Reference has a scope_id() method, the recommended approach is to build a HashMap<ReferenceId, ScopeId> during traverse_mut by recording current_scope_id() in enter_identifier_reference. This is verified as an open question for POC-02.

3. **Capture ordering by encounter order** -- The captures array order follows the order in which captures are first encountered during AST traversal. This must be consistent between the qrl() call site and the segment body.

4. **Props destructuring runs before capture analysis** -- The ({foo}) -> (_rawProps) conversion must happen as a pre-transform before capture analysis, because the scope tree needs to reflect _rawProps as the binding.

## Deviations from Plan

None -- plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- CAPTURE-ANALYSIS-MAPPING.md provides the algorithm spec for POC-02 (capture analysis proof of concept)
- MULTI-MODULE-OUTPUT-MAPPING.md provides the construction spec for POC-03 (multi-module output) and POC-04 (source maps for split modules)
- Both documents are standalone -- a developer can implement the POCs from these docs alone
- Three open questions documented for verification during POC implementation: Reference scope_id, SymbolFlags values for function/class, and AstBuilder::program() signature

## Self-Check: PASSED

- [x] CAPTURE-ANALYSIS-MAPPING.md exists (919 lines)
- [x] MULTI-MODULE-OUTPUT-MAPPING.md exists (1079 lines)
- [x] Task commit bd24f46 exists in git log
- [x] Task commit f769f86 exists in git log
- [x] Both documents reference 4+ spec files
- [x] Both documents contain complete Rust function signatures

---
*Phase: 05-deep-research-proof-of-concept*
*Completed: 2026-02-10*
