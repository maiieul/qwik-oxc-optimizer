//! Const evaluation utilities.
//!
//! Determine whether an expression is a compile-time constant. Used by
//! `const_replace.rs` and the transform pass to decide what can be safely inlined.

#![allow(unused)]

/// Determine whether an expression is a compile-time constant.
///
/// Returns `true` for literals, template literals with no expressions,
/// and other statically-known values.
pub(crate) fn is_const_expression(_expr: &oxc::ast::ast::Expression<'_>) -> bool {
    todo!("Implement const expression detection")
}
