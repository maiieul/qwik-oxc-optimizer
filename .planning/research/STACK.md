# Stack Research

**Domain:** SWC-to-OXC AST port -- spec generation for Qwik optimizer transformations
**Researched:** 2026-02-10
**Confidence:** HIGH (verified via official docs, crates.io, GitHub source, multiple sources)

## Executive Context

The existing Qwik optimizer (`swc-optimizer/core/`) uses `swc_ecmascript`, `swc_common`, and `swc_atoms` to parse JS/TS/TSX, transform ASTs (code-splitting, tree-shaking, JSX transforms), and emit JavaScript output. The goal of this milestone is NOT to build the new optimizer yet -- it is to parse the 162 snapshot test inputs and outputs with OXC and produce spec files documenting the AST structures that the future optimizer must handle.

This means the stack for THIS milestone is focused on: **parsing**, **AST inspection**, and **serialization**. No AST mutation, no code generation, no traversal transforms are needed yet.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
|------------|---------|---------|-----------------|
| `oxc` (umbrella crate) | `0.112.0` | Single dependency for parser + AST + span + allocator | Avoids version-sync headaches across 5+ sub-crates. All OXC sub-crates are versioned in lockstep. Use the umbrella crate with feature flags to pull only what you need. |
| Rust | edition 2021 | Language | Matches existing `swc-optimizer/core` edition. No reason to change. |
| `insta` | `1.29+` | Snapshot testing for spec files | Already used in the existing crate. Supports `assert_json_snapshot!` and `assert_yaml_snapshot!` via serde, which pairs with OXC's `serialize` feature. |
| `serde` / `serde_json` | `1.x` / `1.x` | JSON serialization of OXC ASTs | Already dependencies in the existing crate. OXC's `serialize` feature implements `serde::Serialize` on all AST types. |

### OXC Sub-crates (via umbrella crate features)

The `oxc` umbrella crate re-exports these. You do NOT need to add them individually to Cargo.toml.

| Sub-crate | Accessed Via | Purpose | Notes |
|-----------|-------------|---------|-------|
| `oxc_allocator` | Always included | Arena bump allocator for AST nodes | Required. All AST nodes live in an `Allocator` arena with lifetime `'a`. |
| `oxc_parser` | Always included | Parse JS/TS/JSX/TSX to AST | Core parsing API. 3x faster than SWC parser per official benchmarks. |
| `oxc_ast` | Always included | AST type definitions (`Program`, `Statement`, `Expression`, etc.) | The AST types. Differs from ESTree: uses `BindingIdentifier` / `IdentifierReference` / `IdentifierName` instead of generic `Identifier`. |
| `oxc_span` | Always included | `SourceType`, `Span`, `Atom` types | `SourceType` is how you tell the parser what language to parse (JS/TS/JSX/TSX). |
| `oxc_syntax` | Always included | Language-level utilities | Operator precedence, number parsing helpers. |
| `oxc_diagnostics` | Always included | Error types (`OxcDiagnostic`) | Parser errors are `Vec<OxcDiagnostic>` in `ParserReturn`. |

### Feature Flags Required

```toml
[dependencies]
oxc = { version = "0.112", features = ["serialize"] }
```

**Why `serialize`:** Enables `serde::Serialize` on all AST types (`oxc_ast`, `oxc_span`, `oxc_syntax`, `oxc_allocator`). This lets you serialize parsed ASTs to JSON for spec files using `serde_json::to_string_pretty()`. Also enables the ESTree JSON methods on `Program` (`to_pretty_estree_js_json`, `to_pretty_estree_ts_json`).

**Why NOT `full`:** The `full` feature pulls in `codegen`, `minifier`, `mangler`, `semantic`, `transformer`, `isolated_declarations`. None of these are needed for parsing + serialization. Adding them increases compile time for zero benefit at this stage.

**Why NOT `ast_visit`:** Not needed for spec generation. We parse and serialize, we don't walk the tree. The `ast_visit` feature (which provides `Visit`/`VisitMut` traits) will be needed for the future optimizer milestone, not this one.

