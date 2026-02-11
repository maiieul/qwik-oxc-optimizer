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

/// Normalize JavaScript code by parsing and re-emitting.
///
/// Parses the string as a JavaScript module, then runs Codegen to produce
/// consistently formatted output. If parsing fails, returns the original
/// code unchanged (graceful degradation for string-constructed code).
pub(crate) fn normalize_code(code: &str) -> String {
    let allocator = oxc::allocator::Allocator::default();
    let source_in_arena = allocator.alloc_str(code);
    let source_type = oxc::span::SourceType::mjs();
    let ret = oxc::parser::Parser::new(&allocator, source_in_arena, source_type).parse();
    if ret.panicked || !ret.errors.is_empty() {
        // If parsing fails, return original code unchanged
        return code.to_string();
    }
    oxc::codegen::Codegen::new()
        .with_source_text(source_in_arena)
        .build(&ret.program)
        .code
}
