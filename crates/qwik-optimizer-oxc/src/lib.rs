//! Qwik optimizer using OXC for code transformation.
//!
//! This crate provides `transform_modules()`, which takes one or more input modules
//! and applies Qwik's $-call extraction, import rewriting, and segment splitting
//! transformations. The output matches the SWC optimizer's JSON wire format,
//! enabling drop-in replacement at the TypeScript binding layer.

mod types;
mod errors;
mod words;
mod hash;
mod parse;
mod collector;
mod transform;
mod import_rewrite;
mod code_move;
mod entry_strategy;
mod emit;
mod filter_exports;
mod props_destructuring;
mod is_const;
mod const_replace;

// Re-export public types
pub use types::{
    CtxKind, Diagnostic, DiagnosticCategory, EmitMode, EntryStrategy, MinifyMode,
    SegmentAnalysis, SourceLocation, TransformModule, TransformModuleInput,
    TransformModulesOptions, TransformOutput,
};

/// Main entry point: transform one or more modules.
///
/// Accepts a `TransformModulesOptions` configuration and returns a `TransformOutput`
/// containing all transformed modules (main + extracted segments) and any diagnostics.
///
/// This is a stub implementation that returns an empty output. Real implementation
/// will be added in subsequent phases.
pub fn transform_modules(
    _config: TransformModulesOptions,
) -> Result<TransformOutput, anyhow::Error> {
    // Stub: return empty output
    Ok(TransformOutput {
        modules: vec![],
        diagnostics: vec![],
        is_type_script: false,
        is_jsx: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_modules_stub() {
        let config = TransformModulesOptions::default();
        let result = transform_modules(config).unwrap();

        assert!(result.modules.is_empty());
        assert!(result.diagnostics.is_empty());
        assert!(!result.is_type_script);
        assert!(!result.is_jsx);
    }

    #[test]
    fn test_transform_modules_with_input() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: "const x = 1;".to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };

        let result = transform_modules(config).unwrap();

        // Stub returns empty output regardless of input
        assert!(result.modules.is_empty());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_serde_roundtrip_options() {
        let opts = TransformModulesOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        let _: TransformModulesOptions = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_serde_camel_case() {
        let opts = TransformModulesOptions {
            src_dir: "src".to_string(),
            source_maps: false,
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };

        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains("\"srcDir\""), "Expected srcDir, got: {json}");
        assert!(
            json.contains("\"sourceMaps\""),
            "Expected sourceMaps, got: {json}"
        );
        assert!(
            json.contains("\"entryStrategy\""),
            "Expected entryStrategy, got: {json}"
        );
    }
}
