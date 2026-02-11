# Codebase Concerns

**Analysis Date:** 2026-02-10

## Tech Debt

**Rollup Version Compatibility Checks:**
- Issue: Legacy version detection code for Rollup < 4.52.0 pending removal. Temporary workaround to check rollup version and conditionally set `onlyExplicitManualChunks` flag.
- Files: `swc-optimizer/src/plugins/rollup.ts:244-270`
- Impact: Code will fail to properly optimize preloading if Rollup < 4.52.0 is in use. Warning issued but older versions remain unsupported.
- Fix approach: Set minimum Rollup version requirement in package.json and remove conditional logic. Update documentation to reflect new minimum version.

**Noop Module Handling:**
- Issue: Comment indicates unimplemented handling for noop modules during transformation pipeline
- Files: `swc-optimizer/src/plugins/plugin.ts:822`
- Impact: Noop (no-operation) modules may not be properly handled in the segment transformation flow, potentially causing incorrect bundling
- Fix approach: Implement proper noop module detection and handling, or clarify why they're skipped

**useComputed$ Eager Loading Workaround:**
- Issue: Temporary workaround to eagerly load useComputed$ alongside qwikify$ and useVisibleTask$; marked for removal
- Files: `swc-optimizer/src/plugins/plugin.ts:1061-1062`
- Impact: Segments with useComputed$ are not code-split, increasing initial bundle size. Feature flag logic may become stale.
- Fix approach: Remove workaround once useComputed$ optimization is available and thoroughly tested

**Vite Integration Documentation Gap:**
- Issue: Documentation link missing in error message for Qwik deps marked as external
- Files: `swc-optimizer/src/plugins/vite.ts:721`
- Impact: Users encountering external dependency errors have no linked guidance, increasing support burden
- Fix approach: Add link to relevant documentation in error message

## Known Bugs

**Platform Binding Loading Fallback Chain:**
- Symptoms: Platform detection could silently fail and throw "Platform not supported" error at runtime without clear diagnostics
- Files: `swc-optimizer/src/platform.ts:107-155`
- Trigger: Running in unsupported platform (arm64, unsupported Linux architectures) or missing WASM binary
- Workaround: Explicit error message indicates platform not supported, but diagnosis is not user-friendly

**Package.json Reading Error Handling:**
- Symptoms: Silent catch-all when reading package.json to determine package scope; logged as warning only
- Files: `swc-optimizer/src/plugins/plugin.ts:304-330`
- Trigger: Malformed package.json, permission issues, or filesystem errors during traversal
- Workaround: Plugin continues with undefined scope, which may cause mangled bundle exports

## Security Considerations

**Dynamic Import Usage:**
- Risk: `dynamicImport()` and `strictDynamicImport()` used extensively for runtime loading of Node.js modules without validation
- Files: `swc-optimizer/src/platform.ts:14-22, 32`, `swc-optimizer/src/plugins/plugin.ts` (multiple locations)
- Current mitigation: Limited to known module paths (node:fs, node:url, node:path, node:module); no arbitrary module loading from user input
- Recommendations: Add module whitelist validation; document security assumptions; audit all dynamic import call sites for user-controlled paths

**WebAssembly Compilation:**
- Risk: WASM binary loaded from filesystem without integrity checks; WebAssembly.compile() could execute malicious code
- Files: `swc-optimizer/src/platform.ts:145`
- Current mitigation: WASM only loaded from bundled binaries within package
- Recommendations: Add SHA256 hash verification of WASM file before compilation; document supply chain security model

**Console Output Injection:**
- Risk: Error messages and diagnostics output directly to console without sanitization
- Files: `swc-optimizer/src/plugins/dev/index.ts:70`, `swc-optimizer/src/plugins/plugin.ts:327`, `swc-optimizer/src/manifest.ts:437`
- Current mitigation: User-controlled values are not included in console output; mostly framework-generated diagnostics
- Recommendations: Audit all console.warn/error/log calls for user input; use structured logging where possible

## Performance Bottlenecks

