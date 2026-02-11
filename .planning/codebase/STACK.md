# Technology Stack

**Analysis Date:** 2026-02-10

## Languages

**Primary:**
- TypeScript - Main application code for TypeScript-based optimizer wrapper and plugins
- Rust - Core optimization engine (`qwik-core` library)

**Secondary:**
- JavaScript/JSX - For runtime compatibility and testing

## Runtime

**Environment:**
- Node.js - Primary runtime for platform implementation
- Browser (WebWorker, BrowserMain) - Supported via platform abstraction
- Deno - Supported environment
- Bun - Supported environment

**Package Manager:**
- Cargo (Rust) - `swc-optimizer/core/Cargo.toml`
- npm/pnpm/yarn (JavaScript) - Referenced through Vite/Rollup ecosystem

## Frameworks

**Core:**
- SWC (Speedy Web Compiler) - `swc_ecmascript`, `swc_common`, `swc_atoms` - AST transformation and code generation
- Vite - Build tool and dev server integration (`swc-optimizer/src/plugins/vite.ts`)
- Rollup - Module bundler integration (`swc-optimizer/src/plugins/rollup.ts`)

**Build/Dev:**
- Vitest - Test runner and framework (`*.unit.ts` test files)
- ESLint - Code linting integration (`swc-optimizer/src/plugins/eslint-plugin.ts`)
- API Extractor - TypeScript API documentation extraction (`swc-optimizer/api-extractor.json`)

## Key Dependencies

**Rust (Core Compiler):**
- `swc_ecmascript` v* - Provides JavaScript/TypeScript parsing, transformation, and codegen
  - Features: `codegen`, `utils`, `visit`, `parser`, `transforms`, `typescript`, `react`, `optimization`
- `swc_common` v* - Common utilities for SWC
  - Features: `sourcemap`
- `swc_atoms` v* - Atom interning for efficient string handling
- `serde` v1.0.160 - JSON serialization/deserialization
- `serde_json` v1.0.96 - JSON handling
- `serde_bytes` v0.11.9 - Binary data serialization
- `anyhow` v1.0.70 - Error handling
- `rayon` v1.7.0 - Data parallelism for multi-threaded optimization
- `lazy_static` v1.4.0 - Static initialization
- `base64` v0.22.1 - Base64 encoding for source maps
- `pathdiff` v0.2.1 - Path difference calculations
- `relative-path` v1.8.0 - Relative path handling
- `path-slash` v0.2.1 - Cross-platform path slashing
- `derivative` v2.2.0 - Derive macro utilities
- `simple-error` v0.3.0 - Error creation

**Testing (Rust):**
- `insta` v1.29.0 - Snapshot testing framework

**TypeScript/JavaScript:**
- `vite` - Build plugin development
- `rollup` - Module resolution and bundling
- `eslint` - Linting framework
- `vitest` - Unit test framework
- Node.js built-ins: `fs`, `path` (dynamic imports)

## Configuration

**Environment:**
- Supports multiple runtimes via `SystemEnvironment` type:
  - `'node'` - Full filesystem and OS access
  - `'deno'` - Deno runtime
  - `'bun'` - Bun runtime
  - `'webworker'` - Web Worker environment
  - `'browsermain'` - Browser main thread
  - `'unknown'` - Fallback for unknown environments

**Build Configuration:**
- TypeScript configuration via `tsconfigFileNames` option
- Vite configuration integration (`viteConfig: UserConfig`)
- Rollup options normalization for consistent bundle output
- Entry strategy configuration:
  - `ComponentEntryStrategy`
  - `HookEntryStrategy`
  - `InlineEntryStrategy`
  - `SingleEntryStrategy`
  - `SmartEntryStrategy`

**Source Maps:**
- Configurable via `sourcemap?: boolean` option
- External or inline format support
- SWC `sourcemap` feature enabled in core

**Code Transformation Options:**
- TypeScript transpilation: `transpileTs?: boolean`
- JSX transpilation: `transpileJsx?: boolean`
- Minification modes: `MinifyMode` enum
- Filename preservation: `preserveFilenames?: boolean`
- Explicit file extensions: `explicitExtensions?: boolean`

## Platform Requirements

**Development:**
- Rust 2021 edition (for core compiler)
- TypeScript support
- Node.js (primary development environment)
- Vite (dev server/build orchestration)

**Production:**
- Node.js runtime (default) or alternative runtimes (Deno, Bun, WebWorker, Browser)
- Compiled Rust bindings (platform-specific) - `loadPlatformBinding()` in `swc-optimizer/src/platform.ts`
- Precompiled WASM or native bindings for performance-critical operations

## Binding System

**Cross-language Bridge:**
- `QWIK_BINDING_MAP` (`swc-optimizer/src/qwik-binding-map.ts`) - Maps platform to native bindings
- `loadPlatformBinding()` - Dynamic loading of platform-specific compiled libraries
- Fallback to pure TypeScript implementation if bindings unavailable
- `OptimizerSystem` abstraction layer supports custom implementations

---

*Stack analysis: 2026-02-10*
