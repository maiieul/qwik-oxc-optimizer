# Multi-Module Output: OXC AstBuilder API Mapping (APIM-06)

**Date:** 2026-02-10
**Purpose:** A standalone, implementable specification for constructing multiple output Program ASTs from a single input file using OXC AstBuilder. A developer reading this document can build POC-03 and POC-04 without consulting any other document. This document maps the SWC optimizer's `code_move.rs` segment construction to OXC's AstBuilder, documents allocator strategies, segment hash computation, codegen pipeline with source maps, and provides a complete end-to-end example.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-06 | Entire document | Multi-module output mapped to AstBuilder Program construction |
| Allocator Strategy | Section 2 | Shared vs separate allocator comparison |
| Hash Algorithm | Section 5 | Segment hash computation (DefaultHasher + base64url) |
| Source Maps | Section 6 | Codegen pipeline with source map generation |

---

## 1. Overview: What Multi-Module Output Does

After the Qwik optimizer detects `$()` call sites and extracts their bodies, it must construct **standalone Program ASTs** for each segment. Each segment is a complete JavaScript module file containing:

1. Import declarations (re-emitted imports from capture analysis, framework imports like `_captures`)
2. Optional lazy import declarations (for nested `$`-calls within the segment)
3. An exported const declaration containing the segment body as a function

The SWC optimizer handles this in `code_move.rs`, which:
- Creates a new SWC `Module` for each segment
- Populates it with the needed imports, the exported body, and any lazy imports
- Runs codegen on each module to produce JavaScript output strings

In OXC, this maps to constructing new `Program<'a>` nodes via `AstBuilder`, then running `Codegen::build()` on each. The key architectural constraint is that `Program<'a>` is tied to its `Allocator`'s lifetime -- all programs must be serialized to strings before the allocator drops.

---

## 2. Allocator Strategy

### Option A: Shared Allocator (Recommended for POC)

Use a single `Allocator` for the main module and all segment modules. All `Program<'a>` instances share the same lifetime `'a`.

```rust
fn transform_and_split(source: &str, filename: &str) -> Vec<(String, String)> {
    let allocator = Allocator::default();

    // Phase 1: Parse input into Program<'a>
    let mut program = parse(&allocator, source, filename);

    // Phase 2: Semantic analysis
    let scoping = run_semantic(&program);

    // Phase 3: Transform main module (replace $() with qrl(), collect segments)
    let segments_data = transform_main(&allocator, &mut program, scoping);

    // Phase 4: Codegen main module (MUST happen before allocator drops)
    let main_code = Codegen::new()
        .with_source_text(source)
        .build(&program)
        .code;

    // Phase 5: Build and codegen each segment
    let mut outputs = vec![("main".into(), main_code)];
    for seg in &segments_data {
        let seg_program = build_segment_program(&allocator, seg);
        let seg_code = Codegen::new().build(&seg_program).code;
        outputs.push((seg.output_path.clone(), seg_code));
    }

    outputs
    // Phase 6: allocator drops here -- all ASTs freed
}
```

**Advantages:**
- Simple lifetime management -- single `'a` for all programs
- No need to move data between allocators
- All codegen happens in one scope

**Disadvantages:**
- Cannot free individual segment programs after codegen (the allocator holds all memory until dropped)
- Cannot parallelize codegen across segments (all share one allocator, which is not `Sync`)
- Memory usage grows with number of segments (all programs in memory simultaneously)

### Option B: Separate Allocators Per Segment

Create a new `Allocator` for each segment. Each `Program` has an independent lifetime.

```rust
fn transform_and_split(source: &str, filename: &str) -> Vec<(String, String)> {
    let main_allocator = Allocator::default();

    // Parse and transform main module
    let mut program = parse(&main_allocator, source, filename);
    let scoping = run_semantic(&program);
    let segments_data = transform_main(&main_allocator, &mut program, scoping);
    let main_code = Codegen::new()
        .with_source_text(source)
        .build(&program)
        .code;

    // Build each segment with its own allocator
    let mut outputs = vec![("main".into(), main_code)];
    for seg in &segments_data {
        let seg_allocator = Allocator::default();
        let seg_program = build_segment_program(&seg_allocator, seg);
        let seg_code = Codegen::new().build(&seg_program).code;
        outputs.push((seg.output_path.clone(), seg_code));
        // seg_allocator drops here -- segment AST freed immediately
    }

    outputs
}
```

**Advantages:**
- Memory freed after each segment's codegen
- Enables parallel codegen via rayon (each allocator is independent)
- Better memory characteristics for files with many segments

**Disadvantages:**
- Cannot share AST nodes between main and segment programs (different allocators)
- Must serialize/copy segment body data rather than moving nodes between allocators
- More complex lifetime management

