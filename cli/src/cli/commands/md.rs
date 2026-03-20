//! Markdown to PDF conversion command.

use clap::Parser;
use std::path::PathBuf;

use crate::cli::shared::validate_input_file;

/// Convert Markdown to PDF.
#[derive(Parser, Debug, Clone)]
#[command(about = "Convert Markdown files to PDF")]
pub struct MdCommand {
    /// Input markdown file (.md or .markdown)
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
