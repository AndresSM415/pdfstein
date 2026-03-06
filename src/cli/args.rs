//! CLI argument definitions and validation.
//!
//! This module contains the main Args struct and related types
//! for parsing command-line arguments using clap.

use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

/// Supported input file formats.
#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Docx,
    Pptx,
    Md,
}

/// Main CLI arguments for PDFpstein.
///
/// TODO: Consider moving validation logic into separate methods or a builder pattern.
#[derive(Parser, Debug)]
#[command(name = "pdfpstein")]
#[command(about = "Convert docx/pptx/md files to PDF", long_about = None)]
#[command(bin_name = "pdfpstein")]
#[command(before_help = r#"
    ____  ____  _____          _       _
   |  _ \|  _ \|  ___| __  ___| |_ ___(_)_ __
   | |_) | | | | |_ | '_ \/ __| __/ _ \ | '_ \
   |  __/| |_| |  _|| |_) \__ \ ||  __/ | | | |
   |_|   |____/|_|  | .__/|___/\__\___|_|_| |_|
                    |_|
"#)]
pub struct Args {
    /// Input file path (.docx, .pptx, or .md)
    #[arg(value_parser = validate_input_file)]
    pub file: PathBuf,

    /// Output PDF file path (defaults to same name/dir as input)
    #[arg(short, long, value_parser = validate_output_file)]
    pub output: Option<PathBuf>,

    /// Force specific format (auto-detected from extension by default)
    #[arg(short = 'f', long, value_enum)]
    pub format: Option<Format>,

    /// Open the PDF after successful conversion
    #[arg(short = 'O', long)]
    pub open: bool,

    /// Suppress non-essential output messages
    #[arg(short, long)]
    pub quiet: bool,

    /// Show verbose error messages and progress
    #[arg(long)]
    pub verbose: bool,
}

/// Validate that input file exists and is readable.
fn validate_input_file(path_str: &str) -> Result<PathBuf, String> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Input file does not exist: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Path is not a file: {}", path.display()));
    }

    Ok(path.to_path_buf())
}

fn validate_output_file(path_str: &str) -> Result<PathBuf, String> {
    let path = Path::new(path_str);

    if !path.exists() {
        return Err(format!("Output dir does not exist: {}", path.display()));
    }

    if !path.is_dir() {
        return Err(format!("Path is not a dir: {}", path.display()));
    }

    Ok(path.to_path_buf())
}

impl Args {
    /// Resolve the output path: use provided output, or derive from input file.
    pub fn resolve_output(&self) -> PathBuf {
        if let Some(ref out) = self.output {
            out.clone()
        } else {
            let parent = self.file.parent().unwrap_or(Path::new("."));
            let stem = self.file.file_stem().unwrap_or_default();
            parent.join(stem).with_extension("pdf")
        }
    }

    /// Detect format from file extension if not explicitly set.
    pub fn detect_format(&self) -> Result<Format, String> {
        if let Some(ref fmt) = self.format {
            return Ok(fmt.clone());
        }

        match self.file.extension().and_then(|e| e.to_str()) {
            Some("docx") => Ok(Format::Docx),
            Some("pptx") => Ok(Format::Pptx),
            Some("md") | Some("markdown") => Ok(Format::Md),
            Some(ext) => Err(format!("Unsupported file extension: .{ext}")),
            None => Err("File has no extension. Use --format to specify.".to_string()),
        }
    }

    /// Check if verbose mode is enabled.
    pub fn is_verbose(&self) -> bool {
        self.verbose
    }

    /// Check if output should be suppressed.
    pub fn is_quiet(&self) -> bool {
        self.quiet
    }
}
