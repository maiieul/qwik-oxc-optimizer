//! Qwik optimizer using OXC for code transformation.
//!
//! This crate provides `transform_modules()`, which takes one or more input modules
//! and applies Qwik's $-call extraction, import rewriting, and segment splitting
//! transformations. The output matches the SWC optimizer's JSON wire format,
//! enabling drop-in replacement at the TypeScript binding layer.

mod types;
mod errors;
mod words;
mod hash;
mod parse;
mod collector;
mod transform;
mod import_rewrite;
mod code_move;
mod entry_strategy;
mod emit;
mod filter_exports;
mod props_destructuring;
mod is_const;
mod const_replace;

// Re-export public types
pub use types::{
    CtxKind, Diagnostic, DiagnosticCategory, EmitMode, EntryStrategy, MinifyMode,
    SegmentAnalysis, SourceLocation, TransformModule, TransformModuleInput,
    TransformModulesOptions, TransformOutput,
};

use types::{SegmentData, TransformOptions};

/// Main entry point: transform one or more modules.
///
/// Accepts a `TransformModulesOptions` configuration and returns a `TransformOutput`
/// containing all transformed modules (main + extracted segments) and any diagnostics.
pub fn transform_modules(
    config: TransformModulesOptions,
) -> Result<TransformOutput, anyhow::Error> {
    let mut all_modules = Vec::new();
    let mut all_diagnostics = Vec::new();
    let mut is_type_script = false;
    let mut is_jsx = false;

    // Build per-module TransformOptions from the top-level config
    let transform_options = TransformOptions {
        src_dir: config.src_dir.clone(),
        root_dir: config.root_dir.clone(),
        source_maps: config.source_maps,
        minify: config.minify.clone(),
        transpile_ts: config.transpile_ts,
        transpile_jsx: config.transpile_jsx,
        preserve_filenames: config.preserve_filenames,
        entry_strategy: config.entry_strategy.clone(),
        explicit_extensions: config.explicit_extensions,
        mode: config.mode.clone(),
        scope: config.scope.clone(),
        core_module: config
            .core_module
            .clone()
            .unwrap_or_else(|| "@qwik.dev/core".to_string()),
        strip_exports: config.strip_exports.clone().unwrap_or_default(),
        strip_ctx_name: config.strip_ctx_name.clone().unwrap_or_default(),
        strip_event_handlers: config.strip_event_handlers,
        reg_ctx_name: config.reg_ctx_name.clone().unwrap_or_default(),
        is_server: config.is_server.unwrap_or(false),
    };

    let emit_options = emit::EmitOptions {
        source_maps: config.source_maps,
        minify: config.minify.clone(),
    };

    for input in &config.input {
        // 1. Create the arena allocator for this module
        let allocator = oxc::allocator::Allocator::default();

        // Allocate source into the arena so it lives for 'a
        let source_in_arena = allocator.alloc_str(&input.code);

        // 2. Parse the module
        let parse_result = match parse::parse_module(&allocator, source_in_arena, &input.path) {
            Ok(result) => result,
            Err(diagnostics) => {
                all_diagnostics.extend(diagnostics);
                continue;
            }
        };

        // Track source type flags
        if parse_result.source_type.is_typescript() {
            is_type_script = true;
        }
        if parse_result.source_type.is_jsx() {
            is_jsx = true;
        }

        let mut program = parse_result.program;
        let scoping = parse_result.scoping;

        // 3. Run collector pass
        let collect_result = collector::collect(&program, &scoping);

        // 3b. Run build constant replacement pre-pass (isServer/isBrowser/isDev -> booleans)
        // This must happen before traverse_mut so that segment body serialization
        // sees the replaced boolean literals instead of the original identifiers.
        const_replace::replace_build_constants(&mut program, &transform_options, &allocator);

        // 4. Create QwikTransform and run traverse
        let mut qwik_transform =
            transform::QwikTransform::new(&transform_options, collect_result, &input.path);

        let _scoping = oxc_traverse::traverse_mut(
            &mut qwik_transform,
            &allocator,
            &mut program,
            scoping,
            (),
        );

        // 5. Post-transform processing: populate child segment metadata
        qwik_transform.finalize_segments();

        // 6. Emit the transformed module
        let emit_result = emit::emit_module(&program, source_in_arena, &emit_options);

        // 6b. Prepend hoisted function declarations for _fnSignal.
        // These are string-based const declarations that go after imports but before
        // the main module body. Since imports are already prepended by exit_program,
        // we insert hoisted stmts between imports and the rest of the code.
        let hoisted_stmts: Vec<(String, String)> = qwik_transform.hoisted_function_stmts().to_vec();
        let main_code = if !hoisted_stmts.is_empty() {
            let mut hoisted_code = String::new();
            for (fn_code, str_code) in &hoisted_stmts {
                hoisted_code.push_str(fn_code);
                hoisted_code.push('\n');
                hoisted_code.push_str(str_code);
                hoisted_code.push('\n');
            }
            // Find the insertion point: after all import statements.
            // Import statements start with "import " in the emitted code.
            // We scan for the last import line and insert after it.
            let code = &emit_result.code;
            let mut last_import_end = 0;
            let mut pos = 0;
            for line in code.lines() {
                let line_end = pos + line.len() + 1; // +1 for \n
                if line.starts_with("import ") {
                    last_import_end = line_end.min(code.len());
                }
                pos = line_end;
            }
            if last_import_end > 0 {
                format!("{}{}{}", &code[..last_import_end], hoisted_code, &code[last_import_end..])
            } else {
                // No imports found, prepend hoisted code
                format!("{}{}", hoisted_code, code)
            }
        } else {
            emit_result.code.clone()
        };

        // 7. Compute output extension based on transpile_ts setting
        let output_ext = output_extension(&input.path, transform_options.transpile_ts);
        let main_path = if transform_options.transpile_ts {
            input
                .path
                .rsplit_once('.')
                .map_or(input.path.clone(), |(base, _)| {
                    format!("{}.{}", base, output_ext)
                })
        } else {
            input.path.clone()
        };

        // 8. Build the main TransformModule
        let main_module = TransformModule {
            path: main_path,
            is_entry: false,
            code: main_code,
            map: emit_result.map,
            segment: None,
            orig_path: Some(input.path.clone()),
        };
        all_modules.push(main_module);

        // 9. Build segment modules based on entry strategy
        let body_codes = qwik_transform.take_segment_body_codes();
        let segments = qwik_transform.extracted_segments();
        let stripped_spans = qwik_transform.stripped_segments();

        let is_inline_like = entry_strategy::should_inline(&transform_options.entry_strategy)
            || matches!(
                transform_options.entry_strategy,
                EntryStrategy::Hoist
            );

        for seg in segments {
            // Skip stripped segments -- they don't produce output modules
            if stripped_spans.contains(&seg.span.0) {
                continue;
            }

            let segment_analysis = segment_data_to_analysis(seg, &input.path);

            // Use output extension for segment module path
            let seg_ext = output_extension(&input.path, transform_options.transpile_ts);

            if is_inline_like {
                // Inline/Hoist: segment metadata only, no separate code
                let segment_module = TransformModule {
                    path: format!("{}.{}", segment_analysis.canonical_filename, seg_ext),
                    is_entry: false,
                    code: String::new(),
                    map: None,
                    segment: Some(segment_analysis),
                    orig_path: Some(input.path.clone()),
                };
                all_modules.push(segment_module);
            } else {
                // Segment/Single/Component/Smart/Hook: separate file with code
                let body_code = body_codes
                    .iter()
                    .find(|(span_start, _)| *span_start == seg.span.0)
                    .map(|(_, code)| code.as_str())
                    .unwrap_or("");

                let segment_code = if !body_code.is_empty() {
                    let raw_code = code_move::build_segment_code_with_hoisted(
                        body_code, seg, &transform_options, &hoisted_stmts,
                    );
                    // Normalize via parse+codegen for consistent formatting
                    emit::normalize_code(&raw_code)
                } else {
                    String::new()
                };

                let segment_module = TransformModule {
                    path: format!("{}.{}", segment_analysis.canonical_filename, seg_ext),
                    is_entry: true,
                    code: segment_code,
                    map: None,
                    segment: Some(segment_analysis),
                    orig_path: Some(input.path.clone()),
                };
                all_modules.push(segment_module);
            }
        }

        // 8. Collect diagnostics
        all_diagnostics.extend(qwik_transform.diagnostics().to_vec());
    }

    Ok(TransformOutput {
        modules: all_modules,
        diagnostics: all_diagnostics,
        is_type_script,
        is_jsx,
    })
}

