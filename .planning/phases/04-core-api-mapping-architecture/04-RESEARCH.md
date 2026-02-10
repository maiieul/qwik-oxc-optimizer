# Phase 4: Core API Mapping & Architecture - Research

**Researched:** 2026-02-10
**Domain:** OXC Traverse + AstBuilder APIs for Qwik optimizer transformation patterns; Rust crate architecture
**Confidence:** HIGH

## Summary

Phase 4 requires mapping the three foundational Qwik optimizer transformation patterns ($-extraction, qrl wrapping, import rewriting) to concrete OXC Rust APIs, and designing the complete architecture for the new optimizer crate. The research has identified all necessary OXC APIs and confirmed they are sufficient for the required transformations.

The OXC ecosystem at version 0.113.0 provides a complete transformation pipeline: `oxc_parser` for parsing, `oxc_traverse` with its `Traverse` trait for AST visitation and mutation, `AstBuilder` (accessible via `ctx.ast`) for constructing new AST nodes, and `oxc_codegen` for generating JavaScript from the modified AST. The existing SWC-based Qwik optimizer uses SWC's `fold_with` visitor pattern; the OXC equivalent is `Traverse` with `enter_*`/`exit_*` methods and direct node mutation via `*expr = new_expr`. Statement injection (for new import declarations) uses the `StatementInjectorStore` pattern, and module-level import management uses `ModuleImportsStore`.

The current SWC optimizer's public API (`transform_modules()` -> `TransformOutput` with `TransformModule[]` and `SegmentAnalysis[]`) must be preserved. The OXC crate replaces only the internal transformation engine, not the external interface. The 162 spec files from v1.0 serve as the behavioral ground truth -- every input/output pair documents exactly what the OXC optimizer must produce.

**Primary recommendation:** Create document-level research artifacts mapping each transformation pattern to specific OXC `AstBuilder` method calls, with Rust code examples referencing real spec file inputs/outputs, plus a complete crate module layout with Cargo.toml specification.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `oxc` (umbrella) | `0.113` | Parser, AST, Traverse, Codegen, Transformer infra | Single dependency with feature flags; all sub-crates versioned in lockstep |
| `serde` | `1.x` | JSON serialization for TransformOutput, SegmentAnalysis | Required for FFI serialization to TypeScript layer |
| `serde_json` | `1.x` | JSON encoding/decoding of transform options and output | Industry standard; matches existing SWC optimizer pattern |
| Rust | edition 2024 | Language | Current stable edition; matches existing oxc-ast-util crate |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `anyhow` | `1.x` | Error handling with context | All fallible operations; matches SWC optimizer's error pattern |
| `base64` | `0.22` | Source map encoding | When source maps are enabled |
| `rayon` | `1.x` | Parallel module transformation | Multi-file batch transforms (transform_modules processes N files) |
| `insta` | `1.x` | Snapshot testing | Test harness for comparing output against 162 spec files |
| `pathdiff` | `0.2` | Relative path computation | Computing relative import paths between extracted segments |
| `path-slash` | `0.2` | Cross-platform path normalization | Consistent `/` separators on Windows |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `oxc` umbrella | Individual sub-crates (`oxc_parser`, `oxc_ast`, etc.) | Umbrella is simpler; individual crates give finer control but require manual version sync |
| `rayon` for parallelism | Sequential processing | Rayon matches existing SWC optimizer; 162+ spec files benefit from parallel transform |
| `insta` for snapshots | Custom diff comparison | insta is the standard Rust snapshot testing crate; provides review workflow via `cargo insta review` |

### Required OXC Feature Flags
```toml
[dependencies]
oxc = { version = "0.113", features = [
    "parser",       # oxc_parser for JS/TS/JSX/TSX parsing
    "traverse",     # oxc_traverse for Traverse trait + TraverseCtx
    "transformer",  # oxc_transformer for TransformState, ModuleImportsStore patterns
    "codegen",      # oxc_codegen for AST -> JavaScript output
    "semantic",     # oxc_semantic for SemanticBuilder (scope/symbol analysis)
    "serialize",    # ESTree JSON serialization (for debugging/testing)
] }
```

**Cargo.toml specification:**
```toml
[package]
name = "qwik-optimizer-oxc"
version = "0.1.0"
edition = "2024"

[dependencies]
oxc = { version = "0.113", features = ["parser", "traverse", "transformer", "codegen", "semantic", "serialize"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
base64 = "0.22"
rayon = "1"
pathdiff = "0.2"
path-slash = "0.2"

[dev-dependencies]
insta = { version = "1", features = ["json"] }
```

## Architecture Patterns

### Recommended Crate Module Layout

