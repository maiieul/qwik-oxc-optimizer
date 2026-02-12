---
phase: 16-targeted-fixes
verified: 2026-02-11T21:00:14Z
status: passed
score: 3/3 must-haves verified
re_verification: false
---

# Phase 16: Targeted Fixes Verification Report

**Phase Goal:** Known bug fixed and known performance bottleneck addressed
**Verified:** 2026-02-11T21:00:14Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #   | Truth                                                                                           | Status     | Evidence                                                                                              |
| --- | ----------------------------------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------- |
| 1   | minify_expression_string("a b") produces "a b" (space preserved between identifiers)            | ✓ VERIFIED | Line 1780 in transform.rs: `result.push(' ')` now in non-empty block; unit test passes                |
| 2   | KNOWN_GLOBALS is a HashSet<&str> with O(1) lookup, not a linear-scan slice                     | ✓ VERIFIED | collector.rs:42 defines `LazyLock<HashSet<&'static str>>`; all 3 call sites use HashSet API          |
| 3   | All 161 tests pass after both changes (zero regressions)                                       | ✓ VERIFIED | `cargo test` shows 161 tests passed (154 unit + 7 spec tests); includes 3 new minify unit tests      |

**Score:** 3/3 truths verified

### Required Artifacts

| Artifact                                                 | Expected                                                      | Status     | Details                                                                                                     |
| -------------------------------------------------------- | ------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------- |
| `crates/qwik-optimizer-oxc/src/transform.rs`             | Fixed minify_expression_string that preserves spaces          | ✓ VERIFIED | Line 1779-1780: `if prev_was_space && is_ident_char(c) { result.push(' '); }` — block now non-empty        |
| `crates/qwik-optimizer-oxc/src/transform.rs` (tests)     | Unit tests for minify_expression_string behavior              | ✓ VERIFIED | Lines 2722-2744: 3 new tests (preserves_ident_space, removes_extra_whitespace, preserves_strings) — all pass |
| `crates/qwik-optimizer-oxc/src/collector.rs`             | KNOWN_GLOBALS as LazyLock<HashSet<&str>>                      | ✓ VERIFIED | Lines 42-130: `static KNOWN_GLOBALS: LazyLock<HashSet<&'static str>>` with 87 entries                      |

### Key Link Verification

| From                  | To                              | Via                    | Status     | Details                                                                                                 |
| --------------------- | ------------------------------- | ---------------------- | ---------- | ------------------------------------------------------------------------------------------------------- |
| transform.rs:1440     | crate::collector::KNOWN_GLOBALS | .contains() call       | ✓ WIRED    | `KNOWN_GLOBALS.contains(root_name.as_str())` — HashSet API (no extra `&`)                              |
| transform.rs:1491     | crate::collector::KNOWN_GLOBALS | .contains() call       | ✓ WIRED    | `KNOWN_GLOBALS.contains(name)` — HashSet API (removed extra `&`)                                       |
| collector.rs:170      | KNOWN_GLOBALS                   | .contains() call       | ✓ WIRED    | `KNOWN_GLOBALS.contains(name.as_str())` — HashSet API                                                  |
| Unit tests            | minify_expression_string        | assert_eq! macro       | ✓ WIRED    | 3 tests covering space preservation, whitespace collapse, string literal handling — all passing        |

### Requirements Coverage

| Requirement                                                                              | Status      | Blocking Issue |
| ---------------------------------------------------------------------------------------- | ----------- | -------------- |
| BUG-01: minify_expression_string correctly preserves spaces between identifier characters| ✓ SATISFIED | None           |
| PERF-01: KNOWN_GLOBALS uses HashSet for O(1) lookup instead of linear scan              | ✓ SATISFIED | None           |

### Anti-Patterns Found

None. All "placeholder" occurrences (transform.rs:732, 749, 776, 978) are technical variable names in AST manipulation code, not actual placeholders.

### Commits Verified

| Commit  | Description                                                          | Status     |
| ------- | -------------------------------------------------------------------- | ---------- |
| ff7fa2a | fix(16-01): fix minify_expression_string dropping spaces             | ✓ VERIFIED |
| a78a45e | perf(16-01): convert KNOWN_GLOBALS to LazyLock<HashSet> for O(1)     | ✓ VERIFIED |

### Test Results

```
Unit tests (transform.rs):     154 passed (includes 3 new minify tests)
Spec tests:                     7 passed
Total:                          161 passed, 0 failed
```

**Note on test count:** The phase success criteria mention "165 tests" but the actual count is 161 tests. This discrepancy appears to be a documentation issue in ROADMAP.md (Phase 17 also mentions 165 tests). The SUMMARY.md correctly reports 161 tests, and all tests pass with zero regressions.

### Human Verification Required

None. All verification completed programmatically.

### Summary

Phase 16 goal **ACHIEVED**. Both cataloged issues are resolved:

1. **BUG-01 Fixed**: The minify_expression_string bug where spaces between identifiers were silently dropped is now fixed. The previously empty block body on line 1779 now correctly pushes a space character when needed. Three new unit tests confirm space preservation, whitespace collapse, and string literal handling all work correctly.

2. **PERF-01 Fixed**: KNOWN_GLOBALS converted from `const &[&str]` (O(n) linear scan) to `static LazyLock<HashSet<&'static str>>` (O(1) lookup). All three call sites updated to use HashSet API (removing extra `&` references). No performance regression detected.

3. **Zero Regressions**: All 161 tests pass (154 unit + 7 spec tests), confirming both fixes are correct and no existing behavior was broken.

The implementation exactly matches the plan with no deviations. Both artifacts are substantive and properly wired. All key links verified. Ready to proceed to Phase 17.

---

_Verified: 2026-02-11T21:00:14Z_
_Verifier: Claude (gsd-verifier)_
