# Architecture Research: Spec Generation Pipeline

**Domain:** Code transformation spec generation from SWC snapshot tests
**Researched:** 2026-02-10
**Confidence:** HIGH

## Recommendation: Rust CLI Tool

Build a single-binary Rust CLI that reads snapshot files and test source, parses all code with `oxc_parser`, and emits one markdown spec file per snapshot. Rust because: the target optimizer is Rust, `oxc_parser` is a Rust crate, and the snapshot parsing is string-heavy work that benefits from Rust's zero-cost abstractions. A script-based approach (Python/Node) would require shelling out to a separate Rust binary for AST generation anyway, so consolidating into one tool eliminates that seam.

## System Overview

```
                        INPUTS
    ┌──────────────────────────────────────────────────┐
    │                                                  │
    │  test.rs (5387 lines, 163 test functions)        │
    │  snapshots/ (162 .snap files)                    │
    │                                                  │
    └────────────┬──────────────────────┬──────────────┘
                 │                      │
                 v                      v
    ┌────────────────────┐  ┌────────────────────────┐
    │  Test Config        │  │  Snapshot Parser        │
    │  Extractor          │  │                        │
    │                    │  │  Reads .snap files      │
    │  Parses test.rs    │  │  Splits on delimiters   │
    │  Extracts TestInput │  │  Extracts:             │
    │  config per test   │  │    - input code         │
    │                    │  │    - output modules     │
    │                    │  │    - segment metadata   │
    │                    │  │    - source maps         │
    │                    │  │    - diagnostics         │
    └────────┬───────────┘  └───────────┬─────────────┘
             │                          │
             v                          v
    ┌────────────────────────────────────────────────┐
    │                                                │
    │              Parsed Test Case                   │
    │  (config + input code + output modules +       │
    │   segment metadata + diagnostics)              │
    │                                                │
    └───────────────────────┬────────────────────────┘
                            │
                            v
    ┌────────────────────────────────────────────────┐
    │                                                │
    │              AST Generator                      │
    │                                                │
    │  Uses oxc_parser to parse:                     │
    │    - Input source code -> AST JSON             │
    │    - Each output module code -> AST JSON       │
    │                                                │
    └───────────────────────┬────────────────────────┘
                            │
                            v
    ┌────────────────────────────────────────────────┐
    │                                                │
    │              Convention Analyzer                │
    │                                                │
    │  Walks output ASTs to catalog:                 │
    │    - Function calls (qrl, componentQrl, etc.) │
    │    - Import transformations                    │
    │    - Code movement patterns                    │
    │    - JSX transformations                       │
    │                                                │
    └───────────────────────┬────────────────────────┘
                            │
                            v
    ┌────────────────────────────────────────────────┐
    │                                                │
    │              Markdown Renderer                  │
    │                                                │
    │  Templates all data into structured markdown   │
    │  One file per snapshot in .planning/spec/      │
    │                                                │
    └────────────────────────────────────────────────┘
                            │
                            v
                        OUTPUT
    ┌──────────────────────────────────────────────────┐
    │  .planning/spec/                                 │
    │    example_1.md                                  │
    │    example_2.md                                  │
    │    ...                                           │
    │    should_wrap_store_expression.md                │
    │  (162 files)                                     │
    └──────────────────────────────────────────────────┘
```

## Component Responsibilities

| Component | Responsibility | Input | Output |
|-----------|---------------|-------|--------|
| **Snapshot Parser** | Read `.snap` files, split on `==INPUT==` / `========= [path] ==` / `== DIAGNOSTICS ==` delimiters, extract structured sections | Raw `.snap` file bytes | `ParsedSnapshot` struct with input, modules, diagnostics |
| **Test Config Extractor** | Parse `test.rs` to extract `TestInput` struct fields per test function (entry strategy, emit mode, transpile flags, etc.) | Raw `test.rs` source | `HashMap<test_name, TestConfig>` |
| **AST Generator** | Parse JavaScript/TypeScript/JSX/TSX code strings with `oxc_parser`, serialize ASTs to ESTree-compatible JSON | Code string + `SourceType` | JSON string of AST |
| **Convention Analyzer** | Walk parsed output ASTs to identify optimizer conventions: which functions are called, what imports changed, what code moved | Parsed AST (or raw output code) | `Vec<Convention>` per module |
| **Markdown Renderer** | Assemble all extracted data into a structured markdown document per test | All above outputs combined | `.md` file written to disk |