```
oxc-optimizer/
  Cargo.toml
  src/
    lib.rs               # Public API: transform_modules(), re-exports
    types.rs             # TransformModulesOptions, TransformOutput, SegmentAnalysis, etc.
    parse.rs             # Parse single module: source -> OXC Program AST
    transform.rs         # QwikTransform: main Traverse implementation
    collector.rs         # Global symbol collection: identify $() call sites, imports, exports
    code_move.rs         # Segment extraction: extract $() bodies into separate modules
    entry_strategy.rs    # EntryStrategy application (segment vs inline vs single)
    emit.rs              # Code generation: OXC Codegen -> JavaScript string + source map
    import_rewrite.rs    # Import mutation: remove component$, add componentQrl, add qrl
    hash.rs              # Segment hash computation (display name -> hash)
    words.rs             # String constants: BUILDER_IO_QWIK, known $-suffixed APIs
    filter_exports.rs    # Strip exports for server/client mode
    props_destructuring.rs  # Component props transformation
    is_const.rs          # Const evaluation utilities
    const_replace.rs     # Constant inlining
    errors.rs            # Diagnostic types and error creation
    test.rs              # Test harness (cfg(test))
    snapshots/           # insta snapshot files
```

### Module Dependency Ordering

```
lib.rs
  -> types.rs (data structures, no logic dependencies)
  -> errors.rs (diagnostic types)
  -> words.rs (constants)
  -> hash.rs (pure computation)
  -> parse.rs (depends on: types, errors)
  -> collector.rs (depends on: types, words)
  -> transform.rs (depends on: types, collector, words, hash, import_rewrite)
  -> import_rewrite.rs (depends on: types, words)
  -> code_move.rs (depends on: types, collector, hash, emit)
  -> entry_strategy.rs (depends on: types)
  -> emit.rs (depends on: types)
  -> filter_exports.rs (depends on: types, words)
  -> props_destructuring.rs (depends on: types)
  -> is_const.rs (pure computation)
  -> const_replace.rs (depends on: types, is_const)
```

### Pattern 1: OXC Traverse for $() Detection

**What:** Use the `Traverse` trait to walk the AST and detect `$()` call sites. The Qwik optimizer must find all calls where the callee is `$` or a `$`-suffixed function imported from `@qwik.dev/core`.

**When to use:** During the main transformation pass.

**How it works in OXC:**
```rust
use oxc_traverse::{Traverse, TraverseCtx};
use oxc_ast::ast::*;

struct QwikTransform {
    // Collected $-call metadata
    segments: Vec<SegmentData>,
    // Set of known $-suffixed imports from @qwik.dev/core
    dollar_imports: HashSet<String>,
}

impl<'a> Traverse<'a> for QwikTransform {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        // Detect: $(() => { ... })
        // Detect: component$(() => { ... })
        // Detect: useStyles$('...')
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            if name == "$" || self.dollar_imports.contains(name) {
                // This is a $-call site. Record it for extraction.
                self.record_segment(call, ctx);
            }
        }
    }
}
```

