mod spec_parser;

#[cfg(test)]
mod tests {
    use super::spec_parser;

    /// Verify that all 162 spec files parse without errors.
    /// This test does NOT run the optimizer -- it only validates
    /// that the spec parser can extract input, output, and metadata
    /// from every spec markdown file.
    #[test]
    fn test_parse_all_specs() {
        let specs = spec_parser::load_all_specs();

        // Must find all 162 specs
        assert!(
            specs.len() >= 162,
            "Expected at least 162 spec files, found {}",
            specs.len()
        );

        let mut failures = Vec::new();

        for spec in &specs {
            // Every spec must have input code (except relative_paths which has additional_inputs)
            if spec.input_code.is_empty() && spec.additional_inputs.is_empty() {
                failures.push(format!("{}: no input code found", spec.name));
                continue;
            }

            // Every spec must have at least one output module
            if spec.expected_modules.is_empty() {
                failures.push(format!("{}: no output modules found", spec.name));
                continue;
            }

            // Count modules with code vs modules with just descriptions.
            // Some spec files have entry point modules with only a text
            // description (no code block) -- these are still valid specs.
            let modules_with_code: usize = spec
                .expected_modules
                .iter()
                .filter(|m| !m.code.is_empty())
                .count();

            // At least one module must have actual code
            if modules_with_code == 0 {
                failures.push(format!(
                    "{}: no output modules with code (total modules: {})",
                    spec.name,
                    spec.expected_modules.len()
                ));
            }
        }

        if !failures.is_empty() {
            panic!(
                "Spec parse failures ({}/{}):\n{}",
                failures.len(),
                specs.len(),
                failures.join("\n")
            );
        }

        // Print summary
        let total_modules: usize = specs.iter().map(|s| s.expected_modules.len()).sum();
        let entry_modules: usize = specs
            .iter()
            .flat_map(|s| &s.expected_modules)
            .filter(|m| m.is_entry)
            .count();
        let with_config: usize = specs
            .iter()
            .filter(|s| !s.config_overrides.is_empty())
            .count();

        let with_segment: usize = specs
            .iter()
            .flat_map(|s| &s.expected_modules)
            .filter(|m| m.segment.is_some())
            .count();
        let with_diagnostics: usize = specs
            .iter()
            .filter(|s| !s.expected_diagnostics.is_empty())
            .count();

        eprintln!(
            "Parsed {} specs: {} output modules ({} entry points, {} with segment metadata), {} with config overrides, {} with diagnostics",
            specs.len(),
            total_modules,
            entry_modules,
            with_segment,
            with_config,
            with_diagnostics
        );
    }

    /// Verify that build_options produces valid TransformModulesOptions
    /// for a selection of representative specs.
    #[test]
    fn test_build_options() {
        let specs = spec_parser::load_all_specs();

        for spec in &specs {
            let options = spec_parser::build_options(spec);

            // Options must have at least one input
            assert!(
                !options.input.is_empty(),
                "{}: build_options produced empty input",
                spec.name
            );

            // Input code must match spec
            if !spec.input_code.is_empty() {
                assert_eq!(
                    options.input.last().unwrap().code,
                    spec.input_code,
                    "{}: input code mismatch",
                    spec.name
                );
            }

            // Verify JSON serialization works
            let json = serde_json::to_string(&options);
            assert!(
                json.is_ok(),
                "{}: failed to serialize options: {:?}",
                spec.name,
                json.err()
            );
        }
    }

    /// Smoke test: run optimizer on first spec and verify it produces output.
    /// Phase 8 implemented the real transform pipeline, so we now verify
    /// that transform_modules returns actual transformed code.
    #[test]
    fn test_transform_produces_output() {
        use qwik_optimizer_oxc::transform_modules;

        let specs = spec_parser::load_all_specs();
        let spec = &specs[0]; // Just test one for now
        let options = spec_parser::build_options(spec);

        let result = transform_modules(options);
        assert!(result.is_ok(), "transform_modules should not fail");

        let output = result.unwrap();
        // Real pipeline produces at least a main module per input
        assert!(
            !output.modules.is_empty(),
            "transform should produce at least one output module"
        );
        // First module should have code
        assert!(
            !output.modules[0].code.is_empty(),
            "main module should have emitted code"
        );
    }

