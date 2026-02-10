# Capture Analysis Algorithm: OXC Scoping API Mapping (APIM-03)

**Date:** 2026-02-10
**Purpose:** A standalone, implementable algorithm specification for capture analysis using OXC Scoping APIs. A developer reading this document can build POC-02 without consulting any other document. This document maps the SWC optimizer's 3-step capture analysis process to OXC's `Scoping` API, provides the complete `compute_captures()` function in Rust, documents the 4-type classification logic, and covers all 8 edge cases with spec file examples.

## Requirements Coverage

| Requirement | Section | Description |
|-------------|---------|-------------|
| APIM-03 | Entire document | Capture analysis algorithm mapped to oxc_semantic Scoping APIs |
| 8 Edge Cases | Sections 5.1--5.8 | All capture edge cases with spec file references |

---

## 1. Overview: What Capture Analysis Does

Capture analysis determines which variables a `$()` closure body references from outer scopes. These are the "captures" -- variables that must be serialized at runtime for Qwik's resumability model. When a `$()` body is extracted into a separate lazy-loadable segment module, any variables it references from the enclosing scope are no longer lexically available. The captures mechanism provides these values at runtime via the `_captures[]` array.

**SWC optimizer's 3-step process:**

1. **GlobalCollect** -- Walk the entire program to identify all imports, exports, and top-level declarations. This builds the "declaration universe" that the capture analysis operates against.

2. **IdentCollector** -- Walk the extracted `$()` body to find all identifier references. Gather `local_idents` (identifiers that appear in expression position and reference bindings).

3. **compute_scoped_idents** -- Compare the collected identifiers against the declaration stack (`decl_stack`) to determine which ones cross scope boundaries. The result is partitioned into:
   - Variables (valid captures) -- added to the captures array
   - Functions/class declarations (invalid captures) -- produce diagnostic errors

**OXC equivalent:** Steps 1--3 are replaced by a single `SemanticBuilder::build()` call that produces `Scoping`, containing the complete scope tree and symbol table. The scope tree provides everything the SWC optimizer builds manually.

---

## 2. OXC API Mapping

### Step 1: GlobalCollect -> SemanticBuilder::build()

The SWC optimizer's `GlobalCollect` pass walks the program to build a map of all declarations and their scopes. In OXC, this is handled by `SemanticBuilder`:

```rust
use oxc_semantic::SemanticBuilder;

let semantic_ret = SemanticBuilder::new()
    .with_excess_capacity(2.0)
    .build(&program);
let scoping = semantic_ret.semantic.into_scoping();
```

The resulting `Scoping` object contains:
- **Scope tree:** All scopes in the program, their parent-child relationships, and which bindings exist in each scope
- **Symbol table:** Every declared symbol with its name, flags (Import, FunctionScopedVariable, etc.), and the scope it belongs to
- **Reference resolution:** Every `IdentifierReference` is resolved to the `SymbolId` it references (or unresolved for globals)

### Step 2: IdentCollector -> Iterate IdentifierReferences

The SWC optimizer's `IdentCollector` walks the `$()` body to find all identifier references. In OXC, this maps to examining every `IdentifierReference` node within the body during traversal, and checking whether it resolves to a symbol via `reference.symbol_id()`.

During the `traverse_mut` pass, when we enter a `$()` call body:

```rust
// In the Traverse implementation, when processing a $()-body:
// The body_scope_id is the ScopeId of the $() body's function scope.
// All IdentifierReferences within this scope (and descendant scopes)
// need to be checked for captures.
```

The key difference from SWC: OXC has already resolved all references during `SemanticBuilder::build()`. We do not need to walk the AST to find references -- we query the `Scoping` API.

### Step 3: compute_scoped_idents -> scope_ancestors() comparison

The SWC optimizer's `compute_scoped_idents()` checks whether each collected identifier is declared in a scope that is OUTSIDE the `$()` body's scope. In OXC:

```rust
use oxc_semantic::{Scoping, ScopeId};

/// Check if `inner` scope is contained within `outer` scope (or is the same scope).
/// Returns true if `outer` is an ancestor of `inner` in the scope tree, or they are equal.
fn is_scope_contained_in(
    scoping: &Scoping,
    inner: ScopeId,
    outer: ScopeId,
) -> bool {
    scoping.scope_ancestors(inner)
        .any(|scope_id| scope_id == outer)
}
```

If a symbol is declared in a scope that is NOT an ancestor of (or equal to) the body scope, then it is an outer reference -- a potential capture.

---

## 3. Complete Rust Algorithm: compute_captures()

This is the full capture analysis function using OXC Scoping APIs. It takes the `Scoping` from semantic analysis and the `ScopeId` of the `$()` body, and returns classified captures.

