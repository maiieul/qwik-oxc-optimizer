# OXC API Mapping Guide: Foundational Transformation Patterns

**Date:** 2026-02-10
**OXC Version:** 0.113.0
**Purpose:** Primary implementation reference for the Qwik OXC optimizer. A developer reading this document should know exactly which OXC methods to call, in what order, for each foundational transformation pattern -- without consulting OXC source code or docs.

## Requirements Coverage

| Requirement | Section | Description |
|------------|---------|-------------|
| APIM-01 | [Pattern 1: $() Extraction](#pattern-1-dollar-extraction) | OXC Traverse + AstBuilder APIs for detecting $() call sites |
| APIM-02 | [Pattern 2: QRL Wrapping](#pattern-2-qrl-wrapping) | Constructing qrl() and inlinedQrl() wrapper expressions via AstBuilder |
| APIM-05 | [Pattern 3: Import Rewriting](#pattern-3-import-rewriting) | Rewriting imports using OXC statement mutation APIs |

---

## Pattern 1: $() Extraction (APIM-01)

### What This Pattern Does

The Qwik optimizer detects all `$()` call sites in source code -- both raw `$()` calls and `$`-suffixed function calls like `component$()`, `useStyles$()`, `useBrowserVisibleTask$()` -- and marks them for extraction into lazy-loadable segments. This is the foundation of Qwik's resumability: every `$()` boundary becomes a potential code-splitting point.

The detection logic must handle:
- Raw `$(() => { ... })` calls (see `example_1.md`)
- Named `$`-suffixed calls like `component$(() => { ... })` (see `example_functional_component.md`)
- Nested `$`-calls: a `$`-call inside another `$`-call's body (see `example_1.md` where `$((ctx) => console.log(ctx))` appears inside the outer `$(() => { ... })`)
- JSX attribute `$`-calls like `onClick$={() => ...}` (see `example_inlined_entry_strategy.md`)

### Before/After: example_1.md

**Input:**
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

**Output (main module `test.tsx`):**
```tsx
import { qrl } from "@qwik.dev/core";
const i_U6Kkv07sbpQ = ()=>import("./test.tsx_renderHeader_component_U6Kkv07sbpQ");
const i_zBbHWn4e8Cg = ()=>import("./test.tsx_renderHeader_zBbHWn4e8Cg");
import { component } from '@qwik.dev/core';
export const renderHeader = /*#__PURE__*/ qrl(i_zBbHWn4e8Cg, "renderHeader_zBbHWn4e8Cg");
const renderHeader = component(/*#__PURE__*/ qrl(i_U6Kkv07sbpQ, "renderHeader_component_U6Kkv07sbpQ"));
```

**Output (extracted segment `test.tsx_renderHeader_zBbHWn4e8Cg.tsx`):**
```tsx
import { qrl } from "@qwik.dev/core";
const i_fV2uzAL99u4 = ()=>import("./test.tsx_renderHeader_div_onClick_fV2uzAL99u4");
export const renderHeader_zBbHWn4e8Cg = ()=>{
    return <div onClick={/*#__PURE__*/ qrl(i_fV2uzAL99u4, "renderHeader_div_onClick_fV2uzAL99u4")}/>;
};
```

Key observations:
- Three `$()` calls detected: the outer `$(() => { return <div>... })`, the nested `$((ctx) => console.log(ctx))`, and the `$(() => { console.log("mount"); return render; })` inside `component()`
- The nested `$`-call inside the first `$`-call body is extracted separately (parent-child relationship in segment metadata)
- `$` and `onRender` are removed from the original import; `component` (not `$`-suffixed) is kept
- Each `$()` call replaced with `qrl(import_fn, "segment_name_hash")`

### Before/After: example_functional_component.md

**Input:**
```tsx
import { $, component$, useStore } from '@qwik.dev/core';
const Header = component$(() => {
	const thing = useStore();
	const {foo, bar} = foo();

	return (
		<div>{thing}</div>
	);
});
```

**Output (main module `test.tsx`):**
```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
import { $, component$, useStore } from '@qwik.dev/core';
const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));
```

**Output (extracted segment `test.tsx_Header_component_J4uyIhaBNR4.tsx`):**
```tsx
import { useStore } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
export const Header_component_J4uyIhaBNR4 = ()=>{
    const thing = useStore();
    const { foo, bar } = foo();
    return <div>{thing}</div>;
};
```

Key observations:
- `component$(() => { ... })` is detected as a named `$`-suffixed call
- The callee `component$` is replaced with `componentQrl` (dollar-to-qrl naming)
- `useStore` (non-dollar) is kept in the original imports and also appears in the extracted segment
- The extracted segment gets its own `import { useStore } from "@qwik.dev/core"` because the body references it

### OXC Traverse Implementation: enter_call_expression

The `Traverse` trait from `oxc_traverse` provides the visitor pattern for walking the AST. Detection of `$()` call sites happens in `enter_call_expression`:

```rust
use std::collections::HashSet;
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

/// Identifies the kind of $-call detected
#[derive(Debug, Clone)]
pub enum DollarCallKind {
    /// Raw $() call: $(() => { ... })
    RawDollar,
    /// Named $-suffixed call: component$(() => { ... })
    Named(String),
}

/// Main transformer implementing the Traverse trait
pub struct QwikTransform<'a> {
    /// Set of known $-suffixed imports from @qwik.dev/core
    /// Populated during enter_program or enter_import_declaration
    dollar_imports: HashSet<String>,

    /// Collected segment data from detected $-calls
    segments: Vec<SegmentData>,

    /// Phantom lifetime for arena allocator
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        if let Some(kind) = self.is_dollar_call(call) {
            // This is a $-call site. Record it for extraction.
            self.record_segment(call, &kind, ctx);
        }
    }
}
```

### The `is_dollar_call()` Helper

This function inspects a `CallExpression` and returns a `DollarCallKind` if the callee is a known `$` or `$`-suffixed function:

```rust
use oxc_ast::ast::{CallExpression, Expression};

impl<'a> QwikTransform<'a> {
    /// Check if a CallExpression is a Qwik $-call.
    ///
    /// Returns Some(DollarCallKind) if the callee is:
    /// - `$` (raw dollar, imported from @qwik.dev/core)
    /// - A $-suffixed identifier (component$, useStyles$, etc.) imported from @qwik.dev/core
    ///
    /// Returns None if:
    /// - Callee is not an identifier (e.g., member expression like `obj.$()`)
    /// - Callee is not in the set of known dollar imports
    pub fn is_dollar_call(
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
            // Named $-suffixed call: component$(() => { ... }), useStyles$('...')
            Expression::Identifier(ident) if ident.name.ends_with('$') => {
                let name = ident.name.as_str();
                if self.dollar_imports.contains(name) {
                    Some(DollarCallKind::Named(name.to_string()))
                } else {
                    None
                }
            }
            // Member expression callees like this.onClick$ are NOT handled
            // in the basic pattern. They require additional logic to check
            // if the property name ends with '$'.
            _ => None,
        }
    }
}
```

### Collecting $-imports from @qwik.dev/core

Before detecting `$`-calls, the transformer must know which identifiers are `$`-suffixed imports from `@qwik.dev/core`. This collection happens during `enter_program` (scan all import declarations at program start) or `enter_import_declaration`:

```rust
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn enter_import_declaration(
        &mut self,
        import_decl: &mut ImportDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a>,
    ) {
        let source = import_decl.source.value.as_str();

        // Check if this import is from @qwik.dev/core (or the configured core module)
        if source != "@qwik.dev/core" {
            return;
        }

        // Collect all $-suffixed specifiers
        if let Some(specifiers) = &import_decl.specifiers {
            for specifier in specifiers {
                if let ImportDeclarationSpecifier::ImportSpecifier(spec) = specifier {
                    let imported_name = match &spec.imported {
                        ModuleExportName::IdentifierName(ident) => ident.name.as_str(),
                        ModuleExportName::IdentifierReference(ident) => ident.name.as_str(),
                        ModuleExportName::StringLiteral(s) => s.value.as_str(),
                    };

                    // Record if it's "$" or ends with "$"
                    if imported_name == "$" || imported_name.ends_with('$') {
                        self.dollar_imports.insert(imported_name.to_string());
                    }
                }
            }
        }
    }
}
```

### Edge Cases

**Nested $-calls:** In `example_1.md`, the outer `$(() => { return <div onClick={$((ctx) => ...)}/>; })` contains a nested `$((ctx) => console.log(ctx))`. Both are detected as separate `$`-call sites. The inner one gets a `parent` field in its `SegmentAnalysis` pointing to the outer segment. The `enter_call_expression` visitor naturally handles this because it fires for every `CallExpression` in the tree, including nested ones.

**Member expression callees:** The basic `is_dollar_call` only handles `Identifier` callees. If a future pattern requires detecting `obj.component$()` (a `MemberExpression` callee), the match arm would need to check `Expression::StaticMemberExpression(member)` and inspect `member.property.name`. The current spec files do not show this pattern for function calls, but JSX attributes like `onClick$` are handled differently (see Pattern 2 inline strategy).

**Non-@qwik.dev/core imports:** A function named `component$` imported from a different module should NOT be treated as a dollar call. The `dollar_imports` set only contains names imported from `@qwik.dev/core`.

---

## Pattern 2: QRL Wrapping (APIM-02)

### What This Pattern Does

After detecting a `$()` call site, the optimizer replaces it with a QRL wrapper expression. There are two sub-patterns depending on the entry strategy:

1. **Segment strategy** (default): The `$()` body is extracted to a separate file, and the call site is replaced with `qrl(import_fn, "segment_name_hash")`
2. **Inline strategy**: The `$()` body stays in the same file, and the call site is replaced with `inlinedQrl(body, "segment_name_hash", [captures])`

### Sub-pattern A: Segment Strategy

**Transformation:** `$(() => { ... })` becomes `qrl(i_hashValue, "Name_hash")`

From `example_1.md`, the input:
```tsx
export const renderHeader = $(() => {
	return (
		<div onClick={$((ctx) => console.log(ctx))}/>
	);
});
```

Becomes (in the main module `test.tsx`):
```tsx
export const renderHeader = /*#__PURE__*/ qrl(i_zBbHWn4e8Cg, "renderHeader_zBbHWn4e8Cg");
```

From `example_functional_component.md`, the input:
```tsx
const Header = component$(() => {
	const thing = useStore();
	const {foo, bar} = foo();
	return (
		<div>{thing}</div>
	);
});
```

Becomes (in the main module `test.tsx`):
```tsx
const Header = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_J4uyIhaBNR4, "Header_component_J4uyIhaBNR4"));
```

Note the double wrapping: `component$(() => {...})` becomes `componentQrl(qrl(...))`, where:
- The inner `qrl()` replaces the `$()` call
- The outer `component$` becomes `componentQrl` (see `dollar_to_qrl_name()`)

### Sub-pattern B: Inline Strategy

**Transformation:** `$(() => { ... })` becomes `inlinedQrl(() => { ... }, "Name_hash", [captures])`

From `example_inlined_entry_strategy.md`, the input:
```tsx
export const Child = component$(() => {
	useStyles$('somestring');
	const state = useStore({
		count: 0
	});
	useBrowserVisibleTask$(() => {
		state.count = thing.doStuff() + import("./sibling");
	});
	return (
		<div onClick$={() => console.log(mongodb)}>
		</div>
	);
});
```

Becomes (single output module `test.tsx`):
```tsx
import { componentQrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { useBrowserVisibleTaskQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { useStore } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';
export const Child = /*#__PURE__*/ componentQrl(/*#__PURE__*/ inlinedQrl(()=>{
    useStylesQrl(/*#__PURE__*/ inlinedQrl('somestring', "Child_component_useStyles_qBZTuFM0160"));
    const state = useStore({
        count: 0
    });
    // Double count watch
    useBrowserVisibleTaskQrl(/*#__PURE__*/ inlinedQrl(()=>{
        const state = _captures[0];
        state.count = thing.doStuff() + import("./sibling");
    }, "Child_component_useBrowserVisibleTask_0IGFPOyJmQA", [
        state
    ]));
    return <div q-e:click={/*#__PURE__*/ inlinedQrl(()=>console.log(mongodb), "Child_component_div_q_e_click_cROa4sult1s")}>
		</div>;
}, "Child_component_9GyF01GDKqw"));
```

Key observations for inline strategy:
- No separate segment files are produced -- everything stays in the main module
- `inlinedQrl()` takes 2 or 3 arguments: (body, name_hash, [captures_array]?)
- When a segment captures outer scope variables (like `state` in the `useBrowserVisibleTask$`), the captures array is the 3rd argument
- Inside the inlined body, captured variables are re-declared from `_captures[]`: `const state = _captures[0]`
- `_captures` is imported from `@qwik.dev/core`
- `useStyles$('somestring')` becomes `useStylesQrl(inlinedQrl('somestring', "..."))` -- the string argument itself is wrapped
- JSX `onClick$={() => ...}` becomes `q-e:click={inlinedQrl(() => ..., "...")}`

### Complete Rust Code: `build_qrl_call()`

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a segment-strategy QRL call expression:
///   qrl(i_hashValue, "SegmentName_hash")
///
/// Arguments:
/// - `import_ident_name`: The identifier for the lazy import constant (e.g., "i_zBbHWn4e8Cg")
/// - `segment_export_name`: The segment's export name (e.g., "renderHeader_zBbHWn4e8Cg")
/// - `ctx`: The TraverseCtx providing access to AstBuilder via ctx.ast
///
/// Returns an Expression that can replace the original $() CallExpression.
pub fn build_qrl_call<'a>(
    import_ident_name: &str,
    segment_export_name: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Argument 1: identifier reference to the lazy import function
    // Produces: i_zBbHWn4e8Cg
    let import_ref = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom(import_ident_name),
    );

    // Argument 2: string literal with the segment export name
    // Produces: "renderHeader_zBbHWn4e8Cg"
    let name_literal = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(segment_export_name),
        None,
    );

    // Build arguments vector: [i_hash, "name_hash"]
    let mut arguments = ctx.ast.vec_with_capacity(2);
    arguments.push(Argument::from(import_ref));
    arguments.push(Argument::from(name_literal));

    // Build callee: identifier "qrl"
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("qrl"),
    );

    // Build: qrl(i_hash, "name")
    // For PURE annotation, see PURE Annotation Strategy section below
    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}
