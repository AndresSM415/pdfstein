//! Shared CLI utilities and validators.

use std::path::{Path, PathBuf};

/// Validate that input file exists and is readable.
pub fn validate_input_file(path_str: &str) -> Result<PathBuf, String> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Input file does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }

    Ok(path.to_path_buf())
}
