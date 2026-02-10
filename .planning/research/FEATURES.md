# Feature Landscape: OXC Transformer Patterns for Qwik Optimizer Port

**Domain:** OXC-based AST transformation patterns for code-splitting optimizer
**Researched:** 2026-02-10
**Confidence:** MEDIUM-HIGH (verified via OXC docs.rs, GitHub source, official guides; some APIs are pre-1.0 and may shift)

## Table Stakes

OXC API patterns that the optimizer MUST use. Missing any of these blocks the port entirely.

### 1. AST Traversal via `oxc_traverse`

The Qwik optimizer must walk the entire AST to find `$()` calls, `component$` patterns, JSX elements, import declarations, and variable bindings. OXC provides the `Traverse` trait for this.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Implement `Traverse` trait | `impl Traverse<'a, State> for QwikTransform` | Med | All transforms implement `Traverse`. Entry points are `enter_*` and `exit_*` methods with `&mut` access to current node and `TraverseCtx` for ancestors/scoping. |
| Detect `$()` call sites | `enter_call_expression` / `exit_call_expression` | Med | Match `CallExpression` where callee is `$` or ends with `$` (e.g., `component$`). Use `ctx.parent()` / `ctx.ancestor()` to determine context. |
| Detect JSX elements | `exit_expression` (match `Expression::JSXElement`) | Med | OXC's built-in JSX transformer uses `exit_expression` to catch JSX after children are processed. Same pattern applies. |
| Detect import declarations | `enter_import_declaration` | Low | Collect imports for rename analysis and to track which `$`-suffixed functions come from `@qwik.dev/core`. |
| Access parent/ancestor context | `ctx.parent()`, `ctx.ancestor(N)` via `Ancestor` enum | Med | `Ancestor` enum provides read access to all fields of parent nodes EXCEPT the branch being traversed (aliasing safety). Required for determining if a `$()` is inside an export, assignment, etc. |
| Maintain traversal state | Generic `State` parameter on `Traverse<'a, State>` | Low | Use `State` to accumulate discovered segments, collected imports, capture info, etc. during a single traversal pass. |
| Invoke traversal | `traverse_mut(traverser, allocator, program, scoping, state)` | Low | Returns `Scoping` (updated scope info). Requires `Scoping` from prior `SemanticBuilder::build()`. |

**API Pattern:**
```rust
struct QwikTransform<'a> {
    segments: Vec<SegmentData<'a>>,
    // ... state
}

impl<'a> Traverse<'a, TransformState<'a>> for QwikTransform<'a> {
    fn exit_expression(&mut self, expr: &mut Expression<'a>, ctx: &mut TraverseCtx<'a>) {
        // Match $() calls, JSX elements, etc.
    }
    fn enter_import_declaration(&mut self, decl: &mut ImportDeclaration<'a>, ctx: &mut TraverseCtx<'a>) {
        // Track imports from @qwik.dev/core
    }
}
```

**Confidence:** HIGH -- `Traverse` trait pattern confirmed via docs.rs, OXC transformer source (jsx_impl.rs, annotations.rs), and crates.io documentation.

### 2. AST Mutation (Replace, Wrap, Remove)

The optimizer must replace `component$(fn)` with `componentQrl(qrl(...))`, replace JSX with `_jsxSorted()` calls, remove type annotations, and wrap expressions.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Replace expression in-place | `*expr = new_expression` (direct assignment in `exit_expression`) | Low | Standard pattern: build new `Expression` via `ctx.ast`, assign via `*expr = ...`. Confirmed in OXC JSX transformer (`*expr = self.transform_jsx_element(e, ctx)`). |
| Take/move existing node out | `expr.take_in(ctx.ast)` or `std::mem::replace` | Low | `TakeIn` trait replaces node with a dummy and returns the original, allowing you to decompose and rebuild. Used by OXC JSX transformer: `match expr.take_in(ctx.ast) { ... }`. |
| Remove statements | `stmts.retain_mut(\|stmt\| ...)` in `exit_statements` | Med | OXC TypeScript transformer uses `retain_mut` to filter out type-only declarations. Same pattern for removing stripped exports/contexts. |
| Insert new statements | `StatementInjector` pattern (store pending insertions, apply in `exit_statements`) | Med | OXC uses `FxHashMap<Address, Vec<AdjacentStatement>>` to queue insertions. Apply in `exit_statements` by draining and rebuilding the statement Vec. Not a public API -- must implement in optimizer. |
| Wrap expression (e.g., `qrl(...)` around lazy import) | Build new `CallExpression` with original as argument | Med | Use `ctx.ast.expression_call(span, callee, args, false)` where one of the args contains the original expression. |
| Add `/*#__PURE__*/` annotations | `ctx.ast.expression_call_with_pure(..., true)` | Low | `AstBuilder::expression_call_with_pure` has a `pure: bool` parameter that adds the annotation. Confirmed in docs.rs. |