```

### Complete Rust Code: `build_inlined_qrl_call()`

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build an inline-strategy QRL call expression:
///   inlinedQrl(() => { body }, "Name_hash")                    -- no captures
///   inlinedQrl(() => { body }, "Name_hash", [captured_vars])   -- with captures
///
/// Arguments:
/// - `body_expr`: The arrow function expression (or other expression) to keep inline
/// - `segment_name`: The segment name with hash (e.g., "Child_component_9GyF01GDKqw")
/// - `captures`: Slice of captured variable names (may be empty)
/// - `ctx`: The TraverseCtx providing access to AstBuilder via ctx.ast
///
/// Returns an Expression that can replace the original $() CallExpression.
pub fn build_inlined_qrl_call<'a>(
    body_expr: Expression<'a>,
    segment_name: &str,
    captures: &[String],
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Capacity: body + name + optional captures array
    let capacity = if captures.is_empty() { 2 } else { 3 };
    let mut arguments = ctx.ast.vec_with_capacity(capacity);

    // Argument 1: the expression (arrow function, string literal, etc.)
    // This is the original $() argument, kept inline
    arguments.push(Argument::from(body_expr));

    // Argument 2: string literal segment name
    // Produces: "Child_component_9GyF01GDKqw"
    arguments.push(Argument::from(
        ctx.ast.expression_string_literal(
            SPAN,
            ctx.ast.atom(segment_name),
            None,
        ),
    ));

    // Argument 3 (optional): captures array
    // Only present when the segment captures variables from outer scope
    // Produces: [state] or [state, count, thing]
    if !captures.is_empty() {
        let mut elements = ctx.ast.vec_with_capacity(captures.len());
        for capture_name in captures {
            elements.push(ArrayExpressionElement::from(
                ctx.ast.expression_identifier_reference(
                    SPAN,
                    ctx.ast.atom(capture_name),
                ),
            ));
        }
        arguments.push(Argument::from(
            ctx.ast.expression_array(SPAN, elements, None),
        ));
    }

    // Build callee: identifier "inlinedQrl"
    let callee = ctx.ast.expression_identifier_reference(
        SPAN,
        ctx.ast.atom("inlinedQrl"),
    );

    // Build: inlinedQrl(body, "name", [captures])
    ctx.ast.expression_call(SPAN, callee, NONE, arguments, false)
}
```

