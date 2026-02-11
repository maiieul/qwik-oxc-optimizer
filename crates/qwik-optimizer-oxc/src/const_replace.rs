//! Constant inlining.
//!
//! Replace references to compile-time constants with their values. Used for
//! build-time configuration (e.g., `import.meta.env` replacements, `isDev`
//! constants).

#![allow(unused)]

use std::collections::HashMap;

/// Replace references to compile-time constants with their literal values.
pub(crate) fn replace_constants<'a>(
    _program: &mut oxc::ast::ast::Program<'a>,
    _constants: &HashMap<String, String>,
    _allocator: &'a oxc::allocator::Allocator,
) {
    todo!("Implement constant replacement")
}
