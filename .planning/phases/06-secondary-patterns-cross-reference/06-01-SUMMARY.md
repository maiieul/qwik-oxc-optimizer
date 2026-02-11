---
phase: 06-secondary-patterns-cross-reference
plan: 01
subsystem: api
tags: [jsx, signals, props-destructuring, oxc, ast-builder, wrapProp, fnSignal, jsxSorted]

# Dependency graph
requires:
  - phase: 04-core-api-mapping-architecture
    provides: "AstBuilder API patterns, import management strategy, QRL wrapping"
  - phase: 05-deep-research-proof-of-concept
    provides: "Capture analysis algorithm, multi-module output, props destructuring ordering decision"
provides:
  - "JSX transformation OXC API mapping (_jsxSorted, _jsxSplit, prop classification, Fragment, events)"
  - "Props destructuring OXC API mapping (_rawProps, _restProps transformation algorithm)"
  - "Signal optimization OXC API mapping (_wrapProp two forms, _fnSignal hoisting, non-wrapping rules)"
  - "Hoisted function placement rules (CONV-14)"
affects: [06-02, 06-03, implementation]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Prop classification: var vs const split for component vs native element targets"
    - "_wrapProp Form 1 (signal.value) vs Form 2 (source, propName)"
    - "_fnSignal hoisting with minified string representation via oxc_codegen"
    - "Props destructuring pre-pass ordering before capture analysis"

key-files:
  created:
    - ".planning/phases/06-secondary-patterns-cross-reference/JSX-TRANSFORMS-MAPPING.md"
    - ".planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md"
  modified: []

key-decisions:
  - "Prop classification uses match on expression type with reactive source detection for component targets; simpler rules for native elements"
  - "_wrapProp has two distinct forms: Form 1 (signal only) strips .value; Form 2 (source, propName) for named property access"
  - "_fnSignal string representation generated via oxc_codegen in minify mode"
  - "Props destructuring detection checks BindingPatternKind::ObjectPattern on first param of component$ callback"
  - "Flags values pattern-matched from spec files: 0=spread, 1=multiple/dynamic children, 2=event-only, 3=leaf/simple"

patterns-established:
  - "JSX classification: uppercase tag -> component (identifier), lowercase -> native element (string literal)"
  - "Fragment import from different source: @qwik.dev/core/jsx-runtime (not @qwik.dev/core)"
  - "Event name transform: strip on prefix + $ suffix, add q-e: prefix, lowercase"
  - "Hoisted function numbering resets per segment module (_hf0, _hf1, ...)"

# Metrics
duration: 8min
completed: 2026-02-11
---

# Phase 6 Plan 01: JSX Transform & Props/Signal Optimization OXC API Mapping Summary

**Complete OXC AstBuilder mapping for _jsxSorted/_jsxSplit construction, prop var/const classification, _wrapProp two forms, _fnSignal hoisting with minified strings, and props destructuring _rawProps/_restProps transformation**

## Performance

- **Duration:** 8 min
- **Started:** 2026-02-11T00:23:05Z
- **Completed:** 2026-02-11T00:31:09Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Created JSX-TRANSFORMS-MAPPING.md (1250 lines) with 14 Rust code blocks covering _jsxSorted, _jsxSplit, prop classification, Fragment handling, event handler transformation, dev mode, children encoding, flags semantics, and import management
- Created PROPS-SIGNALS-MAPPING.md (1190 lines) with 15 Rust code blocks covering props destructuring detection and transformation, _wrapProp two forms, _fnSignal hoisting pattern, hoisted function placement, non-wrapping decision tree, ordering constraint, and import management
- Both documents reference 4 spec files with specific line numbers as evidence
- Cross-references established between JSX and PROPS documents, and back to Phase 4/5 mappings

## Task Commits

Each task was committed atomically:

1. **Task 1: Create JSX transformation OXC API mapping (APIM-04)** - `8d422c0` (feat)
2. **Task 2: Create props destructuring and signal optimization OXC API mapping (APIM-09)** - `facebb1` (feat)

## Files Created/Modified
- `.planning/phases/06-secondary-patterns-cross-reference/JSX-TRANSFORMS-MAPPING.md` - Complete OXC API mapping for JSX transformation patterns including _jsxSorted, _jsxSplit, prop classification, Fragment, events, dev mode, children, flags, imports
- `.planning/phases/06-secondary-patterns-cross-reference/PROPS-SIGNALS-MAPPING.md` - Complete OXC API mapping for props destructuring and signal optimization including _wrapProp, _fnSignal, hoisted functions, non-wrapping rules, ordering

## Decisions Made
- Prop classification for components uses a match-based approach on expression types with reactive source detection to distinguish var from const
- _wrapProp has two distinct API forms: Form 1 strips .value for direct signal wrapping, Form 2 passes source and property name string
- _fnSignal string representation uses oxc_codegen in minify mode for consistent minified output
- Props destructuring detection uses BindingPatternKind::ObjectPattern check on first parameter of component$ callback arrow function
- Flags values determined by pattern matching against spec file outputs: 0=spread, 1=container with dynamic children, 2=event-only, 3=leaf/simple

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- JSX and props/signal patterns fully mapped, ready for Phase 6 Plan 02 (entry strategy, stripping, const folding)
- Cross-references to Phase 4 and Phase 5 documents established for implementer navigation
- All CONV types documented: CONV-03, CONV-04, CONV-11, CONV-14

## Self-Check: PASSED

- FOUND: JSX-TRANSFORMS-MAPPING.md
- FOUND: PROPS-SIGNALS-MAPPING.md
- FOUND: 06-01-SUMMARY.md
- FOUND: commit 8d422c0 (Task 1)
- FOUND: commit facebb1 (Task 2)

---
*Phase: 06-secondary-patterns-cross-reference*
*Completed: 2026-02-11*
