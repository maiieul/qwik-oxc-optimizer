---
phase: 08-core-detection-qrl-transforms
verified: 2026-02-11T04:15:00Z
status: passed
score: 4/4 must-haves verified
re_verification: false
---

# Phase 08: Core Detection + QRL Transforms Verification Report

**Phase Goal:** Optimizer detects all $() call sites and produces qrl()/inlinedQrl() replacements
**Verified:** 2026-02-11T04:15:00Z
**Status:** PASSED
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
|---|-------|--------|----------|
| 1 | Optimizer identifies every $-suffixed call site imported from @qwik.dev/core in any input module | ✓ VERIFIED | collector.rs:119 checks `imported_name.ends_with('$')` and populates `dollar_imports` HashSet. Tests pass for component$, useTask$, and bare $ detection. |
| 2 | component$() becomes componentQrl(qrl(...)) and useTask$() becomes useTaskQrl(qrl(...)) (dollar-to-QRL suffix renaming works for all known APIs) | ✓ VERIFIED | transform.rs:316-329 wraps QRL calls with dollar_to_qrl_name(). Test test_transform_component_dollar verifies componentQrl(qrl(...)) output. words.rs:38-44 implements suffix replacement. |
| 3 | Segment entry strategy produces qrl(i_HASH, "segment_name") calls with correct hash and import identifier | ✓ VERIFIED | transform.rs:307-309 builds qrl(i_hash, name) calls. import_rewrite.rs:76-110 implements build_qrl_call(). hash.rs:24-39 computes 11-char deterministic hashes via DefaultHasher + base64url. All 9 hash tests pass. |
| 4 | Inline/hoist entry strategy produces inlinedQrl(fn_body, "name_HASH") calls with function body preserved | ✓ VERIFIED | transform.rs:274-305 implements inline strategy with build_inlined_qrl_call(). import_rewrite.rs:115-160 constructs inlinedQrl(body, name). Test test_transform_inline_strategy confirms inlinedQrl output. |

**Score:** 4/4 truths verified

### Plan 08-01 Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| crates/qwik-optimizer-oxc/src/parse.rs | Module parsing with OXC parser + SemanticBuilder scoping | ✓ VERIFIED | Function parse_module exists at line 57. Calls SemanticBuilder::new() at line 80. Returns ParseResult with scoping field. 7 unit tests pass. |
| crates/qwik-optimizer-oxc/src/collector.rs | First-pass AST analysis collecting dollar imports, call sites, module imports/exports | ✓ VERIFIED | Function collect exists at line 58. Two-pass approach: first collects imports (line 64), second walks for call sites. Returns CollectResult with dollar_imports HashSet. 7 unit tests pass. |
| crates/qwik-optimizer-oxc/src/hash.rs | Segment hash computation using DefaultHasher + base64url | ✓ VERIFIED | Function compute_segment_hash exists at line 24. Uses DefaultHasher (line 29) and base64url encoding (line 37). Produces 11-char hashes. 9 unit tests pass confirming determinism and format. |
| crates/qwik-optimizer-oxc/src/words.rs | Enhanced dollar API helpers with ctx_kind classification | ✓ VERIFIED | Function classify_ctx_kind exists (event$ -> EventHandler). Function is_qwik_core_import exists. Function dollar_to_qrl_name exists at line 38. 3 unit tests pass. |

### Plan 08-02 Required Artifacts

| Artifact | Expected | Status | Details |
|----------|----------|--------|---------|
| crates/qwik-optimizer-oxc/src/transform.rs | QwikTransform Traverse implementation with dollar detection and QRL wrapping | ✓ VERIFIED | impl Traverse<'a, ()> for QwikTransform at line 221. enter_call_expression detects dollar calls. exit_expression at line 246 replaces with QRL wrappers. exit_program at line 337 inserts imports. |
| crates/qwik-optimizer-oxc/src/import_rewrite.rs | Import change computation and new import construction | ✓ VERIFIED | build_qrl_call at line 76, build_inlined_qrl_call at line 115, build_named_import, build_lazy_import_declaration all present. compute_import_changes at line 28. 2 unit tests pass. |
| crates/qwik-optimizer-oxc/src/entry_strategy.rs | Enhanced entry strategy with inline/hoist detection | ✓ VERIFIED | should_inline function exists. should_hoist and needs_separate_file added. Tests verify correct strategy detection. |
| crates/qwik-optimizer-oxc/src/lib.rs | Wired transform_modules() pipeline returning real TransformOutput | ✓ VERIFIED | transform_modules at line 21 wires full pipeline: parse (line 82) -> collect (line 102) -> QwikTransform (line 105) -> traverse_mut (line 108) -> emit (line 117). Returns TransformOutput with real modules. 11 integration tests pass. |

### Plan 08-01 Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| crates/qwik-optimizer-oxc/src/parse.rs | oxc::parser::Parser | Parser::new().parse() | ✓ WIRED | parse.rs imports Parser, calls Parser::new() with allocator, source, source_type. Parse errors converted to Diagnostics. |
| crates/qwik-optimizer-oxc/src/parse.rs | oxc::semantic::SemanticBuilder | SemanticBuilder::new().build() | ✓ WIRED | parse.rs line 80: SemanticBuilder::new().with_excess_capacity(2.0).build(&program). Returns scoping in ParseResult. |
| crates/qwik-optimizer-oxc/src/collector.rs | crates/qwik-optimizer-oxc/src/words.rs | is_dollar_api import check | ✓ WIRED | collector.rs line 119: checks imported_name.ends_with('$') to identify dollar imports. Uses is_qwik_core_import to filter @qwik.dev/core imports. |
| crates/qwik-optimizer-oxc/src/hash.rs | base64::engine | base64url encoding of DefaultHasher output | ✓ WIRED | hash.rs line 37: base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash.to_le_bytes()). Dash/underscore replacement on line 38. |

