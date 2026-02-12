# Output Audit Report

## Executive Summary

- **Total specs audited:** 162
- **Specs passing (all modules match):** 1
- **Specs with deviations:** 161
- **Specs errored (optimizer failure):** 0
- **Total deviations:** 499

### Severity Breakdown

- **Runtime-breaking deviations:** 293 across 140 specs
- **Cosmetic deviations:** 206 across 119 specs
- **Specs with only cosmetic deviations:** 21

### Category Breakdown

| Category | Runtime-Breaking | Cosmetic | Total |
|----------|-----------------|----------|-------|
| capture | 27 | 7 | 34 |
| import | 52 | 86 | 138 |
| qrl | 54 | 0 | 54 |
| codegen | 77 | 56 | 133 |
| module | 83 | 57 | 140 |
| **Total** | **293** | **206** | **499** |

### Pattern Breakdown

The 499 deviations fall into these specific patterns:

| Pattern | Count | Severity | Category | Description |
|---------|-------|----------|----------|-------------|
| missing-module | 78 | runtime-breaking | module | Expected module not found in actual output |
| extra-module | 57 | cosmetic | module | Extra module in actual not in expected |
| entry-extra-imports | 54 | cosmetic | import | Entry module includes segment imports that expected omits |
| qrl-extraction | 45 | runtime-breaking | qrl | Event handlers not extracted to QRL segments |
| missing-imports | 39 | runtime-breaking | import | Required imports missing from module |
| hash-diff | 34 | cosmetic | codegen | Only hash identifiers differ (different hash algorithm) |
| extra-imports | 32 | cosmetic | import | Unused extra imports in module (tree-shakeable) |
| empty-segment | 32 | runtime-breaking | codegen | Module matched but actual body is empty |
| codegen-missing-imports | 24 | runtime-breaking | codegen | Code structure differs with missing imports |
| naming-convention | 21 | cosmetic | codegen | Segment naming differs but self-consistent |
| const-let-var | 20 | runtime-breaking | codegen | Variable declaration count/type differs |
| capture-diff | 14 | runtime-breaking | capture | Both use captures but variable lists differ |
| empty-segment-capture | 13 | runtime-breaking | capture | Segment body empty where capture code expected |
| import-mismatch | 13 | runtime-breaking | import | Some imports missing, others extra |
| inlinedqrl-diff | 9 | runtime-breaking | qrl | Inlined QRL count differs |
| extra-captures | 7 | cosmetic | capture | Actual has unnecessary capture restoration |
| optimizer-failure | 3 | runtime-breaking | module | Optimizer produced zero output modules |
| module-count | 2 | runtime-breaking | module | Module count mismatch with partial output |
| token-diff | 1 | cosmetic | codegen | Token difference in identifiers (non-hash) |
| codegen-extra-imports | 1 | runtime-breaking | codegen | Code differs with extra imports |

---

## Known Deviations Re-evaluated

### 5 Module Count Deviations

These specs have mismatched module counts between expected and actual optimizer output.

#### `example_3`

- **Expected modules:** 3
- **Actual modules:** 0
- **Classification:** Runtime-breaking
- **Rationale:** Optimizer produced no output modules at all. This means the entire spec's code splitting fails -- no lazy-loadable segments are generated. A JavaScript runtime would fail to import any of the expected segments.
- **Impact:** The App/Header component and its onClick handler would not be extractable as separate lazy segments.

#### `example_component_with_event_listeners_inside_loop`

- **Expected modules:** 8
- **Actual modules:** 0
- **Classification:** Runtime-breaking
- **Rationale:** Optimizer produced no output modules at all. This means the entire spec's code splitting fails -- no lazy-loadable segments are generated. A JavaScript runtime would fail to import any of the expected segments.
- **Impact:** 8 expected modules including loop event handlers (for-in, for-of, while, arrow) would all be missing. No lazy loading of any event handlers in loop constructs.

#### `example_immutable_analysis`

- **Expected modules:** 6
- **Actual modules:** 0
- **Classification:** Runtime-breaking
- **Rationale:** Optimizer produced no output modules at all. This means the entire spec's code splitting fails -- no lazy-loadable segments are generated. A JavaScript runtime would fail to import any of the expected segments.
- **Impact:** 6 expected modules including immutability analysis outputs and event handlers would be missing. No fine-grained reactivity optimization.

#### `example_qwik_react`

- **Expected modules:** 3
- **Actual modules:** 1
- **Classification:** Runtime-breaking
- **Rationale:** Only the entry module was produced (1 of 3). The qwikifyQrl component and useWatch segments are missing. A Qwik React integration component would fail to hydrate because its lazy segments don't exist.
- **Missing:** `qwikifyQrl_component_useWatch` and `qwikifyQrl_component` segments

#### `relative_paths`

- **Expected modules:** 5
- **Actual modules:** 3
- **Classification:** Runtime-breaking
- **Rationale:** 3 of 5 modules produced, but 2 segments are missing from the dep library path. The App component and its onClick handler from `node_modules/dep/dist/lib.mjs` would fail to lazy-load.
- **Missing:** `App_component_div_p_button_onClick` and `App_component` segments from dep library

### Capture Deviations (re-evaluated from 16 known)

The Plan 01 audit identified capture analysis as a key area of deviation. After full output comparison, we find **34 capture-related deviations** across 31 specs. The original count of 16 referred to a subset where useLexicalScope differences were directly detectable in the diff summary; the full analysis reveals additional capture issues when comparing actual code content.

#### Pattern: empty-segment-capture (13 deviations, runtime-breaking)