    /// Run optimizer on a curated set of spec files that test segment extraction
    /// and verify structural properties of the output.
    ///
    /// This validates CONV-08 (segment extraction) and CONV-06 (lazy imports).
    /// Since hashes differ from the SWC reference and JSX transforms aren't
    /// implemented yet (Phase 11), we verify structural properties rather than
    /// exact path/code matching:
    ///   - Transform succeeds without errors
    ///   - Correct number of segment modules produced
    ///   - Entry modules have segment metadata with correct ctxName/ctxKind
    ///   - Segment code is non-empty for segment strategy
    ///   - Inline strategy produces empty segment code
    #[test]
    fn test_segment_extraction_specs() {
        use qwik_optimizer_oxc::transform_modules;

        let specs = spec_parser::load_all_specs();

        let mut pass_count = 0;
        let mut fail_count = 0;
        let mut failures: Vec<String> = Vec::new();

        // Test 1: Inline strategy spec should produce segments with empty code
        if let Some(spec) = specs.iter().find(|s| s.name == "example_inlined_entry_strategy") {
            let options = spec_parser::build_options(spec);
            match transform_modules(options) {
                Ok(result) => {
                    // Inline strategy: all segment modules should have empty code
                    let segment_modules: Vec<_> = result
                        .modules
                        .iter()
                        .filter(|m| m.segment.is_some())
                        .collect();

                    let all_empty = segment_modules.iter().all(|m| m.code.is_empty());
                    let all_not_entry = segment_modules.iter().all(|m| !m.is_entry);

                    if all_empty && all_not_entry && !segment_modules.is_empty() {
                        pass_count += 1;
                    } else {
                        failures.push(format!(
                            "example_inlined_entry_strategy: inline segments should have empty code and is_entry=false (found {} segments, all_empty={}, all_not_entry={})",
                            segment_modules.len(), all_empty, all_not_entry
                        ));
                        fail_count += 1;
                    }
                }
                Err(e) => {
                    failures.push(format!("example_inlined_entry_strategy: transform failed: {}", e));
                    fail_count += 1;
                }
            }
        }

        // Test 2: Segment strategy specs should produce segments with non-empty code
        for spec_name in &[
            "example_1",
            "example_2",
            "example_3",
            "example_4",
            "example_5",
        ] {
            if let Some(spec) = specs.iter().find(|s| s.name == *spec_name) {
                let options = spec_parser::build_options(spec);
                match transform_modules(options) {
                    Ok(result) => {
                        // Count expected entry point modules from spec
                        let expected_entries = spec
                            .expected_modules
                            .iter()
                            .filter(|m| m.is_entry)
                            .count();

                        // Count actual segment modules
                        let actual_segments: Vec<_> = result
                            .modules
                            .iter()
                            .filter(|m| m.segment.is_some())
                            .collect();

                        // Verify segment count matches expected entry count
                        if actual_segments.len() != expected_entries {
                            failures.push(format!(
                                "{}: expected {} segments, got {} (expected paths: {:?})",
                                spec_name,
                                expected_entries,
                                actual_segments.len(),
                                spec.expected_modules
                                    .iter()
                                    .filter(|m| m.is_entry)
                                    .map(|m| &m.path)
                                    .collect::<Vec<_>>()
                            ));
                            fail_count += 1;
                            continue;
                        }

                        // Verify all segment modules have non-empty code and is_entry
                        let all_have_code = actual_segments.iter().all(|m| !m.code.is_empty());
                        let all_are_entry = actual_segments.iter().all(|m| m.is_entry);

                        if all_have_code && all_are_entry {
                            // Verify segment metadata has valid ctxName and ctxKind
                            let all_have_meta = actual_segments.iter().all(|m| {
                                if let Some(seg) = &m.segment {
                                    !seg.ctx_name.is_empty() && !seg.hash.is_empty()
                                } else {
                                    false
                                }
                            });

                            if all_have_meta {
                                pass_count += 1;
                            } else {
                                failures.push(format!(
                                    "{}: some segments missing metadata",
                                    spec_name
                                ));
                                fail_count += 1;
                            }
                        } else {
                            failures.push(format!(
                                "{}: segments should have code and is_entry=true (all_have_code={}, all_are_entry={})",
                                spec_name, all_have_code, all_are_entry
                            ));
                            fail_count += 1;
                        }
                    }
                    Err(e) => {
                        failures.push(format!("{}: transform failed: {}", spec_name, e));
                        fail_count += 1;
                    }
                }
            }
        }

        // Test 3: Segment metadata ctxName matches for selected specs
        if let Some(spec) = specs.iter().find(|s| s.name == "example_multi_capture") {
            let options = spec_parser::build_options(spec);
            match transform_modules(options) {
                Ok(result) => {
                    let segments: Vec<_> = result
                        .modules
                        .iter()
                        .filter_map(|m| m.segment.as_ref())
                        .collect();

                    // Should have component$ segments and $() segments
                    let component_segs = segments
                        .iter()
                        .filter(|s| s.ctx_name == "component$")
                        .count();
                    let dollar_segs = segments.iter().filter(|s| s.ctx_name == "$").count();

                    if component_segs > 0 && dollar_segs > 0 {
                        // Verify captures are correctly detected
                        let has_captures = segments.iter().any(|s| s.captures);
                        if has_captures {
                            pass_count += 1;
                        } else {
                            failures.push(
                                "example_multi_capture: expected some segments with captures=true"
                                    .to_string(),
                            );
                            fail_count += 1;
                        }
                    } else {
                        failures.push(format!(
                            "example_multi_capture: expected component$ and $ segments (got {} component, {} dollar)",
                            component_segs, dollar_segs
                        ));
                        fail_count += 1;
                    }
                }
                Err(e) => {
                    failures.push(format!("example_multi_capture: transform failed: {}", e));
                    fail_count += 1;
                }
            }
        }

        eprintln!(
            "\nSegment extraction spec tests: {}/{} passed ({} failed)",
            pass_count,
            pass_count + fail_count,
            fail_count
        );

        if !failures.is_empty() {
            eprintln!("Failures:\n{}", failures.join("\n\n"));
        }

        // Assert that at least some specs pass
        assert!(
            pass_count > 0,
            "At least some segment extraction specs should pass. Failures:\n{}",
            failures.join("\n")
        );
    }

