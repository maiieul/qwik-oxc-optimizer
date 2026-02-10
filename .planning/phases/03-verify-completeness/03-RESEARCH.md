# Phase 3: Verify Completeness - Research

**Researched:** 2026-02-10
**Domain:** Structural auditing and normalization of 162 markdown spec files
**Confidence:** HIGH

## Summary

Phase 3 is a verification and normalization task. All 162 spec files exist, but they were generated across 6 parallel plans using a mix of manual Claude generation (Plan 01) and Python script generation (Plans 02-06), resulting in significant structural inconsistencies. The success criteria require not just existence but structural consistency (same heading hierarchy, same section order) and convention completeness (no missing conventions).

The investigation identified 8 categories of structural inconsistency across the 162 files, ranging from cosmetic (backtick wrapping) to substantive (6 files with zero AST `<details>` blocks, potentially missing CONV-10 detection in several files). The core sections (`## Test Configuration`, `## Input`, `## Output`, `## Conventions Applied`, `## Function Calls in Output`, `## Diagnostics`) are present in all 162 files, so the structural fix is about normalizing formatting variations rather than adding missing sections.

The existing `gen-spec.py` Python script provides a canonical output format that can serve as the normalization target. However, gen-spec.py itself has a known gap: CONV-10 (const replacement) detection uses a `pass` statement and never fires. Convention completeness verification requires comparing each spec file's conventions list against its output code, which is automatable via the same regex patterns used in gen-spec.py.

**Primary recommendation:** Build a Python audit script that reads all 162 spec files, reports every structural inconsistency against a canonical format definition, detects missing conventions by re-running convention regex patterns against the output code sections, and optionally auto-fixes normalizable issues. Then fix all reported issues.

## Standard Stack

### Core
| Tool | Location | Purpose | Why Standard |
|------|----------|---------|--------------|
| Python 3 | System | Audit script for batch validation and normalization | Already used in Phase 2 (gen-spec.py). Regex, file I/O, JSON handling built-in. |
| gen-spec.py | `.planning/tools/gen-spec.py` | Reference implementation for canonical format and convention detection | Contains the 14 CONV pattern regexes and output format template |
| oxc-ast-util | `oxc-ast-util/target/release/oxc-ast-util` | Regenerate missing ASTs if needed | Phase 1 utility, already built and working |
| Snapshot files | `swc-optimizer/core/src/snapshots/` | Source of truth for convention verification | 162 files, one per test |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| Bash (diff, wc) | Quick verification counts | Final validation of 162-file count, section counts |
| grep/awk | Ad-hoc pattern checks | Spot-checking audit script results |

## Architecture Patterns

### Canonical Spec File Format (Normalization Target)

Based on analysis of gen-spec.py output and the Phase 2 plan template, the canonical format is:

```
# Test: <test_name>

## Test Configuration

| Option | Value |
|--------|-------|
| <Key> | <value> |                    # Table format always, even for all-defaults
                                       # Values NOT backtick-wrapped

## Input

### Source Code

```tsx
<input code>
```

<details>
<summary>Input AST (OXC)</summary>

```json
<AST JSON or "AST omitted for brevity. Parse with: `cat <<'EOF' | ./oxc-ast-util/target/release/oxc-ast-util tsx`">
```

</details>

## Output

### Module: <path> (ENTRY POINT)       # Parentheses, NOT square brackets
                                        # NO backtick wrapping on path
                                        # NO extra annotations like "(main)" or "(main module)"

```<language>
<code>
```

<details>
<summary>Output AST (OXC)</summary>

```json
<AST JSON or "AST omitted for brevity...">
```

</details>

#### Segment Metadata                   # Present for ALL entry point modules
```json
<segment metadata JSON>
```

## Conventions Applied

- **[CONV-XX] Convention Name**: <description>
                                        # OR: *No optimizer conventions detected in output.*

## Function Calls in Output

| Function | Module | Import Source | Count |
|----------|--------|--------------|-------|
| `<name>` | <module> | <source> | <N> |

## Diagnostics

```json
<diagnostics JSON or []>
```
```

### Identified Inconsistencies (Quantified Audit)