**Path Normalization Overhead:**
- Problem: Extensive use of `normalizePath()` throughout bundle transformation pipeline; implemented as custom POSIX path parser copied from Node.js v8.11.1
- Files: `swc-optimizer/src/path.ts` (661 lines), called from multiple transform points
- Cause: Every module ID must be normalized for consistency; custom implementation avoids Node.js path module in browser environments
- Improvement path: Consider caching normalized paths; profile usage to identify hot spots; evaluate if browser compatibility justifies custom implementation

**Rollup Output Bundle Graph Generation:**
- Problem: Manifest generation iterates multiple times over output bundles; bundle graph conversion filters dependencies in nested loops
- Files: `swc-optimizer/src/manifest.ts:403-500+`, `swc-optimizer/src/plugins/bundle-graph.ts:37-170`
- Cause: Multiple passes needed to determine bundle names, symbols, and dependencies; complexity increases with bundle count
- Improvement path: Consolidate into single pass where possible; use Map/Set for O(1) lookups; measure performance with large projects

**Module Resolution in Transform Pipeline:**
- Problem: Each transformed module may emit additional files via `ctx.emitFile()` and await loading via `Promise.all([...deps.values()].map((id) => ctx.load({ id })))`
- Files: `swc-optimizer/src/plugins/plugin.ts:829-845`
- Cause: Ensures Rollup cache is populated before continuing; can block if many segments are transformed
- Improvement path: Profile async concurrency limits; consider batching emitted files; investigate if cache warming is always necessary

## Fragile Areas

**Plugin Initialization State Management:**
- Files: `swc-optimizer/src/plugins/plugin.ts:87-120`
- Why fragile: Plugin maintains mutable Maps for clientResults, clientTransformedOutputs, serverTransformedOutputs, parentIds; shared across multiple transform calls without explicit invalidation strategy
- Safe modification: Always check if Map already contains key before adding; document initialization lifecycle; add assertions for expected state during transform
- Test coverage: Unit tests cover basic flow but do not test edge cases like plugin reinitialization or concurrent transforms

**Vite Configuration Integration:**
- Files: `swc-optimizer/src/plugins/vite.ts:60-400+`
- Why fragile: Plugin mutates config object in-place; caches values across multiple hook invocations (cachedPluginOpts); depends on Vite hook call order
- Safe modification: Never assume hook call order; validate all config values on every hook call; document assumptions about Vite version compatibility
- Test coverage: Integration tests exist but do not cover all Vite config combinations

**Manifest Format and Compatibility:**
- Files: `swc-optimizer/src/manifest.ts`, `swc-optimizer/src/plugins/bundle-graph.ts`
- Why fragile: Manifest JSON structure is tightly coupled to SSR runtime assumptions; bundle graph format uses compact array notation that is difficult to debug
- Safe modification: Maintain backward compatibility when changing manifest structure; add migration logic if format changes; include manifest version in generated JSON
- Test coverage: Unit tests cover generation but not validation against SSR runtime expectations

**Environment Detection Logic:**
- Files: `swc-optimizer/src/platform.ts:162-203`
- Why fragile: Runtime environment detection uses multiple typeof checks that can have false positives (e.g., checking for typeof WorkerGlobalScope without context)
- Safe modification: Add explicit environment feature tests; avoid relying on presence of globals that might exist in unexpected contexts
- Test coverage: No unit tests for environment detection; functions must be tested in multiple runtime contexts

## Scaling Limits

**Bundle Count Scaling:**
- Current capacity: Manifest generation uses nested loops over bundles and symbols; observable degradation likely above 1000+ bundles
- Limit: No explicit limits found; quadratic algorithms in dependency resolution could cause performance cliff
- Scaling path: Convert to single-pass algorithms; use memoization for repeated lookups; add performance monitoring for bundle count metrics

**Source File Count Scaling:**
- Current capacity: Plugin maintains Maps keyed by normalized file IDs; memory usage grows linearly with unique modules transformed
- Limit: No explicit limits; memory exhaustion possible with monorepos containing 10,000+ files
- Scaling path: Implement sliding window cache eviction; add memory usage monitoring; consider lazy transformation