**Source:** OXC `Traverse` trait pattern from [oxc_traverse docs](https://docs.rs/oxc_traverse/latest/oxc_traverse/), confirmed by [oxc_transformer implementation](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/lib.rs).

### Pattern 2: AstBuilder for QRL Wrapping

**What:** Replace `$(() => { ... })` with `qrl(import_fn, "segment_name_hash")`. Use `ctx.ast` (the `AstBuilder`) to construct the replacement `CallExpression`.

**When to use:** After detecting a $-call site, replace it in `exit_call_expression` or `exit_expression`.

**How it works in OXC:**
```rust
fn exit_expression(
    &mut self,
    expr: &mut Expression<'a>,
    ctx: &mut TraverseCtx<'a>,
) {
    if let Expression::CallExpression(call) = expr {
        if self.is_dollar_call(call) {
            // Build: qrl(i_hashValue, "Name_hash")
            let import_ident = ctx.ast.expression_identifier_reference(
                SPAN,
                ctx.ast.atom(&format!("i_{}", segment_hash)),
            );
            let symbol_name = ctx.ast.expression_string_literal(
                SPAN,
                ctx.ast.atom(&segment_name),
                None,
            );
            let mut arguments = ctx.ast.vec_with_capacity(2);
            arguments.push(Argument::from(import_ident));
            arguments.push(Argument::from(symbol_name));

            let qrl_ident = ctx.ast.expression_identifier_reference(
                SPAN,
                ctx.ast.atom("qrl"),
            );

            // Replace the expression with: /*#__PURE__*/ qrl(i_hash, "name")
            *expr = ctx.ast.expression_call(
                SPAN,
                qrl_ident,
                NONE, // no type arguments
                arguments,
                false, // not optional
            );
        }
    }
}
```

**Source:** AstBuilder methods from [oxc_ast AstBuilder docs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html); expression replacement pattern from [JSX transformer](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/jsx/jsx_impl.rs).

### Pattern 3: Import Statement Mutation

**What:** Transform imports from `$`-suffixed to `Qrl`-suffixed forms and add new imports.

The transformation rules observed in spec files:
- `import { component$ } from '@qwik.dev/core'` -> remove `component$`, add `import { componentQrl } from "@qwik.dev/core"`
- Add `import { qrl } from "@qwik.dev/core"` (segment strategy) or `import { inlinedQrl } from "@qwik.dev/core"` (inline strategy)
- For captures: add `import { _captures } from "@qwik.dev/core"`
- Keep non-dollar imports unchanged: `import { useStore } from '@qwik.dev/core'` stays

**When to use:** During program-level enter/exit, after collecting all needed imports.

**How it works in OXC:**
```rust
fn exit_program(
    &mut self,
    program: &mut Program<'a>,
    ctx: &mut TraverseCtx<'a>,
) {
    // Strategy 1: Prepend new import declarations to program.body
    let mut new_stmts: Vec<Statement<'a>> = Vec::new();

    // Add: import { qrl } from "@qwik.dev/core"
    if self.needs_qrl_import {
        let qrl_import = self.build_named_import("qrl", "@qwik.dev/core", ctx);
        new_stmts.push(qrl_import);
    }

    // Add: import { componentQrl } from "@qwik.dev/core"
    for qrl_name in &self.needed_qrl_imports {
        let import_stmt = self.build_named_import(qrl_name, "@qwik.dev/core", ctx);
        new_stmts.push(import_stmt);
    }

    // Prepend new imports, then append existing statements
    let existing = std::mem::take(&mut program.body);
    program.body = ctx.ast.vec_from_iter(
        new_stmts.into_iter().chain(existing.into_iter())
    );
}

fn build_named_import(
    &self,
    name: &str,
    source: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let local = ctx.ast.binding_identifier(SPAN, ctx.ast.atom(name));
    let imported = ctx.ast.module_export_name_identifier_name(SPAN, ctx.ast.atom(name));
    let specifier = ctx.ast.import_specifier(SPAN, imported, local, ImportOrExportKind::Value);
    let specifiers = ctx.ast.vec1(ImportDeclarationSpecifier::ImportSpecifier(specifier));
    let source_lit = ctx.ast.string_literal(SPAN, ctx.ast.atom(source), None);

    let import_decl = ctx.ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        NONE, // no import attributes
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}
```

**Source:** AstBuilder `import_declaration`, `import_specifier` methods from [AstBuilder docs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html); import management from [module_imports.rs](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/common/module_imports.rs).

### Pattern 4: OXC Transformation Pipeline

**What:** The complete pipeline from source code to transformed output.

**Pipeline:**
```
Source Code (string)
  -> Parser::new(&allocator, &source, source_type).parse()
  -> SemanticBuilder::new().build(&program)  // scope/symbol analysis
  -> traverse_mut(&mut QwikTransform::new(...), &mut program, ctx)
  -> Codegen::new().build(&program)  // JS output
  -> (code: String, map: Option<String>)
```

**Source:** [OXC transformer example](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/examples/transformer.rs).

### Anti-Patterns to Avoid

- **Cloning AST nodes instead of using `take_in(ctx.ast)`:** OXC's arena allocator means cloning wastes memory. Use `take_in` to move ownership, then construct the replacement node.
- **Trying to return `Program<'a>` from a function where `Allocator` is local:** The program lifetime is tied to the allocator. Always serialize (codegen) in the same scope as the allocator.
- **Using SWC `fold_with` patterns in OXC:** OXC uses `Traverse` with `enter_*`/`exit_*` and direct mutation (`*expr = new_expr`), not SWC's functional fold pattern.
- **Modifying program.body during traversal of program.body:** Use the exit_program hook or the StatementInjectorStore pattern to defer statement insertions until after traversal.
- **Hand-rolling semantic analysis:** Use `SemanticBuilder` to get scopes and symbols. The `TraverseCtx` provides scope information needed for capture analysis.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| JS/TS/JSX parsing | Custom parser | `oxc_parser::Parser` | OXC passes 100% of Test262; handles all JS/TS/JSX edge cases |
| AST node construction | Manual struct initialization | `AstBuilder` methods via `ctx.ast` | Builder handles arena allocation, span management, and type correctness |
| Scope/symbol analysis | Manual scope tracking | `SemanticBuilder` -> `Scoping` | Required for correct capture analysis; OXC's semantic analysis is battle-tested |
| Code generation | String concatenation | `oxc_codegen::Codegen` | Handles all JS syntax edge cases, source maps, minification |
| AST traversal | Manual recursive descent | `oxc_traverse::Traverse` trait | Provides parent context, scope info, and safe mutable access |
| Statement injection | Manual Vec manipulation during traversal | `StatementInjectorStore` pattern | Defers mutations to avoid iterator invalidation |
| Import management | Manual import deduplication | Collect needed imports, build in exit_program | Avoids duplicate imports; matches spec output patterns |
| Hash computation | Custom hash function | Port from SWC optimizer's hash.rs | Must produce identical hashes for spec compatibility |

**Key insight:** OXC provides the complete toolchain for AST transformation. The Qwik optimizer's unique value is the *transformation logic* (what to detect, what to generate), not the infrastructure (parsing, traversal, codegen). Use OXC for all infrastructure.

## Common Pitfalls

### Pitfall 1: Arena Allocator Lifetime Escape
**What goes wrong:** Trying to store `Expression<'a>` or `Statement<'a>` references outside the allocator scope causes lifetime errors.
**Why it happens:** OXC uses arena allocation. All AST nodes live in the `Allocator`. When the allocator drops, all nodes are freed.
**How to avoid:** Complete all AST manipulation and code generation within the same scope as the `Allocator`. For the Qwik optimizer, this means: parse, transform, and codegen within a single function that owns the allocator.
**Warning signs:** Compiler errors about lifetimes like `'a` not living long enough.

### Pitfall 2: Segment Hash Incompatibility
**What goes wrong:** Generated segment names don't match the 162 spec files because the hash function differs.
**Why it happens:** The SWC optimizer uses a specific hash algorithm to generate the 11-character hashes in segment names (e.g., `zBbHWn4e8Cg`, `J4uyIhaBNR4`).
**How to avoid:** The hash computation must be ported exactly from the SWC optimizer. This is a pure function (display name -> hash) with no SWC dependency.
**Warning signs:** All segment names have different hashes than spec files. Check with spec file `example_1.md`: `renderHeader_zBbHWn4e8Cg`.

### Pitfall 3: Import Statement Ordering
**What goes wrong:** Generated imports appear in different order than the SWC optimizer output.
**Why it happens:** The SWC optimizer produces a specific import ordering: new Qrl imports first, then lazy import declarations, then original imports.
**How to avoid:** Study spec file outputs carefully. The ordering pattern from `example_functional_component` is: `import { componentQrl }` -> `import { qrl }` -> `const i_hash = ()=>import(...)` -> original imports (kept) -> transformed declarations.
**Warning signs:** Spec file comparison shows correct code but different line ordering.

### Pitfall 4: PURE Annotation Placement
**What goes wrong:** `/*#__PURE__*/` comments are missing or in wrong positions, breaking tree-shaking.
**Why it happens:** OXC's `AstBuilder` has `call_expression_with_pure` that handles this, but you need to remember to use it.
**How to avoid:** Use `ctx.ast.expression_call_with_pure(span, callee, NONE, args, false, true)` for all `qrl()`, `componentQrl()`, `inlinedQrl()` calls. The `pure: true` parameter adds the annotation.
**Warning signs:** Tree-shaking eliminates code that should be kept, or bundlers include dead code.

### Pitfall 5: Segment Strategy Differences
**What goes wrong:** Using `qrl()` when the entry strategy is `Inline`, or `inlinedQrl()` when the strategy is `Segment`.
**Why it happens:** Two different wrapping patterns exist:
- **Segment strategy:** `$(() => {...})` -> separate file + `qrl(import_fn, "name")`
- **Inline strategy:** `$(() => {...})` -> same file + `inlinedQrl(() => {...}, "name", [captures])`
**How to avoid:** Branch on `EntryStrategy` early in the transformation. The spec files `example_functional_component` (segment) and `example_inlined_entry_strategy` (inline) demonstrate both patterns.
**Warning signs:** Tests passing for segment strategy but failing for inline, or vice versa.

### Pitfall 6: Dollar-to-Qrl Suffix Naming Convention
**What goes wrong:** `component$` is replaced with `component` instead of `componentQrl`.
**Why it happens:** The naming convention is: strip the `$` suffix and append `Qrl`. This applies to all `$`-suffixed functions: `component$` -> `componentQrl`, `useStyles$` -> `useStylesQrl`, `useBrowserVisibleTask$` -> `useBrowserVisibleTaskQrl`.
**How to avoid:** Implement a `dollar_to_qrl_name(name: &str) -> String` utility that strips `$` and appends `Qrl`. Unit test it against all known Qwik APIs.
**Warning signs:** Runtime errors about missing exports from `@qwik.dev/core`.

## Code Examples

### Complete Single-Module Transformation Pipeline

```rust
// Source: Synthesized from OXC transformer example + Qwik optimizer patterns
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenReturn};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

/// Transform a single Qwik module using OXC
fn transform_single_module(
    source: &str,
    filename: &str,
    options: &TransformOptions,
) -> Result<SingleModuleOutput, Error> {
    let allocator = Allocator::default();
    let source_type = SourceType::from_path(filename)
        .unwrap_or_else(|_| SourceType::tsx());

    // 1. Parse
    let ret = Parser::new(&allocator, source, source_type).parse();
    if ret.panicked {
        return Err(anyhow!("Parser panicked on {}", filename));
    }
    let mut program = ret.program;

    // 2. Semantic analysis (scopes + symbols)
    let semantic_ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    // 3. Qwik transformation (traverse + mutate)
    let mut qwik_transform = QwikTransform::new(options, filename);
    // Note: actual traverse invocation requires TraverseCtx setup
    // oxc_traverse::traverse_mut(&mut qwik_transform, &allocator, &mut program);

    // 4. Code generation
    let codegen_result: CodegenReturn = Codegen::new()
        .with_source_text(source)
        .build(&program);

    Ok(SingleModuleOutput {
        code: codegen_result.code,
        map: codegen_result.map.map(|m| m.to_json_string()),
        segments: qwik_transform.extracted_segments(),
    })
}
```

### Detecting $() Call Sites in AST

```rust
// Source: Derived from spec file analysis + OXC Traverse patterns

/// Check if a CallExpression is a Qwik $-call
fn is_dollar_call(
    &self,
    call: &CallExpression<'_>,
) -> Option<DollarCallKind> {
    match &call.callee {
        // Direct $() call: $(() => { ... })
        Expression::Identifier(ident) if ident.name.as_str() == "$" => {
            if self.dollar_imports.contains("$") {
                Some(DollarCallKind::RawDollar)
            } else {
                None
            }
        }
        // Named $-suffixed call: component$(() => { ... })
        Expression::Identifier(ident) if ident.name.ends_with('$') => {
            let name = ident.name.as_str();
            if self.dollar_imports.contains(name) {
                Some(DollarCallKind::Named(name.to_string()))
            } else {
                None
            }
        }
        _ => None,
    }
}

enum DollarCallKind {
    RawDollar,                 // $(() => {...})
    Named(String),             // component$(() => {...})
}
```

### Building qrl() Wrapper Expression

```rust
// Source: Derived from spec output analysis + AstBuilder API

/// Build: qrl(i_hashValue, "SegmentName_hash")
fn build_qrl_call<'a>(
    import_ident_name: &str,
    segment_export_name: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Argument 1: identifier reference to the lazy import function
    let import_ref = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom(import_ident_name),  // e.g., "i_zBbHWn4e8Cg"
    );

    // Argument 2: string literal with the segment export name
    let name_literal = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(segment_export_name),  // e.g., "renderHeader_zBbHWn4e8Cg"
        None,
    );

    // Build arguments vector
    let mut arguments = ctx.ast.vec_with_capacity(2);
    arguments.push(Argument::from(import_ref));
    arguments.push(Argument::from(name_literal));

    // Build callee: identifier "qrl"
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("qrl"),
    );

    // Build: /*#__PURE__*/ qrl(i_hash, "name")
    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
    // Note: For PURE annotation, may need expression_call_with_pure or
    // manual leading comment attachment
}
```

### Building inlinedQrl() with Captures

```rust
// Source: Derived from example_inlined_entry_strategy spec + AstBuilder API

/// Build: inlinedQrl(() => { body }, "Name_hash", [captured_vars])
fn build_inlined_qrl_call<'a>(
    arrow_fn: Expression<'a>,
    segment_name: &str,
    captures: &[String],
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    let mut arguments = ctx.ast.vec_with_capacity(3);

    // Argument 1: the arrow function expression (kept inline)
    arguments.push(Argument::from(arrow_fn));

    // Argument 2: string literal segment name
    arguments.push(Argument::from(
        ctx.ast.expression_string_literal(SPAN, ctx.ast.atom(segment_name), None)
    ));

    // Argument 3: captures array (only if non-empty)
    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier_reference(
                    SPAN,
                    ctx.ast.atom(capture_name),
                )
            ));
        }
        arguments.push(Argument::from(
            ctx.ast.expression_array(SPAN, elements, None)
        ));
    }

    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("inlinedQrl"),
    );

    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}
```

### Building Lazy Import Declaration

```rust
// Source: Derived from spec output + AstBuilder API

/// Build: const i_hash = () => import("./file_segment_hash")
fn build_lazy_import<'a>(
    hash: &str,
    import_path: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let ident_name = format!("i_{}", hash);

    // Build: import("./file_segment_hash")
    let import_source = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(import_path),
        None,
    );
    let import_expr = ctx.ast.expression_import(
        SPAN, import_source, ctx.ast.vec(), None,
    );

    // Build: () => import(...)
    let params = ctx.ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ctx.ast.vec(),
        None,
    );
    let body = ctx.ast.function_body(SPAN, ctx.ast.vec(), ctx.ast.vec());
    // Note: For expression-body arrows, need to handle differently

    let arrow = ctx.ast.expression_arrow_function(
        SPAN,
        true,  // expression body
        false, // not async
        false, // not generator
        params,
        body, // simplified - actual implementation needs expression body
        None, // return type
        None, // type parameters
    );

    // Build: const i_hash = ...
    let binding = ctx.ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ctx.ast.atom(&ident_name),
    );
    let binding_pattern = ctx.ast.binding_pattern(binding, None, false);
    let declarator = ctx.ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        binding_pattern,
        Some(arrow),
        false,
    );
    let declaration = ctx.ast.variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ctx.ast.vec1(declarator),
        false,
    );

    Statement::from(Declaration::VariableDeclaration(
        ctx.ast.alloc(declaration)
    ))
}
```

## Data Flow Specification

### Top-Level Pipeline

```
TransformModulesOptions { input: Vec<TransformModuleInput>, ... }
  |
  v
transform_modules(options) -> Result<TransformOutput, Error>
  |
  |-- For each input module (potentially parallel via rayon):
  |     |
  |     v
  |   parse(source, filename) -> Program<'a>
  |     |
  |     v
  |   SemanticBuilder::build(&program) -> Scoping
  |     |
  |     v
  |   collector::collect(&program) -> GlobalCollectResult
  |     |  (identifies: imports, exports, $-calls, captured variables)
  |     v
  |   QwikTransform::traverse(&mut program, scoping, collected_info)
  |     |  (mutates AST: replaces $-calls with qrl/inlinedQrl wrappers,
  |     |   inserts import declarations, extracts segment bodies)
  |     v
  |   code_move::extract_segments(transform_result)
  |     |  (creates separate module ASTs for each extracted segment)
  |     v
  |   emit::codegen(program, segments) -> Vec<TransformModule>
  |     |  (generates JavaScript + source maps for main module + segments)
  |     v
  |   Collect all TransformModule + SegmentAnalysis
  |
  v
TransformOutput {
  modules: Vec<TransformModule>,   // main module + extracted segments
  diagnostics: Vec<Diagnostic>,    // errors/warnings
  is_type_script: bool,
  is_jsx: bool,
}
```

### Key Type Signatures

```rust
/// Main entry point - matches SWC optimizer's public API
pub fn transform_modules(
    config: TransformModulesOptions,
) -> Result<TransformOutput, anyhow::Error>;

/// Single module transformation (internal)
fn transform_module(
    allocator: &Allocator,
    input: &TransformModuleInput,
    options: &TransformOptions,
) -> Result<ModuleTransformResult, anyhow::Error>;

/// Segment extraction result
struct ModuleTransformResult {
    main_module: TransformModule,
    segments: Vec<TransformModule>,
    diagnostics: Vec<Diagnostic>,
}
```

## Public API Design

The public API must be wire-compatible with the existing SWC optimizer for the TypeScript binding layer:

```rust
// These types must serialize to identical JSON as the SWC versions
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformModulesOptions {
    pub src_dir: String,
    pub root_dir: Option<String>,
    pub input: Vec<TransformModuleInput>,
    pub source_maps: bool,
    pub minify: MinifyMode,
    pub transpile_ts: bool,
    pub transpile_jsx: bool,
    pub preserve_filenames: bool,
    pub entry_strategy: EntryStrategy,
    pub explicit_extensions: bool,
    pub mode: EmitMode,
    pub scope: Option<String>,
    pub core_module: Option<String>,
    pub strip_exports: Option<Vec<String>>,
    pub strip_ctx_name: Option<Vec<String>>,
    pub strip_event_handlers: bool,
    pub reg_ctx_name: Option<Vec<String>>,
    pub is_server: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformOutput {
    pub modules: Vec<TransformModule>,
    pub diagnostics: Vec<Diagnostic>,
    pub is_type_script: bool,
    pub is_jsx: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformModule {
    pub path: String,
    pub is_entry: bool,
    pub code: String,
    pub map: Option<String>,
    pub segment: Option<SegmentAnalysis>,
    pub orig_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentAnalysis {
    pub origin: String,
    pub name: String,
    pub entry: Option<String>,
    pub display_name: String,
    pub hash: String,
    pub canonical_filename: String,
    pub extension: String,
    pub parent: Option<String>,
    pub ctx_kind: CtxKind,   // "eventHandler" | "function"
    pub ctx_name: String,
    pub captures: bool,
    pub loc: (u32, u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EntryStrategy {
    Segment,
    Inline,
    Hoist,
    Single,
    Component,
    Smart,
    Hook,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MinifyMode {
    Simplify,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EmitMode {
    Lib,
    Prod,
    Dev,
}
```

## Test Strategy

### Approach: Spec-Based Snapshot Testing

The 162 spec files from v1.0 are the behavioral ground truth. Each spec file contains:
- **Input:** Source code (tsx/ts/js)
- **Output:** Transformed main module + extracted segment modules
- **Metadata:** SegmentAnalysis JSON for each segment
- **Configuration:** Test-specific options (entry strategy, transpile flags, etc.)

### Test Harness Design

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    /// Parse a spec file and extract input/output/config
    struct SpecFile {
        name: String,
        config: TransformModulesOptions,
        input_code: String,
        expected_modules: Vec<ExpectedModule>,
    }

    struct ExpectedModule {
        path: String,
        code: String,
        segment: Option<SegmentAnalysis>,
    }

    /// Load all spec files from .planning/spec/
    fn load_spec_files() -> Vec<SpecFile> {
        // Parse markdown spec files
        // Extract code blocks and metadata
    }

    #[test]
    fn test_all_specs() {
        let specs = load_spec_files();
        for spec in specs {
            let result = transform_modules(spec.config).unwrap();

            // Compare each output module
            for expected in &spec.expected_modules {
                let actual = result.modules.iter()
                    .find(|m| m.path == expected.path)
                    .unwrap_or_else(|| panic!(
                        "Missing module {} in spec {}", expected.path, spec.name
                    ));

                // Snapshot test the generated code
                assert_snapshot!(
                    format!("{}_{}", spec.name, expected.path),
                    actual.code
                );
            }
        }
    }
}
```

### Comparison Approach

1. **Code comparison:** Normalize whitespace, compare generated code against spec output. Use `insta` snapshots for readable diffs.
2. **Segment metadata comparison:** JSON-serialize `SegmentAnalysis`, compare field-by-field. Hash values must match exactly.
3. **Module count comparison:** Verify same number of output modules (main + segments).
4. **Import verification:** Check that the same imports appear in the same modules.

### Progressive Testing Strategy

Start with the simplest spec files and build up:
1. `example_1.md` - Basic $() extraction with qrl wrapping
2. `example_functional_component.md` - component$ with dollar-to-Qrl renaming
3. `example_capture_imports.md` - Capture patterns with CSS imports
4. `example_inlined_entry_strategy.md` - Inline strategy with captures

## State of the Art

| Old Approach (SWC) | Current Approach (OXC) | Impact |
|--------------------|-----------------------|--------|
| `swc_ecmascript` visitor with `fold_with` | `oxc_traverse::Traverse` with `enter_*`/`exit_*` | Different API surface; OXC provides parent context access |
| SWC `JsWord` for interned strings | OXC `Atom<'a>` for arena-allocated strings | OXC atoms tied to allocator lifetime |
| SWC `Visit`/`VisitMut`/`Fold` traits | OXC single `Traverse` trait | Simpler API; mutation via `*node = new_node` |
| SWC `swc_ecmascript::codegen::Emitter` | OXC `oxc_codegen::Codegen` | Builder pattern; 3-5x faster than SWC |
| SWC per-crate dependencies | OXC umbrella crate with features | Simpler Cargo.toml |
| No built-in semantic analysis in transform | `SemanticBuilder` -> `Scoping` in `TraverseCtx` | Built-in scope/symbol info during traversal |

