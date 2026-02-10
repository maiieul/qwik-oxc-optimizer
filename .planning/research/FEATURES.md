# Feature Landscape: Qwik Optimizer Transformations

**Domain:** SWC-based code optimizer for Qwik framework lazy-loading
**Researched:** 2026-02-10
**Confidence:** HIGH (source code + 163 snapshot tests as primary evidence)

## Table Stakes

Features that MUST be documented or the spec is incomplete. Missing any of these means the spec cannot describe the optimizer's actual behavior.

### 1. Core Segmentation Pipeline

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `$()` extraction into lazy-loadable segments | The fundamental transformation -- without this the optimizer does nothing | High | Extracts closures passed to `$()` into separate modules. Generates unique hash-based names like `App_component_div_onClick_i7ekvWH3674`. |
| `component$` to `componentQrl` conversion | Every Qwik component uses this pattern | Med | Rewrites `component$(fn)` to `componentQrl(qrl(lazyImport, "segmentName"))`. Also handles `useTask$`/`useStyles$`/etc. |
| `foo$` to `fooQrl` generic hook conversion | Extensible pattern used by all `$`-suffixed APIs | Med | Any `name$()` from the core module becomes `nameQrl(qrl(...))`. Handles `useTask$`, `useBrowserVisibleTask$`, `useStyles$`, `server$`, `serverLoader$`, `serverStuff$`, `serverAuth$`, `sync$`, `event$`, custom inlined functions via `wrap()`. |
| Segment naming convention | Required for deterministic output and debugging | Med | Pattern: `{filename}_{componentName}_{ctxName}_{hash}`. E.g., `test.tsx_Header_component_J4uyIhaBNR4`. Hash must be consistent across entry strategies and emit modes. |
| Segment metadata (SegmentAnalysis) | Build tools depend on this metadata | Med | Each segment emits: origin, name, entry, displayName, hash, canonicalFilename, path, extension, parent, ctxKind, ctxName, captures, loc, paramNames, captureNames. |
| Hash consistency guarantee | Incorrect hashes break lazy loading | High | Hashes must remain identical across all EmitMode (Prod/Dev/Test) and EntryStrategy combinations, and across transpile_ts/transpile_jsx flags. Verified by `consistent_hashes` test. |

### 2. Capture Analysis

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Variable capture detection | Segments must know what they close over | High | Identifies all identifiers referenced inside a `$()` closure that are declared outside it. These become `scoped_idents` / `captureNames` in segment metadata. |
| Capture serialization via `_captures` | Runtime needs to restore captures | Med | Captured variables are passed as the third argument to `qrl()` and restored via `const x = _captures[0]` in the segment body. |
| Import capture (hoisting) | Segments need their own imports | Med | When a segment references an import from the original module, that import is replicated in the generated segment file. |
| Function/class capture tracking | Different capture rules for different declarations | Med | Functions, classes, and variables are tracked separately. Functions and classes referenced in segments get captured. |
| Local vs scoped ident classification | Determines what goes into segment vs stays in parent | Med | `local_idents` stay in the segment, `scoped_idents` are captures from parent scope. |

### 3. JSX Transformation

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| JSX to `_jsxSorted` calls | Primary JSX output format when no spread props | Med | `<div class="x">child</div>` becomes `_jsxSorted("div", varProps, constProps, children, flags, key)`. Separates var props from const props. |
| JSX to `_jsxSplit` calls | Required when spread props are present | Med | `<div {...props}>` uses `_jsxSplit` with `_getVarProps(props)` and `_getConstProps(props)` helpers. |
| Event handler naming: `onClick$` to `q-e:click` | DOM event binding convention | Med | In JSX output, `onClick$` becomes `"q-e:click"` attribute. Applied only to native HTML elements, NOT to component elements. |
| Immutability flags | Performance optimization for rendering | Med | The numeric flags argument (e.g., `3`, `1`, `0`) to `_jsxSorted`/`_jsxSplit` indicates constness of the element. |
| Automatic key generation (`u6_0`, `u6_1`, ...) | Stable DOM reconciliation | Low | Auto-generated keys for elements that don't have explicit `key` props. Scoped per component. |
| Fragment handling | JSX fragments must work | Low | `<>...</>` becomes `_jsxSorted(Fragment, ...)` with Fragment imported from `@qwik.dev/core/jsx-runtime`. |
| `dangerouslySetInnerHTML` handling | Special attribute with unique semantics | Low | Treated as a special case in JSX transformation. |
| `className` to `class` conversion | React compat for native elements | Low | `className` prop is rewritten to `class` on native HTML elements. Not rewritten on component elements. |
| `key` prop extraction | Key is a separate argument, not a prop | Low | `key={expr}` is extracted from props and passed as a separate argument to `_jsxSorted`. |

