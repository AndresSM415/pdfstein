//! PDFpstein - Convert DOCX, PPTX, and Markdown files to PDF
//!
//! This library provides the core conversion functionality for the pdfpstein CLI tool.

pub mod cli;
pub mod converter;
pub mod error;
pub mod pdf;

// Re-export commonly used types
pub use error::Result;
