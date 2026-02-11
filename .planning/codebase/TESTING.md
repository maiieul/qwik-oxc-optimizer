# Testing Patterns

**Analysis Date:** 2026-02-10

## Test Framework

**Runner:**
- Vitest (version not specified in codebase)
- Config file: Not found in repository (inherited from parent or monorepo setup)

**Assertion Library:**
- Vitest built-in assertions: `expect()`, `assert`
- Also uses node assertions: `assert` from vitest (works with CommonJS-style assertions)

**Run Commands:**
```bash
# Example patterns observed - actual commands depend on parent monorepo
pnpm test              # Run all tests (inferred)
pnpm test --watch      # Watch mode (inferred)
pnpm test --coverage   # Coverage report (inferred)
```

## Test File Organization

**Location:**
- Co-located with source files using `.unit.ts` suffix
- Same directory as implementation (e.g., `plugin.ts` and `plugin.unit.ts` in same folder)
- Example paths:
  - `src/plugins/plugin.unit.ts` - tests for `src/plugins/plugin.ts`
  - `src/plugins/bundle-graph.unit.ts` - tests for `src/plugins/bundle-graph.ts`
  - `src/plugins/vite.unit.ts` - tests for `src/plugins/vite.ts`
  - `src/plugins/rollup.unit.ts` - tests for `src/plugins/rollup.ts`

**Naming:**
- `.unit.ts` extension for unit test files
- Filename matches source file (e.g., `plugin.ts` → `plugin.unit.ts`)

**Structure:**
```
swc-optimizer/src/
├── plugins/
│   ├── plugin.ts          # Source
│   ├── plugin.unit.ts     # Tests (co-located)
│   ├── vite.ts           # Source
│   ├── vite.unit.ts      # Tests (co-located)
│   └── ...
└── ...
```

## Test Structure

**Suite Organization:**
```typescript
import { assert, describe, expect, test } from 'vitest';

// Simple tests
test('defaults', async () => {
  const plugin = await mockPlugin();
  const opts = await plugin.normalizeOptions();
  assert.deepEqual(opts.target, 'client');
});

// Grouped tests with describe
describe('resolveId', () => {
  test('qrls', async () => {
    const plugin = await mockPlugin();
    expect(await plugin.resolveId(null!, 'foo', undefined)).toBeFalsy();
  });

  test('win32', async () => {
    const plugin = await mockPlugin('win32');
    expect(...).toHaveProperty(...);
  });

  test('libs', async () => {
    const plugin = await mockPlugin();
    expect(...).toHaveProperty(...);
  });
});
```

**Patterns:**
- Setup: Call `mockPlugin()` or `mockOptimizerOptions()` to create test instances
- Assertions: Mix of `assert.deepEqual()` and `expect()` styles
- Async tests: Extensively use `async/await`
- Context setup: Plugin initialization happens within each test

## Mocking

**Framework:** Vitest's built-in mocking capabilities

**Patterns:**
```typescript
// Mock optimizer options
function mockOptimizerOptions(): OptimizerOptions {
  return {
    sys: {
      cwd: () => process.cwd(),
      env: 'node',
      os: process.platform,
      dynamicImport: async (path) => import(path),
      strictDynamicImport: async (path) => import(path),
      path: path as any,
    },
    binding: { mockBinding: true },  // Use mock binding for tests
  };
}

// Mock plugin
async function mockPlugin(os = process.platform) {
  const plugin = createQwikPlugin({
    sys: { ... },
    binding: { mockBinding: true },
  });
  await plugin.init();
  return plugin;
}
```

**What to Mock:**
- File system operations via `OptimizerSystem.sys`
- Platform bindings via `binding: { mockBinding: true }`
- Plugin initialization context
- Rollup/Vite context objects (passed as `null!` or minimal objects with required methods)

**What NOT to Mock:**
- Core logic functions (test actual behavior)
- Type checking utilities
- Path operations (use real `node:path`)
- Actual configuration parsing

## Fixtures and Factories

**Test Data:**
```typescript
// Bundled data fixtures
import outputBundles from './fixture-output-bundles.json';

// Inline mock data
const fakeManifest = {
  bundles: {
    'app.js': { size: 0, total: 0, imports: ['static-dep.js', '@external-dep'] },
    'static-dep.js': { size: 0, total: 0, dynamicImports: [...] },
  },
  mapping: { sym1: 'dynamic-dep.js', sym2: 'has-a-symbol.js' },
  symbols: {},
  preloader: 'no-symbols.js',
  manifestHash: '123',
  version: '1.0.0',
} as QwikManifest;

// Test helper functions
function mockPlugin(os = process.platform) { ... }
function mockOptimizerOptions(): OptimizerOptions { ... }
```

**Location:**
- Fixtures co-located with test files (e.g., `fixture-output-bundles.json` next to `bundle-graph.unit.ts`)
- Mock/factory functions defined within test files as `function mockX()`
- No separate fixtures directory detected

