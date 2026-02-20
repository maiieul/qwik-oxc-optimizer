//! Segment extraction and code generation.
//!
//! After the transform pass identifies segments and serializes their bodies,
//! this module constructs complete JavaScript module source code for each
//! extracted segment. Each segment becomes its own module file containing
//! the extracted function body as an exported const, with any needed imports.

use crate::emit::swc_codegen_options;
use crate::types::{ImportKind, SegmentData, TransformOptions};

/// A single import to emit in a segment module, sortable by local name.
///
/// Used to collect all segment imports (framework + user-code) into a single
/// list that can be sorted alphabetically by `local_name`, matching SWC's
/// `local_idents.sort()` behavior.
struct SegmentImportEntry {
    /// The local binding name (e.g., "_jsxSorted", "useStore", "dep3").
    local_name: String,
    /// The import source module (e.g., "@qwik.dev/core", "dep3/something").
    source: String,
    /// The kind of import (default, namespace, or named).
    kind: ImportKind,
    /// For aliased imports, the original imported name (e.g., "Fragment" for "_Fragment").
    imported_name: Option<String>,
}

/// Build a segment's JavaScript source code with optional hoisted function declarations.
///
/// Takes the serialized body code and segment metadata, constructs a
/// complete JavaScript module string with:
/// 1. `_captures` import first (if needed, matches SWC's special-case emission)
/// 2. Remaining imports sorted alphabetically by local binding name (framework + user-code)
/// 3. Lazy import declarations (`const i_hash = () => import(...)`)
/// 4. Hoisted function declarations for _fnSignal (`const _hfN = ...`)
/// 5. Capture restoration statements (`const varName = _captures[N]`)
/// 6. Export declaration (`export const name = body`)
///
/// Import ordering matches SWC's segment module emission: `_captures` is emitted
/// first (special case in SWC's code_move.rs), then remaining imports follow
/// `local_idents.sort()` order (alphabetical by local binding name).
///
/// Returns the complete JavaScript module source code.
pub(crate) fn build_segment_code_with_hoisted(
    body_code: &str,
    segment: &SegmentData,
    options: &TransformOptions,
    hoisted_stmts: &[(String, String)],
    custom_jsx_source: Option<&str>,
) -> String {
    let core = &options.core_module;
    let jsx_runtime = format!("{}/jsx-runtime", core);
    let has_captures = segment.captures && !segment.capture_names.is_empty();

    // --- Phase 1: Collect all needed imports into a sortable list ---
    // Note: _captures is emitted separately (first), matching SWC's special-case behavior.

    let mut imports: Vec<SegmentImportEntry> = Vec::new();

    // qrl import (needed when segment has child $()-calls)
    if segment.needs_qrl_import {
        imports.push(SegmentImportEntry {
            local_name: "qrl".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }

    // Qrl-suffixed imports needed by this segment (from nested $-calls)
    for qrl_name in &segment.segment_qrl_names {
        imports.push(SegmentImportEntry {
            local_name: qrl_name.clone(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }

    // Framework imports detected by body_code.contains()
    // Custom JSX source check: when set, emit _jsx from custom source instead of _jsxSorted from core
    if let Some(jsx_source) = custom_jsx_source {
        if body_code.contains("_jsx") {
            imports.push(SegmentImportEntry {
                local_name: "_jsx".to_string(),
                source: format!("{}/jsx-runtime", jsx_source),
                kind: ImportKind::Named,
                imported_name: Some("jsx".to_string()),
            });
        }
    } else if body_code.contains("_jsxSorted") {
        imports.push(SegmentImportEntry {
            local_name: "_jsxSorted".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_jsxSplit") {
        imports.push(SegmentImportEntry {
            local_name: "_jsxSplit".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_fnSignal") || !hoisted_stmts.is_empty() {
        imports.push(SegmentImportEntry {
            local_name: "_fnSignal".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_wrapProp") {
        imports.push(SegmentImportEntry {
            local_name: "_wrapProp".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_Fragment") {
        imports.push(SegmentImportEntry {
            local_name: "_Fragment".to_string(),
            source: jsx_runtime.clone(),
            kind: ImportKind::Named,
            imported_name: Some("Fragment".to_string()),
        });
    }
    if body_code.contains("inlinedQrl") {
        imports.push(SegmentImportEntry {
            local_name: "inlinedQrl".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_noopQrl") {
        imports.push(SegmentImportEntry {
            local_name: "_noopQrl".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_qrlSync") {
        imports.push(SegmentImportEntry {
            local_name: "_qrlSync".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_getVarProps") {
        imports.push(SegmentImportEntry {
            local_name: "_getVarProps".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_getConstProps") {
        imports.push(SegmentImportEntry {
            local_name: "_getConstProps".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_restProps") {
        imports.push(SegmentImportEntry {
            local_name: "_restProps".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_chk") {
        imports.push(SegmentImportEntry {
            local_name: "_chk".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }
    if body_code.contains("_val") {
        imports.push(SegmentImportEntry {
            local_name: "_val".to_string(),
            source: core.clone(),
            kind: ImportKind::Named,
            imported_name: None,
        });
    }

    // User-code imports needed by this segment body.
    // These are imports from the original module that the segment references
    // (e.g., `import dep3 from "dep3/something"`, `import { bar as bbar } from "../state"`).
    for import_info in &segment.needed_imports {
        for (idx, spec_name) in import_info.specifiers.iter().enumerate() {
            let kind = import_info
                .specifier_kinds
                .get(idx)
                .unwrap_or(&ImportKind::Named);
            let imported_name = if matches!(kind, ImportKind::Named) {
                import_info.specifier_aliases.get(spec_name).cloned()
            } else {
                None
            };
            imports.push(SegmentImportEntry {
                local_name: spec_name.clone(),
                source: import_info.source.clone(),
                kind: kind.clone(),
                imported_name,
            });
        }
    }

    // --- Phase 2: Sort all imports alphabetically by local name ---
    // Matches SWC's `local_idents.sort()` behavior.
    imports.sort_by(|a, b| a.local_name.cmp(&b.local_name));

    // --- Phase 3: Emit imports ---
    // _captures is emitted first (SWC emits it before the sorted local_idents loop).
    let mut parts: Vec<String> = Vec::new();

    if has_captures {
        parts.push(format!("import {{ _captures }} from \"{}\";", core));
    }

    // Emit remaining sorted imports (one per identifier, no merging -- matches SWC).
    for entry in &imports {
        match entry.kind {
            ImportKind::Default => {
                parts.push(format!(
                    "import {} from \"{}\";",
                    entry.local_name, entry.source
                ));
            }
            ImportKind::Namespace => {
                parts.push(format!(
                    "import * as {} from \"{}\";",
                    entry.local_name, entry.source
                ));
            }
            ImportKind::Named => {
                if let Some(ref imported) = entry.imported_name {
                    parts.push(format!(
                        "import {{ {} as {} }} from \"{}\";",
                        imported, entry.local_name, entry.source
                    ));
                } else {
                    parts.push(format!(
                        "import {{ {} }} from \"{}\";",
                        entry.local_name, entry.source
                    ));
                }
            }
        }
    }

    // --- Phase 4: Lazy import declarations (const, not import statements) ---
    for (hash, import_path) in &segment.child_lazy_imports {
        parts.push(format!(
            "const i_{} = () => import(\"{}\");",
            hash, import_path
        ));
    }

    // --- Phase 5: Hoisted function declarations ---
    for (fn_code, str_code) in hoisted_stmts {
        parts.push(fn_code.clone());
        parts.push(str_code.clone());
    }

    // --- Phase 6: Capture restoration + export ---
    let segment_name = &segment.name;
    if has_captures {
        let capture_stmts: Vec<String> = segment
            .capture_names
            .iter()
            .enumerate()
            .map(|(i, name)| format!("const {} = _captures[{}]", name, i))
            .collect();

        let modified_body = inject_captures_into_body(body_code, &capture_stmts);
        parts.push(format!("export const {} = {}", segment_name, modified_body));
    } else {
        parts.push(format!("export const {} = {}", segment_name, body_code));
    }

    parts.join("\n")
}

/// Inject capture restoration statements into an arrow function body.
///
/// For block bodies like `() => { ... }`:
///   Insert capture stmts after the opening `{`.
///
/// For expression bodies like `() => expr`:
///   Convert to `() => { const x = _captures[0]; return expr; }`
fn inject_captures_into_body(body_code: &str, capture_stmts: &[String]) -> String {
    if capture_stmts.is_empty() {
        return body_code.to_string();
    }

    let capture_code: String = capture_stmts.iter().map(|s| format!("{};\n", s)).collect();

    if let Some(arrow_pos) = find_arrow_position(body_code) {
        let after_arrow = body_code[arrow_pos + 2..].trim_start();

        if after_arrow.starts_with('{') {
            let brace_offset = body_code[arrow_pos + 2..]
                .find('{')
                .map(|p| arrow_pos + 2 + p);

            if let Some(brace_pos) = brace_offset {
                let before = &body_code[..brace_pos + 1];
                let after = &body_code[brace_pos + 1..];
                return format!("{}\n{}{}", before, capture_code, after);
            }
        }

        let prefix = &body_code[..arrow_pos + 2];
        let expr_body = body_code[arrow_pos + 2..].trim();
        let expr_body = expr_body.strip_suffix(';').unwrap_or(expr_body);
        return format!("{} {{\n{}return {};\n}}", prefix, capture_code, expr_body);
    }

    body_code.to_string()
}

/// Find the position of the `=>` arrow operator in an arrow function string.
/// Skips arrows inside parenthesized parameter lists.
fn find_arrow_position(code: &str) -> Option<usize> {
    let bytes = code.as_bytes();
    let mut paren_depth = 0;
    let mut in_string = false;
    let mut string_char: u8 = 0;
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        // Track string literals
        if in_string {
            if b == string_char && (i == 0 || bytes[i - 1] != b'\\') {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if b == b'\'' || b == b'"' || b == b'`' {
            in_string = true;
            string_char = b;
            i += 1;
            continue;
        }

        // Track parentheses depth
        if b == b'(' {
            paren_depth += 1;
        } else if b == b')' {
            paren_depth -= 1;
        }

        // Look for `=>` at top level (outside parens)
        if b == b'=' && i + 1 < bytes.len() && bytes[i + 1] == b'>' && paren_depth == 0 {
            return Some(i);
        }

        i += 1;
    }

    None
}

/// Emit segment code with optional source map generation.
///
/// Parses the raw segment JavaScript string, runs Codegen with optional
/// `source_map_path`, and returns (normalized_code, optional_map_json).
///
/// Since segment code is string-constructed (not AST-extracted with preserved
/// spans), the source maps produce identity-like mappings (line N maps to
/// line N in the re-parsed code). This is still useful for debugging as it
/// provides column-level mapping within each generated line.
pub(crate) fn emit_segment_with_map(
    raw_code: &str,
    segment_filename: &str,
    source_maps: bool,
) -> (String, Option<String>) {
    let allocator = oxc::allocator::Allocator::default();
    let source_in_arena = allocator.alloc_str(raw_code);
    let source_type = oxc::span::SourceType::jsx();
    let ret = oxc::parser::Parser::new(&allocator, source_in_arena, source_type).parse();
    if ret.panicked || !ret.errors.is_empty() {
        return (raw_code.to_string(), None);
    }

    let sm_path = if source_maps {
        use std::path::PathBuf;
        Some(PathBuf::from(segment_filename))
    } else {
        None
    };
    let codegen_result = oxc::codegen::Codegen::new()
        .with_options(swc_codegen_options(sm_path))
        .with_source_text(source_in_arena)
        .build(&ret.program);

    let map = codegen_result.map.map(|sm| sm.to_json_string());
    let code = crate::emit::expand_single_prop_objects(&codegen_result.code);
    (code, map)
}