### Complete Rust Code: `build_lazy_import_declaration()`

For the segment strategy, each extracted segment needs a lazy import constant in the main module. The pattern is:

```
const i_hash = () => import("./file_segment_hash")
```

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a lazy import declaration:
///   const i_hash = () => import("./path_segment_hash")
///
/// Arguments:
/// - `hash`: The segment hash (e.g., "zBbHWn4e8Cg")
/// - `import_path`: The relative path to the segment file (e.g., "./test.tsx_renderHeader_zBbHWn4e8Cg")
/// - `ctx`: The TraverseCtx providing access to AstBuilder via ctx.ast
///
/// Returns a Statement to be inserted at the top of the module.
pub fn build_lazy_import_declaration<'a>(
    hash: &str,
    import_path: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    let ident_name = format!("i_{}", hash);

    // Build the import expression: import("./path_segment_hash")
    let import_source = ctx.ast.expression_string_literal(
        SPAN,
        ctx.ast.atom(import_path),
        None,
    );
    let import_expr = ctx.ast.expression_import(
        SPAN,
        import_source,
        ctx.ast.vec(), // no import attributes
        None,          // no phase
    );

    // Build the arrow function: () => import(...)
    // This is an expression-body arrow (no block, expression: true)
    let params = ctx.ast.formal_parameters(
        SPAN,
        FormalParameterKind::ArrowFormalParameters,
        ctx.ast.vec(), // no parameters
        None,          // no rest element
    );

    // For expression-body arrows, OXC's AstBuilder uses a FunctionBody
    // with the expression as a single ExpressionStatement, but sets
    // expression: true on the ArrowFunctionExpression.
    let expr_stmt = ctx.ast.statement_expression(SPAN, import_expr);
    let body = ctx.ast.function_body(
        SPAN,
        ctx.ast.vec(),        // no directives
        ctx.ast.vec1(expr_stmt), // single expression statement
    );

    let arrow = ctx.ast.expression_arrow_function(
        SPAN,
        true,   // expression body (=> expr, not => { ... })
        false,  // not async
        false,  // not generator
        NONE,   // no type parameters
        params,
        NONE,   // no return type
        body,
    );

    // Build: const i_hash = () => import(...)
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
        false, // not definite
    );
    let declaration = ctx.ast.variable_declaration(
        SPAN,
        VariableDeclarationKind::Const,
        ctx.ast.vec1(declarator),
        false, // not declare
    );

    Statement::from(Declaration::VariableDeclaration(
        ctx.ast.alloc(declaration),
    ))
}
```

### PURE Annotation Strategy

Spec outputs consistently show `/*#__PURE__*/` before `qrl()`, `componentQrl()`, and `inlinedQrl()` calls. This annotation tells bundlers the call has no side effects and can be tree-shaken if the result is unused.

**Option A: `expression_call_with_pure` (if available)**

OXC's `AstBuilder` may provide a method that marks a call expression as pure, causing `Codegen` to emit the `/*#__PURE__*/` comment:

```rust
// If AstBuilder has this method:
let pure_qrl_call = ctx.ast.expression_call_with_pure(
    SPAN,
    callee,
    NONE,      // no type arguments
    arguments,
    false,     // not optional
    true,      // pure annotation
);
```

**Option B: Manual leading comment attachment**

If `expression_call_with_pure` does not emit the comment through Codegen, attach a leading comment manually:

```rust
use oxc_ast::Comment;
use oxc_span::Span;

