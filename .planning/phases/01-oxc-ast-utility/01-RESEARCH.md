# Phase 1: OXC AST Utility - Research

**Researched:** 2026-02-10
**Domain:** Rust CLI utility for OXC parser AST-to-JSON serialization
**Confidence:** HIGH

## Summary

Phase 1 is deliberately narrow: build a small Rust binary that accepts a code string and file extension, parses with `oxc_parser`, and outputs the AST as JSON to stdout. This is NOT the full spec-generation pipeline (that is Claude's job in Phase 2). This utility exists solely because Claude cannot run Rust code directly -- it needs a compiled binary to invoke for AST generation.

The OXC ecosystem has matured significantly. The `oxc` umbrella crate at version 0.113.0 (released 2026-02-10) provides everything needed via the `serialize` feature flag. The official parser example demonstrates exactly the pattern needed: parse code, optionally convert spans to UTF-16, then call `program.to_pretty_estree_js_json()` or `program.to_pretty_estree_ts_json()` to produce ESTree-compatible JSON. No `serde_json` dependency is needed for the primary serialization path -- OXC has its own built-in ESTree serializer.

The utility should be a standalone Rust crate (no workspace relationship with the existing `swc-optimizer/core`). It reads code from stdin (to handle large inputs and avoid shell escaping issues) and takes the file extension as a CLI argument. Error handling is critical: parse errors must appear in the JSON output (or alongside it), not cause panics.

**Primary recommendation:** Create a minimal Rust binary crate using `oxc = { version = "0.113", features = ["serialize"] }` that reads code from stdin, parses with `oxc_parser`, and writes ESTree JSON to stdout. Use `to_pretty_estree_ts_json` for TypeScript/TSX and `to_pretty_estree_js_json` for JavaScript/JSX.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `oxc` (umbrella) | `0.113` | Parser + AST + span + allocator + ESTree serializer | Single dependency avoids version-sync across 5+ sub-crates. All OXC sub-crates versioned in lockstep. |
| Rust | edition 2024 | Language | Current stable edition. No compatibility concerns with oxc. |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `serde_json` | `1.x` | Structured error output as JSON | Only if parse errors need to be emitted as structured JSON alongside the AST. |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `to_pretty_estree_*_json` methods | `serde_json::to_string_pretty` via serde Serialize trait | ESTree methods are the officially supported path. serde_json may still work but OXC is migrating away from serde derive macros toward custom ESTree serialization. |
| `to_pretty_estree_ts_json` for all code | `to_pretty_estree_js_json` for JS files | Using the TS variant for all code would work (TS is a superset) but produces slightly different JSON for TS-specific nodes. Match to actual source type for accuracy. |
| stdin for code input | CLI argument string | stdin avoids shell escaping issues, supports arbitrarily large inputs, works well when Claude invokes via `echo "code" \| ./utility`. |
| Debug format `{:#?}` | ESTree JSON | Debug format shows Rust types and field names. Useful for debugging but not for spec files -- ESTree JSON is the standard AST format understood by JS tooling. |

**Installation (Cargo.toml):**
```toml
[package]
name = "oxc-ast-util"
version = "0.1.0"
edition = "2024"

[dependencies]
oxc = { version = "0.113", features = ["serialize"] }
```

## Architecture Patterns

### Recommended Project Structure
```
oxc-ast-util/
  Cargo.toml
  src/
    main.rs          # Single-file binary: arg parsing, stdin read, parse, serialize, stdout write
```

This is intentionally minimal. A single `main.rs` file is sufficient for a utility this simple. No library crate, no modules, no tests directory. The success criteria are testable by invoking the compiled binary.

### Pattern 1: Parse-Serialize-in-Scope
**What:** The OXC allocator owns AST memory via arena allocation. The `Program<'a>` lifetime is tied to the `Allocator`. You must serialize the AST before the allocator goes out of scope.
**When to use:** Always -- this is the only correct pattern.
**Example:**
```rust
// Source: https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/examples/parser.rs
use oxc::allocator::Allocator;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;

fn parse_to_json(code: &str, source_type: SourceType) -> String {
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, code, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    let mut program = ret.program;

    // For ESTree output, convert spans from UTF-8 byte offsets to UTF-16 code unit offsets
    // This step is required for ESTree compliance
    oxc::ast_visit::utf8_to_utf16::Utf8ToUtf16::new(code).convert_program(&mut program);

    if source_type.is_javascript() {
        program.to_pretty_estree_js_json(false)
    } else {
        program.to_pretty_estree_ts_json(false)
    }
}
```

### Pattern 2: SourceType from Extension
**What:** Use `SourceType::from_path()` to auto-detect language from file extension, with a TSX fallback.
**When to use:** When you have a filename or extension string from the CLI.
**Example:**
```rust
// Source: https://docs.rs/oxc_span/latest/oxc_span/struct.SourceType.html
use oxc::span::SourceType;

// From a full filename:
let st = SourceType::from_path("component.tsx").unwrap();

// From just an extension, construct a fake filename:
let ext = "tsx";
let fake_filename = format!("input.{}", ext);
let st = SourceType::from_path(&fake_filename).unwrap_or_else(|_| SourceType::tsx());

// Convenience constructors:
let st = SourceType::tsx();   // TypeScript + JSX
let st = SourceType::ts();    // TypeScript, no JSX
let st = SourceType::jsx();   // JavaScript + JSX
let st = SourceType::mjs();   // JavaScript ESM
```

### Pattern 3: Graceful Error Reporting
**What:** Report parse errors in a structured way rather than panicking. Some code snippets (extracted optimizer output modules) may have intentional parse issues.
**When to use:** Always -- the utility must handle malformed code.
**Example:**
```rust
let ret = Parser::new(&allocator, code, source_type).parse();

if ret.panicked {
    eprintln!("FATAL: Parser panicked (unrecoverable error)");
    std::process::exit(2);
}

if !ret.errors.is_empty() {
    eprintln!("Parse errors ({}):", ret.errors.len());
    for error in &ret.errors {
        let error = error.clone().with_source_code(code.to_string());
        eprintln!("  {error:?}");
    }
    // Still emit the AST -- partial parse results are useful
}

// Proceed to serialize program even with errors
```

### Anti-Patterns to Avoid
- **Returning Program from a function:** The `Program<'a>` is tied to the `Allocator`'s lifetime. Never try to return it from a function where the allocator is a local variable. Serialize inside the same scope.
- **Using `serde_json::to_string_pretty` instead of ESTree methods:** OXC has moved to custom ESTree serialization. The ESTree methods (`to_pretty_estree_js_json` / `to_pretty_estree_ts_json`) are the officially supported path per the parser example. Using serde_json may work but is not the recommended approach.
- **Skipping the Utf8ToUtf16 conversion:** The ESTree spec uses UTF-16 code unit offsets for spans. Without the conversion step, span values in the JSON will be UTF-8 byte offsets, which differ for any source containing multi-byte characters.
- **Hardcoding SourceType::tsx() for all inputs:** While TSX is a superset, using the wrong SourceType means the AST JSON will contain TypeScript-specific node types for plain JavaScript input. Match the SourceType to the actual file extension for accurate ASTs.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JS/TS/JSX/TSX parsing | Custom parser or regex-based extraction | `oxc_parser::Parser` | Parsing JS/TS is a solved, extremely complex problem. OXC passes all Test262 and 99% of Babel/TypeScript parser tests. |
| AST-to-JSON serialization | Custom serde Serialize impl or manual JSON builder | `program.to_pretty_estree_js_json()` / `to_pretty_estree_ts_json()` | OXC's built-in ESTree serializer handles all the complexity of mapping OXC's Rust types to ESTree-compatible JSON, including identifier type merging, enum flattening, and type tagging. |
| Language detection from extension | Manual match on extension strings | `SourceType::from_path()` | Handles `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs`, `.mts`, `.cts` and more. Edge cases already handled. |
| CLI argument parsing | Manual `std::env::args()` parsing | Simple manual parsing (1 required arg) is fine for this utility | The utility takes exactly one argument (the file extension). `clap` would be overkill. A simple `std::env::args().nth(1)` is appropriate. |

**Key insight:** This utility is essentially a thin wrapper around OXC's parser and serializer. There is almost no custom logic needed. The value is in compiling and shipping a binary that Claude can invoke.

## Common Pitfalls

### Pitfall 1: Allocator Lifetime Escape
**What goes wrong:** Trying to return `Program<'a>` from a function where `Allocator` is a local variable causes a compile error.
**Why it happens:** OXC uses arena allocation. All AST nodes are owned by the allocator. When the allocator drops, all AST memory is freed.
**How to avoid:** Serialize the AST to a `String` within the same scope as the allocator. The `String` owns its data and can be freely returned/printed.
**Warning signs:** Compiler error: "borrowed value does not live long enough" or lifetime mismatch errors involving `'a`.

### Pitfall 2: Wrong ESTree Method for Source Type
**What goes wrong:** Using `to_pretty_estree_ts_json` for a JavaScript file or vice versa produces subtly wrong AST JSON. TypeScript-specific node types appear where they should not, or TypeScript type annotations are missing from the JSON.
**Why it happens:** The two methods serialize different sets of AST node types. The TS variant includes TypeScript-specific nodes; the JS variant does not.
**How to avoid:** Check `source_type.is_javascript()` and branch accordingly, exactly as the official parser example does.
**Warning signs:** AST JSON contains unexpected `TS*` node types for `.js`/`.jsx` files, or is missing type annotations for `.ts`/`.tsx` files.

### Pitfall 3: Not Handling stdin Correctly
**What goes wrong:** Code read from stdin is truncated, mangled, or contains unwanted trailing newlines.
**Why it happens:** Code strings can contain special characters, multiple lines, embedded quotes, etc. If passed as CLI arguments, shell escaping corrupts the input.
**How to avoid:** Read code from stdin using `std::io::read_to_string(std::io::stdin())`. This handles arbitrary content correctly. The caller pipes code in: `echo '...' | ./oxc-ast-util tsx`.
**Warning signs:** Parse errors on code that parses fine in other tools. Missing lines. Backslash sequences being interpreted.

### Pitfall 4: Forgetting the Utf8ToUtf16 Span Conversion
**What goes wrong:** AST JSON span offsets are in UTF-8 byte positions instead of UTF-16 code unit positions. For ASCII-only code this is invisible. For code with non-ASCII characters (emoji, CJK, etc.) spans are wrong.
**Why it happens:** OXC's parser natively uses UTF-8 byte offsets (natural for Rust). ESTree specifies UTF-16 code unit offsets (natural for JavaScript). The conversion is a separate step.
**How to avoid:** Call `Utf8ToUtf16::new(code).convert_program(&mut program)` before serialization, as shown in the official example.
**Warning signs:** Span offsets don't match when compared against Babel/acorn ASTs for the same input.

### Pitfall 5: Large JSON Output and Markdown Embedding
**What goes wrong:** AST JSON for even moderate code snippets is hundreds of lines. Embedding directly in markdown makes spec files unreadable.
**Why it happens:** ASTs are inherently verbose -- every token, span, and nested structure is represented.
**How to avoid:** This is Phase 2's concern (spec file generation), but the utility should output valid JSON that can be wrapped in `<details>` blocks. Ensure the JSON does not contain unescaped HTML characters that would break markdown rendering. ESTree JSON should be clean in this regard, but verify.
**Warning signs:** Markdown rendering breaks (unclosed tags, mangled output) when AST JSON contains angle brackets or ampersands.

## Code Examples

Verified patterns from official sources:

### Complete Main Function
```rust
// Based on: https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/examples/parser.rs
// Adapted for the utility's requirements
use std::io::Read;

use oxc::allocator::Allocator;
use oxc::ast_visit::utf8_to_utf16::Utf8ToUtf16;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;

fn main() {
    // Get file extension from args
    let ext = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: oxc-ast-util <extension>");
        eprintln!("  Reads source code from stdin, outputs ESTree JSON AST to stdout.");
        eprintln!("  Extensions: js, jsx, ts, tsx, mjs, cjs, mts, cts");
        std::process::exit(1);
    });

    // Read code from stdin
    let mut code = String::new();
    std::io::stdin().read_to_string(&mut code).unwrap_or_else(|e| {
        eprintln!("Failed to read stdin: {e}");
        std::process::exit(1);
    });

    // Determine source type from extension
    let fake_filename = format!("input.{ext}");
    let source_type = SourceType::from_path(&fake_filename).unwrap_or_else(|_| {
        eprintln!("Unknown extension: {ext}");
        std::process::exit(1);
    });

    // Parse
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &code, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    // Report errors to stderr (but still emit AST)
    if ret.panicked {
        eprintln!("FATAL: Parser panicked on input");
        std::process::exit(2);
    }

    if !ret.errors.is_empty() {
        eprintln!("Parse errors ({}):", ret.errors.len());
        for error in &ret.errors {
            let error = error.clone().with_source_code(code.clone());
            eprintln!("  {error:?}");
        }
    }

    // Convert spans to UTF-16 for ESTree compliance
    let mut program = ret.program;
    Utf8ToUtf16::new(&code).convert_program(&mut program);

    // Serialize to ESTree JSON
    let json = if source_type.is_javascript() {
        program.to_pretty_estree_js_json(false)
    } else {
        program.to_pretty_estree_ts_json(false)
    };

    println!("{json}");
}
```

### Invoking the Utility (from Claude or shell)
```bash
# Parse TypeScript JSX
echo 'import { component$ } from "@qwik.dev/core";
export const App = component$(() => {
  return <div>Hello</div>;
});' | ./oxc-ast-util tsx

# Parse plain JavaScript
echo 'export function hello() { return 42; }' | ./oxc-ast-util js

# Parse from a file
cat swc-optimizer/core/src/fixtures/index.qwik.mjs | ./oxc-ast-util mjs
```

### ParserReturn Structure
```rust
// Source: https://docs.rs/oxc_parser/latest/oxc_parser/struct.ParserReturn.html
pub struct ParserReturn<'a> {
    pub program: Program<'a>,           // The AST
    pub module_record: ModuleRecord<'a>, // ESM import/export metadata
    pub errors: Vec<OxcDiagnostic>,     // Parse errors (non-fatal)
    pub irregular_whitespaces: Box<[Span]>,
    pub panicked: bool,                 // Unrecoverable parser error
    pub is_flow_language: bool,         // Flow type annotations detected
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `serde::Serialize` derive on AST types | Custom `ESTree` derive via `#[generate_derive(ESTree)]` + `oxc_estree` crate | 2025 (Issue #6347, completed) | `serde_json::to_string_pretty` may still work but is no longer the primary path. Use `to_pretty_estree_*_json` methods instead. |
| Individual sub-crates (`oxc_parser`, `oxc_ast`, `oxc_allocator`, `oxc_span`) | `oxc` umbrella crate with feature flags | Stable since early 2025 | Single dependency line in Cargo.toml. Version sync handled automatically. |
| `serialize` feature enabled `serde::Serialize` | `serialize` feature enables ESTree JSON serialization via `oxc_estree` | Late 2025 | The `serialize` feature now pulls in `oxc_estree` (custom serializer, not serde-based) and `oxc_ast_visit` (for the `Utf8ToUtf16` converter). |

**Deprecated/outdated:**
- **Prior project research stated version 0.112.0**: Current latest is 0.113.0 (released 2026-02-10). Use `0.113`.
- **Prior research recommended `serde_json::to_string_pretty(&ret.program)`**: This may still work but is not the officially demonstrated path. The official parser example uses `program.to_pretty_estree_js_json(false)` / `program.to_pretty_estree_ts_json(false)`.
- **Prior research noted `serde_json` as a required dependency**: With the ESTree methods, `serde_json` is not needed for AST serialization. It may be useful only for auxiliary structured output (error reports, etc.).

## Open Questions

1. **Does `serde_json::to_string_pretty(&program)` still work with `serialize` feature?**
   - What we know: OXC migrated from serde derive to custom ESTree derive. The `serialize` feature now enables `oxc_estree`, not direct serde support. The official example does not use serde_json.
   - What's unclear: Whether the migration removed `serde::Serialize` impls entirely or whether they coexist with the ESTree system. The docs.rs page for Program does not show Serialize in its trait impls.
   - Recommendation: Use the ESTree methods (`to_pretty_estree_*_json`). They are the officially supported path. If serde_json is also needed (e.g., for different JSON format), validate during implementation.

2. **Is the `Utf8ToUtf16` conversion step strictly necessary for our use case?**
   - What we know: ESTree spec requires UTF-16 offsets. The official example performs this conversion. Most Qwik optimizer test code is ASCII-only, so byte offsets and UTF-16 offsets would be identical.
   - What's unclear: Whether the `to_pretty_estree_*_json` methods work correctly without the conversion (they would just emit UTF-8 offsets labeled as "start"/"end").
   - Recommendation: Include the conversion step. It is cheap, correct, and matches the official example. Omitting it would produce subtly wrong spans for any non-ASCII code.

3. **Rust edition 2024 vs 2021?**
   - What we know: The existing `swc-optimizer/core` uses edition 2021. OXC is compatible with both.
   - What's unclear: Whether edition 2024 introduces any issues with OXC or the project's CI.
   - Recommendation: Use edition 2024 (current) for the new crate. It is a new standalone crate with no compatibility constraints. If issues arise, downgrade to 2021.

## Sources

### Primary (HIGH confidence)
- [oxc crate on crates.io](https://crates.io/crates/oxc) - Version 0.113.0, verified via API on 2026-02-10
- [oxc crate versions (API)](https://crates.io/api/v1/crates/oxc/versions) - Version history confirming 0.113.0 released 2026-02-10
- [OXC umbrella Cargo.toml on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc/Cargo.toml) - Feature flags definition, serialize feature dependencies
- [OXC parser example (parser.rs)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_parser/examples/parser.rs) - Official example showing parse + ESTree JSON serialization pattern including Utf8ToUtf16 conversion
- [OXC parser usage guide](https://oxc.rs/docs/guide/usage/parser.html) - Official usage documentation
- [oxc_ast Cargo.toml on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast/Cargo.toml) - serialize feature enables oxc_estree, not serde directly
- [oxc_ast lib.rs on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast/src/lib.rs) - serialize feature described as "serialization to ESTree JSON"
- [oxc_ast js.rs on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast/src/ast/js.rs) - Program struct with `#[generate_derive(ESTree)]`

### Secondary (MEDIUM confidence)
- [Issue #6347: Implement serde::Serialize via generate_derive](https://github.com/oxc-project/oxc/issues/6347) - Migration from serde to custom ESTree derive (closed as completed)
- [Issue #2463: serialize to estree](https://github.com/oxc-project/oxc/issues/2463) - ESTree serialization effort (closed as completed)
- [oxc_estree Cargo.toml on GitHub](https://github.com/oxc-project/oxc/blob/main/crates/oxc_estree/Cargo.toml) - Custom serializer, does NOT depend on serde
- [OXC AST design docs](https://oxc.rs/docs/learn/parser_in_rust/ast) - AST structure, serde techniques for ESTree compatibility
- Prior project research: `.planning/research/STACK.md` - OXC stack analysis (partially outdated on serialization approach)

### Tertiary (LOW confidence)
- [WebSearch: oxc crate latest version](https://crates.io/crates/oxc) - Initial search returned 0.94.0 (stale cache), corrected via API
- Rust edition 2024 compatibility with OXC - unverified, based on general Rust edition compatibility expectations

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Verified via crates.io API and official GitHub source
- Architecture: HIGH - Based on official parser example and project requirements
- Pitfalls: HIGH - Arena allocator lifetime is well-documented; ESTree serialization methods verified from official example
- Serialization approach: MEDIUM - ESTree methods confirmed as official path, but serde_json fallback status unclear

**Research date:** 2026-02-10
**Valid until:** 2026-02-24 (OXC releases weekly; core API is stable but version number and details may change)
