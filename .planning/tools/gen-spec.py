#!/usr/bin/env python3
"""Generate spec markdown files from snapshot files.

Usage: python3 gen-spec.py <test_name> '<config_json>'

Parses the .snap file, generates OXC ASTs, detects conventions,
and writes a structured markdown spec to .planning/spec/<test_name>.md
"""

import sys
import json
import re
import subprocess
import os

BASE_DIR = "/Users/jackshelton/dev/open-source/qwik-optimizer"
SNAP_DIR = f"{BASE_DIR}/swc-optimizer/core/src/snapshots"
SPEC_DIR = f"{BASE_DIR}/.planning/spec"
OXC_BIN = f"{BASE_DIR}/oxc-ast-util/target/release/oxc-ast-util"


def parse_snapshot(snap_path):
    """Parse a snapshot file into input, modules, and diagnostics."""
    with open(snap_path, 'r') as f:
        content = f.read()

    lines = content.split('\n')

    # Skip YAML frontmatter
    fm_start = None
    fm_end = None
    for i, line in enumerate(lines):
        if line.strip() == '---':
            if fm_start is None:
                fm_start = i
            else:
                fm_end = i
                break

    if fm_end is None:
        fm_end = 0

    # Find ==INPUT==
    input_start = None
    for i in range(fm_end, len(lines)):
        if lines[i].strip() == '==INPUT==':
            input_start = i + 1
            break

    if input_start is None:
        return None, [], ""

    # Find first module separator after input
    first_sep = None
    for i in range(input_start, len(lines)):
        if lines[i].startswith('============================='):
            first_sep = i
            break

    input_code = '\n'.join(lines[input_start:first_sep]).strip()

    # Parse modules
    modules = []
    diag_start = None

    # Find all separator lines and diagnostics marker
    sep_indices = []
    for i in range(first_sep, len(lines)):
        if lines[i].startswith('============================='):
            sep_indices.append(i)
        elif lines[i].strip() == '== DIAGNOSTICS ==':
            diag_start = i
            break

    for idx, sep_idx in enumerate(sep_indices):
        # Parse module header
        header = lines[sep_idx]
        # Extract path and flags from: ============================= path [flags]==
        match = re.match(r'^=+ (.+?)\s*==\s*$', header)
        if match:
            path_and_flags = match.group(1)
            is_entry = '(ENTRY POINT)' in path_and_flags
            mod_path = path_and_flags.replace('(ENTRY POINT)', '').strip()
        else:
            mod_path = "unknown"
            is_entry = False

        # Find end of this module section
        if idx + 1 < len(sep_indices):
            end_idx = sep_indices[idx + 1]
        elif diag_start is not None:
            end_idx = diag_start
        else:
            end_idx = len(lines)

        # Parse module content
        mod_lines = lines[sep_idx + 1:end_idx]
        mod_text = '\n'.join(mod_lines)

        # Extract code (before Some(" or None line)
        code_lines = []
        source_map_line = None
        segment_meta = None
        in_segment = False
        segment_lines = []

        for j, ml in enumerate(mod_lines):
            stripped = ml.strip()
            if stripped.startswith('Some("') or stripped == 'None':
                source_map_line = stripped
                # Check for segment metadata after source map
                remaining = mod_lines[j+1:]
                rem_text = '\n'.join(remaining).strip()
                if rem_text.startswith('/*'):
                    # Extract JSON from /* ... */
                    meta_match = re.search(r'/\*\s*(\{.*?\})\s*\*/', rem_text, re.DOTALL)
                    if meta_match:
                        try:
                            segment_meta = json.loads(meta_match.group(1))
                        except json.JSONDecodeError:
                            segment_meta = None
                break
            code_lines.append(ml)

        mod_code = '\n'.join(code_lines).strip()

        modules.append({
            'path': mod_path,
            'is_entry': is_entry,
            'code': mod_code,
            'segment_meta': segment_meta,
        })

    # Extract diagnostics
    diagnostics = "[]"
    if diag_start is not None:
        diag_lines = lines[diag_start + 1:]
        diagnostics = '\n'.join(diag_lines).strip()
        if not diagnostics:
            diagnostics = "[]"

    return input_code, modules, diagnostics


def generate_ast(code, ext):
    """Generate AST using oxc-ast-util."""
    try:
        result = subprocess.run(
            [OXC_BIN, ext],
            input=code,
            capture_output=True,
            text=True,
            timeout=10
        )
        if result.returncode == 0:
            return result.stdout
        elif result.returncode == 2:
            return '{"error": "Parser panic - AST generation failed"}'
        else:
            return result.stdout if result.stdout else '{"error": "AST generation failed"}'
    except Exception as e:
        return f'{{"error": "AST generation error: {str(e)}"}}'