These segments were matched structurally but the actual output has an empty body where the expected code contains `_captures` restoration logic. The expected code uses `_captures[N]` to restore closure variables from the parent scope.

**Why runtime-breaking:** Without capture restoration, the segment function has no access to the variables it needs from its enclosing scope. Any reference to a captured variable would be `undefined`, causing runtime errors.

| Spec | Expected Module | Issue |
|------|----------------|-------|
| `destructure_args_inline_cmp_block_stmt` | `test.tsx_test_div_q_e_click_pFqTss400MA.js` | Empty body, expected `_captures` code |
| `destructure_args_inline_cmp_block_stmt2` | `test.tsx_test_div_q_e_click_pFqTss400MA.js` | Empty body, expected `_captures` code |
| `destructure_args_inline_cmp_expr_stmt` | `test.tsx_test_div_q_e_click_pFqTss400MA.js` | Empty body, expected `_captures` code |
| `example_functional_component_2` | `test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js` | Empty body, expected `_captures` code |
| `example_functional_component_capture_props` | `test.tsx_App_component_div_q_e_click_mi4E1piTWe8.js` | Empty body, expected `_captures` code |
| `impure_template_fns` | `test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw.ts` | Empty body, expected `_captures` code |
| `issue_5008` | `test.tsx_test_component_Fragment_button_q_e_click_7MTd2pAiliw.js` | Empty body, expected `_captures` code |
| `lib_mode_fn_signal` | `test.tsx_Counter_component_div_p_button_q_e_click_Pq1pmfmJWUI.ts` | Empty body, expected `_captures` code |
| `should_handle_dangerously_set_inner_html` | `test.tsx_Cmp_component_div_div_button_q_e_click_kOQ2BgBVVS8.js` | Empty body, expected `_captures` code |
| `should_not_wrap_fn` | `test.tsx_Cmp_component_Fragment_button_q_e_click_veAZ2ow0cnM.js` | Empty body, expected `_captures` code |
| `should_split_spread_props_with_additional_prop4` | `test.tsx_test_component_button_q_e_click_qwSL5gM03T4.js` | Empty body, expected `_captures` code |
| `should_transform_qrls_in_ternary_expression` | `test.tsx_FieldInput_component_input_q_e_focus_Sgf3MDWzexI.js` | Empty body, expected `_captures` code |
| `should_wrap_prop_from_destructured_array` | `test.tsx_test_component_div_button_q_e_click_2TvarUvNGmU.js` | Empty body, expected `_captures` code |

#### Pattern: capture-diff (14 deviations, runtime-breaking)

Both expected and actual code use `_captures` or `useLexicalScope`, but the capture variable lists or usage patterns differ.

**Why runtime-breaking:** If the capture list indices don't match the serialized closure state, variables will be assigned wrong values at runtime. The segment would execute with incorrect data.

| Spec | Expected Module | Actual Module |
|------|----------------|---------------|
| `example_custom_inlined_functions` | `test.tsx_App_component_1_w0t0o3QMovU.js` | `test.tsx_s__Ma8uJBKLX2o.js` |
| `example_jsx` | `test.tsx_Foo_component_1_DvU6FitWglY.js` | `test.tsx_Foo_T0dY5k6mBgU.js` |
| `example_parsed_inlined_qrls` | `test.tsx` | `test.tsx` |
| `example_props_optimization` | `test.js` | `test.js` |
| `example_qwik_react_inline` | `../node_modules/@qwik.dev/react/index.qwik.mjs` | `../node_modules/@qwik.dev/react/index.qwik.mjs` |
| `example_qwik_router_inline` | `../node_modules/@qwik.dev/router/index.qwik.mjs` | `../node_modules/@qwik.dev/router/index.qwik.mjs` |
| `example_renamed_exports` | `test.tsx_App_Component_1_A08tXHb9pEk.js` | `test.tsx_s__Ma8uJBKLX2o.js` |
| `example_use_client_effect` | `test.tsx_Child_component_useBrowserVisibleTask_0IGFPOyJmQA.js` | `test.tsx_useBrowserVisibleTask_0qCMfawldvU.js` |
| `should_convert_rest_props` | `test.tsx_test_component_useTask_jewzFYh3XmQ.js` | `test.tsx_default_useTask_cu78f7005P4.js` |
| `should_mark_props_as_var_props_for_inner_cmp` | `test.tsx_ModelImg_component_imgLoc_useResource_Ogi9hEJvtmI.js` | `test.tsx_imgLoc_useResource_bT0HZe0FfwQ.js` |
| `should_not_generate_conflicting_props_identifiers` | `test.js` | `test.js` |
| `should_transform_qrls_in_ternary_expression` | `test.tsx_FieldInput_component_input_q_e_input_wqR1xEjZjf4.js` | `test.tsx_s_1_axfrT8CMT4s.js` |
| `should_wrap_prop_from_destructured_array` | `test.tsx_Input_component_useTask_Sbgs9Wtfkt0.js` | `test.tsx_Input_useTask_onHnlG6lktY.js` |
| `ternary_prop` | `test.tsx_Cmp_component_handleClick_WawHV3HwS1A.ts` | `test.tsx_handleClick$_RtcZLg0Jc1I.ts` |

#### Pattern: extra-captures (7 deviations, cosmetic)

The actual code includes `_captures` or `useLexicalScope` restoration that the expected code doesn't have.

**Why cosmetic:** Extra capture restoration adds unnecessary closure variable loading but doesn't break execution. The variables are loaded but simply unused. The JavaScript runtime handles unused variables gracefully.

