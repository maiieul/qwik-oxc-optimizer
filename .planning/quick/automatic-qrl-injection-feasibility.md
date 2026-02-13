# Automatic QRL Injection: Technical Feasibility Report

> **Context:** This document synthesizes a technical discussion between project maintainers about implementing automatic QRL (Qwik URL) extraction in the OXC-based Qwik optimizer.
> 
> **Project Status:** Phase 25 of 26 complete (NAPI crate with napi-rs v2, transform_modules export working)

## The Problem

Currently, Qwik has **two inconsistencies** that create friction:

### Inconsistency #1: APIs Without $ Suffix Need Manual Wrapping

```tsx
import { $, component$, useTask$, useOnWindow } from '@qwik.dev/core';

export const App = component$(() => {
  // ✓ Works - $ suffix on API extracts automatically
  useTask$(async () => {
    const data = await fetchData();
    console.log(data);
  });
  
  // ✓ Works - $ suffix on JSX attribute extracts automatically
  return <div onClick$={() => alert('clicked')}>Hello</div>;
  
  // ✗ Doesn't work - API without $ suffix requires manual wrapping
  useOnWindow('resize', $(() => {
    console.log('resized');
  }));
});
```

**Learning curve:** "Why do I need `$` wrapping for `useOnWindow` but not `useTask$`?"

### Inconsistency #2: Cannot Reference Module-Level Functions

```tsx
// ✗ Doesn't work today - function reference won't extract
function handleClick() {
  console.log('clicked');
}

export const App = component$(() => {
  return <div onClick$={handleClick}>Click me</div>;
});

// Workaround - inline arrow function (duplicates code, can't reuse)
export const App = component$(() => {
  return <div onClick$={() => handleClick()}>Click me</div>;
});
```

**Limitation:** You can't define a reusable handler function and reference it. Forces inline definitions.

## The Vision: Automatic Injectable QRLs

### Part 1: Type 3 APIs Without Manual Wrapping

```tsx
// After: Type 3 APIs work without $() wrapping
useOnWindow('resize', () => console.log('resized'));
```

### Part 2: Module-Level Function References Auto-Extract (The Big Win)

```tsx
// Define handlers as regular functions
function handleClick() {
  console.log('clicked');
}

function handleResize() {
  console.log('resized');
}

// Reference them in QRL-accepting contexts
export const App = component$(() => {
  useOnWindow('resize', handleResize);  // Auto-extracts handleResize as QRL
  return <div onClick$={handleClick}>Click me</div>;  // Auto-extracts handleClick as QRL
});

// Reuse handlers across components
export const Button = component$(() => {
  return <button onClick$={handleClick}>Click</button>;
});
```

**Benefits:**
- **Regular function declarations** work (not just arrow functions)
- **Function reuse** - Define once, reference from multiple places
- **Cleaner organization** - Separate handlers from component logic
- **Better testing** - Test handlers in isolation
- **Familiar patterns** - Like React's event handlers but with lazy loading

### What Stays The Same

```tsx
// JSX keeps $ suffix - it's the signal for QRL extraction
<div onClick$={() => ...}>           // Still works
<div onClick$={handler}>             // Still works (handler is extracted)
<div onClick={() => ...}>             // Won't work (no $, no extraction)

// APIs with $ suffix still work (backward compatible)
useTask$(async () => { ... })         // Still works
component$(() => { ... })             // Still works
```

## Technical Feasibility

**Yes, this is fully feasible with the current OXC-based optimizer.**

### Current Architecture

The optimizer already has all necessary components:

1. **Collector** (`crates/qwik-optimizer-oxc/src/collector.rs`)
   - First-pass AST walk to find imports, call sites, and captures
   - Already detects `$`-suffixed imports from `@qwik.dev/core`
   - Tracks `module_level_decls` including function declarations

2. **Transform Visitor** (`crates/qwik-optimizer-oxc/src/transform.rs`)
   - `enter_call_expression` hook inspects every call
   - Uses OXC's `traverse_mut` for full AST mutation

3. **Capture Analysis** (`crates/qwik-optimizer-oxc/src/collector.rs:166`)
   - Analyzes which variables from outer scope are captured in lambdas
   - Already works for nested `$()` calls

### Key Insight: OXC vs SWC

**OXC is NOT limited like SWC.** The optimizer already handles:

- ✓ Arrow functions: `() => ...`
- ✓ Function expressions: `function() { ... }`
- ✓ Module-level declarations (tracked in collector)

What doesn't work today:
- ✗ Identifier references to module-level functions passed to QRL contexts

This is a **feature gap, not an architectural limitation**. OXC has all the information needed to extract module-level function declarations.

### Implementation Strategy

#### Part 1: Type 3 APIs (No Manual Wrapping)

Add a `QRL_ARG_PATTERNS` map:

```rust
// In words.rs
pub static QRL_ARG_PATTERNS: LazyLock<HashMap<&str, Vec<usize>>> = LazyLock::new(|| {
    HashMap::from([
        ("useTask$", vec![0]),           // 1st arg is the task function
        ("useVisibleTask$", vec![0]),    // 1st arg is the task function
        ("useOnWindow", vec![1]),        // 2nd arg is the handler
        ("useOnDocument", vec![1]),      // 2nd arg is the handler
        ("useOn$", vec![1]),             // 2nd arg is the handler
        // ... additional APIs
    ])
});
```

When a lambda is passed to these APIs, auto-wrap it with QRL extraction.

#### Part 2: Module-Level Function References

Extend the collector and transform to handle identifier references:

**Step 1: Track function references in collector**

