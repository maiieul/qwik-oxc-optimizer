#!/usr/bin/env python3
"""Audit all 162 spec files for structural consistency and convention completeness.

Two-pass audit:
  Pass 1: Structural consistency checks (heading format, section order, etc.)
  Pass 2: Convention completeness checks (detected vs documented conventions)

Exit code 0 if no issues, 1 if issues found.
"""

import os
import re
import sys
from collections import defaultdict

SPEC_DIR = os.path.join(os.path.dirname(__file__), "..", "spec")
SPEC_DIR = os.path.normpath(SPEC_DIR)


# --- Pass 1: Structural consistency checks ---

def audit_structure(filename, content):
    """Check structural consistency against canonical format."""
    issues = []

    # 1. Required sections exist
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
            issues.append(("MISSING_SECTION", f"Missing section: {section}"))

    # 2. Section order
    positions = []
    for section in required_sections:
        pos = content.find(section)
        if pos >= 0:
            positions.append(pos)
    if positions != sorted(positions):
        issues.append(("SECTION_ORDER", "Sections appear in wrong order"))

    # 3. Module heading backticks
    for match in re.finditer(r'^### Module: `([^`]+)`', content, re.MULTILINE):
        issues.append(("BACKTICK_MODULE", f"Backtick-wrapped module path: `{match.group(1)}`"))

    # 4. Entry point notation: flag [ENTRY POINT]
    bracket_count = content.count("[ENTRY POINT]")
    if bracket_count > 0:
        issues.append(("BRACKET_ENTRY_POINT", f"[ENTRY POINT] found {bracket_count} time(s) (should be parentheses)"))

    # 5. Extra module annotations: (main), (main module), (main, ...) etc.
    for match in re.finditer(r'^### Module: .+?\s+\(main[^)]*\)', content, re.MULTILINE):
        # Exclude (ENTRY POINT) which is legitimate
        heading = match.group(0)
        if "(ENTRY POINT)" not in heading:
            issues.append(("EXTRA_ANNOTATION", f"Extra module annotation: {heading.strip()}"))

    # 6. Config format: prose instead of table
    config_match = re.search(r'## Test Configuration\n\n(.*?)(?=\n## )', content, re.DOTALL)
    if config_match:
        config_text = config_match.group(1).strip()
        # Check if it's prose (not a table -- tables start with |)
        first_content_line = ""
        for line in config_text.split("\n"):
            stripped = line.strip()
            if stripped and not stripped.startswith("**Note"):
                first_content_line = stripped
                break
        if first_content_line and not first_content_line.startswith("|"):
            issues.append(("PROSE_CONFIG", f"Config is prose instead of table: {first_content_line[:80]}"))

    # 7. Config value backticks in table
    if config_match:
        config_text = config_match.group(1)
        for line in config_text.split("\n"):
            # Only check table data rows (not header or separator)
            if line.strip().startswith("|") and "-----" not in line and "Option" not in line:
                # Check if values are backtick-wrapped
                # Pattern: | Key | `value` |
                if re.search(r'\|\s*`[^`]+`\s*\|', line):
                    # Exclude the first column (key) - check only value column
                    cells = [c.strip() for c in line.split("|")]
                    # cells[0] is empty, cells[1] is key, cells[2] is value, cells[3] is empty
                    if len(cells) >= 3:
                        value_cell = cells[2]
                        if re.match(r'^`[^`]+`$', value_cell):
                            issues.append(("CONFIG_BACKTICK", f"Backtick-wrapped config value: {line.strip()}"))

    # 8. Diagnostics format
    diag_match = re.search(r'## Diagnostics\n\n(.*?)$', content, re.DOTALL)
    if diag_match:
        diag_text = diag_match.group(1).strip()
        # Standard format is ```json\n...\n```
        if diag_text.startswith("```json") or diag_text.startswith("```\n"):
            pass  # Already in code block format
        elif diag_text == "[]":
            issues.append(("NONSTANDARD_DIAG", "Bare [] without code fence"))
        elif diag_text.startswith("No diagnostics"):
            issues.append(("NONSTANDARD_DIAG", "Prose diagnostics: 'No diagnostics.'"))
        elif diag_text.startswith("None"):
            issues.append(("NONSTANDARD_DIAG", f"Prose diagnostics: '{diag_text[:60]}'"))
        elif not diag_text:
            issues.append(("NONSTANDARD_DIAG", "Empty diagnostics section"))

    # 9. Missing <details> blocks
    if "<details>" not in content:
        issues.append(("NO_DETAILS", "No <details> blocks (AST sections missing)"))

    # 10. No-conventions text: non-standard variants
    conv_match = re.search(r'## Conventions Applied\n\n(.*?)(?=\n## )', content, re.DOTALL)
    if conv_match:
        conv_text = conv_match.group(1).strip()
        # Check for non-standard no-conventions text
        if conv_text.startswith("**None**"):
            issues.append(("NONSTANDARD_NOCONV", f"Non-standard no-conventions text: {conv_text[:80]}"))
        elif conv_text.startswith("None"):
            # "None --" or "None —" patterns
            issues.append(("NONSTANDARD_NOCONV", f"Non-standard no-conventions text: {conv_text[:80]}"))

    return issues


