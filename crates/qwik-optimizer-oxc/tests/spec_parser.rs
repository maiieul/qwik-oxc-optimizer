//! Markdown spec file parser for the qwik-optimizer-oxc test harness.
//!
//! Parses 162 behavioral spec markdown files into structured data,
//! extracting input code, expected output modules, segment metadata,
//! diagnostics, and configuration overrides.

#![allow(dead_code)]

use qwik_optimizer_oxc::{
    EmitMode, EntryStrategy, MinifyMode, TransformModuleInput, TransformModulesOptions,
};
use std::fs;
use std::path::{Path, PathBuf};

/// A parsed spec file containing all test expectations.
pub struct SpecFile {
    /// Test name extracted from `# Test: {name}`.
    pub name: String,
    /// Configuration overrides as key-value pairs (order preserved).
    pub config_overrides: Vec<(String, String)>,
    /// Primary input source code.
    pub input_code: String,
    /// Input filename (default "test.tsx").
    pub input_filename: String,
    /// Expected output modules.
    pub expected_modules: Vec<ExpectedModule>,
    /// Expected diagnostics as raw JSON values.
    pub expected_diagnostics: Vec<serde_json::Value>,
    /// Additional input modules for multi-input tests (e.g., relative_paths).
    pub additional_inputs: Vec<(String, String)>,
}

/// An expected output module from the spec file.
pub struct ExpectedModule {
    /// Module path (e.g., "test.tsx", "test.tsx_renderHeader_zBbHWn4e8Cg.tsx").
    pub path: String,
    /// Whether this module is an entry point.
    pub is_entry: bool,
    /// Expected output code.
    pub code: String,
    /// Segment metadata (JSON), present for entry point modules.
    pub segment: Option<serde_json::Value>,
}

/// Returns the path to the spec directory from the workspace root.
pub fn spec_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crate is at crates/qwik-optimizer-oxc/, workspace root is ../../
    manifest_dir
        .join("..")
        .join("..")
        .join(".planning")
        .join("spec")
}

/// Parse a single spec markdown file into a SpecFile.
pub fn parse_spec_file(path: &Path) -> Result<SpecFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return Err(format!("Empty spec file: {}", path.display()));
    }

    // 1. Extract test name from `# Test: {name}`
    let name = extract_test_name(&lines)?;

    // 2. Parse configuration table
    let config_overrides = extract_config_overrides(&lines);

    // 3. Extract input source code
    let (input_code, input_filename, additional_inputs) = extract_input_code(&lines, &name)?;

    // 4. Extract output modules
    let expected_modules = extract_output_modules(&lines)?;

    // 5. Extract diagnostics
    let expected_diagnostics = extract_diagnostics(&lines);

    Ok(SpecFile {
        name,
        config_overrides,
        input_code,
        input_filename,
        expected_modules,
        expected_diagnostics,
        additional_inputs,
    })
}

/// Build a `TransformModulesOptions` from a parsed spec file.
pub fn build_options(spec: &SpecFile) -> TransformModulesOptions {
    let mut opts = TransformModulesOptions::default();

    // Check if config overrides include a Filename override
    let filename_override = spec
        .config_overrides
        .iter()
        .find(|(k, _)| k.trim().to_lowercase() == "filename")
        .map(|(_, v)| {
            v.replace("(default)", "")
                .replace("(non-default)", "")
                .trim()
                .to_string()
        });

    let input_filename = filename_override.unwrap_or_else(|| spec.input_filename.clone());

    // Set default input
    let mut inputs = Vec::new();

    // Add additional inputs first (e.g., dependency modules)
    for (path, code) in &spec.additional_inputs {
        inputs.push(TransformModuleInput {
            code: code.clone(),
            path: path.clone(),
        });
    }

    // Add primary input
    if !spec.input_code.is_empty() {
        inputs.push(TransformModuleInput {
            code: spec.input_code.clone(),
            path: input_filename,
        });
    }

    opts.input = inputs;

    // Apply config overrides
    for (key, value) in &spec.config_overrides {
        apply_config_override(&mut opts, key, value);
    }

    opts
}