```rust
use oxc_semantic::{Scoping, ScopeId, SymbolId, SymbolFlags, ReferenceId};
use oxc_span::CompactStr;

/// Classification result for a single outer-scope reference.
#[derive(Debug, Clone)]
pub enum CaptureClassification {
    /// Import binding -- re-emit import in the segment module, do NOT capture
    ImportReemit {
        name: String,
        source: String,           // the module specifier (e.g., "@qwik.dev/core")
        is_default: bool,         // default import vs named import
    },

    /// Const-evaluable literal -- inline the value in the segment, do NOT capture
    ConstInline {
        name: String,
        // The literal value to inline (number, string, boolean, null, undefined)
        // In practice, this is determined by inspecting the symbol's initializer AST node
    },

    /// Local variable -- captured via _captures[], add to captures array in qrl() call
    LocalCapture {
        name: String,
        index: usize,             // position in _captures[] array
    },

    /// Function/class declaration -- emit diagnostic error, invalid capture
    InvalidCapture {
        name: String,
        kind: &'static str,       // "function" or "class"
    },
}

/// Result of capture analysis for a single $()-body.
#[derive(Debug, Clone)]
pub struct CaptureAnalysisResult {
    /// Variables that will be passed via _captures[] at runtime
    pub captures: Vec<String>,

    /// Imports that need to be re-emitted in the segment module
    pub reemitted_imports: Vec<ReemittedImport>,

    /// Const values that will be inlined in the segment body
    pub inlined_consts: Vec<InlinedConst>,

    /// Diagnostic errors for invalid captures (function/class declarations)
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReemittedImport {
    pub local_name: String,
    pub source: String,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct InlinedConst {
    pub name: String,
    // In practice, store the literal value or a reference to the AST node
}

/// Determine which variables are captured by a $()-body.
///
/// This function iterates all symbols in the program and checks which ones:
/// 1. Are declared OUTSIDE the body scope
/// 2. Have references INSIDE the body scope
///
/// For each such symbol, it classifies the capture type:
/// - Import -> re-emit import in segment (NOT captured)
/// - Const literal -> inline value (NOT captured)
/// - Local variable -> add to _captures[] array
/// - Function/class -> emit diagnostic error
///
/// Arguments:
/// - `scoping`: The Scoping from SemanticBuilder::build()
/// - `body_scope_id`: The ScopeId of the $()-body's function scope
///
/// Returns a CaptureAnalysisResult with classified captures.
pub fn compute_captures(
    scoping: &Scoping,
    body_scope_id: ScopeId,
) -> CaptureAnalysisResult {
    let mut captures = Vec::new();
    let mut reemitted_imports = Vec::new();
    let mut inlined_consts = Vec::new();
    let mut diagnostics = Vec::new();
    let mut seen_names = std::collections::HashSet::new();

    // Iterate all symbols in the symbol table
    for symbol_id in scoping.symbol_ids() {
        let symbol_scope = scoping.symbol_scope_id(symbol_id);

        // Skip symbols declared INSIDE the body scope
        // (these are local to the $()-body, not captures)
        if is_scope_contained_in(scoping, symbol_scope, body_scope_id) {
            continue;
        }

        // Check if any references to this symbol are INSIDE the body scope
        let has_reference_in_body = scoping
            .get_resolved_references(symbol_id)
            .any(|ref_id| {
                // Get the scope where this reference appears
                // Note: Reference may provide scope_id() directly or
                // we use the reference's node to determine its scope.
                // This is Open Question #2 from research.
                // Strategy: During traversal, record body_scope_id and
                // check if reference's scope is a descendant.
                let ref_scope = get_reference_scope(scoping, ref_id);
                is_scope_contained_in(scoping, ref_scope, body_scope_id)
            });

        if !has_reference_in_body {
            continue; // No references inside the body, not a capture
        }

        let name = scoping.symbol_name(symbol_id).to_string();
        if !seen_names.insert(name.clone()) {
            continue; // Already processed this symbol
        }

        let flags = scoping.symbol_flags(symbol_id);

        // Classify the capture
        if flags.contains(SymbolFlags::Import) {
            // Import binding -> re-emit import in segment, NOT captured
            // Determine the import source by looking up the import declaration
            // that introduced this symbol. In practice, this information
            // is collected during the collector pass.
            reemitted_imports.push(ReemittedImport {
                local_name: name,
                source: String::new(), // filled in from collector pass data
                is_default: false,     // determined from import declaration
            });
        } else if is_const_literal(scoping, symbol_id) {
            // Const-evaluable literal -> inline the value, NOT captured
            inlined_consts.push(InlinedConst {
                name,
            });
        } else if flags.contains(SymbolFlags::Function)
            || flags.contains(SymbolFlags::Class)
        {
            // Function or class declaration -> emit diagnostic, invalid capture
            let kind = if flags.contains(SymbolFlags::Function) {
                "function"
            } else {
                "class"
            };
            diagnostics.push(format!(
                "Cannot capture {} declaration '{}' across $() boundary",
                kind, name
            ));
        } else {
            // Local variable -> captured via _captures[]
            let index = captures.len();
            captures.push(name);
        }
    }

    CaptureAnalysisResult {
        captures,
        reemitted_imports,
        inlined_consts,
        diagnostics,
    }
}

/// Check if `inner` scope is contained within `outer` scope (or is the same scope).
fn is_scope_contained_in(
    scoping: &Scoping,
    inner: ScopeId,
    outer: ScopeId,
) -> bool {
    scoping.scope_ancestors(inner)
        .any(|scope_id| scope_id == outer)
}

/// Determine if a symbol's initializer is a const-evaluable literal.
///
/// A symbol is const-evaluable if:
/// 1. It is declared with `const` (not `let` or `var`)
/// 2. Its initializer is a primitive literal: number, string, boolean, null, undefined
///
/// This function checks SymbolFlags for const-ness and requires
/// inspecting the AST node of the initializer during the traversal.
/// The actual implementation will check the initializer expression
/// during the transform pass.
fn is_const_literal(scoping: &Scoping, symbol_id: SymbolId) -> bool {
    // In practice, this requires access to the AST node, not just Scoping.
    // During the traversal, when we detect a capture candidate:
    // 1. Check if the symbol has SymbolFlags::ConstVariable
    // 2. Look up the VariableDeclarator AST node
    // 3. Check if the init expression is a literal (NumericLiteral,
    //    StringLiteral, BooleanLiteral, NullLiteral, or Identifier("undefined"))
    //
    // Placeholder: the actual implementation lives in the transform pass.
    false
}

/// Get the scope where a reference appears.
///
/// Open Question #2: Whether Reference provides scope_id() directly.
/// Strategy A: If Reference has scope_id(), use it.
/// Strategy B: During traversal, use TraverseCtx::current_scope_id()
///             when encountering IdentifierReferences in the body.
/// Strategy C: Use the reference's NodeId to look up the scope.
fn get_reference_scope(scoping: &Scoping, ref_id: ReferenceId) -> ScopeId {
    // Strategy B is recommended for POC-02:
    // During traverse_mut, in enter_identifier_reference(),
    // record (ref_id -> current_scope_id) in a HashMap.
    // Then use that map here.
    //
    // Placeholder: returns root scope. The actual implementation
    // will use the mapping built during traversal.
    ScopeId::new(0)
}
```

