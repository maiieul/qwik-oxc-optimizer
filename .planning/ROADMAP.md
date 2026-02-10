# Roadmap: Qwik Optimizer Spec Generation

## Overview

This milestone produces a complete behavioral specification of the Qwik optimizer by documenting all 162 SWC snapshot tests as structured markdown files. Phase 1 builds a small Rust utility to generate OXC ASTs. Phase 2 generates all 162 spec files (each is independent: read a snapshot, read its test config, generate ASTs, write the markdown). Phase 3 verifies completeness and consistency.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: OXC AST Utility** - Build Rust crate that parses code with oxc_parser and emits AST JSON
- [ ] **Phase 2: Generate All Spec Files** - Generate all 162 spec files by reading snapshots, test configs, and producing ASTs
- [ ] **Phase 3: Verify Completeness** - Verify all 162 specs are complete, consistent in structure, and all conventions documented

## Phase Details

### Phase 1: OXC AST Utility
**Goal**: A working Rust utility exists that accepts code + file extension and outputs oxc_parser AST as JSON
**Depends on**: Nothing (first phase)
**Requirements**: INFRA-01, INFRA-02
**Success Criteria** (what must be TRUE):
  1. Running the utility with a JavaScript code string produces valid JSON AST output
  2. Running the utility with a TypeScript code string (tsx extension) produces valid JSON AST output
  3. The utility reports parse errors in the output rather than crashing on malformed code
  4. The JSON output can be embedded in a markdown `<details>` block without corruption
**Plans**: TBD

Plans:
- [ ] 01-01: Create Rust crate with oxc + serde_json dependencies and implement parse-to-JSON utility

### Phase 2: Generate All Spec Files
**Goal**: All 162 spec files exist, each fully documenting one snapshot test's transformations, conventions, and ASTs
**Depends on**: Phase 1
**Requirements**: SPEC-01, SPEC-02, SPEC-03, SPEC-04, SPEC-05, SPEC-06, SPEC-07, SPEC-08, SPEC-09, SPEC-10, CONV-01, CONV-02, CONV-03, CONV-04, CONV-05, CONV-06, CONV-07, CONV-08, CONV-09, CONV-10, CONV-11, CONV-12, CONV-13, CONV-14, QUAL-01, QUAL-02, QUAL-03
**Success Criteria** (what must be TRUE):
  1. 162 spec files exist at `.planning/spec/<test_name>.md`, one per snapshot test
  2. Each spec file contains all required sections: test config, input code, input AST, output modules with code/AST, segment metadata, conventions applied, function calls, diagnostics
  3. A reader can understand what transformation occurred by reading the spec without expanding AST details
  4. Every convention present in each snapshot is identified and documented (no false negatives across all convention types: qrl calls, component$ conversions, JSX transforms, signal helpers, capture patterns, lazy imports, PURE annotations, segment extraction, stripping, const replacement, destructuring, input binding, sync$, hoisted functions)
  5. Spec files are human-readable and follow a consistent template
**Plans**: TBD

Plans:
- [ ] 02-01: Generate first batch of spec files (establish template with example_1 through example_11, then continue with remaining tests)

Note: Each spec file is independent work -- read a snapshot, read its test config, generate ASTs with the Phase 1 utility, write the markdown. Batching within plans is for parallelization, not dependency ordering.

