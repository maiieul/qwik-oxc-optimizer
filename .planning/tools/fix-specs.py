#!/usr/bin/env python3
"""Deterministic normalizer for all cosmetic inconsistencies in spec files.

Applies transformations in order:
  1. Module heading backticks
  2. Entry point notation
  3. Extra module annotations
  4. Config prose to table
  5. Config value backticks
  6. Diagnostics normalization
  7. No-conventions text normalization

Safety: Preserves **Key behavior:** notes, convention descriptions, code blocks,
<details> blocks, and segment metadata.
"""

import os
import re
import sys

SPEC_DIR = os.path.join(os.path.dirname(__file__), "..", "spec")
SPEC_DIR = os.path.normpath(SPEC_DIR)


def normalize_module_backticks(content):
    """1. Remove backticks from module heading paths."""
    count = 0
    def replacer(m):
        nonlocal count
        count += 1
        return m.group(1) + m.group(2)
    content = re.sub(r'^(### Module: )`([^`]+)`', replacer, content, flags=re.MULTILINE)
    return content, count


def normalize_entry_point_notation(content):
    """2. Replace [ENTRY POINT] with (ENTRY POINT)."""
    count = content.count("[ENTRY POINT]")
    content = content.replace("[ENTRY POINT]", "(ENTRY POINT)")
    return content, count


def normalize_module_annotations(content):
    """3. Remove extra annotations like (main), (main module), etc. from module headings.

    Preserves (ENTRY POINT) which is legitimate.
    Does NOT touch lines that already have (ENTRY POINT).
    """
    count = 0
    lines = content.split("\n")
    new_lines = []
    for line in lines:
        if line.startswith("### Module: ") and "(ENTRY POINT)" not in line:
            # Remove trailing (main), (main module), (main, ...) patterns
            new_line = re.sub(r'\s+\(main[^)]*\)\s*$', '', line)
            if new_line != line:
                count += 1
                line = new_line
        new_lines.append(line)
    content = "\n".join(new_lines)
    return content, count


def normalize_config_prose(content):
    """4. Replace prose config with table format."""
    count = 0

    # Find the Test Configuration section
    config_match = re.search(r'(## Test Configuration\n\n)(.*?)(\n## )', content, re.DOTALL)
    if not config_match:
        return content, 0

    config_text = config_match.group(2).strip()

    # Check if it's prose (doesn't start with | or a **Note line before the table)
    first_content_line = ""
    for line in config_text.split("\n"):
        stripped = line.strip()
        if stripped and not stripped.startswith("**Note"):
            first_content_line = stripped
            break

    if first_content_line and not first_content_line.startswith("|"):
        # It's prose -- replace with table
        table = "| Option | Value |\n|--------|-------|\n| *(all defaults)* | |"
        content = content[:config_match.start(2)] + table + "\n" + content[config_match.end(2):]
        count = 1

    return content, count


def normalize_config_backticks(content):
    """5. Remove backtick wrapping from config table values."""
    count = 0

    # Find the config table section
    config_match = re.search(r'(## Test Configuration\n\n)(.*?)(\n## )', content, re.DOTALL)
    if not config_match:
        return content, 0

    config_text = config_match.group(2)
    new_lines = []
    for line in config_text.split("\n"):
        if line.strip().startswith("|") and "-----" not in line and "Option" not in line:
            # Replace backtick-wrapped values in table cells
            new_line = re.sub(r'\|\s*`([^`]+)`\s*\|', r'| \1 |', line)
            if new_line != line:
                count += 1
                line = new_line
        new_lines.append(line)

    if count > 0:
        new_config = "\n".join(new_lines)
        content = content[:config_match.start(2)] + new_config + content[config_match.end(2):]

    return content, count


def normalize_diagnostics(content):
    """6. Normalize diagnostics section to JSON code block."""
    count = 0

    # Find the Diagnostics section (it's the last section)
    diag_match = re.search(r'(## Diagnostics\n\n)(.*?)$', content, re.DOTALL)
    if not diag_match:
        return content, 0

    diag_text = diag_match.group(2).strip()

    # Already in code block format
    if diag_text.startswith("```json") or diag_text.startswith("```\n"):
        return content, 0

    # Non-standard formats to normalize
    needs_fix = False
    if diag_text.startswith("No diagnostics"):
        needs_fix = True
    elif diag_text.startswith("None"):
        needs_fix = True
    elif diag_text == "[]":
        needs_fix = True
    elif not diag_text:
        needs_fix = True

    if needs_fix:
        replacement = "```json\n[]\n```\n"
        content = content[:diag_match.start(2)] + replacement
        count = 1

    return content, count


