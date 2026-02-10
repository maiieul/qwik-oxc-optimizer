use std::io::Read;

use oxc::allocator::Allocator;
use oxc::ast_visit::utf8_to_utf16::Utf8ToUtf16;
use oxc::parser::{ParseOptions, Parser};
use oxc::span::SourceType;

fn main() {
    // Get file extension from args
    let ext = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: oxc-ast-util <extension>");
        eprintln!("  Reads source code from stdin, outputs ESTree JSON AST to stdout.");
        eprintln!("  Extensions: js, jsx, ts, tsx, mjs, cjs, mts, cts");
        std::process::exit(1);
    });

    // Read code from stdin
    let mut code = String::new();
    std::io::stdin().read_to_string(&mut code).unwrap_or_else(|e| {
        eprintln!("Failed to read stdin: {e}");
        std::process::exit(1);
    });

    // Determine source type from extension
    let fake_filename = format!("input.{ext}");
    let source_type = SourceType::from_path(&fake_filename).unwrap_or_else(|_| {
        eprintln!("Unknown extension: {ext}");
        std::process::exit(1);
    });

    // Parse
    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, &code, source_type)
        .with_options(ParseOptions {
            parse_regular_expression: true,
            ..ParseOptions::default()
        })
        .parse();

    // Report errors to stderr (but still emit AST)
    if ret.panicked {
        eprintln!("FATAL: Parser panicked on input");
        std::process::exit(2);
    }

    if !ret.errors.is_empty() {
        eprintln!("Parse errors ({}):", ret.errors.len());
        for error in &ret.errors {
            let error = error.clone().with_source_code(code.clone());
            eprintln!("  {error:?}");
        }
    }

    // Convert spans to UTF-16 for ESTree compliance
    let mut program = ret.program;
    Utf8ToUtf16::new(&code).convert_program(&mut program);

    // Serialize to ESTree JSON
    let json = if source_type.is_javascript() {
        program.to_pretty_estree_js_json(false)
    } else {
        program.to_pretty_estree_ts_json(false)
    };

    println!("{json}");
}