### Recommendation

**Use shared allocator for POC.** The POC processes one file at a time with a small number of segments. Memory is not a concern. The simplicity of a single lifetime scope avoids the complexity of cross-allocator data transfer.

**For production:** The ARCHITECTURE-BLUEPRINT.md data flow specification recommends that each `transform_single_module()` call owns one allocator. Within that call, the shared approach works well. Parallel codegen across segments can be added later by switching to separate allocators per segment, using rayon.

---

## 3. Segment Program Structure

Each extracted segment is a standalone JavaScript module. The structure varies by what the segment needs, but follows this general pattern:

### 3.1 Segment with No Dependencies

The simplest case: no imports, no captures, no nested `$`-calls.

```javascript
// From example_1.md: renderHeader_div_onClick_fV2uzAL99u4.tsx
export const renderHeader_div_onClick_fV2uzAL99u4 = (ctx)=>console.log(ctx);
```

Program structure:
```
Program {
    body: [
        ExportNamedDeclaration {
            declaration: VariableDeclaration {
                kind: Const,
                declarations: [VariableDeclarator {
                    id: "renderHeader_div_onClick_fV2uzAL99u4",
                    init: ArrowFunctionExpression {
                        params: ["ctx"],
                        body: console.log(ctx)  // original extracted body
                    }
                }]
            }
        }
    ]
}
```

### 3.2 Segment with Framework Imports

When the segment body calls framework functions (e.g., `useStore`) or has nested `$`-calls that produce `qrl()` wrappers.

```javascript
// From example_functional_component.md: Header_component_J4uyIhaBNR4.tsx
import { useStore } from "@qwik.dev/core";
export const Header_component_J4uyIhaBNR4 = ()=>{
    const thing = useStore();
    const { foo, bar } = foo();
    return <div>{thing}</div>;
};
```

Program structure:
```
Program {
    body: [
        ImportDeclaration { specifiers: ["useStore"], source: "@qwik.dev/core" },
        ExportNamedDeclaration {
            declaration: VariableDeclaration { ... }
        }
    ]
}
```

### 3.3 Segment with Captures

When the segment captures variables from an outer scope.

```jsx
// From example_multi_capture.md: Foo_component_1_DvU6FitWglY.jsx
import { _captures } from "@qwik.dev/core";
export const Foo_component_1_DvU6FitWglY = ()=>{
    const _rawProps = _captures[0];
    const fn = ({ aaa })=>aaa;
    return <div>
                {_rawProps.foo}{fn()}{20}
            </div>;
};
```

Program structure:
```
Program {
    body: [
        ImportDeclaration { specifiers: ["_captures"], source: "@qwik.dev/core" },
        ExportNamedDeclaration {
            declaration: VariableDeclaration {
                init: ArrowFunctionExpression {
                    body: FunctionBody {
                        statements: [
                            // Injected: const _rawProps = _captures[0];
                            VariableDeclaration { ... },
                            // Original body statements
                            ...
                        ]
                    }
                }
            }
        }
    ]
}
```

### 3.4 Segment with Nested $-calls (Lazy Imports)

When the segment body contains its own `$`-calls that need lazy imports.

```javascript
// From example_1.md: renderHeader_zBbHWn4e8Cg.tsx
import { qrl } from "@qwik.dev/core";
const i_fV2uzAL99u4 = ()=>import("./test.tsx_renderHeader_div_onClick_fV2uzAL99u4");
export const renderHeader_zBbHWn4e8Cg = ()=>{
    return <div onClick={/*#__PURE__*/ qrl(i_fV2uzAL99u4, "renderHeader_div_onClick_fV2uzAL99u4")}/>;
};
```

Program structure:
```
Program {
    body: [
        ImportDeclaration { specifiers: ["qrl"], source: "@qwik.dev/core" },
        VariableDeclaration {  // lazy import
            const i_fV2uzAL99u4 = () => import("./...")
        },
        ExportNamedDeclaration { ... }
    ]
}
```

### 3.5 Segment with User Imports (CSS, Sibling Modules)

When the segment references user-code imports (not `@qwik.dev/core`).

```javascript
// From example_capture_imports.md: App_component_useStyles_t35nSa5UV7U.js
import css1 from "./global.css";
import css2 from "./style.css";
export const App_component_useStyles_t35nSa5UV7U = `${css1}${css2}`;
```

Import classification for segment construction:

| Import Type | Example | Segment Treatment |
|-------------|---------|-------------------|
| Framework named | `import { useStore } from "@qwik.dev/core"` | Re-emit if body references it |
| Framework special | `import { _captures } from "@qwik.dev/core"` | Add if segment has captures |
| Framework QRL | `import { qrl } from "@qwik.dev/core"` | Add if segment has nested $-calls |
| User named | `import { thing } from "./sibling"` | Re-emit if body references it |
| User default | `import css1 from "./global.css"` | Re-emit if body references it |
| User default | `import mongodb from "mongodb"` | Re-emit if body references it |

---

## 4. AstBuilder Construction API

### 4.1 build_named_import() -- Construct Named Import

Build `import { name } from "source"`:

```rust
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast::AstBuilder;
use oxc_span::SPAN;

/// Build: import { name } from "source"
fn build_named_import<'a>(
    name: &str,
    source: &str,
    ast: &AstBuilder<'a>,
) -> Statement<'a> {
    // Build the local binding identifier
    let local = ast.binding_identifier(SPAN, ast.atom(name));

    // Build the imported name (same as local for non-aliased)
    let imported = ast.module_export_name_identifier_name(SPAN, ast.atom(name));

    // Build the import specifier: { name }
    let specifier = ast.import_specifier(
        SPAN,
        imported,
        local,
        ImportOrExportKind::Value,
    );

    // Wrap in specifiers vec
    let specifiers = ast.vec1(
        ImportDeclarationSpecifier::ImportSpecifier(specifier),
    );

    // Build the source string literal
    let source_lit = ast.string_literal(SPAN, ast.atom(source), None);

    // Build the import declaration
    let import_decl = ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        NONE,                      // no import attributes
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}
```

### 4.2 build_default_import() -- Construct Default Import

Build `import name from "source"`:

```rust
/// Build: import name from "source"
fn build_default_import<'a>(
    name: &str,
    source: &str,
    ast: &AstBuilder<'a>,
) -> Statement<'a> {
    let local = ast.binding_identifier(SPAN, ast.atom(name));

    let specifier = ast.import_default_specifier(SPAN, local);
    let specifiers = ast.vec1(
        ImportDeclarationSpecifier::ImportDefaultSpecifier(specifier),
    );

    let source_lit = ast.string_literal(SPAN, ast.atom(source), None);

    let import_decl = ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        NONE,
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}
```

### 4.3 build_lazy_import_declaration() -- Construct Lazy Import

Build `const i_hash = () => import("./path")`:

```rust
/// Build: const i_{hash} = () => import("./path_segment_hash")
fn build_lazy_import_declaration<'a>(
    hash: &str,
    import_path: &str,
    ast: &AstBuilder<'a>,
) -> Statement<'a> {
    let ident_name = format!("i_{}", hash);

    // Build: import("./path")
    let import_source = ast.expression_string_literal(
        SPAN,
        ast.atom(import_path),
        None,
    );
    let import_expr = ast.expression_import(
        SPAN,
        import_source,
        ast.vec(),  // no import attributes
        None,       // no phase
    );

    // Build arrow: () => import(...)
    let params = ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ast.vec(),
        None,
    );

    let expr_stmt = ast.statement_expression(SPAN, import_expr);
    let body = ast.function_body(
        SPAN,
        ast.vec(),
        ast.vec1(expr_stmt),
    );

    let arrow = ast.expression_arrow_function(
        SPAN,
        true,   // expression body
        false,  // not async
        false,  // not generator
        NONE,   // no type params
        params,
        NONE,   // no return type
        body,
    );

    // Build: const i_hash = () => import(...)
    let binding = ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ast.atom(&ident_name),
    );
    let pattern = ast.binding_pattern(binding, None, false);
    let declarator = ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        pattern,
        Some(arrow),
        false,
    );
    let declaration = ast.variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(declarator),
        false,
    );

    Statement::from(Declaration::VariableDeclaration(
        ast.alloc(declaration),
    ))
}
```

### 4.4 build_segment_export() -- Construct Exported Segment Function

Build `export const name_hash = (params) => { body }`:

```rust
/// Build: export const segment_name = (params) => { body }
///
/// For segment strategy, the body is the extracted arrow function expression.
/// The params come from the original $() argument's parameters.
fn build_segment_export<'a>(
    export_name: &str,
    body_fn: Expression<'a>,
    ast: &AstBuilder<'a>,
) -> Statement<'a> {
    // Build the binding: segment_name
    let binding = ast.binding_pattern_kind_binding_identifier(
        SPAN,
        ast.atom(export_name),
    );
    let pattern = ast.binding_pattern(binding, None, false);

    // Build the variable declarator: segment_name = (params) => { body }
    let declarator = ast.variable_declarator(
        SPAN,
        VariableDeclarationKind::Const,
        pattern,
        Some(body_fn),
        false,
    );

    // Build the variable declaration: const segment_name = ...
    let declaration = ast.variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ast.vec1(declarator),
        false,
    );

    // Wrap in export: export const segment_name = ...
    let export = ast.module_declaration_export_named_declaration(
        SPAN,
        Some(Declaration::VariableDeclaration(ast.alloc(declaration))),
        ast.vec(),  // no specifiers (using declaration form)
        None,       // no source
        ImportOrExportKind::Value,
        None,       // no attributes
    );

    Statement::from(export)
}
```

