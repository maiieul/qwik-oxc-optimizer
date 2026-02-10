use oxc::allocator::Allocator;
use oxc::parser::{Parser, ParserReturn};
use oxc::semantic::{Scoping, SemanticBuilder};
use oxc::span::SourceType;

/// Parse source code into an AST, returning the ParserReturn.
pub fn parse_source<'a>(
    allocator: &'a Allocator,
    source: &'a str,
    source_type: SourceType,
) -> ParserReturn<'a> {
    Parser::new(allocator, source, source_type).parse()
}

/// Build Scoping from a parsed program using SemanticBuilder.
pub fn build_scoping(program: &oxc::ast::ast::Program<'_>) -> Scoping {
    let semantic_ret = SemanticBuilder::new()
        .with_excess_capacity(2.0)
        .build(program);
    semantic_ret.semantic.into_scoping()
}
