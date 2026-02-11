# Coding Conventions

**Analysis Date:** 2026-02-10

## Naming Patterns

**Files:**
- Test files: `.unit.ts` suffix for unit tests (e.g., `plugin.unit.ts`, `bundle-graph.unit.ts`)
- Source files: Plain `.ts` or `.tsx` extension
- Type definitions: Included in same file, no separate `.d.ts` convention except for declaration modules
- Constants: UPPER_SNAKE_CASE (e.g., `CLIENT_OUT_DIR`, `QWIK_BUILD_ID`, `TRANSFORM_EXTS`)
- Plugin files: Named by their framework (e.g., `vite.ts`, `rollup.ts`, `eslint-plugin.ts`)

**Functions:**
- camelCase naming convention (e.g., `createQwikPlugin`, `normalizeOptions`, `transformModules`)
- Getter functions: prefix with `get` (e.g., `getOptimizer`, `getOptions`, `getPath`, `getSys`)
- Factory functions: prefix with `create` (e.g., `createOptimizer`, `createQwikPlugin`, `createLinter`)
- Async functions: same camelCase, async nature indicated by `async` keyword
- Helper/utility functions: camelCase, often not exported unless part of public API

**Variables:**
- camelCase for local variables and parameters (e.g., `opts`, `fileFilter`, `bundleGraphAdders`)
- `let`/`const` preference: use `const` by default, `let` only when reassignment needed
- State maps: descriptive names (e.g., `clientResults`, `clientTransformedOutputs`, `parentIds`)
- Lazy initialized values: prefix with `lazy` (e.g., `lazyNormalizePath`)
- Optional/nullable: explicit `undefined` or `null` type annotations

**Types/Interfaces:**
- PascalCase for types, interfaces, and enums (e.g., `QwikManifest`, `OptimizerSystem`, `ExperimentalFeatures`)
- Public API types: marked with `/** @public */` JSDoc (e.g., `Optimizer`, `OptimizerOptions`, `TransformOutput`)
- Configuration types: suffix with `Options` (e.g., `QwikPluginOptions`, `NormalizedQwikPluginOptions`, `TransformModulesOptions`)
- Plugin API types: suffix with `Plugin` or include context (e.g., `QwikVitePlugin`, `QwikRollupPlugin`)
- Type aliases for function signatures use `type` keyword (e.g., `type ManualChunksOption = ...`)

## Code Style

**Formatting:**
- No explicit linting/formatting configuration found in codebase
- Uses TypeScript directly with node: prefix for Node imports (e.g., `import { resolve } from 'node:path'`)
- Type imports use `import type` syntax (e.g., `import type { HmrContext, Plugin, Rollup, ViteDevServer } from 'vite'`)
- Optional chaining (`?.`) and nullish coalescing (`??`) used liberally

**Linting:**
- ESLint is configured with comment-based suppressions
- Common suppressions: `/* eslint-disable no-console */` for development output
- Inline suppressions: `// eslint-disable-next-line no-console` for specific violations
- No global `.eslintrc` found but rules are enforced via comments

## Import Organization

**Order:**
1. Node.js built-in modules first with `node:` prefix (e.g., `import path from 'node:path'`)
2. External package imports (e.g., `vite`, third-party deps)
3. Type imports (with `type` keyword and from same sources)
4. Relative imports from parent directories (`../../../`)
5. Relative imports from same directory (`./`)

**Path Aliases:**
- No path aliases detected; uses relative paths extensively
- Deep relative paths used for cross-package imports (e.g., `../../../core/shared/utils/hash_code`)
- Virtual module identifiers as constants (e.g., `QWIK_BUILD_ID`, `QWIK_CLIENT_MANIFEST_ID`)

**Example Pattern:**
```typescript
import path, { resolve } from 'node:path';
import { assert, describe, expect, test } from 'vitest';
import { normalizePath } from '../../../testing/util';
import type { QwikManifest } from '../types';
import { createQwikPlugin } from './plugin';
```

## Error Handling

**Patterns:**
- Explicit `throw new Error('message')` for validation failures (e.g., `throw new Error('Qwik plugin has not been initialized')`)
- Early validation in functions: check conditions at start, throw if invalid
- Try-catch blocks for file system operations and dynamic imports
- Silent catch for optional operations: `catch { /* nothing */ }` or `catch { }`
- Error creation utility: `createRollupError(id, diagnostic)` for plugin-specific errors

**Examples:**
```typescript
if (!internalOptimizer) {
  throw new Error(`Qwik plugin has not been initialized`);
}

try {
  maybeFs = await internalOptimizer.sys.dynamicImport('node:fs');
} catch {
  console.log('node:fs not available, disabling automatic manifest reading');
  maybeFs = null;
}

const resolved = await ctx.resolve(origId, importerId);
if (!resolved) {
  throw new Error('Failed to resolve @qwik.dev/core/handlers.mjs');
}
```

