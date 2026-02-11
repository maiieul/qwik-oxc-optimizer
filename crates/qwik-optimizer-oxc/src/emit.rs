//! Code generation.
//!
//! Generate JavaScript source code (and optional source maps) from an OXC
//! `Program` AST. Wraps `oxc_codegen::Codegen` with the crate's options
//! (minification, source maps, etc.) and produces `TransformModule` values.

#![allow(unused)]

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
pub(crate) fn emit_module<'a>(
    _program: &oxc::ast::ast::Program<'a>,
    _source: &str,
    _options: &EmitOptions,
) -> EmitResult {
    todo!("Implement code emission with OXC codegen")
}