### Plan 08-02 Key Link Verification

| From | To | Via | Status | Details |
|------|-----|-----|--------|---------|
| crates/qwik-optimizer-oxc/src/transform.rs | crates/qwik-optimizer-oxc/src/collector.rs | QwikTransform::new takes CollectResult from collector pass | ✓ WIRED | transform.rs QwikTransform::new signature accepts CollectResult. lib.rs line 105-106 passes collect_result to QwikTransform::new. |
| crates/qwik-optimizer-oxc/src/transform.rs | crates/qwik-optimizer-oxc/src/hash.rs | compute_segment_hash called during segment recording | ✓ WIRED | transform.rs calls hash::compute_segment_hash when recording segments. Hash included in SegmentData. |
| crates/qwik-optimizer-oxc/src/transform.rs | oxc_traverse::traverse_mut | traverse_mut runs QwikTransform over the AST | ✓ WIRED | lib.rs line 108-114: oxc_traverse::traverse_mut called with QwikTransform, allocator, mutable program, scoping. |
| crates/qwik-optimizer-oxc/src/lib.rs | crates/qwik-optimizer-oxc/src/parse.rs | transform_modules calls parse_module for each input | ✓ WIRED | lib.rs line 82: parse::parse_module(&allocator, source_in_arena, &input.path) called in per-input loop. Parse errors collected. |
| crates/qwik-optimizer-oxc/src/lib.rs | crates/qwik-optimizer-oxc/src/transform.rs | transform_modules creates QwikTransform and runs traverse_mut | ✓ WIRED | lib.rs line 105-114: QwikTransform::new creates transform instance, traverse_mut executes it. Segments extracted at line 131. |
| crates/qwik-optimizer-oxc/src/transform.rs | crates/qwik-optimizer-oxc/src/import_rewrite.rs | exit_program calls import rewrite to add/remove imports | ✓ WIRED | transform.rs exit_program calls build_named_import, build_lazy_import_declaration. New import statements prepended to program.body. |

### Requirements Coverage

No requirements explicitly mapped to Phase 08 in REQUIREMENTS.md. Phase implements CORE-01 through CORE-04 from ROADMAP.md success criteria.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|------|------|---------|----------|--------|
| crates/qwik-optimizer-oxc/src/lib.rs | 136-140 | Placeholder comment and empty code field for segment modules | ℹ️ Info | Expected for Phase 08. Phase 10 (code_move.rs) will generate segment code. Comment clearly documents this. Test test_transform_segments_recorded confirms segment metadata is recorded correctly. |

**Note:** The placeholder segment code is documented as intentional deferred work for Phase 10. The segment modules have correct metadata (SegmentAnalysis) but empty code field. This is by design and does not block Phase 08 goal achievement — the main module contains correct qrl(i_HASH, "name") calls that reference the segments.

### Human Verification Required

None. All phase goals are deterministic and verifiable via unit tests and code inspection. The transformations produce correct AST output that can be verified programmatically.

### Gaps Summary

**No gaps found.** All 4 observable truths verified, all artifacts substantive and wired, all key links functional. 60 unit tests + 3 spec tests pass (63 total). Phase 08 goal achieved.

---

**Implementation Highlights:**

1. **Parse Pipeline:** parse.rs correctly uses OXC Parser + SemanticBuilder to produce Program AST with Scoping. Source type detection handles .tsx, .ts, .jsx, .js extensions. Parse errors converted to Diagnostics.

2. **Dollar Detection:** collector.rs implements two-pass approach: first pass gathers all imports (building dollar_imports set), second pass recursively walks AST to find call sites. Handles JSX expressions via dedicated walk_jsx_expression_for_calls function (OXC 0.113 inherit_variants! pattern).

3. **QRL Wrapping:** transform.rs implements Traverse<'a, ()> with:
   - enter_call_expression: Detects dollar calls by checking callee against dollar_imports
   - exit_expression: Replaces dollar calls with qrl() or inlinedQrl() based on entry strategy
   - exit_program: Inserts new import declarations and lazy import constants

4. **Segment Strategy:** Produces qrl(i_HASH, "name_HASH") with lazy import constants const i_HASH = () => import("./path"). Hash computed via DefaultHasher + base64url + dash/underscore replacement.

5. **Inline Strategy:** Produces inlinedQrl(body, "name_HASH") with original function body preserved in first argument.

6. **Named Calls:** component$() and useTask$() wrapped with componentQrl(qrl(...)) and useTaskQrl(qrl(...)). The dollar_to_qrl_name function strips $ and appends Qrl.

7. **Import Rewriting:** New imports added to program.body: qrl, inlinedQrl, componentQrl, etc. All from @qwik.dev/core.

8. **Full Pipeline:** lib.rs wires parse -> collect -> transform -> emit for each input module. Segment modules created with metadata but empty code (deferred to Phase 10).

---

_Verified: 2026-02-11T04:15:00Z_
_Verifier: Claude (gsd-verifier)_
_Evidence: 63 passing tests, code inspection of all 8 modified files, wiring verification of full pipeline_