## Logging

**Framework:** Direct `console` methods with ESLint suppressions

**Patterns:**
- `console.log()`: info messages in plugins (e.g., manifest reading)
- `console.debug()`: detailed debugging when `opts.debug` flag is enabled
- `console.warn()`: warning messages for non-fatal issues
- `console.error()`: error messages for diagnostics
- Prefix pattern: All plugin output prefixed with `[QWIK PLUGIN: ${id}]`
- Conditionally disabled: debug output guarded by `if (opts.debug) { console.debug(...) }`

**Example:**
```typescript
const debug = (...str: any[]) => {
  if (opts.debug) {
    // eslint-disable-next-line no-console
    console.debug(`[QWIK PLUGIN: ${id}]`, ...str);
  }
};
```

## Comments

**When to Comment:**
- JSDoc for public APIs: `/** @public */` marking on exported types, interfaces, functions
- Deprecation notices: `/** @deprecated ... */` for outdated APIs
- Complex logic: Comments explaining non-obvious decisions (e.g., "Vite likes to add ?v=1234... to the end of the id")
- Section headers: Large blocks divided by descriptive comments
- Temporary workarounds: Comments like "// Workaround to make the api be defined in the type." or "// V2 official release TODO:"

**JSDoc/TSDoc:**
- Used extensively for public APIs with `@public` tag
- Property descriptions: `/** description */` above each property
- Deprecation: `@deprecated` with explanation
- Access level: `@internal` for internal-only APIs
- Use in interface/type definitions, function declarations

**Example:**
```typescript
/**
 * Use `__EXPERIMENTAL__.x` to check if feature `x` is enabled. It will be replaced with `true` or
 * `false` via an exact string replacement.
 *
 * Add experimental features to this enum definition.
 *
 * @public
 */
export enum ExperimentalFeatures {
  /** Enable the usePreventNavigate hook */
  preventNavigate = 'preventNavigate',
}

/**
 * The types for Vite/Rollup don't allow us to be too specific about the return type.
 *
 * @public
 */
export function qwikVite(qwikViteOpts: QwikVitePluginOptions = {}): any {
```

## Function Design

**Size:**
- Generally concise, averaging 30-50 lines for main logic
- Large functions (100+ lines) used for complex configuration logic
- Private functions defined inside factory functions for encapsulation

**Parameters:**
- Configuration objects over multiple parameters (e.g., `opts: QwikPluginOptions`)
- Destructuring used in function bodies for clarity
- Async functions use promise-based returns
- Callback patterns for event handlers (e.g., `onDiagnostics` callback)

**Return Values:**
- Explicit type annotations on all public functions
- Promise returns for async operations
- Object returns for complex results containing multiple values
- Void returns with side effects clearly documented

**Example:**
```typescript
const normalizeOptions = async (
  inputOpts?: QwikPluginOptions
): Promise<NormalizedQwikPluginOptions> => {
  const updatedOpts: QwikPluginOptions = Object.assign({}, inputOpts);
  // ... validation and transformation
  const out = { ...opts };
  opts.input ||= updatedOpts.input as string[];
  return out;
};
```

## Module Design

**Exports:**
- Named exports for functions, types, constants (e.g., `export function createQwikPlugin(...)`)
- Type exports with `export type` for TypeScript types
- Single default export rarely used; named exports preferred
- Explicit public API exports with `/** @public */` markers

**Barrel Files:**
- `index.ts` files used to re-export from siblings (e.g., `src/index.ts` exports `createOptimizer`, `createQwikPlugin`)
- Core functionality in dedicated modules, aggregated in barrel files

**Example:**
```typescript
// Export from plugin.ts
export function createQwikPlugin(optimizerOptions: OptimizerOptions = {}) { ... }
export const QWIK_BUILD_ID = '@qwik.dev/core/build';
export interface QwikPluginOptions { ... }

// Re-exported in index.ts or consumed directly
```

## Async/Await Patterns

- Async initialization with lazy loading (e.g., `init()` must be called before use)
- Error handling: try-catch or `.catch(() => false)` for optional file system reads
- Map-based caching for expensive operations (e.g., `clientResults`, `clientTransformedOutputs`)

## Type Patterns

- Discriminated unions for strategy types (e.g., `EntryStrategy` with `type` field)
- Generic types with constraints for plugin APIs
- Type narrowing through type guards (e.g., `isServer` boolean to distinguish execution context)
- Spread operator used to avoid mutation: `Object.assign({}, inputOpts)`, `{ ...opts }`

---

*Convention analysis: 2026-02-10*
