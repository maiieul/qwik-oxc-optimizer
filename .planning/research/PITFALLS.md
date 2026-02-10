# Pitfalls Research

**Domain:** Compiler transformation spec documentation and SWC-to-OXC port
**Researched:** 2026-02-10
**Confidence:** HIGH (based on codebase analysis, official OXC docs, and compiler engineering literature)

## Critical Pitfalls

### Pitfall 1: Spec Leaks SWC AST Shapes Instead of Capturing Semantics

**What goes wrong:**
The spec describes transformations in terms of SWC-specific AST node types (`ast::CallExpr`, `ast::ArrowExpr`, `SyntaxContext`, etc.) rather than language-level semantic operations. When porting to OXC, implementers discover the spec does not actually tell them what the transformation *does*---it tells them how SWC *implements* it. The entire spec becomes a translation guide for SWC internals, not a behavioral contract.

**Why it happens:**
The existing code IS the only source of truth. When documenting it, the path of least resistance is to describe what the code does at the AST node level. The `Id` type (`(Atom, SyntaxContext)`) is used everywhere in the SWC optimizer---162+ occurrences of `SyntaxContext` across 7 files. SWC's hygiene system (marks, SyntaxContext) has no direct equivalent in OXC, which uses a completely different scope resolution model (ScopeId + SymbolId + ReferenceId via `oxc_semantic`). A spec written in SWC terms is useless for OXC.

**How to avoid:**
Describe transformations at three levels: (1) the **semantic intent** ("identify all captured variables that cross a `$` boundary"), (2) the **input/output contract** (input JS/TS -> expected output JS), (3) **SWC-specific implementation notes** in a clearly separated section. The spec's primary content must be levels 1 and 2. Level 3 is reference material, not specification.

Concretely: every transformation rule must have a "Given this input, produce this output" section using actual JavaScript/TypeScript code, not AST node descriptions.

**Warning signs:**
- Spec text mentions `SyntaxContext`, `Atom`, `Span`, `DUMMY_SP`, or `ast::` types in the core rules
- Transformation descriptions say "match on `ast::CallExpr`" instead of "when a function call matches pattern X"
- Reviewers unfamiliar with SWC cannot understand what the spec describes
- No plain JavaScript examples accompany transformation rules

**Phase to address:**
Spec documentation phase (Milestone 1). This is the foundational risk---if the spec leaks SWC shapes, the entire port phase is building on a broken foundation.

---

### Pitfall 2: SWC `Fold` Ownership Model Implicitly Shapes the Spec's Transformation Order

**What goes wrong:**
SWC's `Fold` trait takes ownership of each node and returns a new one. This ownership-transfer model means transformations happen in a specific bottom-up order as `fold_children_with(self)` recurses before the parent processes the result. The spec inherits this traversal order as an implicit assumption. OXC's `Traverse` trait uses `enter_*`/`exit_*` callbacks with mutable references and a `TraverseCtx`---a fundamentally different control flow. Transformations that rely on SWC's fold-children-first-then-process-parent pattern silently break when ported to enter/exit traversal.

**Why it happens:**
The QwikTransform has 20+ `fold_*` method overrides across 3,800 lines. Each method calls `node.fold_children_with(self)` at different points---sometimes at the start (process children first), sometimes at the end (process parent first), sometimes conditionally. This ordering is the actual algorithm, but it appears as a minor implementation detail rather than a critical specification element.

For example, `fold_call_expr` (line 3101) processes the callee and args in a specific order, with marker function detection happening *before* child folding for some paths and *after* for others. In OXC's enter/exit model, you must explicitly choose whether logic goes in `enter_call_expression` or `exit_call_expression`.

**How to avoid:**
For every `fold_*` method, document when children are processed relative to the parent logic. Explicitly state: "Children must be transformed BEFORE this rule applies" or "This rule must fire BEFORE children are visited." The spec should have a "Traversal Requirements" section for each transformation that states the required ordering constraint, decoupled from the Fold/Traverse mechanism.

**Warning signs:**
- Spec says "transform X" without specifying whether it happens before or after child transformations
- Any transformation that reads `self.segments` or `self.decl_stack` (accumulated from children) but the spec does not document this data dependency
- Tests pass in SWC but fail in OXC with "correct output but wrong nesting" symptoms