**Deprecated/outdated:**
- SWC-based optimizer: The entire purpose of this project is to replace it. The SWC API patterns should NOT be replicated; use OXC idioms.
- `swc_atoms::JsWord`: Replaced by `oxc::span::Atom<'a>` in the new crate.

## Open Questions

1. **Exact hash algorithm from SWC optimizer**
   - What we know: The SWC optimizer generates 11-character hashes like `zBbHWn4e8Cg`. These appear in spec files and must be reproduced exactly.
   - What's unclear: The exact hash function (likely a custom base64-encoded hash). The SWC source code at `core/src/` is not in this repo.
   - Recommendation: Fetch the hash implementation from the Qwik monorepo (`QwikDev/qwik`). It should be a pure function that can be ported directly.

2. **PURE comment attachment in OXC Codegen**
   - What we know: OXC's `AstBuilder` has `call_expression_with_pure` and `expression_call_with_pure`. The Codegen must output `/*#__PURE__*/` before these calls.
   - What's unclear: Whether `expression_call_with_pure` actually emits the comment through Codegen, or if manual comment attachment is needed.
   - Recommendation: Verify during implementation by generating a PURE-annotated call and checking Codegen output.

3. **JSX handling in the OXC pipeline**
   - What we know: The SWC optimizer transpiles JSX as part of transformation. OXC has a JSX transformer built into `oxc_transformer`.
   - What's unclear: Whether we should use OXC's built-in JSX transform or handle JSX preservation (since some spec outputs still contain JSX).
   - Recommendation: Some spec files output JSX (when `transpile_jsx: false`). The optimizer must conditionally transpile JSX based on the `transpile_jsx` option.