### 4. Derived Signal Optimization

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `_wrapProp` for direct signal/store access | Fine-grained reactivity for props | High | When a prop value is `signal.value` or `store.path`, wraps with `_wrapProp(signal)` or `_wrapProp(store, "path")` to enable reactive tracking. |
| `_fnSignal` for computed expressions | Inline reactive computations | High | When a prop value is a computed expression involving signals/stores (e.g., `12 + signal.value`), generates a hoisted function: `const _hf0 = (p0) => 12 + p0.value` with string representation `"12+p0.value"`, then `_fnSignal(_hf0, [signal], _hf0_str)`. |
| Var props vs const props separation | Enables runtime optimization of static vs dynamic props | High | Props are classified: static/const values go to `constProps` (2nd arg), dynamic/reactive values go to `varProps` (1st arg). Classification uses `is_const_expr` analysis. |
| Signal detection in children | Children need same reactive wrapping | Med | Children content like `{signal.value}` or `{store.path}` gets wrapped with `_wrapProp`/`_fnSignal` just like props. |
| No-inline detection | Some expressions cannot be converted to derived signals | Med | Function calls (`signal.value()`), mixed unknown calls (`signal.value + unknown()`), `mutable()` calls, and mixed types (`signal.value + dep`) fall back to var props without wrapping. |

### 5. Props Destructuring Optimization

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Simple destructuring to `_rawProps` access | Enables signal-level prop tracking | High | `component$(({count, stuff: hey}) => ...)` becomes `component$((_rawProps) => { ... _rawProps.count ... _rawProps.stuff ... })`. Props accessed as `_rawProps.propName`. |
| Default value preservation | Destructuring defaults must work | Med | `{some = 1+2}` becomes `_rawProps.some ?? 3` (pre-computed constant). |
| Rest props via `_restProps` | Spread/rest patterns need runtime support | Med | `{count, ...rest}` generates `const rest = _restProps(_rawProps, ["count", ...])`. |
| Non-optimizable destructuring fallthrough | Complex patterns bail out | Med | Nested destructuring like `{stuff: {hey}}` or function-call defaults like `{stuff = hola()}` do NOT get the optimization -- original destructuring preserved. |
| Colon-keyed props (`bind:value`) | Two-way binding syntax needs special handling | Med | `props['bind:value']` destructuring is preserved through the optimization. |

### 6. Entry Strategies

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `Segment` strategy (one file per segment) | Default strategy, each `$()` gets its own file | Low | `PerSegmentStrategy` -- every segment becomes a separate entry point. |
| `Inline` strategy (all in one file) | Used for dev/SSR bundling | Low | `InlineStrategy` -- all segments stay in the parent module using `inlinedQrl()` instead of `qrl()` with dynamic imports. |
| `Hoist` strategy (same as Inline) | Alias behavior for Inline | Low | Mapped to same `InlineStrategy` as Inline. |
| `Single` strategy (all segments in one bundle) | Simplest bundling approach | Low | `SingleStrategy` -- all segments go into one shared entry. |
| `Component` strategy (segments grouped by component) | Groups related segments | Med | `PerComponentStrategy` -- segments within same component share an entry file named `{origin}_entry_{rootComponent}`. |
| `Smart` strategy (context-aware grouping) | Production optimization | Med | `SmartStrategy` -- event handlers without captured variables get their own files; everything else grouped by component. Top-level QRLs get separate files. |
| `Hook` strategy (alias for Segment) | Legacy alias | Low | Maps to `PerSegmentStrategy`. |

