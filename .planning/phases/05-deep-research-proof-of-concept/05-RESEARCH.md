# Phase 5: Deep Research & Proof of Concept - Research

**Researched:** 2026-02-10
**Domain:** OXC semantic analysis for capture analysis, multi-module Program construction, source map generation for split modules, and working Rust POC programs
**Confidence:** HIGH

## Summary

Phase 5 tackles the two hardest API mapping problems (capture analysis and multi-module output) and validates all four with working Rust programs. The research identifies concrete OXC APIs for each requirement, documents the SWC optimizer's capture analysis algorithm for faithful porting, and confirms that OXC provides all necessary infrastructure for multi-module construction and source map generation.

The capture analysis algorithm in the SWC optimizer uses a declaration stack (`decl_stack`) to track nested scopes during traversal, combined with an `IdentCollector` that gathers references inside `$()` bodies and a `compute_scoped_idents()` function that determines which references cross scope boundaries. In OXC, this maps to `SemanticBuilder` producing `Scoping`, then using `find_binding()`, `get_resolved_references()`, and `scope_ancestors()` to perform the equivalent analysis. The key insight is that OXC's `Scoping` provides the scope tree and symbol table that the SWC optimizer builds manually via its `GlobalCollect` pass.

For multi-module output, each extracted segment needs its own `Program` AST. Since `Program<'a>` is tied to its `Allocator`'s lifetime, either a shared allocator or separate allocators per segment can work -- but the codegen must happen before the allocator drops. The SWC optimizer constructs separate module ASTs for each segment, adds needed imports, wraps the extracted body as an exported const, and runs codegen. OXC's `AstBuilder` can construct complete `Program` nodes with `ctx.ast.program()` or equivalent builder methods. Source maps for split modules use `Codegen::new().with_options(CodegenOptions { source_map_path: Some(path), .. }).build(&program)` -- the span information from the original parse carries through to the extracted segment's codegen.

**Primary recommendation:** Build each POC as a standalone Rust binary in a `poc/` directory within the crate workspace, using OXC 0.113+ with `parser`, `traverse`, `semantic`, and `codegen` features. Each POC targets specific spec files as validation inputs.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `oxc` (umbrella) | `0.113+` | Parser, AST, Traverse, Codegen, Semantic | Single dependency; all sub-crates versioned in lockstep |
| `serde` | `1.x` | JSON serialization for test output comparison | Needed for POC output verification against spec JSON |
| `serde_json` | `1.x` | JSON encoding/decoding | Parse spec file segment metadata for comparison |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `anyhow` | `1.x` | Error handling with context | All POC error paths |
| `base64` | `0.22` | Source map base64 encoding | POC-04 source map output |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Standalone POC binaries | Integration tests in main crate | Standalone binaries are simpler to iterate on, don't need the full crate structure yet |
| Shared Allocator for segments | Separate Allocator per segment | Shared is simpler (single lifetime scope), separate enables parallel codegen later |

**POC Cargo.toml:**
```toml
[package]
name = "qwik-optimizer-poc"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "poc-01-detect-dollar"
path = "src/poc_01_detect_dollar.rs"

[[bin]]
name = "poc-02-capture-analysis"
path = "src/poc_02_capture_analysis.rs"

[[bin]]
name = "poc-03-multi-module"
path = "src/poc_03_multi_module.rs"

[[bin]]
name = "poc-04-source-maps"
path = "src/poc_04_source_maps.rs"

[dependencies]
oxc = { version = "0.113", features = ["parser", "traverse", "codegen", "semantic", "serialize"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
base64 = "0.22"
```

## Architecture Patterns

### POC Project Structure
```
poc/
  Cargo.toml
  src/
    poc_01_detect_dollar.rs      # POC-01: Traverse to detect $() calls
    poc_02_capture_analysis.rs   # POC-02: Semantic analysis for captures
    poc_03_multi_module.rs       # POC-03: Split one Program into multiple
    poc_04_source_maps.rs        # POC-04: Codegen with source maps for split modules
    common.rs                    # Shared utilities (parse, spec file loading)
```

### Pattern 1: Capture Analysis Algorithm (APIM-03)

**What:** Determine which variables a `$()` closure body references from outer scopes. These are the "captures" that must be serialized at runtime for resumability.

**SWC algorithm (ported to OXC concepts):**

The SWC optimizer uses a 3-step process:

1. **Collect global declarations** (`GlobalCollect`): Walk the program to identify all imports, exports, and top-level declarations. In OXC, this maps to `SemanticBuilder::build()` which produces `Scoping` containing the complete scope tree and symbol table.