### Supporting Libraries (existing, no changes needed)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde` | `1.0.160` | Serialization framework | Already in Cargo.toml. Required by OXC `serialize` feature. |
| `serde_json` | `1.0.96` | JSON serialization | Already in Cargo.toml. Use for `serde_json::to_string_pretty(&program)`. |
| `insta` | `1.29.0` | Snapshot testing | Already in dev-dependencies. Use `assert_json_snapshot!` or `assert_snapshot!` for spec files. |

## How to Parse JS/TS/TSX with OXC

### Minimal Parsing Example

```rust
use oxc::allocator::Allocator;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;

fn parse_code(source_text: &str, filename: &str) -> String {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(filename).unwrap();

    let ret = Parser::new(&allocator, source_text, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    if !ret.errors.is_empty() {
        for error in &ret.errors {
            eprintln!("Parse error: {error:?}");
        }
    }

    // Option A: Rust Debug format (verbose, includes all fields)
    format!("{:#?}", ret.program)
}
```

### SourceType Construction

```rust
// From file path (auto-detects from extension):
let st = SourceType::from_path("component.tsx").unwrap();  // TypeScript + JSX
let st = SourceType::from_path("utils.ts").unwrap();       // TypeScript
let st = SourceType::from_path("index.js").unwrap();       // JavaScript
let st = SourceType::from_path("app.jsx").unwrap();        // JavaScript + JSX

// From explicit constructors (when you don't have a real filename):
let st = SourceType::tsx();   // TypeScript + JSX
let st = SourceType::ts();    // TypeScript, no JSX
let st = SourceType::jsx();   // JavaScript + JSX
let st = SourceType::mjs();   // JavaScript ESM

// Builder pattern for fine control:
let st = SourceType::tsx().with_module(true);
```

### Three Ways to Serialize the AST

**1. Rust Debug format** (`{:#?}`) -- Best for human-readable spec files:
```rust
let debug_output = format!("{:#?}", ret.program);
```
- Pro: Shows exact Rust types, field names, all data
- Pro: No feature flags needed beyond default
- Con: Verbose, Rust-specific notation

**2. Serde JSON** -- Best for machine-readable spec files:
```rust
// Requires: oxc = { features = ["serialize"] }
let json = serde_json::to_string_pretty(&ret.program).unwrap();
```
- Pro: Standard format, easy to diff
- Pro: Consumable by any language
- Con: Large output for complex ASTs

**3. ESTree JSON** -- Best for cross-tool compatibility:
```rust
// Requires: oxc = { features = ["serialize"] }
// For JavaScript source:
let estree = ret.program.to_pretty_estree_js_json(false);
// For TypeScript source:
let estree = ret.program.to_pretty_estree_ts_json(false);
```
- Pro: Standard ESTree format, matches what JS tools produce
- Pro: Can be compared against Babel/acorn ASTs
- Con: Loses OXC-specific type distinctions (merges BindingIdentifier/IdentifierReference back to Identifier)

**Recommendation for spec files:** Use **Serde JSON** (option 2). It preserves OXC's richer type distinctions (which the optimizer needs to understand), is diffable, and can be consumed programmatically when building the actual optimizer. ESTree JSON is useful for cross-referencing but hides information the optimizer will need.

### ParserReturn Structure

```rust
pub struct ParserReturn<'a> {
    pub program: Program<'a>,          // The AST
    pub module_record: ModuleRecord<'a>, // ESM import/export metadata
    pub errors: Vec<OxcDiagnostic>,    // Parse errors
    pub irregular_whitespaces: Box<[Span]>,
    pub panicked: bool,                // Unrecoverable error?
    pub is_flow_language: bool,        // Flow type annotations detected?
}
```

The `program` field is what you serialize. The `module_record` contains import/export metadata that could be useful for documenting the optimizer's module analysis.

## Key AST Differences: SWC vs OXC

Understanding these differences is critical for writing accurate specs.