// Build the call expression normally
let qrl_call = ctx.ast.expression_call(SPAN, callee, NONE, arguments, false);

// Attach a leading comment: /*#__PURE__*/
// The exact API for comment attachment depends on OXC version.
// In OXC 0.113, comments are stored on the Program and associated by span.
// The Codegen outputs leading comments before the expression.
```

**Recommendation:** Start with Option A (`expression_call_with_pure`). Verify by generating a PURE-annotated call and checking Codegen output. If the comment does not appear, fall back to Option B. This is documented as an open question from research (see Phase 4 Research, Open Question 2).

### Expression Replacement Pattern

The replacement of `$()` calls with `qrl()` / `inlinedQrl()` calls happens by mutating the AST node in an `exit_expression` or `exit_call_expression` hook:

```rust
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn exit_expression(
        &mut self,
        expr: &mut Expression<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        if let Expression::CallExpression(call) = expr {
            if let Some(kind) = self.is_dollar_call(call) {
                // Build the replacement expression
                let replacement = match &self.entry_strategy {
                    EntryStrategy::Segment => {
                        // Build: qrl(i_hash, "name_hash")
                        let segment_info = self.get_segment_info(call);
                        build_qrl_call(
                            &segment_info.import_ident,
                            &segment_info.export_name,
                            ctx,
                        )
                    }
                    EntryStrategy::Inline => {
                        // Build: inlinedQrl(body, "name_hash", [captures])
                        let segment_info = self.get_segment_info(call);
                        let body = call.arguments[0].take_in(ctx.ast);
                        build_inlined_qrl_call(
                            body,
                            &segment_info.export_name,
                            &segment_info.captures,
                            ctx,
                        )
                    }
                };

                // For named $-suffixed calls (component$, useStyles$, etc.),
                // wrap with the Qrl-suffixed version:
                // component$(body) -> componentQrl(qrl_or_inlinedQrl)
                let final_expr = match &kind {
                    DollarCallKind::RawDollar => replacement,
                    DollarCallKind::Named(name) => {
                        let qrl_name = dollar_to_qrl_name(name);
                        let qrl_callee = ctx.ast.expression_identifier_reference(
                            SPAN,
                            ctx.ast.atom(&qrl_name),
                        );
                        let mut args = ctx.ast.vec_with_capacity(1);
                        args.push(Argument::from(replacement));
                        ctx.ast.expression_call(SPAN, qrl_callee, NONE, args, false)
                    }
                };

                // Replace the expression in-place
                *expr = final_expr;
            }
        }
    }
}
```

### The `dollar_to_qrl_name()` Utility

Converts `$`-suffixed function names to their `Qrl`-suffixed equivalents:

```rust
/// Convert a $-suffixed function name to its Qrl-suffixed equivalent.
///
/// Examples:
///   "component$"             -> "componentQrl"
///   "useStyles$"             -> "useStylesQrl"
///   "useBrowserVisibleTask$" -> "useBrowserVisibleTaskQrl"
///   "$"                      -> "qrl"  (special case for raw $)
///
/// Panics if the input does not end with '$'.
pub fn dollar_to_qrl_name(name: &str) -> String {
    if name == "$" {
        return "qrl".to_string();
    }
    assert!(name.ends_with('$'), "Expected $-suffixed name, got: {}", name);
    let base = &name[..name.len() - 1]; // Strip trailing '$'
    format!("{}Qrl", base)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_to_qrl_name() {
        assert_eq!(dollar_to_qrl_name("$"), "qrl");
        assert_eq!(dollar_to_qrl_name("component$"), "componentQrl");
        assert_eq!(dollar_to_qrl_name("useStyles$"), "useStylesQrl");
        assert_eq!(dollar_to_qrl_name("useBrowserVisibleTask$"), "useBrowserVisibleTaskQrl");
    }
}
```

---

## Pattern 3: Import Rewriting (APIM-05)

### What This Pattern Does

The optimizer transforms import statements to reflect the `$`-to-`Qrl` renaming and adds new imports needed by the generated code. The rules are:

1. Remove all `$`-suffixed specifiers from original `@qwik.dev/core` imports
2. Add `{name}Qrl` import for each `$`-suffixed function that was used
3. Add `qrl` import (segment strategy) or `inlinedQrl` import (inline strategy)
4. Add `_captures` import if any segment has captures (inline strategy only)
5. Keep non-dollar specifiers unchanged (`useStore`, `useSignal`, etc.)
6. Keep non-`@qwik.dev/core` imports unchanged

### Before/After: example_functional_component.md

**Input imports:**
```tsx
import { $, component$, useStore } from '@qwik.dev/core';
```

**Output imports (main module `test.tsx`):**
```tsx
import { componentQrl } from "@qwik.dev/core";
import { qrl } from "@qwik.dev/core";
const i_J4uyIhaBNR4 = ()=>import("./test.tsx_Header_component_J4uyIhaBNR4");
import { $, component$, useStore } from '@qwik.dev/core';
```

**Output imports (extracted segment `test.tsx_Header_component_J4uyIhaBNR4.tsx`):**
```tsx
import { useStore } from "@qwik.dev/core";
```

Observations:
- New `import { componentQrl }` added (because `component$` was used)
- New `import { qrl }` added (segment strategy always needs `qrl`)
- Original import `{ $, component$, useStore }` is kept in the main module (the SWC optimizer preserves the original import declaration)
- The extracted segment gets only the imports it actually references (`useStore`)

### Before/After: example_inlined_entry_strategy.md

**Input imports:**
```tsx
import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';
```

**Output imports (inline strategy, single module):**
```tsx
import { componentQrl } from "@qwik.dev/core";
import { useStylesQrl } from "@qwik.dev/core";
import { inlinedQrl } from "@qwik.dev/core";
import { useBrowserVisibleTaskQrl } from "@qwik.dev/core";
import { _captures } from "@qwik.dev/core";
import { useStore } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';
```

Observations:
- Each `$`-suffixed function gets a separate `Qrl` import: `componentQrl`, `useStylesQrl`, `useBrowserVisibleTaskQrl`
- `inlinedQrl` import added (inline strategy)
- `_captures` import added (because `useBrowserVisibleTask$` callback captures `state`)
- `useStore` kept from original (non-dollar)
- Non-`@qwik.dev/core` imports unchanged: `thing` from `./sibling`, `mongodb` default import

### Import Ordering

The import ordering observed in spec outputs follows this pattern:

1. **New Qrl-suffixed imports** from `@qwik.dev/core` (one per specifier)
2. **`qrl` or `inlinedQrl` import** from `@qwik.dev/core`
3. **`_captures` import** (if needed)
4. **Lazy import constants** (segment strategy: `const i_hash = () => import(...)`)
5. **Original imports** (kept as-is, including the original `@qwik.dev/core` import with all specifiers)
6. **Remaining code** (declarations, exports, etc.)

### Complete Rust Code: `build_named_import()`

```rust
use oxc_ast::ast::*;
use oxc_span::SPAN;
use oxc_traverse::TraverseCtx;