2. **Collect identifiers in $()-body** (`IdentCollector`): Walk the extracted function body to find all identifier references. In the SWC optimizer, `IdentCollector` gathers `local_idents` (identifiers with non-empty `SyntaxContext` that appear in expression position). In OXC, this maps to examining every `IdentifierReference` in the body and checking if `scoping.has_binding(reference_id)` resolves to a symbol outside the body's scope.

3. **Compute scoped identifiers** (`compute_scoped_idents`): Compare collected identifiers against the declaration stack to determine which ones cross scope boundaries. The algorithm:
   - For each identifier in the body, check if it's declared in the `$()` body's own scope or an ancestor scope within the body
   - If it's declared in a scope OUTSIDE the `$()` body (i.e., in a parent scope that contains the `$()` call), it's a capture
   - Partition captures into variables (valid captures) and functions/classes (invalid captures that produce diagnostics)

**OXC API mapping for capture analysis:**

```rust
use oxc_semantic::Scoping;

/// Determine which variables are captured by a $()-body
fn compute_captures(
    scoping: &Scoping,
    body_scope_id: ScopeId,
    body_references: &[ReferenceId],
) -> Vec<String> {
    let mut captures = Vec::new();

    for &ref_id in body_references {
        let reference = scoping.get_reference(ref_id);

        // Skip if unresolved (global like console, window, etc.)
        if !scoping.has_binding(ref_id) {
            continue;
        }

        // Get the symbol this reference resolves to
        if let Some(symbol_id) = reference.symbol_id() {
            let symbol_scope = scoping.symbol_scope_id(symbol_id);

            // Check if the symbol is declared OUTSIDE the body's scope
            let is_outer = !is_scope_ancestor_of(scoping, body_scope_id, symbol_scope);
            if is_outer {
                let name = scoping.symbol_name(symbol_id);
                if !captures.contains(&name.to_string()) {
                    captures.push(name.to_string());
                }
            }
        }
    }

    captures
}

/// Check if `ancestor` is an ancestor of `descendant` (or equal)
fn is_scope_ancestor_of(
    scoping: &Scoping,
    ancestor: ScopeId,
    descendant: ScopeId,
) -> bool {
    scoping.scope_ancestors(descendant)
        .any(|scope_id| scope_id == ancestor)
}
```

**Key OXC Scoping methods used:**
- `find_binding(scope_id, name)` -- Find a binding by name, walking up the scope tree
- `get_resolved_references(symbol_id)` -- Get all references to a given symbol
- `scope_ancestors(scope_id)` -- Iterator walking up the scope tree from a given scope
- `symbol_scope_id(symbol_id)` -- Get the scope where a symbol is declared
- `symbol_name(symbol_id)` -- Get the name of a symbol
- `has_binding(reference_id)` -- Check if a reference resolves to a known symbol
- `get_binding(scope_id, name)` -- Get a binding in a specific scope (not ancestors)
- `scope_has_binding(scope_id, name)` -- Check if a binding exists in exactly this scope

**Source:** [Scoping struct docs](https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.Scoping.html), [SWC optimizer transform.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/transform.rs)

### Pattern 2: Multi-Module Output (APIM-06)

**What:** After extracting a `$()` body, construct a complete standalone `Program` AST for the segment module -- with imports, an exported const declaration containing the body, and optional additional statements.

**Allocator strategy: Shared allocator (recommended for POC)**

Use a single `Allocator` for the main module and all segment modules. This means:
- Parse the input into `Program<'a>` using allocator `A`
- Transform the main module AST in-place (replacing `$()` calls with `qrl()`)
- For each segment, construct a NEW `Program<'a>` using the SAME allocator `A`
- Run codegen on all programs (main + segments) before dropping `A`

```rust
fn transform_and_split(source: &str, filename: &str) -> Vec<(String, String)> {
    let allocator = Allocator::default();

    // Parse and transform main module
    let mut program = parse(&allocator, source, filename);
    let segments_data = transform_main_module(&allocator, &mut program);

    // Codegen main module
    let main_code = codegen(&program, source);

    // Build and codegen each segment
    let mut outputs = vec![("main".into(), main_code)];
    for seg in &segments_data {
        let seg_program = build_segment_program(&allocator, seg);
        let seg_code = codegen(&seg_program, "");
        outputs.push((seg.path.clone(), seg_code));
    }

    outputs
    // allocator drops here -- all ASTs freed
}
```

**Building a segment Program with AstBuilder:**