| Inconsistency | Files Affected | Canonical Format | Variants Found |
|---|---|---|---|
| **AST inclusion** | 6 files | `<details>` block with AST or "omitted" placeholder | Zero `<details>` blocks: `example_dead_code`, `example_dev_mode`, `example_dev_mode_inlined`, `example_noop_dev_mode`, `example_prod_node`, `example_server_auth` |
| **AST omitted vs full** | 48 files | Either full AST or "omitted for brevity" placeholder is acceptable | Both patterns coexist -- this is acceptable, not an inconsistency to fix |
| **Module path backticks** | ~89 module headings | `### Module: path` (no backticks) | `### Module: \`path\`` (backtick-wrapped) |
| **ENTRY POINT notation** | 45 module headings | `(ENTRY POINT)` (parentheses) | `[ENTRY POINT]` (square brackets) |
| **Config table: all defaults** | 12 files | Table with `| *(all defaults)* | |` row | Prose: "All defaults (Entry Strategy: Segment, Mode: Test, no transpilation)" |
| **Config value backticks** | ~55 files | Values without backticks: `| Transpile TS | true |` | Values with backticks: `| Transpile Ts | \`True\` |` |
| **Diagnostics format** | ~79 files | `\`\`\`json\n[]\n\`\`\`` (JSON code block) | "No diagnostics.", "None (`[]`)", bare "[]", prose variants |
| **Module annotations** | ~33 files | No extra annotation | "(main)", "(main module)", "(main, Inline -- single module)" |
| **Conventions: no-match text** | 4 files | `*No optimizer conventions detected in output.*` | "None --", "**None**" |
| **Key behavior notes** | 65 files | Not in canonical format (but adds value) | `**Key behavior:**` or `**Key dead code behavior:**` free-form notes |
| **Segment Metadata for ENTRY POINTs** | 1 file | Present for every ENTRY POINT module | `example_server_auth.md` has 2 ENTRY POINT modules but 0 Segment Metadata sections |
| **CONV-10 detection gap** | ~3 files | Should detect isServer/isBrowser/isDev const replacement | gen-spec.py has `pass` for CONV-10; manual specs may have caught it in some cases but not all |
| **relative_paths** | 1 file | Unique: dual-input test with `### Input Modules` subsection | Not a bug -- this test genuinely has different structure. Document as acceptable exception. |

### Pattern: Two-Pass Verification

**Pass 1: Structural audit (automatable)**
- Section headings present and in correct order
- Module heading format normalized
- ENTRY POINT notation consistent
- Config table format consistent
- Diagnostics format consistent
- `<details>` blocks present for all code blocks
- Segment metadata present for all ENTRY POINT modules

**Pass 2: Convention completeness (automatable)**
- For each spec file, extract all output code
- Run the 14 CONV regex patterns against the output code
- Compare detected conventions against the `## Conventions Applied` section
- Report any convention present in code but missing from the list
- Special attention to CONV-10 (known gap in gen-spec.py)

### Anti-Patterns to Avoid

- **Regenerating all 162 files from scratch:** The existing spec files contain valuable human-written convention descriptions (especially the `**Key behavior:**` notes) that would be lost. Fix inconsistencies in-place, do not regenerate.
- **Making AST inclusion mandatory:** 48 files use "omitted for brevity" and that is acceptable per the QUAL-01 requirement that specs be human-readable without expanding AST details. The 6 files with zero `<details>` blocks need at least an "omitted" placeholder added.
- **Over-normalizing the relative_paths spec:** This test genuinely uses a different API (`transform_modules` with two input modules) and its spec structure legitimately differs. Document the exception rather than force-fitting it.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Convention detection | Custom per-file analysis | Reuse regex patterns from `gen-spec.py` `detect_conventions()` | Already tested against 162 files, covers all 14 CONV types (except CONV-10 gap which needs a one-line fix) |
| Spec file parsing | Custom markdown parser | Simple regex/split on `## ` headings | Spec files have predictable heading-based structure, no need for a full markdown parser |
| Batch file processing | Manual file-by-file Claude editing | Python script iterating over all 162 files | Consistency requires identical logic applied to every file; manual editing introduces new inconsistencies |

**Key insight:** The gen-spec.py script's `detect_conventions()` function is the closest thing to a "canonical convention detector." Fix its CONV-10 gap, then use it as the verification oracle.

## Common Pitfalls

### Pitfall 1: Introducing New Inconsistencies During Fixes
**What goes wrong:** Manually fixing files one-by-one introduces new formatting variations
**Why it happens:** Different Claude context windows, different prompts, subtle formatting drift
**How to avoid:** Use a script to apply ALL formatting normalization in a single deterministic pass
**Warning signs:** Running the audit script after fixes still reports inconsistencies