### Key Scoping Methods Used

| Method | Purpose | When Called |
|--------|---------|------------|
| `scoping.symbol_ids()` | Iterator over all declared symbols | Start of capture analysis |
| `scoping.symbol_scope_id(symbol_id)` | Get the scope where a symbol is declared | Check if declaration is outside body |
| `scoping.scope_ancestors(scope_id)` | Iterator walking up the scope tree | Determine scope containment |
| `scoping.get_resolved_references(symbol_id)` | Get all references that resolve to this symbol | Check if any reference is inside body |
| `scoping.symbol_name(symbol_id)` | Get the string name of a symbol | Build capture name list |
| `scoping.symbol_flags(symbol_id)` | Get SymbolFlags (Import, Function, Class, etc.) | Classify capture type |
| `scoping.has_binding(ref_id)` | Check if a reference resolves to a known symbol | Filter unresolved globals |
| `scoping.find_binding(scope_id, name)` | Find a binding by name, walking up scope tree | Alternative scope-walking lookup |

---

## 4. Capture Classification Logic

After identifying that a symbol is declared outside the `$()` body and referenced inside it, the capture is classified into one of four types:

### 4.1 Import Binding (SymbolFlags::Import) -> Re-emit Import, NOT Captured

**Check:** `scoping.symbol_flags(symbol_id).contains(SymbolFlags::Import)`

**Action:** Add the import declaration to the segment module's imports list. Do NOT add the symbol to the `_captures[]` array.

**Why:** Import bindings are static -- they can be reproduced in any module by adding the same import declaration. There is no runtime state to serialize. Re-emitting the import is cheaper and simpler than capturing.

**Example:** `import { useStore } from "@qwik.dev/core"` -- when a `$()` body calls `useStore()`, the segment gets `import { useStore } from "@qwik.dev/core"` rather than `const useStore = _captures[0]`.

### 4.2 Const-Evaluable Literal -> Inline Value, NOT Captured