# --- Pass 2: Convention completeness checks ---

def detect_conventions_in_code(all_output_code, modules_text):
    """Detect conventions present in output code using regex patterns from gen-spec.py."""
    detected = set()

    # CONV-01: QRL calls
    if re.search(r'\bqrl\(', all_output_code) or re.search(r'\bqrlDEV\(', all_output_code) or re.search(r'\binlinedQrl\(', all_output_code):
        detected.add("CONV-01")

    # CONV-02: Dollar-to-Qrl
    qrl_suffixes = re.findall(r'\b(\w+Qrl)\(', all_output_code)
    dollar_qrls = [q for q in qrl_suffixes if q not in ('qrl', 'qrlDEV', 'inlinedQrl') and q.endswith('Qrl')]
    if dollar_qrls:
        detected.add("CONV-02")

    # CONV-03: JSX transforms
    if re.search(r'_jsxSorted\(', all_output_code) or re.search(r'_jsxSplit\(', all_output_code) or re.search(r'_jsxQ\(', all_output_code) or re.search(r'_jsxC\(', all_output_code):
        detected.add("CONV-03")

    # CONV-04: Signal helpers
    if re.search(r'_wrapProp\(', all_output_code) or re.search(r'_wrapSignal\(', all_output_code) or re.search(r'_fnSignal\(', all_output_code) or re.search(r'_getVarProps\(', all_output_code) or re.search(r'_getConstProps\(', all_output_code):
        detected.add("CONV-04")

    # CONV-05: Capture patterns
    if re.search(r'_captures\[', all_output_code):
        detected.add("CONV-05")

    # CONV-06: Lazy imports
    if re.search(r'const i_\w+\s*=\s*\(\)\s*=>\s*import\(', all_output_code):
        detected.add("CONV-06")

    # CONV-07: PURE annotations
    if re.search(r'/\*#__PURE__\*/', all_output_code):
        detected.add("CONV-07")

    # CONV-08: Segment extraction (check for ENTRY POINT modules)
    if "(ENTRY POINT)" in modules_text or "[ENTRY POINT]" in modules_text:
        detected.add("CONV-08")

    # CONV-09: Code stripping
    if '"Symbol removed by Qwik Optimizer"' in all_output_code or re.search(r'_noopQrl\(', all_output_code) or re.search(r'_noopQrlDEV\(', all_output_code):
        detected.add("CONV-09")

    # CONV-10: Const replacement (heuristic -- see plan)
    # Skip in pass 2 -- this requires input vs output comparison and is handled separately

    # CONV-11: Props destructuring
    if re.search(r'_rawProps', all_output_code) or re.search(r'_restProps\(', all_output_code):
        detected.add("CONV-11")

    # CONV-12: Input binding
    if re.search(r'"bind:', all_output_code) or re.search(r'\b_val\b', all_output_code) or re.search(r'\b_chk\b', all_output_code):
        detected.add("CONV-12")

    # CONV-13: Sync$ serialization
    if re.search(r'_qrlSync\(', all_output_code):
        detected.add("CONV-13")

    # CONV-14: Hoisted functions
    if re.findall(r'const _hf\d+\s*=', all_output_code):
        detected.add("CONV-14")

    return detected


