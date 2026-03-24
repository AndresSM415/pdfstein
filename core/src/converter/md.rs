//! Markdown format converter.
//!
//! This module handles conversion of Markdown (.md) files to PDF.

use std::fs;
use std::path::Path;

use crate::error::{Error, Result};

/// Converter for Markdown files.
pub struct MdConverter;

impl MdConverter {
    /// Create a new Markdown converter.
    pub fn new() -> Self {
        Self
    }

    /// Convert Markdown content to PDF.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the input Markdown file
    /// * `output_path` - Path where the PDF will be saved
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an `Error` if conversion fails.
    pub fn convert(&self, input_path: &Path, output_path: &Path) -> Result<()> {
        // Read the Markdown file
        let markdown_content = fs::read_to_string(input_path)
            .map_err(|e| Error::FileReadError(format!("{}: {}", input_path.display(), e)))?;

        // Convert Markdown to PDF with default configuration
        markdown2pdf::parse_into_file(
            markdown_content,
            output_path
                .to_str()
                .ok_or_else(|| Error::ConversionError("Invalid output path".to_string()))?,
            markdown2pdf::config::ConfigSource::Default,
            None,
        )?;

        Ok(())
    }
}

impl Default for MdConverter {
    fn default() -> Self {
        Self::new()
    }
}
