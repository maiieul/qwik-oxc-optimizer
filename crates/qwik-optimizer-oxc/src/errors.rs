//! Diagnostic types and error helpers.
//!
//! Helper functions for creating `Diagnostic` values with consistent formatting.
//! Centralizes error message templates so the rest of the crate can report errors
//! without constructing Diagnostic structs manually.

#![allow(unused)]

use crate::types::{Diagnostic, DiagnosticCategory, SourceLocation};

/// Create an error diagnostic.
pub(crate) fn create_error(message: &str, file: &str) -> Diagnostic {
    Diagnostic {
        category: DiagnosticCategory::Error,
        code: None,
        file: file.to_string(),
        message: message.to_string(),
        highlights: None,
        suggestions: None,
    }
}

/// Create a warning diagnostic.
pub(crate) fn create_warning(message: &str, file: &str) -> Diagnostic {
    Diagnostic {
        category: DiagnosticCategory::Warning,
        code: None,
        file: file.to_string(),
        message: message.to_string(),
        highlights: None,
        suggestions: None,
    }
}

/// Create a source error diagnostic (e.g., syntax error).
pub(crate) fn create_source_error(message: &str, file: &str) -> Diagnostic {
    Diagnostic {
        category: DiagnosticCategory::SourceError,
        code: None,
        file: file.to_string(),
        message: message.to_string(),
        highlights: None,
        suggestions: None,
    }
}
