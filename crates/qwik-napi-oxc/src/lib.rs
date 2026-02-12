//! NAPI binding for the Qwik OXC optimizer.
//!
//! Exposes `transform_modules` to Node.js via napi-rs v2, with wire format
//! adaptation so `platform.ts` can call it without changes.
//!
//! The wire format adapter handles the key differences between the JS calling
//! convention (from `convertOptions()` in optimizer.ts) and the internal OXC types:
//! - `entryStrategy` arrives as a plain string ("smart", "segment", etc.)
//! - `manualChunks` arrives as an optional object (ignored by OXC)

use std::collections::HashMap;

use napi_derive::napi;
use serde::Deserialize;

use qwik_optimizer_oxc::{
    EmitMode, EntryStrategy, MinifyMode, TransformModuleInput, TransformModulesOptions,
};

// ---------------------------------------------------------------------------
// Wire format input types (matches JS convertOptions() output)
// ---------------------------------------------------------------------------

/// NAPI-specific input struct matching the JS wire format from `convertOptions()`.
///
/// Key differences from the internal OXC `TransformModulesOptions`:
/// - `entry_strategy` is a plain string, not a tagged enum
/// - `manual_chunks` is a separate optional field (ignored)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NapiTransformModulesOptions {
    /// Root directory for resolving relative paths.
    src_dir: String,

    /// Optional root directory override.
    #[serde(default)]
    root_dir: Option<String>,

    /// List of input modules to transform.
    input: Vec<NapiTransformModuleInput>,

    /// Whether to generate source maps.
    #[serde(default)]
    source_maps: bool,

    /// Minification mode as string ("simplify" or "none").
    #[serde(default = "default_minify")]
    minify: String,

    /// Whether to strip TypeScript type annotations.
    #[serde(default)]
    transpile_ts: bool,

    /// Whether to transpile JSX to function calls.
    #[serde(default)]
    transpile_jsx: bool,

    /// Whether to preserve original filenames in output paths.
    #[serde(default)]
    preserve_filenames: bool,

    /// Entry strategy as plain string ("smart", "segment", "inline", etc.).
    #[serde(default = "default_entry_strategy")]
    entry_strategy: String,

    /// Manual chunks mapping (ignored by OXC optimizer).
    #[serde(default)]
    #[allow(dead_code)]
    manual_chunks: Option<HashMap<String, String>>,

    /// Whether to use explicit file extensions in import paths.
    #[serde(default)]
    explicit_extensions: bool,

    /// Output mode ("lib", "prod", "dev").
    #[serde(default = "default_mode")]
    mode: String,

    /// Optional scope prefix for segment names.
    #[serde(default)]
    scope: Option<String>,

    /// Override the core module import path.
    #[serde(default)]
    core_module: Option<String>,

    /// List of export names to strip from output.
    #[serde(default)]
    strip_exports: Option<Vec<String>>,

    /// List of ctx names to strip.
    #[serde(default)]
    strip_ctx_name: Option<Vec<String>>,

    /// Whether to strip event handler registrations.
    #[serde(default)]
    strip_event_handlers: bool,

    /// List of ctx names to register.
    #[serde(default)]
    reg_ctx_name: Option<Vec<String>>,

    /// Whether this build targets SSR.
    #[serde(default)]
    is_server: Option<bool>,
}

/// A single input file (matches JS wire format).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NapiTransformModuleInput {
    /// The source code content.
    code: String,
    /// The file path (relative to src_dir).
    path: String,
}

fn default_minify() -> String {
    "simplify".to_string()
}

fn default_entry_strategy() -> String {
    "smart".to_string()
}

fn default_mode() -> String {
    "lib".to_string()
}

// ---------------------------------------------------------------------------
// Wire format conversion
// ---------------------------------------------------------------------------

/// Convert from the NAPI wire format to the internal OXC `TransformModulesOptions`.
fn from_napi_options(napi_opts: NapiTransformModulesOptions) -> TransformModulesOptions {
    let entry_strategy = match napi_opts.entry_strategy.as_str() {
        "inline" => EntryStrategy::Inline,
        "hoist" => EntryStrategy::Hoist,
        "single" => EntryStrategy::Single,
        "hook" => EntryStrategy::Hook,
        "segment" => EntryStrategy::Segment,
        "component" => EntryStrategy::Component,
        "smart" => EntryStrategy::Smart,
        _ => EntryStrategy::Segment, // default fallback
    };

    let minify = match napi_opts.minify.as_str() {
        "none" => MinifyMode::None,
        _ => MinifyMode::Simplify,
    };

    let mode = match napi_opts.mode.as_str() {
        "prod" => EmitMode::Prod,
        "dev" => EmitMode::Dev,
        _ => EmitMode::Lib,
    };

    TransformModulesOptions {
        src_dir: napi_opts.src_dir,
        root_dir: napi_opts.root_dir,
        input: napi_opts
            .input
            .into_iter()
            .map(|i| TransformModuleInput {
                code: i.code,
                path: i.path,
            })
            .collect(),
        source_maps: napi_opts.source_maps,
        minify,
        transpile_ts: napi_opts.transpile_ts,
        transpile_jsx: napi_opts.transpile_jsx,
        preserve_filenames: napi_opts.preserve_filenames,
        entry_strategy,
        explicit_extensions: napi_opts.explicit_extensions,
        mode,
        scope: napi_opts.scope,
        core_module: napi_opts.core_module,
        strip_exports: napi_opts.strip_exports,
        strip_ctx_name: napi_opts.strip_ctx_name,
        strip_event_handlers: napi_opts.strip_event_handlers,
        reg_ctx_name: napi_opts.reg_ctx_name,
        is_server: napi_opts.is_server,
    }
}

// ---------------------------------------------------------------------------
// NAPI export
// ---------------------------------------------------------------------------

/// Transform one or more Qwik modules.
///
/// Accepts a JSON object matching the wire format from `convertOptions()` in
/// optimizer.ts and returns a JSON object matching `TransformOutput`.
///
/// This is synchronous in Rust -- platform.ts wraps the return value as a
/// resolved Promise at the TS layer.
#[napi(js_name = "transform_modules")]
pub fn transform_modules(opts: serde_json::Value) -> napi::Result<serde_json::Value> {
    // Deserialize from JS wire format
    let napi_opts: NapiTransformModulesOptions = serde_json::from_value(opts)
        .map_err(|e| napi::Error::from_reason(format!("Invalid options: {e}")))?;

    // Convert to internal format
    let internal_opts = from_napi_options(napi_opts);

    // Call the optimizer
    let output = qwik_optimizer_oxc::transform_modules(internal_opts)
        .map_err(|e| napi::Error::from_reason(format!("Transform error: {e}")))?;

    // Serialize to JS wire format (camelCase via serde rename_all)
    serde_json::to_value(&output)
        .map_err(|e| napi::Error::from_reason(format!("Serialization error: {e}")))
}
