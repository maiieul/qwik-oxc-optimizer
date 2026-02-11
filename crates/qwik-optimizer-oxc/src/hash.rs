//! Segment hash computation.
//!
//! Computes the 11-character hash that appears in segment names
//! (e.g., `zBbHWn4e8Cg` in `renderHeader_zBbHWn4e8Cg`).
//!
//! Algorithm: `DefaultHasher(scope?, rel_path, display_name) -> u64 -> LE bytes -> base64url -> replace -/_ with 0`
//! This is an exact port of the SWC optimizer hash algorithm validated in POC-03.

use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use base64::Engine;

/// Compute the segment hash for a display name.
///
/// Returns an 11-character base64url hash. The exact algorithm matches the
/// SWC optimizer: feed scope (if any), rel_path, and display_name bytes into
/// a `DefaultHasher`, encode the u64 result as LE bytes in base64url, and
/// replace `-` and `_` with `0`.
///
/// # Arguments
/// - `scope` - Optional scope prefix (e.g., package name for monorepos)
/// - `rel_path` - Relative file path (e.g., "test.tsx")
/// - `display_name` - Full display name (e.g., "test.tsx_renderHeader")
pub(crate) fn compute_segment_hash(
    scope: Option<&str>,
    rel_path: &str,
    display_name: &str,
) -> String {
    let mut hasher = DefaultHasher::new();
    if let Some(scope) = scope {
        hasher.write(scope.as_bytes());
    }
    hasher.write(rel_path.as_bytes());
    hasher.write(display_name.as_bytes());
    let hash = hasher.finish();

    let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash.to_le_bytes());
    encoded.replace(['-', '_'], "0")
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

    #[test]
    fn test_hash_determinism() {
        let h1 = compute_segment_hash(None, "test.tsx", "test.tsx_renderHeader");
        let h2 = compute_segment_hash(None, "test.tsx", "test.tsx_renderHeader");
        assert_eq!(h1, h2, "Same inputs must produce the same hash");
    }

    #[test]
    fn test_hash_different_inputs() {
        let h1 = compute_segment_hash(None, "test.tsx", "test.tsx_renderHeader");
        let h2 = compute_segment_hash(None, "test.tsx", "test.tsx_renderHeader_component");
        assert_ne!(h1, h2, "Different inputs must produce different hashes");
    }

    #[test]
    fn test_hash_structural_validity() {
        let hash = compute_segment_hash(None, "test.tsx", "test.tsx_renderHeader");
        assert_eq!(
            hash.len(),
            11,
            "Hash must be 11 characters, got {} ('{}')",
            hash.len(),
            hash
        );
        assert!(
            hash.chars().all(|c| c.is_ascii_alphanumeric()),
            "Hash must be alphanumeric (no - or _), got '{}'",
            hash
        );
    }

    #[test]
    fn test_hash_with_scope_none() {
        let hash = compute_segment_hash(None, "test.tsx", "test.tsx_Foo_component");
        assert_eq!(hash.len(), 11);
        assert!(hash.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_hash_with_scope_some() {
        let hash = compute_segment_hash(Some("test-scope"), "test.tsx", "test.tsx_Foo_component");
        assert_eq!(hash.len(), 11);
        assert!(hash.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_hash_scope_changes_result() {
        let h1 = compute_segment_hash(None, "test.tsx", "test.tsx_Foo_component");
        let h2 = compute_segment_hash(Some("my-scope"), "test.tsx", "test.tsx_Foo_component");
        assert_ne!(h1, h2, "Scope should affect the hash output");
    }

    #[test]
    fn test_hash_different_paths() {
        let h1 = compute_segment_hash(None, "foo.tsx", "foo.tsx_App_component");
        let h2 = compute_segment_hash(None, "bar.tsx", "bar.tsx_App_component");
        assert_ne!(
            h1, h2,
            "Different file paths should produce different hashes"
        );
    }

    #[test]
    fn test_hash_no_dashes_or_underscores() {
        // Run several hashes and confirm none contain - or _
        let test_cases = [
            (None, "a.tsx", "a.tsx_X"),
            (Some("scope"), "b.tsx", "b.tsx_Y"),
            (None, "c.tsx", "c.tsx_Z_component"),
            (None, "d.tsx", "d.tsx_W_useTask"),
        ];
        for (scope, path, display) in &test_cases {
            let hash = compute_segment_hash(*scope, path, display);
            assert!(
                !hash.contains('-') && !hash.contains('_'),
                "Hash should not contain - or _, got '{}' for display='{}'",
                hash,
                display
            );
        }
    }
}