## Coverage

**Requirements:** Not enforced (no coverage config found)

**View Coverage:**
```bash
# Not configured in codebase - would be:
pnpm test --coverage
```

## Test Types

**Unit Tests:**
- Individual function/method testing
- Input validation and default behavior
- Configuration normalization
- Path handling and cross-platform compatibility
- Scope: Single module or closely related functions
- Example: `plugin.unit.ts` tests `plugin.ts` functions in isolation

**Integration Tests:**
- Plugin behavior with mocked Vite/Rollup context
- Resolver ID chains and load sequences
- Manifest generation from bundle data
- Scope: Multiple components interacting
- Example: `resolveId` tests with context objects containing `resolve` methods

**E2E Tests:**
- Not used (would require full Vite/Rollup build)
- Manual testing via test starters mentioned in comments

## Common Patterns

**Async Testing:**
```typescript
test('defaults (buildMode: production)', async () => {
  const plugin = await mockPlugin();
  const opts = await plugin.normalizeOptions({ buildMode: 'production' });
  assert.deepEqual(opts.buildMode, 'production');
});

// Promise assertions
test('resolveId qrls', async () => {
  const plugin = await mockPlugin();
  await expect(
    plugin.resolveId(ctx, '/root/src/routes/layout.tsx_s_7xk04rim0vu.js', undefined)
  ).resolves.toHaveProperty('id', '/root/src/routes/layout.tsx_s_7xk04rim0vu.js');
});
```

**Error Testing:**
```typescript
// Expecting falsy values
test('resolveId', async () => {
  const plugin = await mockPlugin();
  expect(await plugin.resolveId(null!, 'foo', undefined)).toBeFalsy();
});

// Property checking
test('defaults', async () => {
  const plugin = await mockPlugin();
  const opts = await plugin.normalizeOptions();
  assert.deepEqual(opts.target, 'client');
  assert.deepEqual(opts.buildMode, 'development');
});
```

**Deep Equality:**
```typescript
// Objects and nested values
assert.deepEqual(opts.entryStrategy, { type: 'segment' });
assert.deepEqual((opts.input as string[]).map(normalizePath), [
  normalizePath(resolve(cwd, 'src', 'root')),
]);

// Expectation style
expect(convertManifestToBundleGraph(fakeManifest)).toEqual([
  'app.js',
  'static-dep.js',
  // ...
]);
```

**Platform-Specific Testing:**
```typescript
test('win32', async () => {
  const plugin = await mockPlugin('win32');  // Pass OS as parameter
  expect(
    await plugin.resolveId(ctx, 'C:\\src\\routes\\layout.tsx_s_7xk04rim0vu.js', undefined)
  ).toHaveProperty('id', 'C:/src/routes/layout.tsx_s_7xk04rim0vu.js');
});

// Platform detection in source
const isWin = process.platform === 'win32';
```

## Test Utility Functions

**From shared testing module (`../../../testing/util`):**
- `normalizePath(path)`: Normalize paths for cross-platform comparison (convert backslashes to forward slashes)
- Used extensively in assertions for path comparisons

**In-test helpers:**
```typescript
function mockPlugin(os = process.platform) { ... }
function mockOptimizerOptions(): OptimizerOptions { ... }
const getPlugin = (opts: QwikVitePluginOptions | undefined) =>
  (qwikVite(opts) as any)[0] as QwikVitePlugin;
```

## Mocking Rollup Context

**Minimal context objects:**
```typescript
// For optional context
const ctx = null! as Rollup.PluginContext;

// With resolve method
const ctx = {
  resolve: async () => ({ id: 'Yey' })
} as any;

// With custom resolver
const ctx = {
  resolve: (id: string, importer: string) => {
    // Custom resolution logic
    return { id: 'hi' };
  },
} as any;
```

## Test Data Organization

**Mock bundle data:**
```typescript
const chunkInfoMocks = [
  {
    exports: [''],
    name: 'chunk.tsx',
    facadeModuleId: 'chunk.tsx',
    moduleIds: ['chunk.tsx'],
  },
  // ...
] as Rollup.PreRenderedChunk[];

const noExternal = [
  '@qwik.dev/core',
  '@qwik.dev/core/internal',
  '@qwik.dev/core/server',
];
```

## Testing Best Practices Observed

1. **Normalize paths in assertions** - Always use `normalizePath()` for file path comparisons across platforms
2. **Test default values** - Extensive coverage of configuration defaults and overrides
3. **Async initialization** - Plugin must be initialized with `await plugin.init()` before testing
4. **Mock binding for tests** - Use `{ mockBinding: true }` to avoid native binding dependencies
5. **Use `assert.deepEqual` for object comparison** - Preferred for matching complex config objects
6. **Describe blocks for related tests** - Group related tests under `describe()` blocks
7. **Async test functions** - All plugin tests are async due to initialization

---

*Testing analysis: 2026-02-10*