/// Build a named import declaration:
///   import { name } from "source"
///
/// Arguments:
/// - `name`: The imported identifier (e.g., "componentQrl", "qrl", "inlinedQrl")
/// - `source`: The module specifier (e.g., "@qwik.dev/core")
/// - `ctx`: The TraverseCtx providing access to AstBuilder via ctx.ast
///
/// Returns a Statement containing the ImportDeclaration.
pub fn build_named_import<'a>(
    name: &str,
    source: &str,
    ctx: &mut TraverseCtx<'a>,
) -> Statement<'a> {
    // Build the local binding identifier
    let local = ctx.ast.binding_identifier(SPAN, ctx.ast.atom(name));

    // Build the imported name (same as local for non-aliased imports)
    let imported = ctx.ast.module_export_name_identifier_name(
        SPAN,
        ctx.ast.atom(name),
    );

    // Build the import specifier: { name }
    let specifier = ctx.ast.import_specifier(
        SPAN,
        imported,
        local,
        ImportOrExportKind::Value,
    );

    // Wrap in specifiers vec
    let specifiers = ctx.ast.vec1(
        ImportDeclarationSpecifier::ImportSpecifier(specifier),
    );

    // Build the source string literal: "@qwik.dev/core"
    let source_lit = ctx.ast.string_literal(SPAN, ctx.ast.atom(source), None);

    // Build the import declaration
    let import_decl = ctx.ast.module_declaration_import_declaration(
        SPAN,
        Some(specifiers),
        source_lit,
        NONE,                      // no import attributes
        ImportOrExportKind::Value,
    );

    Statement::from(import_decl)
}
```

### Import Management Strategy: Collect During Traversal, Build in exit_program

The recommended approach is to collect all needed imports during the traversal, then construct and insert them in the `exit_program` hook. This avoids modifying `program.body` while iterating over it:

```rust
use oxc_ast::ast::*;
use oxc_traverse::{Traverse, TraverseCtx};