### 7. Emit Modes

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `Prod` mode | Production builds: `qrl()` calls, no dev info | Med | Uses `qrl()` for segment references. No source location metadata. |
| `Dev` mode | Development builds: `qrlDEV()` with source locations | Med | Uses `qrlDEV()` with `{file, lo, hi, displayName}` metadata. JSX elements get `{fileName, lineNumber, columnNumber}` debug info. |
| `Test` mode | Test builds: similar to Prod but no const replacement | Low | Like Prod but skips `ConstReplacerVisitor`. |
| `Lib` mode | Library builds: skip most transforms | Low | Skips QwikTransform entirely. Only runs TS/JSX transpilation, rename imports, and resolution. |

### 8. Code Stripping

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `strip_exports` (named export removal) | Server-only exports removed from client bundles | Med | Specified exports (e.g., `onGet`) are replaced with throwing stubs: `export const onGet = () => { throw "Symbol removed by Qwik Optimizer..." }`. Unused imports are cleaned up by DCE. |
| `strip_ctx_name` (context-based stripping) | Remove server-only `$()` calls from client | High | E.g., `strip_ctx_name: ["server"]` removes all `server$()`, `serverLoader$()`, `serverStuff$()` calls. Replaced with `_noopQrl("hash")` to preserve the QRL reference shape while eliminating the implementation. |
| `strip_event_handlers` (event handler removal) | Server builds don't need client event handlers | Med | When true, removes all `onClick$`, `onInput$`, etc. event handler QRLs from JSX output. Used in server-side rendering builds. |
| `isServer`/`isBrowser`/`isDev` const replacement | Dead code elimination for platform-specific code | Med | `isServer` replaced with `true`/`false` literal based on `is_server` config. `isBrowser` replaced with inverse. `isDev` replaced based on emit mode. Enables DCE to remove dead branches. |

### 9. Import Management

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `@builder.io/qwik` to `@qwik.dev/core` rename | Migration support | Low | `RenameTransform` rewrites all `@builder.io/qwik*` imports to `@qwik.dev/*` equivalents. Also handles `@builder.io/qwik-city` to `@qwik.dev/router` and `@builder.io/qwik-react` to `@qwik.dev/react`. |
| Synthetic import generation | Segments need their own imports | Med | Generated segment files get imports for `qrl`, `_captures`, `_jsxSorted`, `_fnSignal`, `_wrapProp`, `componentQrl`, `inlinedQrl`, etc. from `@qwik.dev/core`. |
| Dynamic import generation for segments | Lazy loading mechanism | Med | Parent modules generate lazy import functions: `const i_HASH = () => import("./segment_file")`. |
| Import assertion preservation | `assert { type: "json" }` syntax | Low | Import assertions are preserved through transformation. |
| Explicit extensions option | Some bundlers need file extensions | Low | When `explicit_extensions: true`, generated import paths include `.js`/`.tsx` extensions. |
| Relative path normalization | Cross-platform path handling | Low | Windows backslash paths converted to forward slashes. Paths normalized relative to `src_dir`. |

### 10. Side Effect Analysis and Cleanup

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| Tree shaking (Treeshaker) | Client bundles should not include server-only code | High | Two-phase: (1) `CleanMarker` marks top-level `new` and `call` expressions before simplification, (2) `CleanSideEffects` removes unmarked expressions after simplification (those that were assigned to removed variables). Only runs on client (`!is_server`). |
| Side effect import addition (SideEffectVisitor) | Inline strategy needs all imports present | Med | For Inline/Hoist strategies, adds missing side-effect imports (relative imports from `src_dir` that are in global imports but not in the current module). |
| DCE via SWC simplifier | Dead code after const replacement | Med | Runs `simplify::simplifier` with DCE config. `preserve_imports_with_side_effects: false` enables aggressive import removal. |
| `#__PURE__` annotations | Enables tree shaking in downstream bundlers | Low | `componentQrl()`, `qrl()`, `_jsxSorted()` calls are annotated with `/*#__PURE__*/` comments so bundlers can safely remove unused results. |