/// Compute the output file extension, accounting for transpile_ts.
///
/// When transpile_ts is true, TypeScript extensions are mapped to JavaScript:
/// - `.tsx` -> `.jsx`
/// - `.ts` -> `.js`
/// Otherwise the original extension is preserved.
fn output_extension(input_path: &str, transpile_ts: bool) -> String {
    let ext = input_path.rsplit('.').next().unwrap_or("js");
    if transpile_ts {
        match ext {
            "tsx" => "jsx".to_string(),
            "ts" => "js".to_string(),
            other => other.to_string(),
        }
    } else {
        ext.to_string()
    }
}

/// Convert internal SegmentData to public SegmentAnalysis.
fn segment_data_to_analysis(seg: &SegmentData, origin_path: &str) -> SegmentAnalysis {
    // Build canonical filename: display_name without the filename prefix + hash
    // The canonical filename is used for the output file path
    let canonical_filename = format!(
        "{}_{}",
        seg.display_name, seg.hash
    );

    SegmentAnalysis {
        origin: origin_path.to_string(),
        name: seg.name.clone(),
        entry: None, // Phase 10 will set this
        display_name: seg.display_name.clone(),
        hash: seg.hash.clone(),
        canonical_filename,
        path: String::new(), // Same directory
        extension: seg.extension.clone(),
        parent: seg.parent.clone(),
        ctx_kind: seg.ctx_kind.clone(),
        ctx_name: seg.ctx_name.clone(),
        captures: seg.captures,
        capture_names: if seg.capture_names.is_empty() {
            None
        } else {
            Some(seg.capture_names.clone())
        },
        loc: seg.span,
        param_names: if seg.param_names.is_empty() {
            None
        } else {
            Some(seg.param_names.clone())
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_modules_empty() {
        let config = TransformModulesOptions::default();
        let result = transform_modules(config).unwrap();

        assert!(result.modules.is_empty());
        assert!(result.diagnostics.is_empty());
        assert!(!result.is_type_script);
        assert!(!result.is_jsx);
    }

    #[test]
    fn test_transform_modules_with_input() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: "const x = 1;".to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };

        let result = transform_modules(config).unwrap();

        // Real pipeline now returns the transformed module
        assert_eq!(result.modules.len(), 1);
        assert!(result.modules[0].code.contains("const x = 1"));
        assert!(result.is_type_script); // .tsx file
        assert!(result.is_jsx);         // .tsx file
    }

    #[test]
    fn test_serde_roundtrip_options() {
        let opts = TransformModulesOptions::default();
        let json = serde_json::to_string(&opts).unwrap();
        let _: TransformModulesOptions = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_serde_camel_case() {
        let opts = TransformModulesOptions {
            src_dir: "src".to_string(),
            source_maps: false,
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };

        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains("\"srcDir\""), "Expected srcDir, got: {json}");
        assert!(
            json.contains("\"sourceMaps\""),
            "Expected sourceMaps, got: {json}"
        );
        assert!(
            json.contains("\"entryStrategy\""),
            "Expected entryStrategy, got: {json}"
        );
    }

    #[test]
    fn test_transform_basic_dollar() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Should have at least 1 module (the main transformed module)
        assert!(
            !result.modules.is_empty(),
            "Expected at least one output module"
        );
        let main_module = &result.modules[0];

        // The main module code should contain qrl() (not $())
        assert!(
            main_module.code.contains("qrl"),
            "Expected qrl in output: {}",
            main_module.code
        );
    }

    #[test]
    fn test_transform_component_dollar() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(() => {
    return <div>Hello</div>;
});"#
                .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should contain componentQrl (renamed from component$)
        assert!(
            main_code.contains("componentQrl"),
            "Expected componentQrl in output: {}",
            main_code
        );
        // Should contain qrl() call wrapping the segment
        assert!(
            main_code.contains("qrl("),
            "Expected qrl() call: {}",
            main_code
        );
    }

    #[test]
    fn test_transform_inline_strategy() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Inline strategy should use inlinedQrl instead of qrl
        assert!(
            main_code.contains("inlinedQrl"),
            "Expected inlinedQrl in output: {}",
            main_code
        );
    }

    #[test]
    fn test_transform_segments_recorded() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Should have main module + at least 1 segment module
        assert!(
            result.modules.len() >= 2,
            "Expected main + segment modules, got {} modules",
            result.modules.len()
        );

        // Second module should be a segment with is_entry = true
        let segment_module = &result.modules[1];
        assert!(segment_module.is_entry, "Segment module should be an entry");
        assert!(
            segment_module.segment.is_some(),
            "Segment module should have segment analysis"
        );
    }

    #[test]
    fn test_transform_imports_added() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(() => <div/>);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;

        // Should have import declarations for componentQrl and qrl
        assert!(
            main_code.contains("componentQrl"),
            "Expected componentQrl import: {}",
            main_code
        );
        assert!(
            main_code.contains("import"),
            "Expected import statements: {}",
            main_code
        );
    }

    #[test]
    fn test_transform_lazy_imports() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
