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

        // 5. Emit the transformed module
        let emit_result = emit::emit_module(&program, source_in_arena, &emit_options);

        // 6. Build the main TransformModule
        let main_module = TransformModule {
            path: input.path.clone(),
            is_entry: false,
            code: emit_result.code,
            map: emit_result.map,
            segment: None,
            orig_path: Some(input.path.clone()),
        };
        all_modules.push(main_module);

        // 7. Build segment modules
        let segments = qwik_transform.extracted_segments();
        for seg in segments {
            let segment_analysis = segment_data_to_analysis(seg, &input.path);

            // For segment strategy, the segment code is generated in Phase 10 (code_move.rs)
            // For now, create a placeholder TransformModule
            let segment_module = TransformModule {
                path: format!("{}.{}", segment_analysis.canonical_filename, seg.extension),
                is_entry: true,
                code: String::new(), // Placeholder -- Phase 10 will generate segment code
                map: None,
                segment: Some(segment_analysis),
                orig_path: Some(input.path.clone()),
            };
            all_modules.push(segment_module);
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
}
