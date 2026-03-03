//! Document conversion module.
//!
//! This module provides the core conversion functionality for transforming
//! DOCX, PPTX, and Markdown files into PDF format.
//!
//! ## Architecture
//!
//! The conversion system uses a trait-based approach where each format
//! implements the `Converter` trait. A factory function selects the
//! appropriate converter based on the input format.

mod docx;
mod md;
mod pptx;

pub use docx::DocxConverter;
pub use md::MdConverter;
pub use pptx::PptxConverter;

use crate::config::ConversionOptions;
use crate::error::Result;
use std::path::{Path, PathBuf};

/// Trait for document converters.
///
/// All format-specific converters must implement this trait.
pub trait Converter {
    /// Convert the input file to PDF.
    ///
    /// # Arguments
    ///
    /// * `input` - Path to the input file
    /// * `output` - Path for the output PDF
    /// * `options` - Conversion options and settings
    ///
    /// # Returns
    ///
    /// Returns the path to the generated PDF on success.
    fn convert(&self, input: &Path, output: &Path, options: &ConversionOptions) -> Result<PathBuf>;

    /// Get the format name this converter handles.
    fn format_name(&self) -> &'static str;
}

/// Factory function to create a converter for the specified format.
///
/// # Arguments
///
/// * `format` - The input file format (docx, pptx, or md)
///
/// # Returns
///
/// Returns a boxed converter trait object.
pub fn create_converter(format: crate::cli::Format) -> Box<dyn Converter> {
    match format {
        crate::cli::Format::Docx => Box::new(DocxConverter::new()),
        crate::cli::Format::Pptx => Box::new(PptxConverter::new()),
        crate::cli::Format::Md => Box::new(MdConverter::new()),
    }
}