def audit_conventions(filename, content):
    """Check convention completeness by re-detecting from output code."""
    issues = []

    # Extract the Output section
    output_match = re.search(r'## Output\n(.*?)## Conventions Applied', content, re.DOTALL)
    if not output_match:
        issues.append(("CONV_PARSE_ERROR", "Cannot extract Output section for convention check"))
        return issues

    output_text = output_match.group(1)

    # Extract code blocks from output section (inside ``` fences)
    code_blocks = re.findall(r'```(?:javascript|tsx|typescript|jsx|js|mjs)\n(.*?)```', output_text, re.DOTALL)
    all_output_code = '\n'.join(code_blocks)

    if not all_output_code.strip():
        # No code blocks found -- skip convention check
        return issues

    # Detect conventions in code
    detected = detect_conventions_in_code(all_output_code, output_text)

    # Extract documented conventions
    conv_match = re.search(r'## Conventions Applied\n\n(.*?)(?=\n## )', content, re.DOTALL)
    if not conv_match:
        issues.append(("CONV_PARSE_ERROR", "Cannot extract Conventions Applied section"))
        return issues

    conv_text = conv_match.group(1)
    documented_nums = set(re.findall(r'\[CONV-(\d+)\]', conv_text))
    documented = {f"CONV-{c.zfill(2)}" for c in documented_nums}

    # Report detected but not documented (missing conventions)
    missing = detected - documented
    for conv in sorted(missing):
        issues.append(("MISSING_CONV", f"{conv} detected in output code but not documented"))

    return issues


# --- Main ---

def main():
    if not os.path.isdir(SPEC_DIR):
        print(f"ERROR: Spec directory not found: {SPEC_DIR}", file=sys.stderr)
        sys.exit(2)

    spec_files = sorted(f for f in os.listdir(SPEC_DIR) if f.endswith('.md'))
    total_files = len(spec_files)

    print(f"Auditing {total_files} spec files in {SPEC_DIR}\n")
    print("=" * 70)

    all_issues = {}  # filename -> list of (category, message)
    category_counts = defaultdict(set)  # category -> set of affected filenames

    for filename in spec_files:
        filepath = os.path.join(SPEC_DIR, filename)
        with open(filepath, 'r') as f:
            content = f.read()

        struct_issues = audit_structure(filename, content)
        conv_issues = audit_conventions(filename, content)

        file_issues = struct_issues + conv_issues
        if file_issues:
            all_issues[filename] = file_issues
            for cat, msg in file_issues:
                category_counts[cat].add(filename)

    # Print file-grouped results
    print("\nPASS 1: STRUCTURAL CONSISTENCY")
    print("=" * 70)

    structural_categories = {
        "MISSING_SECTION", "SECTION_ORDER", "BACKTICK_MODULE",
        "BRACKET_ENTRY_POINT", "EXTRA_ANNOTATION", "PROSE_CONFIG",
        "CONFIG_BACKTICK", "NONSTANDARD_DIAG", "NO_DETAILS",
        "NONSTANDARD_NOCONV",
    }

    struct_file_count = 0
    for filename in sorted(all_issues.keys()):
        struct = [(c, m) for c, m in all_issues[filename] if c in structural_categories]
        if struct:
            struct_file_count += 1
            print(f"\n  {filename}:")
            for cat, msg in struct:
                print(f"    [{cat}] {msg}")

    if struct_file_count == 0:
        print("\n  No structural issues found.")

    print(f"\n\nPASS 2: CONVENTION COMPLETENESS")
    print("=" * 70)

    conv_categories = {"MISSING_CONV", "CONV_PARSE_ERROR"}

    conv_file_count = 0
    for filename in sorted(all_issues.keys()):
        conv = [(c, m) for c, m in all_issues[filename] if c in conv_categories]
        if conv:
            conv_file_count += 1
            print(f"\n  {filename}:")
            for cat, msg in conv:
                print(f"    [{cat}] {msg}")

    if conv_file_count == 0:
        print("\n  No convention completeness issues found.")

    # Summary table
    print(f"\n\n{'=' * 70}")
    print("SUMMARY")
    print(f"{'=' * 70}\n")
    print(f"Total files audited: {total_files}")
    print(f"Files with structural issues: {struct_file_count}")
    print(f"Files with convention issues: {conv_file_count}")
    print()

    if category_counts:
        print(f"{'Category':<25} {'Files Affected':>15}")
        print(f"{'-'*25} {'-'*15}")
        for cat in sorted(category_counts.keys()):
            print(f"{cat:<25} {len(category_counts[cat]):>15}")
    else:
        print("No issues found across all files.")

    print()

    total_issue_files = len(all_issues)
    if total_issue_files > 0:
        print(f"RESULT: {total_issue_files} files have issues")
        sys.exit(1)
    else:
        print("RESULT: All files pass audit")
        sys.exit(0)


if __name__ == "__main__":
    main()