| Spec | Actual Module |
|------|---------------|
| `example_capturing_fn_class` | `test.tsx_App_w3unmdi0sm4.js` |
| `example_exports` | `project/test.tsx_Header_0hncTFTaevI.jsx` |
| `example_invalid_segment_expr1` | `test.tsx_useStyles_zs0gpM06Gq4.js` |
| `example_invalid_segment_expr1` | `test.tsx_s__Ma8uJBKLX2o.js` |
| `example_reg_ctx_name_segments` | `test.js` |
| `example_reg_ctx_name_segments_hoisted` | `test.js` |
| `example_reg_ctx_name_segments_inlined` | `test.js` |

---

## Runtime-Breaking Deviations

**Total: 293 deviations across 140 specs**

Grouped by category. Each deviation would cause different runtime behavior than expected.

### Category: module

**83 deviations** -- Missing modules break lazy loading. When a QRL references an import that doesn't exist, the dynamic `import()` fails at runtime.

#### optimizer-failure (3 deviations)

These specs produced zero output modules. The optimizer failed to generate any segments.

| Spec | Expected Modules | Detail |
|------|-----------------|--------|
| `example_3` | 3 expected | Module count: expected 3, got 0. Expected paths: ["test.tsx_App_Header_component |
| `example_component_with_event_listeners_inside_loop` | 8 expected | Module count: expected 8, got 0. Expected paths: ["test.js", "test.tsx_App_compo |
| `example_immutable_analysis` | 6 expected | Module count: expected 6, got 0. Expected paths: ["test.tsx_App_component_Fragme |

#### module-count (2 deviations)

These specs produced some modules but not all expected ones.

| Spec | Expected | Actual | Missing |
|------|----------|--------|---------|
| `example_qwik_react` | 3 | 1 | 2 segments |
| `relative_paths` | 5 | 3 | 2 segments |

#### missing-module (78 deviations)

Individual expected modules that have no counterpart in actual output.

| Spec | Missing Modules | Count |
|------|----------------|-------|
| `example_1` | `test.tsx_renderHeader_component_U6Kkv07sbpQ.tsx`, `test.tsx_renderHeader_div_onClick_fV2uzAL99u4.tsx` | 2 |
| `example_3` | `test.tsx_App_Header_component_B9F3YeqcO1w.tsx`, `test.tsx_App_Header_component_div_onClick_aO7uI7Iw6oQ.tsx` +1 more | 3 |
| `example_capture_imports` | `test.tsx_App_component_useStyles_t35nSa5UV7U.js`, `test.tsx_App_component_useStyles_1_xBK4W0ZKWe8.js` | 2 |
| `example_component_with_event_listeners_inside_loop` | `test.js`, `test.tsx_App_component_ckEPmXZlub0.js` +6 more | 8 |
| `example_custom_inlined_functions` | `test.tsx_App_component_useMemo_6Sc9KVki3Y0.js` | 1 |
| `example_functional_component_2` | `test.tsx_App_component_div_button_q_e_click_UB6Fs5a3bd8.js` | 1 |
| `example_immutable_analysis` | `test.tsx_App_component_Fragment_Div_onEvent_zrFduYbT3xM.js`, `test.js` +4 more | 6 |
| `example_jsx_listeners` | `test.tsx_Foo_component_div_host_onDocumentScroll_Zip7mifsjRY.js`, `test.tsx_Foo_component_div_host_onClick_cPEH970JbEY.js` +6 more | 8 |
| `example_lightweight_functional` | `test.tsx_Button_button_q_e_click_6YaNiKLqRnQ.tsx`, `test.tsx_ButtonArrow_button_q_e_click_rEE0GCaea7M.tsx` | 2 |
| `example_manual_chunks` | `test.tsx_Parent_component_useTask_gDH1EtUWqBU.js`, `test.tsx_Child_component_useTask_Oh4n7ZeqJkU.js` | 2 |
| `example_multi_capture` | `test.tsx_Foo_component_1_DvU6FitWglY.jsx`, `test.tsx_Bar_component_1_0xSyNSnVu3k.jsx` | 2 |
| `example_preserve_filenames` | `test.tsx` | 1 |
| `example_preserve_filenames_segments` | `test.tsx` | 1 |
| `example_prod_node` | `test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.tsx`, `test.tsx_Foo_component_div_div_q_e_click_1_VSoqbTjzr4w.tsx` +1 more | 3 |
| `example_props_wrapping_children` | `test.js (single file, Inline strategy)` | 1 |
| `example_props_wrapping_children2` | `test.js (single file, Inline strategy)` | 1 |
| `example_qwik_conflict` | `test.tsx_Root_component_useStyles_u5DkUxGrGnU.js`, `test.tsx_Root_component_1_cBpQNYDUHI4.js` | 2 |
| `example_qwik_react` | `index.qwik.mjs_qwikifyQrl_component_useWatch_x04JC5xeP1U.mjs`, `index.qwik.mjs_qwikifyQrl_component_zH94hIe0Ick.mjs` | 2 |
| `example_strip_server_code` | `test.tsx_Parent_component_useTask_gDH1EtUWqBU.js`, `test.tsx_Parent_component_useTask_1_P8oRQhHsurk.js` | 2 |
| `example_use_server_mount` | `test.tsx_Parent_component_useTask_gDH1EtUWqBU.js`, `test.tsx_Child_component_useTask_Oh4n7ZeqJkU.js` | 2 |
| `issue_150` | `test.tsx_Greeter_component_1_krCndSwhX4U.js` | 1 |
| `relative_paths` | `../../node_modules/dep/dist/lib.mjs_App_component_div_p_button_onClick_8dWUa0cJAr4.js`, `components/main.tsx_Local_component_jJ0v28bs0p8.js` | 2 |
| `should_convert_jsx_events` | `test.tsx_ManyEventsComponent_component_div_button_q_e_click_1_BOulU3QpiyA.js`, `test.tsx_ManyEventsComponent_component_div_button_q_e_click_z0X12CPQocg.js` | 2 |
| `should_extract_single_qrl` | `test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48.js`, `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` | 2 |
| `should_extract_single_qrl_2` | `test.tsx_Parent_component_div_button_q_e_click_5khsVRINUws.js`, `test.tsx_Parent_component_div_button_q_e_click_1_rAeuW6OvuXM.js` | 2 |
| `should_extract_single_qrl_with_index` | `test.tsx_App_component_div_tr_td_a_q_e_click_1_40fnSAlYI48.js`, `test.tsx_App_component_div_tr_td_a_q_e_click_lgbZkJXyLtg.js` | 2 |
| `should_extract_single_qrl_with_nested_components` | `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` | 1 |
| `should_not_transform_events_on_non_elements` | `test.tsx_Greeter_component_div_AnotherComponent_onClick_9BwXJW3s0yA.tsx`, `test.tsx_Greeter_component_div_CustomComponent_onClick_6xTF8kMcS9w.tsx` | 2 |
| `should_transform_component_with_normal_function` | `test.tsx_Foo_component_Inner_component_AuJ9mTBx5YA.js` | 1 |
| `should_transform_event_names_without_jsx_transpile` | `test.tsx_Greeter_component_div_div_q_e_click_1_s7p0zjWZpqo.tsx`, `test.tsx_Greeter_component_div_div_q_e_click_wYSPnQEGCbA.tsx` +1 more | 3 |
| `should_transform_multiple_event_handlers` | `test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.js`, `test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk.js` | 2 |
| `should_transform_multiple_event_handlers_case2` | `test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.js`, `test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk.js` | 2 |
| `should_transform_nested_loops` | `test.tsx_Foo_component_div_div_q_e_click_vKrX4PmH2aM.js`, `test.tsx_Foo_component_div_div_p_q_e_click_PjMbeUzoAMk.js` | 2 |
| `should_wrap_prop_from_destructured_array` | `test.tsx_test_component_LUXeXe0DQrg.js` | 1 |
| `support_windows_paths` | `components/apps/apps.ts` | 1 |

### Category: qrl

**54 deviations** -- QRL extraction failures. Event handlers should be extracted into separate segments and wrapped with `qrl()` for lazy loading. When this doesn't happen, the event handler code is inlined in the parent module, defeating code splitting.

#### qrl-extraction (45 deviations)

The expected code uses `qrl(import_fn, 'segment_name')` to reference a lazy-loaded segment, but the actual code has the event handler inlined as an arrow function. This means the handler code loads eagerly with the parent module instead of being deferred.

| Spec | Affected Modules | Impact |
|------|-----------------|--------|
| `example_manual_chunks` | 2 | Event handler(s) not extracted to lazy segment |
| `example_qwik_conflict` | 2 | Event handler(s) not extracted to lazy segment |
| `example_use_server_mount` | 2 | Event handler(s) not extracted to lazy segment |
| `destructure_args_inline_cmp_block_stmt` | 1 | Event handler(s) not extracted to lazy segment |
| `destructure_args_inline_cmp_block_stmt2` | 1 | Event handler(s) not extracted to lazy segment |
| `destructure_args_inline_cmp_expr_stmt` | 1 | Event handler(s) not extracted to lazy segment |
| `example_default_export` | 1 | Event handler(s) not extracted to lazy segment |
| `example_default_export_invalid_ident` | 1 | Event handler(s) not extracted to lazy segment |
| `example_dev_mode` | 1 | Event handler(s) not extracted to lazy segment |
| `example_drop_side_effects` | 1 | Event handler(s) not extracted to lazy segment |
| `example_functional_component_2` | 1 | Event handler(s) not extracted to lazy segment |
| `example_functional_component_capture_props` | 1 | Event handler(s) not extracted to lazy segment |
| `example_jsx_keyed_dev` | 1 | Event handler(s) not extracted to lazy segment |
| `example_lightweight_functional` | 1 | Event handler(s) not extracted to lazy segment |
| `example_noop_dev_mode` | 1 | Event handler(s) not extracted to lazy segment |
| `example_preserve_filenames_segments` | 1 | Event handler(s) not extracted to lazy segment |
| `example_prod_node` | 1 | Event handler(s) not extracted to lazy segment |
| `example_strip_server_code` | 1 | Event handler(s) not extracted to lazy segment |
| `example_transpile_jsx_only` | 1 | Event handler(s) not extracted to lazy segment |
| `impure_template_fns` | 1 | Event handler(s) not extracted to lazy segment |
| `issue_5008` | 1 | Event handler(s) not extracted to lazy segment |
| `issue_7216_add_test` | 1 | Event handler(s) not extracted to lazy segment |
| `lib_mode_fn_signal` | 1 | Event handler(s) not extracted to lazy segment |
| `relative_paths` | 1 | Event handler(s) not extracted to lazy segment |
| `should_convert_jsx_events` | 1 | Event handler(s) not extracted to lazy segment |
| `should_extract_single_qrl` | 1 | Event handler(s) not extracted to lazy segment |
| `should_extract_single_qrl_2` | 1 | Event handler(s) not extracted to lazy segment |
| `should_extract_single_qrl_with_index` | 1 | Event handler(s) not extracted to lazy segment |
| `should_handle_dangerously_set_inner_html` | 1 | Event handler(s) not extracted to lazy segment |
| `should_merge_bind_checked_and_on_input` | 1 | Event handler(s) not extracted to lazy segment |
| `should_merge_bind_value_and_on_input` | 1 | Event handler(s) not extracted to lazy segment |
| `should_merge_on_input_and_bind_checked` | 1 | Event handler(s) not extracted to lazy segment |
| `should_merge_on_input_and_bind_value` | 1 | Event handler(s) not extracted to lazy segment |
| `should_move_bind_value_to_var_props` | 1 | Event handler(s) not extracted to lazy segment |
| `should_not_transform_events_on_non_elements` | 1 | Event handler(s) not extracted to lazy segment |
| `should_not_wrap_fn` | 1 | Event handler(s) not extracted to lazy segment |
| `should_split_spread_props_with_additional_prop4` | 1 | Event handler(s) not extracted to lazy segment |
| `should_transform_event_names_without_jsx_transpile` | 1 | Event handler(s) not extracted to lazy segment |
| `should_transform_multiple_event_handlers` | 1 | Event handler(s) not extracted to lazy segment |
| `should_transform_multiple_event_handlers_case2` | 1 | Event handler(s) not extracted to lazy segment |
| `should_transform_nested_loops` | 1 | Event handler(s) not extracted to lazy segment |
| `should_transform_qrls_in_ternary_expression` | 1 | Event handler(s) not extracted to lazy segment |

#### inlinedqrl-diff (9 deviations)

The inlined QRL count differs between expected and actual. This affects how segments are bundled when using the `inline` entry strategy.

| Spec | Module | Detail |
|------|--------|--------|
| `example_default_export_index` | `src/components/mongo/index.tsx` | inlinedQrl count: expected 2, actual 1 |
| `example_dev_mode_inlined` | `test.js` | missing imports: 1; extra imports: 2; inlinedQrl count: expe |
| `example_inlined_entry_strategy` | `test.tsx` | const/let/var diff: expected {"const": 3}, actual {"const":  |
| `example_issue_33443` | `test.js` | const/let/var diff: expected {"const": 7}, actual {"const":  |
| `example_optimization_issue_3542` | `test.jsx` | const/let/var diff: expected {"let": 1, "const": 2}, actual  |
| `example_qwik_react` | `../node_modules/@qwik.dev/react/index.qwik.mjs` | const/let/var diff: expected {"const": 3}, actual {"const":  |
| `example_strip_client_code` | `components/component.js` | const/let/var diff: expected {"const": 3}, actual {"const":  |
| `example_transpile_ts_only` | `test.jsx` | extra imports: 1; inlinedQrl count: expected 2, actual 1 |
| `relative_paths` | `../../node_modules/dep/dist/lib.mjs` | const/let/var diff: expected {"const": 3}, actual {"const":  |

### Category: import

**52 deviations** -- Missing imports mean required symbols are not available at runtime. The module will fail with `ReferenceError` or `TypeError` when trying to use an unimported function or value.

#### missing-imports (39 deviations)

Modules that are missing one or more `import` statements compared to expected output.

| Spec | Affected Modules | Pattern |
|------|-----------------|---------|
| `example_11` | 3 | Missing required imports |
| `example_invalid_references` | 2 | Missing required imports |
| `example_server_auth` | 2 | Missing required imports |
| `example_7` | 1 | Missing required imports |
| `example_build_server` | 1 | Missing required imports |
| `example_export_issue` | 1 | Missing required imports |
| `example_fix_dynamic_import` | 1 | Missing required imports |
| `example_functional_component_capture_props` | 1 | Missing required imports |
| `example_getter_generation` | 1 | Missing required imports |
| `example_import_assertion` | 1 | Missing required imports |
| `example_lightweight_functional` | 1 | Missing required imports |
| `example_renamed_exports` | 1 | Missing required imports |
| `example_spread_jsx` | 1 | Missing required imports |
| `example_strip_exports_used` | 1 | Missing required imports |
| `example_ts_enums_no_transpile` | 1 | Missing required imports |
| `example_use_client_effect` | 1 | Missing required imports |
| `rename_builder_io` | 1 | Missing required imports |
| `should_convert_rest_props` | 1 | Missing required imports |
| `should_destructure_args` | 1 | Missing required imports |
| `should_extract_single_qrl_with_nested_components` | 1 | Missing required imports |
| `should_mark_props_as_var_props_for_inner_cmp` | 1 | Missing required imports |
| `should_move_props_related_to_iteration_variables_to_var_props` | 1 | Missing required imports |
| `should_not_transform_bind_checked_in_var_props_for_jsx_split` | 1 | Missing required imports |
| `should_not_transform_bind_value_in_var_props_for_jsx_split` | 1 | Missing required imports |
| `should_not_wrap_ternary_function_operator_with_fn` | 1 | Missing required imports |
| `should_not_wrap_var_template_string` | 1 | Missing required imports |
| `should_split_spread_props` | 1 | Missing required imports |
| `should_split_spread_props_with_additional_prop` | 1 | Missing required imports |
| `should_split_spread_props_with_additional_prop2` | 1 | Missing required imports |
| `should_split_spread_props_with_additional_prop3` | 1 | Missing required imports |
| `should_split_spread_props_with_additional_prop5` | 1 | Missing required imports |
| `should_transform_component_with_normal_function` | 1 | Missing required imports |
| `should_wrap_inner_inline_component_prop` | 1 | Missing required imports |
| `should_wrap_type_asserted_variables_in_template` | 1 | Missing required imports |
| `transform_qrl_in_regular_prop` | 1 | Missing required imports |

#### import-mismatch (13 deviations)

Modules where some imports are missing AND other unexpected imports are present.

| Spec | Module | Detail |
|------|--------|--------|
| `example_custom_inlined_functions` | `test.tsx_App_component_XumqEURaLxI.js` | missing imports: 2; extra imports: 1 |
| `example_derived_signals_complext_children` | `test.js` | missing imports: 1; extra imports: 3 |
| `example_dev_mode` | `test.tsx_App_component_XumqEURaLxI.js` | missing imports: 1; extra imports: 1 |
| `example_drop_side_effects` | `test.tsx_default_component_jN9ZUuuADmk.js` | missing imports: 1; extra imports: 1 |
| `example_functional_component` | `test.tsx` | missing imports: 1; extra imports: 1 |
| `example_jsx_import_source` | `test.js` | missing imports: 2; extra imports: 3 |
| `example_jsx_import_source` | `test.tsx_App2_qwikify_4Tc4MzcFY0U.js` | missing imports: 1; extra imports: 1 |
| `example_missing_custom_inlined_functions` | `test.js` | missing imports: 2; extra imports: 3 |
| `example_noop_dev_mode` | `test.tsx_App_component_XumqEURaLxI.js` | missing imports: 2; extra imports: 1 |
| `example_props_wrapping` | `test.js` | missing imports: 1; extra imports: 1 |
| `example_server_auth` | `test.js` | missing imports: 2; extra imports: 6 |
| `rename_builder_io` | `test.ts` | missing imports: 1; extra imports: 5 |
| `special_jsx` | `test.tsx` | missing imports: 1; extra imports: 1 |

### Category: codegen

**77 deviations** -- Code generation differences that affect runtime behavior. These include missing code, structural differences, and variable declaration mismatches.

#### empty-segment (32 deviations)

Modules that were structurally matched but the actual output has an empty body. The module file exists but contains no code.

**Why runtime-breaking:** An empty module means the exported function doesn't exist. Any `import()` of this segment will resolve but the expected export will be `undefined`, causing `TypeError: X is not a function` at runtime.

| Spec | Empty Segments | Pattern |
|------|---------------|---------|
| `should_convert_jsx_events` | 5 | Segment body missing |
| `example_jsx_listeners` | 4 | Segment body missing |
| `example_manual_chunks` | 2 | Segment body missing |
| `example_use_server_mount` | 2 | Segment body missing |
| `issue_7216_add_test` | 2 | Segment body missing |
| `example_default_export` | 1 | Segment body missing |
| `example_default_export_invalid_ident` | 1 | Segment body missing |
| `example_dev_mode` | 1 | Segment body missing |
| `example_drop_side_effects` | 1 | Segment body missing |
| `example_preserve_filenames_segments` | 1 | Segment body missing |
| `example_qwik_conflict` | 1 | Segment body missing |
| `example_strip_server_code` | 1 | Segment body missing |
| `example_transpile_jsx_only` | 1 | Segment body missing |
| `should_extract_single_qrl_with_nested_components` | 1 | Segment body missing |
| `should_merge_bind_checked_and_on_input` | 1 | Segment body missing |
| `should_merge_bind_value_and_on_input` | 1 | Segment body missing |
| `should_merge_on_input_and_bind_checked` | 1 | Segment body missing |
| `should_merge_on_input_and_bind_value` | 1 | Segment body missing |
| `should_move_bind_value_to_var_props` | 1 | Segment body missing |
| `should_transform_component_with_normal_function` | 1 | Segment body missing |
| `should_transform_multiple_event_handlers` | 1 | Segment body missing |
| `should_transform_multiple_event_handlers_case2` | 1 | Segment body missing |

#### codegen-missing-imports (24 deviations)

Modules with both code structure differences AND missing imports. The code differs in variable declarations or expressions, and required imports are absent.

| Spec | Affected Modules |
|------|-----------------|
| `destructure_args_colon_props` | 1 |
| `destructure_args_colon_props2` | 1 |
| `destructure_args_colon_props3` | 1 |
| `example_1` | 1 |
| `example_8` | 1 |
| `example_custom_inlined_functions` | 1 |
| `example_derived_signals_children` | 1 |
| `example_derived_signals_cmp` | 1 |
| `example_derived_signals_div` | 1 |
| `example_derived_signals_multiple_children` | 1 |
| `example_functional_component` | 1 |
| `example_functional_component_2` | 1 |
| `example_getter_generation` | 1 |
| `example_immutable_function_components` | 1 |
| `example_invalid_segment_expr1` | 1 |
| `example_issue_4438` | 1 |
| `example_mutable_children` | 1 |
| `example_props_wrapping2` | 1 |
| `should_ignore_null_inlined_qrl` | 1 |
| `should_merge_attributes_with_spread_props` | 1 |
| `should_merge_attributes_with_spread_props_before_and_after` | 1 |
| `should_wrap_logical_expression_in_template` | 1 |
| `should_wrap_store_expression` | 1 |
| `ternary_prop` | 1 |

#### const-let-var (20 deviations)

Pure variable declaration count/type differences. The actual output has a different number of `const`, `let`, or `var` declarations than expected.

**Why runtime-breaking:** `const` vs `let` affects reassignability. Extra or missing `const` declarations indicate structural code differences (e.g., additional intermediate variables, missing destructuring). These change program semantics even if the high-level behavior might seem similar.

| Spec | Affected Modules |
|------|-----------------|
| `example_10` | 2 |
| `example_9` | 2 |
| `example_multi_capture` | 2 |
| `example_2` | 1 |
| `example_4` | 1 |
| `example_5` | 1 |
| `example_7` | 1 |
| `example_8` | 1 |
| `example_default_export_invalid_ident` | 1 |
| `example_explicit_ext_no_transpile` | 1 |
| `example_exports` | 1 |
| `example_optimization_issue_4386` | 1 |
| `example_prod_node` | 1 |
| `example_with_style` | 1 |
| `example_with_tagname` | 1 |
| `hoisted_fn_signal_in_loop` | 1 |
| `should_wrap_object_with_fn_signal` | 1 |

#### codegen-extra-imports (1 deviations)

Code generation difference with extra imports and structural code changes.

- `should_wrap_prop_from_destructured_array`: const/let/var diff: expected {"const": 2}, actual {"const": 12}; extra imports: 

### Category: capture

**27 deviations** -- Capture analysis determines which variables from the parent scope need to be serialized and restored in lazy-loaded segments. Incorrect captures cause runtime errors or wrong data.

See the **Capture Deviations** section above for detailed breakdown by pattern (empty-segment-capture, capture-diff).

---

## Cosmetic Deviations

**Total: 206 deviations** -- These do NOT affect runtime behavior. The code executes identically despite surface-level differences.

| Category | Pattern | Count | Explanation |
|----------|---------|-------|-------------|
| module | extra-module | 57 | Extra segments in output are valid JS but not referenced by QRLs in expected code. They exist but ar |
| import | entry-extra-imports | 54 | Entry module hoists imports from child segments. Tree shaking removes them in production builds. No  |
| codegen | hash-diff | 34 | Different hash algorithm produces different identifier suffixes (e.g., `_jN9ZUuuADmk` vs `_LUXeXe0DQ |
| import | extra-imports | 32 | Module includes imports not in expected output. Unused imports are tree-shaken in production. |
| codegen | naming-convention | 21 | Segment names use different convention (e.g., `default_component` vs `test_component`). Both sides a |
| capture | extra-captures | 7 | Actual code loads extra closure variables via `_captures`/`useLexicalScope`. The extra variables are |
| codegen | token-diff | 1 | Token-level difference in non-hash identifiers that are internally consistent. |

### Cosmetic-Only Specs (21 specs)

These specs have deviations but ALL are cosmetic -- the optimizer output would produce identical runtime behavior:

- `example_6`: hash-diff(2)
- `example_capturing_fn_class`: entry-extra-imports(1), hash-diff(1), extra-captures(1)
- `example_class_name`: extra-imports(1), hash-diff(1)
- `example_dead_code`: extra-imports(1), hash-diff(1)
- `example_explicit_ext_transpile`: naming-convention(2), entry-extra-imports(1), hash-diff(1)
- `example_input_bind`: naming-convention(1)
- `example_jsx_keyed`: extra-imports(1), hash-diff(1)
- `example_of_synchronous_qrl`: extra-imports(1), naming-convention(1)
- `example_optimization_issue_3561`: naming-convention(1)
- `example_optimization_issue_3795`: naming-convention(1)
- `example_reg_ctx_name_segments`: extra-captures(1)
- `example_reg_ctx_name_segments_hoisted`: extra-captures(1)
- `example_reg_ctx_name_segments_inlined`: extra-captures(1)
- `example_skip_transform`: token-diff(1)
- `example_strip_exports_unused`: extra-imports(1), naming-convention(1)
- `example_ts_enums`: entry-extra-imports(1), hash-diff(1)
- `example_ts_enums_issue_1341`: extra-imports(1), hash-diff(1)
- `example_use_optimization`: entry-extra-imports(1)
- `issue_476`: naming-convention(1)
- `issue_964`: extra-imports(1), hash-diff(1)
- `should_not_move_over_side_effects`: naming-convention(1)

---

## Full Deviation Details

### High-Impact Specs (10+ runtime-breaking deviations)

#### `example_jsx_listeners` (12 runtime-breaking)

- **Categories:** module(8), codegen(4)
- **Patterns:** missing-module(8), empty-segment(4)

#### `example_component_with_event_listeners_inside_loop` (9 runtime-breaking)

- **Categories:** module(9)
- **Patterns:** missing-module(8), optimizer-failure(1)

#### `should_convert_jsx_events` (8 runtime-breaking)

- **Categories:** codegen(5), module(2), qrl(1)
- **Patterns:** empty-segment(5), missing-module(2), qrl-extraction(1)

#### `example_immutable_analysis` (7 runtime-breaking)

- **Categories:** module(7)
- **Patterns:** missing-module(6), optimizer-failure(1)

#### `example_manual_chunks` (6 runtime-breaking)

- **Categories:** module(2), qrl(2), codegen(2)
- **Patterns:** missing-module(2), qrl-extraction(2), empty-segment(2)

#### `example_use_server_mount` (6 runtime-breaking)

- **Categories:** module(2), qrl(2), codegen(2)
- **Patterns:** missing-module(2), qrl-extraction(2), empty-segment(2)

#### `example_prod_node` (5 runtime-breaking)

- **Categories:** module(3), codegen(1), qrl(1)
- **Patterns:** missing-module(3), const-let-var(1), qrl-extraction(1)

#### `example_qwik_conflict` (5 runtime-breaking)

- **Categories:** module(2), qrl(2), codegen(1)
- **Patterns:** missing-module(2), qrl-extraction(2), empty-segment(1)

#### `relative_paths` (5 runtime-breaking)

- **Categories:** module(3), qrl(2)
- **Patterns:** missing-module(2), module-count(1), qrl-extraction(1), inlinedqrl-diff(1)

### Common Deviation Patterns (with examples)

#### Pattern: qrl-extraction

Event handlers remain inline instead of being extracted to separate lazy-loaded segments.

**Spec: `destructure_args_inline_cmp_block_stmt`** (module: `test.js`)

Expected (uses `qrl()` for lazy loading):
```js
import { _fnSignal } from "@qwik.dev/core"; import { _jsxSorted } from "@qwik.dev/core"; import { qrl } from "@qwik.dev/core"; const _hf0 = (p0)=>p0.data.selectedOutputDetail === 'options'; const _hf0_str = 'p0.data.selectedOutputDetail==="options"'; const i_pFqTss400MA = ()=>import("./test.tsx_test...
```

Actual (inline handler, no lazy loading):
```js
import { _jsxSorted } from "@qwik.dev/core"; import { qrl } from "@qwik.dev/core"; const i_Lo4b850Oi8I = () => import("./test.tsx_div_q_e_click_Lo4b850Oi8I"); export default ({ data }: { data: any; }) => { return /* @__PURE__ */ _jsxSorted("div", { data-is-active: data.selectedOutputDetail === "opti...
```

**Spec: `destructure_args_inline_cmp_block_stmt2`** (module: `test.js`)

Expected (uses `qrl()` for lazy loading):
```js
import { _fnSignal } from "@qwik.dev/core"; import { _jsxSorted } from "@qwik.dev/core"; import { qrl } from "@qwik.dev/core"; const _hf0 = (p0)=>p0.data.selectedOutputDetail === 'options'; const _hf0_str = 'p0.data.selectedOutputDetail==="options"'; const i_pFqTss400MA = ()=>import("./test.tsx_test...
```

Actual (inline handler, no lazy loading):
```js
import { _jsxSorted } from "@qwik.dev/core"; import { qrl } from "@qwik.dev/core"; const i_Lo4b850Oi8I = () => import("./test.tsx_div_q_e_click_Lo4b850Oi8I"); export default (props: { data: any; }) => { const { data } = props; return /* @__PURE__ */ _jsxSorted("div", { data-is-active: data.selectedO...
```

#### Pattern: empty-segment

Segments match structurally but the actual module body is completely empty.

**Spec: `example_default_export`** (expected module: `[[...slug]].tsx_slug_component_div_q_e_click_bCwVPYSTQ0w.js`)

Expected code:
```js
import { sibling } from "./sibling"; export const slug_component_div_q_e_click_bCwVPYSTQ0w = ()=>console.log(mongodb, sibling);
```

Actual: *(empty)*

**Spec: `example_default_export_invalid_ident`** (expected module: `src/components/mongo/404.tsx__404_component_div_q_e_click_aMLnLWtkRhc.tsx`)

Expected code:
```js
export const _404_component_div_q_e_click_aMLnLWtkRhc = ()=>console.log(mongodb);
```

Actual: *(empty)*

#### Pattern: missing-imports

Required imports are absent from the module.

**Spec: `example_11`** (module: `project/test.tsx_Header_0hncTFTaevI.tsx`)

Expected imports: 1
  `import dep3 from "dep3/something";`
Actual imports: 0

**Spec: `example_11`** (module: `project/test.tsx_Header_component_sHTSaq0AU4w.tsx`)

Expected imports: 4
  `import * as dep2 from "dep2";`
  `import { Header } from "./test";`
  `import { bar as bbar } from "../state";`
  `import { qrl } from "@qwik.dev/core";`
Actual imports: 1
  `import { qrl } from "@qwik.dev/core";`

#### Pattern: entry-extra-imports (cosmetic)

Entry modules in the actual output include imports from child segments that the expected output omits. This is a tree-shaking difference -- the extra imports are for symbols used in child segments, hoisted to the entry module. Production builds would tree-shake these away.

**Spec: `example_11`** (module: `project/test.tsx`)

Expected imports: 2 | Actual imports: 5 (extra: 3)

---

## Summary for Phase 24

### What Needs Fixing (Runtime-Breaking)

Phase 24 should prioritize these categories of fixes:

1. **Module generation failures** (83 deviations): 3 specs produce zero output, 2 produce partial output, and 78 individual expected modules are missing from actual output. These are the most severe -- entire specs or segments don't work at all.

2. **QRL extraction** (54 deviations): Event handlers are not being extracted to separate lazy-loaded segments. Instead they remain as inline arrow functions. This defeats Qwik's code splitting for interactivity.

3. **Code generation** (77 deviations): Empty segment bodies (32), missing imports with code diffs (24), and variable declaration differences (20). These indicate the code generation pipeline is producing structurally different output.

4. **Import resolution** (52 deviations): Required imports missing from modules. Without the right imports, the code crashes on first use of the missing symbol.

5. **Capture analysis** (27 deviations): Closure variables not properly captured and restored in lazy segments. This is critical for Qwik's resumability model.

### What Can Be Deferred (Cosmetic)

The 206 cosmetic deviations across 21 cosmetic-only specs (plus cosmetic deviations mixed with runtime-breaking in other specs) do not need immediate fixes:

- **Extra imports in entry modules** (54): Tree-shakeable, no runtime impact
- **Extra modules** (57): Unused segments, no runtime impact
- **Hash differences** (34): Different hash algorithm, self-consistent
- **Extra imports in segments** (32): Unused, tree-shakeable
- **Naming conventions** (21): Different segment names, self-consistent
- **Extra captures** (7): Unnecessary but harmless capture restoration

---

*Report generated from audit-raw.json (499 deviations across 161/162 specs)*
*Classification criteria from Phase 23 context decisions (locked)*