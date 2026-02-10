mod common;

use common::{build_scoping, parse_source};
use oxc::allocator::Allocator;
use oxc::ast::ast::*;
use oxc::span::SourceType;
use oxc_traverse::{Traverse, TraverseCtx, traverse_mut};
use std::collections::HashSet;

/// Represents a detected $() call site in the source code.
#[derive(Debug)]
struct DollarCallSite {
    callee: String,
    span_start: u32,
    span_end: u32,
}

/// Traversal visitor that detects $-suffixed imports from @qwik.dev/core
/// and records $() call sites.
struct DollarDetector {
    /// Names of imported $-suffixed identifiers from @qwik.dev/core
    dollar_imports: HashSet<String>,
    /// Detected call sites
    found_sites: Vec<DollarCallSite>,
}

impl<'a> Traverse<'a, ()> for DollarDetector {
    fn enter_import_declaration(
        &mut self,
        import: &mut ImportDeclaration<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if import.source.value.as_str() != "@qwik.dev/core" {
            return;
        }
        if let Some(specifiers) = &import.specifiers {
            for spec in specifiers {
                if let ImportDeclarationSpecifier::ImportSpecifier(s) = spec {
                    let name = match &s.imported {
                        ModuleExportName::IdentifierName(id) => id.name.as_str(),
                        ModuleExportName::IdentifierReference(id) => id.name.as_str(),
                        ModuleExportName::StringLiteral(s) => s.value.as_str(),
                    };
                    if name == "$" || name.ends_with('$') {
                        self.dollar_imports.insert(name.to_string());
                    }
                }
            }
        }
    }

    fn enter_call_expression(
        &mut self,
        call: &mut CallExpression<'a>,
        _ctx: &mut TraverseCtx<'a, ()>,
    ) {
        if let Expression::Identifier(ident) = &call.callee {
            let name = ident.name.as_str();
            if self.dollar_imports.contains(name) {
                self.found_sites.push(DollarCallSite {
                    callee: name.to_string(),
                    span_start: call.span.start,
                    span_end: call.span.end,
                });
            }
        }
    }
}

/// Run dollar detection on a source string and print results.
fn detect_dollar_calls(label: &str, source: &str) -> Vec<DollarCallSite> {
    println!("=== {} ===", label);
    println!("Input ({} bytes):", source.len());

    let allocator = Allocator::default();
    let source_type = SourceType::tsx();
    let ret = parse_source(&allocator, source, source_type);

    if !ret.errors.is_empty() {
        println!("  Parse errors:");
        for err in &ret.errors {
            println!("    {}", err);
        }
    }

    let mut program = ret.program;
    let scoping = build_scoping(&program);

    let mut detector = DollarDetector {
        dollar_imports: HashSet::new(),
        found_sites: Vec::new(),
    };

    let _scoping = traverse_mut(&mut detector, &allocator, &mut program, scoping, ());

    println!(
        "  Dollar imports found: {:?}",
        detector.dollar_imports.iter().collect::<Vec<_>>()
    );
    println!("  Found {} $-call sites:", detector.found_sites.len());
    for site in &detector.found_sites {
        println!(
            "    {} at span {}..{}",
            site.callee, site.span_start, site.span_end
        );
    }
    println!();

    detector.found_sites
}

fn main() {
    println!("POC-01: Dollar Call Site Detection using OXC Traverse");
    println!("=====================================================\n");

    // --- Test Case 1: example_1.md ---
    // Input from spec: has $() and component() wrapping $()
    let source_1 = r#"import { $, component, onRender } from '@qwik.dev/core';

export const renderHeader = $(() => {
	return (
		<div onClick={$((ctx) => console.log(ctx))}/>
	);
});
const renderHeader = component($(() => {
	console.log("mount");
	return render;
}));"#;

    let sites_1 = detect_dollar_calls("Test Case 1: example_1.md", source_1);

    // Verify: should find 3 $() call sites
    // 1. The outer $(() => { return <div ... /> })
    // 2. The inner $((ctx) => console.log(ctx))
    // 3. The $(() => { console.log("mount"); ... }) inside component()
    assert_eq!(
        sites_1.len(),
        3,
        "Expected 3 $-call sites in example_1, found {}",
        sites_1.len()
    );
    for site in &sites_1 {
        assert_eq!(
            site.callee, "$",
            "Expected callee '$', got '{}'",
            site.callee
        );
    }
    println!("  PASS: Found 3 $() call sites with callee '$'\n");

    // --- Test Case 2: component$ pattern ---
    // Input from spec: has component$() call
    let source_2 = r#"import { $, component$ } from '@qwik.dev/core';

export const Foo = component$(({foo}) => {
	const arg0 = 20;
	return $(() => {
		const fn = ({aaa}) => aaa;
		return (
			<div>
				{foo}{fn()}{arg0}
			</div>
		)
	});
})"#;

    let sites_2 = detect_dollar_calls("Test Case 2: example_multi_capture.md", source_2);

    // Verify: should find 2 sites: component$(...) and $(() => { ... })
    assert_eq!(
        sites_2.len(),
        2,
        "Expected 2 $-call sites in example_multi_capture, found {}",
        sites_2.len()
    );

    let callee_names: Vec<&str> = sites_2.iter().map(|s| s.callee.as_str()).collect();
    assert!(
        callee_names.contains(&"component$"),
        "Expected 'component$' callee, got {:?}",
        callee_names
    );
    assert!(
        callee_names.contains(&"$"),
        "Expected '$' callee, got {:?}",
        callee_names
    );
    println!("  PASS: Found component$() and $() call sites\n");

    // --- Test Case 3: Multiple dollar-suffixed imports ---
    let source_3 = r#"import { component$, useBrowserVisibleTask$, useStore, useStyles$ } from '@qwik.dev/core';
import { thing } from './sibling';

export const Child = component$(() => {
    useStyles$('somestring');
    const state = useStore({count: 0});
    useBrowserVisibleTask$(() => {
        state.count = thing.doStuff();
    });
    return <div onClick$={() => console.log('click')}/>;
});"#;

    let sites_3 = detect_dollar_calls(
        "Test Case 3: example_inlined_entry_strategy.md",
        source_3,
    );

    // Should find: component$(), useStyles$(), useBrowserVisibleTask$()
    // Note: onClick$ is a JSX attribute, not a call expression with a $-imported callee.
    // The $ in onClick$ is part of the attribute name, not a function call.
    assert!(
        sites_3.len() >= 3,
        "Expected at least 3 $-call sites, found {}",
        sites_3.len()
    );

    let callee_names_3: Vec<&str> = sites_3.iter().map(|s| s.callee.as_str()).collect();
    assert!(
        callee_names_3.contains(&"component$"),
        "Expected 'component$' callee"
    );
    assert!(
        callee_names_3.contains(&"useStyles$"),
        "Expected 'useStyles$' callee"
    );
    assert!(
        callee_names_3.contains(&"useBrowserVisibleTask$"),
        "Expected 'useBrowserVisibleTask$' callee"
    );
    println!("  PASS: Found component$(), useStyles$(), useBrowserVisibleTask$() call sites\n");

    println!("=====================================================");
    println!("All POC-01 tests passed!");
}