### 4.5 build_segment_program() -- Assemble Complete Segment

Build the complete `Program` for a segment module:

```rust
use oxc_span::SourceType;

/// Data needed to construct a segment Program.
pub struct SegmentBuildData<'a> {
    /// Segment export name (e.g., "renderHeader_zBbHWn4e8Cg")
    pub export_name: String,

    /// The extracted function body as an expression
    pub body_expression: Expression<'a>,

    /// Named imports the segment needs from @qwik.dev/core
    /// e.g., ["useStore", "qrl"]
    pub framework_imports: Vec<String>,

    /// Whether the segment needs _captures import
    pub needs_captures: bool,

    /// User imports to re-emit
    /// Each entry: (local_name, source, is_default)
    pub user_imports: Vec<(String, String, bool)>,

    /// Lazy imports for nested $-calls
    /// Each entry: (hash, import_path)
    pub lazy_imports: Vec<(String, String)>,
}

/// Build a complete segment Program from the provided data.
///
/// The resulting Program has this structure:
///   1. Framework imports (import { X } from "@qwik.dev/core")
///   2. _captures import (if needed)
///   3. User imports (import X from "./source")
///   4. Lazy import declarations (const i_hash = () => import(...))
///   5. Export declaration (export const name = (params) => { body })
pub fn build_segment_program<'a>(
    allocator: &'a Allocator,
    segment: &SegmentBuildData<'a>,
) -> Program<'a> {
    let ast = AstBuilder::new(allocator);
    let mut body = ast.vec();

    // 1. Add framework imports
    for import_name in &segment.framework_imports {
        let stmt = build_named_import(import_name, "@qwik.dev/core", &ast);
        body.push(stmt);
    }

    // 2. Add _captures import if segment has captures
    if segment.needs_captures {
        let captures_stmt = build_named_import("_captures", "@qwik.dev/core", &ast);
        body.push(captures_stmt);
    }

    // 3. Add user imports
    for (local_name, source, is_default) in &segment.user_imports {
        let stmt = if *is_default {
            build_default_import(local_name, source, &ast)
        } else {
            build_named_import(local_name, source, &ast)
        };
        body.push(stmt);
    }

    // 4. Add lazy imports for nested $-calls
    for (hash, import_path) in &segment.lazy_imports {
        let stmt = build_lazy_import_declaration(hash, import_path, &ast);
        body.push(stmt);
    }

    // 5. Add the exported segment function
    // Note: body_expression must be moved here. In practice, use take_in()
    // during the transform pass to extract the body from the original AST,
    // preserving original Spans for source map fidelity.
    let export_stmt = build_segment_export(
        &segment.export_name,
        // In actual implementation, this is the extracted arrow function.
        // For the POC, we construct a placeholder or pass it from transform.
        segment.body_expression.clone_in(allocator),
        &ast,
    );
    body.push(export_stmt);

    // 6. Construct the Program
    ast.program(
        SPAN,
        SourceType::mjs(),  // segments are always JS modules
        None,               // no hashbang
        ast.vec(),          // no directives
        body,
    )
}
```

### 4.6 AstBuilder Method Reference

Complete list of `AstBuilder` methods used in segment construction:

| Method | Purpose | Used In |
|--------|---------|---------|
| `ast.vec()` | Create empty arena-allocated Vec | All builders |
| `ast.vec1(item)` | Create Vec with one element | Specifiers, declarations |
| `ast.vec_with_capacity(n)` | Create Vec with pre-allocated capacity | When adding multiple items |
| `ast.atom(s)` | Create interned string atom | All identifier/string construction |
| `ast.alloc(val)` | Box a value in the arena | Declaration wrapping |
| `ast.binding_identifier(span, name)` | Create a BindingIdentifier | Import locals, export bindings |
| `ast.binding_pattern(kind, annotation, optional)` | Create a BindingPattern | Variable declarators |
| `ast.binding_pattern_kind_binding_identifier(span, name)` | Create BindingPatternKind | Variable declarators |
| `ast.variable_declarator(span, kind, pattern, init, definite)` | Create a VariableDeclarator | All variable declarations |
| `ast.variable_declaration(span, kind, declarators, declare)` | Create a VariableDeclaration | All variable declarations |
| `ast.formal_parameters(span, kind, params, rest)` | Create FormalParameters | Arrow function params |
| `ast.function_body(span, directives, statements)` | Create FunctionBody | Arrow function bodies |
| `ast.expression_arrow_function(span, expr, async, gen, tp, params, rt, body)` | Create ArrowFunctionExpression | Lazy imports, segment bodies |
| `ast.expression_string_literal(span, value, raw)` | Create a StringLiteral expression | Import paths, segment names |
| `ast.expression_identifier_reference(span, name)` | Create an IdentifierReference | Any identifier usage |
| `ast.expression_import(span, source, attributes, phase)` | Create ImportExpression | Dynamic import() |
| `ast.statement_expression(span, expr)` | Create ExpressionStatement | Expression body arrows |
| `ast.string_literal(span, value, raw)` | Create a StringLiteral node | Import source values |
| `ast.import_specifier(span, imported, local, kind)` | Create ImportSpecifier | Named import specifiers |
| `ast.import_default_specifier(span, local)` | Create ImportDefaultSpecifier | Default import specifiers |
| `ast.module_export_name_identifier_name(span, name)` | Create ModuleExportName | Imported name in specifier |
| `ast.module_declaration_import_declaration(span, specifiers, source, attributes, kind)` | Create ImportDeclaration | All imports |
| `ast.module_declaration_export_named_declaration(span, decl, specs, source, kind, attrs)` | Create ExportNamedDeclaration | Segment exports |
| `ast.program(span, source_type, hashbang, directives, body)` | Create Program | Final segment assembly |

---

## 5. Segment Hash Computation

The segment hash is an 11-character string that uniquely identifies each extracted segment. It appears in segment names (e.g., `zBbHWn4e8Cg` in `renderHeader_zBbHWn4e8Cg`) and must be computed identically to the SWC optimizer for spec file compatibility.

### Algorithm

The SWC optimizer uses Rust's `DefaultHasher` (SipHash-2-4) combined with base64url encoding:

```rust
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use base64::Engine;

/// Compute the 11-character hash for a segment name.
///
/// Arguments:
/// - `scope`: Optional scope prefix (e.g., package name for monorepo)
/// - `rel_path`: Relative file path using forward slashes (e.g., "test.tsx")
/// - `display_name`: Display name for the segment (e.g., "renderHeader")
///
/// Returns an 11-character base64url-encoded hash string.
///
/// The hash is deterministic: same inputs always produce the same output.
/// A single byte difference in any input produces a completely different hash.
pub fn compute_segment_hash(
    scope: Option<&str>,
    rel_path: &str,
    display_name: &str,
) -> String {
    let mut hasher = DefaultHasher::new();

    // Hash inputs in order: scope (optional), rel_path, display_name
    if let Some(scope) = scope {
        hasher.write(scope.as_bytes());
    }
    hasher.write(rel_path.as_bytes());
    hasher.write(display_name.as_bytes());

    let hash = hasher.finish(); // u64 (8 bytes)

    // Convert u64 to 8 bytes in little-endian order
    let bytes = hash.to_le_bytes();

    // Encode with base64url (no padding) -> 11 characters
    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(bytes);

    // Replace '-' and '_' with '0' for URL safety
    encoded.replace(['-', '_'], "0")
}

/// Format the full segment name: "{display_name}_{hash}"
pub fn format_segment_name(display_name: &str, hash: &str) -> String {
    format!("{}_{}", display_name, hash)
}
```

### Hash Inputs

The hash inputs must match exactly what the SWC optimizer uses. The three inputs are:

| Input | Source | Example |
|-------|--------|---------|
| `scope` | `TransformModulesOptions.scope` | `None` (default) or `Some("@my-org")` |
| `rel_path` | Relative path from src_dir, **forward slashes** | `"test.tsx"` |
| `display_name` | Derived from lexical context of the `$`-call | `"test.tsx_renderHeader"` |

### Display Name Derivation

The display name is derived from the file path and the lexical context where the `$()` call appears:

| Pattern | Display Name | Example |
|---------|-------------|---------|
| `export const X = $(() => ...)` | `"{file}_X"` | `"test.tsx_renderHeader"` |
| `const X = component$(() => ...)` | `"{file}_X_component"` | `"test.tsx_Header_component"` |
| `useBrowserVisibleTask$(() => ...)` inside component X | `"{file}_X_component_useBrowserVisibleTask"` | `"test.tsx_Child_component_useBrowserVisibleTask"` |
| Nested `$()` inside segment N | `"{file}_X_component_1"` | `"test.tsx_Foo_component_1"` |
| `$()` in onClick attribute | `"{file}_X_div_onClick"` | `"test.tsx_renderHeader_div_onClick"` |