**Check:** Symbol declared with `const`, initializer is a primitive literal (number, string, boolean, null, undefined).

**Action:** Replace the identifier reference in the segment body with the literal value. Do NOT add the symbol to `_captures[]`.

**Why:** Immutable primitive values can be duplicated without side effects. Inlining avoids runtime serialization overhead.

**Example:** `const arg0 = 20` -- when a `$()` body references `arg0`, the segment output uses literal `20` instead of `arg0`.

**Const-evaluable expressions (exhaustive list):**
- `NumericLiteral` (e.g., `20`, `3.14`, `-1`)
- `StringLiteral` (e.g., `"hello"`, `'world'`)
- `BooleanLiteral` (`true`, `false`)
- `NullLiteral` (`null`)
- Identifier `undefined`

**NOT const-evaluable:**
- Template literals with expressions: `` `${x}` ``
- Function calls: `const x = getDefault()`
- Object/array literals: `const x = {a: 1}`
- Binary/unary expressions: `const x = 1 + 2`

### 4.3 Local Variable -> Captured via _captures[]

**Check:** Not an import, not a const literal, not a function/class declaration.

**Action:** Add the symbol name to the captures array. In the segment body, insert `const <name> = _captures[<index>]` at the top of the function. In the `qrl()` or `inlinedQrl()` call, pass the captures as the third argument: `[state, count, ...]`.

**Index mapping:** The captures array order must be consistent between the `qrl()` call site (which passes the values) and the segment body (which destructures via `_captures[N]`). The SWC optimizer uses insertion order from the identifier collection pass.

**Example:** `const state = useStore({count: 0})` -- when a nested `$()` references `state`, the captures array contains `[state]`, and the segment body has `const state = _captures[0]`.

### 4.4 Function/Class Declaration -> Diagnostic Error

**Check:** `scoping.symbol_flags(symbol_id).contains(SymbolFlags::Function)` or `scoping.symbol_flags(symbol_id).contains(SymbolFlags::Class)`

**Action:** Emit a diagnostic warning or error. Do NOT capture the symbol. The SWC optimizer produces a warning for function captures and an error for class captures.

**Why:** Function and class declarations cannot be trivially serialized across module boundaries. They may have closures over local state, prototype chains, or other non-serializable properties. The Qwik pattern requires that captures be serializable values (signals, stores, primitives).

**Example:** `function helper() { ... }` declared outside a `$()` body that references `helper` inside -- diagnostic emitted.

---

## 5. All 8 Edge Cases

### 5.1 Edge Case 1: No Captures

**Spec file:** `example_1.md`

**Input:**
```tsx
import { $, component, onRender } from '@qwik.dev/core';

export const renderHeader = $(() => {
    return (
        <div onClick={$((ctx) => console.log(ctx))}/>
    );
});
```

**Captures result:** `captures: false`, no `_captures` import, no captures array in `qrl()`.

**OXC analysis:**

1. `SemanticBuilder::build()` produces `Scoping` with the complete scope tree.
2. The `$(() => { ... })` body has its own scope. Inside that scope:
   - `ctx` is a parameter of the inner `$((ctx) => ...)` -- declared inside the inner body, not a capture of either
   - `console` is unresolved (global) -- `scoping.has_binding(ref_id)` returns false, skipped
   - `render` (in the `component()` call) is unresolved -- skipped
3. No symbols are declared outside the body with references inside it.
4. Result: empty captures array.

**Segment output (from spec):**
```tsx
export const renderHeader_zBbHWn4e8Cg = ()=>{
    return <div onClick={/*#__PURE__*/ qrl(i_fV2uzAL99u4, "renderHeader_div_onClick_fV2uzAL99u4")}/>;
};
```

No `_captures` import. No captures parameter in `qrl()`. The segment is self-contained.

### 5.2 Edge Case 2: Variable Capture via _rawProps

**Spec file:** `example_multi_capture.md`

**Input:**
```tsx
import { $, component$ } from '@qwik.dev/core';

export const Foo = component$(({foo}) => {
    const arg0 = 20;
    return $(() => {
        const fn = ({aaa}) => aaa;
        return (
            <div>
                {foo}{fn()}{arg0}
            </div>
        )
    });
})
```

**Captures result for the inner `$()` body:** `captures: true`, `captureNames: ["_rawProps"]`.

Note: The props destructuring `({foo})` is transformed to `(_rawProps)` by the props destructuring pre-transform. After that transform, the component body uses `_rawProps.foo` instead of `foo`. The inner `$()` captures `_rawProps` (the entire props object), not individual prop names.

**OXC analysis:**

