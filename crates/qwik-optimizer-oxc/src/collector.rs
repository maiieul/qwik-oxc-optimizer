//! First-pass AST analysis.
//!
//! Walk the parsed AST once (before transformation) to collect information
//! needed by the transform pass: which imports come from `@qwik.dev/core`,
//! which of those are `$`-suffixed, where `$()` call sites appear, and what
//! the module exports. This is a read-only pass -- it does not mutate the AST.

#![allow(unused)]

use crate::types::CollectResult;

/// Perform first-pass analysis of the parsed module.
pub(crate) fn collect<'a>(
    _program: &oxc::ast::ast::Program<'a>,
    _scoping: &oxc::semantic::Scoping,
) -> CollectResult {
    todo!("Implement collector pass")
}
