# Phase 2: Generate All Spec Files - Research

**Researched:** 2026-02-10
**Domain:** Parsing insta snapshot files, extracting code/metadata, generating OXC ASTs, writing structured markdown spec files
**Confidence:** HIGH

## Summary

Phase 2 is a documentation-generation task: read each of 162 insta snapshot files, parse their structured text format to extract input code, output modules, segment metadata, and diagnostics; then generate OXC parser ASTs for each code block using the Phase 1 utility; identify which of the 14 convention types are present in each snapshot's output; and write a structured markdown spec file.

The work is embarrassingly parallel -- each of the 162 spec files is completely independent. The snapshot file format is consistent and mechanically parseable: every snapshot (except `relative_paths`) begins with a YAML frontmatter block, followed by `==INPUT==` with the source code, followed by output modules separated by `===== <path> [flags]==` headers, each containing code, a source map line, and optionally a segment metadata JSON comment block, and ending with `== DIAGNOSTICS ==`. The test configuration for each snapshot must be extracted from `test.rs` (5387 lines, one function per test).

The primary bottleneck is AST generation via the `oxc-ast-util` binary: 162 input code blocks plus ~439 output modules = ~601 invocations. Each invocation is fast (parsing + serialization takes milliseconds) but the AST JSON is verbose (a simple component produces ~230+ lines of JSON, larger inputs produce thousands). ASTs must go in collapsible `<details>` blocks per SPEC-04/SPEC-06.

**Primary recommendation:** Process each snapshot by parsing the `.snap` file text, extracting the test config from `test.rs`, running `oxc-ast-util` on each code block, identifying conventions via pattern matching on the output code, and writing a markdown file following a consistent template. No external libraries needed -- this is text processing and file I/O that Claude performs directly.

## Standard Stack

### Core
| Tool | Location | Purpose | Why Standard |
|------|----------|---------|--------------|
| `oxc-ast-util` | `oxc-ast-util/target/release/oxc-ast-util` | Parse code to ESTree JSON | Built in Phase 1. Only external tool needed. |
| Snapshot files | `swc-optimizer/core/src/snapshots/qwik_core__test__<name>.snap` | Source data for each spec | 162 files, one per test. Authoritative source of truth. |
| `test.rs` | `swc-optimizer/core/src/test.rs` | Test configurations | 5387 lines. Each test function specifies `TestInput` overrides. |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| Bash (echo + pipe) | Pipe code to `oxc-ast-util` | Every AST generation: `echo '<code>' \| ./oxc-ast-util <ext>` |
| Python json.tool or similar | Validate AST JSON | Optional validation pass if AST output seems malformed |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Claude generating specs directly | A script/tool that generates specs | Roadmap explicitly chose Claude-generated specs. A script would need to parse snapshots, identify conventions, write prose -- essentially duplicating what Claude does well. |
| Running oxc-ast-util per code block | Batching all code blocks into a single invocation | The utility reads stdin, single invocation per code block. Batch mode would require modifying the utility (out of scope). Individual invocations are fast enough. |
| Generating ASTs for every code block | Skipping ASTs for known-unparseable code | Some output modules contain JSX without transpilation (raw TSX). The utility handles this -- just use the correct extension. Only truly malformed code (intentional error tests) should note parse failures. |

## Architecture Patterns

### Snapshot File Format (Verified from 20+ snapshot files)

Every snapshot file follows this structure:

```
---
source: packages/qwik/src/optimizer/core/src/test.rs
assertion_line: <line>
expression: output
[snapshot_kind: text]
---
==INPUT==

<input source code>

============================= <module_path> [flags]==

<module code>

Some("<source_map_json>")
[/*
<segment_metadata_json>
*/]
============================= <next_module_path> [flags]==
...
== DIAGNOSTICS ==

<diagnostics_json>
```