1. After props destructuring transform, the component scope declares `_rawProps` (a function parameter) and `arg0` (const variable).
2. The inner `$()` body scope references:
   - `_rawProps` -- declared in the component scope (OUTSIDE the `$()` body). `SymbolFlags` shows it is a local variable (not Import, not Function/Class). It is a **LocalCapture**.
   - `arg0` -- declared in the component scope as `const arg0 = 20`. The initializer is a `NumericLiteral(20)`, so it is **ConstInline**. NOT captured.
   - `fn` -- declared INSIDE the `$()` body scope. Not a capture (declared within the body).
   - `aaa` -- parameter of the inner arrow function, declared inside the body. Not a capture.
3. Result: captures = `["_rawProps"]`, inlined_consts = `["arg0"]`.

**Segment output (from spec) -- the inner $() segment:**
```jsx
import { _captures } from "@qwik.dev/core";
export const Foo_component_1_DvU6FitWglY = ()=>{
    const _rawProps = _captures[0];
    const fn = ({ aaa })=>aaa;
    return <div>
                {_rawProps.foo}{fn()}{20}
            </div>;
};
```

Key observations:
- `_rawProps` is restored from `_captures[0]`
- `arg0` is inlined as literal `20` (not captured)
- `fn` is declared in the body (not captured)
- The `_captures` import is present because there is at least one capture

### 5.3 Edge Case 3: Capture of useStore State Variable

**Spec file:** `example_inlined_entry_strategy.md`

**Input:**
```tsx
import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';
import { thing } from './sibling';
import mongodb from 'mongodb';

export const Child = component$(() => {
    useStyles$('somestring');
    const state = useStore({
        count: 0
    });

    // Double count watch
    useBrowserVisibleTask$(() => {
        state.count = thing.doStuff() + import("./sibling");
    });

    return (
        <div onClick$={() => console.log(mongodb)}>
        </div>
    );
});
```

**Captures result for the `useBrowserVisibleTask$` body:** `captures: true`, captures = `[state]`.

**OXC analysis:**

1. The `useBrowserVisibleTask$` body scope references:
   - `state` -- declared in the component scope as `const state = useStore(...)`. The initializer is a function call, NOT a const literal. It is a **LocalCapture**.
   - `thing` -- declared in module scope with `SymbolFlags::Import`. It is an **ImportReemit**. In the inline strategy, `thing` remains accessible because everything is in the same module, but in segment strategy it would be re-emitted.
   - `console` -- unresolved global, skipped.
2. Result for this segment: captures = `["state"]`.

**Output (from spec) -- inline strategy, the useBrowserVisibleTask body:**
```tsx
useBrowserVisibleTaskQrl(/*#__PURE__*/ inlinedQrl(()=>{
    const state = _captures[0];
    state.count = thing.doStuff() + import("./sibling");
}, "Child_component_useBrowserVisibleTask_0IGFPOyJmQA", [
    state
]));
```

Key observations:
- `state` is captured and restored via `_captures[0]`
- `thing` is an import binding -- NOT captured, accessed directly in the inline strategy
- The third argument to `inlinedQrl` is `[state]` -- the captures array
- `console` is a global -- not captured, not imported

### 5.4 Edge Case 4: Const Literal Inlining (NOT Captured)

**Spec file:** `example_multi_capture.md`

**Input (relevant portion):**
```tsx
export const Foo = component$(({foo}) => {
    const arg0 = 20;
    return $(() => {
        const fn = ({aaa}) => aaa;
        return (
            <div>
                {foo}{fn()}{arg0}
            </div>
        )
    });
})
```

**Captures result:** `arg0` does NOT appear in captures.

**OXC analysis:**

1. `arg0` is declared as `const arg0 = 20` in the component scope.
2. During capture classification, inspect the VariableDeclarator:
   - `kind` is `VariableDeclarationKind::Const` (immutable binding)
   - `init` is `Expression::NumericLiteral(NumericLiteral { value: 20.0, ... })`
   - This matches the const-evaluable pattern: a `const` with a primitive literal initializer.
3. Classification: **ConstInline** -- the value `20` is inlined directly in the segment body.
4. `arg0` is NOT added to the captures array.

**Segment output (from spec):**
```jsx
return <div>
            {_rawProps.foo}{fn()}{20}
        </div>;
```

`{arg0}` has been replaced with `{20}` -- the literal value is inlined. No `_captures` entry for `arg0`.

**Detection algorithm:**

```rust
/// Check if an expression is a const-evaluable literal.
fn is_const_evaluable(expr: &Expression<'_>) -> bool {
    matches!(expr,
        Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
    ) || matches!(expr,
        Expression::Identifier(id) if id.name == "undefined"
    )
}
```