/// Load all spec files from the spec directory.
pub fn load_all_specs() -> Vec<SpecFile> {
    let dir = spec_dir();
    let mut specs = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("Failed to read spec directory {}: {}", dir.display(), e))
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map_or(false, |ext| ext == "md")
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        match parse_spec_file(&path) {
            Ok(spec) => specs.push(spec),
            Err(e) => eprintln!("WARNING: Failed to parse {}: {}", path.display(), e),
        }
    }

    specs
}

// ---------------------------------------------------------------------------
// Internal parsing helpers
// ---------------------------------------------------------------------------

fn extract_test_name(lines: &[&str]) -> Result<String, String> {
    for line in lines {
        if let Some(name) = line.strip_prefix("# Test: ") {
            return Ok(name.trim().to_string());
        }
    }
    Err("No '# Test: {name}' heading found".to_string())
}

fn extract_config_overrides(lines: &[&str]) -> Vec<(String, String)> {
    let mut overrides = Vec::new();
    let mut in_config_section = false;
    let mut in_table = false;

    for line in lines {
        let trimmed = line.trim();

        if trimmed == "## Test Configuration" {
            in_config_section = true;
            continue;
        }

        // End of config section on next ## heading
        if in_config_section && trimmed.starts_with("## ") && trimmed != "## Test Configuration" {
            break;
        }

        if !in_config_section {
            continue;
        }

        // Skip "*(all defaults)*" lines
        if trimmed.contains("*(all defaults)*") {
            continue;
        }

        // Detect table header row
        if trimmed.starts_with("| Option") {
            in_table = true;
            continue;
        }

        // Skip separator row
        if in_table && trimmed.starts_with("|---") {
            continue;
        }

        // Parse table data rows
        if in_table && trimmed.starts_with('|') {
            let parts: Vec<&str> = trimmed.split('|').collect();
            // parts[0] is empty (before first |), parts[1] is key, parts[2] is value
            if parts.len() >= 3 {
                let key = parts[1].trim().to_string();
                let value = parts[2].trim().to_string();
                if !key.is_empty() && !key.starts_with("---") {
                    overrides.push((key, value));
                }
            }
        }

        // End table when we hit a non-table, non-empty line
        if in_table && !trimmed.is_empty() && !trimmed.starts_with('|') {
            // Could be a note or other text, keep scanning
            in_table = false;
        }
    }

    overrides
}

fn extract_input_code(
    lines: &[&str],
    name: &str,
) -> Result<(String, String, Vec<(String, String)>), String> {
    let mut input_code = String::new();
    let mut input_filename = "test.tsx".to_string();
    let mut additional_inputs: Vec<(String, String)> = Vec::new();

    // Check for relative_paths special case: "### Source Code: {label}"
    let is_multi_input = name == "relative_paths";

    if is_multi_input {
        // Parse multiple labeled source code sections
        let mut i = 0;
        let mut found_any = false;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            if trimmed.starts_with("### Source Code:") {
                found_any = true;
                let label = trimmed
                    .strip_prefix("### Source Code:")
                    .unwrap()
                    .trim()
                    .to_string();

                // Find the code block
                i += 1;
                let code = extract_next_code_block(lines, &mut i);

                // Determine if this is the primary input or additional
                // The "main component" is the primary; everything else is additional
                if label.contains("main") {
                    input_code = code;
                    // Extract filename from the heading text
                    // e.g., "main.tsx (main component)" -> "components/main.tsx"
                    // We use the config for actual path; default to test.tsx
                    input_filename = "components/main.tsx".to_string();
                } else {
                    // Additional input (dependency module)
                    // Extract a reasonable path from the label
                    let dep_path = if label.contains("lib.mjs") {
                        "../../node_modules/dep/dist/lib.mjs".to_string()
                    } else {
                        label.clone()
                    };
                    additional_inputs.push((dep_path, code));
                }
            } else {
                i += 1;
            }
        }

        if !found_any {
            return Err("relative_paths: no Source Code sections found".to_string());
        }
    } else {
        // Standard single source code section
        let mut i = 0;
        while i < lines.len() {
            let trimmed = lines[i].trim();

            if trimmed == "### Source Code" {
                i += 1;
                input_code = extract_next_code_block(lines, &mut i);
                break;
            }
            i += 1;
        }
    }

    Ok((input_code, input_filename, additional_inputs))
}

