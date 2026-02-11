//! Segment extraction.
//!
//! After the transform pass identifies segments, this module creates separate
//! `Program` ASTs for each extracted segment. Each segment becomes its own
//! module file containing the extracted function body as an exported const.

#![allow(unused)]

use crate::types::{SegmentAnalysis, SegmentData, TransformOptions};

/// A segment that has been extracted into its own Program AST.
pub(crate) struct SegmentProgram<'a> {
    pub program: oxc::ast::ast::Program<'a>,
    pub path: String,
    pub analysis: SegmentAnalysis,
}

/// Extract segments into standalone module Programs.
pub(crate) fn extract_segments<'a>(
    _allocator: &'a oxc::allocator::Allocator,
    _segments: &[SegmentData],
    _filename: &str,
    _options: &TransformOptions,
) -> Vec<SegmentProgram<'a>> {
    todo!("Implement segment extraction")
}