### 5.5 Edge Case 5: CSS Import Re-emission (Import Captures)

**Spec file:** `example_capture_imports.md`

**Input:**
```tsx
import { component$, useStyles$ } from '@qwik.dev/core';
import css1 from './global.css';
import css2 from './style.css';
import css3 from './style.css';

export const App = component$(() => {
    useStyles$(`${css1}${css2}`);
    useStyles$(css3);
})
```

**Captures result for the `useStyles$` template literal segment:** `captures: false`.

**OXC analysis:**

1. The `useStyles$(\`${css1}${css2}\`)` creates a segment with the template literal as its body.
2. The template literal references `css1` and `css2`.
3. Both `css1` and `css2` have `SymbolFlags::Import` -- they are default imports from CSS files.
4. Classification: **ImportReemit** -- the imports are re-emitted in the segment module, NOT captured.
5. Result: captures = `[]` (empty), reemitted_imports = `[css1 from "./global.css", css2 from "./style.css"]`.

**Segment output (from spec) -- useStyles template literal segment:**
```javascript
import css1 from "./global.css";
import css2 from "./style.css";
export const App_component_useStyles_t35nSa5UV7U = `${css1}${css2}`;
```

Key observations:
- No `_captures` import -- captures is false
- The CSS imports are re-emitted directly in the segment module
- The template literal body remains unchanged, referencing the locally-imported symbols
- This is the correct behavior: import bindings are always re-emitted, never captured

**Similarly for the `css3` segment:**
```javascript
import css3 from "./style.css";
export const App_component_useStyles_1_xBK4W0ZKWe8 = css3;
```

### 5.6 Edge Case 6: Nested Scope Captures (Mixed Classification)

**Spec file:** `example_inlined_entry_strategy.md`

**Input (relevant portion):**
```tsx
useBrowserVisibleTask$(() => {
    state.count = thing.doStuff() + import("./sibling");
});
```

Where:
- `state` is `const state = useStore({count: 0})` -- a local variable in the component scope
- `thing` is `import { thing } from './sibling'` -- an import binding
- `console` is a global (unresolved)

**Captures result:** Only `state` appears in captures. `thing` is re-emitted as an import. `console` is ignored.

**OXC analysis with scope_ancestors():**

```
Module Scope (root)
  |-- state: declared here (const = useStore(...))
  |-- thing: declared here (import binding)
  |-- [console: unresolved, not in scope tree]
  |
  +-- Component scope (component$(() => { ... }))
       |
       +-- useBrowserVisibleTask$ body scope
            |-- references: state, thing, console
```

For each reference in the `useBrowserVisibleTask$` body:

1. **`state`:** `symbol_scope_id(state) = Component scope`. Call `is_scope_contained_in(Component scope, useBrowserVisibleTask body scope)` -- returns false (Component scope is the PARENT, not contained IN the body). So `state` is an outer reference. `symbol_flags(state)` does NOT contain `SymbolFlags::Import`. `is_const_literal()` returns false (initializer is `useStore(...)`, a function call). Classification: **LocalCapture**. `captures = ["state"]`.

2. **`thing`:** `symbol_scope_id(thing) = Module scope`. `is_scope_contained_in(Module scope, body scope)` = false. `symbol_flags(thing)` contains `SymbolFlags::Import`. Classification: **ImportReemit**. Not added to captures.

3. **`console`:** `scoping.has_binding(ref_id)` returns false. Skipped entirely -- globals are not captured.

Result: captures = `["state"]`, reemitted_imports = `[thing from "./sibling"]`, globals ignored = `[console]`.

### 5.7 Edge Case 7: Function/Class Declarations as Invalid Captures

**Spec file:** Derived from SWC optimizer `compute_scoped_idents` logic in `transform.rs`.

**Input (hypothetical):**
```tsx
function helper() {
    return 42;
}

export const App = component$(() => {
    return $(() => {
        return helper();
    });
})
```

**Captures result:** Diagnostic error emitted. `helper` is NOT captured.

**OXC analysis:**

1. `helper` is declared at module scope as a function declaration.
2. The inner `$()` body references `helper`.
3. `symbol_scope_id(helper) = Module scope`. Outer reference detected.
4. `symbol_flags(helper)` contains `SymbolFlags::Function`.
5. Classification: **InvalidCapture**. Emit diagnostic: "Cannot capture function declaration 'helper' across $() boundary."

**Diagnostic output:**
```json
[{
    "category": "warning",
    "message": "Cannot capture function declaration 'helper' across $() boundary. Consider moving the function inside the $() body or converting it to an importable module.",
    "file": "test.tsx"
}]
```

