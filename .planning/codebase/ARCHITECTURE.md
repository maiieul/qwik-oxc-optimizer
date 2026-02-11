# Architecture

**Analysis Date:** 2026-02-10

## Pattern Overview

**Overall:** Layered hybrid architecture combining TypeScript/Node.js frontend binding layer with a Rust core compiler backend, exposed through Vite and Rollup build plugins.

**Key Characteristics:**
- Platform-agnostic optimizer interface with environment detection (Node, Bun, Deno, Web Workers, Browser)
- Native binding fallback to WebAssembly compilation
- Plugin architecture for Vite and Rollup integration
- Code transformation pipeline: input modules → optimizer core → output segments with manifests
- Manifest-driven preloading with dynamic dependency probability calculation

## Layers

**Platform Layer (System Abstraction):**
- Purpose: Abstract away runtime environment differences and provide unified file system/path operations
- Location: `src/platform.ts`, `src/path.ts`
- Contains: Environment detection, dynamic imports, native binding loading, WASM initialization
- Depends on: Node.js APIs (fs, path, url), WebAssembly APIs
- Used by: Optimizer core, plugins

**Optimizer Core Layer:**
- Purpose: Main transformation engine that converts input modules into optimized output
- Location: `src/optimizer.ts`, `src/types.ts`
- Contains: Optimizer factory, transform options, input/output interfaces
- Depends on: Platform layer, Rust/WASM binding
- Used by: Plugins

**Rust Compiler Backend:**
- Purpose: Performs the actual code transformation, analysis, and optimization
- Location: `core/src/` (Rust module)
- Contains: Code parsing, entry strategy application, code movement, side effect analysis
- Key modules: `parse.rs` (AST transformation), `collector.rs` (symbol analysis), `code_move.rs` (code extraction), `entry_strategy.rs` (chunking logic)
- Depends on: SWC (Speedy Web Compiler) for parsing/codegen
- Used by: JavaScript binding via FFI

**Plugin Layer:**
- Purpose: Integrate optimizer into build tools (Vite, Rollup)
- Location: `src/plugins/`
- Contains: Plugin implementations, virtual module resolution, manifest generation, diagnostics
- Key files: `plugin.ts` (core plugin logic), `vite.ts` (Vite integration), `rollup.ts` (Rollup integration)
- Depends on: Optimizer core layer, manifest generation, bundle graph utilities
- Used by: Build tools via plugin API

**Manifest Generation Layer:**
- Purpose: Create build metadata for SSR and client runtime preloading
- Location: `src/manifest.ts`
- Contains: Symbol prioritization, manifest structure building, server manifest extraction
- Depends on: Bundle graph conversion
- Used by: Build plugins

**Bundle Graph Layer:**
- Purpose: Construct optimized dependency graph with preloading probabilities
- Location: `src/plugins/bundle-graph.ts`
- Contains: Dependency compaction, probability calculation for lazy imports, transitive dependency removal
- Depends on: QwikManifest types
- Used by: Manifest generation

**Development Utilities Layer:**
- Purpose: Provide dev-time features (image preloading, error handling, click-to-component)
- Location: `src/plugins/dev/`
- Contains: Dev server middleware, performance warnings, debugging tools
- Used by: Vite plugin

## Data Flow

**Build-Time Flow (Client Build):**

1. User runs build with Vite/Rollup + Qwik plugin
2. Plugin calls `createOptimizer()` → loads platform-specific binding (native or WASM)
3. Plugin normalizes Qwik options (entry strategy, build mode, optimization settings)
4. For each source module:
   - Plugin reads file contents
   - Calls `optimizer.transformModules()` with input batch
   - Rust backend parses as AST, analyzes symbols/dependencies, applies entry strategy
   - Returns transformed modules with segment analysis (metadata about code fragments)
5. Plugin collects all transformed modules into in-memory registry
6. On bundle generation:
   - Plugin generates `QwikManifest` from transformed outputs
   - Manifest includes: symbol mapping, bundle metadata, global injections
   - Bundle graph is computed from manifest to optimize preloading
   - Manifest written to disk (typically `q-manifest.json`)