### Pitfall 2: Destroying Valuable Convention Descriptions
**What goes wrong:** Normalizing the Conventions Applied section strips the human-written descriptions and replaces them with generic auto-detected text
**Why it happens:** The audit script auto-generates convention text; if used to overwrite rather than supplement, detailed descriptions are lost
**How to avoid:** Only ADD missing conventions; never replace existing convention descriptions. The audit script should detect missing conventions, not rewrite existing ones.
**Warning signs:** Convention descriptions become generic ("Output uses `qrl()` calls") instead of specific ("The Inline entry strategy combined with strip_event_handlers produces...")

### Pitfall 3: False Negatives on CONV-10
**What goes wrong:** CONV-10 (const replacement: `isServer`/`isBrowser`/`isDev` replaced with literal `true`/`false`) is not detected because gen-spec.py has a `pass` for it
**Why it happens:** CONV-10 detection requires comparing input vs output to see if the identifier was replaced with a literal. The gen-spec.py script could not determine this from output code alone.
**How to avoid:** For CONV-10 verification, check if `isServer`/`isBrowser`/`isDev` appears in the INPUT code and the OUTPUT code replaces it with `true`/`false` literal. Files to check: `example_build_server`, `example_prod_node`, `example_dev_mode`, `example_dev_mode_inlined`, `example_noop_dev_mode`, `example_strip_server_code`, `example_strip_client_code`, `example_server_auth`.
**Warning signs:** Test names containing "server", "prod", "dev_mode" with no CONV-10 in their conventions list

### Pitfall 4: Treating Structural Normalization as Semantic Change
**What goes wrong:** Changing `[ENTRY POINT]` to `(ENTRY POINT)` is treated as a semantic concern when it is purely cosmetic
**Why it happens:** Over-caution about modifying generated spec files
**How to avoid:** Clearly distinguish cosmetic normalization (heading format, backtick wrapping, diagnostics format) from semantic fixes (missing conventions, missing segment metadata, missing AST details blocks). Apply cosmetic fixes via automated script without concern.

## Code Examples

### Convention Detection (from gen-spec.py)

The authoritative regex patterns for all 14 conventions:

```python
# Source: .planning/tools/gen-spec.py lines 181-293
# CONV-01: QRL calls
re.search(r'\bqrl\(', code) or re.search(r'\bqrlDEV\(', code) or re.search(r'\binlinedQrl\(', code)

# CONV-02: Dollar-to-Qrl
qrl_suffixes = re.findall(r'\b(\w+Qrl)\(', code)
dollar_qrls = [q for q in qrl_suffixes if q not in ('qrl', 'qrlDEV', 'inlinedQrl') and q.endswith('Qrl')]

# CONV-03: JSX transforms
re.search(r'_jsxSorted\(', code) or re.search(r'_jsxSplit\(', code) or re.search(r'_jsxQ\(', code) or re.search(r'_jsxC\(', code)

# CONV-04: Signal helpers
re.search(r'_wrapProp\(', code) or re.search(r'_wrapSignal\(', code) or re.search(r'_fnSignal\(', code) or re.search(r'_getVarProps\(', code) or re.search(r'_getConstProps\(', code)

# CONV-05: Capture patterns
re.search(r'_captures\[', code)

# CONV-06: Lazy imports
re.search(r'const i_\w+\s*=\s*\(\)\s*=>\s*import\(', code)

# CONV-07: PURE annotations
re.search(r'/\*#__PURE__\*/', code)

# CONV-08: Segment extraction -- check for (ENTRY POINT) modules
entry_modules = [m for m in modules if m['is_entry']]

# CONV-09: Code stripping
'"Symbol removed by Qwik Optimizer"' in code or re.search(r'_noopQrl\(', code)

# CONV-10: Const replacement -- NEEDS FIX (currently `pass` in gen-spec.py)
# Correct detection: check if isServer/isBrowser/isDev in INPUT is replaced with true/false in OUTPUT
# Simple heuristic: if output contains `true` or `false` where input had `isServer`/`isBrowser`/`isDev`

# CONV-11: Props destructuring
re.search(r'_rawProps', code) or re.search(r'_restProps\(', code)

# CONV-12: Input binding
'bind:' in code or re.search(r'\b_val\b', code) or re.search(r'\b_chk\b', code)

# CONV-13: Sync$ serialization
re.search(r'_qrlSync\(', code)

# CONV-14: Hoisted functions
re.findall(r'const _hf(\d+)\s*=', code)
```

