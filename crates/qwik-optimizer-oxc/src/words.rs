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
/// Returns `CtxKind::EventHandler` for:
/// - `event$` (explicit event handler API)
/// - JSX event handler attribute names: `on[A-Z]*$` (e.g., `onClick$`, `onInput$`)
/// - Namespaced JSX event handlers: `document:onClick$`, `window:onFocus$`
///
/// Returns `CtxKind::Function` for everything else:
/// - `$`, `component$`, `useTask$`, `useStyles$`, `useVisibleTask$`, etc.
///
/// This matches the SWC optimizer behavior where `component$`, `useTask$`,
/// and bare `$` all get `Function`, while JSX `onClick$` attributes get
/// `EventHandler`.
pub(crate) fn classify_ctx_kind(callee_name: &str) -> CtxKind {
    // Strip any namespace prefix (e.g., "document:onClick$" -> "onClick$")
    let base_name = if let Some(pos) = callee_name.find(':') {
        &callee_name[pos + 1..]
    } else {
        callee_name
    };

    if base_name == "event$" {
        return CtxKind::EventHandler;
    }

    // Check for on[A-Z]*$ pattern (JSX event handler attributes)
    if base_name.starts_with("on") && base_name.ends_with('$') && base_name.len() > 3 {
        // Verify the third character is uppercase (on[A-Z]...)
        if let Some(ch) = base_name.chars().nth(2) {
            if ch.is_ascii_uppercase() || ch == '-' {
                return CtxKind::EventHandler;
            }
        }
    }

    CtxKind::Function
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
        assert!(matches!(
            classify_ctx_kind("useVisibleTask$"),
            CtxKind::Function
        ));
    }

    #[test]
    fn test_classify_ctx_kind_event_handler() {
        assert!(matches!(classify_ctx_kind("event$"), CtxKind::EventHandler));
        // JSX event handler attributes
        assert!(matches!(
            classify_ctx_kind("onClick$"),
            CtxKind::EventHandler
        ));
        assert!(matches!(
            classify_ctx_kind("onInput$"),
            CtxKind::EventHandler
        ));
        assert!(matches!(
            classify_ctx_kind("onBlur$"),
            CtxKind::EventHandler
        ));
        assert!(matches!(
            classify_ctx_kind("onFocus$"),
            CtxKind::EventHandler
        ));
        assert!(matches!(
            classify_ctx_kind("onMouseover$"),
            CtxKind::EventHandler
        ));
        // Namespaced JSX event handlers
        assert!(matches!(
            classify_ctx_kind("document:onClick$"),
            CtxKind::EventHandler
        ));
        assert!(matches!(
            classify_ctx_kind("window:onFocus$"),
            CtxKind::EventHandler
        ));
        // on-custom$ (hyphenated) should also be event handler
        assert!(matches!(
            classify_ctx_kind("on-anotherCustom$"),
            CtxKind::EventHandler
        ));
        // Not event handlers (no on[A-Z] pattern)
        assert!(matches!(classify_ctx_kind("onl$"), CtxKind::Function));
        assert!(matches!(classify_ctx_kind("$"), CtxKind::Function));
    }

    #[test]
    fn test_is_qwik_core_import() {
        assert!(is_qwik_core_import("@qwik.dev/core"));
        assert!(!is_qwik_core_import("@builder.io/qwik"));
        assert!(!is_qwik_core_import("./utils"));
        assert!(!is_qwik_core_import("react"));
    }
}