4. **`onClick$` -> `q-e:click` attribute renaming**
   - What we know: In inline mode, `onClick$` JSX attributes become `q-e:click` (using JSXNamespacedName). This is visible in `example_inlined_entry_strategy`.
   - What's unclear: The complete mapping rules for all event handler attributes.
   - Recommendation: This is a Phase 5 concern (deep research). For Phase 4, document the basic pattern.

5. **Exact OXC feature flag combinations**
   - What we know: The umbrella crate `oxc` has many feature flags. We need parser, traverse, codegen, semantic, and possibly transformer.
   - What's unclear: Whether the `transformer` feature is needed (for `TransformState` and `ModuleImportsStore`) or if we should implement import management ourselves.
   - Recommendation: Start without the `transformer` feature and implement import management directly. The transformer feature pulls in babel-compat layers we don't need.

## Sources

### Primary (HIGH confidence)
- [OXC crate on crates.io](https://crates.io/crates/oxc) - Version 0.113.0, verified via API on 2026-02-10
- [AstBuilder docs.rs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) - Complete method signatures for AST node construction
- [oxc_traverse crate](https://docs.rs/oxc_traverse/latest/oxc_traverse/) - Traverse trait and TraverseCtx documentation
- [Codegen docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) - Code generation API
- [OXC transformer example](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/examples/transformer.rs) - Complete pipeline: parse -> semantic -> transform -> codegen
- [OXC traverse context source](https://github.com/oxc-project/oxc/blob/main/crates/oxc_traverse/src/context/mod.rs) - TraverseCtx struct with ast, ancestry, scoping fields
- [OXC JSX transformer source](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/jsx/jsx_impl.rs) - Expression replacement patterns via `*expr = new_expr`
- [OXC statement injector source](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/common/statement_injector.rs) - Statement insertion during traversal
- [OXC module imports source](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/common/module_imports.rs) - Import management during transformation
- [Qwik optimizer types.ts](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/src/types.ts) - Public API type definitions
- [Qwik optimizer lib.rs](https://github.com/QwikDev/qwik/blob/main/packages/qwik/src/optimizer/core/src/lib.rs) - Rust public API: transform_modules() signature

### Secondary (MEDIUM confidence)
- [OXC traverse README](https://github.com/oxc-project/oxc/tree/main/crates/oxc_traverse) - Overview of traversal design and ancestor safety model
- [OXC transformer usage guide](https://oxc.rs/docs/guide/usage/transformer.html) - High-level transformer usage patterns
- Phase 1 research (`01-RESEARCH.md`) in this repo - OXC parser patterns and arena allocator details
- Project spec files (162 files in `.planning/spec/`) - Behavioral ground truth for all transformation patterns

### Tertiary (LOW confidence)
- Training data knowledge of Qwik optimizer internals - May be outdated; verify against spec files
- OXC `expression_call_with_pure` behavior for `/*#__PURE__*/` comments - Needs runtime verification

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - OXC 0.113.0 verified on crates.io; all required APIs confirmed in docs.rs and source
- Architecture: HIGH - Module layout derived from SWC optimizer structure + OXC API patterns; public API types confirmed from Qwik monorepo
- API mapping: HIGH - AstBuilder methods verified from docs.rs; Traverse pattern confirmed from transformer source; code examples synthesized from verified APIs
- Pitfalls: HIGH - Arena lifetime issues documented in Phase 1 research; hash compatibility identified from spec file analysis
- Test strategy: MEDIUM - Spec file format understood but harness parsing not yet implemented; insta snapshot approach is standard

**Research date:** 2026-02-10
**Valid until:** 2026-03-10 (OXC API is stable at this point; weekly releases are additive)
