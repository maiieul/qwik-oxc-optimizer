//! Export stripping.
//!
//! Strip exports from the module based on `strip_exports` and `strip_ctx_name`
//! options. Used for server/client mode where certain exports should be removed.

#![allow(unused)]

/// Filter (strip) exports from the program AST.
pub(crate) fn filter_exports<'a>(
    _program: &mut oxc::ast::ast::Program<'a>,
    _strip_exports: &[String],
    _strip_ctx_name: &[String],
    _allocator: &'a oxc::allocator::Allocator,
) {
    todo!("Implement export filtering")
}
