//! Centralized error types for PDFpstein.
//!
//! This module defines all error types used throughout the application,
//! providing a unified error handling strategy.

use std::fmt;

/// Result type alias using the application's Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error enum for PDFpstein.
#[derive(Debug)]
pub enum Error {
    /// Input file not found.
    FileNotFound(String),
    /// Failed to read input file.
    FileReadError(String),
    /// Failed to write output file.
    FileWriteError(String),
    /// Markdown conversion failure.
    ConversionError(String),
    /// Unsupported format.
    UnsupportedFormat(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileNotFound(path) => write!(f, "File not found: {}", path),
            Error::FileReadError(path) => write!(f, "Failed to read file: {}", path),
            Error::FileWriteError(path) => write!(f, "Failed to write file: {}", path),
            Error::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
            Error::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
        }
    }
}

impl std::error::Error for Error {}

impl From<markdown2pdf::MdpError> for Error {
    fn from(err: markdown2pdf::MdpError) -> Self {
        Error::ConversionError(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::ConversionError(err.to_string())
    }
}