def get_ext_from_path(mod_path):
    """Determine the file extension for AST generation from a module path."""
    if mod_path.endswith('.tsx'):
        return 'tsx'
    elif mod_path.endswith('.ts'):
        return 'ts'
    elif mod_path.endswith('.jsx'):
        return 'jsx'
    elif mod_path.endswith('.js'):
        return 'js'
    elif mod_path.endswith('.mjs'):
        return 'mjs'
    else:
        return 'js'


def detect_conventions(modules):
    """Detect which conventions are present across all output modules."""
    all_code = '\n'.join(m['code'] for m in modules)
    all_meta = [m.get('segment_meta') for m in modules if m.get('segment_meta')]

    conventions = []

    # CONV-01: QRL calls
    if re.search(r'\bqrl\(', all_code) or re.search(r'\bqrlDEV\(', all_code) or re.search(r'\binlinedQrl\(', all_code):
        desc = []
        if re.search(r'\bqrl\(', all_code):
            desc.append('`qrl()` calls')
        if re.search(r'\bqrlDEV\(', all_code):
            desc.append('`qrlDEV()` calls')
        if re.search(r'\binlinedQrl\(', all_code):
            desc.append('`inlinedQrl()` calls')
        conventions.append(('CONV-01', 'QRL Calls', f"Output uses {', '.join(desc)} to create lazy-loadable references"))

    # CONV-02: Dollar-to-Qrl
    qrl_suffixes = re.findall(r'\b(\w+Qrl)\(', all_code)
    # Filter out plain 'qrl(' which is CONV-01
    dollar_qrls = [q for q in qrl_suffixes if q not in ('qrl', 'qrlDEV', 'inlinedQrl') and q.endswith('Qrl')]
    if dollar_qrls:
        unique = list(set(dollar_qrls))
        conventions.append(('CONV-02', 'Dollar-to-Qrl Transform', f"Dollar-sign functions converted: {', '.join('`' + q + '()`' for q in unique)}"))

    # CONV-03: JSX transforms
    jsx_fns = []
    if re.search(r'_jsxSorted\(', all_code):
        jsx_fns.append('`_jsxSorted()`')
    if re.search(r'_jsxSplit\(', all_code):
        jsx_fns.append('`_jsxSplit()`')
    if re.search(r'_jsxQ\(', all_code):
        jsx_fns.append('`_jsxQ()`')
    if re.search(r'_jsxC\(', all_code):
        jsx_fns.append('`_jsxC()`')
    if jsx_fns:
        conventions.append(('CONV-03', 'JSX Transforms', f"JSX transpiled using {', '.join(jsx_fns)}"))

    # CONV-04: Signal helpers
    sig_fns = []
    if re.search(r'_wrapProp\(', all_code):
        sig_fns.append('`_wrapProp()`')
    if re.search(r'_wrapSignal\(', all_code):
        sig_fns.append('`_wrapSignal()`')
    if re.search(r'_fnSignal\(', all_code):
        sig_fns.append('`_fnSignal()`')
    if re.search(r'_getVarProps\(', all_code):
        sig_fns.append('`_getVarProps()`')
    if re.search(r'_getConstProps\(', all_code):
        sig_fns.append('`_getConstProps()`')
    if sig_fns:
        conventions.append(('CONV-04', 'Signal Helpers', f"Signal optimization via {', '.join(sig_fns)}"))

    # CONV-05: Capture patterns
    if re.search(r'_captures\[', all_code):
        conventions.append(('CONV-05', 'Capture Patterns', "Captured variables accessed via `_captures[]` array in extracted segments"))

    # CONV-06: Lazy imports
    if re.search(r'const i_\w+\s*=\s*\(\)\s*=>\s*import\(', all_code):
        count = len(re.findall(r'const i_\w+\s*=\s*\(\)\s*=>\s*import\(', all_code))
        conventions.append(('CONV-06', 'Lazy Imports', f"{'Multiple lazy' if count > 1 else 'Lazy'} import {'declarations' if count > 1 else 'declaration'} (`const i_HASH = () => import(...)`) for deferred module loading ({count} total)"))

    # CONV-07: PURE annotations
    if re.search(r'/\*#__PURE__\*/', all_code):
        count = len(re.findall(r'/\*#__PURE__\*/', all_code))
        conventions.append(('CONV-07', 'PURE Annotations', f"`/*#__PURE__*/` tree-shaking hints on {count} call expressions"))

    # CONV-08: Segment extraction
    entry_modules = [m for m in modules if m['is_entry']]
    if entry_modules:
        conventions.append(('CONV-08', 'Segment Extraction', f"Code extracted into {len(entry_modules)} separate entry point module(s)"))

    # CONV-09: Code stripping
    if '"Symbol removed by Qwik Optimizer"' in all_code or re.search(r'_noopQrl\(', all_code):
        conventions.append(('CONV-09', 'Code Stripping', "Code removed/stubbed by optimizer"))

    # CONV-10: Const replacement
    if re.search(r'\bisServer\b', all_code) or re.search(r'\bisBrowser\b', all_code) or re.search(r'\bisDev\b', all_code):
        # Check if these are literal replacements (hard to detect without input comparison)
        pass  # Only add if we see evidence of replacement

    # CONV-11: Props destructuring
    prop_fns = []
    if re.search(r'_rawProps', all_code):
        prop_fns.append('`_rawProps`')
    if re.search(r'_restProps\(', all_code):
        prop_fns.append('`_restProps()`')
    if prop_fns:
        conventions.append(('CONV-11', 'Props Destructuring', f"Props handling via {', '.join(prop_fns)}"))

    # CONV-12: Input binding
    bind_parts = []
    if 'bind:' in all_code or '"bind:' in all_code:
        bind_parts.append('`bind:` directives')
    if re.search(r'\b_val\b', all_code):
        bind_parts.append('`_val` handler')
    if re.search(r'\b_chk\b', all_code):
        bind_parts.append('`_chk` handler')
    if bind_parts:
        conventions.append(('CONV-12', 'Input Binding', f"Input binding via {', '.join(bind_parts)}"))

    # CONV-13: Sync$ serialization
    if re.search(r'_qrlSync\(', all_code):
        conventions.append(('CONV-13', 'Sync$ Serialization', "`_qrlSync()` with stringified function for synchronous QRL"))

    # CONV-14: Hoisted functions
    hf_matches = re.findall(r'const _hf(\d+)\s*=', all_code)
    if hf_matches:
        count = len(set(hf_matches))
        conventions.append(('CONV-14', 'Hoisted Functions', f"{count} hoisted function(s) (`_hfN`/`_hfN_str` pairs) for signal-derived expressions"))

    return conventions


