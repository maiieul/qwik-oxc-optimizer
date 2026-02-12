//! Output Audit: Comprehensive comparison of OXC optimizer output
//! against all 162 spec expected outputs, module by module.
//!
//! This is a one-time audit tool, not a permanent regression test.
//! It writes structured deviation data to audit-raw.json for downstream
//! classification in Plan 02.

mod spec_parser;

#[cfg(test)]
mod tests {
    use super::spec_parser;
    use qwik_optimizer_oxc::transform_modules;
    use serde_json::json;
    use std::collections::HashMap;

    // -----------------------------------------------------------------------
    // Normalization helpers
    // -----------------------------------------------------------------------

    /// Normalize code for comparison: collapse whitespace, sort imports.
    /// Per user decision: ignore whitespace, normalize import ordering,
    /// but preserve const/let/var differences.
    fn normalize_code(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut imports: Vec<String> = Vec::new();
        let mut non_imports: Vec<String> = Vec::new();

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if trimmed.starts_with("import ") || trimmed.starts_with("import{") {
                imports.push(trimmed.split_whitespace().collect::<Vec<_>>().join(" "));
            } else {
                non_imports.push(trimmed.split_whitespace().collect::<Vec<_>>().join(" "));
            }
        }

        imports.sort();
        let mut result = imports;
        result.extend(non_imports);
        result.join(" ")
    }

    /// Strip the hash portion from a segment module path.
    /// Pattern: "prefix_HASH.ext" where HASH is a variable-length alphanumeric string.
    /// Examples:
    ///   "test.tsx_renderHeader_zBbHWn4e8Cg.tsx" -> "test.tsx_renderHeader_.tsx"
    ///   "test.tsx_s_8GbMvm1RDPk.ts" -> "test.tsx_s_.ts"
    fn strip_hash_from_path(path: &str) -> String {
        // Find the extension
        if let Some(dot_pos) = path.rfind('.') {
            let ext = &path[dot_pos..];
            let before_ext = &path[..dot_pos];

            // Find the last underscore before the extension - the hash follows it
            if let Some(underscore_pos) = before_ext.rfind('_') {
                let candidate_hash = &before_ext[underscore_pos + 1..];
                // Hash is typically alphanumeric, 5-15 chars
                if !candidate_hash.is_empty()
                    && candidate_hash.len() >= 3
                    && candidate_hash.chars().all(|c| c.is_alphanumeric())
                {
                    return format!("{}{}{}", &before_ext[..underscore_pos + 1], "", ext);
                }
            }
        }
        path.to_string()
    }

    /// Generate a brief diff summary describing what differs between expected and actual.
    fn diff_summary(expected: &str, actual: &str) -> String {
        // Tokenize by whitespace
        let expected_tokens: Vec<&str> = expected.split_whitespace().collect();
        let actual_tokens: Vec<&str> = actual.split_whitespace().collect();

        let mut diffs: Vec<String> = Vec::new();

        // Check for const/let/var differences
        let const_let_var = |tokens: &[&str]| -> HashMap<String, usize> {
            let mut counts = HashMap::new();
            for &t in tokens {
                if t == "const" || t == "let" || t == "var" {
                    *counts.entry(t.to_string()).or_insert(0) += 1;
                }
            }
            counts
        };
        let expected_clv = const_let_var(&expected_tokens);
        let actual_clv = const_let_var(&actual_tokens);
        if expected_clv != actual_clv {
            diffs.push(format!(
                "const/let/var diff: expected {:?}, actual {:?}",
                expected_clv, actual_clv
            ));
        }

        // Check for import differences
        let extract_imports = |s: &str| -> Vec<String> {
            let mut imports = Vec::new();
            // Already normalized, so imports are sorted and whitespace-collapsed
            // Find "import" tokens in the normalized string
            let tokens: Vec<&str> = s.split_whitespace().collect();
            let mut i = 0;
            while i < tokens.len() {
                if tokens[i] == "import" {
                    let start = i;
                    // Scan until we find a semicolon or another import/non-import statement
                    while i < tokens.len() && !tokens[i].ends_with(';') {
                        i += 1;
                    }
                    if i < tokens.len() {
                        i += 1;
                    }
                    let import_str: String = tokens[start..i].join(" ");
                    imports.push(import_str);
                } else {
                    i += 1;
                }
            }
            imports
        };

        let expected_imports = extract_imports(expected);
        let actual_imports = extract_imports(actual);
        if expected_imports != actual_imports {
            // Find missing and extra imports
            let missing: Vec<&String> = expected_imports
                .iter()
                .filter(|i| !actual_imports.contains(i))
                .collect();
            let extra: Vec<&String> = actual_imports
                .iter()
                .filter(|i| !expected_imports.contains(i))
                .collect();
            if !missing.is_empty() {
                diffs.push(format!("missing imports: {}", missing.len()));
            }
            if !extra.is_empty() {
                diffs.push(format!("extra imports: {}", extra.len()));
            }
            if missing.is_empty() && extra.is_empty() {
                diffs.push("import content differs".to_string());
            }
        }

        // Check for QRL wrapper differences
        let qrl_wrappers = ["qrl(", "inlinedQrl(", "_jsxQ(", "_jsxC(", "_jsxS("];
        for wrapper in &qrl_wrappers {
            let expected_count = expected.matches(wrapper).count();
            let actual_count = actual.matches(wrapper).count();
            if expected_count != actual_count {
                diffs.push(format!(
                    "{} count: expected {}, actual {}",
                    wrapper.trim_end_matches('('),
                    expected_count,
                    actual_count
                ));
            }
        }

        // Check for capture-related differences (useLexicalScope)
        let expected_lex = expected.matches("useLexicalScope").count();
        let actual_lex = actual.matches("useLexicalScope").count();
        if expected_lex != actual_lex {
            diffs.push(format!(
                "useLexicalScope: expected {}, actual {}",
                expected_lex, actual_lex
            ));
        }

        // If nothing specific found, do a general token-level diff
        if diffs.is_empty() {
            // Find first divergence point
            let max_check = expected_tokens.len().min(actual_tokens.len()).min(50);
            for i in 0..max_check {
                if expected_tokens[i] != actual_tokens[i] {
                    diffs.push(format!(
                        "token diff at position {}: expected '{}', actual '{}'",
                        i, expected_tokens[i], actual_tokens[i]
                    ));
                    break;
                }
            }
            if diffs.is_empty() {
                if expected_tokens.len() != actual_tokens.len() {
                    diffs.push(format!(
                        "length diff: expected {} tokens, actual {} tokens",
                        expected_tokens.len(),
                        actual_tokens.len()
                    ));
                } else {
                    diffs.push("content differs (no specific pattern detected)".to_string());
                }
            }
        }

        diffs.join("; ")
    }

    // -----------------------------------------------------------------------
    // Runtime-breaking classification helpers
    // -----------------------------------------------------------------------

    /// Extract import symbol names from a normalized code string.
    /// Returns the set of imported identifiers (e.g., {"foo", "bar"} from
    /// "import { foo, bar } from './module';").
    fn extract_import_symbols(normalized: &str) -> std::collections::HashSet<String> {
        let mut symbols = std::collections::HashSet::new();
        let tokens: Vec<&str> = normalized.split_whitespace().collect();
        let mut i = 0;
        while i < tokens.len() {
            if tokens[i] == "import" {
                // Scan tokens until semicolon, collecting identifiers
                let mut j = i + 1;
                let mut past_from = false;
                while j < tokens.len() {
                    let t = tokens[j];
                    if t == "from" {
                        past_from = true;
                    } else if !past_from {
                        // Clean up braces and commas
                        let cleaned = t.replace(['{', '}', ','], "");
                        let cleaned = cleaned.trim();
                        if !cleaned.is_empty()
                            && cleaned != "as"
                            && cleaned != "*"
                            && cleaned != "type"
                            && !cleaned.starts_with('"')
                            && !cleaned.starts_with('\'')
                        {
                            // Handle "X as Y" -- take Y (the local binding)
                            // But since we check if the symbol is *used in body*,
                            // we want both the imported and local names
                            symbols.insert(cleaned.to_string());
                        }
                    }
                    if t.ends_with(';') {
                        break;
                    }
                    j += 1;
                }
                i = j + 1;
            } else {
                i += 1;
            }
        }
        symbols
    }

    /// Extract the body (non-import) portion of normalized code.
    fn extract_body_code(normalized: &str) -> String {
        let tokens: Vec<&str> = normalized.split_whitespace().collect();
        let mut body_parts = Vec::new();
        let mut i = 0;
        let mut in_import = false;
        while i < tokens.len() {
            if tokens[i] == "import" && !in_import {
                in_import = true;
            }
            if in_import {
                if tokens[i].ends_with(';') {
                    in_import = false;
                }
                i += 1;
                continue;
            }
            body_parts.push(tokens[i]);
            i += 1;
        }
        body_parts.join(" ")
    }

    /// Classify a deviation as runtime-breaking.
    ///
    /// A deviation is runtime-breaking if:
    /// 1. match_type == "unmatched_expected" AND the spec has no corresponding
    ///    unmatched_actual module to pair with (truly missing module), OR
    /// 2. The diff_summary mentions "missing imports" AND the missing import
    ///    symbols are actually used in the actual body code.
    fn is_runtime_breaking_code_deviation(deviation: &serde_json::Value) -> bool {
        let summary = deviation
            .get("diff_summary")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if !summary.contains("missing imports") {
            return false;
        }
        let expected_code = deviation
            .get("expected_code_normalized")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let actual_code = deviation
            .get("actual_code_normalized")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let expected_imports = extract_import_symbols(expected_code);
        let actual_imports = extract_import_symbols(actual_code);
        let body = extract_body_code(actual_code);

        // Find symbols in expected imports but not in actual imports
        let missing: Vec<&String> = expected_imports
            .iter()
            .filter(|s| !actual_imports.contains(*s))
            .collect();

        // Check if any missing symbol is referenced in the body
        missing.iter().any(|sym| body.contains(sym.as_str()))
    }

    // -----------------------------------------------------------------------
    // Module matching
    // -----------------------------------------------------------------------

    #[derive(Debug, Clone)]
    struct ModuleMatch {
        expected_path: String,
        actual_path: String,
        match_type: String,
        expected_code: String,
        actual_code: String,
    }

    /// Match expected modules to actual modules by path, handling hash differences.
    fn match_modules(
        expected_modules: &[spec_parser::ExpectedModule],
        actual_modules: &[qwik_optimizer_oxc::TransformModule],
    ) -> (Vec<ModuleMatch>, Vec<String>, Vec<String>) {
        let mut matches = Vec::new();
        let mut unmatched_expected: Vec<String> = Vec::new();
        let mut unmatched_actual: Vec<String> = actual_modules
            .iter()
            .map(|m| m.path.clone())
            .collect::<Vec<_>>();

        for expected in expected_modules {
            // Try exact path match first
            if let Some(pos) = unmatched_actual
                .iter()
                .position(|p| *p == expected.path)
            {
                let actual = actual_modules
                    .iter()
                    .find(|m| m.path == expected.path)
                    .unwrap();
                matches.push(ModuleMatch {
                    expected_path: expected.path.clone(),
                    actual_path: actual.path.clone(),
                    match_type: "exact_path".to_string(),
                    expected_code: expected.code.clone(),
                    actual_code: actual.code.clone(),
                });
                unmatched_actual.remove(pos);
                continue;
            }

            // Try structural match (strip hash)
            let expected_stripped = strip_hash_from_path(&expected.path);
            let mut found = false;
            for (idx, actual_path) in unmatched_actual.iter().enumerate() {
                let actual_stripped = strip_hash_from_path(actual_path);
                if expected_stripped == actual_stripped {
                    let actual = actual_modules
                        .iter()
                        .find(|m| m.path == *actual_path)
                        .unwrap();
                    matches.push(ModuleMatch {
                        expected_path: expected.path.clone(),
                        actual_path: actual.path.clone(),
                        match_type: "structural".to_string(),
                        expected_code: expected.code.clone(),
                        actual_code: actual.code.clone(),
                    });
                    unmatched_actual.remove(idx);
                    found = true;
                    break;
                }
            }

            if !found {
                // Try matching by segment ctx_name/display_name as fallback
                let mut fallback_found = false;
                if let Some(seg_json) = &expected.segment {
                    let expected_display = seg_json
                        .get("displayName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let expected_ctx = seg_json
                        .get("ctxName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    for (idx, actual_path) in unmatched_actual.iter().enumerate() {
                        let actual = actual_modules
                            .iter()
                            .find(|m| m.path == *actual_path)
                            .unwrap();
                        if let Some(ref seg) = actual.segment {
                            // Match by display_name suffix or ctx_name
                            let display_match = !expected_display.is_empty()
                                && (seg.display_name == expected_display
                                    || display_name_suffix_match(
                                        expected_display,
                                        &seg.display_name,
                                    ));
                            let ctx_match = !expected_ctx.is_empty()
                                && seg.ctx_name == expected_ctx
                                && actual_modules
                                    .iter()
                                    .filter(|m| {
                                        m.segment
                                            .as_ref()
                                            .map_or(false, |s| s.ctx_name == expected_ctx)
                                    })
                                    .count()
                                    == 1;

                            if display_match || ctx_match {
                                matches.push(ModuleMatch {
                                    expected_path: expected.path.clone(),
                                    actual_path: actual.path.clone(),
                                    match_type: "structural".to_string(),
                                    expected_code: expected.code.clone(),
                                    actual_code: actual.code.clone(),
                                });
                                unmatched_actual.remove(idx);
                                fallback_found = true;
                                break;
                            }
                        }
                    }
                }

                if !fallback_found {
                    unmatched_expected.push(expected.path.clone());
                }
            }
        }

        (matches, unmatched_expected, unmatched_actual)
    }

    /// Check if two display names match after stripping the file prefix.
    /// E.g., "test.tsx_renderHeader" matches "test.tsx_renderHeader".
    fn display_name_suffix_match(expected: &str, actual: &str) -> bool {
        // Extract the function name portion after the first underscore
        fn extract_fn(name: &str) -> &str {
            if let Some(pos) = name.find('_') {
                &name[pos + 1..]
            } else {
                name
            }
        }
        let e = extract_fn(expected);
        let a = extract_fn(actual);
        !e.is_empty() && !a.is_empty() && e == a
    }

    // -----------------------------------------------------------------------
    // Main audit test
    // -----------------------------------------------------------------------

    #[test]
    fn output_audit() {
        let specs = spec_parser::load_all_specs();
        assert!(
            specs.len() >= 162,
            "Expected at least 162 specs, found {}",
            specs.len()
        );

        let mut all_deviations: Vec<serde_json::Value> = Vec::new();
        let mut all_errors: Vec<serde_json::Value> = Vec::new();

        let mut specs_run = 0u32;
        let mut specs_matched_all = 0u32;
        let mut specs_with_deviations = 0u32;
        let mut specs_errored = 0u32;
        let mut total_module_count_mismatches = 0u32;
        let mut total_code_deviations = 0u32;
        let mut total_unmatched_modules = 0u32;

        for spec in &specs {
            let options = spec_parser::build_options(spec);
            let result = match transform_modules(options) {
                Ok(r) => r,
                Err(e) => {
                    specs_errored += 1;
                    all_errors.push(json!({
                        "spec_name": spec.name,
                        "error": format!("{}", e)
                    }));
                    continue;
                }
            };

            specs_run += 1;

            // Check module count mismatch
            let expected_count = spec.expected_modules.len();
            let actual_count = result.modules.len();

            if expected_count != actual_count {
                total_module_count_mismatches += 1;
                specs_with_deviations += 1;

                // Record as a module count deviation
                all_deviations.push(json!({
                    "spec_name": spec.name,
                    "module_path_expected": format!("(spec expects {} modules)", expected_count),
                    "module_path_actual": format!("(optimizer produced {} modules)", actual_count),
                    "match_type": "module_count_mismatch",
                    "expected_code_normalized": "",
                    "actual_code_normalized": "",
                    "diff_summary": format!(
                        "Module count: expected {}, got {}. Expected paths: {:?}. Actual paths: {:?}",
                        expected_count,
                        actual_count,
                        spec.expected_modules.iter().map(|m| &m.path).collect::<Vec<_>>(),
                        result.modules.iter().map(|m| m.path.as_str()).collect::<Vec<_>>()
                    )
                }));

                // Still try to match whatever modules we can
            }

            // Match modules
            let (matched, unmatched_expected, unmatched_actual) =
                match_modules(&spec.expected_modules, &result.modules);

            let mut spec_has_deviation = expected_count != actual_count;

            // Record unmatched modules
            for path in &unmatched_expected {
                total_unmatched_modules += 1;
                all_deviations.push(json!({
                    "spec_name": spec.name,
                    "module_path_expected": path,
                    "module_path_actual": "",
                    "match_type": "unmatched_expected",
                    "expected_code_normalized": "",
                    "actual_code_normalized": "",
                    "diff_summary": format!("Expected module '{}' has no matching actual module", path)
                }));
                spec_has_deviation = true;
            }

            for path in &unmatched_actual {
                total_unmatched_modules += 1;
                all_deviations.push(json!({
                    "spec_name": spec.name,
                    "module_path_expected": "",
                    "module_path_actual": path,
                    "match_type": "unmatched_actual",
                    "expected_code_normalized": "",
                    "actual_code_normalized": "",
                    "diff_summary": format!("Actual module '{}' has no matching expected module", path)
                }));
                spec_has_deviation = true;
            }

            // Compare code for matched modules
            for m in &matched {
                // Skip modules where expected has no code (description-only modules)
                if m.expected_code.is_empty() {
                    continue;
                }

                let expected_normalized = normalize_code(&m.expected_code);
                let actual_normalized = normalize_code(&m.actual_code);

                if expected_normalized != actual_normalized {
                    total_code_deviations += 1;
                    let summary = diff_summary(&expected_normalized, &actual_normalized);

                    all_deviations.push(json!({
                        "spec_name": spec.name,
                        "module_path_expected": m.expected_path,
                        "module_path_actual": m.actual_path,
                        "match_type": m.match_type,
                        "expected_code_normalized": expected_normalized,
                        "actual_code_normalized": actual_normalized,
                        "diff_summary": summary
                    }));
                    spec_has_deviation = true;
                }
            }

            if spec_has_deviation {
                // Only count once even if there are multiple deviations
                if expected_count == actual_count {
                    specs_with_deviations += 1;
                }
                // (module count mismatch case already counted above)
            } else {
                specs_matched_all += 1;
            }
        }

        // ---------------------------------------------------------------
        // Classify deviations as runtime-breaking vs cosmetic (FIX-03)
        // ---------------------------------------------------------------
        //
        // Runtime-breaking deviations are those that would cause a runtime
        // error if the optimized code were loaded:
        //   1. truly-missing-module: match_type == "unmatched_expected" with
        //      no naming-convention pair in the same spec's unmatched_actual
        //   2. missing-import-used: code deviation where diff_summary has
        //      "missing imports" and the missing symbol is used in the body
        //
        // Everything else is cosmetic (codegen-style, naming-convention,
        // missing-import-cosmetic, module-count-mismatch).

        // Group unmatched modules by spec to identify naming-convention pairs
        let mut ue_by_spec: HashMap<String, Vec<&serde_json::Value>> = HashMap::new();
        let mut ua_by_spec: HashMap<String, usize> = HashMap::new();

        for d in &all_deviations {
            let mt = d.get("match_type").and_then(|v| v.as_str()).unwrap_or("");
            let spec = d
                .get("spec_name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            match mt {
                "unmatched_expected" => {
                    ue_by_spec.entry(spec).or_default().push(d);
                }
                "unmatched_actual" => {
                    *ua_by_spec.entry(spec).or_insert(0) += 1;
                }
                _ => {}
            }
        }

        // Count truly-missing-module (unmatched_expected with no pairing actual)
        let mut truly_missing_count: usize = 0;
        for (spec, ue_list) in &ue_by_spec {
            let ua_count = ua_by_spec.get(spec).copied().unwrap_or(0);
            if ue_list.len() > ua_count {
                truly_missing_count += ue_list.len() - ua_count;
            }
            // If ua_count >= ue_list.len(), all are naming-convention pairs
        }

        // Count missing-import-used (code deviations where missing symbol is in body)
        let mut missing_import_used_count: usize = 0;
        for d in &all_deviations {
            let mt = d.get("match_type").and_then(|v| v.as_str()).unwrap_or("");
            if mt == "exact_path" || mt == "structural" {
                if is_runtime_breaking_code_deviation(d) {
                    missing_import_used_count += 1;
                }
            }
        }

        let runtime_breaking_count = truly_missing_count + missing_import_used_count;

        // Build final JSON output
        let audit_result = json!({
            "total_specs": specs.len(),
            "specs_run": specs_run,
            "specs_matched_all_modules": specs_matched_all,
            "specs_with_deviations": specs_with_deviations,
            "specs_errored": specs_errored,
            "total_deviations": all_deviations.len(),
            "deviations": all_deviations,
            "errors": all_errors,
            "summary": {
                "module_count_mismatches": total_module_count_mismatches,
                "code_deviations": total_code_deviations,
                "unmatched_modules": total_unmatched_modules
            }
        });

        // Write JSON to file
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let output_path = manifest_dir
            .join("..")
            .join("..")
            .join(".planning")
            .join("phases")
            .join("23-output-audit")
            .join("audit-raw.json");

        let json_str = serde_json::to_string_pretty(&audit_result)
            .expect("Failed to serialize audit results");
        std::fs::write(&output_path, &json_str).unwrap_or_else(|e| {
            panic!(
                "Failed to write audit-raw.json to {}: {}",
                output_path.display(),
                e
            )
        });

        // Print summary to stderr
        eprintln!("\n=== Output Audit Results ===");
        eprintln!("Total specs: {}", specs.len());
        eprintln!("Specs run (no transform error): {}", specs_run);
        eprintln!("Specs errored: {}", specs_errored);
        eprintln!("Specs matched all modules: {}", specs_matched_all);
        eprintln!("Specs with deviations: {}", specs_with_deviations);
        eprintln!("---");
        eprintln!("Module count mismatches: {}", total_module_count_mismatches);
        eprintln!("Code deviations: {}", total_code_deviations);
        eprintln!("Unmatched modules: {}", total_unmatched_modules);
        eprintln!("Total deviation records: {}", all_deviations.len());
        eprintln!("---");
        eprintln!(
            "Runtime-breaking deviations: {} (threshold: {})",
            runtime_breaking_count, RUNTIME_BREAKING_THRESHOLD
        );
        eprintln!(
            "  truly-missing-module: {}",
            truly_missing_count
        );
        eprintln!(
            "  missing-import-used: {}",
            missing_import_used_count
        );
        eprintln!("---");
        eprintln!("Output written to: {}", output_path.display());
        eprintln!("============================\n");

        assert!(
            output_path.exists(),
            "audit-raw.json should have been written"
        );

        // Regression gate (FIX-03): runtime-breaking deviations must not exceed threshold.
        // After Phase 24 Plans 01-10, 4 runtime-breaking deviations remain:
        //   - 0 missing-import-used (Plan 10 fixed example_drop_side_effects api import)
        //   - 4 truly-missing-module (example_qwik_react: 2, relative_paths: 2)
        // These are accepted architectural limitations. If this assertion fails,
        // a code change has reintroduced runtime-breaking deviations.
        const RUNTIME_BREAKING_THRESHOLD: usize = 4;
        assert!(
            runtime_breaking_count <= RUNTIME_BREAKING_THRESHOLD,
            "Regression detected: {} runtime-breaking deviations exceeds threshold of {}. \
             Previously fixed deviations may have been reintroduced. \
             Check audit-raw.json for details. \
             (truly-missing-module: {}, missing-import-used: {})",
            runtime_breaking_count,
            RUNTIME_BREAKING_THRESHOLD,
            truly_missing_count,
            missing_import_used_count
        );
    }
}