**Key observations:**
- Module separator: `=============================` followed by space, path, optional ` (ENTRY POINT)`, then `==`
- Source maps appear as `Some("...")` or `None` after each module's code (always present)
- Segment metadata appears as `/* { ... } */` after the source map line, only for entry point modules
- The main (non-entry-point) module is always `test.tsx` or `test.js` or `test.ts` (or a custom path like `components/main.js` for `relative_paths`)
- Diagnostics is always the last section, containing JSON array (usually `[]`)

**Exception:** `relative_paths` snapshot has NO `==INPUT==` section. It calls `snapshot_res!` directly with an empty prefix. Its input code must be extracted from `test.rs` lines 3338-3383.

### Test Configuration Extraction Pattern

Each test function in `test.rs` specifies a `TestInput` struct with overrides from the default:

```rust
// Default values (from TestInput::default()):
TestInput {
    filename: "test.tsx",
    dev_path: None,
    src_dir: "/user/qwik/src/",
    root_dir: None,
    entry_strategy: EntryStrategy::Segment,
    minify: MinifyMode::Simplify,
    transpile_ts: false,
    transpile_jsx: false,
    preserve_filenames: false,
    explicit_extensions: false,
    snapshot: true,
    mode: EmitMode::Test,
    scope: None,
    core_module: None,
    reg_ctx_name: None,
    strip_exports: None,
    strip_ctx_name: None,
    strip_event_handlers: false,
    is_server: None,
}
```

**Config fields relevant to spec documentation:**
| Field | Spec Impact | Common Non-Default Values |
|-------|-------------|---------------------------|
| `entry_strategy` | Determines how segments are extracted | `Segment` (default), `Inline`, `Single`, `Hoist`, `Smart`, `Component` |
| `mode` | Affects output naming, dev annotations | `Test` (default), `Dev`, `Prod`, `Lib` |
| `transpile_ts` | Whether TS is transpiled out | `true` in ~100 tests |
| `transpile_jsx` | Whether JSX is transpiled to function calls | `true` in ~100 tests |
| `is_server` | Server/client mode | `Some(true)`, `Some(false)` |
| `strip_exports` | Which exports get stubbed | `Some(vec!["onGet".into()])` |
| `strip_ctx_name` | Which `$`-suffixed functions get stripped | `Some(vec!["server".into()])`, `Some(vec!["useClientMount$".into()])` |
| `reg_ctx_name` | Custom registered context names | `Some(vec!["server".into()])` |
| `strip_event_handlers` | Whether event handlers are stripped | `true` in a few tests |
| `explicit_extensions` | Whether imports have file extensions | `true` in ~20 tests |
| `preserve_filenames` | Use original filenames in output | `true` in 2 tests |
| `dev_path` | Development source path | Custom path in 1 test |
| `minify` | Minification mode | `MinifyMode::Simplify` (default), `MinifyMode::None` in 1 test |

### Output Module Extension Determination