**Phase to address:**
Spec documentation phase. Each transformation rule must include an explicit traversal-order annotation.

---

### Pitfall 3: Identifier Resolution Model Mismatch (SyntaxContext vs ScopeId/SymbolId)

**What goes wrong:**
The SWC optimizer identifies variables using `Id = (Atom, SyntaxContext)`. SyntaxContext is a hygiene marker assigned by SWC's resolver pass---two variables with the same name but different scopes get different SyntaxContext values. The entire capture analysis (`collector.rs`, `is_const.rs`, `code_move.rs`) depends on comparing `Id` tuples for equality.

OXC has no SyntaxContext. It uses `oxc_semantic` to build a scope tree with `ScopeId`, `SymbolId`, and `ReferenceId`. Identifiers are resolved by walking up the scope tree. The `Id` comparison pattern (`id_eq!` macro, `HashMap<Id, ...>` lookups) does not translate.

If the port naively maps `SyntaxContext` to something in OXC without understanding the semantic difference, capture analysis will be wrong---variables will be incorrectly identified as captured or not captured, producing broken lazy-loaded segments.

**Why it happens:**
The `id!` macro appears in nearly every file: `($ident.sym.clone(), $ident.ctxt)`. It is the fundamental building block. The `GlobalCollect` struct stores `imports: HashMap<Id, Import>`, `exports: HashMap<Id, Option<Atom>>`, `root: HashMap<Id, Span>`. The entire data model assumes SWC's `Id` type.

OXC's semantic analysis is a separate pass (not integrated into parsing like SWC's resolver). You must run `oxc_semantic::SemanticBuilder` after parsing and then use `SymbolId`/`ReferenceId` for lookups.

**How to avoid:**
The spec must define identifier resolution in abstract terms: "Two identifier references refer to the same binding if and only if they resolve to the same declaration in the nearest enclosing scope." Then provide a mapping appendix: "In SWC, this is represented by `Id = (Atom, SyntaxContext)`. In OXC, this requires `oxc_semantic` scope analysis with `SymbolId` equality."

The spec should define a `ResolvedIdentifier` concept that abstracts over both representations.

**Warning signs:**
- Spec mentions `SyntaxContext` or `Id = (Atom, SyntaxContext)` in core rules
- Port code has hand-rolled scope tracking instead of using `oxc_semantic`
- Tests fail on shadowed variables or variables with the same name in different scopes

**Phase to address:**
Spec documentation phase (define abstract identity model) and early port phase (validate `oxc_semantic` integration with test cases involving shadowed/same-name variables).

---

### Pitfall 4: Capture Analysis Correctness Is the Highest-Risk Transformation

**What goes wrong:**
The Qwik optimizer's core job is extracting closures across `$` boundaries into separate lazy-loadable modules. This requires determining which variables from the outer scope are "captured" by the inner `$` expression. Getting this wrong means: (1) missing captures produce runtime errors (variable undefined), (2) extra captures produce unnecessary serialization and larger bundles, (3) incorrect const analysis produces wrong optimization decisions.

The capture analysis spans `collector.rs` (GlobalCollect, IdentCollector), `is_const.rs` (ConstCollector), and `transform.rs` (decl_stack, scoped_idents). It tracks: imports, exports, root declarations, const-ness, function-ness, class-ness, and scope nesting depth. The `decl_stack: Vec<Vec<IdPlusType>>` is a manual scope stack maintained across fold callbacks.

**Why it happens:**
Capture analysis is the intersection of two hard problems: scope analysis and code extraction. SWC's optimizer maintains its own manual scope stack (`decl_stack`) on top of SWC's hygiene system. This is doubly difficult to port because: (1) you must understand what the manual scope tracking adds beyond SWC's resolver, and (2) you must figure out which parts OXC's semantic analysis handles natively vs. what still needs manual tracking.

**How to avoid:**
The spec must exhaustively enumerate capture rules with test cases:
- Local variable in outer scope captured by `$` inner scope
- Import in outer scope (NOT captured---imports are global)
- Export in outer scope (NOT captured---exports are global)
- Const-qualified variable (affects optimization, not capture)
- Shadowed variable (inner scope rebinds---must NOT capture outer)
- Function declaration hoisting (captures the hoisted binding)
- Loop iteration variables (the optimizer has special handling at lines 2786-2910)
- Destructured parameters (`props_destructuring.rs` has its own pass)

