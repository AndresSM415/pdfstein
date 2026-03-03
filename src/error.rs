//! Centralized error types for PDFpstein.
//!
//! This module defines all error types used throughout the application,
//! providing a unified error handling strategy.

/// Result type alias using the application's Error type.
pub type Result<T> = std::result::Result<T, Error>;

/// Main error enum for PDFpstein.
///
/// TODO: Implement specific error variants for different failure modes:
/// - File not found
/// - Unsupported format
/// - Conversion failure
/// - Invalid configuration
#[derive(Debug)]
pub enum Error {
    /// Placeholder for future error variants.
    Unimplemented,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unimplemented => write!(f, "This feature is not yet implemented"),
        }
    }
}

impl std::error::Error for Error {}