**Key insight:** OXC mutations happen in `exit_*` methods (post-order) so children are already processed. This matters because the Qwik optimizer must process nested `$()` calls (inner segments before outer segments).

**Confidence:** HIGH -- replacement via `*expr = ...` and `take_in` confirmed in OXC JSX transformer source code. Statement filtering via `retain_mut` confirmed in TypeScript annotations transformer.

### 3. AST Node Construction via `AstBuilder`

The optimizer must construct entirely new AST nodes: `qrl()` calls, `inlinedQrl()` calls, `_jsxSorted()` calls, lazy import declarations, export declarations for segments, `_captures` member expressions, and more.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Access builder during traversal | `ctx.ast` (the `AstBuilder<'a>` on `TraverseCtx`) | Low | Always available during traversal. All node construction goes through this. |
| Build call expressions | `ctx.ast.expression_call(span, callee, args, optional)` | Med | For `qrl(import_fn, "segmentName")`, `componentQrl(qrl_expr)`, `_jsxSorted(tag, varProps, constProps, children, flags, key)`. |
| Build identifier references | `ctx.ast.expression_identifier(span, name)` | Low | For referencing `qrl`, `componentQrl`, `_jsxSorted`, `_captures`, etc. |
| Build string literals | `ctx.ast.expression_string_literal(span, value, raw)` | Low | For segment names in `qrl()` calls, event handler names, CSS strings in `useStyles$`. |
| Build arrow functions | `ctx.ast.expression_arrow_function_with_scope_id_and_pure_and_pife(...)` | High | For lazy import functions: `() => import("./segment_file")`. Requires scope ID from semantic analysis. |
| Build import declarations | `ctx.ast.import_declaration(span, specifiers, source, ...)` | Med | For synthetic imports in generated segment files. |
| Build export declarations | `ctx.ast.plain_export_named_declaration_declaration(span, decl)` | Med | For `export const SegmentName = () => { ... }` in segment files. |
| Build variable declarations | `ctx.ast.declaration_variable(span, kind, declarators, false)` | Med | For `const i_HASH = () => import("./...")` lazy import variables. |
| Build member expressions | `ctx.ast.expression_member(...)` / static member | Med | For `_captures[0]`, `_rawProps.propName`, `signal.value` patterns. |
| Build array expressions | `ctx.ast.expression_array(span, elements, trailing_comma)` | Low | For capture arrays passed as third argument to `qrl()`. |
| Allocate vectors | `ctx.ast.vec()`, `ctx.ast.vec1(item)`, `ctx.ast.vec_from_iter(iter)` | Low | All child lists (arguments, statements, specifiers) are `Vec<'a, T>` arena-allocated. |
| Allocate strings | `ctx.ast.atom("string_value")` | Low | All string data in AST nodes is `Atom<'a>` arena-allocated. |