/// Tracks which imports are needed for the transformed module
#[derive(Default)]
struct ImportTracker {
    /// Qrl-suffixed imports needed: "componentQrl", "useStylesQrl", etc.
    qrl_imports: Vec<String>,

    /// Whether the module needs `import { qrl }` (segment strategy)
    needs_qrl: bool,

    /// Whether the module needs `import { inlinedQrl }` (inline strategy)
    needs_inlined_qrl: bool,

    /// Whether the module needs `import { _captures }` (inline + captures)
    needs_captures: bool,

    /// Lazy import constants to insert (segment strategy)
    /// Each entry: (hash, import_path)
    lazy_imports: Vec<(String, String)>,
}

impl<'a> Traverse<'a> for QwikTransform<'a> {
    fn exit_program(
        &mut self,
        program: &mut Program<'a>,
        ctx: &mut TraverseCtx<'a>,
    ) {
        let mut new_stmts: Vec<Statement<'a>> = Vec::new();

        // 1. Add Qrl-suffixed imports (one per specifier, matching spec output)
        for qrl_name in &self.import_tracker.qrl_imports {
            let import_stmt = build_named_import(qrl_name, "@qwik.dev/core", ctx);
            new_stmts.push(import_stmt);
        }

        // 2. Add qrl or inlinedQrl import
        if self.import_tracker.needs_qrl {
            let import_stmt = build_named_import("qrl", "@qwik.dev/core", ctx);
            new_stmts.push(import_stmt);
        }
        if self.import_tracker.needs_inlined_qrl {
            let import_stmt = build_named_import("inlinedQrl", "@qwik.dev/core", ctx);
            new_stmts.push(import_stmt);
        }

        // 3. Add _captures import if needed
        if self.import_tracker.needs_captures {
            let import_stmt = build_named_import("_captures", "@qwik.dev/core", ctx);
            new_stmts.push(import_stmt);
        }

        // 4. Add lazy import constants (segment strategy)
        for (hash, import_path) in &self.import_tracker.lazy_imports {
            let lazy_stmt = build_lazy_import_declaration(hash, import_path, ctx);
            new_stmts.push(lazy_stmt);
        }

        // 5. Prepend new statements before existing program body
        let existing = std::mem::take(&mut program.body);
        program.body = ctx.ast.vec_from_iter(
            new_stmts.into_iter().chain(existing.into_iter()),
        );
    }
}
```

### Import Rewriting Rules Summary

| Rule | Input | Output | Example |
|------|-------|--------|---------|
| Remove `$`-suffixed specifiers | `import { component$ }` | (removed from original) | `component$` removed |
| Add `{name}Qrl` import | `component$` was used | `import { componentQrl }` | One per `$`-function |
| Add `qrl` import | Segment strategy | `import { qrl }` | Always for segment strategy |
| Add `inlinedQrl` import | Inline strategy | `import { inlinedQrl }` | Always for inline strategy |
| Add `_captures` import | Any segment has captures | `import { _captures }` | Inline strategy with captures |
| Keep non-dollar specifiers | `import { useStore }` | `import { useStore }` | Unchanged |
| Keep non-core imports | `import { thing } from './sibling'` | `import { thing } from './sibling'` | Unchanged |
| Import ordering | New QRL imports first | See ordering section | Matches spec output order |

---

## Cross-Cutting: OXC Transformation Pipeline

### Complete Pipeline: Parse -> Semantic -> Traverse -> Codegen

```rust
use oxc_allocator::Allocator;
use oxc_codegen::{Codegen, CodegenReturn};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_traverse::traverse_mut;

