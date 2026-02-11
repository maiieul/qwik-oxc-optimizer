//! String constants and dollar API helpers.
//!
//! Centralized string constants prevent typos and provide a single location
//! to update package names or API lists. Also provides helpers for classifying
//! dollar API call sites by context kind.

use crate::types::CtxKind;

/// The core Qwik module path.
pub(crate) const BUILDER_IO_QWIK: &str = "@qwik.dev/core";

/// Alternate import path for the Qwik core module.
pub(crate) const QWIK_CORE_ID: &str = "@qwik.dev/core";

/// List of known $-suffixed Qwik APIs.
pub(crate) const KNOWN_DOLLAR_APIS: &[&str] = &[
    "$",
    "component$",
    "useTask$",
    "useVisibleTask$",
    "useBrowserVisibleTask$",
    "useStyles$",
    "useStylesScoped$",
    "useOnDocument$",
    "useOnWindow$",
    "useOn$",
    "event$",
    "eventQrl",
];

/// Check if a name is a known $-suffixed Qwik API.
pub(crate) fn is_known_dollar_api(name: &str) -> bool {
    KNOWN_DOLLAR_APIS.contains(&name)
}

/// Convert a $-suffixed name to its Qrl-suffixed equivalent.
///
/// Example: "component$" -> "componentQrl"
pub(crate) fn dollar_to_qrl_name(name: &str) -> String {
    name.strip_suffix('$')
        .map(|s| format!("{s}Qrl"))
        .unwrap_or_else(|| name.to_string())
}

/// Classify the context kind of a dollar call site.
///
/// Most dollar calls produce `CtxKind::Function`. Only explicit event handler
/// patterns like `event$` produce `CtxKind::EventHandler`. This matches the
/// SWC optimizer behavior where `component$`, `useTask$`, `useStyles$`, and
/// bare `$` all get `Function`, while JSX `onClick$` attributes get `EventHandler`
/// during the transform pass (not the collector pass).
pub(crate) fn classify_ctx_kind(callee_name: &str) -> CtxKind {
    if callee_name == "event$" {
        CtxKind::EventHandler
    } else {
        CtxKind::Function
    }
}

/// Check if an import source is the Qwik core module.
///
/// Returns true for `"@qwik.dev/core"`. The collector can also pass in
/// a custom `core_module` override to check against.
pub(crate) fn is_qwik_core_import(source: &str) -> bool {
    source == BUILDER_IO_QWIK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_known_dollar_api() {
        assert!(is_known_dollar_api("$"));
        assert!(is_known_dollar_api("component$"));
        assert!(is_known_dollar_api("useTask$"));
        assert!(is_known_dollar_api("useVisibleTask$"));
        assert!(!is_known_dollar_api("unknownFunc$"));
        assert!(!is_known_dollar_api("component"));
    }

    #[test]
    fn test_dollar_to_qrl_name() {
        assert_eq!(dollar_to_qrl_name("component$"), "componentQrl");
        assert_eq!(dollar_to_qrl_name("$"), "Qrl");
        assert_eq!(dollar_to_qrl_name("useTask$"), "useTaskQrl");
        assert_eq!(dollar_to_qrl_name("noDollar"), "noDollar");
    }

    #[test]
    fn test_classify_ctx_kind_function() {
        assert!(matches!(classify_ctx_kind("$"), CtxKind::Function));
        assert!(matches!(classify_ctx_kind("component$"), CtxKind::Function));
        assert!(matches!(classify_ctx_kind("useTask$"), CtxKind::Function));
        assert!(matches!(classify_ctx_kind("useStyles$"), CtxKind::Function));
        assert!(matches!(classify_ctx_kind("useVisibleTask$"), CtxKind::Function));
    }

    #[test]
    fn test_classify_ctx_kind_event_handler() {
        assert!(matches!(classify_ctx_kind("event$"), CtxKind::EventHandler));
    }

    #[test]
    fn test_is_qwik_core_import() {
        assert!(is_qwik_core_import("@qwik.dev/core"));
        assert!(!is_qwik_core_import("@builder.io/qwik"));
        assert!(!is_qwik_core_import("./utils"));
        assert!(!is_qwik_core_import("react"));
    }
}
