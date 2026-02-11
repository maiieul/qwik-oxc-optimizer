//! Module parsing.
//!
//! Parse a single source file (JS/TS/JSX/TSX) into an OXC `Program` AST
//! with semantic scoping from `SemanticBuilder`. Handles source type detection
//! from filename extension and reports parse errors as `Diagnostic` values.

use crate::errors;
use crate::types::Diagnostic;
use oxc::semantic::Scoping;

/// Result of parsing a single source file.
pub(crate) struct ParseResult<'a> {
    pub program: oxc::ast::ast::Program<'a>,
    pub source_type: oxc::span::SourceType,
    pub scoping: Scoping,
}

// Manual Debug impl because oxc::ast::ast::Program does not derive Debug
impl std::fmt::Debug for ParseResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseResult")
            .field("source_type", &self.source_type)
            .field("program.body.len", &self.program.body.len())
            .finish()
    }
}

/// Detect `SourceType` from a filename extension.
///
/// - `.tsx` -> TSX (TypeScript + JSX)
/// - `.ts`  -> TypeScript with JSX enabled (Qwik allows JSX in .ts files)
/// - `.jsx` -> JSX (JavaScript + JSX)
/// - `.js` / `.mjs` / `.cjs` -> ESM module (JavaScript)
/// - Default: ESM module
fn source_type_from_filename(filename: &str) -> oxc::span::SourceType {
    if filename.ends_with(".tsx") {
        oxc::span::SourceType::tsx()
    } else if filename.ends_with(".ts") {
        // Qwik allows JSX in .ts files, so enable JSX
        oxc::span::SourceType::ts().with_jsx(true)
    } else if filename.ends_with(".jsx") {
        oxc::span::SourceType::jsx()
    } else if filename.ends_with(".js")
        || filename.ends_with(".mjs")
        || filename.ends_with(".cjs")
    {
        oxc::span::SourceType::mjs()
    } else {
        oxc::span::SourceType::mjs()
    }
}

/// Parse a single source file into an OXC Program AST with semantic scoping.
///
/// The `source` must have lifetime `'a` tied to the allocator so the AST
/// can reference it. Returns `Err(diagnostics)` if parse errors are found.
pub(crate) fn parse_module<'a>(
    allocator: &'a oxc::allocator::Allocator,
    source: &'a str,
    filename: &str,
) -> Result<ParseResult<'a>, Vec<Diagnostic>> {
    let source_type = source_type_from_filename(filename);

    // Parse source into AST
    let ret = oxc::parser::Parser::new(allocator, source, source_type).parse();

    // Check for parse errors
    if !ret.errors.is_empty() {
        let diagnostics: Vec<Diagnostic> = ret
            .errors
            .iter()
            .map(|err| errors::create_source_error(&err.to_string(), filename))
            .collect();
        return Err(diagnostics);
    }

    let program = ret.program;

    // Build semantic scoping
    let semantic_ret = oxc::semantic::SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(&program);
    let scoping = semantic_ret.semantic.into_scoping();

    Ok(ParseResult {
        program,
        source_type,
        scoping,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxc::allocator::Allocator;

    #[test]
    fn test_parse_tsx_source() {
        let allocator = Allocator::default();
        let source = r#"import { component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return <div>Hello</div>;
});"#;

        let result = parse_module(&allocator, source, "app.tsx");
        assert!(result.is_ok(), "Expected successful parse of TSX source");

        let parsed = result.unwrap();
        assert!(parsed.source_type.is_typescript());
        assert!(parsed.source_type.is_jsx());
        // Program body should have statements
        assert!(!parsed.program.body.is_empty());
    }

    #[test]
    fn test_parse_ts_source() {
        let allocator = Allocator::default();
        let source = r#"const x: number = 42;"#;

        let result = parse_module(&allocator, source, "utils.ts");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(parsed.source_type.is_typescript());
        // JSX is enabled for .ts in Qwik
        assert!(parsed.source_type.is_jsx());
    }

    #[test]
    fn test_parse_jsx_source() {
        let allocator = Allocator::default();
        let source = r#"export const App = () => <div>Hello</div>;"#;

        let result = parse_module(&allocator, source, "app.jsx");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(!parsed.source_type.is_typescript());
        assert!(parsed.source_type.is_jsx());
    }

    #[test]
    fn test_parse_js_source() {
        let allocator = Allocator::default();
        let source = r#"export const x = 1;"#;

        let result = parse_module(&allocator, source, "utils.js");
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert!(!parsed.source_type.is_typescript());
    }

    #[test]
    fn test_parse_mjs_source() {
        let allocator = Allocator::default();
        let source = r#"export const x = 1;"#;

        let result = parse_module(&allocator, source, "utils.mjs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_error_returns_diagnostics() {
        let allocator = Allocator::default();
        let source = r#"export const = ;"#;

        let result = parse_module(&allocator, source, "bad.tsx");
        assert!(result.is_err(), "Expected parse error for invalid syntax");

        let diagnostics = result.unwrap_err();
        assert!(!diagnostics.is_empty());
        // Should be a SourceError category
        assert!(matches!(
            diagnostics[0].category,
            crate::types::DiagnosticCategory::SourceError
        ));
        assert_eq!(diagnostics[0].file, "bad.tsx");
    }

    #[test]
    fn test_parse_returns_scoping() {
        let allocator = Allocator::default();
        let source = r#"
import { $ } from '@qwik.dev/core';
const x = $(() => {
    const inner = 1;
    return inner;
});
"#;

        let result = parse_module(&allocator, source, "test.tsx");
        assert!(result.is_ok());
        // Scoping is available (not a direct assertion on content,
        // but confirms SemanticBuilder completed successfully)
        let _scoping = &result.unwrap().scoping;
    }

    #[test]
    fn test_source_type_detection() {
        assert!(source_type_from_filename("app.tsx").is_typescript());
        assert!(source_type_from_filename("app.tsx").is_jsx());

        assert!(source_type_from_filename("app.ts").is_typescript());
        assert!(source_type_from_filename("app.ts").is_jsx());

        assert!(!source_type_from_filename("app.jsx").is_typescript());
        assert!(source_type_from_filename("app.jsx").is_jsx());

        assert!(!source_type_from_filename("app.js").is_typescript());
        assert!(!source_type_from_filename("app.mjs").is_typescript());
        assert!(!source_type_from_filename("app.cjs").is_typescript());

        // Unknown extension defaults to mjs
        assert!(!source_type_from_filename("app.txt").is_typescript());
    }
}