/// Extract the next code block starting from position i.
/// Advances i past the closing triple-backtick line.
fn extract_next_code_block(lines: &[&str], i: &mut usize) -> String {
    // Find the opening ```
    while *i < lines.len() {
        let trimmed = lines[*i].trim();
        if trimmed.starts_with("```") && trimmed.len() > 3 {
            // Opening fence with language tag
            *i += 1;
            break;
        } else if trimmed == "```" {
            // Opening fence without language (unlikely for source but handle it)
            *i += 1;
            break;
        }
        *i += 1;
    }

    // Collect lines until closing ```
    let mut code_lines = Vec::new();
    while *i < lines.len() {
        let trimmed = lines[*i].trim();
        if trimmed == "```" {
            *i += 1;
            break;
        }
        code_lines.push(lines[*i]);
        *i += 1;
    }

    code_lines.join("\n")
}

fn extract_output_modules(lines: &[&str]) -> Result<Vec<ExpectedModule>, String> {
    let mut modules = Vec::new();
    let mut i = 0;

    // Find the ## Output section
    while i < lines.len() {
        if lines[i].trim() == "## Output" {
            i += 1;
            break;
        }
        i += 1;
    }

    // Parse ### Module: sections
    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Stop at the next ## section (like ## Conventions Applied, ## Diagnostics, etc.)
        if trimmed.starts_with("## ")
            && !trimmed.starts_with("### ")
        {
            break;
        }

        if trimmed.starts_with("### Module:") {
            let (module, new_i) = parse_module_section(lines, i)?;
            modules.push(module);
            i = new_i;
        } else {
            i += 1;
        }
    }

    Ok(modules)
}