Each rule must have an input/output snapshot pair.

**Warning signs:**
- Spec describes capture analysis as a single rule rather than an enumerated set
- Port code reimplements scope tracking instead of leveraging `oxc_semantic` scope tree
- Test suite has no cases for shadowed variables, loop vars, or destructured captures
- Tests pass for simple cases but fail for `example_multi_capture`, `example_capture_imports`, or `example_functional_component_capture_props`

**Phase to address:**
Spec phase (enumerate all capture rules) and port phase (implement with `oxc_semantic` and validate against the full 162 snapshot suite).

---

### Pitfall 5: Snapshot Tests Define Behavior but Not Intent

**What goes wrong:**
The 162 snapshot tests show input/output pairs but do not explain *why* the output is correct. When a port produces output that differs from the snapshot---perhaps with equivalent but non-identical formatting, different import ordering, or slightly different variable names from hygiene---there is no way to distinguish "this is wrong" from "this is an acceptable equivalent output."

**Why it happens:**
Snapshot tests are characterization tests. They capture the current behavior of the SWC implementation, including incidental details like: whitespace, import ordering, the specific hash algorithm's output, comment placement, and source map offsets. Some of these details are part of the behavioral contract (hashes must match for runtime compatibility); others are SWC artifacts (whitespace, import order).

The snapshot format includes source maps (`Some("{\"version\":3,...}"`), segment metadata JSON, and formatted code. The source maps will necessarily differ between SWC codegen and OXC codegen. The code formatting will differ. The hashes depend on file paths and scope strings, which should be stable, but the hash algorithm itself must be verified.

**How to avoid:**
Classify snapshot assertions into three tiers:
1. **Semantic contract** (MUST match): segment names, hash values, capture flags, import specifiers, exported symbol names, QRL references
2. **Structural equivalence** (must be equivalent but not identical): generated code should parse to the same AST, but whitespace/formatting can differ
3. **Incidental** (may differ): source maps, comment positions, import ordering

The spec should define which aspects of each snapshot are tier 1 vs tier 2 vs tier 3. The port's test harness should compare tier 1 exactly, tier 2 via AST comparison, and skip tier 3.

**Warning signs:**
- Port team debates whether a differing output is "a bug" or "just formatting"
- Tests are updated to match OXC output without verifying semantic equivalence
- Hash values differ and nobody knows if that breaks runtime compatibility
- Source map differences are ignored but cause debugger issues later

**Phase to address:**
Spec documentation phase (classify snapshot tiers) and port phase (build a tiered test comparison harness).

---

### Pitfall 6: OXC Arena Allocator Lifetime Constraints Break SWC Patterns