| Concept | SWC (`swc_ecmascript::ast`) | OXC (`oxc::ast::ast`) |
|---------|----------------------------|----------------------|
| Memory model | Standard heap allocation (`Box`, `Vec`) | Arena allocation (`oxc::allocator::Box`, `oxc::allocator::Vec`) with lifetime `'a` |
| Identifier types | Single `Ident` type | Three types: `BindingIdentifier`, `IdentifierReference`, `IdentifierName` |
| Pattern type | `Pat` enum | `BindingPattern` + `AssignmentTarget` (split by semantic role) |
| Atoms/strings | `swc_atoms::Atom` (string interning) | `oxc::span::Atom` (arena-inlined string) |
| Spans | `swc_common::Span` with `BytePos` | `oxc::span::Span` with `u32` start/end offsets |
| Node wrapper | `Spanned` trait | `span` field directly on each struct |
| Program | `ast::Program` enum (Module or Script) | `ast::Program` struct with `source_type` + `body` |
| Comments | `SingleThreadedComments` (separate structure) | `program.comments` field directly on Program |
| Visitor pattern | `Visit`/`VisitMut`/`Fold` traits | `Visit`/`VisitMut` (read) + `Traverse` (mutable with parent context) |

## Installation

```toml
# In the spec-generation crate's Cargo.toml:

[dependencies]
oxc = { version = "0.112", features = ["serialize"] }
serde = "1.0"
serde_json = "1.0"

[dev-dependencies]
insta = { version = "1.29", features = ["json"] }
```

Note: Use `insta` with the `json` feature to get `assert_json_snapshot!` macro support, which pairs naturally with OXC's serialized ASTs.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
|-------------|-------------|-------------------------|
| `oxc` umbrella crate | Individual `oxc_parser` + `oxc_ast` + `oxc_allocator` + `oxc_span` crates | Never for this project. Individual crates require manual version synchronization across 5+ crates that must all be the same version. The umbrella crate handles this. |
| `oxc` umbrella crate | `swc_ecmascript` (current) | Only if OXC parsing fails for specific edge cases. SWC is the incumbent but slower (3x) and uses more memory. |
| Serde JSON serialization | Rust Debug format (`{:#?}`) | For quick debugging or when you want to see exact Rust types. Not recommended for committed spec files because it's Rust-specific and harder to diff. |
| Serde JSON serialization | ESTree JSON (`to_pretty_estree_*_json`) | When comparing against Babel/acorn output or when the spec consumer is a JavaScript tool. Loses OXC-specific type distinctions. |
| `insta` snapshots | Manual file I/O | Never. `insta` provides diffing, review workflow (`cargo insta review`), and CI integration that manual file management cannot match. |

## What NOT to Use

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `oxc` with `features = ["full"]` | Pulls in codegen, minifier, mangler, semantic, transformer -- all unnecessary for spec generation. Adds ~30s+ to compile time. | `oxc = { features = ["serialize"] }` |
| Individual OXC sub-crates (`oxc_parser`, `oxc_ast`, etc.) | Version sync nightmare. All must be exactly the same version. Umbrella crate solves this. | `oxc` umbrella crate |
| `oxc_traverse` / `oxc_ast_visit` directly | Not needed for spec generation (parse + serialize only). Will be needed for the optimizer milestone. | Skip for now, add `ast_visit` feature when building the optimizer. |
| `swc_ecmascript` for parsing in the spec crate | The whole point is to document OXC ASTs. Using SWC to parse defeats the purpose. | `oxc` |
| `oxc_codegen` | Code generation (AST to JS string) is not needed for spec generation. Will be needed for the optimizer milestone. | Skip entirely for this milestone. |
| Pinning OXC to exact version (e.g., `=0.112.0`) | OXC is pre-1.0 and releases frequently (weekly). Semver-compatible ranges (`0.112`) let you get bugfixes. Pin only if a specific version breaks something. | `version = "0.112"` (allow patch updates) |

## Stack Patterns by Variant

**If generating spec files from snapshot test fixtures:**
- Parse each `.txt` / `.js` / `.tsx` fixture with `oxc::parser::Parser`
- Serialize AST with `serde_json::to_string_pretty()`
- Store as `.json` spec files alongside or via `insta` snapshots
- Because the allocator owns the AST lifetime, each parse must happen in its own scope

**If generating spec files from inline code strings:**
- Same approach, but `SourceType` must be constructed explicitly (no file path to infer from)
- Use `SourceType::tsx()` as the default since most Qwik components are TSX

**If comparing SWC AST vs OXC AST for the same input:**
- Parse with both SWC and OXC in separate functions
- Serialize both to JSON
- Use structural diff tooling or `insta` inline snapshots to document differences
- This is useful for identifying where OXC's richer type system changes the spec