def normalize_noconv_text(content):
    """7. Replace non-standard no-conventions text.

    Preserves **Key behavior:** notes that may follow the no-conventions line.
    """
    count = 0

    conv_match = re.search(r'(## Conventions Applied\n\n)(.*?)(\n\n## )', content, re.DOTALL)
    if not conv_match:
        return content, 0

    conv_text = conv_match.group(2)

    # Split into lines/paragraphs to isolate the "None" line from Key behavior notes
    paragraphs = conv_text.split("\n\n")
    first_para = paragraphs[0].strip()

    # Check for non-standard no-conventions patterns
    # "**None** -- ..." or "None — ..." or "None --"
    if first_para.startswith("**None**") or re.match(r'^None\s*[\u2014\-]', first_para):
        # Replace only the first paragraph, preserve the rest (Key behavior notes, etc.)
        paragraphs[0] = "*No optimizer conventions detected in output.*"
        replacement = "\n\n".join(paragraphs)
        content = content[:conv_match.start(2)] + replacement + content[conv_match.end(2):]
        count = 1

    return content, count


def fix_file(filepath, filename):
    """Apply all normalizations to a single file."""
    with open(filepath, 'r') as f:
        original = f.read()

    content = original
    transformations = []

    # Count key behavior notes before
    kb_before = len(re.findall(r'\*\*Key behavior:', content))
    kb_dead_before = len(re.findall(r'\*\*Key dead code behavior:', content))
    total_kb_before = kb_before + kb_dead_before

    # Apply transformations in order
    content, n = normalize_module_backticks(content)
    if n: transformations.append(f"backtick-module: {n}")

    content, n = normalize_entry_point_notation(content)
    if n: transformations.append(f"bracket-entry-point: {n}")

    content, n = normalize_module_annotations(content)
    if n: transformations.append(f"module-annotation: {n}")

    content, n = normalize_config_prose(content)
    if n: transformations.append(f"prose-config: {n}")

    content, n = normalize_config_backticks(content)
    if n: transformations.append(f"config-backtick: {n}")

    content, n = normalize_diagnostics(content)
    if n: transformations.append(f"diagnostics: {n}")

    content, n = normalize_noconv_text(content)
    if n: transformations.append(f"noconv-text: {n}")

    # Safety check: Key behavior note preservation
    kb_after = len(re.findall(r'\*\*Key behavior:', content))
    kb_dead_after = len(re.findall(r'\*\*Key dead code behavior:', content))
    total_kb_after = kb_after + kb_dead_after

    if total_kb_before != total_kb_after:
        print(f"  ABORT {filename}: Key behavior count changed ({total_kb_before} -> {total_kb_after})")
        return False, 0

    if content == original:
        return False, 0

    # Write modified content
    with open(filepath, 'w') as f:
        f.write(content)

    kb_msg = f", Key behavior notes: preserved ({total_kb_before} occurrences)" if total_kb_before > 0 else ""
    print(f"  {filename}: {', '.join(transformations)}{kb_msg}")
    return True, len(transformations)


def main():
    if not os.path.isdir(SPEC_DIR):
        print(f"ERROR: Spec directory not found: {SPEC_DIR}", file=sys.stderr)
        sys.exit(2)

    spec_files = sorted(f for f in os.listdir(SPEC_DIR) if f.endswith('.md'))
    total_files = len(spec_files)

    print(f"Normalizing {total_files} spec files in {SPEC_DIR}\n")

    modified_count = 0
    transform_count = 0
    aborted = []

    for filename in spec_files:
        filepath = os.path.join(SPEC_DIR, filename)
        modified, transforms = fix_file(filepath, filename)
        if modified:
            modified_count += 1
            transform_count += transforms

    print(f"\n{'=' * 50}")
    print(f"Files modified: {modified_count}/{total_files}")
    print(f"Total transformation categories applied: {transform_count}")

    if aborted:
        print(f"\nABORTED files (Key behavior mismatch): {len(aborted)}")
        for f in aborted:
            print(f"  - {f}")

    print()


if __name__ == "__main__":
    main()