**What goes wrong:**
SWC AST nodes use standard Rust `Box<T>` and `Vec<T>`. You can freely create, clone, store, and return them. OXC AST nodes use arena-allocated `Box<'a, T>` and `Vec<'a, T>` tied to the arena's lifetime `'a`. You cannot:
- Store OXC AST nodes outside the arena's lifetime
- Clone AST nodes trivially (must allocate new nodes in the arena via `AstBuilder`)
- Return newly-created AST nodes from helper functions without threading the allocator

The SWC optimizer creates AST nodes freely: `Box::new(ast::Expr::...)`, `vec![ast::VarDeclarator{...}]`, `segment.expr.clone()`. Every one of these patterns must change to use `ctx.ast.expression_*()` or `ctx.ast.alloc()` with the arena allocator.

**Why it happens:**
OXC chose arena allocation for performance (3x faster parsing, bulk deallocation). This is a fundamental architectural decision that affects every line of code that creates or manipulates AST nodes. The SWC optimizer has hundreds of AST construction sites across `transform.rs` (3,800 lines), `code_move.rs`, `inlined_fn.rs`, and `collector.rs`.

Additionally, OXC AST nodes with `Drop` implementations cannot be arena-allocated (the arena never calls drop). Types that are trivially `Drop`-able in SWC may need restructuring for OXC.

**How to avoid:**
The spec should NOT prescribe AST construction patterns (those are implementation details). Instead, it should describe what nodes to create semantically: "Create an import declaration for `qrl` from `@qwik.dev/core`". The port phase then implements this using `AstBuilder` methods.

For the port phase specifically:
- Identify all AST construction sites in the SWC code (grep for `Box::new(ast::`, `vec![ast::`, `ast::Module {`, etc.)
- Map each to the equivalent `AstBuilder` method
- Thread `TraverseCtx` (which contains `AstBuilder`) through all helper functions

**Warning signs:**
- Compiler errors about lifetime `'a` not living long enough
- Attempting to store AST nodes in `HashMap` or `Vec` outside the traversal context
- Using `unsafe` to work around lifetime issues instead of restructuring data flow
- Cloning nodes via `node.clone()` instead of `ctx.ast.copy_*()` or `ctx.ast.move_*()` patterns

**Phase to address:**
Port phase. The spec should be lifetime-agnostic. The port architecture must be designed around arena constraints from day one.

---

### Pitfall 7: JSX Transformation Has Hidden Mode Interactions

**What goes wrong:**
The optimizer's JSX handling is not a single transformation---it is a matrix of modes that interact: `EmitMode` (Prod/Lib/Dev/Test) x `transpile_jsx` (bool) x `transpile_ts` (bool) x `is_server` (bool) x `entry_strategy` (Inline/Hoist/Smart/...) x `strip_ctx_name` x `strip_event_handlers`. The spec documents each transformation in isolation, but the interactions between modes produce edge cases.

For example: in Dev mode, QRL references include source location metadata (`qrlDEV` instead of `qrl`). In server mode with `strip_event_handlers`, JSX event handlers are removed. When `entry_strategy` is `Hoist`, segments are inlined as const declarations in the module rather than extracted to separate files. These interact: what happens in Dev mode + Hoist strategy + server strip? The SWC code handles this implicitly through the Fold traversal order and conditional branching.

**Why it happens:**
The `QwikTransformOptions` struct has 14 fields. The `TransformCodeOptions` struct has 18 fields. The `fold_call_expr` alone has branching on `entry_strategy`, `mode`, `is_server`, `strip_event_handlers`, and marker function detection. Combinatorial explosion makes it impractical to test every combination, so edge cases hide in untested mode intersections.

**How to avoid:**
The spec should define a mode matrix that explicitly states which transformations are active in each mode combination. Use a table:

| Transformation | Prod | Dev | Lib | Test | Server | Client |
|---|---|---|---|---|---|---|
| QRL extraction | Yes | Yes (with debug info) | No | Yes | Yes | Yes |
| Event handler stripping | No | No | No | No | If configured | No |
| Side effect removal | Yes | No | No | No | No | Yes |
| etc. | | | | | | |

Then identify which combinations have snapshot coverage and which are gaps.

**Warning signs:**
- Spec describes transformations without stating which modes they apply to
- Port passes all "example_*" tests but fails on "example_dev_mode_inlined" or "example_strip_server_code"
- New mode combinations (OXC may introduce new options) interact unexpectedly with existing logic

**Phase to address:**
Spec documentation phase (build mode matrix) and port phase (ensure mode coverage in test harness).

---

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Copy SWC test snapshots verbatim for OXC | Quick test setup | Locks in SWC-specific output details (formatting, import order) as the contract | Only for tier-1 semantic assertions; tier-2/3 must be adapted |
| Use string comparison for hash values in tests | Simple test assertions | Hash algorithm changes (e.g., different `DefaultHasher` seed) silently break tests without breaking runtime | Only if hash algorithm is explicitly specified in spec |
| Skip `oxc_semantic` and hand-roll scope tracking | Faster initial port; avoids learning OXC semantic API | Incorrect scope resolution on complex code; duplicates work OXC already does; breaks on future OXC updates | Never---always use `oxc_semantic` for scope analysis |
| Implement all transformations in one giant `Traverse` impl | Matches SWC's monolithic `QwikTransform` struct | Cannot compose, test, or disable individual transformations; cannot run subset in different modes | Acceptable for first port; refactor into composable transforms in subsequent milestone |
| Ignore source map generation during port | Reduces scope; codegen handles it | Users cannot debug Qwik apps; DX regression | Acceptable for alpha/internal testing; must ship before beta |

## Integration Gotchas

Common mistakes when connecting to OXC's toolchain.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| `oxc_parser` | Assuming parser output includes scope/symbol info (like SWC after resolver) | Parser ONLY produces AST. Run `SemanticBuilder::new().build(&program)` separately to get scope tree, symbol table, and reference resolution |
| `oxc_codegen` | Expecting output identical to SWC's codegen (swc_ecmascript::codegen) | OXC codegen produces different formatting. Tests must compare AST equivalence or use normalized comparison |
| `oxc_traverse` | Using `enter_*` when `exit_*` is needed (or vice versa) | Map each SWC `fold_*` method: if it calls `fold_children_with` first then processes, use `exit_*`. If it processes first then folds children, use `enter_*` |
| `oxc_allocator` | Creating AST nodes with `Box::new()` or `vec![]` | Use `ctx.ast.alloc()`, `ctx.ast.vec()`, and `AstBuilder` methods. All nodes must be arena-allocated |
| `oxc_transformer` | Assuming OXC's built-in JSX transform produces identical output to SWC's react transform | OXC's JSX transform has different defaults and may handle edge cases differently. Verify with snapshot tests specific to Qwik's `import_source: "@qwik.dev/core"` |
| Pure comments (`#__PURE__`) | Assuming OXC handles pure annotations the same way SWC does via `comments.add_pure_comment(span.lo)` | OXC codegen has its own pure annotation mechanism. Verify that `#__PURE__` annotations appear correctly in codegen output |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Cloning full AST subtrees for segment extraction | Slow transformation on large files | Use `AstBuilder::move_expression` to transfer ownership instead of cloning; OXC's arena makes this efficient | Files with 50+ `$` boundaries or deeply nested components |
| Running `SemanticBuilder` multiple times (once per transform pass) | O(n) overhead per pass on file size | Run semantic analysis once, cache the result, pass `Scoping`/`SymbolTable` to all transforms | Files over 1000 LOC with many scopes |
| String-based hash computation on every segment | Hash collisions or slow hashing on large files | Use the same `DefaultHasher` algorithm as SWC to maintain hash compatibility; precompute file hash once | Projects with 1000+ components |
| Rebuilding `GlobalCollect` equivalent for each file independently | Redundant work in multi-file transforms | Cache import/export metadata across files in `transform_modules` batch mode | Projects with 500+ source files |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Spec completeness:** Spec covers all 20+ `fold_*` methods but is missing the `QwikTransform::new()` initialization logic (lines 164-260) which detects marker functions, JSX functions, and immutable components. This detection logic IS part of the transformation contract.
- [ ] **Capture analysis:** Port handles simple captures but has not been tested with loop iteration variables (`fold_for_stmt`, `fold_for_in_stmt`, `fold_for_of_stmt` all have special iteration variable tracking at lines 2786-2910) or destructured props (`props_destructuring.rs`).
- [ ] **Hash compatibility:** Port produces correct transformed code but hash values differ from SWC. Hashes are embedded in filenames (`test.tsx_renderHeader_zBbHWn4e8Cg.tsx`) and used by the Qwik runtime for lazy loading. Different hashes = broken runtime.
- [ ] **Comment preservation:** `#__PURE__` annotations are added at specific spans. If codegen strips or mispositions them, tree-shaking and dead code elimination in downstream bundlers (Rollup, Vite) breaks silently---output is too large but functionally correct.
- [ ] **Segment module generation:** The `code_move.rs::new_module` function generates complete standalone modules for each extracted segment, including only the imports needed by that segment. Missing an import = runtime error. Extra imports = larger bundles (minor).
- [ ] **Error diagnostics:** SWC uses `HANDLER.with(|handler| handler.struct_span_err_with_code(...))` for user-facing errors. OXC has a different diagnostic system. User-facing error messages must be preserved with equivalent span information.
- [ ] **Entry strategy variations:** Code paths differ for `Inline`, `Hoist`, `Smart`, and `Segment` strategies. Testing only one strategy leaves the others unverified.
- [ ] **`noop_fold_type!()` equivalent:** SWC's `noop_fold_type!()` macro skips TypeScript type nodes during transformation. OXC's traverse must similarly skip type-only nodes. Missing this causes the transformer to process type annotations as runtime code.

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Spec leaks SWC AST shapes | MEDIUM | Add a "Semantic Intent" section to each rule retroactively; use the 162 snapshots as the source of truth for input/output pairs; have a non-SWC-expert review each rule |
| Fold/Traverse ordering mismatch | HIGH | Instrument both SWC and OXC transforms with logging to trace visitation order on a test case; compare traces; fix ordering in OXC Traverse |
| Identifier resolution mismatch | HIGH | Build a "scope dump" utility that prints resolved scopes for both SWC and OXC on the same input; diff results; fix OXC semantic integration |
| Capture analysis errors | HIGH | Run all 162 snapshots; diff the `scoped_idents` / `captures` fields in segment metadata; fix capture rules one category at a time |
| Snapshot test false positives | LOW | Build tiered comparison harness; rewrite tests to compare tier-1 fields exactly and tier-2 via AST equivalence |
| Arena lifetime errors | MEDIUM | Restructure helper functions to accept `&TraverseCtx<'a>` parameter; avoid storing AST nodes in long-lived structs; use `AstBuilder::move_*` patterns |
| Mode interaction bugs | MEDIUM | Build mode matrix test generator that runs each snapshot in all applicable mode combinations; identify gaps; add targeted tests |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Spec leaks SWC AST shapes | Spec Documentation | Non-SWC-expert can read spec and understand transformations; no `ast::` types in core rules |
| Fold/Traverse ordering mismatch | Spec Documentation | Each transformation rule has explicit "traversal requirements" annotation |
| Identifier resolution mismatch | Spec Documentation + Early Port | Abstract `ResolvedIdentifier` concept in spec; `oxc_semantic` integration validated with shadowing tests |
| Capture analysis correctness | Spec Documentation + Port | Exhaustive capture rule enumeration in spec; all 162 snapshots pass with correct `captures`/`scoped_idents` |
| Snapshot tests define behavior not intent | Spec Documentation | Tier classification for all snapshot assertions; tiered comparison harness built |
| Arena lifetime constraints | Port Architecture | Zero `unsafe` blocks for lifetime workarounds; all AST creation via `AstBuilder` |
| JSX mode interactions | Spec Documentation + Port Testing | Mode matrix table in spec; automated mode combination test generator |
| Hash compatibility | Spec Documentation + Port | Hash algorithm explicitly specified in spec; hash values match SWC output for all 162 snapshots |

## Sources

- SWC optimizer codebase analysis: `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/` (18 Rust source files, 162 snapshot tests)
- [OXC AST Design](https://oxc.rs/docs/learn/parser_in_rust/ast) - Arena allocation, enum size control, memory model
- [OXC Traverse Documentation](https://docs.rs/oxc_traverse/latest/oxc_traverse/) - Enter/exit pattern, TraverseCtx, ancestor access
- [OXC Semantic Analysis](https://oxc.rs/docs/learn/parser_in_rust/semantic_analysis) - ScopeId, SymbolId, scope tree
- [OXC AstBuilder](https://docs.rs/oxc_ast/latest/oxc_ast/struct.AstBuilder.html) - Arena-allocated node construction
- [OXC AST Manipulation Challenges](https://github.com/oxc-project/oxc/issues/5359) - Statement replacement, node movement costs
- [OXC Transformer Alpha](https://oxc.rs/blog/2024-09-29-transformer-alpha) - Transform architecture, performance characteristics
- [SWC Fold/VisitMut Discussion](https://github.com/swc-project/swc/discussions/4830) - Fold vs VisitMut tradeoffs
- [SWC Architecture](https://github.com/swc-project/swc/blob/main/ARCHITECTURE.md) - SyntaxContext, resolver, hygiene
- [Snapshot Testing for Compilers](https://www.cs.cornell.edu/~asampson/blog/turnt.html) - Brittleness vs coverage tradeoff
- [Qwik Optimizer Rules](https://qwik.dev/docs/advanced/optimizer/) - `$` boundary rules, capture constraints
- [OXC Codegen](https://docs.rs/oxc/latest/oxc/codegen/struct.Codegen.html) - Code generation, source map support
- [OXC Dead Code Elimination](https://oxc.rs/docs/guide/usage/minifier/dead-code-elimination) - Pure annotation handling
- [OXC Allocator](https://docs.rs/oxc_allocator/latest/oxc_allocator/struct.Box.html) - Arena Box, no-Drop constraint

---
*Pitfalls research for: Qwik optimizer SWC-to-OXC port*
*Researched: 2026-02-10*
