//! Code generation.
//!
//! Generate JavaScript source code (and optional source maps) from an OXC
//! `Program` AST. Wraps `oxc::codegen::Codegen` with the crate's options
//! (minification, source maps, etc.) and produces `TransformModule` values.

use crate::types::MinifyMode;

/// Options controlling code emission.
pub(crate) struct EmitOptions {
    pub source_maps: bool,
    pub minify: MinifyMode,
}

/// Result of emitting a program to JavaScript source.
pub(crate) struct EmitResult {
    pub code: String,
    pub map: Option<String>,
}

/// Emit a Program AST to JavaScript source code.
///
/// Uses OXC's Codegen to serialize the AST back to JavaScript.
/// Source maps are deferred to Phase 13 (returns None for map).
/// Minification is deferred to Phase 12.
pub(crate) fn emit_module<'a>(
    program: &oxc::ast::ast::Program<'a>,
    source: &str,
    _options: &EmitOptions,
) -> EmitResult {
    let codegen_result = oxc::codegen::Codegen::new()
        .with_source_text(source)
        .build(program);

    EmitResult {
        code: codegen_result.code,
        map: None, // Source maps deferred to Phase 13
    }
}
