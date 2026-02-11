# External Integrations

**Analysis Date:** 2026-02-10

## APIs & External Services

**Qwik Framework Integration:**
- Qwik Core Module (`@builder.io/qwik`) - Primary integration target
  - Default module name: `BUILDER_IO_QWIK` constant in `swc-optimizer/core/src/words.rs`
  - Configurable via `core_module: Option<String>` in transform options (`swc-optimizer/src/plugins/plugin.ts:67`)
  - Used for identifying Qwik-specific directives and symbols

**Build Tool Plugins:**
- Vite Plugin API - Full integration as Vite plugin (`swc-optimizer/src/plugins/vite.ts`)
  - Provides: `resolveId`, `load`, `transform`, `handleHmrUpdate` hooks
  - HMR (Hot Module Replacement) support for development
  - Dev server integration via `ViteDevServer`
- Rollup Plugin API - Bundler integration (`swc-optimizer/src/plugins/rollup.ts`)
  - Module resolution and bundle analysis
  - Output bundle processing
  - Manifest generation from Rollup output

**ESLint Integration:**
- ESLint Plugin Framework - `swc-optimizer/src/plugins/eslint-plugin.ts`
  - Linting rules for Qwik code patterns
  - Custom rule enforcement in development

## Data Storage

**Not Detected** - This optimizer is a compiler/build tool with no persistent data storage layer.

**In-Memory Structures:**
- Bundle graph representation - `BundleGraphAdder` (`swc-optimizer/src/plugins/bundle-graph.ts`)
- Qwik manifest generation - `QwikManifest` and related types in `swc-optimizer/src/types.ts`
- Symbol mapping and analysis - `SegmentAnalysis`, `SymbolMapper` interfaces
- Source code AST representation - SWC AST structures

## File Storage

**Local Filesystem:**
- File reading: `fs.promises.readFile()` (Node.js environment)
- File writing: Handled by Vite/Rollup build system
- Path operations via abstracted `Path` interface (`swc-optimizer/src/path.ts`)
- Directory traversal: `fs.promises.readdir()` for input file collection

**Custom File System:**
- `OptimizerSystem.getInputFiles?: (rootDir: string) => Promise<TransformModuleInput[]>` - Customizable file input
- Allows overriding filesystem for testing or virtual filesystems
- Default implementation traverses directories for supported file extensions

## Caching

**Not Explicitly Detected** - No built-in cache layer.

**Optimization Opportunities:**
- SWC compiler already uses internal caching for parsed ASTs
- Manifest and symbol analysis cached during single build
- Hot Module Replacement (HMR) rebuilds only changed files via Vite/Rollup

## Authentication & Identity

**Not Applicable** - Build tool with no authentication requirements.

## Monitoring & Observability

**Error Tracking:**
- Diagnostic system via `Diagnostic` type (`swc-optimizer/src/types.ts`)
  - `DiagnosticCategory` enum: error levels
  - `SourceLocation` for error positioning
  - Collected during transformation: `diagnostics: Diagnostic[]` in `TransformOutput`

**Logging:**
- Console-based warnings in manifest generation:
  - `console.warn()` for skipped external imports (`swc-optimizer/src/manifest.ts`)
- Debug mode support via `debug?: boolean` option in plugin options (`swc-optimizer/src/plugins/plugin.ts:107`)

## CI/CD & Deployment

**Not Integrated** - This is a build tool meant to be integrated into CI/CD systems.

**Integration Points:**
- Vite integration handles dev and production builds
- Rollup integration for bundling
- Works in any Node.js-based CI/CD environment
- Platform-specific bindings loaded via `loadPlatformBinding()` at runtime

## Environment Configuration

**Required Options:**
- `srcDir: string` - Source directory to analyze
- `rootDir?: string` - Project root for relative path resolution
- `target?: 'client' | 'ssr'` - Build target (client-side or server-side rendering)
- `entryStrategy?: EntryStrategy` - Code splitting strategy

**Build Mode Options:**
- `buildMode?: 'development' | 'production'` - Optimization level
- `minify?: MinifyMode` - Minification strategy
- `sourcemap?: boolean` - Source map generation
- `transpileTs?: boolean` - TypeScript transpilation
- `transpileJsx?: boolean` - JSX transpilation

**Feature Configuration:**
- `experimental?: ExperimentalFeatures` - Feature flags
  - `preventNavigate` - usePreventNavigate hook
  - `valibot` - Valibot form validation
  - `noSPA` - Disable SPA navigation
  - `enableRequestRewrite` - Request.rewrite() support
  - `webWorker` - Worker support
  - `insights` - Qwik Insights plugin support

**Plugin Configuration:**
- `csr?: boolean` - Client-side rendering mode
- `lint?: boolean` - Enable linting
- `devTools?: boolean` - Enable dev tools
- `ssr?: { outDir?, manifestInput?, manifestInputPath? }` - Server build options
- `client?: { input?, manifestOutput? }` - Client build options

**Qwik-Specific Options:**
- `scope?: string` - CSS scope for components
- `stripExports?: string[]` - Exports to remove (e.g., server handlers)
- `stripCtxName?: string[]` - Context names to strip
- `stripEventHandlers?: boolean` - Remove event handlers
- `regCtxName?: string[]` - Context names to register
- `isServer?: boolean` - Mark as server environment

**File Extension Handling:**
- `explicitExtensions?: boolean` - Force explicit file extensions
- `preserveFilenames?: boolean` - Keep original filenames

## Platform-Specific Bindings

**Binding Resolution:**
- `QWIK_BINDING_MAP` maps platform identifiers to binding locations
- `loadPlatformBinding()` (`swc-optimizer/src/platform.ts`) - Dynamic binding loader
- Fallback to pure JavaScript/TypeScript if bindings unavailable
- Supports platform-specific optimization via native code

**Supported Runtimes:**
- `SystemEnvironment` detection in `getSystem()` (`swc-optimizer/src/platform.ts:11`)
- Node.js: Full feature set with filesystem and OS API access
- Web environments (WebWorker, BrowserMain): Limited to in-memory processing
- Deno/Bun: Alternative Node.js-compatible environments

## Webhooks & Callbacks

**Hot Module Replacement (HMR):**
- `handleHmrUpdate(ctx: HmrContext)` - Vite HMR callback (`swc-optimizer/src/plugins/vite.ts`)
- Triggers on file changes during development
- Updates module state and triggers Vite reload

**Custom Hooks:**
- `OptimizerSystem.dynamicImport: (path: string) => Promise<any>` - Module loading
- `OptimizerSystem.strictDynamicImport: (path: string) => Promise<any>` - Strict mode loading
- `SymbolMapperFn` - Custom symbol mapping function
- `BundleGraphAdder` - Custom bundle graph modification interface (`swc-optimizer/src/plugins/bundle-graph.ts`)

## Bundle Analysis & Manifest Generation

**Manifest System:**
- `QwikManifest` - Describes all optimized bundles and symbols
- `generateManifestFromBundles()` - Creates manifest from Rollup output (`swc-optimizer/src/manifest.ts`)
- `QwikBundle` - Individual bundle metadata
- `QwikSymbol` - Symbol export tracking
- `QwikBundleGraph` - Cross-bundle dependency graph

**Symbol Analysis:**
- `SegmentAnalysis` - Code segment analysis with entry strategy info
- `SymbolMapper` - Maps symbols to bundle locations
- Tracks captures, display names, context kinds (eventHandler/function)
- Canonical filenames and extension tracking

---

*Integration audit: 2026-02-10*