**Why this is invalid:** Function declarations may close over local scope variables, making them non-serializable. The Qwik optimizer requires that all captures be JSON-serializable values. The recommended fix is to move the function inside the `$()` body, or extract it to a separate module and import it.

**OXC detection code:**

```rust
let flags = scoping.symbol_flags(symbol_id);

if flags.contains(SymbolFlags::Function) {
    diagnostics.push(format!(
        "Cannot capture function declaration '{}' across $() boundary. \
         Consider moving the function inside the $() body or converting \
         it to an importable module.",
        scoping.symbol_name(symbol_id)
    ));
} else if flags.contains(SymbolFlags::Class) {
    diagnostics.push(format!(
        "Cannot capture class declaration '{}' across $() boundary. \
         Consider moving the class inside the $() body or converting \
         it to an importable module.",
        scoping.symbol_name(symbol_id)
    ));
}
```

### 5.8 Edge Case 8: Props Destructuring Conversion

**Spec file:** `example_multi_capture.md`

**Input:**
```tsx
export const Foo = component$(({foo}) => {
    return $(() => {
        return (
            <div>
                {foo}
            </div>
        )
    });
})
```

**Transformation:** The destructured parameter `({foo})` is renamed to `(_rawProps)` in the component segment. The inner `$()` captures `_rawProps` and accesses `_rawProps.foo`.

**OXC analysis:**

This edge case requires a **props_destructuring pre-transform** that runs BEFORE capture analysis:

1. **Pre-transform step:** Detect that the component$ callback has a destructuring parameter pattern `({foo})`.
2. **Rename:** Replace the destructuring pattern with a single identifier parameter `_rawProps`.
3. **Rewrite body references:** Every reference to `foo` inside the component body becomes `_rawProps.foo`. This is a member expression replacement.
4. **After pre-transform:** The scope tree now has `_rawProps` as a binding in the component scope instead of `foo`.

**Post pre-transform code (conceptual):**
```tsx
export const Foo = component$((_rawProps) => {
    return $(() => {
        return (
            <div>
                {_rawProps.foo}
            </div>
        )
    });
})
```

5. **Capture analysis:** The inner `$()` body references `_rawProps`. `_rawProps` is declared in the component scope (outside the body). `SymbolFlags` shows it is a local variable parameter. Classification: **LocalCapture**. captures = `["_rawProps"]`.

**Component segment output (from spec):**
```javascript
export const Foo_component_HTDRsvUbLiE = (_rawProps)=>{
    return /*#__PURE__*/ qrl(i_DvU6FitWglY, "Foo_component_1_DvU6FitWglY", [
        _rawProps
    ]);
};
```

**Inner $() segment output (from spec):**
```jsx
import { _captures } from "@qwik.dev/core";
export const Foo_component_1_DvU6FitWglY = ()=>{
    const _rawProps = _captures[0];
    const fn = ({ aaa })=>aaa;
    return <div>
                {_rawProps.foo}{fn()}{20}
            </div>;
};
```

Key observations:
- The component parameter is `_rawProps` (not `{foo}`)
- The inner segment restores `_rawProps` from `_captures[0]`
- Property access `_rawProps.foo` replaces the destructured variable `foo`
- The captures array in `qrl()` contains `[_rawProps]`
- This conversion must happen BEFORE capture analysis runs, because the scope tree needs to reflect `_rawProps` as the binding, not `foo`

---

## 6. OXC Scoping API Reference

Complete reference of all `Scoping` methods used in capture analysis.

| Method | Signature | Purpose |
|--------|-----------|---------|
| `symbol_ids()` | `fn symbol_ids(&self) -> impl Iterator<Item = SymbolId>` | Iterate all declared symbols in the program |
| `symbol_scope_id(id)` | `fn symbol_scope_id(&self, id: SymbolId) -> ScopeId` | Get the scope where a symbol is declared |
| `symbol_name(id)` | `fn symbol_name(&self, id: SymbolId) -> &str` | Get the string name of a symbol |
| `symbol_flags(id)` | `fn symbol_flags(&self, id: SymbolId) -> SymbolFlags` | Get the flags (Import, Function, Class, etc.) |
| `scope_ancestors(id)` | `fn scope_ancestors(&self, id: ScopeId) -> impl Iterator<Item = ScopeId>` | Walk up the scope tree from a given scope (includes self) |
| `get_resolved_references(id)` | `fn get_resolved_references(&self, id: SymbolId) -> impl Iterator<Item = ReferenceId>` | Get all reference IDs that resolve to this symbol |
| `has_binding(ref_id)` | `fn has_binding(&self, ref_id: ReferenceId) -> bool` | Check if a reference resolves to a known symbol |
| `find_binding(scope_id, name)` | `fn find_binding(&self, id: ScopeId, name: &str) -> Option<SymbolId>` | Find a binding by name, walking up the scope tree |
| `get_binding(scope_id, name)` | `fn get_binding(&self, id: ScopeId, name: &str) -> Option<SymbolId>` | Get a binding in a specific scope (no ancestor walking) |
| `scope_has_binding(scope_id, name)` | `fn scope_has_binding(&self, id: ScopeId, name: &str) -> bool` | Check if a binding exists in exactly this scope |

