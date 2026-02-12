mod spec_parser;

#[cfg(test)]
mod tests {
    use super::spec_parser;
    use qwik_optimizer_oxc::transform_modules;
    use std::path::PathBuf;

    /// Extract all spec input code to individual .tsx files in tests/input/.
    ///
    /// Run manually once:
    ///   cargo test -p qwik-optimizer-oxc extract_inputs -- --ignored
    #[test]
    #[ignore]
    fn extract_inputs() {
        let specs = spec_parser::load_all_specs();
        let input_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("input");
        std::fs::create_dir_all(&input_dir).unwrap();

        let mut count = 0;
        for spec in &specs {
            if spec.input_code.is_empty() {
                continue;
            }
            let filename = format!("{}.tsx", spec.name);
            std::fs::write(input_dir.join(&filename), &spec.input_code).unwrap();
            count += 1;
        }

        eprintln!("Extracted {} input files to tests/input/", count);
    }

    /// Snapshot test: run every spec through the optimizer and snapshot the output.
    ///
    /// Each spec gets a named snapshot showing all output modules and diagnostics.
    /// Snapshots live at tests/snapshots/{spec_name}.snap
    ///
    /// First run:  cargo insta test -p qwik-optimizer-oxc -- snapshot_all_transforms
    /// Review:     cargo insta review
    /// Accept all: cargo insta test -p qwik-optimizer-oxc --accept -- snapshot_all_transforms
    #[test]
    fn snapshot_all_transforms() {
        let specs = spec_parser::load_all_specs();

        for spec in &specs {
            let mut options = spec_parser::build_options(spec);
            options.source_maps = false;

            let output = match transform_modules(options) {
                Ok(result) => format_transform_output(&result),
                Err(e) => format!("TRANSFORM ERROR: {}", e),
            };

            insta::with_settings!({prepend_module_to_snapshot => false}, {
                insta::assert_snapshot!(spec.name.clone(), output);
            });
        }
    }

    fn format_transform_output(result: &qwik_optimizer_oxc::TransformOutput) -> String {
        let mut s = String::new();

        for (i, module) in result.modules.iter().enumerate() {
            if i > 0 {
                s.push('\n');
            }
            let entry_marker = if module.is_entry { " (ENTRY)" } else { "" };
            s.push_str(&format!("=== {} ==={}\n", module.path, entry_marker));
            s.push_str(&module.code);
            if !module.code.ends_with('\n') {
                s.push('\n');
            }
        }

        if !result.diagnostics.is_empty() {
            s.push_str("\n=== DIAGNOSTICS ===\n");
            for diag in &result.diagnostics {
                s.push_str(&format!("[{:?}] {}\n", diag.category, diag.message));
            }
        }

        s
    }
}
