---
phase: quick-2
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - .planning/quick/2-explain-how-source-maps-currently-work-i/SOURCE-MAPS.md
autonomous: true
must_haves:
  truths:
    - "Document explains the two distinct source map paths (main module vs segment module)"
    - "Document describes the OXC codegen API surface used for source map generation"
    - "Document identifies the current limitation: segment source maps are identity-mapped from re-parsed string-constructed code, not from original source spans"
    - "Document traces the full data flow from TransformModulesOptions.source_maps through to TransformModule.map"
  artifacts:
    - path: ".planning/quick/2-explain-how-source-maps-currently-work-i/SOURCE-MAPS.md"
      provides: "Comprehensive explanation of source map architecture in the optimizer"
---

<objective>
Produce a written explanation of how source maps currently work in the qwik-optimizer-oxc crate.

Purpose: Provide a clear reference document that explains the source map architecture, data flow, current limitations, and the difference between main-module and segment-module source map quality.

Output: A single markdown document at `.planning/quick/2-explain-how-source-maps-currently-work-i/SOURCE-MAPS.md`
</objective>

<execution_context>
@/Users/jackshelton/.claude/get-shit-done/workflows/execute-plan.md
@/Users/jackshelton/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@crates/qwik-optimizer-oxc/src/emit.rs
@crates/qwik-optimizer-oxc/src/code_move.rs
@crates/qwik-optimizer-oxc/src/lib.rs
@crates/qwik-optimizer-oxc/src/types.rs
@poc/src/poc_04_source_maps.rs
</context>

<tasks>

<task type="auto">
  <name>Task 1: Write source map architecture explanation</name>
  <files>.planning/quick/2-explain-how-source-maps-currently-work-i/SOURCE-MAPS.md</files>
  <action>
Read the following source files thoroughly to understand the complete source map flow:

1. `crates/qwik-optimizer-oxc/src/types.rs` -- Look at:
   - `TransformModulesOptions.source_maps` (bool, defaults to true)
   - `TransformOptions.source_maps` (internal copy)
   - `TransformModule.map` (Option<String> -- the JSON source map output)

2. `crates/qwik-optimizer-oxc/src/emit.rs` -- Look at:
   - `EmitOptions.source_maps` controls whether maps are generated
   - `emit_module()` -- the main module source map path: uses OXC's `Codegen::new().with_options(CodegenOptions { source_map_path: Some(PathBuf::from(source_filename)) }).with_source_text(source).build(program)`. The `program` here has PRESERVED SPANS from the original parse, so OXC codegen produces accurate v3 source maps mapping generated positions back to original source positions.
   - `normalize_code()` -- re-parses and re-emits without source maps (used for formatting)

3. `crates/qwik-optimizer-oxc/src/code_move.rs` -- Look at:
   - `build_segment_code()` / `build_segment_code_with_hoisted()` -- these construct segment JavaScript as STRING CONCATENATION (not AST manipulation). The code is built by joining import statements, capture restoration, and the serialized body code.
   - `emit_segment_with_map()` -- the segment source map path: takes the string-constructed code, RE-PARSES it with `oxc::parser::Parser`, then runs codegen with `source_map_path`. Since the AST was freshly parsed from the constructed string, spans point into the constructed string (not the original source). This produces IDENTITY-LIKE mappings (line N in output maps to line N in the re-parsed code).

4. `crates/qwik-optimizer-oxc/src/lib.rs` (the `transform_modules()` function) -- Trace the flow:
   - Creates `emit_options` from `config.source_maps`
   - For each input: parses -> collects -> transforms -> calls `emit::emit_module()` for the main module (gets `emit_result.map`)
   - For each segment: calls `code_move::build_segment_code_with_hoisted()` to get raw string code, then `code_move::emit_segment_with_map()` to get `(segment_code, segment_map)`
   - Both maps end up in `TransformModule.map`

5. `poc/src/poc_04_source_maps.rs` -- The proof-of-concept that demonstrates:
   - Nodes with original spans produce rich source map mappings
   - Nodes with SPAN (zero span) produce minimal/empty mappings
   - The key insight: span preservation determines source map quality

Write the document with these sections:

**Overview** -- What source maps are in this context, and the high-level architecture (two paths: main module and segment modules).

**Configuration** -- How source maps are enabled/disabled (`TransformModulesOptions.source_maps`, defaults to `true`). How it flows through `TransformOptions` and `EmitOptions`.