### 11. Input Binding Transformation

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `bind:value` to signal binding | Two-way data binding for inputs | Med | `<input bind:value={signal} />` generates both a value prop and an `onInput$` handler that updates the signal. Merges with existing `onInput$` handlers. |
| `bind:checked` to signal binding | Checkbox two-way binding | Med | Same as `bind:value` but for checkbox `checked` property. |
| `bind:*` generic handling | Extensible binding syntax | Low | Other `bind:*` attributes treated as regular props (e.g., `bind:stuff`). |
| Bind + existing handler merging | Both user handler and binding handler must fire | Med | When both `bind:value` and `onInput$` are present, they are merged into an array of handlers. Order-independent (works regardless of which comes first in JSX). |

### 12. Synchronous QRL (sync$)

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| `sync$` serialization | Synchronous event handlers for perf-critical cases | Med | `sync$((event, target) => event.preventDefault())` is serialized to a string representation. Comments are stripped. Supports function expressions and arrow functions. |

## Differentiators

Nice-to-have documentation. Valuable but the spec can function without these being fully specified.

| Feature | Value Proposition | Complexity | Notes |
|---------|-------------------|------------|-------|
| Custom inlined functions via `wrap()` | Allows user-defined `$`-suffixed APIs (e.g., `useMemo$`) | Med | `export const useMemo$ = wrap(useMemoQrl)` teaches the optimizer to treat `useMemo$` like a built-in. Diagnostic emitted if `wrap()` call not found. |
| `preserve_filenames` option | Debugging-friendly output | Low | When true, keeps original filenames instead of generating hash-based names. |
| `scope` option | Scoping for isolated builds | Low | Passed through to transform but minimally documented in tests. |
| `core_module` customization | Using optimizer with non-standard Qwik package names | Low | Defaults to `@qwik.dev/core`, can be overridden. |
| `dev_path` for source mapping | Alternative dev paths for dev mode source locations | Low | Allows `qrlDEV` file paths to use a different base path than the actual file path. |
| `_noopQrl` for stripped segments | Preserves QRL shape without implementation | Low | When a `$()` call is stripped (via `strip_ctx_name`), replaced with `_noopQrl("hash")` / `_noopQrlDEV("hash", {devInfo})` to maintain the API contract. |
| Parsed/pre-compiled QRL passthrough | Already-compiled QRLs are not re-transformed | Low | `inlinedQrl(fn, "hash")` patterns are recognized and not double-transformed. Null QRLs (`inlinedQrl(null, "hash")`) are ignored. |
| Source map generation | Debugging support | Med | Each segment and the main module get their own source maps. Maps are generated via SWC's codegen. |
| TS enum handling | TypeScript enums need special treatment | Low | When `transpile_ts: true`, enums are transpiled before the optimizer runs. When false, they pass through. |
| `@jsxImportSource` comment handling | React/Preact compat | Low | `/* @jsxImportSource react */` comment suppresses Qwik JSX transformation for that file. |
| Windows path normalization | Cross-platform development | Low | All backslashes converted to forward slashes in module paths. Verified by `support_windows_paths` test. |

## Anti-Features

Things to deliberately NOT include in the spec.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Internal SWC AST manipulation details | Implementation detail, not behavioral spec | Document input/output transformations, not the Rust/SWC visitor internals. |
| Exact source map content | Too volatile, changes with any code change | Spec that source maps ARE generated with correct mappings, not their exact content. |
| Specific hash values | Hashes depend on content, not a stable API | Spec the hashing properties (consistency, uniqueness) not specific hash outputs. |
| `MinifyMode::None` behavior specifics | Internal testing mode | Mention it exists, don't spec it in detail. |
| `EmitMode::Test` internal behavior | Test harness mode, not user-facing | Note it exists as the default test mode; skip detailed spec. |
| SWC version-specific behavior | May change with SWC upgrades | Spec the transformation intent, not SWC-specific output formatting. |
| Comment positioning in output | Cosmetic, not semantic | Don't spec where `/*#__PURE__*/` comments appear relative to whitespace. |
| Module ordering in output | Implementation detail of `TransformOutput` | Spec that modules are produced, not their sort order. |