## Component Details

### 1. Snapshot Parser

The snapshot format is well-defined but has subtle variations. Each `.snap` file has this structure:

```
---
source: packages/qwik/src/optimizer/core/src/test.rs
assertion_line: <line>
expression: output
snapshot_kind: text        # sometimes absent (older snapshots)
---
==INPUT==

<input code>

============================= <module_path> <(ENTRY POINT)?>==

<output code>

Some("<source_map_json>")
/*
<segment_metadata_json>
*/

============================= <next_module_path> ...==
...

== DIAGNOSTICS ==

<diagnostics_json_array>
```

Key parsing rules:
- **Input section** starts after `==INPUT==\n\n` and ends at the first `========` delimiter
- **Module sections** are delimited by `============================= <path> ==` lines (with optional `(ENTRY POINT)` marker)
- Each module section contains: code, then `Some("<sourcemap>")` or `None`, then optionally a `/* ... */` block containing segment metadata JSON
- **Diagnostics section** starts after `== DIAGNOSTICS ==\n\n` and contains a JSON array (often just `[]`)
- The insta snapshot header (YAML frontmatter between `---`) should be stripped

**Data structure:**

```rust
struct ParsedSnapshot {
    test_name: String,            // derived from filename
    assertion_line: u32,
    input_code: String,
    modules: Vec<OutputModule>,
    diagnostics: Vec<Diagnostic>,
}

struct OutputModule {
    path: String,
    is_entry: bool,
    code: String,
    source_map: Option<String>,
    segment: Option<SegmentMetadata>,
}

struct SegmentMetadata {
    origin: String,
    name: String,
    entry: Option<String>,
    display_name: String,
    hash: String,
    canonical_filename: String,
    path: String,
    extension: String,
    parent: Option<String>,
    ctx_kind: String,           // "function" or "event"
    ctx_name: String,           // "$", "component$", "useStyles$", etc.
    captures: bool,
    loc: (u32, u32),
    param_names: Option<Vec<String>>,
    capture_names: Option<Vec<String>>,
}
```

### 2. Test Config Extractor

The 163 test functions in `test.rs` each construct a `TestInput` struct with varying fields. The extractor needs to:

1. Find each `#[test] fn <name>()` block
2. Extract the `TestInput { ... }` literal within
3. Parse the fields that differ from `TestInput::default()`

**Default values** (from `TestInput::default()`):
- `filename`: `"test.tsx"`
- `src_dir`: `"/user/qwik/src/"`
- `entry_strategy`: `EntryStrategy::Segment`
- `minify`: `MinifyMode::Simplify`
- `transpile_ts`: `false`
- `transpile_jsx`: `false`
- `preserve_filenames`: `false`
- `explicit_extensions`: `false`
- `mode`: `EmitMode::Test`
- `is_server`: `None`
- `scope`: `None`
- `core_module`: `None`
- `strip_exports`: `None`
- `strip_ctx_name`: `None`
- `reg_ctx_name`: `None`
- `strip_event_handlers`: `false`

**Approach:** Regex-based extraction, not full Rust parsing. The test file follows a consistent pattern:

```rust
#[test]
fn test_name() {
    test_input!(TestInput {
        code: r#"..."#.to_string(),
        entry_strategy: EntryStrategy::Inline,
        ..TestInput::default()
    });
}
```

Extract: test function name, any non-default field assignments. The `code` field is already in the snapshot's `==INPUT==` section, so the extractor only needs the config fields.

**Data structure:**

```rust
struct TestConfig {
    test_name: String,
    filename: String,                          // if non-default
    entry_strategy: EntryStrategy,
    minify: MinifyMode,
    mode: EmitMode,
    transpile_ts: bool,
    transpile_jsx: bool,
    preserve_filenames: bool,
    explicit_extensions: bool,
    is_server: Option<bool>,
    scope: Option<String>,
    core_module: Option<String>,
    strip_exports: Option<Vec<String>>,
    strip_ctx_name: Option<Vec<String>>,
    reg_ctx_name: Option<Vec<String>>,
    strip_event_handlers: bool,
}
```

### 3. AST Generator