```rust
// In collector.rs walk_expression_for_calls
fn walk_expression_for_calls(ctx: &mut CollectContext, expr: &Expression<'_>) {
    if let Expression::CallExpression(call) = expr {
        // ... existing QRL_ARG_PATTERNS detection
        
        // NEW: Detect identifier references to module-level functions
        for (idx, arg) in call.arguments.iter().enumerate() {
            if let Argument::Identifier(ident) = arg {
                let name = ident.name.as_str();
                if ctx.module_level_decls.contains(name) {
                    // This is a reference to a module-level declaration
                    ctx.function_references.push(FunctionReference {
                        name: name.to_string(),
                        call_span: call.span,
                        arg_index: idx,
                    });
                }
            }
        }
    }
}
```

**Step 2: Extract module-level functions as QRLs**

```rust
// In transform.rs
fn enter_call_expression(
    &mut self,
    call: &mut CallExpression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    // ... existing logic
    
    // NEW: Handle identifier references to module-level functions
    if let Expression::Identifier(ident) = &call.callee {
        let name = ident.name.as_str();
        if let Some(arg_positions) = words::QRL_ARG_PATTERNS.get(name) {
            for idx in arg_positions {
                if let Some(arg) = call.arguments.get(*idx) {
                    // Check if argument is an identifier reference
                    if let Argument::Identifier(ident_ref) = arg {
                        let ref_name = ident_ref.name.as_str();
                        if self.collected.module_level_decls.contains(ref_name) {
                            // Extract the referenced function as a QRL
                            // Transform to: qrl(import_path, "functionName_hash")
                            let replacement = self.create_qrl_for_function_ref(
                                ref_name, 
                                ctx
                            );
                            // Replace the argument with the QRL call
                            if let Some(arg_mut) = call.arguments.get_mut(*idx) {
                                *arg_mut = replacement;
                            }
                        }
                    }
                }
            }
        }
    }
}
```

**Step 3: Create QRL segments for module-level functions**

When a module-level function is referenced in a QRL context:
1. Treat the function declaration as a segment (like `component$` segments)
2. Generate a unique hash and segment name
3. Extract the function body to a separate module
4. Replace the reference with `qrl(i_hash, "functionName_hash")`

## Benefits

### 1. Massive DX Improvement
- **Zero learning curve** - Write JavaScript normally
- **No API mental model** - No need to remember where `$` goes
- **Function reuse** - Define handlers once, use everywhere
- **Familiar patterns** - Like React but with lazy loading
- **Better code organization** - Separate concerns

### 2. Competitive Advantage
Qwik becomes the first framework where **resumability is completely invisible**. Developers write normal JavaScript and get automatic lazy loading.

### 3. Migration Path
Can be implemented incrementally:
- Phase 1: Type 3 APIs without manual wrapping (easiest)
- Phase 2: Module-level function references (bigger win)
- Config flag: `automaticQrlInjection: true` to opt-in
- Backward compatible - explicit `$` still works

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Over-extraction (wrapping wrong functions) | Conservative pattern matching - only known APIs |
| Side effects in module-level functions | Document that referenced functions must be pure/stateless |
| Debugging confusion | Dev mode logging showing what was extracted |
| Bundle size surprises | Build report showing all QRL segments |
| Custom user APIs | JSDoc `@qrlHandler` or config registration |

## Files to Modify

| File | Changes |
|------|---------|
| `words.rs` | Add `QRL_ARG_PATTERNS` mapping |
| `collector.rs` | Detect identifier references to module-level functions in QRL contexts |
| `transform.rs` | Handle module-level function extraction in `enter_call_expression` |
| `types.rs` | Add `function_references` to `CollectResult` |
| `code_move.rs` | Generate QRL segments for module-level function declarations |

## Testing Strategy

1. **Unit tests** for each API pattern (Type 3 APIs)
2. **Function reference tests** - Module-level functions extracted correctly
3. **Capture analysis** - Functions capturing outer scope variables
4. **Multiple references** - Same function used in multiple places
5. **Edge cases** - Async functions, generators, nested references

## Research Questions

### Q: Can OXC handle regular functions (not just arrow functions)?

**A: Yes!** Looking at `jsx_transform.rs:21-22`:

```rust
JSXExpression::ArrowFunctionExpression(arrow) => Some((arrow.span.start, arrow.span.end)),
JSXExpression::FunctionExpression(func) => Some((func.span.start, func.span.end)),
```

OXC already handles both. The limitation is with **identifier references** to module-level declarations, which is a logic gap, not an architectural limitation.

### Q: What's the difference from SWC?

**A:** SWC had architectural limitations with certain AST patterns. OXC's traverse API is more flexible and can handle:
- Module-level declaration tracking
- Identifier reference analysis
- Cross-reference resolution

This makes module-level function extraction feasible in OXC where it wasn't in SWC.

## Next Steps

1. **Proof of concept** - Implement Type 3 API auto-wrapping for 2-3 APIs
2. **Function reference POC** - Extract one module-level function as QRL
3. **Benchmark** - Measure compile-time overhead
4. **Spec test integration** - Add test cases for new patterns
5. **Community RFC** - Gather feedback on the approach

## Related Work

- **React Server Components** - Automatic server/client boundaries
- **SolidStart** - Automatic server functions  
- **SvelteKit** - Automatic form actions
- **Next.js App Router** - Automatic code splitting

Qwik would lead by making component-level lazy loading automatic.

---

**Document Status:** Research synthesis complete  
**Date:** 2026-02-13  
**Source:** Technical discussion between maintainers on OXC optimizer capabilities and Qwik API design