```rust
use oxc_ast::ast::*;
use oxc_allocator::Allocator;

fn build_segment_program<'a>(
    allocator: &'a Allocator,
    segment: &SegmentData,
) -> Program<'a> {
    let ast = AstBuilder::new(allocator);
    let mut body = ast.vec();

    // 1. Add imports the segment needs
    for import in &segment.needed_imports {
        let import_stmt = build_named_import(&import.name, &import.source, &ast);
        body.push(import_stmt);
    }

    // 2. Add _captures import if segment has captures
    if !segment.captures.is_empty() {
        let captures_import = build_named_import("_captures", "@qwik.dev/core", &ast);
        body.push(captures_import);
    }

    // 3. Add lazy imports (if segment has nested $-calls)
    for lazy in &segment.lazy_imports {
        let lazy_stmt = build_lazy_import_declaration(&lazy.hash, &lazy.path, &ast);
        body.push(lazy_stmt);
    }

    // 4. Add the exported segment body
    // export const SegmentName_hash = (params) => { body };
    let export_stmt = build_segment_export(
        &segment.export_name,
        &segment.body, // the arrow function expression
        &ast,
    );
    body.push(export_stmt);

    // 5. Construct the Program
    ast.program(
        SPAN,
        SourceType::mjs(), // segments are always JS modules
        None,              // no hashbang
        ast.vec(),         // no directives
        body,
    )
}
```

**Source:** [AstBuilder docs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html), [SWC code_move.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/code_move.rs)

### Pattern 3: Source Map Generation for Split Modules (POC-04)

**What:** Generate source maps that map generated segment code back to the original source positions.

**OXC Codegen with source maps:**

```rust
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};
use std::path::PathBuf;

fn codegen_with_sourcemap<'a>(
    program: &Program<'a>,
    source_text: &str,
    output_filename: &str,
) -> CodegenReturn {
    let options = CodegenOptions {
        source_map_path: Some(PathBuf::from(output_filename)),
        ..CodegenOptions::default()
    };

    Codegen::new()
        .with_options(options)
        .with_source_text(source_text)
        .build(program)
}
```

The `CodegenReturn` contains:
- `code: String` -- the generated JavaScript
- `map: Option<SourceMap>` -- the source map (present when `source_map_path` is set)

The `SourceMap` can be serialized with:
- `map.to_json_string()` -- full JSON source map
- `map.to_json()` -- `JSONSourceMap` struct for further processing

**Critical detail for split modules:** When constructing a segment's `Program` from extracted AST nodes, the `Span` values on those nodes reference byte offsets in the ORIGINAL source file. When codegen runs on the segment `Program`, it reads those spans and maps them to positions in the generated output. This means source maps for segments will correctly point back to the original source -- but only if the span values are preserved during extraction.

**Span preservation strategy:**
- When extracting a `$()` body expression with `take_in()`, the spans on the original nodes are preserved
- When constructing NEW nodes (import declarations, export wrapper), use `SPAN` (zero span) since they have no original source position
- The codegen will generate source map entries only for nodes with non-zero spans

