# Codebase Structure

**Analysis Date:** 2026-02-10

## Directory Layout

```
qwik-optimizer/
├── swc-optimizer/           # Main optimizer package
│   ├── core/                # Rust compiler core (SWC-based)
│   │   ├── src/             # Rust source modules
│   │   ├── benches/         # Benchmark tests
│   │   └── Cargo.toml       # Rust package configuration
│   ├── src/                 # TypeScript/JavaScript bindings and plugins
│   │   ├── plugins/         # Build tool integrations
│   │   │   ├── dev/         # Development utilities
│   │   │   ├── vite.ts      # Vite plugin implementation
│   │   │   ├── rollup.ts    # Rollup plugin implementation
│   │   │   ├── plugin.ts    # Core plugin logic (shared)
│   │   │   ├── bundle-graph.ts   # Dependency graph optimization
│   │   │   └── *.unit.ts    # Plugin unit tests
│   │   ├── optimizer.ts     # Main optimizer factory
│   │   ├── platform.ts      # Runtime environment abstraction
│   │   ├── path.ts          # Cross-platform path utilities
│   │   ├── manifest.ts      # Build manifest generation
│   │   ├── types.ts         # Public type definitions
│   │   ├── index.ts         # Public API exports
│   │   └── versions.ts      # Version info
│   ├── api-extractor.json   # TypeScript API documentation config
│   └── global.d.ts          # Global TypeScript declarations
└── oxc-optimizer/           # Placeholder for OXC implementation (empty)
```

## Directory Purposes

**`swc-optimizer/`:**
- Purpose: Main Qwik optimizer implementation
- Contains: All build-time transformation logic
- Key files: `src/index.ts` (entry point), `src/optimizer.ts` (factory), `core/src/lib.rs` (Rust backend)

**`swc-optimizer/core/`:**
- Purpose: Rust implementation of code optimization
- Contains: SWC-based AST transformation, symbol analysis, code movement
- Key files: `Cargo.toml` (Rust dependencies), `src/lib.rs` (public API), `src/test.rs` (unit tests)
- Generated: `bindings/` directory (compiled WASM/native bindings) - not committed

**`swc-optimizer/core/src/`:**
- Purpose: Rust compiler modules
- Contains: Transformation passes and analysis utilities
- Key modules:
  - `lib.rs`: Main entry point, exports `transform_modules()`
  - `parse.rs`: AST parsing and code transformation
  - `collector.rs`: Symbol and dependency collection
  - `code_move.rs`: Code extraction and movement logic
  - `entry_strategy.rs`: Entry point strategy application
  - `props_destructuring.rs`: Component props handling
  - `filter_exports.rs`: Export filtering for server/client
  - `const_replace.rs`: Constant inlining
  - `is_const.rs`: Const evaluation utilities

**`swc-optimizer/src/`:**
- Purpose: TypeScript integration layer and build plugins
- Contains: Platform abstraction, plugin implementations, manifest generation
- Key roles: Bridge between Node.js/Browsers and Rust backend

**`swc-optimizer/src/plugins/`:**
- Purpose: Build tool integration
- Contains: Vite and Rollup plugin implementations
- Key files:
  - `plugin.ts` (1220 lines): Core plugin logic shared by both build tools
  - `vite.ts` (800 lines): Vite-specific hooks and configuration
  - `rollup.ts` (370 lines): Rollup adapter using core plugin
  - `bundle-graph.ts` (225 lines): Bundle dependency graph optimization
  - `eslint-plugin.ts`: ESLint integration for linting
  - `vite-utils.ts`, `utils.ts`: Utility functions
  - `vite.unit.ts`, `plugin.unit.ts`, `rollup.unit.ts`: Unit tests
  - `fixture-output-bundles.json`: Test fixtures for bundle graph tests

**`swc-optimizer/src/plugins/dev/`:**
- Purpose: Development-time utilities
- Contains: Dev server features and debugging tools
- Key files:
  - `image-size-server.ts`: Image preloading optimization
  - `index.ts`: Dev plugin coordination
  - `error-host.js`, `perf-warning.js`, `click-to-component.js`: Dev tools
  - `image-size-runtime.js`: Runtime image measurement

**`oxc-optimizer/`:**
- Purpose: Placeholder for future OXC-based implementation
- Status: Empty directory - no current implementation
- Note: OXC is alternative JavaScript optimizer (faster than SWC)

## Key File Locations

**Entry Points:**
- `src/index.ts`: Public API - exports `createOptimizer()`, types, plugin factories
- `src/optimizer.ts`: `createOptimizer()` factory function
- `src/plugins/vite.ts`: `qwikVite()` plugin factory
- `src/plugins/rollup.ts`: `qwikRollup()` plugin factory

**Configuration:**
- `core/Cargo.toml`: Rust dependencies (SWC, serde, rayon for parallel processing)
- `api-extractor.json`: TypeScript public API documentation
- `global.d.ts`: Module declaration for compiled string imports

**Core Logic:**
- `src/platform.ts`: Environment detection and binding loading
- `src/optimizer.ts`: Optimizer initialization and module transformation
- `src/types.ts`: All public TypeScript interfaces
- `src/manifest.ts`: QwikManifest generation and symbol prioritization
- `src/plugins/plugin.ts`: Core plugin business logic (file transformation, manifest generation)
- `core/src/lib.rs`: Rust transformation entry point
- `core/src/parse.rs`: Rust AST parsing and code generation