## Feature Dependencies

```
Props Destructuring Optimization --> Derived Signal Optimization (signals need prop access rewriting first)
$() Extraction --> Capture Analysis (must detect captures to generate correct segments)
Capture Analysis --> Code Movement (segments need to know their captures for import generation)
Entry Strategy --> Code Movement (determines whether segments become separate files or inlined)
Emit Mode --> Dev Mode Instrumentation (qrlDEV vs qrl selection)
Emit Mode --> Const Replacement (Test mode skips const replacement)
Const Replacement --> DCE / Tree Shaking (const values enable dead branch elimination)
strip_ctx_name --> _noopQrl generation (stripped segments need placeholder QRLs)
JSX Transformation --> Event Handler Naming (onClick$ -> q-e:click happens during JSX transform)
JSX Transformation --> Derived Signal Optimization (signal wrapping happens during JSX prop analysis)
Var/Const Prop Classification --> _jsxSorted/_jsxSplit selection (spread props trigger _jsxSplit)
bind:value Transformation --> Event Handler Merging (bind generates handlers that may merge with existing)
RenameTransform --> Everything else (must run first to normalize import paths)
```

## MVP Spec Recommendation

### Phase 1: Core Segmentation (must-document-first)
1. `$()` extraction and segment generation
2. `component$` / `foo$` to `Qrl` conversion pattern
3. Capture analysis (scoped_idents, local_idents)
4. Segment naming and hash generation
5. Segment metadata (SegmentAnalysis) structure

### Phase 2: JSX and Reactivity
6. JSX transformation (`_jsxSorted` / `_jsxSplit`)
7. Event handler naming (`onClick$` to `q-e:click`)
8. Derived signal optimization (`_wrapProp` / `_fnSignal`)
9. Var vs const prop classification
10. Props destructuring optimization

### Phase 3: Build Configuration
11. Entry strategies (all 7)
12. Emit modes (Prod/Dev/Lib/Test)
13. Code stripping (strip_exports, strip_ctx_name, strip_event_handlers)
14. Const replacement (isServer/isBrowser/isDev)

### Phase 4: Supporting Features
15. Import management (rename, synthetic generation, dynamic imports)
16. Side effect analysis and cleanup
17. Input binding transformation (bind:value, bind:checked)
18. sync$ serialization
19. Source map generation

### Defer
- Custom inlined functions via `wrap()`: edge case, document after core
- `@jsxImportSource` handling: React interop, not core Qwik
- Windows path normalization: infrastructure concern, not transformation spec
- Parsed QRL passthrough: pre-compiled code handling, secondary concern

## Sources

All findings derived directly from source code analysis (HIGH confidence):

- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/lib.rs` -- Module list, public API, TransformModulesOptions
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/test.rs` -- 163 test cases covering all transformations (5388 lines)
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/parse.rs` -- Transformation pipeline order, EmitMode/MinifyMode definitions
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/transform.rs` -- Core QwikTransform, SegmentKind, SegmentData, prop classification
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/entry_strategy.rs` -- All 7 entry strategies and their grouping logic
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/const_replace.rs` -- isServer/isBrowser/isDev replacement
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/filter_exports.rs` -- strip_exports implementation (throwing stub)
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/clean_side_effects.rs` -- Treeshaker two-phase approach
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/add_side_effect.rs` -- SideEffectVisitor for import hoisting
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/code_move.rs` -- Segment file generation, capture restoration
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/inlined_fn.rs` -- `_fnSignal` generation for computed props
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/props_destructuring.rs` -- Props optimization (destructuring to raw props)
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/rename_imports.rs` -- @builder.io to @qwik.dev migration
- `/Users/jackshelton/dev/open-source/qwik-optimizer/swc-optimizer/core/src/snapshots/` -- 160+ snapshot files verifying exact output