The file extension of each output module determines which extension to pass to `oxc-ast-util`:
- `.tsx` modules: pass `tsx` (default when no transpile flags)
- `.ts` modules: pass `ts` (when `transpile_jsx: true` but `transpile_ts: false`)
- `.js` modules: pass `js` (when both `transpile_ts: true` and `transpile_jsx: true`)
- `.mjs` modules: pass `mjs` (for the `relative_paths` test's lib.mjs)

The extension is visible in the module path header and also in the segment metadata's `"extension"` field.

### Convention Detection Patterns (HIGH confidence - verified from snapshot analysis)

Each convention type maps to specific textual patterns in the output code:

| Convention | Requirement | Detection Pattern | Example |
|------------|-------------|-------------------|---------|
| QRL calls | CONV-01 | `qrl(`, `qrlDEV(`, `inlinedQrl(` in output | `/*#__PURE__*/ qrl(i_HASH, "name")` |
| Dollar-to-Qrl | CONV-02 | `componentQrl(`, `useTaskQrl(`, `useStylesQrl(`, etc. | `component$` -> `componentQrl` |
| JSX transforms | CONV-03 | `_jsxSorted(`, `_jsxSplit(`, `_jsxQ(` in output | `_jsxSorted("div", null, {...}, ...)` |
| Signal helpers | CONV-04 | `_wrapProp(`, `_wrapSignal(`, `_fnSignal(`, `_getVarProps(`, `_getConstProps(` | `_wrapProp(store, "count")` |
| Capture patterns | CONV-05 | `_captures[`, capture array as 3rd arg to `qrl()` | `const state = _captures[0]` |
| Lazy imports | CONV-06 | `const i_HASH = () => import("./...")` | `const i_HTDRsvUbLiE = ()=>import("./test.tsx_Foo_component_HTDRsvUbLiE")` |
| PURE annotations | CONV-07 | `/*#__PURE__*/` before function calls | `/*#__PURE__*/ componentQrl(...)` |
| Segment extraction | CONV-08 | Entry point modules (code moved to separate file) | Module with `(ENTRY POINT)` flag |
| Code stripping | CONV-09 | `throw "Symbol removed by Qwik Optimizer"`, `_noopQrl(` | Strip exports, noop QRLs |
| Const replacement | CONV-10 | `isServer`, `isBrowser`, `isDev` replaced with literals | `if (true)` replacing `if (isServer)` |
| Props destructuring | CONV-11 | `_rawProps`, `_restProps(`, destructured params renamed | `(_rawProps) => { ... }` |
| Input binding | CONV-12 | `bind:value` -> `"value": signal`, `_val`, `_chk` | `"q-e:input": inlinedQrl(_val, ...)` |
| Sync$ serialization | CONV-13 | `_qrlSync(` with stringified function | `_qrlSync(function(...){...}, "stringified")` |
| Hoisted functions | CONV-14 | `const _hf0 = (p0) => ...` with `_hf0_str` | `const _hf0 = (p0)=>p0.value; const _hf0_str = "p0.value"` |

### Spec File Template

Based on the requirements (SPEC-01 through SPEC-10, QUAL-01):

```markdown
# Test: <test_name>

## Test Configuration

| Option | Value |
|--------|-------|
| Entry Strategy | <value> |
| Mode | <value> |
| Transpile TS | <value> |
| Transpile JSX | <value> |
| ... (only non-default values) |

## Input

### Source Code
\`\`\`tsx
<input code from ==INPUT== section>
\`\`\`

<details>
<summary>Input AST (OXC)</summary>

\`\`\`json
<oxc-ast-util output>
\`\`\`

</details>

## Output

### Module: <module_path> [ENTRY POINT]

\`\`\`javascript
<module code>
\`\`\`

<details>
<summary>Output AST (OXC)</summary>

\`\`\`json
<oxc-ast-util output>
\`\`\`

</details>

#### Segment Metadata
\`\`\`json
<segment JSON>
\`\`\`

### Module: <next_module_path>
...

## Conventions Applied

- **[CONV-XX] Convention Name**: <description of how it applies>

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| <name> | <which output module> | <where imported from> | <N> |

## Diagnostics

None (or JSON array of diagnostics)
```

### Anti-Patterns to Avoid
- **Generating ASTs inline in markdown without collapsible blocks:** ASTs are hundreds to thousands of lines. They MUST be in `<details>` blocks (SPEC-04, SPEC-06).
- **Including source maps in spec files:** Source maps are explicitly out of scope per REQUIREMENTS.md. Note their existence only.
- **Documenting exact hash values:** Hashes are content-dependent. Document hashing *properties* (hash exists, where it appears) not *values*.
- **Missing conventions:** The QUAL-02 requirement demands zero false negatives. Every convention present in a snapshot MUST be listed. Pattern-match against ALL 14 convention types for every output module.
- **Inconsistent template structure:** QUAL-01 and QUAL-04 require consistent, human-readable structure across all 162 files. Use the exact same heading hierarchy and section order.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| AST generation | Manual AST construction or pretty-printing | `oxc-ast-util` binary (Phase 1) | The utility is already built, tested, and handles all extensions. |
| Snapshot parsing | Complex regex or custom parser | Simple line-by-line text splitting on known delimiters | Snapshot format is regular and consistent. Split on `==INPUT==`, `=====`, `== DIAGNOSTICS ==`. |
| Convention detection | Manual reading of each snapshot | Pattern matching on known strings | The 14 conventions have distinct textual signatures (see table above). Search for each pattern in each output module. |

**Key insight:** This phase is text-processing and documentation writing. No compilation, no complex data structures. The snapshot format is designed for human readability and is trivially parseable by splitting on delimiter lines.

## Common Pitfalls

### Pitfall 1: Forgetting to Extract Test Config from test.rs
**What goes wrong:** A spec file documents the snapshot output but omits the test configuration, making it impossible to understand WHY the output looks the way it does (e.g., why is the output `.js` instead of `.tsx`?).
**Why it happens:** Test config is in `test.rs`, not in the snapshot file itself. Easy to overlook.
**How to avoid:** For EVERY spec file, look up the corresponding function in `test.rs` and document all non-default `TestInput` fields.
**Warning signs:** Spec file has no test configuration section, or lists "defaults" when the test actually overrides important fields.

### Pitfall 2: Shell Escaping When Piping Code to oxc-ast-util
**What goes wrong:** Code containing backticks, dollar signs, single quotes, or backslashes gets corrupted when piped through `echo` to the utility.
**Why it happens:** Bash interprets special characters. Qwik code heavily uses `$` (in `component$`, `$()`, etc.) and backticks (template literals).
**How to avoid:** Use heredoc or write code to a temp file, then pipe with `cat`. For heredoc: `cat <<'ENDOFCODE' | ./oxc-ast-util tsx`. The single-quoted `'ENDOFCODE'` prevents all expansion.
**Warning signs:** Parse errors from `oxc-ast-util` on code that should parse fine. Missing `$` characters in the input. Unexpected variable expansion.

### Pitfall 3: Wrong File Extension for AST Generation
**What goes wrong:** Passing `js` to `oxc-ast-util` for code that contains JSX produces parse errors (or wrong AST node types).
**Why it happens:** Output modules use `.js` extension when both `transpile_ts` and `transpile_jsx` are true, but the *input* code always contains JSX and TypeScript.
**How to avoid:** For input code, always use `tsx` (the input is always TypeScript + JSX). For output modules, use the extension from the module path header (`.js`, `.ts`, `.tsx`). Note: some `.js` output modules still contain JSX (when `transpile_jsx: false`) -- these should be parsed as `jsx`.
**Warning signs:** Parse errors containing "Unexpected token `<`" on output modules that visually contain JSX.

### Pitfall 4: Missing the `relative_paths` Edge Case
**What goes wrong:** The `relative_paths` snapshot has no `==INPUT==` section. A generic parser that expects `==INPUT==` in every snapshot will fail or produce wrong results.
**Why it happens:** This test calls `snapshot_res!` directly with an empty prefix instead of going through `test_input!`.
**How to avoid:** Handle `relative_paths` as a special case. Its input code is split across two variables in `test.rs` (lines 3338-3383): `dep` (a pre-transformed module) and `code` (the actual source). Document both.
**Warning signs:** The spec file for `relative_paths` has no input code section, or the parser errors on this snapshot.

### Pitfall 5: AST Generation on Unparseable Output Modules
**What goes wrong:** Some output modules contain intentionally broken or non-standard code that OXC cannot parse cleanly (e.g., output from `example_skip_transform` which has un-transpiled JSX with mixed contexts).
**Why it happens:** The optimizer's output may contain valid runtime code that is technically parseable, but edge cases exist.
**How to avoid:** Per QUAL-03, note parse errors rather than failing. The `oxc-ast-util` already handles recoverable errors (exit 0 + stderr) and panics (exit 2). If exit code is 2, note "AST: Parser error (see note)" in the spec file.
**Warning signs:** `oxc-ast-util` exits with code 2 or produces stderr output for a particular module.

### Pitfall 6: Overlooking Conventions That Appear Only in Non-Main Modules
**What goes wrong:** A spec file lists conventions from the main module but misses conventions in entry point modules (e.g., `_captures` pattern only appears in extracted segments, not the main module).
**Why it happens:** Scanning only the main module output, or scanning the input code instead of all output modules.
**How to avoid:** Scan ALL output modules for convention patterns. Many conventions (captures, segment extraction, lazy imports) appear exclusively in extracted entry point modules.
**Warning signs:** Convention count seems low. `_captures` never detected. Lazy imports not found.

## Code Examples

### Parsing a Snapshot File (Pseudocode)

```
1. Read the .snap file
2. Skip YAML frontmatter (between first and second '---' lines)
3. Find '==INPUT==' line -> everything after it until next '=====' line is input code
4. Split remaining content on '=====' separator lines
5. For each module section:
   a. Parse module path and flags from separator line
   b. Extract code (everything before 'Some(' or 'None' line)
   c. Extract source map (the 'Some("...")' or 'None' line)
   d. Extract segment metadata (/* { JSON } */ block if present)
6. Find '== DIAGNOSTICS ==' line -> everything after is diagnostics JSON
```

### Invoking oxc-ast-util with Heredoc (Safe for Special Characters)

```bash
# For input code (always tsx):
cat <<'ENDOFCODE' | ./oxc-ast-util/target/release/oxc-ast-util tsx
import { component$ } from '@qwik.dev/core';
export const App = component$(() => {
  return <div>{`template ${literal}`}</div>;
});
ENDOFCODE

# For output module (use extension from module path):
cat <<'ENDOFCODE' | ./oxc-ast-util/target/release/oxc-ast-util js
import { componentQrl } from "@qwik.dev/core";
export const App = /*#__PURE__*/ componentQrl(/*#__PURE__*/ qrl(i_HASH, "name"));
ENDOFCODE
```

### Convention Detection (Pattern Matching)

```
For each output module code:
  CONV-01: search for /qrl\(|qrlDEV\(|inlinedQrl\(/
  CONV-02: search for /componentQrl|useTaskQrl|useStylesQrl|...(any Qrl suffix)/
  CONV-03: search for /_jsxSorted\(|_jsxSplit\(|_jsxQ\(/
  CONV-04: search for /_wrapProp\(|_wrapSignal\(|_fnSignal\(|_getVarProps\(|_getConstProps\(/
  CONV-05: search for /_captures\[/ or qrl 3rd argument being an array
  CONV-06: search for /const i_\w+ = \(\) ?=> ?import\(/
  CONV-07: search for /\/\*#__PURE__\*\//
  CONV-08: if module is marked (ENTRY POINT) -> segment extraction occurred
  CONV-09: search for /"Symbol removed by Qwik Optimizer"|_noopQrl\(/
  CONV-10: search for const replacement of isServer/isBrowser/isDev (visible as literal true/false where identifiers were in input)
  CONV-11: search for /_rawProps|_restProps\(/
  CONV-12: search for /bind:|_val|_chk/ in const props context
  CONV-13: search for /_qrlSync\(/
  CONV-14: search for /const _hf\d+ = / (hoisted function pattern)
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Multi-phase thematic batching (8 phases for different test categories) | Single Phase 2 generating all 162 specs | 2026-02-10 roadmap revision | Each spec is independent work. No reason to split by transformation type. |
| CLI tool for spec generation | Claude generates specs directly | 2026-02-10 roadmap creation | Only small Rust utility for AST JSON. Claude handles all text processing, convention identification, and markdown writing. |

## Open Questions

1. **How to handle output modules that contain JSX but have `.js` extension?**
   - What we know: When `transpile_jsx: false` and `transpile_ts: true`, output modules get `.ts` extension. When both transpile flags are false (defaults), output is `.tsx`. When both are true, output is `.js`. But some tests set `transpile_jsx: true` yet the component segment's output still contains JSX-like `_jsxSorted` calls -- these are function calls, not JSX syntax, so `.js` parsing works fine.
   - What's unclear: Whether any `.js` output module contains actual JSX angle-bracket syntax (which would need `jsx` extension for parsing). Initial analysis suggests all JSX is transpiled to function calls when `transpile_jsx: true`.
   - Recommendation: Use the extension from the module path directly. If `oxc-ast-util` reports parse errors, try alternative extensions (e.g., `jsx` instead of `js`). Note any parse issues in the spec file per QUAL-03.

2. **How to determine CONV-10 (const replacement) when both input and output code are available?**
   - What we know: `isServer`, `isBrowser`, `isDev` from `@qwik.dev/core/build` get replaced with literal booleans based on `is_server` and `mode` config. This is visible by comparing input imports to output code.
   - What's unclear: In inline entry strategy tests where all code is in one module, the const replacement happens in-place and the import may be removed.
   - Recommendation: Check if the input imports from `@qwik.dev/core/build` and the output lacks those imports but contains literal booleans in the same positions. This convention is only present in tests that import from `@qwik.dev/core/build` (relatively few).

3. **Scale: Can all 162 specs be generated in a single plan?**
   - What we know: Each spec is independent. The roadmap suggests a single plan (02-01). But 162 files * multiple AST invocations per file is significant I/O.
   - What's unclear: Whether tool/context limits will require splitting into multiple plans.
   - Recommendation: Start with a single plan. The planner should batch tests into groups (e.g., 10-20 per task) to manage progress tracking. If a single plan proves unwieldy, split into 2-3 plans by test count (not by theme).

## Sources

### Primary (HIGH confidence)
- Snapshot files at `swc-optimizer/core/src/snapshots/qwik_core__test__*.snap` -- 162 files examined, 20+ read in full detail
- `swc-optimizer/core/src/test.rs` -- 5387 lines, all 163 test functions analyzed for configuration extraction
- `oxc-ast-util/src/main.rs` -- Phase 1 utility source (66 lines), verified working
- `oxc-ast-util/target/release/oxc-ast-util` -- Compiled binary (2.3MB), tested against example code
- `.planning/REQUIREMENTS.md` -- SPEC-01 through SPEC-10, CONV-01 through CONV-14, QUAL-01 through QUAL-03
- `.planning/ROADMAP.md` -- Phase 2 description, test list, success criteria
- `.planning/phases/01-oxc-ast-utility/01-VERIFICATION.md` -- Phase 1 verified complete, utility working

### Secondary (MEDIUM confidence)
- `.planning/phases/01-oxc-ast-utility/01-RESEARCH.md` -- OXC utility research, API details
- `.planning/phases/01-oxc-ast-utility/01-01-SUMMARY.md` -- Phase 1 implementation summary and decisions

### Tertiary (LOW confidence)
- Convention detection patterns are derived from manual analysis of ~20 snapshots. While the 14 convention categories are well-defined in REQUIREMENTS.md, edge cases in detection may exist. The complete set of patterns will only be validated once all 162 specs are generated and cross-checked in Phase 3.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Only tool is Phase 1 utility, already built and tested
- Architecture: HIGH - Snapshot format verified from 20+ files, parsing approach validated
- Pitfalls: HIGH - Shell escaping, extension mapping, and edge cases identified from actual data
- Convention detection: MEDIUM - Patterns identified for all 14 types, but some edge cases may exist in untested snapshots

**Research date:** 2026-02-10
**Valid until:** 2026-03-10 (stable -- snapshot files and test.rs are static, not changing)
