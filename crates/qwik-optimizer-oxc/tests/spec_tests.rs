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
}