**All 162 tests:**
example_1, example_2, example_3, example_4, example_5, example_6, example_7, example_8, example_9, example_10, example_11, example_build_server, example_functional_component, example_functional_component_2, example_functional_component_capture_props, example_lightweight_functional, example_custom_inlined_functions, example_missing_custom_inlined_functions, example_of_synchronous_qrl, example_parsed_inlined_qrls, example_inlined_entry_strategy, example_manual_chunks, example_use_client_effect, example_use_optimization, example_use_server_mount, example_with_style, example_with_tagname, should_extract_single_qrl, should_extract_single_qrl_2, should_extract_single_qrl_with_index, should_extract_single_qrl_with_nested_components, should_transform_component_with_normal_function, should_transform_qrls_in_ternary_expression, example_jsx, example_jsx_import_source, example_jsx_keyed, example_jsx_keyed_dev, example_jsx_listeners, example_spread_jsx, example_input_bind, example_class_name, example_mutable_children, special_jsx, should_convert_jsx_events, should_handle_dangerously_set_inner_html, should_merge_attributes_with_spread_props, should_merge_attributes_with_spread_props_before_and_after, should_merge_bind_checked_and_on_input, should_merge_bind_value_and_on_input, should_merge_on_input_and_bind_checked, should_merge_on_input_and_bind_value, should_move_bind_value_to_var_props, should_not_transform_bind_checked_in_var_props_for_jsx_split, should_not_transform_bind_value_in_var_props_for_jsx_split, should_not_transform_events_on_non_elements, should_split_spread_props, should_transform_event_names_without_jsx_transpile, example_derived_signals_children, example_derived_signals_cmp, example_derived_signals_complext_children, example_derived_signals_div, example_derived_signals_multiple_children, example_getter_generation, example_immutable_analysis, example_immutable_function_components, example_props_optimization, example_props_wrapping, example_props_wrapping2, example_props_wrapping_children, example_props_wrapping_children2, hoisted_fn_signal_in_loop, impure_template_fns, lib_mode_fn_signal, should_mark_props_as_var_props_for_inner_cmp, should_not_wrap_fn, should_not_wrap_ternary_function_operator_with_fn, should_not_wrap_var_template_string, should_wrap_inner_inline_component_prop, should_wrap_logical_expression_in_template, destructure_args_colon_props, destructure_args_colon_props2, destructure_args_colon_props3, destructure_args_inline_cmp_block_stmt, destructure_args_inline_cmp_block_stmt2, destructure_args_inline_cmp_expr_stmt, example_capture_imports, example_capturing_fn_class, example_multi_capture, should_convert_rest_props, should_destructure_args, should_move_props_related_to_iteration_variables_to_var_props, should_not_generate_conflicting_props_identifiers, should_not_move_over_side_effects, should_wrap_object_with_fn_signal, should_wrap_prop_from_destructured_array, should_wrap_store_expression, should_wrap_type_asserted_variables_in_template, example_dev_mode, example_dev_mode_inlined, example_noop_dev_mode, example_prod_node, example_server_auth, example_dead_code, example_drop_side_effects, example_strip_client_code, example_strip_exports_unused, example_strip_exports_used, example_strip_server_code, example_transpile_jsx_only, example_transpile_ts_only, example_explicit_ext_no_transpile, example_explicit_ext_transpile, example_ts_enums, example_ts_enums_issue_1341, example_ts_enums_no_transpile, example_skip_transform, example_component_with_event_listeners_inside_loop, example_default_export, example_default_export_index, example_default_export_invalid_ident, example_export_issue, example_exports, example_renamed_exports, example_fix_dynamic_import, example_import_assertion, example_preserve_filenames, example_preserve_filenames_segments, example_reg_ctx_name_segments, example_reg_ctx_name_segments_hoisted, example_reg_ctx_name_segments_inlined, example_qwik_conflict, example_qwik_react, example_qwik_react_inline, example_qwik_router_inline, rename_builder_io, relative_paths, example_invalid_references, example_invalid_segment_expr1, example_issue_33443, example_issue_4438, example_optimization_issue_3542, example_optimization_issue_3561, example_optimization_issue_3795, example_optimization_issue_4386, issue_117, issue_150, issue_476, issue_5008, issue_7216_add_test, issue_964, should_ignore_null_inlined_qrl, should_split_spread_props_with_additional_prop, should_split_spread_props_with_additional_prop2, should_split_spread_props_with_additional_prop3, should_split_spread_props_with_additional_prop4, should_split_spread_props_with_additional_prop5, should_transform_multiple_event_handlers, should_transform_multiple_event_handlers_case2, should_transform_nested_loops, support_windows_paths, ternary_prop, transform_qrl_in_regular_prop

### Phase 3: Verify Completeness
**Goal**: All 162 spec files are verified complete, structurally consistent, and all conventions across the entire test suite are documented
**Depends on**: Phase 2
**Requirements**: QUAL-04
**Success Criteria** (what must be TRUE):
  1. Exactly 162 spec files exist in `.planning/spec/`
  2. Every spec file contains all required sections (test config, input code, input AST, output modules, segment metadata, conventions, function calls, diagnostics)
  3. No spec file is missing conventions that are present in its snapshot output
  4. Spec files are structurally consistent (same heading hierarchy, same section order)
**Plans**: TBD

Plans:
- [ ] 03-01: Audit all 162 spec files for completeness and structural consistency

## Progress

**Execution Order:**
Phases execute in numeric order: 1 -> 2 -> 3

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. OXC AST Utility | 0/1 | Not started | - |
| 2. Generate All Spec Files | 0/1 | Not started | - |
| 3. Verify Completeness | 0/1 | Not started | - |