fn parse_module_section(
    lines: &[&str],
    start: usize,
) -> Result<(ExpectedModule, usize), String> {
    let header = lines[start].trim();

    // Parse "### Module: {path}" or "### Module: {path} (ENTRY POINT)"
    let after_prefix = header
        .strip_prefix("### Module:")
        .unwrap()
        .trim();

    let (path, is_entry) = if after_prefix.ends_with("(ENTRY POINT)") {
        let p = after_prefix
            .strip_suffix("(ENTRY POINT)")
            .unwrap()
            .trim();
        (p.to_string(), true)
    } else {
        (after_prefix.to_string(), false)
    };

    let mut i = start + 1;

    // Check if there's a code block before the next section boundary.
    // Some modules have only descriptive text (e.g., "*(Component body...)*")
    // instead of a code block.
    let code = extract_module_code_block(lines, &mut i);

    // Scan for <details> blocks and #### Segment Metadata
    let mut segment = None;
    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Stop if we hit another ### Module: or a ## section (not ### or ####)
        if trimmed.starts_with("### Module:")
            || (trimmed.starts_with("## ")
                && !trimmed.starts_with("### ")
                && !trimmed.starts_with("#### "))
        {
            break;
        }

        // Skip <details> blocks (AST JSON we don't need)
        if trimmed == "<details>" {
            while i < lines.len() {
                if lines[i].trim() == "</details>" {
                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Handle "#### Segment Metadata" with optional JSON code block
        if trimmed.starts_with("#### Segment Metadata") {
            if trimmed == "#### Segment Metadata" {
                // Standard format: JSON code block follows
                i += 1;
                let json_str = extract_next_code_block(lines, &mut i);
                if !json_str.trim().is_empty() {
                    match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(val) => segment = Some(val),
                        Err(e) => {
                            return Err(format!(
                                "Failed to parse segment metadata JSON for module '{}': {}",
                                path, e
                            ));
                        }
                    }
                }
            } else {
                // Inline format: "#### Segment Metadata (captures: true, ...)"
                // Extract the parenthesized description as a simple JSON object
                let inline_desc = trimmed
                    .strip_prefix("#### Segment Metadata")
                    .unwrap()
                    .trim();
                segment = parse_inline_segment_metadata(inline_desc);
                i += 1;
            }
            continue;
        }

        i += 1;
    }

    Ok((
        ExpectedModule {
            path,
            is_entry,
            code,
            segment,
        },
        i,
    ))
}

/// Extract a code block for a module, handling the case where no code block exists.
/// Returns empty string if no code block is found before the next section boundary.
fn extract_module_code_block(lines: &[&str], i: &mut usize) -> String {
    // Look ahead to see if there's a code block before the next ### or ## heading
    let saved_i = *i;
    while *i < lines.len() {
        let trimmed = lines[*i].trim();

        // If we hit another section boundary before finding a code block, no code
        if trimmed.starts_with("### Module:")
            || (trimmed.starts_with("## ")
                && !trimmed.starts_with("### ")
                && !trimmed.starts_with("#### "))
        {
            *i = saved_i;
            // Advance past any non-section content
            while *i < lines.len() {
                let t = lines[*i].trim();
                if t.starts_with("### Module:")
                    || (t.starts_with("## ") && !t.starts_with("### ") && !t.starts_with("#### "))
                    || t == "<details>"
                    || t.starts_with("#### Segment Metadata")
                {
                    break;
                }
                *i += 1;
            }
            return String::new();
        }

        // Found a code block opening
        if trimmed.starts_with("```") {
            *i = saved_i;
            return extract_next_code_block(lines, i);
        }

        *i += 1;
    }

    *i = saved_i;
    String::new()
}

/// Parse inline segment metadata like "(captures: true, captureNames: [\"cart\"])"
/// into a JSON value. Returns None if parsing fails.
fn parse_inline_segment_metadata(desc: &str) -> Option<serde_json::Value> {
    // Strip parentheses if present
    let inner = desc
        .trim()
        .strip_prefix('(')
        .and_then(|s| s.strip_suffix(')'))
        .unwrap_or(desc.trim());

    if inner.is_empty() {
        return None;
    }

    // Build a simple JSON object from "key: value, key: value" format
    let mut map = serde_json::Map::new();
    for part in split_inline_metadata(inner) {
        let part = part.trim();
        if let Some(colon_pos) = part.find(':') {
            let key = part[..colon_pos].trim();
            let val_str = part[colon_pos + 1..].trim();

            let val = if val_str == "true" {
                serde_json::Value::Bool(true)
            } else if val_str == "false" {
                serde_json::Value::Bool(false)
            } else if let Ok(v) = serde_json::from_str::<serde_json::Value>(val_str) {
                v
            } else {
                serde_json::Value::String(val_str.to_string())
            };

            map.insert(key.to_string(), val);
        }
    }

    if map.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(map))
    }
}

