//! Import mutation logic.
//!
//! Transform import declarations after the main traversal. Removes `$`-suffixed
//! imports that were consumed by the transform, adds new Qrl-suffixed imports,
//! and adds `qrl` or `inlinedQrl` imports as needed.

#![allow(unused)]

use crate::types::ImportInfo;

/// Describes what imports to add/remove/keep after transformation.
pub(crate) struct ImportChanges {
    /// Import specifiers to add (e.g., "componentQrl", "qrl").
    pub to_add: Vec<String>,

    /// Import specifiers to remove (e.g., "component$").
    pub to_remove: Vec<String>,

    /// Import specifiers to keep unchanged.
    pub to_keep: Vec<String>,
}

/// Compute what import changes are needed after transformation.
pub(crate) fn compute_import_changes(
    _original_imports: &[ImportInfo],
    _needed_qrl_names: &[String],
    _needs_qrl: bool,
    _needs_inlined_qrl: bool,
) -> ImportChanges {
    todo!("Implement import change computation")
}
