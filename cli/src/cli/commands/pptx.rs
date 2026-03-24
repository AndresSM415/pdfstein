//! PPTX to PDF conversion command.

use clap::Parser;
use std::path::PathBuf;

use crate::cli::shared::validate_input_file;

/// Convert PPTX to PDF.
#[derive(Parser, Debug, Clone)]
#[command(about = "Convert PPTX files to PDF")]
pub struct PptxCommand {
    /// Input PPTX file (.pptx)
    #[arg(value_parser = validate_input_file)]
    pub file: PathBuf,

    /// Output PDF file path (defaults to same name as input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    // === Common flags ===
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
