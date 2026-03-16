//! Markdown format converter.
//!
//! This module handles conversion of Markdown (.md) files to PDF.
//!
//! TODO: Implementation details:
//! - Parse Markdown syntax (headers, lists, code blocks, etc.)
//! - Support GitHub Flavored Markdown (tables, task lists, etc.)
//! - Apply syntax highlighting for code blocks
//! - Generate PDF with proper typography

use super::Converter;
use crate::config::ConversionOptions;
use crate::error::Result;
use std::path::{Path, PathBuf};

/// Converter for Markdown files.
pub struct MdConverter {
    // TODO: Add fields for converter state/configuration
}

impl MdConverter {
    /// Create a new Markdown converter.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for MdConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter for MdConverter {
    fn convert(
        &self,
        _input: &Path,
        _output: &Path,
        _options: &ConversionOptions,
    ) -> Result<PathBuf> {
        // TODO: Implement Markdown to PDF conversion
        todo!("Markdown conversion not yet implemented")
    }

    fn format_name(&self) -> &'static str {
        "md"
    }
}