Uses `oxc_parser` to parse each code string and serialize the AST. The critical design decision: **how verbose should the AST be in the spec?**

**Recommendation:** Use ESTree-compatible JSON serialization (`serde` with oxc's custom Serialize impls). The ESTree format is the standard AST representation that JavaScript tooling authors understand. Include the full AST, but render it in a collapsible markdown details block so it does not overwhelm the spec document.

**Parsing considerations:**
- Input code is always JSX/TSX (mostly `.tsx`, sometimes `.ts`)
- Output module code varies: `.tsx`, `.ts`, `.js` extensions appear in module paths
- Some output modules have intentional errors (capturing diagnostics) -- parse these best-effort
- The `SourceType` must be inferred from the module's file extension in the path

```rust
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

fn parse_to_json(code: &str, filename: &str) -> Result<String, String> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(filename)
        .unwrap_or_else(|_| SourceType::tsx());
    let ret = Parser::new(&allocator, code, source_type).parse();
    // Serialize program to ESTree JSON
    // ret.program can be serialized via serde
    serde_json::to_string_pretty(&ret.program)
        .map_err(|e| e.to_string())
}
```

**Confidence note:** The exact serialization API (`serde_json::to_string_pretty` on the program) needs verification. The oxc AST types implement `Serialize` but the ESTree-compatible serialization may require enabling specific features or using a wrapper. **MEDIUM confidence** -- verify during implementation.

### 4. Convention Analyzer

This component catalogs the optimizer conventions visible in each test's output. It does NOT need to understand the transformation logic -- it observes what changed between input and output.

**Conventions to detect:**

| Convention | Detection Method |
|-----------|-----------------|
| `$()` -> `qrl()` conversion | Function call `qrl(...)` in output where `$()` was in input |
| `component$` -> `componentQrl` | Import + call site transformation |
| `useStyles$` -> `useStylesQrl` | Import + call site transformation |
| `useBrowserVisibleTask$` -> `useBrowserVisibleTaskQrl` | Import + call site transformation |
| Lazy import generation | `const i_<hash> = () => import("./...")` pattern |
| Segment extraction | Code moved to separate module file |
| JSX transformation | `_jsxSorted`, `_jsxSplit` calls |
| Signal wrapping | `_wrapProp`, `_wrapSignal` calls |
| Inline strategy | `inlinedQrl(...)` instead of `qrl(...)` |
| Capture analysis | `_captures[N]` references, capture array in `inlinedQrl` third arg |
| `#__PURE__` annotations | `/*#__PURE__*/` comments on `qrl()` and `componentQrl()` calls |
| Code stripping | `strip_exports`, `strip_ctx_name`, `strip_event_handlers` effects |
| Build constant replacement | `isServer`, `isBrowser`, `isDev` replaced with `true`/`false` |

**Approach:** String-matching on output code is sufficient. Full AST walking is overkill for convention detection because the patterns are syntactically distinctive. Use regex patterns:

```rust
// Detect function calls
let qrl_calls = regex::Regex::new(r"(?:qrl|inlinedQrl|qrlDEV)\(").unwrap();
let jsx_calls = regex::Regex::new(r"_jsx(?:Sorted|Split|Q)\(").unwrap();
let signal_fns = regex::Regex::new(r"_(?:wrapProp|wrapSignal|getVarProps|getConstProps)\(").unwrap();
let captures = regex::Regex::new(r"_captures\[(\d+)\]").unwrap();
```

### 5. Markdown Renderer

Templates all extracted data into a single markdown file per test. The structure should be consistent and machine-readable for downstream consumers (the roadmap and eventually the OXC optimizer builder).

**Spec file template:**

```markdown
# Spec: <test_name>

## Test Configuration

| Setting | Value |
|---------|-------|
| Entry Strategy | Segment |
| Emit Mode | Test |
| ...

## Input

### Source Code

```tsx
<input code>
```

<details><summary>Input AST (ESTree JSON)</summary>

```json
<ast json>
```

</details>

## Output Modules

### Module: <path> (ENTRY POINT)

```tsx
<output code>
```

<details><summary>Output AST (ESTree JSON)</summary>

```json
<ast json>
```

</details>

#### Segment Metadata

```json
<segment analysis json>
```

#### Source Map

```json
<source map json>
```

### Module: <path>
...

## Conventions Applied

- **Segment extraction:** `$()` callback at line X extracted to `<module_path>`
- **Lazy import:** `const i_<hash> = () => import("./...")`
- **QRL conversion:** `component$` -> `componentQrl` with `qrl()` wrapper
- ...

## Function Calls in Output

| Function | Module | Count | Purpose |
|----------|--------|-------|---------|
| `qrl` | test.tsx | 2 | Lazy QRL reference |
| `componentQrl` | test.tsx | 1 | Component registration |
| `_jsxSorted` | test.tsx_App_component_... | 1 | JSX element creation |

## Diagnostics

<diagnostics or "None">
```

## Data Flow

```
┌─────────────┐     ┌─────────────────┐
│  test.rs    │     │  snapshots/*.snap│
│  (source)   │     │  (162 files)    │
└──────┬──────┘     └────────┬────────┘
       │                     │
       v                     v
┌──────────────┐    ┌────────────────┐
│ Test Config  │    │  Snapshot      │
│ Extractor    │    │  Parser        │
│              │    │                │
│ Output:      │    │ Output:        │
│ Map<name,    │    │ Vec<Parsed     │
│   config>    │    │   Snapshot>    │
└──────┬───────┘    └───────┬────────┘
       │                    │
       └────────┬───────────┘
                │
                v
       ┌────────────────┐
       │  JOIN on        │
       │  test_name      │
       │                │
       │  test_name is  │
       │  derived from  │
       │  snap filename │
       └───────┬────────┘
               │
               v
       ┌────────────────┐
       │  For each test:│
       │                │
       │  1. Parse input│    ┌──────────────┐
       │     code with  │───>│  oxc_parser  │
       │     oxc_parser │<───│  (library)   │
       │                │    └──────────────┘
       │  2. Parse each │
       │     output     │
       │     module     │
       │                │
       │  3. Detect     │
       │     conventions│
       │                │
       │  4. Render     │
       │     markdown   │
       └───────┬────────┘
               │
               v
       ┌────────────────────┐
       │  .planning/spec/   │
       │                    │
       │  162 markdown files│
       └────────────────────┘
```

**Join key:** The snapshot filename `qwik_core__test__<test_name>.snap` maps directly to the test function `fn <test_name>()` in test.rs. Strip the `qwik_core__test__` prefix and `.snap` suffix to get the test name.

## Recommended Project Structure

```
spec-gen/
├── Cargo.toml              # Dependencies: oxc_parser, oxc_ast, oxc_allocator,
│                           #   oxc_span, serde, serde_json, regex, clap
├── src/
│   ├── main.rs             # CLI entry point, argument parsing, orchestration
│   ├── snapshot.rs         # Snapshot file parser (delimiter splitting)
│   ├── test_config.rs      # test.rs parser (regex-based config extraction)
│   ├── ast.rs              # oxc_parser wrapper (parse + serialize to JSON)
│   ├── conventions.rs      # Convention detection (regex-based pattern matching)
│   ├── renderer.rs         # Markdown template rendering
│   └── types.rs            # Shared data structures (ParsedSnapshot, TestConfig, etc.)
└── tests/
    ├── snapshot_parser.rs  # Test with known snapshot content
    └── convention_detection.rs  # Test convention regex patterns
```

### Structure Rationale

- **`snapshot.rs` + `test_config.rs` separated:** They parse different file formats with different strategies (delimiter-based vs regex-based). Clean boundary.
- **`ast.rs` isolated:** Single responsibility -- wraps `oxc_parser`. If the oxc API changes, only this file changes.
- **`conventions.rs` separate from `ast.rs`:** Convention detection uses regex on code strings, not AST walking. Different concern from AST generation.
- **`renderer.rs` owns all markdown output:** Template changes are isolated here. No other module knows about markdown.
- **`types.rs` for shared structures:** All components exchange data through these types. No circular dependencies.

## Architectural Patterns

### Pattern 1: Pipeline Architecture

**What:** Each component transforms data and passes it downstream. No component reaches back upstream.

**When to use:** Data flows in one direction from input files to output files.

**Trade-offs:**
- Pro: Easy to test each stage independently
- Pro: Easy to parallelize (parse all snapshots in parallel, then render in parallel)
- Con: Cannot easily optimize across stages (e.g., skip AST generation for modules you know are simple)

The pipeline stages are: Read -> Parse -> Analyze -> Render -> Write. Each stage has a clear input/output contract.

### Pattern 2: Fail-Forward with Diagnostics

**What:** If AST parsing fails for a code snippet (some output modules have intentional errors), log a warning in the spec file rather than aborting the entire pipeline.

**When to use:** Always. Some snapshots produce code that references undefined identifiers (like `hola()` in `example_capturing_fn_class`) -- this is intentional test behavior, not a bug.

**Trade-offs:**
- Pro: One broken snapshot does not block the other 161
- Pro: The spec file itself documents the parse failure, which is useful information
- Con: Must carefully distinguish "expected parse issues" from "parser bug"

### Pattern 3: Deterministic Output

**What:** Given the same input files, the tool always produces byte-identical output. Sort modules by their order in the snapshot. Use consistent JSON formatting. No timestamps.

**When to use:** Always. Spec files will be committed to git. Non-deterministic output creates noisy diffs.

**Trade-offs:**
- Pro: Git-friendly, reviewable diffs
- Pro: Can re-run tool and verify nothing changed
- Con: Must explicitly sort any collections

## Anti-Patterns to Avoid

### Anti-Pattern 1: Full Rust Parser for test.rs

**What people do:** Use `syn` or `rust-analyzer` to parse test.rs into a full Rust AST to extract TestInput structs.

**Why it's wrong:** test.rs uses macros (`test_input!`, `snapshot_res!`) that syn cannot expand. The file also uses nightly features (`#![feature(box_patterns)]`). A full Rust parse is fragile and heavyweight for extracting config fields from what is essentially a structured text file.

**Do this instead:** Regex-based extraction. The test functions follow a rigid pattern. Match `#[test]\nfn <name>()` blocks, then extract `field_name: value` pairs within `TestInput { ... }`. This is robust because the test file format has been stable across 163 tests.

### Anti-Pattern 2: AST Diffing Between Input and Output

**What people do:** Parse both input and output ASTs, then compute a structural diff to identify transformations.

**Why it's wrong:** The optimizer fundamentally changes the code structure -- it splits one file into multiple modules, rewrites imports, extracts callbacks. A structural AST diff would be noise, not signal. The transformations are too dramatic for diffing to be useful.

**Do this instead:** Document input and output independently. Let the human reader (or future AI agent) understand the transformation by reading both. The convention analyzer catalogs patterns without trying to correlate specific AST nodes.

### Anti-Pattern 3: Embedding ASTs Inline in Markdown

**What people do:** Put the full AST JSON directly in the markdown body, making spec files thousands of lines long.

**Why it's wrong:** A typical AST for even 10 lines of code is 200+ lines of JSON. With input AST + 3 output module ASTs, a spec file would be 1000+ lines of JSON drowning 50 lines of meaningful content.

**Do this instead:** Use HTML `<details>` blocks so ASTs are collapsible. The spec is readable without expanding them, but the full AST is available when needed.

## Integration Points

### Input Files (Read-Only)

| File | Access Pattern | Notes |
|------|---------------|-------|
| `swc-optimizer/core/src/test.rs` | Read once, parse all test configs | 5387 lines, 163 test functions |
| `swc-optimizer/core/src/snapshots/*.snap` | Read each file independently | 162 files, varying sizes (20-250 lines) |

### Output Files (Write-Only)

| File | Pattern | Notes |
|------|---------|-------|
| `.planning/spec/<test_name>.md` | One per snapshot | 162 files total |

### Library Dependencies

| Dependency | Purpose | Why This One |
|-----------|---------|-------------|
| `oxc_parser` | Parse JS/TS/JSX/TSX code to AST | Same parser the OXC optimizer will use -- dogfooding the target tool |
| `oxc_ast` | AST type definitions | Required by `oxc_parser`, provides the `Program` type |
| `oxc_allocator` | Arena allocator for AST nodes | Required by `oxc_parser` |
| `oxc_span` | `SourceType` for language detection | Required to tell parser what language variant to parse |
| `serde` + `serde_json` | AST serialization to JSON | Standard Rust serialization, oxc AST types implement `Serialize` |
| `regex` | Convention detection + test config extraction | Lightweight pattern matching, no need for a parser combinator |
| `clap` | CLI argument parsing | Standard Rust CLI library |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| Snapshot Parser -> Pipeline | Returns `Vec<ParsedSnapshot>` | Pure data, no side effects |
| Test Config Extractor -> Pipeline | Returns `HashMap<String, TestConfig>` | Pure data, no side effects |
| AST Generator -> Renderer | Returns `String` (JSON) | AST stays serialized, renderer embeds it |
| Convention Analyzer -> Renderer | Returns `Vec<Convention>` | Enum-based, renderer formats to text |
| Renderer -> Filesystem | Writes markdown files | Only component that touches the output directory |

## Build Order (Phase Dependencies)

The components have clear dependency ordering:

```
Phase 1: types.rs + snapshot.rs
    │
    ├── types.rs defines all shared data structures
    │   (no dependencies on other project modules)
    │
    └── snapshot.rs can be built + tested independently
        (only depends on types.rs, reads .snap files)

Phase 2: test_config.rs (parallel with Phase 1 completion)
    │
    └── Parses test.rs, depends only on types.rs
        Can be tested independently with test.rs excerpts

Phase 3: ast.rs
    │
    └── Wraps oxc_parser
        Depends on types.rs for SourceType inference
        Can be tested with hardcoded code strings

Phase 4: conventions.rs
    │
    └── Regex-based detection
        Depends on types.rs for Convention enum
        Can be tested with known output code strings

Phase 5: renderer.rs
    │
    └── Depends on ALL types (assembles everything)
        Can be tested with mock data

Phase 6: main.rs (orchestration)
    │
    └── Wires everything together
        CLI argument parsing
        Parallel processing of snapshots
        Error reporting
```

**Key insight for roadmap:** Phases 1-4 are independently testable and can proceed in parallel after types.rs is defined. Phase 5 (renderer) is the integration point. Phase 6 is trivial glue code.

**Build the types first.** Getting the data structures right determines everything downstream. Spend time on `ParsedSnapshot`, `OutputModule`, `SegmentMetadata`, `TestConfig`, and `Convention` before writing any parsing code.

## Scaling Considerations

This is a one-shot tool processing 162 files, not a long-running service. "Scaling" means "does it run in reasonable time?"

| Concern | At 162 snapshots (current) | At 500+ snapshots (future) |
|---------|---------------------------|---------------------------|
| Parse time | Negligible (<1s total with oxc) | Still negligible -- oxc parses millions of lines/sec |
| Memory | Each snapshot is independent, <1MB each | No concern |
| Output size | ~162 markdown files, probably 5-10MB total | Linear growth, still fine |
| Parallelism | Not needed but easy (rayon over snapshot iterator) | Use rayon if it matters |

No scaling concerns. The tool processes a fixed, small dataset. Optimize for correctness and readability, not performance.

## Sources

- `swc-optimizer/core/src/test.rs` -- test structure, TestInput struct, snapshot_res macro (lines 1-50, 5339-5387)
- `swc-optimizer/core/src/parse.rs` -- TransformOutput, TransformModule, SegmentAnalysis structs (lines 40-184)
- `swc-optimizer/core/src/utils.rs` -- Diagnostic, DiagnosticCategory, DiagnosticScope structs (lines 46-71)
- `swc-optimizer/core/src/entry_strategy.rs` -- EntryStrategy enum (lines 14-22)
- `swc-optimizer/core/src/lib.rs` -- TransformModulesOptions, TransformModuleInput structs (lines 43-73)
- [oxc_parser docs.rs](https://docs.rs/oxc_parser) -- Parser API, ParserReturn struct (HIGH confidence)
- [oxc_ast docs.rs](https://docs.rs/oxc_ast/latest/oxc_ast/) -- AST types and serde Serialize implementations (MEDIUM confidence on exact serialization API)
- [oxc parser usage guide](https://oxc.rs/docs/guide/usage/parser.html) -- Usage examples (HIGH confidence)
- Snapshot files examined: `example_1.snap`, `example_build_server.snap`, `example_capturing_fn_class.snap`, `example_invalid_segment_expr1.snap`, `example_missing_custom_inlined_functions.snap`, `example_inlined_entry_strategy.snap`

---
*Architecture research for: Qwik optimizer spec generation pipeline*
*Researched: 2026-02-10*
