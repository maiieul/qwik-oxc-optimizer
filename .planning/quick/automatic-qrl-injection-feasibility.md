# Automatic QRL Injection: Technical Feasibility Report

> **Context:** This document synthesizes a technical discussion between project maintainers about implementing automatic QRL (Qwik URL) extraction in the OXC-based Qwik optimizer.
> 
> **Project Status:** Phase 25 of 26 complete (NAPI crate with napi-rs v2, transform_modules export working)

## The Problem

Currently, Qwik developers must manually wrap callbacks with `$()` to mark lazy-loading boundaries:

```tsx
import { $, component$, useTask$, useOnWindow } from '@qwik.dev/core';

export const App = component$(() => {
  // Manual wrapping required
  useTask$($(async () => {
    const data = await fetchData();
    console.log(data);
  }));
  
  useOnWindow('resize', $(() => {
    console.log('resized');
  }));
  
  return <div onClick$={$(() => alert('clicked'))}>Hello</div>;
});
```

This creates friction:
- Must remember which APIs require `$` suffix
- Cognitive overhead of understanding lazy-loading boundaries
- IDE/type-checker confusion with `$` patterns
- Learning curve for new developers

## The Vision: Automatic Injectable QRLs

Instead of manual wrapping, the optimizer would **automatically detect** when a lambda is passed to a known QRL-accepting API and inject the `$` transformation:

```tsx
// After: Write normal JavaScript
import { component$, useTask$, useOnWindow } from '@qwik.dev/core';

export const App = component$(() => {
  useTask$(async () => {
    const data = await fetchData();
    console.log(data);
  });
  
  useOnWindow('resize', () => {
    console.log('resized');
  });
  
  return <div onClick={() => alert('clicked')}>Hello</div>;
});
```

Behind the scenes, the optimizer transforms this to the same QRL calls, but developers write "normal" code.

## Technical Feasibility

**Yes, this is fully feasible with the current OXC-based optimizer.**

### Current Architecture

The optimizer already has all necessary components:

1. **Collector** (`crates/qwik-optimizer-oxc/src/collector.rs`)
   - First-pass AST walk to find imports, call sites, and captures
   - Already detects `$`-suffixed imports from `@qwik.dev/core`

2. **Transform Visitor** (`crates/qwik-optimizer-oxc/src/transform.rs`)
   - `enter_call_expression` hook inspects every call
   - Currently checks `is_dollar_call()` to detect `component$()`, `useTask$()`, etc.
   - Uses OXC's `traverse_mut` for full AST mutation

3. **Capture Analysis** (`crates/qwik-optimizer-oxc/src/collector.rs:166`)
   - Analyzes which variables from outer scope are captured in lambdas
   - Already works for nested `$()` calls
   - Can be applied to implicit QRLs

4. **Words/Patterns Module** (`crates/qwik-optimizer-oxc/src/words.rs`)
   - Contains constants like `dollar_to_qrl_name()`
   - Natural place to add QRL-accepting API patterns

### Implementation Strategy

Add a `QRL_ARG_PATTERNS` map that defines which argument positions accept QRLs:

```rust
// In words.rs
pub static QRL_ARG_PATTERNS: LazyLock<HashMap<&str, Vec<usize>>> = LazyLock::new(|| {
    HashMap::from([
        ("useTask$", vec![0]),           // 1st arg is the task function
        ("useVisibleTask$", vec![0]),   // 1st arg is the task function
        ("useOnWindow", vec![1]),       // 2nd arg is the handler
        ("useOnDocument", vec![1]),      // 2nd arg is the handler
        ("useOn$", vec![1]),             // 2nd arg is the handler
        // ... additional APIs
    ])
});
```

#### Step 1: Detection in Collector

Extend `walk_expression_for_calls` to detect implicit patterns:

```rust
fn walk_expression_for_calls(ctx: &mut CollectContext, expr: &Expression<'_>) {
    if let Expression::CallExpression(call) = expr {
        // Check for implicit QRL patterns
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            if let Some(arg_positions) = QRL_ARG_PATTERNS.get(name) {
                for (idx, arg) in call.arguments.iter().enumerate() {
                    if arg_positions.contains(&idx) && is_lambda(arg) {
                        ctx.implicit_dollar_calls.push(ImplicitDollarCall {
                            callee_name: name,
                            arg_index: idx,
                            span: call.span,
                        });
                    }
                }
            }
        }
        // ... existing dollar call detection
    }
}
```

