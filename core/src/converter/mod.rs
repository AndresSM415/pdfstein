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

pub use md::MdConverter;

/// Trait for document converters.
///
/// Implementors of this trait can convert a specific input format to PDF.
pub trait Converter {
    /// Convert the input file to a PDF at the specified output path.
    ///
    /// # Arguments
    ///
    /// * `input_path` - Path to the input file
    /// * `output_path` - Path where the PDF will be saved
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success, or an error if conversion fails.
    fn convert(
        &self,
        input_path: &std::path::Path,
        output_path: &std::path::Path,
    ) -> crate::error::Result<()>;
}

impl Converter for MdConverter {
    fn convert(
        &self,
        input_path: &std::path::Path,
        output_path: &std::path::Path,
    ) -> crate::error::Result<()> {
        self.convert(input_path, output_path)
    }
}