## Version Compatibility

| Package | Compatible With | Notes |
|---------|-----------------|-------|
| `oxc` 0.112.x | Rust edition 2021 | Matches existing crate edition |
| `oxc` 0.112.x | `serde` 1.x | OXC's `serialize` feature generates `serde::Serialize` impls |
| `oxc` 0.112.x | `serde_json` 1.x | For JSON serialization of ASTs |
| `oxc` 0.112.x | `insta` 1.29+ | `insta`'s `json` feature + OXC's `serialize` feature = `assert_json_snapshot!` on AST nodes |
| `oxc` 0.112.x | `swc_ecmascript` * | Can coexist in same workspace (different crates). No conflicts -- they're completely independent Rust crates. |

## Critical Note: Arena Allocator Lifetime

OXC's `Allocator` owns all AST memory. The `Program<'a>` lifetime is tied to the `Allocator`. This means:

```rust
// WRONG: Allocator dropped, program is dangling
fn parse_broken(code: &str) -> Program {  // <-- no lifetime!
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, code, SourceType::tsx()).parse();
    ret.program  // ERROR: program borrows allocator, which is dropped here
}

// RIGHT: Serialize within allocator scope
fn parse_to_json(code: &str) -> String {
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, code, SourceType::tsx()).parse();
    serde_json::to_string_pretty(&ret.program).unwrap()  // Serialize before allocator drops
}

// RIGHT: Keep allocator alive alongside program
fn parse_with_allocator(code: &str) -> (Allocator, /* can't return Program separately */) {
    // The program's lifetime is tied to the allocator.
    // Either serialize in-scope or pass the allocator along.
}
```

This lifetime constraint is the single most important API difference from SWC, where ASTs are heap-allocated and freely movable. For spec generation this is simple (parse, serialize, done), but the future optimizer will need careful lifetime management.

## Sources

- [oxc crate on crates.io](https://crates.io/crates/oxc) -- version 0.112.0, umbrella crate (MEDIUM confidence, crates.io verified)
- [oxc_parser on crates.io](https://crates.io/crates/oxc_parser) -- version 0.112.0 (MEDIUM confidence, crates.io verified)
- [oxc_parser docs.rs](https://docs.rs/oxc_parser) -- Parser API, ParserReturn struct (HIGH confidence, official docs)
- [oxc_ast docs.rs](https://docs.rs/oxc_ast) -- AST types, serialize feature (HIGH confidence, official docs)
- [oxc_allocator docs.rs](https://docs.rs/oxc_allocator/latest/oxc_allocator/) -- Arena allocator API (HIGH confidence, official docs)
- [oxc_span SourceType docs.rs](https://docs.rs/oxc_span/latest/oxc_span/struct.SourceType.html) -- SourceType constructors and methods (HIGH confidence, official docs)
- [OXC parser usage guide](https://oxc.rs/docs/guide/usage/parser.html) -- Official usage documentation (HIGH confidence)
- [OXC parser example on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/examples/parser.rs) -- Complete working example (HIGH confidence, primary source)
- [OXC AST design docs](https://oxc.rs/docs/learn/parser_in_rust/ast) -- AST structure explanation (HIGH confidence)
- [OXC feature flags on lib.rs](https://lib.rs/crates/oxc/features) -- Feature flag documentation (HIGH confidence)
- [OXC umbrella Cargo.toml](https://github.com/oxc-project/oxc/blob/main/crates/oxc/Cargo.toml) -- Dependency structure (HIGH confidence, primary source)
- [OXC GitHub releases](https://github.com/oxc-project/oxc/releases) -- Release history (HIGH confidence)
- [OXC benchmark: parser comparison](https://github.com/oxc-project/bench-javascript-parser-written-in-rust) -- "3x faster than SWC" claim (MEDIUM confidence, OXC team's own benchmark)
- [OXC AST serialization issue #6347](https://github.com/oxc-project/oxc/issues/6347) -- serde::Serialize implementation details (MEDIUM confidence)

---
*Stack research for: Qwik optimizer SWC-to-OXC spec generation*
*Researched: 2026-02-10*
