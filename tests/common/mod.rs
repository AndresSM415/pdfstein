//! Common test utilities and helpers.
//!
//! This module provides shared functionality for integration tests,
//! including test fixtures, temporary file handling, and assertions.

use std::path::PathBuf;

/// Create a temporary directory for tests.
///
/// # Returns
///
/// Returns a PathBuf to the temporary test directory.
pub fn temp_dir() -> PathBuf {
    // TODO: Implement test temporary directory creation
    std::env::temp_dir().join("pdfpstein-test")
}

/// Clean up test artifacts.
///
/// # Arguments
///
/// * `path` - Path to clean up
pub fn cleanup(path: &std::path::Path) {
    let _ = std::fs::remove_dir_all(path);
}
