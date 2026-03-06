//! PPTX format converter.
//!
//! This module handles conversion of Microsoft PowerPoint (.pptx) files to PDF.
//!
//! TODO: Implementation details:
//! - Parse PPTX structure (XML-based format)
//! - Extract slides, images, text, and animations
//! - Convert each slide to a PDF page
//! - Handle transitions and animations (flatten to static pages)

use super::Converter;
use crate::config::ConversionOptions;
use crate::error::Result;
use std::path::{Path, PathBuf};

/// Converter for PPTX files.
pub struct PptxConverter {
    // TODO: Add fields for converter state/configuration
}

impl PptxConverter {
    /// Create a new PPTX converter.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PptxConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter for PptxConverter {
    fn convert(
        &self,
        _input: &Path,
        _output: &Path,
        _options: &ConversionOptions,
    ) -> Result<PathBuf> {
        // TODO: Implement PPTX to PDF conversion
        todo!("PPTX conversion not yet implemented")
    }

    fn format_name(&self) -> &'static str {
        "pptx"
    }
}