def extract_function_calls(modules):
    """Extract notable function calls from output modules."""
    calls = []
    seen = set()

    for mod in modules:
        code = mod['code']
        path = mod['path']

        # Find imports
        imports = {}
        for m in re.finditer(r'import\s*\{([^}]+)\}\s*from\s*["\']([^"\']+)["\']', code):
            names = [n.strip() for n in m.group(1).split(',')]
            source = m.group(2)
            for name in names:
                if name:
                    imports[name] = source

        # Find function calls that are notable (qwik internal functions)
        notable_patterns = [
            r'\b(componentQrl)\(',
            r'\b(qrl)\(',
            r'\b(qrlDEV)\(',
            r'\b(inlinedQrl)\(',
            r'\b(_jsxSorted)\(',
            r'\b(_jsxSplit)\(',
            r'\b(_jsxQ)\(',
            r'\b(_jsxC)\(',
            r'\b(_fnSignal)\(',
            r'\b(_wrapProp)\(',
            r'\b(_wrapSignal)\(',
            r'\b(_getVarProps)\(',
            r'\b(_getConstProps)\(',
            r'\b(_captures)',
            r'\b(useSignal)\(',
            r'\b(useStore)\(',
            r'\b(useStylesQrl)\(',
            r'\b(useTaskQrl)\(',
            r'\b(useBrowserVisibleTaskQrl)\(',
            r'\b(_noopQrl)\(',
            r'\b(_qrlSync)\(',
            r'\b(_restProps)\(',
            r'\b(_val)\b',
            r'\b(_chk)\b',
        ]

        for pattern in notable_patterns:
            matches = re.findall(pattern, code)
            for fn_name in matches:
                key = (fn_name, path)
                if key not in seen:
                    count = len(re.findall(re.escape(fn_name) + r'[\(\[]?', code))
                    source = imports.get(fn_name, '-')
                    calls.append({
                        'function': fn_name,
                        'module': path,
                        'source': source,
                        'count': count,
                    })
                    seen.add(key)

    return calls