/// Transform a single Qwik module using the complete OXC pipeline.
///
/// This function owns the Allocator, so all AST manipulation and
/// code generation must complete within this scope. The returned
/// strings (code, source map) are owned data that outlives the allocator.
pub fn transform_single_module(
    source: &str,
    filename: &str,
    options: &TransformOptions,
) -> Result<SingleModuleOutput, anyhow::Error> {
    // 1. Create the arena allocator -- all AST nodes will live here
    let allocator = Allocator::default();

    // 2. Determine source type from filename extension
    let source_type = SourceType::from_path(filename)
        .unwrap_or_else(|_| SourceType::tsx());

    // 3. Parse source code into AST
    let parser_ret = Parser::new(&allocator, source, source_type).parse();
    if parser_ret.panicked {
        return Err(anyhow::anyhow!("Parser panicked on {}", filename));
    }
    let mut program = parser_ret.program;

    // 4. Semantic analysis (scopes + symbols)
    // Required for capture analysis in later phases.
    // with_excess_capacity(2.0) pre-allocates for the mutations
    // the transformer will make.
    let semantic_ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);
    let (symbols, scopes) = semantic_ret.semantic.into_symbol_table_and_scope_tree();

    // 5. Create the Qwik transformer
    let mut qwik_transform = QwikTransform::new(options, filename);

    // 6. Run the traversal -- this mutates the AST in-place
    // traverse_mut walks the AST, calling enter_*/exit_* methods
    // on the transformer for each node type
    traverse_mut(
        &mut qwik_transform,
        &allocator,
        &mut program,
        symbols,
        scopes,
    );

    // 7. Code generation -- serialize AST back to JavaScript
    // MUST happen in the same scope as the allocator, because
    // the Program<'a> references arena-allocated data
    let codegen_result: CodegenReturn = Codegen::new()
        .with_source_text(source)
        .build(&program);

    // 8. Return owned data (String) that outlives the allocator
    Ok(SingleModuleOutput {
        code: codegen_result.code,
        map: codegen_result.map.map(|m| m.to_json_string()),
        segments: qwik_transform.extracted_segments(),
    })
}
```

### Arena Allocator Lifetime Rules

OXC uses an arena allocator (`oxc_allocator::Allocator`) for all AST nodes. This has critical implications:

**Rule 1: Serialize in the same scope as the allocator.** The `Program<'a>` and all its child nodes have a lifetime `'a` tied to the allocator. You cannot return a `Program<'a>` from a function where the `Allocator` is a local variable -- the nodes would reference freed memory. Always run `Codegen::build(&program)` before the allocator drops.