**Manifest & Bundle Management:**
- `src/plugins/bundle-graph.ts`: Bundle dependency graph with preload probabilities
- `src/manifest.ts`: Manifest building from transform outputs

**Testing:**
- `src/plugins/*.unit.ts`: Plugin unit tests (Vitest)
- `core/src/test.rs`: Rust unit tests with snapshot testing
- `core/src/snapshots/`: Snapshot test fixtures
- `core/src/fixtures/`: Test input files
- `src/plugins/fixture-output-bundles.json`: Test data for bundle graph

## Naming Conventions

**Files:**
- `*.ts` or `*.tsx`: TypeScript source files
- `*.unit.ts`: Unit tests (Vitest convention)
- `*.rs`: Rust source files
- `Cargo.toml`: Rust package manifest
- Files match module purpose: `optimizer.ts` (optimizer logic), `platform.ts` (platform abstraction)

**Directories:**
- Lowercase with hyphens: `src/plugins/dev/` (feature grouping)
- Core module: `core/` (Rust backend)
- Plugin modules: `plugins/` (build tool integrations)

**Exports:**
- Functions: camelCase (`createOptimizer`, `qwikVite`)
- Types: PascalCase (`Optimizer`, `QwikManifest`, `EntryStrategy`)
- Interfaces: PascalCase with prefix (`OptimizerSystem`, `TransformOutput`)
- Enums: PascalCase (`ExperimentalFeatures`)

**Imports:**
- Type imports use `type` keyword: `import type { Optimizer } from './types'`
- Circular dependency prevention: Plugin layer doesn't import from plugins
- Relative imports within module, absolute to public API

## Where to Add New Code

**New Build Tool Integration (e.g., Webpack):**
- Primary code: `src/plugins/webpack.ts` (mirror `vite.ts` structure)
- Shared logic: Leverage existing `createQwikPlugin()` from `plugin.ts`
- Tests: `src/plugins/webpack.unit.ts`
- Export: Add to `src/index.ts`

**New Transformation Pass (e.g., new code optimization):**
- Implementation: `core/src/new_transform.rs` (following SWC visitor pattern)
- Integration: Add module to `core/src/lib.rs` and call from `parse.rs`
- Tests: Add cases to `core/src/test.rs`

**New Development Utility:**
- Location: `src/plugins/dev/new-util.ts` or `.js` if runtime
- Integration: Export from `src/plugins/dev/index.ts`
- Plugin hook: Add to `createQwikPlugin()` dev-specific hooks

**New Utility Function:**
- Shared across module: `src/plugins/utils.ts` or domain-specific file
- Path utilities: `src/path.ts` (for cross-platform concerns)
- Platform-specific: `src/platform.ts`

**New Type Definition:**
- Public types: `src/types.ts`
- Plugin-specific: Define in corresponding plugin file, re-export from `src/types.ts`
- Entry strategy types: Already in `src/types.ts`, no new file needed

**New Test:**
- Unit tests: Co-located with source file (`.unit.ts` suffix)
- Integration tests: Create in `src/plugins/*.unit.ts`
- Rust tests: Add to `core/src/test.rs` with snapshot testing (insta crate)
- Test data: `src/plugins/fixture-*.json` for large fixtures

## Special Directories

**`core/src/snapshots/`:**
- Purpose: Snapshot test fixtures for Rust unit tests
- Generated: Automatically by `insta` crate during test runs
- Committed: Yes - snapshots are committed to verify test behavior
- Usage: Compare expected vs actual transformation output

**`core/src/fixtures/`:**
- Purpose: Test input files and expected outputs
- Contains: TypeScript/JavaScript fixtures for transformation testing
- Generated: Manually created for test cases
- Committed: Yes

**`bindings/`:**
- Purpose: Compiled native/WASM bindings (Rust outputs)
- Generated: During build via `cargo build`
- Committed: No - built for distribution
- Location: Published in npm distribution, loaded at runtime

**`.planning/codebase/`:**
- Purpose: Generated architecture documentation
- Contains: ARCHITECTURE.md, STRUCTURE.md, etc.
- Generated: By `/gsd:map-codebase` command
- Committed: Yes - reference documentation

## Build Configuration Details

**TypeScript:**
- Uses `tsconfig.json` (inferred - not visible but referenced by tools)
- Emits ES modules (ESM) for Node.js and browsers
- Target: ES2020+ with optional chaining, nullish coalescing

**Rust:**
- Edition 2021 (modern Rust)
- Key features: SWC for parsing, Rayon for parallelization, Serde for serialization
- Test framework: insta (snapshot testing)

**Output Structure:**
- Compiled Rust outputs to `bindings/` directory with platform-specific suffixes
- Native bindings: `bindings/qwik-{platform}-{arch}.node` (e.g., `qwik-win32-x64.node`)
- WASM fallback: `bindings/qwik.wasm.mjs` + `bindings/qwik_wasm_bg.wasm`

---

*Structure analysis: 2026-02-10*