**Build-Time Flow (SSR Build):**

1. SSR build uses same plugin but with `target: 'ssr'`
2. Optimizer applies server-specific transformations (strip client-only code)
3. Manifest input from client build is provided to ensure symbol consistency
4. Outputs server-optimized bundles

**Runtime Flow (Preloading):**

1. Server renders HTML, extracts symbols used on page
2. Uses bundle graph from manifest to compute transitive dependencies
3. Injects preload tags for all bundles needed (static + dynamic with probability)
4. Client uses preloaded bundles for near-instant interactivity activation

## Key Abstractions

**Optimizer:**
- Purpose: Single interface for all transformations
- Examples: `src/optimizer.ts`, type interface in `src/types.ts`
- Pattern: Factory pattern with lazy platform binding initialization

**OptimizerSystem:**
- Purpose: Runtime environment abstraction
- Examples: `src/platform.ts` (default system), user-provided custom systems
- Pattern: Dependency injection for pluggable environments

**EntryStrategy:**
- Purpose: Defines how code is split across bundles
- Examples: `smart`, `segment`, `single`, `component`, `hook`, `hoist`, `inline`
- Pattern: Polymorphic strategy with type union

**TransformOutput:**
- Purpose: Complete transformation result with modules and diagnostics
- Includes: Transformed code, source maps, segment analysis, build diagnostics
- Pattern: Immutable result object

**QwikManifest:**
- Purpose: Complete build metadata
- Includes: Symbol registry, bundle graph, asset metadata, global injections
- Used by: Server runtime, preloader, symbol mapper

**Virtual Modules:**
- Purpose: Inject special modules into build (build metadata, Qwik internals)
- Pattern: Rollup/Vite virtual module resolution
- Examples: `@qwik-build-id`, `@qwik-client-manifest`, `@qwik-core`

## Entry Points

**`src/index.ts`:**
- Location: Main library export
- Triggers: Package import by build tools or programmatic consumers
- Responsibilities: Re-export optimizer, versions, all public types and plugin factories

**`createOptimizer()`:**
- Location: `src/optimizer.ts`
- Triggers: Called during plugin initialization
- Responsibilities: Platform detection, binding initialization, Optimizer instance creation

**`qwikVite()`:**
- Location: `src/plugins/vite.ts`
- Triggers: Vite config.plugins array
- Responsibilities: Create Vite plugin instance with full hook implementations

**`qwikRollup()`:**
- Location: `src/plugins/rollup.ts`
- Triggers: Rollup plugins configuration
- Responsibilities: Create Rollup plugin instance with Vite-compatible core plugin

**`createQwikPlugin()`:**
- Location: `src/plugins/plugin.ts`
- Triggers: Called by both Vite and Rollup wrappers
- Responsibilities: Core plugin logic shared between build tools

## Error Handling

**Strategy:** Multi-layer diagnostic collection with categorized severity.

**Patterns:**

- Optimizer returns `Diagnostic[]` alongside successful transforms
- Diagnostics include: file, category (error/warning/sourceError), code, message, source locations, suggestions
- Build plugins convert diagnostics to build-tool-specific warnings/errors
- Rust backend uses `anyhow::Error` with source context
- Rollup integration uses `createRollupError()` to format diagnostics as Rollup errors

**Example:** `src/plugins/rollup.ts` lines 82-92 - diagnostic callback chains optimizer diagnostics into Rollup's warning system

## Cross-Cutting Concerns

**Logging:** No centralized logging framework. Debug output via `console.warn()` and `console.log()`. Vite plugin respects build tool verbosity.

**Validation:** Input validation in type definitions. Rust backend validates AST structure during parsing. Entry strategy types enforce valid combinations.

**Authentication:** Not applicable - offline transformer tool with no network access.

**Path Normalization:** Critical concern handled by abstracted `Path` interface (`src/path.ts`). All file paths normalized through this layer for cross-platform consistency.

**Source Maps:** Optional, controlled via `SourceMapsOption` ('external', 'inline', or undefined). Rust backend produces SWC-compatible source maps when enabled.

---

*Architecture analysis: 2026-02-10*