```rust
// CORRECT: Codegen in same scope as allocator
fn transform(source: &str) -> String {
    let allocator = Allocator::default();
    let program = parse(&allocator, source);
    // ... mutate program ...
    let result = Codegen::new().build(&program); // <-- same scope
    result.code // String is owned, outlives allocator
}

// WRONG: Returning Program from function
fn parse_only<'a>(source: &str) -> Program<'a> {
    let allocator = Allocator::default(); // dropped at end of function!
    let ret = Parser::new(&allocator, source, source_type).parse();
    ret.program // LIFETIME ERROR: program references freed allocator
}
```

**Rule 2: Use `take_in(ctx.ast)` instead of cloning.** When you need to move an AST node from one location to another (e.g., extracting the body of a `$()` call), use the `take_in` method which moves ownership within the arena. Cloning allocates new memory unnecessarily:

```rust
// CORRECT: Move ownership using take_in
fn extract_body<'a>(
    call: &mut CallExpression<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    // Take the first argument out, leaving a placeholder
    call.arguments[0].take_in(ctx.ast)
}

// AVOID: Cloning wastes arena memory
fn extract_body_bad<'a>(
    call: &CallExpression<'a>,
    ctx: &mut TraverseCtx<'a>,
) -> Expression<'a> {
    call.arguments[0].clone_in(ctx.ast) // Unnecessary allocation
}
```

**Rule 3: Do not modify `program.body` during traversal of `program.body`.** Inserting or removing statements while the traversal is iterating over them causes undefined behavior or panics. Instead:
- Collect mutations during traversal (in `enter_*`/`exit_*` hooks)
- Apply all mutations in `exit_program` after traversal completes
- Or use OXC's `StatementInjectorStore` pattern for deferred insertion

---

*This document covers APIM-01 ($-extraction), APIM-02 (QRL wrapping), and APIM-05 (import rewriting). Architecture (module layout, public API types, data flow, test harness, Cargo.toml) is documented in Plan 02.*