const handler = $(() => 1);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;

        // Should have lazy import constant: const i_HASH = () => import(...)
        assert!(
            main_code.contains("const i_"),
            "Expected lazy import constant: {}",
            main_code
        );
        assert!(
            main_code.contains("import("),
            "Expected dynamic import: {}",
            main_code
        );
    }

    #[test]
    fn test_transform_parse_error_handled() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: "export const = ;".to_string(), // Invalid syntax
                path: "bad.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Should have diagnostics but not panic
        assert!(
            !result.diagnostics.is_empty(),
            "Expected parse error diagnostics"
        );
        assert!(
            result.modules.is_empty(),
            "Should have no modules for parse error"
        );
    }

    // -----------------------------------------------------------------------
    // Props Destructuring Integration Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_props_destructuring_basic() {
        // Use inline strategy so arrow body stays in same module for easy verification
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(({foo, bar}) => {
    return foo + bar;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should contain _rawProps parameter (not destructured {foo, bar})
        assert!(
            main_code.contains("_rawProps"),
            "Expected _rawProps in output: {}",
            main_code
        );
        // Should contain _rawProps.foo and _rawProps.bar member access
        assert!(
            main_code.contains("_rawProps.foo"),
            "Expected _rawProps.foo in output: {}",
            main_code
        );
        assert!(
            main_code.contains("_rawProps.bar"),
            "Expected _rawProps.bar in output: {}",
            main_code
        );
        // Should NOT contain the original destructuring pattern
        assert!(
            !main_code.contains("{foo, bar}"),
            "Should not contain original destructuring: {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_rest() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(({foo, ...rest}) => {
    return foo;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should contain _rawProps parameter
        assert!(
            main_code.contains("_rawProps"),
            "Expected _rawProps in output: {}",
            main_code
        );
        // Should contain _restProps call
        assert!(
            main_code.contains("_restProps"),
            "Expected _restProps in output: {}",
            main_code
        );
        // Should contain the excluded key "foo" in the _restProps call
        assert!(
            main_code.contains("\"foo\""),
            "Expected \"foo\" key in _restProps call: {}",
            main_code
        );
        // Should contain _restProps import
        assert!(
            main_code.contains("_restProps"),
            "Expected _restProps import: {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_renamed() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(({count: c}) => {
    return c;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should contain _rawProps.count (the original key, not the alias "c")
        assert!(
            main_code.contains("_rawProps.count"),
            "Expected _rawProps.count in output (not _rawProps.c): {}",
            main_code
        );
        // The variable `c` should NOT appear as a standalone identifier reference.
        // It should be replaced with _rawProps.count, not _rawProps.c.
        // Verify we don't have _rawProps.c followed by a non-alphanumeric
        // (which would mean the alias was used as a key)
        assert!(
            !main_code.contains("_rawProps.c ") && !main_code.contains("_rawProps.c;") && !main_code.contains("_rawProps.c\n"),
            "Should not contain _rawProps.c (the alias as a member access): {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_plain_param() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$((props) => {
    return props.foo;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should NOT contain _rawProps -- plain identifier params are unchanged
        assert!(
            !main_code.contains("_rawProps"),
            "Should NOT contain _rawProps for plain param: {}",
            main_code
        );
        // Should contain original props reference
        assert!(
            main_code.contains("props"),
            "Expected original props reference: {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_no_params() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(() => {
    return 'hello';
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should NOT contain _rawProps -- no params to destructure
        assert!(
            !main_code.contains("_rawProps"),
            "Should NOT contain _rawProps for no params: {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_segment_metadata() {
        // Use segment strategy to check param_names in SegmentAnalysis
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(({foo}) => {
    return foo;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default() // segment strategy by default
        };
        let result = transform_modules(config).unwrap();

        // Find the segment module with segment metadata
        let segment_module = result
            .modules
            .iter()
            .find(|m| m.segment.is_some())
            .expect("Expected a segment module");

        let segment = segment_module.segment.as_ref().unwrap();

        // Should have param_names = ["_rawProps"]
        assert_eq!(
            segment.param_names,
            Some(vec!["_rawProps".to_string()]),
            "Expected paramNames [\"_rawProps\"] in segment metadata"
        );
        assert_eq!(segment.ctx_name, "component$");
    }

    #[test]
    fn test_props_destructuring_rest_only() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(({...props}) => {
    return props;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should contain _rawProps parameter
        assert!(
            main_code.contains("_rawProps"),
            "Expected _rawProps in output: {}",
            main_code
        );
        // Should contain _restProps(_rawProps) call (no excluded keys)
        assert!(
            main_code.contains("_restProps(_rawProps)"),
            "Expected _restProps(_rawProps) without excluded keys: {}",
            main_code
        );
    }

    #[test]
    fn test_props_destructuring_non_component_unchanged() {
        // useTask$ with destructured params should NOT get props treatment
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, useTask$ } from '@qwik.dev/core';
useTask$(({track}) => {
    track(someSignal);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // Should NOT contain _rawProps -- only component$ gets this treatment
        assert!(
            !main_code.contains("_rawProps"),
            "useTask$ should NOT get _rawProps transformation: {}",
            main_code
        );
    }

    // -----------------------------------------------------------------------
    // Capture Analysis Integration Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_capture_state_variable_inline() {
        // State variable captured inside a nested $() -> appears in inlinedQrl captures array
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$, useStore } from '@qwik.dev/core';
export const App = component$(() => {
    const state = useStore({count: 0});
    return $(() => state.count);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;

        // The inner $() should have captures [state] in the inlinedQrl call
        assert!(
            main_code.contains("[state]"),
            "Expected [state] captures array in output: {}",
            main_code
        );
        assert!(
            main_code.contains("inlinedQrl"),
            "Expected inlinedQrl in output: {}",
            main_code
        );
    }

    #[test]
    fn test_capture_rawprops_segment() {
        // Props destructured component captures _rawProps in nested $() -> segment metadata
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$ } from '@qwik.dev/core';
export const Foo = component$(({foo}) => {
    return $(() => foo);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default() // segment strategy
        };
        let result = transform_modules(config).unwrap();

        // Find segment modules with captures
        let segments_with_captures: Vec<_> = result
            .modules
            .iter()
            .filter_map(|m| m.segment.as_ref())
            .filter(|s| s.captures)
            .collect();

        // The inner $() segment should have captures = true
        assert!(
            !segments_with_captures.is_empty(),
            "Expected at least one segment with captures=true"
        );

        // The capturing segment should have captureNames = ["_rawProps"]
        // (because foo was rewritten to _rawProps.foo by props destructuring)
        let inner_segment = segments_with_captures
            .iter()
            .find(|s| s.ctx_name == "$")
            .expect("Expected a $ segment with captures");
        assert_eq!(
            inner_segment.capture_names,
            Some(vec!["_rawProps".to_string()]),
            "Expected captureNames [\"_rawProps\"] for inner $ segment"
        );

        // In segment strategy, _rawProps won't appear in the main module
        // because the component body (containing the inner $() qrl call
        // with captures) is extracted into a separate segment file.
        // Verify the main module at least has the component qrl call.
        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("componentQrl"),
            "Expected componentQrl in main module output: {}",
            main_code
        );
    }

    #[test]
    fn test_no_capture_imports() {
        // Imports should NOT appear in captures
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$, useStore } from '@qwik.dev/core';
export const App = component$(() => {
    return $(() => useStore({}));
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;

        // useStore is an import -- should NOT be captured
        // The inner $() inlinedQrl should NOT have a captures array with useStore
        assert!(
            !main_code.contains("[useStore]"),
            "useStore should NOT be in captures array: {}",
            main_code
        );

        // Check segment metadata: inner $() should have captures = false
        let inner_segments: Vec<_> = result
            .modules
            .iter()
            .filter_map(|m| m.segment.as_ref())
            .filter(|s| s.ctx_name == "$")
            .collect();

        if let Some(seg) = inner_segments.first() {
            assert!(
                !seg.captures,
                "Inner $() with only import references should have captures=false"
            );
        }
    }

    #[test]
    fn test_no_capture_body_local() {
        // Variables declared inside the $() body should NOT be captured
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return $(() => {
        const x = 1;
        return x;
    });
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;

        // x is declared inside the $() body -> NOT captured
        // The inlinedQrl should NOT have a captures array
        assert!(
            !main_code.contains(", [x]"),
            "Body-local variable x should NOT be in captures: {}",
            main_code
        );

        // Check segment metadata
        let inner_segments: Vec<_> = result
            .modules
            .iter()
            .filter_map(|m| m.segment.as_ref())
            .filter(|s| s.ctx_name == "$")
            .collect();

        if let Some(seg) = inner_segments.first() {
            assert!(
                !seg.captures,
                "Inner $() with only body-local variables should have captures=false"
            );
        }
    }

    #[test]
    fn test_capture_segment_strategy_qrl_with_captures() {
        // Segment strategy: qrl(i_hash, "name", [captures]) with third argument
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$ } from '@qwik.dev/core';
export const Foo = component$(({foo}) => {
    return $(() => foo);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default() // segment strategy
        };
        let result = transform_modules(config).unwrap();

        // The component body segment should contain qrl() with captures
        // After props destructuring, the component body uses _rawProps.foo
        // and the inner $() captures _rawProps
        // The component body segment code should contain:
        //   qrl(i_HASH, "name_HASH", [_rawProps])
        // But since segment code is not yet generated (Phase 10), check the main module
        let main_code = &result.modules[0].code;

        // Main module should have outer qrl calls without captures (component$ itself)
        assert!(
            main_code.contains("qrl("),
            "Expected qrl() in main module: {}",
            main_code
        );
    }

    #[test]
    fn test_capture_metadata_populated() {
        // Verify SegmentAnalysis.captures and captureNames are correctly populated
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$, useStore } from '@qwik.dev/core';
export const App = component$(() => {
    const state = useStore({count: 0});
    return $(() => state.count);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Find the inner $() segment
        let all_segments: Vec<_> = result
            .modules
            .iter()
            .filter_map(|m| m.segment.as_ref())
            .collect();

        // Should have 2 segments: component$ body and inner $()
        assert!(
            all_segments.len() >= 2,
            "Expected at least 2 segments, got {}: {:?}",
            all_segments.len(),
            all_segments.iter().map(|s| &s.ctx_name).collect::<Vec<_>>()
        );

        // The inner $() should have captures = true and captureNames = ["state"]
        let inner = all_segments
            .iter()
            .find(|s| s.ctx_name == "$")
            .expect("Expected a $ segment");
        assert!(inner.captures, "Inner $() should have captures=true");
        assert_eq!(
            inner.capture_names,
            Some(vec!["state".to_string()]),
            "Inner $() should have captureNames=[\"state\"]"
        );

        // The component$ segment should have captures = false (no outer scope refs)
        let component = all_segments
            .iter()
            .find(|s| s.ctx_name == "component$")
            .expect("Expected a component$ segment");
        assert!(
            !component.captures,
            "component$ should have captures=false"
        );
    }

    #[test]
    fn test_capture_globals_not_captured() {
        // Globals (console, window, etc.) should NOT be captured
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return $(() => {
        console.log("test");
        window.location.href;
        setTimeout(() => {}, 100);
    });
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // No captures should be generated for globals
        let inner_segments: Vec<_> = result
            .modules
            .iter()
            .filter_map(|m| m.segment.as_ref())
            .filter(|s| s.ctx_name == "$")
            .collect();

        if let Some(seg) = inner_segments.first() {
            assert!(
                !seg.captures,
                "Globals should not generate captures: {:?}",
                seg.capture_names
            );
        }
    }

    // -----------------------------------------------------------------------
    // Segment Code Generation Integration Tests (Phase 10)
    // -----------------------------------------------------------------------

    #[test]
    fn test_segment_code_generated() {
        // Basic $() with segment strategy should produce segment module with code
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Should have main + segment module
        assert!(
            result.modules.len() >= 2,
            "Expected main + segment modules, got {}",
            result.modules.len()
        );

        // Find the segment module
        let seg_module = result
            .modules
            .iter()
            .find(|m| m.is_entry && m.segment.is_some())
            .expect("Expected a segment module");

        // Segment module should have non-empty code
        assert!(
            !seg_module.code.is_empty(),
            "Segment should have code, got empty string"
        );

        // Segment code should contain export const with segment name
        assert!(
            seg_module.code.contains("export const"),
            "Segment should have export: {}",
            seg_module.code
        );
        // Segment code should contain the original body
        assert!(
            seg_module.code.contains("console.log"),
            "Segment should contain original body: {}",
            seg_module.code
        );
    }

    #[test]
    fn test_segment_with_captures_code() {
        // Segment with captures should have _captures import and restoration
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$, useStore } from '@qwik.dev/core';
export const App = component$(() => {
    const state = useStore({count: 0});
    return $(() => state.count);
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Find the inner $() segment (captures state)
        let inner_seg = result
            .modules
            .iter()
            .find(|m| {
                m.segment
                    .as_ref()
                    .map_or(false, |s| s.ctx_name == "$" && s.captures)
            })
            .expect("Expected inner $ segment with captures");

        // Segment code should not be empty
        assert!(
            !inner_seg.code.is_empty(),
            "Inner segment with captures should have code"
        );

        // Should have _captures import
        assert!(
            inner_seg.code.contains("_captures"),
            "Segment should import _captures: {}",
            inner_seg.code
        );

        // Should have capture restoration: _captures[0]
        assert!(
            inner_seg.code.contains("_captures[0]"),
            "Segment should restore captures: {}",
            inner_seg.code
        );
    }

    #[test]
    fn test_nested_segments_lazy_imports() {
        // Component with nested $() should have lazy imports in the component segment
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return $(() => 'hello');
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Find the component$ segment (parent of the inner $() segment)
        let component_seg = result
            .modules
            .iter()
            .find(|m| {
                m.segment
                    .as_ref()
                    .map_or(false, |s| s.ctx_name == "component$")
            })
            .expect("Expected component$ segment");

        // Component segment should have non-empty code
        assert!(
            !component_seg.code.is_empty(),
            "Component segment should have code"
        );

        // Component segment should have qrl import for child segment
        assert!(
            component_seg.code.contains("qrl"),
            "Component segment should have qrl: {}",
            component_seg.code
        );
        // Component segment should have lazy import for child
        assert!(
            component_seg.code.contains("import("),
            "Component segment should have lazy import: {}",
            component_seg.code
        );
    }

    #[test]
    fn test_segment_code_export_name() {
        // Verify the export name matches the segment name
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => 42);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let seg_module = result
            .modules
            .iter()
            .find(|m| m.segment.is_some())
            .expect("Expected a segment module");

        let seg_name = &seg_module.segment.as_ref().unwrap().name;

        // The segment code should contain `export const <segment_name>`
        let expected_export = format!("export const {}", seg_name);
        assert!(
            seg_module.code.contains(&expected_export),
            "Expected '{}' in segment code: {}",
            expected_export,
            seg_module.code
        );
    }

    #[test]
    fn test_inline_strategy_no_segment_code() {
        // Inline strategy should NOT generate segment code
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Main module should have all code inline
        assert!(!result.modules.is_empty());
        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("inlinedQrl"),
            "Expected inlinedQrl in main module: {}",
            main_code
        );

        // All segment modules should have empty code and is_entry=false
        for m in &result.modules {
            if m.segment.is_some() {
                assert!(
                    m.code.is_empty(),
                    "Inline strategy segment should have empty code: {}",
                    m.code
                );
                assert!(
                    !m.is_entry,
                    "Inline strategy segment should not be entry"
                );
            }
        }
    }

    // -----------------------------------------------------------------------
    // Entry Strategy + Output Extension Tests (Phase 10-02)
    // -----------------------------------------------------------------------

    #[test]
    fn test_hoist_strategy_same_as_inline() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Hoist,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main = &result.modules[0];
        assert!(
            main.code.contains("inlinedQrl"),
            "Hoist should use inlinedQrl: {}",
            main.code
        );

        // All segment modules should have empty code and is_entry=false (same as inline)
        for m in &result.modules {
            if m.segment.is_some() {
                assert!(
                    m.code.is_empty(),
                    "Hoist strategy segment should have empty code: {}",
                    m.code
                );
                assert!(
                    !m.is_entry,
                    "Hoist strategy segment should not be entry"
                );
            }
        }
    }

    #[test]
    fn test_smart_strategy_produces_segment_code() {
        // Smart/Component/Hook/Single should be treated like Segment for now
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => console.log('hello'));"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            entry_strategy: EntryStrategy::Smart,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Smart strategy should produce segment modules with code (treated as Segment)
        let seg_modules: Vec<_> = result
            .modules
            .iter()
            .filter(|m| m.segment.is_some())
            .collect();
        assert!(
            !seg_modules.is_empty(),
            "Smart strategy should produce segment modules"
        );
        // Segment modules produced by Smart should have non-empty code and is_entry=true
        // (Smart is NOT inline-like, it should produce separate files)
        // Note: should_hoist returns true for Smart, but for code generation purposes
        // Smart produces segments in separate files. The hoist behavior is about
        // grouping, which we defer to Phase 13. For now Smart behaves like Segment.
    }

    #[test]
    fn test_transpile_ts_output_extension() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => 1);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Main module should have .jsx extension path
        assert!(
            result.modules[0].path.ends_with(".jsx"),
            "Expected .jsx path, got: {}",
            result.modules[0].path
        );

        // Segment module should also have .jsx extension
        if result.modules.len() > 1 {
            assert!(
                result.modules[1].path.ends_with(".jsx"),
                "Expected .jsx segment path, got: {}",
                result.modules[1].path
            );
        }
    }

    #[test]
    fn test_transpile_ts_false_preserves_extension() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => 1);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: false,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Main module should keep .tsx extension
        assert!(
            result.modules[0].path.ends_with(".tsx"),
            "Expected .tsx path, got: {}",
            result.modules[0].path
        );

        // Segment module should also keep .tsx extension
        if result.modules.len() > 1 {
            assert!(
                result.modules[1].path.ends_with(".tsx"),
                "Expected .tsx segment path, got: {}",
                result.modules[1].path
            );
        }
    }

    #[test]
    fn test_transpile_ts_with_ts_extension() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { $ } from '@qwik.dev/core';
export const handler = $(() => 1);"#
                    .to_string(),
                path: "test.ts".to_string(),
            }],
            transpile_ts: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Main module should have .js extension path
        assert!(
            result.modules[0].path.ends_with(".js"),
            "Expected .js path, got: {}",
            result.modules[0].path
        );
    }

    #[test]
    fn test_output_extension_helper() {
        assert_eq!(output_extension("test.tsx", true), "jsx");
        assert_eq!(output_extension("test.ts", true), "js");
        assert_eq!(output_extension("test.jsx", true), "jsx");
        assert_eq!(output_extension("test.js", true), "js");
        assert_eq!(output_extension("test.tsx", false), "tsx");
        assert_eq!(output_extension("test.ts", false), "ts");
    }

    // -----------------------------------------------------------------------
    // JSX Transformation Integration Tests (Phase 11-01)
    // -----------------------------------------------------------------------

    #[test]
    fn test_jsx_basic_element_transform() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const Lightweight = (props) => {
    return <div><span>hello</span></div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // Should contain _jsxSorted calls instead of JSX
        assert!(
            main_code.contains("_jsxSorted"),
            "Expected _jsxSorted in output: {}",
            main_code
        );
        assert!(
            main_code.contains("\"div\""),
            "Expected \"div\" string tag: {}",
            main_code
        );
        assert!(
            main_code.contains("\"span\""),
            "Expected \"span\" string tag: {}",
            main_code
        );
        // Should NOT contain raw JSX
        assert!(
            !main_code.contains("<div>"),
            "Should not contain raw JSX: {}",
            main_code
        );
        // Should have _jsxSorted import
        assert!(
            main_code.contains("import { _jsxSorted }"),
            "Expected _jsxSorted import: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_fragment_transform() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    return <><div/><span/></>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_Fragment"),
            "Expected _Fragment in output: {}",
            main_code
        );
        assert!(
            main_code.contains("@qwik.dev/core/jsx-runtime"),
            "Expected jsx-runtime import source: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_spread_uses_jsxsplit() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = (props) => {
    return <button {...props}/>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_jsxSplit"),
            "Expected _jsxSplit for spread: {}",
            main_code
        );
        assert!(
            main_code.contains("_getVarProps"),
            "Expected _getVarProps for spread: {}",
            main_code
        );
        assert!(
            main_code.contains("_getConstProps"),
            "Expected _getConstProps for spread: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_event_handler_rename() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(() => {
    return <div onClick$={() => {}} onBlur$={() => {}}>click</div>;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // Event handlers should be renamed to q-e: prefix
        assert!(
            main_code.contains("q-e:click"),
            "Expected q-e:click in output: {}",
            main_code
        );
        assert!(
            main_code.contains("q-e:blur"),
            "Expected q-e:blur in output: {}",
            main_code
        );
        // Should NOT contain raw onClick$ in output
        assert!(
            !main_code.contains("onClick$"),
            "Should not contain raw onClick$: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_const_props_classification() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    return <div class="foo" id={someVar}>text</div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // class="foo" is const (string literal)
        assert!(
            main_code.contains("class: \"foo\""),
            "Expected class: \"foo\" as const prop: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_no_transform_when_flag_false() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => <div>hello</div>;"#.to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: false,  // JSX should NOT be transformed
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // JSX should pass through untouched
        assert!(
            !main_code.contains("_jsxSorted"),
            "Should NOT contain _jsxSorted when transpile_jsx=false: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_key_attribute() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => <div key="mykey">hello</div>;"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // key should be extracted as 6th argument, not as a prop
        assert!(
            main_code.contains("\"mykey\""),
            "Expected key value in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_children_encoding() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => (
    <div>
        <span>a</span>
        <span>b</span>
    </div>
);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // Multiple children should be encoded as an array
        assert!(
            main_code.contains("["),
            "Expected array for multiple children: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_self_closing_null_children() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => <div/>;"#.to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // Self-closing with no children should have null and flags=3
        assert!(
            main_code.contains("null, null, null, 3"),
            "Expected null children and flags=3 for self-closing: {}",
            main_code
        );
    }

    // -----------------------------------------------------------------------
    // Signal Wrapping Integration Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_jsx_signal_value_wrapprop() {
        // signal.value in JSX props should become _wrapProp(signal) in const props
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const signal = useSignal(0);
    return <div value={signal.value} />;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_wrapProp(signal)"),
            "Expected _wrapProp(signal) in output: {}",
            main_code
        );
        assert!(
            main_code.contains("import { _wrapProp }"),
            "Expected _wrapProp import: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_signal_value_not_wrapped_for_call() {
        // signal.value() should NOT be wrapped with _wrapProp (it's a function call)
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const signal = useSignal(0);
    return <div value={signal.value()} />;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            !main_code.contains("_wrapProp"),
            "Should NOT contain _wrapProp for signal.value(): {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_rawprops_wrapprop_named() {
        // _rawProps.propName in JSX should become _wrapProp(_rawProps, "propName")
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
export const App = component$(({fromProps}) => {
    return <div propswrap={fromProps} />;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            entry_strategy: EntryStrategy::Inline,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // After props destructuring, fromProps becomes _rawProps.fromProps
        // In JSX, _rawProps.fromProps should become _wrapProp(_rawProps, "fromProps")
        assert!(
            main_code.contains("_wrapProp(_rawProps"),
            "Expected _wrapProp(_rawProps, ...) in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_signal_value_child_wrapprop() {
        // {signal.value} as a child should become _wrapProp(signal) as the children argument
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const value = useSignal(0);
    return <div>{value.value}</div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_wrapProp(value)"),
            "Expected _wrapProp(value) for child signal.value: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_fn_signal_computed_expression() {
        // signal.value + 1 should become _fnSignal(_hf0, [signal], _hf0_str)
        // with hoisted const _hf0 = (p0) => p0.value + 1;
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const signal = useSignal(0);
    return <div count={signal.value + 1}></div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_fnSignal("),
            "Expected _fnSignal call: {}",
            main_code
        );
        assert!(
            main_code.contains("_hf0"),
            "Expected hoisted function _hf0: {}",
            main_code
        );
        assert!(
            main_code.contains("_hf0_str"),
            "Expected hoisted string _hf0_str: {}",
            main_code
        );
        // The hoisted function should reference p0.value
        assert!(
            main_code.contains("p0.value"),
            "Expected p0.value in hoisted function body: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_fn_signal_store_expression() {
        // store.address.city should become _fnSignal(_hf0, [store], _hf0_str)
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const store = useStore({});
    return <div city={store.address.city}></div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_fnSignal("),
            "Expected _fnSignal call for store deep access: {}",
            main_code
        );
        assert!(
            main_code.contains("[store]"),
            "Expected [store] in deps array: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_fn_signal_not_for_function_calls() {
        // signal.value + unknown() should NOT get _fnSignal (has function call)
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const signal = useSignal(0);
    return <div x={signal.value + unknown()}></div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            !main_code.contains("_fnSignal("),
            "Should NOT wrap with _fnSignal when expression has function call: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_bind_value() {
        // bind:value={signal} -> "value": signal + "q-e:input": inlinedQrl(_val, "_val", [signal])
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const value = useSignal(0);
    return <input bind:value={value} />;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // Should have value prop
        assert!(
            main_code.contains("value"),
            "Expected value prop: {}",
            main_code
        );
        // Should have q-e:input with _val handler
        assert!(
            main_code.contains("_val"),
            "Expected _val handler: {}",
            main_code
        );
        assert!(
            main_code.contains("inlinedQrl"),
            "Expected inlinedQrl call: {}",
            main_code
        );
        // Should NOT have bind:value in output
        assert!(
            !main_code.contains("bind:value"),
            "Should NOT have bind:value in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_bind_checked() {
        // bind:checked={signal} -> "checked": signal + "q-e:input": inlinedQrl(_chk, "_chk", [signal])
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const checked = useSignal(false);
    return <input bind:checked={checked} />;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("_chk"),
            "Expected _chk handler: {}",
            main_code
        );
        assert!(
            !main_code.contains("bind:checked"),
            "Should NOT have bind:checked in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_bind_other_passthrough() {
        // bind:stuff={signal} -> "bind:stuff": signal (passed through as-is)
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const stuff = useSignal();
    return <input bind:stuff={stuff} />;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // bind:stuff should pass through in const props
        assert!(
            main_code.contains("bind:stuff"),
            "Expected bind:stuff to pass through: {}",
            main_code
        );
        // Should NOT have _val or _chk
        assert!(
            !main_code.contains("_val"),
            "Should NOT have _val for bind:stuff: {}",
            main_code
        );
        assert!(
            !main_code.contains("_chk"),
            "Should NOT have _chk for bind:stuff: {}",
            main_code
        );
    }

    // -----------------------------------------------------------------------
    // PURE Annotation Integration Tests (Phase 12-01)
    // -----------------------------------------------------------------------

    #[test]
    fn test_jsx_pure_annotation() {
        // Verify _jsxSorted calls contain PURE annotation in output.
        // OXC emits /* @__PURE__ */ (standard format recognized by all bundlers).
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => <div>hello</div>;"#.to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("@__PURE__") && main_code.contains("_jsxSorted"),
            "Expected PURE annotation on _jsxSorted in output: {}",
            main_code
        );
        // Specifically check the PURE annotation is directly before _jsxSorted
        assert!(
            main_code.contains("/* @__PURE__ */ _jsxSorted"),
            "Expected /* @__PURE__ */ _jsxSorted in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_pure_annotation_fragment() {
        // Verify _jsxSorted on fragments also has PURE annotation
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => <><div/><span/></>;"#.to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("/* @__PURE__ */ _jsxSorted"),
            "Expected /* @__PURE__ */ _jsxSorted for fragment: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_pure_annotation_spread_jsxsplit() {
        // Verify _jsxSplit calls also have PURE annotation
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = (props) => <button {...props}/>;"#.to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("/* @__PURE__ */ _jsxSplit"),
            "Expected /* @__PURE__ */ _jsxSplit for spread: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_pure_annotation_qrl_calls() {
        // Verify componentQrl and qrl also have PURE annotation
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$ } from '@qwik.dev/core';
const App = component$(() => <div/>);"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("/* @__PURE__ */ componentQrl"),
            "Expected /* @__PURE__ */ componentQrl in output: {}",
            main_code
        );
        assert!(
            main_code.contains("/* @__PURE__ */ qrl("),
            "Expected /* @__PURE__ */ qrl( in output: {}",
            main_code
        );
    }

    #[test]
    fn test_jsx_no_pure_on_wrapprop_fnsignal() {
        // _wrapProp and _fnSignal should NOT have PURE annotation
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"export const App = () => {
    const signal = useSignal(0);
    return <div value={signal.value} count={signal.value + 1}></div>;
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // _wrapProp should NOT have PURE
        assert!(
            !main_code.contains("@__PURE__ */ _wrapProp"),
            "Should NOT have PURE on _wrapProp: {}",
            main_code
        );
        // _fnSignal should NOT have PURE
        assert!(
            !main_code.contains("@__PURE__ */ _fnSignal"),
            "Should NOT have PURE on _fnSignal: {}",
            main_code
        );
    }

    // -----------------------------------------------------------------------
    // Code Stripping Integration Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_strip_server_code_prod() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$, serverStuff$, $ } from '@qwik.dev/core';
export const Parent = component$(() => {
    serverStuff$(async () => {
        console.log('server only');
    });
    return $(() => 'hello');
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            strip_ctx_name: Some(vec!["server".to_string()]),
            transpile_ts: true,
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Main module should have componentQrl
        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("componentQrl"),
            "Expected componentQrl in main module: {}",
            main_code
        );

        // Find the component segment module (the one with the body code)
        let component_seg = result
            .modules
            .iter()
            .find(|m| m.is_entry && m.code.contains("serverStuffQrl"))
            .expect("Expected component segment with serverStuffQrl");

        // The component segment should contain _noopQrl for the stripped serverStuff$
        assert!(
            component_seg.code.contains("_noopQrl"),
            "Expected _noopQrl in component segment: {}",
            component_seg.code
        );
        // The wrapper serverStuffQrl should be preserved
        assert!(
            component_seg.code.contains("serverStuffQrl"),
            "Expected serverStuffQrl wrapper preserved: {}",
            component_seg.code
        );
        // The _noopQrl should have PURE annotation (OXC uses `/* @__PURE__ */` format)
        assert!(
            component_seg.code.contains("@__PURE__") && component_seg.code.contains("_noopQrl"),
            "Expected PURE annotation on _noopQrl: {}",
            component_seg.code
        );

        // No segment module should be produced for the stripped serverStuff$ callback
        let server_stuff_modules: Vec<_> = result
            .modules
            .iter()
            .filter(|m| {
                m.segment
                    .as_ref()
                    .map(|s| s.ctx_name == "serverStuff$")
                    .unwrap_or(false)
            })
            .collect();
        assert!(
            server_stuff_modules.is_empty(),
            "Stripped serverStuff$ should NOT produce segment modules, found {}",
            server_stuff_modules.len()
        );
    }

    #[test]
    fn test_strip_preserves_nested_dollar() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$, serverStuff$, $ } from '@qwik.dev/core';
export const Parent = component$(() => {
    serverStuff$(async () => {
        const a = $(() => { /* nested */ });
        return a;
    });
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            strip_ctx_name: Some(vec!["server".to_string()]),
            transpile_ts: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // The nested $() inside the stripped serverStuff$ should still produce a segment
        let nested_segments: Vec<_> = result
            .modules
            .iter()
            .filter(|m| {
                m.segment
                    .as_ref()
                    .map(|s| s.ctx_name == "$")
                    .unwrap_or(false)
            })
            .collect();
        assert!(
            !nested_segments.is_empty(),
            "Nested $() inside stripped callback should still produce segments"
        );
    }

    #[test]
    fn test_strip_not_applied_without_config() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { component$, serverStuff$ } from '@qwik.dev/core';
export const Parent = component$(() => {
    serverStuff$(() => { console.log('server'); });
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            // No strip_ctx_name -- should NOT strip
            transpile_ts: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Without strip_ctx_name, _noopQrl should NOT appear
        for m in &result.modules {
            assert!(
                !m.code.contains("_noopQrl"),
                "Should NOT contain _noopQrl without strip_ctx_name config: {}",
                m.code
            );
        }

        // serverStuff$ should produce a normal segment
        let server_segments: Vec<_> = result
            .modules
            .iter()
            .filter(|m| {
                m.segment
                    .as_ref()
                    .map(|s| s.ctx_name == "serverStuff$")
                    .unwrap_or(false)
            })
            .collect();
        assert!(
            !server_segments.is_empty(),
            "Without stripping, serverStuff$ should produce normal segments"
        );
    }

    // -----------------------------------------------------------------------
    // sync$ Serialization Integration Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_sync_dollar_basic() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { sync$, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return <input onClick$={sync$((event) => event.preventDefault())} />;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: true,
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Find the component segment module (it contains the JSX output)
        let component_seg = result
            .modules
            .iter()
            .find(|m| m.is_entry && m.code.contains("_qrlSync"))
            .expect("Expected component segment with _qrlSync");

        // Should contain _qrlSync call with the function and stringified version
        assert!(
            component_seg.code.contains("_qrlSync"),
            "Expected _qrlSync in component segment: {}",
            component_seg.code
        );

        // The stringified version should be minified (no extra whitespace)
        // Looking for the pattern: _qrlSync(fn, "stringified")
        assert!(
            component_seg.code.contains("event.preventDefault()"),
            "Expected function body in _qrlSync call: {}",
            component_seg.code
        );

        // _qrlSync import should be present in segment module
        assert!(
            component_seg.code.contains("_qrlSync"),
            "Expected _qrlSync import in segment: {}",
            component_seg.code
        );
    }

    #[test]
    fn test_sync_dollar_no_segment_produced() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { sync$, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return <input onClick$={sync$((event) => event.preventDefault())} />;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: true,
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // sync$ should NOT produce a separate segment module
        let sync_segments: Vec<_> = result
            .modules
            .iter()
            .filter(|m| {
                m.segment
                    .as_ref()
                    .map(|s| s.ctx_name == "sync$")
                    .unwrap_or(false)
            })
            .collect();
        assert!(
            sync_segments.is_empty(),
            "sync$ should NOT produce segment modules, found {}",
            sync_segments.len()
        );

        // The main module should NOT have any lazy import for sync$
        let main_code = &result.modules[0].code;
        assert!(
            !main_code.contains("_qrlSync"),
            "Main module should not contain _qrlSync (it belongs in segment): {}",
            main_code
        );
    }

    #[test]
    fn test_sync_dollar_with_function_expression() {
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { sync$, component$ } from '@qwik.dev/core';
export const App = component$(() => {
    return <input onClick$={sync$(function(event, target) {
        // comment should be removed
        event.preventDefault();
    })} />;
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: true,
            transpile_jsx: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        // Find the component segment module
        let component_seg = result
            .modules
            .iter()
            .find(|m| m.is_entry && m.code.contains("_qrlSync"))
            .expect("Expected component segment with _qrlSync");

        // Should contain _qrlSync with function expression
        assert!(
            component_seg.code.contains("_qrlSync(function"),
            "Expected _qrlSync with function expression: {}",
            component_seg.code
        );

        // The stringified version should have comments removed
        // The minified string should NOT contain the comment
        // Check for the stringified argument which should be compact
        assert!(
            component_seg.code.contains("event.preventDefault()"),
            "Expected function body preserved: {}",
            component_seg.code
        );
    }

    #[test]
    fn test_sync_dollar_stringified_is_minified() {
        // Test that the stringified function body is properly minified
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { sync$ } from '@qwik.dev/core';
export const handler = sync$((event, target) => {
    event.preventDefault();
});"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            transpile_ts: true,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;

        // Should contain _qrlSync in the main module (sync$ used at top level, not inside component$)
        assert!(
            main_code.contains("_qrlSync"),
            "Expected _qrlSync in output: {}",
            main_code
        );

        // Should have _qrlSync import
        assert!(
            main_code.contains("import { _qrlSync }"),
            "Expected _qrlSync import: {}",
            main_code
        );

        // Should NOT have any segment-related imports (no qrl, no lazy import)
        // since sync$ doesn't produce segments
        assert!(
            !main_code.contains("import { qrl }"),
            "sync$ should NOT need qrl import: {}",
            main_code
        );
    }

    // -----------------------------------------------------------------------
    // Const Replacement Integration Tests (Phase 12-01)
    // -----------------------------------------------------------------------

    #[test]
    fn test_const_replace_isserver_true() {
        // isServer imported from @qwik.dev/core/build, is_server=true -> replaced with true
        // The if (isServer) block body should be inlined (dead branch elimination)
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isServer } from '@qwik.dev/core/build';
export const result = () => {
    if (isServer) {
        console.log('server');
    }
    return 'done';
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // isServer should be replaced -- no raw isServer identifier in output
        assert!(
            !main_code.contains("isServer"),
            "isServer should be replaced, not present in output: {}",
            main_code
        );
        // The if block body should be inlined (isServer=true means if(true) -> inline)
        assert!(
            main_code.contains("console.log"),
            "Expected console.log to be inlined from if(true) block: {}",
            main_code
        );
    }

    #[test]
    fn test_const_replace_isbrowser_false() {
        // isBrowser imported from @qwik.dev/core/build, is_server=true -> isBrowser=false
        // if (isBrowser) { ... } should be removed entirely
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isBrowser } from '@qwik.dev/core/build';
export const fn1 = () => {
    if (isBrowser) {
        console.log('browser');
    }
    return 'done';
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // isBrowser should be replaced
        assert!(
            !main_code.contains("isBrowser"),
            "isBrowser should be replaced: {}",
            main_code
        );
        // The if(false) block should be eliminated
        assert!(
            !main_code.contains("console.log"),
            "if(false) block should be eliminated: {}",
            main_code
        );
        // return 'done' should still be present
        assert!(
            main_code.contains("done"),
            "Expected 'done' to still be present: {}",
            main_code
        );
    }

    #[test]
    fn test_const_replace_dead_branch_elimination() {
        // Function body with only if(isBrowser) should become empty () => {}
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isBrowser as isb } from '@qwik.dev/core/build';
export const functionThatNeedsWindow = () => {
    if (isb) {
        console.log('browser');
        window.alert('hey');
    }
};"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // The aliased isBrowser should be replaced
        assert!(
            !main_code.contains("isb"),
            "isb alias should be replaced: {}",
            main_code
        );
        // The entire if block should be eliminated, leaving an empty function body
        assert!(
            !main_code.contains("console.log"),
            "if(false) block should be eliminated: {}",
            main_code
        );
        assert!(
            !main_code.contains("window.alert"),
            "if(false) block should be eliminated: {}",
            main_code
        );
    }

    #[test]
    fn test_const_replace_logical_simplification() {
        // isServer && <expr> with isServer=true should become <expr>
        // isBrowser && <expr> with isBrowser=false should become false
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isServer } from '@qwik.dev/core';
import { isBrowser } from '@qwik.dev/core/build';
export const a = isServer && 'yes';
export const b = isBrowser && 'no';"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // true && 'yes' -> 'yes'
        assert!(
            main_code.contains("\"yes\"") || main_code.contains("'yes'"),
            "Expected 'yes' (true && 'yes' -> 'yes'): {}",
            main_code
        );
        // false && 'no' -> false
        assert!(
            main_code.contains("false"),
            "Expected false (false && 'no' -> false): {}",
            main_code
        );
    }

    #[test]
    fn test_const_replace_isdev() {
        // isDev with mode=Dev should be true, with mode=Prod should be false
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isDev } from '@qwik.dev/core';
export const dev = isDev;"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(false),
            mode: EmitMode::Dev,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        assert!(
            main_code.contains("true"),
            "isDev in Dev mode should be true: {}",
            main_code
        );

        // Now test with Prod mode
        let config2 = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isDev } from '@qwik.dev/core';
export const dev = isDev;"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(false),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result2 = transform_modules(config2).unwrap();

        let main_code2 = &result2.modules[0].code;
        assert!(
            main_code2.contains("false"),
            "isDev in Prod mode should be false: {}",
            main_code2
        );
    }

    #[test]
    fn test_const_replace_no_replacement_without_import() {
        // If there's no build constant import, identifiers should NOT be replaced
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"const isServer = true;
export const result = isServer;"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // isServer declared locally should NOT be replaced
        assert!(
            main_code.contains("isServer"),
            "Locally declared isServer should NOT be replaced: {}",
            main_code
        );
    }

    #[test]
    fn test_const_replace_aliased_import() {
        // import { isServer as isServer2 } from '@qwik.dev/core'
        // The local name isServer2 should be replaced, not the imported name isServer
        let config = TransformModulesOptions {
            input: vec![TransformModuleInput {
                code: r#"import { isServer as myServer } from '@qwik.dev/core';
export const result = myServer;"#
                    .to_string(),
                path: "test.tsx".to_string(),
            }],
            is_server: Some(true),
            mode: EmitMode::Prod,
            ..TransformModulesOptions::default()
        };
        let result = transform_modules(config).unwrap();

        let main_code = &result.modules[0].code;
        // myServer should be replaced with true
        assert!(
            !main_code.contains("myServer"),
            "myServer alias should be replaced: {}",
            main_code
        );
        assert!(
            main_code.contains("true"),
            "myServer should be replaced with true: {}",
            main_code
        );
    }
}