**Rollup Output Size Scaling:**
- Current capacity: Manifest generation buffers all output in memory (manifest.bundles, manifest.symbols, manifest.mapping); each symbol adds metadata
- Limit: Manifest could exceed JSON parse limits in browsers or HTTP request limits if too large
- Scaling path: Consider streaming manifest generation; implement manifest compression; add manifest size monitoring

## Dependencies at Risk

**WebAssembly Binding Dependency:**
- Risk: Core transformation logic depends on external WASM binary compiled from Rust. Native Node.js bindings available only for darwin/win32/linux x64/arm64
- Impact: Unsupported architectures (arm32, MIPS, etc.) fall back to WASM or fail entirely. Binary incompatibility if SWC Rust version mismatches.
- Migration plan: Maintain version alignment with SWC releases; document supported platforms; consider pure JS fallback for simple transformations

**Vite Tight Coupling:**
- Risk: Multiple plugin hooks depend on undocumented Vite internals (moduleGraph, devServer properties, hook call order)
- Impact: Future Vite versions could break plugin functionality; workarounds (e.g., repl re-initialization) suggest fragile assumptions
- Migration plan: Reduce reliance on Vite internals; use only documented plugin API; maintain compatibility matrix with Vite versions

**Node.js Module System:**
- Risk: Code uses `module.createRequire(import.meta.url)` in ESM context; assumes CommonJS interop available in all Node versions
- Impact: Future Node versions may deprecate createRequire; other runtimes (Deno, Bun) may not support this pattern
- Migration plan: Add feature detection before using createRequire; provide alternative loading mechanism for ESM-only environments

## Missing Critical Features

**Error Recovery and Partial Builds:**
- Problem: Build failure in one module transformation may abort entire build; no mechanism to skip failing modules and continue
- Blocks: Partial builds, incremental builds with errors, graceful degradation in large monorepos

**Manifest Validation:**
- Problem: No validation that generated manifest matches expected structure or that SSR runtime can consume it
- Blocks: Early detection of manifest generation bugs; confidence that builds are valid before deployment

**Performance Profiling Integration:**
- Problem: No built-in metrics for transform time, bundling time, or manifest generation time
- Blocks: Identification of performance regressions; optimization of slow builds

## Test Coverage Gaps

**Environment Detection:**
- What's not tested: getEnv() function in different runtime contexts (Node, Bun, Deno, browsers, workers)
- Files: `swc-optimizer/src/platform.ts:162-203`
- Risk: Wrong environment detected in production, falling back to incorrect binding loading strategy
- Priority: High - affects core plugin initialization

**Rollup Version Detection:**
- What's not tested: Version parsing and comparison logic with edge cases (pre-release versions, malformed versions, missing package.json)
- Files: `swc-optimizer/src/plugins/rollup.ts:252-259`
- Risk: Incorrect version detection leading to wrong output options and suboptimal preloading
- Priority: Medium - only affects optimization, not correctness

**Platform Binding Fallback Chain:**
- What's not tested: Fallback from native binding to WASM when native not available; unsupported platform error conditions
- Files: `swc-optimizer/src/platform.ts:107-155`
- Risk: Silent failures or misleading error messages in unsupported environments
- Priority: High - affects usability on different platforms

**Manifest Generation Edge Cases:**
- What's not tested: External imports, missing bundles, symbols with no parent, duplicate exports, circular dependencies in bundle graph
- Files: `swc-optimizer/src/manifest.ts:403-500+`
- Risk: Corrupted manifests that SSR runtime cannot consume; missing symbols in preload lists
- Priority: High - affects production builds

**Virtual Module Resolution:**
- What's not tested: Virtual module ID parsing with edge cases; virtual module cache consistency across hot reloads
- Files: `swc-optimizer/src/plugins/vite.ts` (multiple virtual module handlers)
- Risk: Virtual module collisions or stale module references in dev server
- Priority: Medium - affects development experience

---

*Concerns audit: 2026-02-10*