**Main Module Source Maps** -- The high-quality path. Original source is parsed by OXC parser, transformed via `traverse_mut` (which preserves AST node spans), then emitted by OXC codegen with `source_map_path` set. Since spans are preserved from the original parse, the generated source map accurately maps every token in the output back to its original position in the input source. The key OXC APIs: `CodegenOptions { source_map_path }`, `.with_source_text(source)`, `CodegenReturn.map.to_json_string()`.

**Segment Module Source Maps** -- The limited path. Segment bodies are serialized to JavaScript strings during the transform pass. Then `build_segment_code()` constructs a complete module as a string (concatenating imports + captures + body). This string is then RE-PARSED by `emit_segment_with_map()` and emitted with codegen. Since the AST comes from re-parsing the constructed string, all spans reference positions in the constructed string, not the original source. The resulting source map is identity-like (output line N maps to input line N of the same constructed code). The doc comment in `emit_segment_with_map()` acknowledges this explicitly.

**Data Flow Diagram** -- An ASCII diagram showing:
```
Input source code
    |
    v
OXC Parser (preserves spans)
    |
    v
traverse_mut (QwikTransform)
    |
    +---> Main module AST (spans preserved)
    |         |
    |         v
    |     emit_module() -> (code, Some(source_map_json))
    |         |                    ^-- accurate: spans point to original source
    |         v
    |     TransformModule { code, map }
    |
    +---> Segment bodies serialized to strings during transform
              |
              v
          build_segment_code() -> raw JavaScript string
              |
              v
          emit_segment_with_map():
              Parser::new(raw_string)  <-- re-parses from string
              Codegen::new().with_source_text(raw_string).build()
              |
              v
          TransformModule { code, map }  <-- identity-like mappings
```

**OXC Codegen API** -- Document the specific OXC API surface used:
- `oxc::codegen::CodegenOptions { source_map_path: Option<PathBuf> }` -- Setting this to Some enables source map generation. The path value becomes the `"file"` field in the source map JSON.
- `oxc::codegen::Codegen::new().with_options(options).with_source_text(source).build(program)` -- The `.with_source_text()` call provides the original source text that OXC uses to compute source positions.
- `oxc::codegen::CodegenReturn { code: String, map: Option<oxc_sourcemap::SourceMap> }` -- The `map` is present when `source_map_path` was set.
- `SourceMap::to_json_string()` -- Serializes to standard v3 source map JSON.

**Output Format** -- The source map is a standard v3 JSON string stored in `TransformModule.map: Option<String>`. It contains `version`, `sources`, `sourcesContent` (if available), `names`, and `mappings` (VLQ-encoded). When `source_maps: false`, all modules get `map: None`.

**Current Limitation and POC-04 Insight** -- The segment source maps are essentially identity maps. POC-04 proved that span preservation is the key: AST nodes constructed with real spans (pointing to positions in the original source) produce meaningful source map entries, while nodes with zero-span (`SPAN`) produce nothing useful. The current implementation serializes segment bodies to strings and re-parses, losing all original span information. To get accurate segment source maps, the implementation would need to either (a) preserve original AST spans when extracting segment bodies, or (b) implement post-hoc source map remapping.

Do NOT include any code changes or recommendations for how to fix this. This is purely an explanatory document about the current state.
  </action>
  <verify>
The file exists at `.planning/quick/2-explain-how-source-maps-currently-work-i/SOURCE-MAPS.md` and contains all required sections: Overview, Configuration, Main Module Source Maps, Segment Module Source Maps, Data Flow Diagram, OXC Codegen API, Output Format, Current Limitation.
  </verify>
  <done>
A comprehensive written explanation exists that accurately describes the two source map generation paths, the OXC APIs used, the data flow from config to output, and the current limitation where segment source maps are identity-mapped due to string-based code construction.
  </done>
</task>

</tasks>

<verification>
- Document exists and is readable markdown
- All seven sections are present
- ASCII data flow diagram is included
- The explanation correctly distinguishes main module (accurate spans) from segment module (identity-like) source maps
- File paths and function names referenced in the document match the actual codebase
</verification>

<success_criteria>
The document can be read by someone unfamiliar with the codebase and they would understand: (1) how to enable/disable source maps, (2) what OXC APIs generate them, (3) why main module maps are accurate but segment maps are not, (4) what the data flow looks like end to end.
</success_criteria>

<output>
After completion, create `.planning/quick/2-explain-how-source-maps-currently-work-i/2-SUMMARY.md`
</output>
