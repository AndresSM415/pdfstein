//! PDFpstein - Convert DOCX, PPTX, and Markdown files to PDF
//!
//! This library provides the core conversion functionality for the pdfpstein CLI tool.

pub mod cli;
pub mod config;
pub mod converter;
pub mod error;
pub mod pdf;
pub mod utils;

// Re-export commonly used types
pub use converter::Converter;
pub use error::Result;