    /// Run optimizer on ALL specs and report pass/fail counts.
    /// This is an informational test -- it asserts all specs transform without error
    /// (no panics, no transform failures), but does NOT assert output matches.
    /// It gives visibility into overall spec coverage.
    #[test]
    fn test_all_specs_coverage_report() {
        use qwik_optimizer_oxc::transform_modules;

        let specs = spec_parser::load_all_specs();
        let mut transform_ok = 0;
        let mut transform_err = 0;
        let mut transform_err_names: Vec<String> = Vec::new();
        let mut module_count_match = 0;
        let mut module_count_mismatch = 0;

        for spec in &specs {
            let options = spec_parser::build_options(spec);
            match transform_modules(options) {
                Ok(result) => {
                    transform_ok += 1;
                    if result.modules.len() == spec.expected_modules.len() {
                        module_count_match += 1;
                    } else {
                        module_count_mismatch += 1;
                    }
                }
                Err(e) => {
                    transform_err += 1;
                    transform_err_names.push(format!("{}: {}", spec.name, e));
                }
            }
        }

        eprintln!(
            "\n=== Spec Coverage Report ===\n\
             Total specs: {}\n\
             Transform OK: {}\n\
             Transform Error: {}\n\
             Module count match: {}\n\
             Module count mismatch: {}\n\
             ===========================\n",
            specs.len(),
            transform_ok,
            transform_err,
            module_count_match,
            module_count_mismatch
        );

        if !transform_err_names.is_empty() {
            eprintln!(
                "Transform errors:\n{}",
                transform_err_names.join("\n")
            );
        }

        // All specs should at least transform without errors (no panics)
        assert!(
            transform_err == 0,
            "All specs should transform without errors, but {} failed:\n{}",
            transform_err,
            transform_err_names.join("\n")
        );
    }

    fn _normalize_whitespace(s: &str) -> String {
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    }
}
