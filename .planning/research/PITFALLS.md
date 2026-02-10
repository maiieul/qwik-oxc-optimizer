# Domain Pitfalls: OXC Optimizer Port

**Domain:** Rust code transformer port (SWC to OXC) -- multi-output code splitter with capture analysis
**Researched:** 2026-02-10
**Confidence:** HIGH (verified via OXC source code, official docs.rs API docs, GitHub issues, and codebase analysis of 162 spec files)

---

## Critical Pitfalls

Mistakes that cause rewrites, runtime failures, or fundamental architecture problems.

---

### Pitfall 1: Arena Lifetime Infection -- Every Helper Function Needs `'a`

**What goes wrong:**
OXC's arena allocator (`oxc_allocator::Allocator`) owns all AST memory. Every AST node carries lifetime `'a` tied to the allocator. When porting SWC helper functions that create or return AST nodes, the lifetime `'a` "infects" every function signature in the call chain. A helper that was `fn make_import() -> ImportDeclaration` in SWC becomes `fn make_import<'a>(ast: &AstBuilder<'a>) -> ImportDeclaration<'a>` in OXC. If you have 5 layers of helper functions, all 5 must carry `'a`.

This is not a bug you fix once. It is a structural constraint that reshapes how you write every function in the optimizer. The SWC optimizer has helper functions in `code_move.rs` (builds new module ASTs), `inlined_fn.rs` (transforms inline strategies), and scattered throughout `transform.rs`. Every one of these becomes lifetime-parameterized.

**Why it happens:**
SWC uses `Box<T>` and `Vec<T>` -- standard heap allocation with no lifetime parameter. You can freely store, return, and move AST nodes. OXC uses `oxc_allocator::Box<'a, T>` and `oxc_allocator::Vec<'a, T>`. The `'a` ties the node to the allocator's lifetime. Rust's borrow checker enforces this at compile time -- you literally cannot write the old signatures.

**Consequences:**
- Dozens of compile errors when first porting helper functions
- Temptation to use `unsafe` to "just make it work"
- Architecture degrades into passing `&AstBuilder<'a>` or `&TraverseCtx<'a>` through every function
- Helper functions that worked independently in SWC now need the allocator context

**Prevention:**
Design the helper function architecture around arena constraints from day one. Accept that `TraverseCtx<'a>` (which contains `AstBuilder<'a>`) is a required parameter for any function that creates AST nodes. Organize helpers into two categories:

1. **Pure analysis functions** (no AST creation) -- these do NOT need `'a` and should be kept lifetime-free
2. **AST construction functions** (create/modify nodes) -- these MUST take `&AstBuilder<'a>` or `&mut TraverseCtx<'a>`

Separate analysis from construction. The capture analysis phase (identifying which variables are captured) produces data (lists of symbol IDs, import metadata). The code generation phase (building new module ASTs) consumes that data and needs the allocator. Keep these phases distinct.

**Detection:**
- Compile errors mentioning `lifetime 'a` in function signatures
- Any use of `unsafe` related to lifetimes
- Functions taking more than 3 parameters that include allocator/context references (restructure into a context struct)

**Phase to address:** Port architecture design (before writing any transform code). Define the helper function convention in a project-level coding standard.

---

### Pitfall 2: `move_expression` Leaves Arena Garbage -- Cannot "Take" Nodes Cleanly

**What goes wrong:**
The Qwik optimizer extracts `$()` call expressions from the AST and moves them into separate modules. In SWC, you take ownership of an expression and put a replacement in its place: `std::mem::replace(&mut node.expr, replacement_expr)`. In OXC, `AstBuilder::move_expression` does something subtle: it replaces the original with a dummy `NullExpression` allocated in the arena. This dummy allocation advances the bump allocator pointer by 8 bytes every time. For files with many `$()` boundaries, these dummy allocations accumulate.

More critically, the dummy `NullExpression` left behind is a valid AST node. If any subsequent traversal pass encounters it without expecting it, it will be processed as real code. If codegen encounters it, it will emit `null` into the output.

**Why it happens:**
Arena allocators cannot deallocate individual nodes. When you "move" a node out, something must remain at the old memory location because other parts of the AST (parent nodes, vectors) still hold a reference to that slot. OXC chose to fill the slot with a `NullExpression` via the `Dummy` trait. This is documented in [GitHub issue #5359](https://github.com/oxc-project/oxc/issues/5359), which discusses making `move_*` cheaper by adding `Expression::None` / `Statement::None` enum variants that require zero allocation.

**Consequences:**
- Phantom `null` expressions appear in generated output if moved-from nodes are not properly replaced
- Arena memory grows with each move operation (8 bytes per `NullExpression`)
- Subsequent traversal passes must handle or skip dummy nodes
- Subtle bugs where "empty" slots silently produce incorrect output

**Prevention:**
Always replace moved expressions in the same operation. Never move an expression without immediately putting the real replacement back:

```rust
// WRONG: move then later replace
let extracted = ctx.ast.move_expression(&mut call_expr.argument);
// ... call_expr.argument is now NullExpression
// ... if anything traverses the tree before replacement, it sees null

// RIGHT: extract and replace atomically
let extracted = std::mem::replace(
    &mut call_expr.argument,
    ctx.ast.expression_identifier_reference(SPAN, "placeholder")
);
```

For statement removal (removing `$()` declarations that were extracted), replace with `Statement::Empty` or collect indices and remove in a cleanup pass during `exit_statements`.

Use the `exit_statements` pattern from [issue #4767](https://github.com/oxc-project/oxc/issues/4767): accumulate pending statement insertions/removals in a `HashMap<AstNodeId, Vec<Statement>>`, then process all mutations in the `exit_statements` callback where you have access to the full statement list.

**Detection:**
- `null` appearing in generated output where no `null` exists in input
- Unexpected `NullExpression` nodes found during AST dumps
- Growing arena memory usage on large files (measure with `Allocator` stats if available)

**Phase to address:** Core transform implementation. Establish the extract-and-replace pattern as a project convention before writing any `$()` extraction code.

---

### Pitfall 3: Semantic Info Invalidation After AST Mutation

**What goes wrong:**
You run `SemanticBuilder` to get scope trees, symbol tables, and reference resolution. Then you start mutating the AST (extracting expressions, rewriting imports, inserting new declarations). After mutation, all the `ScopeId`, `SymbolId`, and `ReferenceId` values stored in the original semantic data are stale. Nodes you removed still have entries in the symbol table. New nodes you created have no semantic data. If you query the semantic info after mutation, you get wrong answers.

The Qwik optimizer needs semantic info both BEFORE mutation (to analyze captures) and DURING mutation (to resolve identifiers in newly created nodes). This creates a chicken-and-egg problem.

**Why it happens:**
OXC's semantic analysis is a one-shot pass. `SemanticBuilder::new().build(&program)` traverses the AST and builds symbol tables. There is no incremental update API. When you add a new `import` statement or remove a variable declaration, the semantic data does not automatically update. The `TraverseCtx` provides `ctx.scoping_mut()` for manual updates, but you must update scope IDs, symbol entries, and references yourself.

This differs from SWC where `SyntaxContext` is baked into each identifier at parse time and remains valid regardless of AST mutations. SWC's `Id = (Atom, SyntaxContext)` identity is stable across mutations because it is a property of the token, not a graph relationship.

**Consequences:**
- Capture analysis returns wrong results if run after partial mutations
- Newly created identifiers (e.g., `_captures[0]`) have no `SymbolId` and cannot be looked up
- Scope queries return incorrect ancestors after nodes are moved between scopes
- Silent correctness bugs: code compiles and runs but produces wrong captures or imports

**Prevention:**
Run all analysis before any mutation. Structure the optimizer as a two-phase pipeline:

**Phase A (Read-Only Analysis):** Parse -> SemanticBuilder -> analyze captures, identify `$()` calls, determine imports/exports, classify variables. Store results in your own data structures (not OXC semantic IDs).

**Phase B (Mutation):** Using analysis results from Phase A, mutate the AST. For new nodes, use `ctx.generate_binding()` to create properly scoped identifiers. Use `ctx.create_bound_reference()` and `ctx.create_unbound_reference()` to register new references with the semantic system.

During traversal with `Traverse`, use `ctx.scoping_mut()` to update the scope tree when adding new scopes or bindings. The `TraverseScoping` API supports `rename_symbol()`, and the `Scoping::iter_bindings_in()` method for targeted updates.

**Detection:**
- Capture analysis produces different results when run before vs. after AST mutations
- `SymbolId` lookups return `None` for identifiers you just created
- Test failures that appear only when multiple `$()` boundaries exist in the same file (single-boundary tests pass because mutations are minimal)

**Phase to address:** Architecture design. The two-phase (analyze-then-mutate) pattern must be the foundational architecture decision.

---

### Pitfall 4: Building New Modules Requires a Separate Allocator Per Output

**What goes wrong:**
The Qwik optimizer produces multiple output modules from a single input file. Each extracted `$()` expression becomes its own module with its own imports, exports, and code. In SWC, `code_move.rs::new_module` builds a fresh `ast::Module` using standard heap allocation -- no lifetime concerns. In OXC, each `Program<'a>` is tied to an `Allocator`'s lifetime. You cannot build nodes for a new output module using the input module's allocator during traversal, because the input's allocator is borrowed mutably by the traverse.

If you try to build output module AST nodes during the input module's traversal, you hit a double-borrow: the traverse holds `&mut Allocator` via `TraverseCtx`, and building output nodes also needs `&Allocator`.

**Why it happens:**
OXC's `Allocator` is a bump allocator that owns all memory for nodes allocated within it. The `Program` returned by parsing borrows the allocator. The `Traverse` borrows the program (and transitively the allocator). You cannot create a second `Program` in the same allocator while traversing the first one.

**Consequences:**
- Cannot build output module ASTs during the input traversal pass
- Attempting to share one allocator between input and output modules causes borrow conflicts
- Architecture must handle the one-allocator-per-program constraint

**Prevention:**
Use a two-stage approach for multi-output generation:

**Stage 1 (During Traversal):** Collect the data needed to build output modules -- extracted expression source spans, captured variable lists, import requirements, segment metadata. Store this as plain Rust data (no AST references -- use `String`, `Vec<String>`, spans as `(u32, u32)`, etc.). Use `CloneIn` to deep-copy extracted AST subtrees into a separate allocator if you need to preserve AST fragments.

**Stage 2 (After Traversal):** For each output module, create a new `Allocator`, build a fresh `Program` using `AstBuilder`, populate it with the collected data, and run `Codegen` to produce the output string and source map.

Alternative: Use `CloneIn` to clone extracted subtrees into per-module allocators. The `clone_in(&allocator)` method on AST nodes performs a deep copy into the target allocator.

```rust
// Clone an expression subtree into a new allocator for an output module
let output_allocator = Allocator::default();
let cloned_expr = extracted_expr.clone_in(&output_allocator);
```

**Detection:**
- Borrow checker errors about `Allocator` being borrowed mutably and immutably at the same time
- Lifetime errors where output module ASTs outlive the input allocator
- Architecture designs that try to "share" a single allocator across input and all output modules

**Phase to address:** Architecture design. The multi-allocator pattern must be decided before any code is written.

---

### Pitfall 5: Capture Analysis Scope Boundary Edge Cases

**What goes wrong:**
The Qwik optimizer must determine which variables from the enclosing scope are "captured" by a `$()` expression. This sounds simple but has at least 8 distinct edge cases, each of which has caused bugs in real-world Qwik code:

1. **Imports are NOT captured.** `import { thing } from './sibling'` used inside `$()` is not a capture -- the output module gets its own import statement. The spec files `example_capture_imports.md` confirms this: `css1` and `css2` are imported in the output module, not captured.

2. **Exports at module root are NOT captured.** Exported variables are available globally.

3. **Loop iteration variables ARE captured and need special parameter passing.** The spec `example_component_with_event_listeners_inside_loop.md` shows that `for-i` loop variables become `_captures[N]` references. Indexed loops (for-i, for-in, while) capture the loop counter differently from for-of/map iterators.

4. **Shadowed variables must use the INNER binding.** If a variable name `x` exists in both outer and inner scope, the `$()` body uses the inner `x`, which is NOT a capture.

5. **Destructured component props need transformation.** `({foo}) => ...` in `component$` must be analyzed for which destructured fields are accessed inside nested `$()` calls. The SWC optimizer has an entire dedicated pass for this (`props_destructuring.rs`).

6. **`const` vs `let`/`var` affects optimization flags** but NOT capture semantics. Both are captured if used across a `$` boundary, but `const` variables may get additional optimization hints.

7. **Function declarations are hoisted** and may or may not be captured depending on whether they are referenced inside the `$()` body.

8. **TypeScript type-only imports must NOT be treated as captures.** A `type`-imported identifier used in a type annotation inside `$()` is erased at compile time and is not a runtime capture.

**Why it happens:**
OXC's `oxc_semantic` provides the raw scope and symbol data, but it does NOT implement "capture analysis" as a concept. You must build capture analysis on top of `oxc_semantic`. The algorithm is: for each identifier reference inside the `$()` body, resolve its `SymbolId` via semantic info, then check if that symbol is declared in a scope that is an ancestor of the `$()` boundary AND is NOT a module-level import/export. This requires walking the scope tree.

The subtlety is that "module-level import/export" is not just about scope depth. Default imports, namespace imports, re-exports, and `type`-only imports all have different capture semantics.

**Consequences:**
- Missing a capture: runtime `ReferenceError` when the lazy-loaded segment tries to access an undefined variable
- Extra capture: unnecessary serialization, larger bundle, slower hydration
- Wrong capture for loop variables: subtle runtime bugs where all iterations share the same captured value (the classic closure-in-loop bug, but at the module boundary level)

**Prevention:**
Build capture analysis as a standalone, heavily-tested module. Use `oxc_semantic` for ALL scope/symbol resolution. Implement as a function:

```
fn analyze_captures(
    dollar_scope_id: ScopeId,
    body_references: &[ReferenceId],
    scoping: &Scoping,
    module_imports: &HashSet<SymbolId>,
    module_exports: &HashSet<SymbolId>,
) -> CaptureResult
```

Write unit tests for EACH of the 8 edge cases above BEFORE implementing the rest of the optimizer. The spec files `example_multi_capture.md`, `example_capture_imports.md`, `example_functional_component_capture_props.md`, and `example_component_with_event_listeners_inside_loop.md` are the critical test cases.

Use `Scoping::find_binding()` to resolve names up the scope tree. Use `Scoping::get_resolved_references()` to find all uses of a symbol. Use `Scoping::scope_ancestors()` to determine if a symbol's declaring scope is inside or outside the `$()` boundary.

**Detection:**
- Output module has `_captures` array in segment metadata but runtime throws `ReferenceError`
- Segment metadata says `"captures": false` but the extracted code references outer variables
- Loop-related tests produce segments where all iterations share one captured value
- TypeScript-only files pass but `.tsx` files with type imports fail

**Phase to address:** Implement as the FIRST module in the optimizer, with its own test suite, before any code-movement logic.

---

### Pitfall 6: Source Maps for Split Modules Point to Wrong Original Positions

**What goes wrong:**
Each output module's source map must point back to the ORIGINAL source file positions, not to intermediate representations. When the optimizer extracts an expression from line 15 of the input file and places it at line 1 of a new output module, the source map must map output line 1 back to input line 15. Getting this wrong means breakpoints in the debugger land on the wrong line.

The SWC optimizer stores original `Span` values and passes them through to codegen. OXC's `Span` is `{ start: u32, end: u32 }` -- byte offsets into the source text. When you move an AST subtree to a new module, the spans still reference the ORIGINAL source text. But `Codegen` expects spans relative to its own `source_text`. If you pass the original source text to `Codegen::with_source_text()` for each output module, the spans will map correctly. If you pass the output module's generated text, spans will be garbage.

**Why it happens:**
OXC's `Codegen` generates source maps by reading `Span` values from AST nodes and mapping them to the source text provided via `with_source_text()`. The `Codegen` struct has:
- `with_source_text(&str)` -- sets the original source for mapping
- `CodegenOptions { source_map_path: Option<PathBuf> }` -- enables source map generation

`CodegenReturn` contains `map: Option<oxc_sourcemap::SourceMap>` (when `sourcemap` feature is enabled).

The trap: for extracted segments, the AST nodes carry spans from the original file, but the codegen is generating a new file. If you use `CloneIn` to copy nodes into a new allocator, the spans are preserved. If you reconstruct nodes from scratch using `AstBuilder`, you must manually set spans to the original source positions.

**Consequences:**
- Debugger shows wrong file or wrong line for breakpoints in lazy-loaded segments
- "Step into" from the main module into a segment jumps to the wrong location
- Source map validation tools report errors
- Developer experience degrades silently -- code works but debugging is broken

**Prevention:**
For each output module's `Codegen`, always pass the ORIGINAL input source text as `source_text`, not the output module's generated code. This ensures spans in moved nodes map back to original positions.

For nodes constructed from scratch (import declarations, export wrappers), use `Span::default()` or `SPAN` (zero-length span) -- these produce no mapping entries and are expected for generated code.

Use `oxc_sourcemap::SourcemapVisualizer` to verify source maps in tests. This utility prints the mapping between original and generated tokens, making it easy to spot misaligned positions.

For the `ConcatSourceMapBuilder`, use it if you need to compose source maps from multiple generation passes into a single output map.

**Detection:**
- Source map `sources` field contains wrong file paths
- `mappings` field is empty or contains only zero-offset entries
- Debugger shows `(generated)` instead of original file when stepping into segments
- `SourcemapVisualizer` output shows tokens mapped to wrong original positions

**Phase to address:** Implement alongside codegen. Write source map validation tests BEFORE other output tests.

---

### Pitfall 7: Statement Insertion During Traversal Requires Deferred Mutation

**What goes wrong:**
The Qwik optimizer needs to INSERT new statements during traversal: lazy import declarations (`const i_hash = () => import("./segment")`), QRL call wrappers, and replacement variable declarations. In SWC's `Fold`, you return a new `Vec<ModuleItem>` with additional items inserted. In OXC's `Traverse`, you receive `&mut Statement` in `enter_statement`/`exit_statement` -- a single statement, not the containing list. You cannot insert siblings.

Attempting to insert statements by mutating the parent's statement list from inside a child visitor causes a borrow conflict: the traversal holds `&mut` to the child, which borrows the parent's list.

**Why it happens:**
OXC's `Traverse` is designed for in-place mutation of individual nodes, not for structural changes to the tree (inserting/removing siblings). The [issue #4767](https://github.com/oxc-project/oxc/issues/4767) discusses this limitation extensively. The recommended pattern is deferred mutation.

**Consequences:**
- Cannot directly translate SWC's "return Vec with extra items" pattern
- Naive attempts cause borrow checker errors
- Incorrect workarounds (collecting indices, mutating after traversal) miss newly-inserted statements in subsequent passes

**Prevention:**
Use the **deferred statement insertion** pattern:

1. During `enter_*`/`exit_*` callbacks, collect pending insertions in a `HashMap<ScopeId, Vec<Statement<'a>>>` or `Vec<Statement<'a>>` stored on your `Traverse` impl struct.

2. Implement `exit_statements` (plural) on your `Traverse` impl. This callback receives `&mut oxc_allocator::Vec<'a, Statement<'a>>` -- the full statement list. Here you can drain your pending insertions and extend the statement list.

3. For module-level insertions (imports, exports), the callback is `exit_program` where you can modify `program.body`.

```rust
impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn exit_call_expression(&mut self, node: &mut CallExpression<'a>, ctx: &mut TraverseCtx<'a>) {
        if self.is_dollar_call(node) {
            let import_stmt = self.build_lazy_import(node, ctx);
            self.pending_imports.push(import_stmt);
        }
    }

    fn exit_program(&mut self, program: &mut Program<'a>, ctx: &mut TraverseCtx<'a>) {
        // Insert all pending imports at the top of the module
        for stmt in self.pending_imports.drain(..) {
            program.body.insert(0, stmt);
        }
    }
}
```

**Detection:**
- Borrow checker errors in `enter_statement` or `exit_statement` when trying to access the parent statement list
- Missing import declarations in output modules (they were never inserted)
- Import declarations appearing at wrong positions (bottom instead of top)

**Phase to address:** Core transform implementation. Establish the deferred insertion pattern as the standard for ALL statement mutations.

---

## Moderate Pitfalls

---

### Pitfall 8: `#__PURE__` Comment Attachment Is Positional, Not Semantic

**What goes wrong:**
The Qwik optimizer marks `componentQrl()` and `qrl()` calls with `/*#__PURE__*/` annotations so bundlers (Rollup, Vite) can tree-shake unused components. In SWC, comments are stored in a `SingleThreadedComments` map keyed by byte position. In OXC, comments are stored in `program.comments` as a flat list and are associated with nodes by span position during codegen.

When you construct new `CallExpression` nodes using `AstBuilder`, the comments from the original nodes are NOT automatically transferred. If you build a `qrl(...)` call from scratch, it has no `#__PURE__` comment. You must explicitly add the comment.

OXC's `CodegenOptions` has `comments: CommentOptions { annotation: true, ... }`. The `annotation` flag controls whether `#__PURE__` and `#__NO_SIDE_EFFECTS__` annotations are emitted. But this only works if the comments exist in `program.comments`. For NEW nodes created by the optimizer, you must INSERT the comment yourself.

**Why it happens:**
SWC's `comments.add_pure_comment(span.lo)` explicitly attaches a comment at a span position. OXC stores comments on the `Program` node and associates them with AST nodes by position during codegen. Creating a new node at a new span position means no existing comment is associated with it.

**Consequences:**
- Generated code is functionally correct but missing `#__PURE__` annotations
- Bundlers cannot tree-shake unused components
- Bundle sizes silently increase (no error, just larger output)
- Only noticed in production builds where tree-shaking matters

**Prevention:**
After constructing each `qrl()` or `componentQrl()` call expression, add a `Comment` to `program.comments` at the call expression's span position:

```rust
use oxc::ast::Comment;
use oxc::span::Span;

// After building the call expression
let pure_comment = Comment::new(
    span.start,  // Must precede the call expression
    span.start,
    CommentKind::Block,  // /* */ style
);
// The comment text "#__PURE__" must be the raw content
```

Verify with a test that greps for `/*#__PURE__*/` in the codegen output for every `qrl()` and `componentQrl()` call. The 162 spec files document exactly which calls get pure annotations.

**Detection:**
- Output code missing `/*#__PURE__*/` before `qrl()` or `componentQrl()` calls
- Bundle size regression in production builds
- Tree-shaking tools (Rollup plugin `rollup-plugin-visualizer`) show components that should be eliminated

**Phase to address:** Codegen integration. Add pure annotation tests as part of the initial codegen test suite.

---

### Pitfall 9: `CloneIn` Deep-Copies Everything -- Including Stale Semantic IDs

**What goes wrong:**
When using `CloneIn` to clone an AST subtree into a new allocator (for output module generation), the clone copies ALL fields, including `scope_id`, `symbol_id`, and `reference_id` fields on AST nodes. These IDs reference the ORIGINAL file's semantic data. In the new output module, these IDs are meaningless -- there is no corresponding semantic analysis for the output module.

If you run `SemanticBuilder` on the output module, it assigns NEW IDs that conflict with the cloned ones. If you run codegen that queries semantic info (e.g., for mangling or scope-based optimizations), it will read the stale IDs and produce wrong results.

**Why it happens:**
`CloneIn` is a mechanical deep copy. It does not know that semantic IDs are context-dependent. The OXC documentation notes a variant `clone_in_with_semantic` that "for some special types, will also clone the semantic ids" but warns "you should only use this method if you make sure semantic info is synced." For cross-allocator clones (input -> output module), semantic info cannot be synced.

**Consequences:**
- Codegen with mangling produces wrong variable names in output modules
- Semantic queries on cloned nodes return data from the wrong file
- Subtle bugs that only appear when codegen uses scope-aware features

**Prevention:**
After cloning AST subtrees into output module allocators, run `SemanticBuilder` on the output program to generate fresh semantic data. Do NOT rely on cloned semantic IDs.

Alternatively, build output module ASTs from scratch using `AstBuilder` methods rather than cloning. This avoids stale IDs entirely. Use cloning only for complex expression subtrees that would be tedious to reconstruct manually, and always re-run semantic analysis afterward.

For `Codegen`, if you do not need scope-aware features (mangling, renaming), you can skip passing `Scoping` via `with_scoping(None)`. Basic codegen works without semantic info.

**Detection:**
- Unexpected variable renaming in output modules
- Scope-related assertions failing in output module codegen
- Semantic builder panics about "scope already exists" or "duplicate symbol" in output modules

**Phase to address:** Output module generation. Establish whether to use clone-then-rebuild-semantic or build-from-scratch as the standard pattern.

---

### Pitfall 10: OXC's Three Identifier Types vs SWC's One

**What goes wrong:**
SWC has a single `Ident` type for all identifier positions: binding sites, reference sites, and property names. The `Id = (Atom, SyntaxContext)` extraction works uniformly. OXC distinguishes three types:

- `BindingIdentifier` -- where a name is declared (`const x`, `function foo`, `import { x }`)
- `IdentifierReference` -- where a name is used (`console.log(x)`)
- `IdentifierName` -- property names (`obj.prop`, `{ prop: value }`)

Each carries different metadata. `BindingIdentifier` has `symbol_id`. `IdentifierReference` has `reference_id`. `IdentifierName` has neither.

When porting SWC code that extracts `ident.sym` uniformly, you must now match on three different types. Code that builds identifiers must choose the right type for the context.

**Why it happens:**
OXC's richer type system prevents bugs (you cannot accidentally use a binding identifier where a reference is expected), but it means every SWC pattern that extracts or compares identifiers needs three code paths.

**Consequences:**
- Pattern matching on identifier nodes requires matching all three variants
- Building a `VariableDeclarator` requires `BindingIdentifier` for the name, but `IdentifierReference` for the initializer
- Import specifiers use `BindingIdentifier` for the local name and `IdentifierName` for the imported name
- Errors appear as type mismatches in the Rust compiler, not runtime bugs

**Prevention:**
Create utility functions that abstract over the three types for common operations:

```rust
fn ident_name<'a>(binding: &BindingIdentifier<'a>) -> &str { &binding.name }
fn ref_name<'a>(reference: &IdentifierReference<'a>) -> &str { &reference.name }

// For building new identifiers, use AstBuilder methods:
// ctx.ast.binding_identifier(span, name)
// ctx.ast.expression_identifier_reference(span, name)
```

Map each SWC identifier usage site to the correct OXC type during the port. The `AstBuilder` has distinct methods for each: `binding_identifier()`, `expression_identifier_reference()`, `identifier_name()`.

**Detection:**
- Rust type errors about expected `BindingIdentifier` but found `IdentifierReference`
- Using `IdentifierName` where `IdentifierReference` is needed (property access vs variable reference)

**Phase to address:** Initial port scaffolding. Create the identifier utility module early.

---

### Pitfall 11: Hash Compatibility Between SWC and OXC Outputs

**What goes wrong:**
The Qwik runtime resolves lazy-loaded segments by hash values embedded in filenames: `test.tsx_renderHeader_zBbHWn4e8Cg.tsx`. These hashes are computed from the segment's display name, file path, and scope context. If the OXC optimizer produces different hash values for the same input, every QRL reference in the runtime becomes invalid. The app loads but lazy-loaded segments fail with 404 errors because the runtime requests a file that does not exist.

**Why it happens:**
The hash algorithm is `DefaultHasher` from Rust's standard library, which uses SipHash. The hash inputs are strings (display name, file path). If ANY of these strings differ between SWC and OXC output (e.g., different scope name derivation, different display name computation), the hash differs.

The hash is a BASE64-encoded truncation of the SipHash output. Even one bit difference in the input changes the entire hash.

**Consequences:**
- 404 errors for lazy-loaded segments at runtime
- App appears to work (main module loads) but interactive features fail
- Extremely difficult to debug because the hash looks random
- Every deployment is broken, not just edge cases

**Prevention:**
Extract the hash computation into a standalone, independently-testable function. Port the EXACT algorithm from SWC, including:
- The specific `DefaultHasher` usage (SipHash-1-3 or SipHash-2-4 depending on Rust version)
- The exact string inputs (display name format, file path normalization)
- The BASE64 encoding variant (standard vs URL-safe, with or without padding)
- The truncation length (how many characters of the hash are kept)

Test hash compatibility by computing hashes for ALL 162 spec files and comparing against the SWC-produced hashes in the segment metadata.

**Detection:**
- Segment metadata `hash` field differs from SWC snapshots
- Filename in lazy import (`const i_HASH = () => import("./file_HASH")`) differs from spec
- Runtime 404 errors for segment files

**Phase to address:** Early implementation. Port the hash function FIRST and validate against all 162 specs before any other work.

---

## Minor Pitfalls

---

### Pitfall 12: OXC Codegen Formatting Differs from SWC

**What goes wrong:**
OXC's codegen produces different whitespace, semicolons, and formatting compared to SWC's codegen. Arrow functions, import declarations, and object literals may use different spacing. Tests that compare output strings character-by-character will fail even when the code is semantically identical.

**Prevention:**
Use AST-based comparison for semantic equivalence. Parse both expected and actual output with `oxc_parser` and compare the AST structures. Only compare strings for tier-1 semantic properties (hash values, symbol names, import specifiers).

`CodegenOptions` provides `minify: bool`, `single_quote: bool`, `indent_char: IndentChar`, and `indent_width: usize` for controlling output format, but matching SWC's exact formatting is not feasible or necessary.

**Phase to address:** Test harness setup. Build a comparison utility that supports both exact-match and AST-equivalence modes.

---

### Pitfall 13: `generate_uid` / `generate_binding` Name Collisions

**What goes wrong:**
The optimizer creates new identifier names: `i_HASH` for lazy imports, `_captures` for capture arrays, `_rawProps` for destructured props. If these names collide with existing identifiers in the source code, the output has naming conflicts.

**Prevention:**
Use `TraverseCtx::generate_binding(name, scope_id, flags)` instead of manually constructing identifier names. This method checks the scope tree for existing bindings and appends numeric suffixes to avoid collisions. For hash-based names like `i_HASH`, collisions are astronomically unlikely but should still be validated.

**Phase to address:** Identifier generation implementation.

---

### Pitfall 14: TypeScript-Only Nodes Leaking Into Runtime Output

**What goes wrong:**
SWC's `noop_fold_type!()` macro skips TypeScript type nodes during transformation. In OXC, the `Traverse` trait visits ALL nodes including type annotations, `TSTypeAliasDeclaration`, `TSInterfaceDeclaration`, etc. If the optimizer processes these as runtime code (e.g., treating a type import as a value import for capture analysis), the output will contain incorrect imports or captures.

**Prevention:**
In capture analysis, check `SymbolFlags` for the `TypeImport` flag. In statement processing, skip `Statement::TSTypeAliasDeclaration`, `Statement::TSInterfaceDeclaration`, `Statement::TSEnumDeclaration` (when not transpiling enums), and `Statement::TSImportEqualsDeclaration`. The `SourceType::is_typescript()` check determines if type stripping is needed.

When `transpile_ts: true`, type nodes are removed by OXC's TypeScript transform. When `transpile_ts: false`, type nodes remain but must be preserved as-is and excluded from capture analysis.

**Phase to address:** Capture analysis and statement processing.

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Architecture design | Pitfall 1 (lifetime infection), Pitfall 4 (multi-allocator) | Design helper function conventions and allocator strategy before coding |
| Capture analysis | Pitfall 5 (scope edge cases), Pitfall 3 (semantic invalidation) | Build and test capture module independently with all 8 edge cases |
| `$()` extraction | Pitfall 2 (move_expression garbage), Pitfall 7 (statement insertion) | Establish extract-and-replace pattern; use deferred insertion |
| Code generation (output modules) | Pitfall 6 (source maps), Pitfall 9 (stale semantic IDs) | Pass original source text to Codegen; re-run SemanticBuilder on output |
| Codegen integration | Pitfall 8 (pure comments), Pitfall 12 (formatting diffs) | Add pure annotation to program.comments; use AST-based test comparison |
| Hash computation | Pitfall 11 (hash compatibility) | Port exact hash algorithm; validate against all 162 spec hashes |
| Identifier handling | Pitfall 10 (three identifier types), Pitfall 13 (name collisions) | Create abstraction utilities; use generate_binding() |
| TypeScript handling | Pitfall 14 (type nodes in runtime) | Check SymbolFlags; filter type-only declarations |

## OXC-Specific Gotchas Quick Reference

| OXC API | Gotcha | Correct Usage |
|---------|--------|---------------|
| `AstBuilder::move_expression()` | Leaves `NullExpression` in arena (8 bytes each) | Use `std::mem::replace` for atomic swap; clean up dummy nodes |
| `CloneIn::clone_in()` | Copies stale `ScopeId`/`SymbolId` from source | Re-run `SemanticBuilder` on cloned trees; or build from scratch |
| `Traverse` enter/exit | Cannot insert sibling statements | Use deferred insertion in `exit_statements`/`exit_program` |
| `TraverseCtx.ast` | Borrows allocator -- cannot create nodes for different allocator | Use separate `Allocator` per output module |
| `TraverseCtx.scoping_mut()` | Semantic data stale after AST mutation | Run analysis before mutation; use `generate_binding()` for new nodes |
| `Codegen::with_source_text()` | Must be original source for correct source maps | Pass input file source text, not output module text |
| `CodegenOptions.source_map_path` | `None` means no source map generated | Always set for output modules |
| `CodegenOptions.comments.annotation` | Controls `#__PURE__` output but requires comment in `program.comments` | Explicitly add pure comments for new nodes |
| `Program.comments` | Flat list; comments associated by span position | Insert comments at correct span position for new nodes |
| `BindingIdentifier` vs `IdentifierReference` | Wrong type = compile error | Use `ast.binding_identifier()` for declarations, `ast.expression_identifier_reference()` for uses |
| `Scoping::find_binding()` | Walks up scope tree; returns None for unresolved | Check for unresolved references separately |
| `Dummy` trait | Creates placeholder nodes with allocation cost | Prefer `Expression::None` pattern when available |
| `oxc_sourcemap::ConcatSourceMapBuilder` | For combining source maps from multiple passes | Use when composing multi-pass output |

## Sources

- [OXC AstBuilder docs.rs](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) -- Arena-allocated node construction methods (HIGH confidence)
- [OXC Allocator docs.rs](https://docs.rs/oxc_allocator/latest/oxc_allocator/) -- CloneIn trait, Dummy trait, Box/Vec without Drop (HIGH confidence)
- [OXC Traverse docs.rs](https://docs.rs/oxc_traverse/latest/oxc_traverse/) -- TraverseCtx, enter/exit pattern, ancestor access (HIGH confidence)
- [OXC Scoping docs.rs](https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.Scoping.html) -- find_binding, get_resolved_references, scope_ancestors (HIGH confidence)
- [OXC Codegen docs.rs](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) -- CodegenReturn, source map generation (HIGH confidence)
- [OXC Sourcemap docs.rs](https://docs.rs/oxc_sourcemap/latest/oxc_sourcemap/) -- SourceMapBuilder, ConcatSourceMapBuilder, Token (HIGH confidence)
- [GitHub issue #5359: Statement replacement and move_* costs](https://github.com/oxc-project/oxc/issues/5359) -- NullExpression dummy, arena allocation overhead (HIGH confidence)
- [GitHub issue #4767: Statement insertion during traversal](https://github.com/oxc-project/oxc/issues/4767) -- Deferred insertion pattern, exit_statements (HIGH confidence)
- [GitHub discussion #2704: Transformer bottom-up implementation](https://github.com/oxc-project/oxc/discussions/2704) -- Semantic synchronization challenges (HIGH confidence)
- [GitHub issue #1803: Memory leak in arena with Atoms](https://github.com/oxc-project/oxc/issues/1803) -- Arena memory growth concerns (MEDIUM confidence)
- [OXC Transformer Alpha announcement](https://oxc.rs/blog/2024-09-29-transformer-alpha) -- Performance characteristics, 3-5x faster than SWC (MEDIUM confidence)
- [OXC Codegen options source](https://github.com/oxc-project/oxc/blob/main/crates/oxc_codegen/src/options.rs) -- CodegenOptions, CommentOptions struct definitions (HIGH confidence)
- [OXC Dead Code Elimination](https://oxc.rs/docs/guide/usage/minifier/dead-code-elimination) -- Pure annotation semantics (MEDIUM confidence)
- [Source Map Revision 3 Spec](https://sourcemaps.info/spec.html) -- Sections field, VLQ encoding, offset handling (HIGH confidence)
- Spec analysis of 162 files in `.planning/spec/` -- example_multi_capture, example_capture_imports, example_component_with_event_listeners_inside_loop, example_inlined_entry_strategy (HIGH confidence)

---
*Pitfalls research for: Qwik optimizer OXC port implementation*
*Researched: 2026-02-10*
*Supersedes: v1.0 pitfalls (spec generation phase)*
