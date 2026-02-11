//! Segment extraction and code generation.
//!
//! After the transform pass identifies segments and serializes their bodies,
//! this module constructs complete JavaScript module source code for each
//! extracted segment. Each segment becomes its own module file containing
//! the extracted function body as an exported const, with any needed imports.

use crate::types::{SegmentData, TransformOptions};

/// Build a segment's JavaScript source code.
///
/// Takes the serialized body code and segment metadata, constructs a
/// complete JavaScript module string with:
/// 1. Framework imports (`import { _captures } from "@qwik.dev/core"`)
/// 2. QRL import for nested $-calls (`import { qrl } from "@qwik.dev/core"`)
/// 3. Lazy import declarations (`const i_hash = () => import(...)`)
/// 4. Capture restoration statements (`const varName = _captures[N]`)
/// 5. Export declaration (`export const name = body`)
///
/// Returns the complete JavaScript module source code.
pub(crate) fn build_segment_code(
    body_code: &str,
    segment: &SegmentData,
    options: &TransformOptions,
) -> String {
    let mut parts: Vec<String> = Vec::new();

    // 1. Add _captures import if segment has captures
    if segment.captures && !segment.capture_names.is_empty() {
        parts.push(format!(
            "import {{ _captures }} from \"{}\";",
            options.core_module
        ));
    }

    // 2. Add qrl import if segment has child $()-calls
    if segment.needs_qrl_import {
        parts.push(format!(
            "import {{ qrl }} from \"{}\";",
            options.core_module
        ));
    }

    // 3. Add lazy import declarations for child segments
    for (hash, import_path) in &segment.child_lazy_imports {
        parts.push(format!(
            "const i_{} = () => import(\"{}\");",
            hash, import_path
        ));
    }

    // 4. Build the export declaration with capture restoration
    let segment_name = &segment.name;
    if segment.captures && !segment.capture_names.is_empty() {
        // Build capture restoration statements:
        // const varName = _captures[0]; const varName2 = _captures[1]; etc.
        let capture_stmts: Vec<String> = segment
            .capture_names
            .iter()
            .enumerate()
            .map(|(i, name)| format!("const {} = _captures[{}]", name, i))
            .collect();

        // Inject captures into the body and wrap in export
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

    let capture_code: String = capture_stmts
        .iter()
        .map(|s| format!("{};\n", s))
        .collect();

    // Find the arrow (`=>`) to determine what follows
    if let Some(arrow_pos) = find_arrow_position(body_code) {
        let after_arrow = body_code[arrow_pos + 2..].trim_start();

        if after_arrow.starts_with('{') {
            // Block body: inject after opening {
            // Find the actual `{` position in the original string
            let brace_offset = body_code[arrow_pos + 2..]
                .find('{')
                .map(|p| arrow_pos + 2 + p);

            if let Some(brace_pos) = brace_offset {
                let before = &body_code[..brace_pos + 1];
                let after = &body_code[brace_pos + 1..];
                return format!("{}\n{}{}", before, capture_code, after);
            }
        }

        // Expression body: wrap in block with return
        let prefix = &body_code[..arrow_pos + 2];
        let expr_body = body_code[arrow_pos + 2..].trim();
        // Remove trailing semicolon if present
        let expr_body = expr_body.strip_suffix(';').unwrap_or(expr_body);
        return format!(
            "{} {{\n{}return {};\n}}",
            prefix, capture_code, expr_body
        );
    }

    // Fallback: if we can't parse the arrow, just prepend captures as a comment
    // This shouldn't happen for valid arrow functions
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

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CtxKind;

    fn make_test_options() -> TransformOptions {
        TransformOptions {
            src_dir: ".".to_string(),
            root_dir: None,
            source_maps: false,
            minify: crate::types::MinifyMode::None,
            transpile_ts: false,
            transpile_jsx: false,
            preserve_filenames: false,
            entry_strategy: crate::types::EntryStrategy::Segment,
            explicit_extensions: false,
            mode: crate::types::EmitMode::Lib,
            scope: None,
            core_module: "@qwik.dev/core".to_string(),
            strip_exports: vec![],
            strip_ctx_name: vec![],
            strip_event_handlers: false,
            reg_ctx_name: vec![],
            is_server: false,
        }
    }

    fn make_segment(name: &str) -> SegmentData {
        SegmentData {
            display_name: format!("test.tsx_{}", name),
            hash: "abc123".to_string(),
            name: format!("{}_{}", name, "abc123"),
            ctx_name: "$".to_string(),
            ctx_kind: CtxKind::Function,
            origin: "test.tsx".to_string(),
            extension: "tsx".to_string(),
            span: (0, 100),
            parent: None,
            captures: false,
            capture_names: vec![],
            needed_imports: vec![],
            body_span: (10, 90),
            param_names: vec![],
            body_code: String::new(),
            child_lazy_imports: vec![],
            needs_qrl_import: false,
        }
    }

    #[test]
    fn test_simple_segment_no_deps() {
        let options = make_test_options();
        let segment = make_segment("handler");
        let body_code = "() => console.log(\"hello\")";

        let result = build_segment_code(body_code, &segment, &options);

        assert!(
            result.contains("export const handler_abc123 = () => console.log(\"hello\")"),
            "Expected export declaration: {}",
            result
        );
        // No imports needed
        assert!(
            !result.contains("import"),
            "Should not have imports: {}",
            result
        );
    }

    #[test]
    fn test_segment_with_captures() {
        let options = make_test_options();
        let mut segment = make_segment("handler");
        segment.captures = true;
        segment.capture_names = vec!["state".to_string(), "count".to_string()];

        let body_code = "() => state.count";

        let result = build_segment_code(body_code, &segment, &options);

        // Should have _captures import
        assert!(
            result.contains("import { _captures } from \"@qwik.dev/core\""),
            "Expected _captures import: {}",
            result
        );
        // Should have capture restoration
        assert!(
            result.contains("const state = _captures[0]"),
            "Expected state restoration: {}",
            result
        );
        assert!(
            result.contains("const count = _captures[1]"),
            "Expected count restoration: {}",
            result
        );
        // Should have export with modified body
        assert!(
            result.contains("export const handler_abc123"),
            "Expected export: {}",
            result
        );
    }

    #[test]
    fn test_segment_with_captures_block_body() {
        let options = make_test_options();
        let mut segment = make_segment("handler");
        segment.captures = true;
        segment.capture_names = vec!["state".to_string()];

        let body_code = "() => {\n  return state.count;\n}";

        let result = build_segment_code(body_code, &segment, &options);

        // Should inject capture restoration after opening {
        assert!(
            result.contains("const state = _captures[0]"),
            "Expected capture restoration: {}",
            result
        );
        assert!(
            result.contains("return state.count"),
            "Expected original body preserved: {}",
            result
        );
    }

    #[test]
    fn test_segment_with_child_lazy_imports() {
        let options = make_test_options();
        let mut segment = make_segment("App_component");
        segment.needs_qrl_import = true;
        segment.child_lazy_imports = vec![
            ("xyz789".to_string(), "./test.tsx_App_component_1_xyz789".to_string()),
        ];

        let body_code = "() => {\n  return qrl(i_xyz789, \"App_component_1_xyz789\");\n}";

        let result = build_segment_code(body_code, &segment, &options);

        // Should have qrl import
        assert!(
            result.contains("import { qrl } from \"@qwik.dev/core\""),
            "Expected qrl import: {}",
            result
        );
        // Should have lazy import declaration
        assert!(
            result.contains("const i_xyz789 = () => import(\"./test.tsx_App_component_1_xyz789\")"),
            "Expected lazy import: {}",
            result
        );
        // Should have export
        assert!(
            result.contains("export const App_component_abc123"),
            "Expected export: {}",
            result
        );
    }

    #[test]
    fn test_find_arrow_position() {
        assert_eq!(find_arrow_position("() => 1"), Some(3));
        assert_eq!(find_arrow_position("(a, b) => a + b"), Some(7));
        assert_eq!(find_arrow_position("x => x"), Some(2));
        assert_eq!(find_arrow_position("(a = '=> ') => a"), Some(12));
        assert_eq!(find_arrow_position("const x = 1"), None);
    }

    #[test]
    fn test_inject_captures_expression_body() {
        let body = "() => state.count";
        let captures = vec!["const state = _captures[0]".to_string()];
        let result = inject_captures_into_body(body, &captures);

        assert!(
            result.contains("const state = _captures[0]"),
            "Expected capture stmt: {}",
            result
        );
        assert!(
            result.contains("return state.count"),
            "Expected return: {}",
            result
        );
    }

    #[test]
    fn test_inject_captures_block_body() {
        let body = "() => {\n  return x;\n}";
        let captures = vec!["const x = _captures[0]".to_string()];
        let result = inject_captures_into_body(body, &captures);

        assert!(
            result.contains("const x = _captures[0]"),
            "Expected capture stmt: {}",
            result
        );
        assert!(
            result.contains("return x;"),
            "Expected original body: {}",
            result
        );
    }
}