/// Split inline metadata by commas, but respect brackets (for arrays like ["cart"]).
fn split_inline_metadata(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut bracket_depth = 0;

    for ch in s.chars() {
        match ch {
            '[' => {
                bracket_depth += 1;
                current.push(ch);
            }
            ']' => {
                bracket_depth -= 1;
                current.push(ch);
            }
            ',' if bracket_depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
}

fn extract_diagnostics(lines: &[&str]) -> Vec<serde_json::Value> {
    let mut i = 0;

    // Find ## Diagnostics section
    while i < lines.len() {
        if lines[i].trim() == "## Diagnostics" {
            i += 1;
            break;
        }
        i += 1;
    }

    if i >= lines.len() {
        return Vec::new();
    }

    // Extract the JSON code block
    let json_str = extract_next_code_block(lines, &mut i);

    if json_str.trim().is_empty() {
        return Vec::new();
    }

    match serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
        Ok(diagnostics) => diagnostics,
        Err(e) => {
            eprintln!("WARNING: Failed to parse diagnostics JSON: {}", e);
            Vec::new()
        }
    }
}

fn apply_config_override(opts: &mut TransformModulesOptions, key: &str, value: &str) {
    // Strip "(default)" and "(non-default)" annotations from values
    let clean_value = value
        .replace("(default)", "")
        .replace("(non-default)", "")
        .trim()
        .to_string();

    // Normalize key to handle case variations (e.g., "Transpile Ts" vs "Transpile TS")
    let normalized_key = key.trim().to_lowercase();

    match normalized_key.as_str() {
        "entry strategy" => {
            opts.entry_strategy = match clean_value.as_str() {
                "Inline" => EntryStrategy::Inline,
                "Segment" => EntryStrategy::Segment,
                "Hoist" => EntryStrategy::Hoist,
                "Single" => EntryStrategy::Single,
                "Component" => EntryStrategy::Component,
                "Smart" => EntryStrategy::Smart,
                "Hook" => EntryStrategy::Hook,
                _ => EntryStrategy::Segment, // default
            };
        }
        "mode" => {
            opts.mode = match clean_value.as_str() {
                "Dev" => EmitMode::Dev,
                "Prod" => EmitMode::Prod,
                "Lib" => EmitMode::Lib,
                "Test" => EmitMode::Lib, // Test mode maps to Lib (default)
                _ => EmitMode::Lib,
            };
        }
        "minify" => {
            opts.minify = match clean_value.as_str() {
                "Simplify" => MinifyMode::Simplify,
                "None" => MinifyMode::None,
                _ => MinifyMode::Simplify,
            };
        }
        "transpile ts" => {
            opts.transpile_ts = parse_bool(&clean_value);
        }
        "transpile jsx" => {
            opts.transpile_jsx = parse_bool(&clean_value);
        }
        "explicit extensions" => {
            opts.explicit_extensions = parse_bool(&clean_value);
        }
        "preserve filenames" => {
            opts.preserve_filenames = parse_bool(&clean_value);
        }
        "source maps" => {
            opts.source_maps = parse_bool(&clean_value);
        }
        "src dir" => {
            opts.src_dir = clean_value;
        }
        "root dir" => {
            opts.root_dir = Some(clean_value);
        }
        "scope" => {
            opts.scope = Some(clean_value);
        }
        "core module" => {
            opts.core_module = Some(clean_value);
        }
        "is server" => {
            // Handle "Some(true)", "Some(false)", "true", "false"
            let v = clean_value
                .replace("Some(", "")
                .replace(')', "")
                .trim()
                .to_string();
            opts.is_server = Some(parse_bool(&v));
        }
        "strip exports" => {
            // Parse JSON array like ["onGet"]
            if let Ok(arr) = serde_json::from_str::<Vec<String>>(&clean_value) {
                opts.strip_exports = Some(arr);
            }
        }
        "strip ctx name" => {
            // Parse JSON array like ["server"]
            if let Ok(arr) = serde_json::from_str::<Vec<String>>(&clean_value) {
                opts.strip_ctx_name = Some(arr);
            }
        }
        "strip event handlers" => {
            opts.strip_event_handlers = parse_bool(&clean_value);
        }
        "reg ctx name" => {
            // Parse JSON array like ["server"]
            if let Ok(arr) = serde_json::from_str::<Vec<String>>(&clean_value) {
                opts.reg_ctx_name = Some(arr);
            }
        }
        // Additional config keys found in some spec files
        "filename" => {
            // Override the input filename
            // This is applied to opts via the input, not a direct field.
            // We store it; build_options will check config for Filename.
            // Actually, we handle this in the input: the spec's input_filename
            // should be overridden. We'll handle this specially.
        }
        "dev path" => {
            // Development path override -- not mapped to TransformModulesOptions
            // (used internally by the SWC test harness, not a public option)
        }
        _ => {
            // Truly unknown config key
        }
    }
}

fn parse_bool(s: &str) -> bool {
    matches!(s.to_lowercase().as_str(), "true" | "yes" | "1")
}