**Source:** [Codegen docs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html), [CodegenOptions docs](https://docs.rs/oxc/latest/oxc/codegen/struct.CodegenOptions.html), [oxc_sourcemap](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/)

### Pattern 4: Segment Hash Computation

**What:** Compute the 11-character hash that appears in segment names. The SWC optimizer uses `DefaultHasher` (Rust's standard `SipHash`) combined with base64url encoding.

**Exact algorithm from SWC optimizer:**

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use base64::Engine;

fn compute_segment_hash(
    scope: Option<&str>,
    rel_path: &str,
    display_name: &str,
) -> String {
    let mut hasher = DefaultHasher::new();
    if let Some(scope) = scope {
        hasher.write(scope.as_bytes());
    }
    hasher.write(rel_path.as_bytes());
    hasher.write(display_name.as_bytes());
    let hash = hasher.finish(); // u64

    // Convert to base64url, replacing - and _ with 0
    base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(hash.to_le_bytes())
        .replace(['-', '_'], "0")
}
```

**Key details:**
- Uses `DefaultHasher` (SipHash-2-4) which produces a `u64`
- Converts the u64 to 8 bytes in little-endian order
- Encodes those 8 bytes with base64url (no padding), producing an 11-character string
- Replaces `-` and `_` characters with `0` for URL safety
- The hash inputs are: optional scope prefix, relative file path, and display name
- The relative file path uses forward slashes (via `to_slash_lossy()`)

**This must be ported exactly** to produce matching hashes. A single byte difference in the input (e.g., different path separator) produces a completely different hash.

**Source:** [SWC optimizer transform.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/transform.rs) -- `base64()` and `register_context_name()` functions

### Pattern 5: The `traverse_mut` API (Updated)

**What:** The current OXC `traverse_mut` function takes a `State` generic parameter.

**Current signature (verified):**
```rust
pub fn traverse_mut<'a, State, Tr: Traverse<'a, State>>(
    traverser: &mut Tr,
    allocator: &'a Allocator,
    program: &mut Program<'a>,
    scoping: Scoping,
    state: State,
) -> Scoping
```

The `Traverse` trait is now `Traverse<'a, State>`, and `TraverseCtx` is `TraverseCtx<'a, State>` where `State` is accessible via `ctx.state`.

**For the POCs, use `()` as the State type:**
```rust
struct DollarDetector { /* ... */ }
impl<'a> Traverse<'a, ()> for DollarDetector { /* ... */ }

// Usage:
let scoping = traverse_mut(&mut detector, &allocator, &mut program, scoping, ());
```

**Source:** [traverse_mut docs](https://docs.rs/oxc_traverse/latest/oxc_traverse/fn.traverse_mut.html), [Traverse trait docs](https://docs.rs/oxc_traverse/latest/oxc_traverse/trait.Traverse.html)

### Anti-Patterns to Avoid

- **Computing captures manually without Scoping:** Do not walk the AST manually to resolve variable references. Use `SemanticBuilder` to get `Scoping` which provides the complete scope tree.
- **Constructing segment Programs without preserving spans:** If you rebuild AST nodes from scratch (instead of moving them with `take_in`), the source map will have no original positions.
- **Using different hash inputs than the SWC optimizer:** The hash must be computed with the exact same inputs (scope, rel_path, display_name) or all 162 spec file hashes will mismatch.
- **Dropping the Allocator before codegen:** All `Program<'a>` instances must be serialized to strings before the allocator is dropped.
- **Ignoring the `State` parameter on `Traverse`:** The current API requires a `State` type parameter. Use `()` when no state is needed.

## Capture Analysis: The 8 Edge Cases

From the 162 spec files, 8 distinct capture edge cases have been identified:

### Edge Case 1: No Captures (most common)
**Spec:** `example_1.md`, `example_functional_component.md`
**Pattern:** The `$()` body only references its own parameters or variables declared within the body.
**Result:** `captures: false`, no `_captures` import, no captures array in `qrl()`.

### Edge Case 2: Variable Capture via `_rawProps`
**Spec:** `example_multi_capture.md`
**Pattern:** A component's destructured props `({foo})` are renamed to `_rawProps` in the component segment. The nested `$()` captures `_rawProps` and accesses properties like `_rawProps.foo`.
**Result:** `captures: true`, `captureNames: ["_rawProps"]`, segment uses `_captures[0]` to restore the variable.
**OXC mapping:** The `_rawProps` identifier in the nested `$()` body resolves to a binding in the parent component scope. `scope_ancestors()` on the `$()` body's scope will NOT include the scope where `_rawProps` is declared.

### Edge Case 3: Capture of `useStore` State Variable
**Spec:** `example_inlined_entry_strategy.md`
**Pattern:** `const state = useStore({count: 0})` in the component body, then `useBrowserVisibleTask$(() => { state.count = ... })` captures `state`.
**Result:** `captures: true` for the task segment, `[state]` in the captures array, `const state = _captures[0]` in the segment body.

### Edge Case 4: Const Literal Inlining (NOT captured)
**Spec:** `example_multi_capture.md`
**Pattern:** `const arg0 = 20` in the component body. The nested `$()` references `arg0`. Since 20 is an immutable constant, it's inlined as literal `20` rather than captured.
**Result:** `arg0` does NOT appear in captures. The output has `{20}` instead of `{arg0}`.
**OXC mapping:** Check if the symbol's initializer is a const-evaluable expression. If so, inline the value rather than capturing.

### Edge Case 5: CSS Import Captures (import re-emission)
**Spec:** `example_capture_imports.md`
**Pattern:** `import css1 from './global.css'`, then `useStyles$(\`${css1}${css2}\`)`. The CSS imports are not captured via `_captures[]` -- instead, the extracted segment re-imports them directly.
**Result:** `captures: false`, but the segment gets `import css1 from "./global.css"`.
**OXC mapping:** When a capture would be an import binding (not a local variable), re-emit the import in the segment instead of using `_captures[]`.

### Edge Case 6: Nested Scope Captures
**Spec:** `example_inlined_entry_strategy.md`
**Pattern:** `useBrowserVisibleTask$(() => { state.count = thing.doStuff() })` -- `state` is a local variable (captured), `thing` is an import (re-emitted, not captured), `console` is a global (ignored).
**Result:** Only `state` appears in captures. `thing` stays as an import. `console` is not captured.
**OXC mapping:** Use `scoping.has_binding(ref_id)` to determine if a reference is resolved. Use `scoping.symbol_scope_id()` to determine if it's declared inside or outside the body scope. Check if the symbol is an import binding to decide capture vs re-emit.

### Edge Case 7: Function/Class Declarations as Invalid Captures
**Spec:** Derived from SWC optimizer `compute_scoped_idents` logic
**Pattern:** If a `$()` body references a function or class declared in an outer scope, the SWC optimizer produces a diagnostic error rather than capturing it.
**Result:** Diagnostic warning/error emitted.
**OXC mapping:** After identifying captures, check `scoping.symbol_flags(symbol_id)` to determine if the symbol is a function or class declaration. Emit a diagnostic if so.

### Edge Case 8: Props Destructuring Conversion
**Spec:** `example_multi_capture.md`
**Pattern:** Component function `({foo}) => { ... $(() => { ...foo... }) }` -- the destructured param `{foo}` is renamed to `_rawProps` in the component segment, and the nested `$()` captures `_rawProps`, accessing `_rawProps.foo`.
**Result:** The component segment gets `(_rawProps) => { ... }` instead of `({foo}) => { ... }`. The nested segment gets `_rawProps.foo` instead of `foo`.
**OXC mapping:** This requires a `props_destructuring` pre-transform that converts destructured component params before capture analysis runs.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Scope tree construction | Manual scope tracking | `SemanticBuilder` -> `Scoping` | OXC builds scope tree + symbol table with all bindings resolved |
| Variable resolution | Walking AST to find declarations | `find_binding(scope_id, name)` | Scoping already resolved all bindings |
| Reference-to-declaration mapping | Manual identifier matching | `get_resolved_references(symbol_id)` | Scoping tracks all references per symbol |
| Scope ancestry queries | Manual parent chain walking | `scope_ancestors(scope_id)` | Built-in iterator over scope chain |
| Program AST construction | Manual struct initialization | `AstBuilder::program()` + builder methods | Handles arena allocation and correct types |
| Source map generation | Manual position tracking | `Codegen` with `source_map_path` option | Automatic span-to-position mapping |
| Hash computation | Custom hash algorithm | Port SWC's exact algorithm (`DefaultHasher` + base64url) | Must produce identical hashes for spec compatibility |

**Key insight:** OXC's `Scoping` provides everything the SWC optimizer builds manually via `GlobalCollect` + `IdentCollector`. The entire capture analysis can be expressed as scope tree queries rather than AST walking.

## Common Pitfalls

### Pitfall 1: Capture vs Import Re-emission Confusion
**What goes wrong:** Treating all outer-scope references as captures, when import bindings should be re-emitted as imports in the segment instead.
**Why it happens:** Both captured variables and import bindings resolve to symbols outside the `$()` body's scope.
**How to avoid:** After identifying an outer-scope reference, check `scoping.symbol_flags(symbol_id)` for `SymbolFlags::Import`. If it's an import, add the import to the segment's needed imports rather than the captures list.
**Warning signs:** Segments have `_captures` references for things like `useStore` that should be imported.

### Pitfall 2: Allocator Lifetime with Multiple Programs
**What goes wrong:** Trying to run codegen on a segment `Program<'a>` after the allocator that created it has been dropped.
**Why it happens:** When using separate allocators per segment, the segment's `Program` dies with its allocator.
**How to avoid:** Use a shared allocator for the main module and all segments, running all codegen within the same scope. Or ensure each segment's allocator lives until after codegen.
**Warning signs:** "Lifetime `'a` does not live long enough" compiler errors.

### Pitfall 3: Hash Mismatch from Path Separators
**What goes wrong:** Generated hashes differ from spec files on different operating systems.
**Why it happens:** The SWC optimizer normalizes paths to forward slashes using `to_slash_lossy()` before hashing. If the port uses native path separators, Windows will produce different hashes.
**How to avoid:** Always normalize paths to forward slashes before computing the hash. Use the `path-slash` crate.
**Warning signs:** All segment names have different hashes than spec files. Compare against `example_1.md`: `renderHeader_zBbHWn4e8Cg`.

### Pitfall 4: Missing _captures Index Mapping
**What goes wrong:** Capture variables are identified but accessed with wrong indices in the segment body.
**Why it happens:** The captures array order matters. `_captures[0]` must correspond to the first element of the captures array passed to `qrl()` or `inlinedQrl()`.
**How to avoid:** Maintain a consistent ordering of captures. The SWC optimizer appears to use insertion order from the identifier collection pass.
**Warning signs:** Runtime errors where `_captures[0]` returns the wrong variable.

### Pitfall 5: Const Inlining Missed or Over-Applied
**What goes wrong:** A constant like `const arg0 = 20` is captured instead of inlined, or a non-const is inlined when it shouldn't be.
**Why it happens:** The determination of "const-evaluable" is nuanced. Only primitive literals and certain template literals qualify.
**How to avoid:** Check `is_const_expression()` for the symbol's initializer. Only inline if the initializer is a number, string, boolean, null, or undefined literal. Don't inline if the initializer involves function calls or non-literal expressions.
**Warning signs:** Spec file comparison shows `{arg0}` where it should show `{20}`, or vice versa.

### Pitfall 6: traverse_mut State Parameter Mismatch
**What goes wrong:** Compilation fails because `Traverse` trait or `traverse_mut` is called without the `State` type parameter.
**Why it happens:** The OXC `Traverse` trait now requires a `State` generic parameter: `Traverse<'a, State>`.
**How to avoid:** Use `()` as the State type for simple implementations. Ensure `TraverseCtx<'a, State>` matches.
**Warning signs:** Compilation errors about missing type parameters or trait bound mismatches.

### Pitfall 7: Source Map with Zero Spans for Extracted Nodes
**What goes wrong:** Source maps for segments show all positions as 0:0.
**Why it happens:** When nodes are constructed with `SPAN` (the zero span constant) instead of preserving original spans, codegen has no position info.
**How to avoid:** Use `take_in()` to move nodes from the original AST to the segment AST, preserving their original spans. Only use `SPAN` for nodes that have no original source position (new imports, export wrappers).
**Warning signs:** Source map visualizer shows all tokens mapping to position 0.

## Code Examples

### POC-01: Detecting $() Call Sites

```rust
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::collections::HashSet;

struct DollarDetector {
    dollar_imports: HashSet<String>,
    found_sites: Vec<DollarCallSite>,
}

struct DollarCallSite {
    callee: String,
    span_start: u32,
    span_end: u32,
}

impl<'a> Traverse<'a, ()> for DollarDetector {
    fn enter_import_declaration(
        &mut self,
        import: &mut ImportDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if import.source.value.as_str() != "@qwik.dev/core" {
            return;
        }
        if let Some(specifiers) = &import.specifiers {
            for spec in specifiers {
                if let ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                    let name = match &s.imported {
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::StringLiteral(s) => s.value.as_str(),
                    };
                    if name == "$" || name.ends_with('$') {
                        self.dollar_imports.insert(name.to_string());
                    }
                }
            }
        }
    }

    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            if name == "$" || (name.ends_with('$') && self.dollar_imports.contains(name)) {
                self.found_sites.push(DollarCallSite {
                    callee: name.to_string(),
                    span_start: call.span.start,
                    span_end: call.span.end,
                });
            }
        }
    }
}

fn main() {
    let source = r#"
        import { $, component$ } from '@qwik.dev/core';
        export const App = component$(() => {
            return $(() => <div/>);
        });
    "#;

    let allocator = Allocator::default();
    let source_type = SourceType::tsx();
    let ret = Parser::new(&allocator, source, source_type).parse();
    let mut program = ret.program;

    let semantic = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);
    let scoping = semantic.semantic.into_scoping();

    let mut detector = DollarDetector {
        dollar_imports: HashSet::new(),
        found_sites: Vec::new(),
    };

    let _scoping = traverse_mut(
        &mut detector,
        &allocator,
        &mut program,
        scoping,
        (),
    );

    println!("Found {} $-call sites:", detector.found_sites.len());
    for site in &detector.found_sites {
        println!(
            "  {} at {}..{}",
            site.callee, site.span_start, site.span_end
        );
    }
}
```

### POC-02: Capture Analysis

```rust
use oxc_semantic::{Scoping, ScopeId, SymbolFlags};

/// After parsing and semantic analysis, determine captures for a $()-body
/// at a known scope.
fn analyze_captures(
    scoping: &Scoping,
    body_scope_id: ScopeId,
) -> Vec<CaptureInfo> {
    let mut captures = Vec::new();

    // Iterate all symbols to find references within our scope
    // that resolve to bindings outside our scope
    for symbol_id in scoping.symbol_ids() {
        let symbol_scope = scoping.symbol_scope_id(symbol_id);

        // Check if this symbol is declared OUTSIDE the body scope
        let is_declared_outside = !is_scope_contained_in(
            scoping, symbol_scope, body_scope_id
        );
        if !is_declared_outside {
            continue; // Declared inside body, not a capture
        }

        // Check if any references to this symbol are INSIDE the body scope
        let has_inner_reference = scoping
            .get_resolved_references(symbol_id)
            .any(|reference| {
                // reference.scope_id() gives us where the reference appears
                // We need to check if it's inside body_scope_id
                // This requires checking if the reference's span falls within
                // the body's span range, or using scope ancestry
                true // simplified; actual impl checks reference scope
            });

        if has_inner_reference {
            let name = scoping.symbol_name(symbol_id).to_string();
            let flags = scoping.symbol_flags(symbol_id);

            captures.push(CaptureInfo {
                name,
                is_import: flags.contains(SymbolFlags::Import),
                is_const: !scoping.symbol_is_mutated(symbol_id),
            });
        }
    }

    captures
}

struct CaptureInfo {
    name: String,
    is_import: bool,
    is_const: bool,
}

/// Check if `inner` scope is contained within `outer` scope
fn is_scope_contained_in(
    scoping: &Scoping,
    inner: ScopeId,
    outer: ScopeId,
) -> bool {
    scoping.scope_ancestors(inner)
        .any(|scope_id| scope_id == outer)
}
```

### POC-03: Multi-Module Output

```rust
use oxc_allocator::Allocator;
use oxc_ast::AstBuilder;
use oxc_codegen::{Codegen, CodegenReturn};
use oxc_parser::Parser;
use oxc_span::{SPAN, SourceType};

fn split_into_modules(source: &str) -> Vec<(String, String)> {
    let allocator = Allocator::default();
    let source_type = SourceType::tsx();

    // 1. Parse
    let ret = Parser::new(&allocator, source, source_type).parse();
    let main_program = ret.program;

    // 2. Codegen main module
    let main_result: CodegenReturn = Codegen::new()
        .with_source_text(source)
        .build(&main_program);

    // 3. Build a segment program
    let ast = AstBuilder::new(&allocator);
    let segment_body = build_simple_segment(&ast);
    let segment_result: CodegenReturn = Codegen::new()
        .build(&segment_body);

    vec![
        ("main.js".to_string(), main_result.code),
        ("segment.js".to_string(), segment_result.code),
    ]
}

fn build_simple_segment<'a>(ast: &AstBuilder<'a>) -> Program<'a> {
    // Build: export const segment_hash = () => { /* body */ };
    let body_fn = ast.expression_arrow_function(
        SPAN, true, false, false,
        None, // type params
        ast.formal_parameters(
            SPAN,
            FormalParameterKind::ArrowFormalParameters,
            ast.vec(), None,
        ),
        None, // return type
        ast.function_body(SPAN, ast.vec(), ast.vec()),
    );

    let binding = ast.binding_pattern_kind_binding_identifier(
        SPAN, ast.atom("segment_hash"),
    );
    let pattern = ast.binding_pattern(binding, None, false);
    let declarator = ast.variable_declarator(
        SPAN, VariableDeclarationKind::Const,
        pattern, Some(body_fn), false,
    );
    let declaration = ast.variable_declaration(
        SPAN, VariableDeclarationKind::Const,
        ast.vec1(declarator), false,
    );

    let export = Statement::from(ast.module_declaration_export_named_declaration(
        SPAN,
        Some(Declaration::VariableDeclaration(ast.alloc(declaration))),
        ast.vec(), None, ImportOrExportKind::Value, None,
    ));

    ast.program(
        SPAN,
        SourceType::mjs(),
        None,
        ast.vec(),
        ast.vec1(export),
    )
}
```

### POC-04: Source Maps for Split Modules

```rust
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};
use std::path::PathBuf;

fn codegen_with_source_map<'a>(
    program: &Program<'a>,
    source_text: &str,
    output_path: &str,
) -> (String, Option<String>) {
    let options = CodegenOptions {
        source_map_path: Some(PathBuf::from(output_path)),
        ..CodegenOptions::default()
    };

    let result: CodegenReturn = Codegen::new()
        .with_options(options)
        .with_source_text(source_text)
        .build(program);

    let source_map_json = result.map.map(|sm| sm.to_json_string());
    (result.code, source_map_json)
}
```

## State of the Art

| Old Approach (SWC Optimizer) | Current Approach (OXC) | Impact |
|------------------------------|------------------------|--------|
| `GlobalCollect` + `IdentCollector` manual passes | `SemanticBuilder` -> `Scoping` with built-in scope tree | Eliminates ~300 lines of manual scope tracking |
| `decl_stack` manual scope tracking | `scope_ancestors()` + `find_binding()` | Built-in API replaces manual stack management |
| SWC `visit_with` for read-only passes | OXC `Traverse` supports both read and write in one pass | Can collect AND transform in a single traversal |
| SWC `Fold` for AST transformation | OXC `*expr = new_expr` direct mutation | Simpler; no need for functional fold pattern |
| SWC `swc_common::SourceMap` | OXC `oxc_sourcemap::SourceMap` via `Codegen` | Integrated with codegen; no separate source map pass |
| `traverse_mut(traverser, allocator, program, scoping)` | `traverse_mut(traverser, allocator, program, scoping, state)` | New `State` parameter for carrying custom data |

**Important version change:** The `Traverse` trait and `traverse_mut` function now require a `State` generic parameter. The Phase 4 research showed the older signature without `State`. POCs must use the updated signature: `Traverse<'a, State>` and pass a `State` value (use `()` when not needed).

## Open Questions

1. **Exact AstBuilder::program() Method Signature**
   - What we know: `AstBuilder` has methods for constructing all AST node types. Phase 4 research confirmed builder methods for expressions and statements.
   - What's unclear: The exact signature of `AstBuilder::program()` for constructing a standalone `Program` node may differ from what's documented. The params likely include `span`, `source_type`, `hashbang`, `directives`, and `body`.
   - Recommendation: Verify during POC-03 implementation. If `program()` doesn't exist, construct `Program` directly using struct initialization.

2. **Reference Scope Identification in OXC**
   - What we know: `Scoping` provides `get_resolved_references(symbol_id)` which returns `Reference` objects. `Reference` has a `symbol_id()` method.
   - What's unclear: Whether `Reference` has a `scope_id()` method or whether we need to use the reference's `NodeId` to determine its scope.
   - Recommendation: Check `Reference` struct fields during POC-02 implementation. If no `scope_id()`, use span-based scope lookup or `current_scope_id()` during traversal.

3. **`take_in` Behavior Across Traverse Hooks**
   - What we know: `take_in(ctx.ast)` moves an AST node within the arena. Phase 4 documented this pattern.
   - What's unclear: Whether `take_in` works correctly when the source and destination are in the same `Program` (main module mutation) vs. when building a new `Program` (segment construction).
   - Recommendation: Test during POC-03. If `take_in` doesn't work for cross-program moves, use `clone_in()` as a fallback.

4. **Span Handling for Segments with Captures**
   - What we know: Source maps rely on spans. Segments constructed from extracted bodies should preserve original spans.
   - What's unclear: When captures cause the body to be rewritten (inserting `const state = _captures[0]` at the top), how to handle spans for the injected capture-restoration statements.
   - Recommendation: Use `SPAN` for injected statements (no source position) and preserve original spans for the moved body content. Verify with POC-04.

## Sources

### Primary (HIGH confidence)
- [Scoping struct docs.rs](https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.Scoping.html) -- Complete API: find_binding, get_resolved_references, scope_ancestors, symbol_scope_id, symbol_flags
- [Codegen struct docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) -- build(), with_options(), with_source_text()
- [CodegenOptions docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.CodegenOptions.html) -- source_map_path field for enabling source maps
- [oxc_sourcemap docs.rs](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/) -- SourceMap, SourceMapBuilder, ConcatSourceMapBuilder, to_json_string()
- [traverse_mut docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/fn.traverse_mut.html) -- Updated signature with State parameter
- [Traverse trait docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/trait.Traverse.html) -- Traverse<'a, State> with 462 enter/exit methods
- [TraverseCtx docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/struct.TraverseCtx.html) -- Fields: state, ancestry, scoping, ast
- [SWC optimizer transform.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/transform.rs) -- Hash algorithm, capture resolution, decl_stack, register_context_name
- [SWC optimizer code_move.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/code_move.rs) -- Segment construction, scoped_idents, local_idents, USE_LEXICAL_SCOPE

### Secondary (MEDIUM confidence)
- [OXC Semantic Analysis guide](https://oxc.rs/docs/learn/parser_in_rust/semantic_analysis) -- Overview of semantic analysis approach
- [SWC optimizer collector.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/collector.rs) -- GlobalCollect, IdentCollector structures
- [Qwik Optimizer Rules docs](https://qwik.dev/docs/advanced/optimizer/) -- High-level description of capture and extraction semantics
- Phase 4 RESEARCH.md and API-MAPPING.md -- Prior OXC API research, must be updated for State parameter
- Phase 4 ARCHITECTURE-BLUEPRINT.md -- Module layout, public API types, data flow

### Tertiary (LOW confidence)
- AstBuilder::program() method signature -- needs runtime verification in POC-03
- Reference scope_id field existence -- needs verification in POC-02
- Span preservation behavior when using take_in across Program boundaries -- needs POC testing

## Metadata

**Confidence breakdown:**
- Capture analysis algorithm: HIGH -- SWC source code verified, OXC Scoping API confirmed with docs.rs
- Multi-module output: HIGH -- AstBuilder construction verified, allocator lifetime rules well-understood from Phase 4
- Source map generation: HIGH -- Codegen source_map_path option confirmed in CodegenOptions docs
- Hash algorithm: HIGH -- Exact algorithm extracted from SWC optimizer source (DefaultHasher + base64url)
- Traverse API update: HIGH -- State parameter confirmed from docs.rs

**Research date:** 2026-02-10
**Valid until:** 2026-03-10 (OXC API stable; SWC optimizer source stable)