### Verification Against Spec Files

The hash must produce identical results to the SWC optimizer. Verify against known spec file hashes:

| Spec File | Display Name | Expected Hash |
|-----------|-------------|---------------|
| `example_1.md` | `test.tsx_renderHeader` | `zBbHWn4e8Cg` |
| `example_1.md` | `test.tsx_renderHeader_component` | `U6Kkv07sbpQ` |
| `example_1.md` | `test.tsx_renderHeader_div_onClick` | `fV2uzAL99u4` |
| `example_multi_capture.md` | `test.tsx_Foo_component` | `HTDRsvUbLiE` |
| `example_multi_capture.md` | `test.tsx_Bar_component` | `L80pS8Hxf1Y` |
| `example_multi_capture.md` | `test.tsx_Foo_component_1` | `DvU6FitWglY` |
| `example_capture_imports.md` | `test.tsx_App_component` | `ckEPmXZlub0` |
| `example_inlined_entry_strategy.md` | `test.tsx_Child_component` | `9GyF01GDKqw` |

### Critical: Path Normalization

The `rel_path` must use **forward slashes** regardless of the operating system. The SWC optimizer uses `to_slash_lossy()` to normalize paths before hashing. On Windows, `test.tsx` would still be `test.tsx`, but subdirectory paths like `components\Header.tsx` must become `components/Header.tsx`.

```rust
// Use the path-slash crate for normalization:
use path_slash::PathExt;

let rel_path = std::path::Path::new(filename)
    .to_slash_lossy()
    .to_string();
```

---

## 6. Codegen Pipeline

### 6.1 Main Module Codegen

For the main (transformed) module, use `Codegen` with source text for source map support:

```rust
use oxc_codegen::{Codegen, CodegenReturn};

/// Generate JavaScript from the transformed main module Program.
fn codegen_main<'a>(
    program: &Program<'a>,
    source_text: &str,
) -> CodegenReturn {
    Codegen::new()
        .with_source_text(source_text)
        .build(program)
}
```

The `CodegenReturn` contains:
- `code: String` -- the generated JavaScript
- `map: Option<SourceMap>` -- source map (only present when `source_map_path` is set in options)

### 6.2 Segment Codegen with Source Maps

For segment modules, enable source maps by providing `source_map_path` in `CodegenOptions`:

```rust
use oxc_codegen::{Codegen, CodegenOptions, CodegenReturn};
use std::path::PathBuf;

/// Generate JavaScript with source map for a segment Program.
///
/// The source_text is the ORIGINAL source file's content (not the segment's).
/// This is because the segment's AST nodes carry Span values that reference
/// byte offsets in the original source.
fn codegen_segment_with_sourcemap<'a>(
    program: &Program<'a>,
    original_source_text: &str,
    output_filename: &str,
) -> CodegenReturn {
    let options = CodegenOptions {
        source_map_path: Some(PathBuf::from(output_filename)),
        ..CodegenOptions::default()
    };

    Codegen::new()
        .with_options(options)
        .with_source_text(original_source_text)
        .build(program)
}
```

### 6.3 Source Map Serialization

The `SourceMap` from `CodegenReturn` can be serialized to JSON:

```rust
let result = codegen_segment_with_sourcemap(&program, source, "segment.js");

if let Some(source_map) = result.map {
    // Full JSON source map string
    let json_string: String = source_map.to_json_string();

    // Or structured JSON for further processing
    let json_value: oxc_sourcemap::JSONSourceMap = source_map.to_json();
}
```

### 6.4 Span Preservation Rules

Source map accuracy depends on preserving original `Span` values on extracted AST nodes:

| Node Origin | Span Value | Source Map Effect |
|-------------|------------|-------------------|
| Extracted from original AST (body, params) | Original span (non-zero) | Mapped to original source position |
| Constructed by builder (imports, export wrapper) | `SPAN` (zero span) | No source map entry |
| Constructed identifier references | `SPAN` | No source map entry |

**How this works in practice:**

1. When the transform pass extracts a `$()` body using `take_in(ctx.ast)`, the moved nodes retain their original `Span` values. These spans reference byte offsets in the original source file.

2. When `build_segment_program()` constructs new nodes (import declarations, export wrapper, lazy imports), it uses `SPAN` (the zero-span constant) because these nodes have no position in the original source.

3. When `Codegen` runs on the segment `Program`, it reads each node's span. For nodes with non-zero spans, it creates source map entries mapping the generated position to the original position. For nodes with `SPAN`, no mapping is created.