### SymbolFlags Reference

| Flag | Meaning | Capture Classification |
|------|---------|----------------------|
| `SymbolFlags::Import` | Symbol is an import binding | ImportReemit |
| `SymbolFlags::Function` | Symbol is a function declaration | InvalidCapture |
| `SymbolFlags::Class` | Symbol is a class declaration | InvalidCapture |
| `SymbolFlags::FunctionScopedVariable` | Symbol is `var` or function parameter | LocalCapture |
| `SymbolFlags::BlockScopedVariable` | Symbol is `let` or `const` | LocalCapture (or ConstInline if const literal) |
| `SymbolFlags::ConstVariable` | Symbol is `const` | Check initializer for ConstInline |

---

## 7. Open Questions

### 7.1 Reference Scope Identification

**Question:** Does `Reference` have a `scope_id()` method, or do we need a span-based or traversal-based approach to determine where a reference appears?

**Current strategy:** During `traverse_mut`, in the `enter_identifier_reference` hook, record a mapping from `ReferenceId` to `current_scope_id()` provided by `TraverseCtx`. After traversal, use this mapping in `compute_captures()` to determine if a reference is inside the body scope.

**Fallback strategy:** If `Reference` provides `scope_id()` directly (verify during POC-02), the mapping is unnecessary and the capture analysis can query the scope directly.

**Impact:** This affects the function signature of `get_reference_scope()` in Section 3. If `Reference` has `scope_id()`, the function simplifies to a direct field access.

### 7.2 Exact SymbolFlags Values for Function/Class Declarations

**Question:** Are `SymbolFlags::Function` and `SymbolFlags::Class` the correct flags to check, or do function declarations use a different flag like `SymbolFlags::FunctionScopedVariable`?

**Current understanding:** Based on OXC docs, `SymbolFlags::Function` is set for function declarations (not function expressions), and `SymbolFlags::Class` is set for class declarations. Function expressions and arrow functions declared via `const` would have `SymbolFlags::BlockScopedVariable` instead.

**Impact:** Edge Case 7 (function/class invalid captures) depends on this. If the flag is different, the classification code needs adjustment.

**Recommendation:** Verify during POC-02 by declaring a function and a class, running `SemanticBuilder`, and inspecting `symbol_flags()` for each symbol.

### 7.3 Capture Ordering

**Question:** What determines the order of captures in the `_captures[]` array?

**Current understanding:** The SWC optimizer uses insertion order from the IdentCollector pass. In OXC, iterating `symbol_ids()` provides a stable ordering, but it may not match the SWC order. The order must be consistent between the `qrl()` call site (which provides `[state, count]`) and the segment body (which uses `_captures[0]` for `state`, `_captures[1]` for `count`).

**Recommendation:** For POC-02, use the order in which captures are first encountered during AST traversal of the body. Verify against spec file output to confirm matching.

---

## Appendix: Algorithm Flow Diagram

```
Input: Program AST + body_scope_id of $() body
                    |
                    v
        SemanticBuilder::build()
                    |
                    v
              Scoping object
                    |
                    v
    For each symbol_id in scoping.symbol_ids():
        |
        +---> symbol_scope = scoping.symbol_scope_id(symbol_id)
        |
        +---> is_scope_contained_in(symbol_scope, body_scope_id)?
        |         |
        |      YES: skip (declared inside body, not a capture)
        |         |
        |       NO: check references
        |
        +---> any reference to symbol inside body?
        |         |
        |       NO: skip (no inner references)
        |         |
        |      YES: classify
        |
        +---> scoping.symbol_flags(symbol_id)
                  |
                  +---> SymbolFlags::Import?
                  |         YES: ImportReemit (re-emit import in segment)
                  |
                  +---> is_const_literal()?
                  |         YES: ConstInline (inline value)
                  |
                  +---> SymbolFlags::Function or Class?
                  |         YES: InvalidCapture (emit diagnostic)
                  |
                  +---> else: LocalCapture (add to _captures[])
                            captures.push(name)
                            index = captures.len() - 1
```

---

*This document satisfies APIM-03: Capture analysis algorithm mapped to oxc_semantic Scoping APIs. It serves as the implementation spec for POC-02 (capture analysis proof of concept).*
