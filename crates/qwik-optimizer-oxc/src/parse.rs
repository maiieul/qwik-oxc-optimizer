//! Module parsing.
//!
//! Parse a single source file (JS/TS/JSX/TSX) into an OXC `Program` AST.
//! Handles source type detection from filename extension and reports parse
//! errors as `Diagnostic` values.

#![allow(unused)]

use crate::types::Diagnostic;

/// Result of parsing a single source file.
pub(crate) struct ParseResult<'a> {
    pub program: oxc::ast::ast::Program<'a>,
    pub source_type: oxc::span::SourceType,
}

/// Parse a single source file into an OXC Program AST.
pub(crate) fn parse_module<'a>(
    _allocator: &'a oxc::allocator::Allocator,
    _source: &str,
    _filename: &str,
) -> Result<ParseResult<'a>, Vec<Diagnostic>> {
    todo!("Implement module parsing with OXC parser")
}