4. The `with_source_text(original_source_text)` call provides the original source content, which the source map needs to accurately resolve line/column positions from byte offsets.

**Consequence:** The extracted body code in the segment will have accurate source map mappings back to the original file. The constructed import/export wrapper code will not have source map entries, which is correct -- they don't exist in the original source.

### 6.5 Codegen Without Source Maps

For simple codegen without source maps (e.g., during POC testing):

```rust
fn codegen_simple<'a>(program: &Program<'a>) -> String {
    Codegen::new().build(program).code
}
```

---

## 7. End-to-End Example: example_1.md

This section traces the complete flow from input source to final output for `example_1.md`, showing exactly how multi-module output is constructed.

### Step 1: Input

From `example_1.md`, the input source:

```tsx
import { $, component, onRender } from '@qwik.dev/core';

export const renderHeader = $(() => {
    return (
        <div onClick={$((ctx) => console.log(ctx))}/>
    );
});
const renderHeader = component($(() => {
    console.log("mount");
    return render;
}));
```

### Step 2: Parse

```rust
let allocator = Allocator::default();
let source_type = SourceType::tsx();
let ret = Parser::new(&allocator, source, source_type).parse();
let mut program = ret.program;
```

The parser produces a `Program` with 3 top-level statements:
1. `ImportDeclaration` -- `import { $, component, onRender } from '@qwik.dev/core'`
2. `ExportNamedDeclaration` -- `export const renderHeader = $(() => { ... })`
3. `VariableDeclaration` -- `const renderHeader = component($(() => { ... }))`

### Step 3: Detect $() Sites and Extract

The transform pass detects 3 `$()` call sites:

| # | Location | Display Name | Hash |
|---|----------|-------------|------|
| 1 | `export const renderHeader = $(...)` | `test.tsx_renderHeader` | `zBbHWn4e8Cg` |
| 2 | `$((ctx) => console.log(ctx))` inside #1 | `test.tsx_renderHeader_div_onClick` | `fV2uzAL99u4` |
| 3 | `component($(...))`| `test.tsx_renderHeader_component` | `U6Kkv07sbpQ` |

For each, the transform:
- Extracts the arrow function body (using `take_in`)
- Records segment metadata (hash, captures, needed imports)
- Replaces the `$()` call with `qrl(i_hash, "name_hash")`

### Step 4: Build Segment Programs

**Segment 1: `renderHeader_zBbHWn4e8Cg`**

This segment has a nested `$()` call (the onClick handler), so it needs:
- `import { qrl } from "@qwik.dev/core"` (for the nested `$()` -> `qrl()` replacement)
- `const i_fV2uzAL99u4 = () => import("./test.tsx_renderHeader_div_onClick_fV2uzAL99u4")` (lazy import for nested segment)
- The extracted body: `() => { return <div onClick={qrl(i_fV2uzAL99u4, "...")}/>; }`

```rust
let segment1 = SegmentBuildData {
    export_name: "renderHeader_zBbHWn4e8Cg".to_string(),
    body_expression: extracted_body_1,  // the arrow function
    framework_imports: vec!["qrl".to_string()],
    needs_captures: false,
    user_imports: vec![],
    lazy_imports: vec![
        ("fV2uzAL99u4".into(), "./test.tsx_renderHeader_div_onClick_fV2uzAL99u4".into()),
    ],
};

let program1 = build_segment_program(&allocator, &segment1);
```

Expected output:
```tsx
import { qrl } from "@qwik.dev/core";
const i_fV2uzAL99u4 = ()=>import("./test.tsx_renderHeader_div_onClick_fV2uzAL99u4");
export const renderHeader_zBbHWn4e8Cg = ()=>{
    return <div onClick={/*#__PURE__*/ qrl(i_fV2uzAL99u4, "renderHeader_div_onClick_fV2uzAL99u4")}/>;
};
```

**Segment 2: `renderHeader_div_onClick_fV2uzAL99u4`**

This segment has no dependencies -- just the extracted body:

```rust
let segment2 = SegmentBuildData {
    export_name: "renderHeader_div_onClick_fV2uzAL99u4".to_string(),
    body_expression: extracted_body_2,  // (ctx) => console.log(ctx)
    framework_imports: vec![],
    needs_captures: false,
    user_imports: vec![],
    lazy_imports: vec![],
};

let program2 = build_segment_program(&allocator, &segment2);
```

Expected output:
```tsx
export const renderHeader_div_onClick_fV2uzAL99u4 = (ctx)=>console.log(ctx);
```

**Segment 3: `renderHeader_component_U6Kkv07sbpQ`**

This segment has no dependencies:

```rust
let segment3 = SegmentBuildData {
    export_name: "renderHeader_component_U6Kkv07sbpQ".to_string(),
    body_expression: extracted_body_3,  // () => { console.log("mount"); return render; }
    framework_imports: vec![],
    needs_captures: false,
    user_imports: vec![],
    lazy_imports: vec![],
};

let program3 = build_segment_program(&allocator, &segment3);
```

Expected output:
```tsx
export const renderHeader_component_U6Kkv07sbpQ = ()=>{
    console.log("mount");
    return render;
};
```

### Step 5: Codegen All Programs

```rust
// Codegen main module
let main_result = Codegen::new()
    .with_source_text(source)
    .build(&program);

// Codegen segments
let seg1_result = Codegen::new().build(&program1);
let seg2_result = Codegen::new().build(&program2);
let seg3_result = Codegen::new().build(&program3);

// Collect all outputs
let outputs = vec![
    ("test.tsx".to_string(), main_result.code),
    ("test.tsx_renderHeader_zBbHWn4e8Cg.tsx".to_string(), seg1_result.code),
    ("test.tsx_renderHeader_div_onClick_fV2uzAL99u4.tsx".to_string(), seg2_result.code),
    ("test.tsx_renderHeader_component_U6Kkv07sbpQ.tsx".to_string(), seg3_result.code),
];
```

### Step 6: Main Module Output

From the spec file, the expected main module output:

```tsx
import { qrl } from "@qwik.dev/core";
const i_U6Kkv07sbpQ = ()=>import("./test.tsx_renderHeader_component_U6Kkv07sbpQ");
const i_zBbHWn4e8Cg = ()=>import("./test.tsx_renderHeader_zBbHWn4e8Cg");
import { component } from '@qwik.dev/core';
export const renderHeader = /*#__PURE__*/ qrl(i_zBbHWn4e8Cg, "renderHeader_zBbHWn4e8Cg");
const renderHeader = component(/*#__PURE__*/ qrl(i_U6Kkv07sbpQ, "renderHeader_component_U6Kkv07sbpQ"));
```

Key observations:
- The main module has `qrl` import (added during transform)
- Lazy import declarations for segments
- Original `import { component } from '@qwik.dev/core'` kept (non-dollar import)
- `$()` calls replaced with `qrl()` wrappers
- Three separate segment files produced

---

## 8. Integration with Architecture

This document covers stages 6 (extract) and 7 (emit) from the ARCHITECTURE-BLUEPRINT.md data flow specification:

### Stage 6: Extract Segments (code_move.rs)

```
Input:  &Allocator, &[SegmentData], filename, &TransformOptions
Output: Vec<SegmentProgram { program, path, analysis }>

For each SegmentData:
  1. Classify imports (framework vs user, see import classification table)
  2. Determine if _captures import is needed
  3. Collect lazy imports from nested $-calls
  4. Call build_segment_program() with SegmentBuildData
  5. Return SegmentProgram with the constructed Program and SegmentAnalysis
```

### Stage 7: Emit (emit.rs)

```
Input:  &Program<'a>, source: &str, EmitOptions
Output: EmitResult { code: String, map: Option<String> }

For main module:
  Codegen::new().with_source_text(source).build(&program)

For each segment:
  Codegen::new()
    .with_options(CodegenOptions { source_map_path: ... })
    .with_source_text(original_source)
    .build(&segment_program)
```

### Pipeline Position

```
[Stage 4: TRANSFORM]
    |
    v  segments: Vec<SegmentData>   (from QwikTransform)
    |  mutated program: Program<'a> (main module, $()s replaced with qrl()s)
    |
[Stage 5: FILTER EXPORTS] (optional)
    |
    v
[Stage 6: EXTRACT SEGMENTS]  <-- THIS DOCUMENT: build_segment_program()
    |
    v  segment_programs: Vec<SegmentProgram<'a>>
    |  Each contains: Program<'a>, path: String, analysis: SegmentAnalysis
    |
[Stage 7: EMIT]               <-- THIS DOCUMENT: codegen pipeline
    |
    v  main_code: String, main_map: Option<String>
    |  segment_codes: Vec<(String, Option<String>)>
    |
[Stage 8: ASSEMBLE]
    |
    v  TransformOutput { modules: Vec<TransformModule>, ... }
```

### Key Constraint

All stages 4-7 must execute within the same scope as the `Allocator`. The `Program<'a>` instances (both main and segments) reference arena-allocated data that is freed when the allocator drops. Only `String` values (generated code, source maps) escape the allocator scope.

---

*This document satisfies APIM-06: Multi-module output pattern mapped to AstBuilder Program construction with allocator strategy. It serves as the construction spec for POC-03 (multi-module output) and POC-04 (source maps for split modules).*