def generate_spec(test_name, config):
    """Generate a complete spec file."""
    snap_path = f"{SNAP_DIR}/qwik_core__test__{test_name}.snap"

    if not os.path.exists(snap_path):
        print(f"ERROR: Snapshot not found: {snap_path}", file=sys.stderr)
        return False

    input_code, modules, diagnostics = parse_snapshot(snap_path)
    if input_code is None:
        print(f"ERROR: Failed to parse snapshot: {snap_path}", file=sys.stderr)
        return False

    # Generate input AST
    input_ast = generate_ast(input_code, 'tsx')

    # Generate output ASTs
    for mod in modules:
        ext = get_ext_from_path(mod['path'])
        mod['ast'] = generate_ast(mod['code'], ext)

    # Detect conventions
    conventions = detect_conventions(modules)

    # Extract function calls
    fn_calls = extract_function_calls(modules)

    # Determine output code language
    def get_lang(mod_path):
        ext = get_ext_from_path(mod_path)
        return {'js': 'javascript', 'jsx': 'jsx', 'ts': 'typescript', 'tsx': 'tsx', 'mjs': 'javascript'}.get(ext, 'javascript')

    # Build config table
    defaults = {
        'entry_strategy': 'Segment',
        'mode': 'Test',
        'transpile_ts': False,
        'transpile_jsx': False,
        'explicit_extensions': False,
        'preserve_filenames': False,
        'is_server': None,
        'filename': 'test.tsx',
        'src_dir': '/user/qwik/src/',
        'minify': 'Simplify',
        'strip_exports': None,
        'strip_ctx_name': None,
        'strip_event_handlers': False,
        'reg_ctx_name': None,
        'scope': None,
        'core_module': None,
    }

    config_rows = []
    for key, default_val in defaults.items():
        val = config.get(key, default_val)
        if val != default_val:
            display_key = key.replace('_', ' ').title()
            config_rows.append(f"| {display_key} | `{val}` |")

    if not config_rows:
        config_rows.append("| *(all defaults)* | |")

    # Build spec file content
    spec = []
    spec.append(f"# Test: {test_name}\n")
    spec.append("## Test Configuration\n")
    spec.append("| Option | Value |")
    spec.append("|--------|-------|")
    spec.extend(config_rows)
    spec.append("")

    # Input section
    spec.append("## Input\n")
    spec.append("### Source Code\n")
    spec.append("```tsx")
    spec.append(input_code)
    spec.append("```\n")
    spec.append("<details>")
    spec.append("<summary>Input AST (OXC)</summary>\n")
    spec.append("```json")
    spec.append(input_ast.strip())
    spec.append("```\n")
    spec.append("</details>\n")

    # Output section
    spec.append("## Output\n")

    for mod in modules:
        entry_tag = " (ENTRY POINT)" if mod['is_entry'] else ""
        lang = get_lang(mod['path'])
        spec.append(f"### Module: `{mod['path']}`{entry_tag}\n")
        spec.append(f"```{lang}")
        spec.append(mod['code'])
        spec.append("```\n")

        spec.append("<details>")
        spec.append("<summary>Output AST (OXC)</summary>\n")
        spec.append("```json")
        spec.append(mod['ast'].strip())
        spec.append("```\n")
        spec.append("</details>\n")

        if mod.get('segment_meta'):
            spec.append("#### Segment Metadata\n")
            spec.append("```json")
            spec.append(json.dumps(mod['segment_meta'], indent=2))
            spec.append("```\n")

    # Conventions section
    spec.append("## Conventions Applied\n")
    if conventions:
        for conv_id, conv_name, conv_desc in conventions:
            spec.append(f"- **[{conv_id}] {conv_name}**: {conv_desc}")
    else:
        spec.append("*No optimizer conventions detected in output.*")
    spec.append("")

    # Function calls table
    spec.append("## Function Calls in Output\n")
    if fn_calls:
        spec.append("| Function | Module | Import Source | Count |")
        spec.append("|----------|--------|--------------|-------|")
        for call in fn_calls:
            spec.append(f"| `{call['function']}` | `{call['module']}` | `{call['source']}` | {call['count']} |")
    else:
        spec.append("*No notable function calls detected.*")
    spec.append("")

    # Diagnostics section
    spec.append("## Diagnostics\n")
    spec.append(f"```json\n{diagnostics}\n```\n")

    # Write spec file
    spec_path = f"{SPEC_DIR}/{test_name}.md"
    with open(spec_path, 'w') as f:
        f.write('\n'.join(spec))

    print(f"OK: {spec_path}", file=sys.stderr)
    return True


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("Usage: python3 gen-spec.py <test_name> '<config_json>'", file=sys.stderr)
        sys.exit(1)

    test_name = sys.argv[1]
    config = json.loads(sys.argv[2])
    success = generate_spec(test_name, config)
    sys.exit(0 if success else 1)