#### Step 2: Transform in Visitor

In `enter_call_expression`, auto-wrap detected lambdas:

```rust
fn enter_call_expression(
    &mut self,
    call: &mut CallExpression<'a>,
    ctx: &mut TraverseCtx<'a, ()>,
) {
    // Existing $-call detection
    if let Some(kind) = self.is_dollar_call(call) {
        // ... existing logic
    }
    
    // NEW: Implicit QRL injection
    if let Expression::Identifier(ident) = &call.callee {
        let name = ident.name.as_str();
        if let Some(arg_positions) = words::QRL_ARG_PATTERNS.get(name) {
            for idx in arg_positions {
                if let Some(arg) = call.arguments.get_mut(*idx) {
                    if let Some(expr) = arg.as_expression_mut() {
                        if is_lambda_expression(expr) {
                            // Transform to qrl()
                            *expr = self.transform_to_qrl(expr, ctx);
                        }
                    }
                }
            }
        }
    }
}
```

#### Step 3: Capture Analysis

The existing capture analysis already handles nested scopes. For a pattern like:

```tsx
export const App = component$(() => {
  const state = useStore({ count: 0 });
  
  useTask$(async () => {
    // Captures `state` from outer scope
    state.count = await fetchCount();
  });
});
```

The collector's `compute_captures()` function will:
1. Detect `state` is referenced in the lambda
2. See it's declared in the outer `component$` scope
3. Classify it as a capture
4. Generate the appropriate `_captures` array

## Benefits

### 1. Massive DX Improvement
- **Zero learning curve** - Write JavaScript normally
- **No API mental model** - Don't need to know which functions are "special"
- **IDE-friendly** - Full TypeScript/intellisense support without `$` syntax
- **Refactoring safety** - Move code freely without adding/removing `$`

### 2. Competitive Advantage
Qwik becomes the first resumable framework where the lazy-loading feels **completely invisible**. Developers get the benefits without the cognitive overhead.

### 3. Migration Path
Can be implemented as an opt-in feature initially:
- Config flag: `automaticQrlInjection: true`
- Gradually make it default
- Backward compatible (explicit `$` still works)

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Debugging confusion when extraction fails | Clear error messages showing what was/wasn't extracted; dev mode logging |
| Custom user functions accepting callbacks | JSDoc annotation `@qrlHandler` or config to register custom patterns |
| Bundle surprises (unexpected splits) | Build report showing all extracted QRLs; visualizer integration |
| Over-extraction (wrapping non-QRL callbacks) | Conservative pattern matching; only extract for known APIs |

## Files to Modify

| File | Changes |
|------|---------|
| `crates/qwik-optimizer-oxc/src/words.rs` | Add `QRL_ARG_PATTERNS` mapping |
| `crates/qwik-optimizer-oxc/src/collector.rs` | Detect implicit patterns in `walk_expression_for_calls` |
| `crates/qwik-optimizer-oxc/src/transform.rs` | Handle implicit QRL wrapping in `enter_call_expression` |
| `crates/qwik-optimizer-oxc/src/types.rs` | Add `implicit_dollar_calls` to `CollectResult` |

## Testing Strategy

1. **Unit tests** for each QRL-accepting API pattern
2. **Spec test integration** - Add new test cases alongside existing 162 spec tests
3. **Capture analysis tests** - Ensure implicit QRLs correctly capture outer scope
4. **Edge cases** - Nested lambdas, async functions, generator functions

## Next Steps

1. **Create proof-of-concept** for `useTask$` and `useOnWindow` (2-3 APIs)
2. **Benchmark** compile-time overhead of additional pattern matching
3. **Community feedback** - Share with beta testers
4. **Full implementation** - Roll out to all QRL-accepting APIs
5. **Documentation** - Update guides to show "modern" syntax (with note about classic `$` syntax)

## Related Work

This aligns with modern framework trends:
- **React Server Components** - Automatic server/client boundaries
- **SolidStart** - Automatic server functions
- **SvelteKit** - Automatic form actions

Qwik would lead here by making resumability automatic rather than manual.

---

**Document Status:** Research synthesis complete
**Date:** 2026-02-13
**Source:** Technical discussion between maintainers on OXC optimizer capabilities