### Audit Script Structure (Recommended)

```python
#!/usr/bin/env python3
"""Audit all 162 spec files for structural consistency and convention completeness."""

import os, re, json, sys

SPEC_DIR = ".planning/spec"
SNAP_DIR = "swc-optimizer/core/src/snapshots"

def audit_structure(filepath, content):
    """Check structural consistency against canonical format."""
    issues = []

    # Required sections in order
    required_sections = [
        "## Test Configuration",
        "## Input",
        "## Output",
        "## Conventions Applied",
        "## Function Calls in Output",
        "## Diagnostics",
    ]

    for section in required_sections:
        if section not in content:
            issues.append(f"MISSING SECTION: {section}")

    # Check section order
    positions = [content.find(s) for s in required_sections if s in content]
    if positions != sorted(positions):
        issues.append("SECTIONS OUT OF ORDER")

    # Check module heading format
    for match in re.finditer(r'^### Module: (.+)$', content, re.MULTILINE):
        heading = match.group(1)
        if heading.startswith('`'):
            issues.append(f"BACKTICK MODULE PATH: {heading}")
        if '[ENTRY POINT]' in heading:
            issues.append(f"SQUARE BRACKET ENTRY POINT: {heading}")
        if '(main' in heading.lower():
            issues.append(f"EXTRA MODULE ANNOTATION: {heading}")

    # Check for <details> blocks
    if '<details>' not in content:
        issues.append("NO AST DETAILS BLOCKS")

    # Check config format
    config_section = re.search(r'## Test Configuration\n\n(.*?)(?:\n## )', content, re.DOTALL)
    if config_section and 'All defaults' in config_section.group(1) and '|' not in config_section.group(1):
        issues.append("CONFIG AS PROSE INSTEAD OF TABLE")

    # Check diagnostics format consistency
    diag_section = re.search(r'## Diagnostics\n\n(.*?)$', content, re.DOTALL)
    if diag_section:
        diag = diag_section.group(1).strip()
        if diag.startswith('No diagnostics') or diag.startswith('None'):
            issues.append(f"NON-STANDARD DIAGNOSTICS FORMAT: {diag[:50]}")

    return issues

def audit_conventions(filepath, content):
    """Check convention completeness by re-detecting from output code."""
    issues = []

    # Extract output code sections
    output_match = re.search(r'## Output\n(.*?)## Conventions Applied', content, re.DOTALL)
    if not output_match:
        return ["CANNOT EXTRACT OUTPUT SECTION"]

    output_text = output_match.group(1)
    # Extract code blocks from output section
    code_blocks = re.findall(r'```(?:javascript|tsx|typescript|jsx|js)\n(.*?)```', output_text, re.DOTALL)
    all_output_code = '\n'.join(code_blocks)

    # Detect conventions present in output
    detected = set()
    if re.search(r'\bqrl\(|\bqrlDEV\(|\binlinedQrl\(', all_output_code):
        detected.add('CONV-01')
    # ... (all 14 patterns)

    # Extract documented conventions
    conv_section = re.search(r'## Conventions Applied\n\n(.*?)(?:\n## )', content, re.DOTALL)
    if conv_section:
        documented = set(re.findall(r'\[CONV-(\d+)\]', conv_section.group(1)))
        documented = {f'CONV-{c}' for c in documented}

    missing = detected - documented
    for conv in sorted(missing):
        issues.append(f"MISSING CONVENTION: {conv} detected in output but not documented")

    return issues

# Main
for filename in sorted(os.listdir(SPEC_DIR)):
    if not filename.endswith('.md'):
        continue
    filepath = os.path.join(SPEC_DIR, filename)
    with open(filepath) as f:
        content = f.read()

    struct_issues = audit_structure(filepath, content)
    conv_issues = audit_conventions(filepath, content)

    if struct_issues or conv_issues:
        print(f"\n{filename}:")
        for issue in struct_issues + conv_issues:
            print(f"  - {issue}")
```

### Fix Script Structure (Recommended)

```python
#!/usr/bin/env python3
"""Auto-fix normalizable structural inconsistencies in spec files."""

