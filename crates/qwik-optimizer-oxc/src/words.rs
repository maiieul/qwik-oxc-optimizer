//! String constants used across the crate.
//!
//! Centralized string constants prevent typos and provide a single location
//! to update package names or API lists.

#![allow(unused)]

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
}