**Productivity tool:** [js_to_oxc](https://github.com/KermanX/js_to_oxc) converts JS source to OXC AstBuilder Rust code. Supports `$`-prefixed holes for dynamic values. Use this to generate the boilerplate for complex output patterns like `qrl()` calls and `_jsxSorted()` calls, then parameterize the holes.

**Confidence:** HIGH -- AstBuilder API confirmed via docs.rs and ast_builder_impl.rs source. `ctx.ast` access during traversal confirmed via multiple transformer source files.

### 4. Semantic Analysis via `oxc_semantic`

The optimizer must perform capture analysis: determining which variables referenced inside a `$()` closure are declared outside it. This requires scope and binding resolution.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Build semantic info | `SemanticBuilder::new().build(&program)` | Low | Run after parsing, before traversal. Produces `Semantic` with scoping, symbol table, reference tracking. |
| Get Scoping for traversal | `semantic.into_scoping()` | Low | `traverse_mut()` requires `Scoping` as input. |
| Find all references to a symbol | `scoping.get_resolved_references(symbol_id)` | Med | Returns iterator of `Reference` structs with node IDs and flags (read/write). |
| Get symbol's declaring scope | `scoping.symbol_scope_id(symbol_id)` | Low | Returns `ScopeId` where the symbol was declared. |
| Walk scope ancestry | `scoping.scope_ancestors(scope_id)` | Low | Iterator of ancestor scope IDs including the scope itself. |
| Find binding in scope chain | `scoping.find_binding(scope_id, name)` | Low | Walks up scope tree looking for a name. Returns `Option<SymbolId>`. |
| Detect closure capture | Compare `symbol_scope_id(sym)` vs reference scope | High | If a reference's scope is a descendant of the symbol's scope but NOT the same scope, the reference is a capture. Core algorithm for Qwik capture analysis. |
| Check if symbol is mutated | `scoping.symbol_is_mutated(symbol_id)` | Low | Useful for immutability analysis in derived signal optimization. |
| Iterate bindings in scope | `scoping.iter_bindings_in(scope_id)` | Low | List all symbols declared in a scope. Useful for collecting local vs captured identifiers. |

**Capture analysis algorithm using OXC semantic:**
```rust
fn find_captures(
    closure_scope: ScopeId,
    scoping: &Scoping,
) -> Vec<SymbolId> {
    let mut captures = Vec::new();
    // Walk all references in the closure's scope and descendants
    for sym_id in scoping.iter_bindings_in(closure_scope) {
        // These are local -- not captures
    }
    // For each reference that resolves to a symbol declared
    // in an ancestor scope of the closure: it's a capture
    // ... (requires traversing the AST within the closure scope)
    captures
}
```

**Confidence:** HIGH -- Scoping API confirmed via docs.rs documentation. `get_resolved_references`, `symbol_scope_id`, `scope_ancestors`, `find_binding` all documented with clear types.

### 5. Code Generation via `oxc_codegen`

The optimizer must produce JavaScript source code from modified ASTs -- both the transformed main module and each extracted segment module.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Generate JS from AST | `Codegen::new().build(&program)` | Low | Returns `CodegenReturn { code: String, map: Option<SourceMap>, legal_comments: Vec<Comment> }`. |
| Enable source maps | `CodegenOptions { source_map_path: Some(path) }` | Low | Set `source_map_path` to get `map: Some(SourceMap)` in the return. |
| Provide original source text | `.with_source_text(original_source)` | Low | Required for accurate source map generation -- the codegen needs the original text to compute mappings. |
| Provide scoping for mangling | `.with_scoping(scoping)` | Low | Optional; enables identifier renaming if desired. Not needed for basic codegen. |
| Minified output | `CodegenOptions { minify: true }` (if available) | Low | Check CodegenOptions for minification flag. Likely available given OXC's minifier. |
| Print single expression | `codegen.print_expression(expr)` | Low | For `sync$` serialization where a single expression needs to be stringified. |

**Critical pattern for Qwik optimizer:** The optimizer produces MULTIPLE output modules from a single input. Each output module (main module + N segment modules) must be a separately constructed `Program<'a>` that gets independently codegen'd. This means:

1. Parse input -> `Program<'a>`
2. Run semantic analysis
3. Traverse and collect segment data
4. For each segment: construct a NEW `Program<'a>` with the segment's body, imports, exports
5. Codegen each `Program` independently
6. Return all `CodegenReturn` values as the `TransformOutput`

**Confidence:** HIGH -- `Codegen::new().build(&program)` confirmed via docs.rs with full example. `CodegenReturn` fields confirmed. Source map option confirmed.

### 6. Source Map Production

Each output module needs a source map mapping back to the original input source.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Per-module source maps | `Codegen::new().with_options(CodegenOptions { source_map_path: Some(...) }).with_source_text(src).build(&program)` | Med | Each segment `Program` gets its own codegen with source maps enabled. |
| Source map composition | `ConcatSourceMapBuilder` from `oxc_sourcemap` | High | If doing multi-pass transforms (e.g., TS transpile then Qwik transform), source maps from each pass must be chained. |
| Source map serialization | `source_map.to_json_string()` (likely method) | Low | Convert `SourceMap` to JSON string for output in `TransformOutput`. |
| Span preservation | Carry original spans through AST construction | High | When building new AST nodes, use spans from the original source code so codegen can produce correct source map mappings. If spans are `SPAN` (0,0), source maps will not map correctly. |

**Key challenge:** When building new `Program<'a>` nodes for segments, the spans in the constructed AST must point back to the original source positions for source maps to work. This means: when extracting a closure body from the input AST and moving it to a segment's `Program`, preserve the original spans.

**Confidence:** MEDIUM -- `CodegenReturn.map` confirmed as `Option<SourceMap>`. `ConcatSourceMapBuilder` exists in `oxc_sourcemap` but exact composition API needs verification during implementation. Span preservation strategy is based on how OXC's own transformer handles this (they preserve original spans when possible).

### 7. Multi-Module Output (Code Splitting)

The optimizer splits one input file into multiple output modules. This is the most architecturally significant pattern.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Build separate `Program` per segment | `AstBuilder::new(allocator)` outside traversal | High | Each segment module is a new `Program` with its own body, imports, exports. Must be constructed AFTER traversal collects segment data. |
| Allocator lifetime management | One `Allocator` per output module, OR share the input allocator | High | Arena allocator owns all AST memory. If sharing one allocator, all Programs share the same arena (simple, more memory). If separate allocators, each Program is independent (cleaner, but cannot share nodes). |
| Program construction | `AstBuilder::program(span, source_type, hashbang, directives, body)` (likely) | Med | The exact API for constructing a `Program` from scratch needs verification. Build the body as `Vec<'a, Statement>`. |
| Import hoisting for segments | Build `ImportDeclaration` nodes manually | Med | Captured imports from the original module must be recreated as new `ImportDeclaration` nodes in the segment's `Program`. |
| Export wrapping for segments | Build `ExportNamedDeclaration` around segment body | Med | Each segment exports its entry function: `export const SegmentName = () => { ... }`. |

**Recommended architecture:**
1. **Pass 1 (Traverse):** Walk input AST, collect all segment data (closure bodies, captures, imports needed) into a Vec of segment descriptors. Mutate the main module's AST in-place (replace `component$(fn)` with `componentQrl(qrl(...))`).
2. **Pass 2 (Build segments):** For each segment descriptor, construct a new `Program<'a>` with the appropriate imports, export declaration, and function body.
3. **Pass 3 (Codegen):** Run `Codegen::build` on the main module and each segment `Program` independently.

**Confidence:** MEDIUM -- This is the novel part with no direct OXC precedent. OXC transformers modify in-place; they do not split one file into many. The pattern must be invented for this optimizer. The underlying APIs (AstBuilder, Codegen) are confirmed; the composition pattern is new.

## Differentiators

Features that enhance the optimizer but are not strictly required for basic functionality.

| Feature | OXC API | Complexity | Notes |
|---------|---------|------------|-------|
| Single-pass traversal for all transforms | Compose multiple `Traverse` impls | Med | OXC's transformer runs TS removal, JSX transform, and lowering in a single traversal pass. The Qwik optimizer could similarly handle `$()` extraction, JSX transform, props destructuring, and signal analysis in one pass -- but ONLY if ordering constraints allow it. |
| `js_to_oxc` for template generation | External tool: `KermanX/js_to_oxc` | Low | Use the online tool or CLI to generate Rust `AstBuilder` code from JS output templates. Dramatically reduces boilerplate for constructing complex output patterns like `qrl()` calls. |
| `BoundIdentifier` for safe renaming | `TraverseCtx::generate_uid(name, scope, flags)` | Med | When creating new identifiers (e.g., `_rawProps`, `_hf0`), use `ctx.generate_uid()` to avoid name collisions with existing bindings. Returns `BoundIdentifier` with correct `SymbolId`. |
| Reusable traverse context | `traverse_mut_with_ctx` + `ReusableTraverseCtx` | Low | If doing multiple traversal passes, reuse the context to avoid re-allocating internal state. |
| TS transpilation via OXC transformer | `oxc::transformer::Transformer` with `TransformOptions` for TS/JSX | Low | When `transpile_ts: true`, use OXC's built-in TypeScript transformer before running Qwik transform. Avoids reimplementing TS stripping. |
| JSX transpilation via OXC transformer | `oxc::transformer::Transformer` with JSX options | Low | When `transpile_jsx: true`, use OXC's built-in JSX transformer (React classic or automatic mode) as a post-pass for segments that need JSX transpiled. |
| Diagnostic emission | Custom error/warning reporting | Low | Use OXC's `OxcDiagnostic` type or custom diagnostics for "QWIK(x): ..." error messages (e.g., invalid segment expressions, missing `wrap()` functions). |

## Anti-Features

Patterns to explicitly NOT use in the OXC port.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| `Visit` / `VisitMut` traits for mutation | Read-only visitors; cannot mutate safely with parent context | Use `Traverse` trait from `oxc_traverse` which provides `TraverseCtx` with ancestor access and scoping |
| Building AST from string parsing | Parsing strings to create nodes is wasteful and loses span info | Use `AstBuilder` methods to construct nodes programmatically. Reserve parsing for the initial input only. |
| Sharing AST nodes between Programs | Arena allocator makes cross-allocator references unsound | Clone/rebuild nodes for each output module's `Program`. Use `AstBuilder` to construct fresh nodes with correct spans. |
| `unsafe` for node replacement | Tempting to use raw pointer tricks for complex mutations | Use `take_in()`, `std::mem::replace`, and `*expr = ...` patterns which OXC explicitly supports |
| Global mutable state during traversal | Rust borrow checker will fight you | Use the `State` generic on `Traverse<'a, State>` for accumulated data. Use `TraverseCtx` for scoping queries. |
| Multiple sequential full-AST passes | Performance cost multiplies with each pass | Combine transforms into a single `Traverse` impl where possible. Use `StatementInjector` pattern for deferred insertions. |
| Constructing `Program` via parsing generated strings | Tempting for complex segment output (parse a string template) | Construct via `AstBuilder`. The `js_to_oxc` tool can generate the builder code from template strings as a dev-time aid. |

## Feature Dependencies

```
Parsing
  --> Semantic Analysis (requires parsed Program)
    --> Traverse with Scoping (requires Scoping from semantic)
      --> $() Detection (enter_call_expression)
      --> JSX Detection (exit_expression matching JSXElement)
      --> Import Collection (enter_import_declaration)
      --> Capture Analysis (scope comparison using Scoping)
        --> Segment Data Collection (captures + closure body)
          --> Main Module Mutation (replace component$, generate lazy imports)
          --> Segment Program Construction (AstBuilder, new Program per segment)
            --> Codegen for Main Module
            --> Codegen for Each Segment
              --> Source Map Generation (per module)
                --> TransformOutput Assembly

Separately:
  TS Transpilation (OXC Transformer) --> before Qwik transform
  JSX Transpilation (OXC Transformer) --> after Qwik transform (on segments)
```

**Critical ordering constraints:**
1. **Semantic analysis BEFORE traversal** -- `traverse_mut` requires `Scoping`
2. **Inner `$()` calls BEFORE outer** -- use `exit_*` (post-order) so nested segments are processed first
3. **Capture analysis DURING traversal** -- scope info available via `ctx.scoping()`
4. **Segment construction AFTER traversal** -- cannot build segment Programs while still traversing the input
5. **TS transpile BEFORE Qwik transform** -- type annotations must be stripped before capture analysis (types are not runtime values)
6. **JSX transpile AFTER Qwik transform** -- Qwik's JSX transform produces `_jsxSorted` calls, not React `createElement`

## Complexity Assessment per Pattern

| Pattern | Complexity | Risk | Notes |
|---------|------------|------|-------|
| Traversal setup (Traverse trait) | Low | Low | Well-documented, many examples in OXC codebase |
| Expression replacement | Low | Low | Standard `*expr = ...` assignment, proven pattern |
| AstBuilder node construction | Med | Med | Verbose but mechanical. `js_to_oxc` tool helps. Risk: API may change between OXC versions |
| Semantic/capture analysis | High | High | Core algorithm must be correct. OXC provides the scope/reference data but the capture detection logic is custom |
| Statement injection/removal | Med | Med | No public StatementInjector API; must implement the HashMap-based pattern |
| Multi-module output | High | High | Novel pattern with no OXC precedent. Allocator lifetime management is the key risk |
| Source map preservation | Med | Med | Span fidelity during AST construction determines source map quality |
| Source map chaining | High | Med | `ConcatSourceMapBuilder` exists but exact multi-pass composition needs verification |
| Single-pass optimization | Med | Low | Desirable but not required. Can start with multi-pass and optimize later |

## MVP Recommendation

Build the OXC optimizer in this order:

### Phase 1: Traversal + Detection
1. Set up `Traverse` impl with `enter_call_expression` and `enter_import_declaration`
2. Detect `$()` / `component$` / `foo$` call sites
3. Collect import information from `@qwik.dev/core`
4. **Test:** Input AST is traversed, `$()` calls are identified, no mutations yet

### Phase 2: Semantic + Capture Analysis
5. Run `SemanticBuilder` before traversal
6. Implement capture analysis using `Scoping` APIs
7. Classify local vs captured identifiers for each `$()` closure
8. **Test:** Capture lists match spec files for all 162 tests

### Phase 3: Main Module Mutation
9. Replace `component$(fn)` with `componentQrl(qrl(lazy_import, name))`
10. Generate lazy import declarations (`const i_HASH = () => import("./segment")`)
11. Rewrite imports (`component$` import removed, `componentQrl` + `qrl` imports added)
12. **Test:** Main module output matches spec for core test cases

### Phase 4: Segment Module Construction
13. Build `Program` per segment with imports, export, function body
14. Handle capture restoration (`const x = _captures[0]`)
15. Handle import hoisting (re-import what the segment needs)
16. **Test:** Segment module output matches spec

### Phase 5: Codegen + Source Maps
17. Run `Codegen::build` on main module and each segment
18. Enable source map generation via `CodegenOptions`
19. Assemble `TransformOutput` with all modules and metadata
20. **Test:** Full end-to-end output matches spec for all 162 tests

### Phase 6: JSX + Advanced Features
21. JSX to `_jsxSorted`/`_jsxSplit` transformation
22. Props destructuring optimization
23. Derived signal optimization (`_wrapProp`, `_fnSignal`)
24. Entry strategies, emit modes, code stripping

### Defer
- `sync$` serialization (small scope, can add anytime)
- `bind:value`/`bind:checked` (isolated feature)
- Custom inlined functions via `wrap()` (edge case)
- `@jsxImportSource` handling (React interop)
- Windows path normalization (infrastructure)

## Sources

### HIGH Confidence (official docs, source code)
- [oxc_traverse docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/) -- Traverse trait, TraverseCtx, Ancestor, traverse_mut
- [oxc_traverse lib.rs (GitHub)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_traverse/src/lib.rs) -- traverse_mut signature, public API
- [oxc_ast AstBuilder docs.rs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) -- Node construction methods
- [oxc_ast AstBuilder impl (GitHub)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_ast/src/ast_builder_impl.rs) -- Implementation details
- [Codegen docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) -- Code generation API, CodegenReturn
- [CodegenReturn docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.CodegenReturn.html) -- { code, map, legal_comments }
- [oxc_semantic Scoping docs.rs](https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.Scoping.html) -- Symbol/scope/reference query API
- [OXC Semantic Analysis guide](https://oxc.rs/docs/learn/parser_in_rust/semantic_analysis) -- Scope building, symbol resolution
- [OXC Transformer usage guide](https://oxc.rs/docs/guide/usage/transformer.html) -- Built-in TS/JSX transforms
- [OXC JSX transformer source (GitHub)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/jsx/jsx_impl.rs) -- exit_expression pattern, expression_call_with_pure
- [OXC TS annotations transformer (GitHub)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/typescript/annotations.rs) -- retain_mut for statement removal
- [OXC StatementInjector source (GitHub)](https://github.com/oxc-project/oxc/blob/main/crates/oxc_transformer/src/common/statement_injector.rs) -- HashMap-based injection pattern

### MEDIUM Confidence (GitHub issues, community tools)
- [oxc_sourcemap docs.rs](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/) -- SourceMap, SourceMapBuilder, ConcatSourceMapBuilder
- [Statement replacement discussion (GitHub #5359)](https://github.com/oxc-project/oxc/issues/5359) -- move_expression, TakeIn, replacement patterns
- [Statement helper discussion (GitHub #6993)](https://github.com/oxc-project/oxc/issues/6993) -- insert/delete/replace status, closed as not planned
- [Statement insertion discussion (GitHub #4767)](https://github.com/oxc-project/oxc/issues/4767) -- exit_statements drain pattern
- [js_to_oxc tool (GitHub)](https://github.com/KermanX/js_to_oxc) -- JS-to-AstBuilder code generation, hole system

### LOW Confidence (needs verification during implementation)
- Multi-module output pattern (no OXC precedent, architecture must be invented)
- Source map chaining across TS-transpile + Qwik-transform passes
- Exact `Program` construction API via AstBuilder (verify method signature)
- Allocator sharing strategy for multiple output Programs

---
*Feature landscape for: OXC transformer patterns for Qwik optimizer port*
*Researched: 2026-02-10*
