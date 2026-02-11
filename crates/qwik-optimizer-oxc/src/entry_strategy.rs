//! EntryStrategy application.
//!
//! Apply the configured `EntryStrategy` to determine how segments are output.
//! The strategy controls whether segments become separate files (Segment),
//! stay inline (Inline), are hoisted, etc.

#![allow(unused)]

use crate::types::{EntryStrategy, SegmentData};

/// Determine whether the strategy results in inline segments.
pub(crate) fn should_inline(strategy: &EntryStrategy) -> bool {
    matches!(strategy, EntryStrategy::Inline)
}

/// Determine whether the strategy results in extracted segments.
pub(crate) fn should_extract(strategy: &EntryStrategy) -> bool {
    !matches!(strategy, EntryStrategy::Inline)
}

/// Determine whether the strategy results in hoisted segments.
pub(crate) fn should_hoist(strategy: &EntryStrategy) -> bool {
    matches!(
        strategy,
        EntryStrategy::Hoist
            | EntryStrategy::Smart
            | EntryStrategy::Component
            | EntryStrategy::Hook
    )
}

/// Determine whether the strategy needs separate files for segments.
/// Only the Segment strategy produces separate files.
pub(crate) fn needs_separate_file(strategy: &EntryStrategy) -> bool {
    matches!(strategy, EntryStrategy::Segment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_inline() {
        assert!(should_inline(&EntryStrategy::Inline));
        assert!(!should_inline(&EntryStrategy::Segment));
        assert!(!should_inline(&EntryStrategy::Hoist));
        assert!(!should_inline(&EntryStrategy::Smart));
    }

    #[test]
    fn test_should_extract() {
        assert!(!should_extract(&EntryStrategy::Inline));
        assert!(should_extract(&EntryStrategy::Segment));
        assert!(should_extract(&EntryStrategy::Hoist));
    }

    #[test]
    fn test_should_hoist() {
        assert!(should_hoist(&EntryStrategy::Hoist));
        assert!(should_hoist(&EntryStrategy::Smart));
        assert!(should_hoist(&EntryStrategy::Component));
        assert!(should_hoist(&EntryStrategy::Hook));
        assert!(!should_hoist(&EntryStrategy::Inline));
        assert!(!should_hoist(&EntryStrategy::Segment));
    }

    #[test]
    fn test_needs_separate_file() {
        assert!(needs_separate_file(&EntryStrategy::Segment));
        assert!(!needs_separate_file(&EntryStrategy::Inline));
        assert!(!needs_separate_file(&EntryStrategy::Hoist));
        assert!(!needs_separate_file(&EntryStrategy::Smart));
    }
}
