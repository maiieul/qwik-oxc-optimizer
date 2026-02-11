//! Segment hash computation.
//!
//! Computes the 11-character hash that appears in segment names
//! (e.g., `zBbHWn4e8Cg` in `renderHeader_zBbHWn4e8Cg`).

#![allow(unused)]

/// Compute the segment hash for a display name.
///
/// Returns an 11-character base64url hash. The exact algorithm must match the
/// SWC optimizer for spec compatibility.
///
/// TODO: Port the exact hash algorithm from the SWC optimizer.
pub(crate) fn compute_segment_hash(_display_name: &str) -> String {
    todo!("Port hash algorithm from SWC optimizer")
}

/// Format a full segment name from a display name and hash.
///
/// Example: `format_segment_name("renderHeader", "zBbHWn4e8Cg")` -> `"renderHeader_zBbHWn4e8Cg"`
pub(crate) fn format_segment_name(display_name: &str, hash: &str) -> String {
    format!("{display_name}_{hash}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_segment_name() {
        assert_eq!(
            format_segment_name("renderHeader", "zBbHWn4e8Cg"),
            "renderHeader_zBbHWn4e8Cg"
        );
    }
}
