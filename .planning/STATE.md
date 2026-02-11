# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-10)

**Core value:** A working OXC-based Qwik optimizer crate that passes all 162 spec tests
**Current focus:** Phase 13 -- Source Maps & Full Validation

## Current Position

Phase: 13 of 13 (Source Maps & Full Validation)
Plan: 1 of 4 in current phase -- 13-01 complete
Status: Source map wiring complete. emit_module() generates v3 source maps via OXC codegen. Main and segment modules receive maps when source_maps=true. 150 unit tests + 5 spec tests + 162/162 spec transforms pass.
Last activity: 2026-02-11 -- Completed 13-01: Source map wiring

Progress: [################░░░░] 60% (30/31 total plans across all milestones; v3.0 13/14)

## Performance Metrics

**Velocity (v1.0):**
- Total plans completed: 9
- Average duration: 9min
- Total execution time: 1.42 hours

**Velocity (v2.0):**
- Total plans completed: 8
- Average duration: 8min
- Total execution time: ~1.1 hours

**Velocity (v3.0):**
- Total plans completed: 13
- Average duration: 12min
- Total execution time: ~168min

## Accumulated Context

### Decisions

Full decision log in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- 9-tier CONV implementation order validated in cross-reference (dependency graph drives phase structure)
- Two-option PURE annotation strategy: Option A (OXC built-in) preferred, Option B (manual comment) fallback
- Props destructuring must run before capture analysis (CONV-11 before CONV-05)
- CONV-10 must run before CONV-09 (const replacement before stripping)
- oxc 0.113: parser/traverse always included (not features); codegen/semantic/serialize are features
- oxc_traverse 0.113 TraverseCtx requires State generic parameter
- Case-insensitive config key matching for spec parser (Transpile TS vs Transpile Ts)
- EmitMode::Test mapped to EmitMode::Lib (specs mark Test as default)
- base64 made non-optional in Cargo.toml (hash needs it unconditionally)
- OXC 0.113 BindingPattern is a flat enum (no kind field); JSXExpression uses inherit_variants! (Expression variants directly on enum)
- Collector uses two-pass recursive walk (not Traverse trait) for read-only analysis
- Use exit_expression (not exit_call_expression) for QRL replacement -- gives mutable &mut Expression
- Arena string allocation via ctx.ast.atom() for runtime-constructed strings in AstBuilder
- std::mem::swap with ctx.ast.vec() for OXC Vec mutation (no Default impl)
- expression_call_with_pure for PURE annotations on qrl/inlinedQrl calls
- Pending dollar calls tracked by span.start u32 in HashSet for O(1) lookup
- Post-analysis mutation pattern for props destructuring: analyze in enter_call_expression, mutate in exit_expression
- Recursive walk for identifier replacement with scope-aware shadowing for nested arrows
- param_names as Option<Vec<String>> on SegmentAnalysis with skip_serializing_if
- Simplified string-based capture analysis instead of OXC Scoping API (traverse_mut consumes Scoping)
- Stack-based capture_stack Vec for nested $()-body tracking (each frame independent)
- Post-process child segment captures during component$ exit for props destructuring interaction
- capture_names as Option<Vec<String>> on SegmentAnalysis with skip_serializing_if
- String-based segment code construction (code_move builds JS as string, normalize_code formats via parse+codegen)
- Codegen::print_expression + into_source_text for serializing AST expressions to strings during traverse
- finalize_segments() as post-traverse pass for child segment metadata (avoids nested mutation during traverse)
- Extract-and-discard pattern: body extracted from call.arguments, serialized, dropped (segment strategy replaces entire call)
- Hoist treated as inline-like for code output (body in main module via inlinedQrl, segments have empty code)
- Smart/Component/Hook/Single treated as Segment for now (separate files with code, grouping deferred to Phase 13)
- Structural spec testing: verify module counts, is_entry, metadata properties (not exact hash/code matching)
- Recursive free functions for JSX transform (JSXChild::Element calls walk_jsx_element, not walk_expression)
- Exhaustive JSXExpression->Expression match covering all 39 inherited variants (no catch-all to undefined)
- Auto-key u6_N generation for elements with content; null key for empty self-closing
- Event handlers to const props with q-e: prefix; non-const values to var props
- String-based hoisted function approach: store _hfN declarations as code strings, inject into codegen output after imports
- Signal-wrapped values (_wrapProp, _fnSignal) go to const props since wrapping handles reactivity
- Reactive dep detection: .value = signal, deep chains = store, _rawProps = props, imports = not reactive
- bind:value/bind:checked expand to value/checked const prop + q-e:input inlinedQrl event handler
- Span-based tracking (HashSet<u32>) for stripped_segments and pending_sync_calls with O(1) lookup
- Capture stack frame pushed for sync$ calls to isolate identifiers (popped on exit, frame discarded)
- Strategy-aware import tracking: _qrlSync import only added to main module for top-level sync$ calls
- Filter stripped children from finalize_segments() child_lazy_imports to prevent unnecessary lazy imports in parent
- const_replace runs as pre-pass before traverse_mut so segment body serialization sees replaced boolean literals
- AstBuilder::new(&allocator) for AST construction outside traverse context (no TraverseCtx)
- Build constant imports stripped after identifier replacement for clean output matching spec
- OXC Box::unbox() for taking ownership of boxed AST nodes in dead branch elimination
- Segment source maps from re-parsed string-constructed code (identity-like mappings, not original-position mappings)
- emit_segment_with_map() replaces normalize_code() for segment codegen with optional source maps
- emit_module() source_filename parameter sets source map "file" field via CodegenOptions source_map_path

### Pending Todos

None.

### Blockers/Concerns

None.

## Session Continuity

Last session: 2026-02-11
Stopped at: Completed 13-01-PLAN.md. Source map wiring complete. Ready for 13-02 (source map validation).
Resume file: None