def normalize_module_headings(content):
    """Remove backticks, normalize ENTRY POINT notation, remove annotations."""
    # Remove backticks from module paths
    content = re.sub(r'^(### Module: )`([^`]+)`', r'\1\2', content, flags=re.MULTILINE)
    # Square brackets to parentheses
    content = re.sub(r'\[ENTRY POINT\]', '(ENTRY POINT)', content)
    # Remove (main) annotations
    content = re.sub(r'^(### Module: [^\n]+) \(main(?:[^)]*)\)', r'\1', content, flags=re.MULTILINE)
    return content

def normalize_diagnostics(content):
    """Standardize diagnostics section to JSON code block."""
    # Replace "No diagnostics." / "None" with ```json [] ```
    content = re.sub(
        r'(## Diagnostics\n\n)(?:No diagnostics\.?|None.*?)(\n*$)',
        r'\1```json\n[]\n```\n',
        content, flags=re.DOTALL
    )
    return content

def add_missing_details_blocks(content):
    """Add 'omitted' AST placeholder where <details> blocks are missing."""
    # For files with zero <details> blocks, add placeholder after each code block
    if '<details>' not in content:
        # Add after input code block
        content = re.sub(
            r'(### Source Code\n\n```tsx\n.*?```)\n\n(## Output)',
            r'\1\n\n<details>\n<summary>Input AST (OXC)</summary>\n\nAST omitted for brevity.\n\n</details>\n\n\2',
            content, flags=re.DOTALL
        )
    return content
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|---|---|---|---|
| Manual Claude generation (Plan 01) | Python script generation (Plans 02-06) | During Phase 2 execution | Two different formatting styles in the 162 files |
| gen-spec.py with CONV-10 `pass` | Need to fix CONV-10 detection | Phase 3 | 3-8 files may be missing CONV-10 convention documentation |

**Known generation artifacts:**
- Plan 01 (27 files): Manual Claude generation, full ASTs inline, `[ENTRY POINT]` brackets, no backticks, prose config for defaults
- Plans 02-06 (135 files): Mix of Python script and Claude generation, "omitted for brevity" ASTs, `(ENTRY POINT)` parentheses, backtick-wrapped module paths in ~27 files

## Open Questions

1. **Should "Key behavior" notes be preserved or stripped?**
   - What we know: 65 files have `**Key behavior:**` free-form notes in the Conventions section that add valuable context
   - What's unclear: Whether these are part of the canonical format or ad-hoc additions
   - Recommendation: PRESERVE them. They add significant value for the OXC port reader. They don't violate any requirement. Do not strip during normalization.

2. **Should all ASTs be regenerated or are "omitted" placeholders acceptable?**
   - What we know: SPEC-04 and SPEC-06 say "includes the input/output AST in a `<details>` block." 48 files have "omitted" placeholders.
   - What's unclear: Whether "omitted for brevity" satisfies SPEC-04/SPEC-06
   - Recommendation: Accept "omitted" as satisfying the requirement. The 6 files with ZERO `<details>` blocks need at least a placeholder added. Full AST regeneration for 48 files would be high cost, low value.

3. **CONV-10 false negative scope**
   - What we know: gen-spec.py never detects CONV-10. 3 files already have CONV-10 (manually added). At least 4-5 more files may need it.
   - What's unclear: Exact set of files where isServer/isBrowser/isDev replacement occurs
   - Recommendation: Check all files with "server", "prod", "dev_mode" in their test names. Compare input code's isServer/isBrowser/isDev usage with output code. Add CONV-10 where the identifier is replaced with a literal.

## Sources

### Primary (HIGH confidence)
- Direct file analysis of all 162 spec files in `.planning/spec/`
- `.planning/tools/gen-spec.py` -- Python script used for batch generation, contains canonical format and convention detection patterns
- `.planning/phases/02-generate-all-spec-files/02-01-PLAN.md` through `02-06-PLAN.md` -- Phase 2 plans defining the expected template
- `.planning/REQUIREMENTS.md` -- QUAL-04, SPEC-01 through SPEC-10, CONV-01 through CONV-14
- `.planning/ROADMAP.md` -- Phase 3 success criteria

### Secondary (MEDIUM confidence)
- Phase 2 SUMMARY files documenting generation approach variations (Python vs shell, manual vs automated)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- Python scripting is proven for this codebase (gen-spec.py already works)
- Architecture: HIGH -- Two-pass audit+fix pattern is well-understood; all inconsistencies are fully cataloged from direct file analysis
- Pitfalls: HIGH -- Specific files and counts identified; CONV-10 gap is verified in gen-spec.py source code

**Research date:** 2026-02-10
**Valid until:** Indefinite (static file analysis, no external dependencies)
