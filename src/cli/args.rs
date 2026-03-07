//! CLI argument definitions and validation.
//!
//! This module contains the main Args struct and related types
//! for parsing command-line arguments using clap.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

/// Supported input file formats.
#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Docx,
    Pptx,
    Md,
}

/// Convert Markdown to PDF.
#[derive(Parser, Debug, Clone)]
#[command(about = "Convert Markdown files to PDF")]
pub struct MdCommand {
    /// Input Markdown file (.md or .markdown)
    #[arg(value_parser = validate_input_file)]
    pub file: PathBuf,

    /// Output PDF file path (defaults to same name as input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Enable GitHub Flavored Markdown extensions
    #[arg(long)]
    pub gfm: bool,

    /// Enable syntax highlighting for code blocks
    #[arg(long)]
    pub highlight: bool,

    /// Maximum heading depth for table of contents
    #[arg(long, default_value = "3")]
    pub toc_depth: u8,

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

/// Convert DOCX to PDF.
#[derive(Parser, Debug, Clone)]
#[command(about = "Convert DOCX files to PDF")]
pub struct DocxCommand {
    /// Input DOCX file (.docx)
    #[arg(value_parser = validate_input_file)]
    pub file: PathBuf,

    /// Output PDF file path (defaults to same name as input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Extract and embed images from the document
    #[arg(long)]
    pub extract_images: bool,

    /// Preserve document styles and formatting
    #[arg(long, default_value = "true")]
    pub preserve_styles: bool,

    /// Handle track changes (accept, reject, or show)
    #[arg(long, default_value = "accept")]
    pub track_changes: String,

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

    /// Slide range to convert (e.g., "1-5,8,10-")
    #[arg(long)]
    pub slide_range: Option<String>,

    /// Include speaker notes in the PDF
    #[arg(long)]
    pub include_notes: bool,

    /// Slides per page (1-9)
    #[arg(long, default_value = "1")]
    pub slides_per_page: u8,

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

/// Which command to run.
#[derive(Subcommand, Debug, Clone)]
pub enum ConvertCommand {
    Md(MdCommand),
    Docx(DocxCommand),
    Pptx(PptxCommand),
}

/// Main CLI arguments for PDFpstein.
#[derive(Parser, Debug)]
#[command(name = "pdfpstein")]
#[command(about = "Convert docx/pptx/md files to PDF")]
#[command(long_about = r#"
PDFpstein - A unified document to PDF converter

Usage:
  pdfpstein <command> [options]

Examples:
  pdfpstein md readme.md
  pdfpstein docx report.docx --extract-images
  pdfpstein pptx slides.pptx --slide-range "1-5,8"
"#)]
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
    /// Subcommand to run
    #[command(subcommand)]
    pub command: ConvertCommand,
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

impl Args {
    /// Get the input file path from the selected command.
    pub fn input_file(&self) -> &PathBuf {
        match &self.command {
            ConvertCommand::Md(cmd) => &cmd.file,
            ConvertCommand::Docx(cmd) => &cmd.file,
            ConvertCommand::Pptx(cmd) => &cmd.file,
        }
    }

    /// Get the output file path from the selected command.
    pub fn resolve_output(&self) -> PathBuf {
        let (file, output) = match &self.command {
            ConvertCommand::Md(cmd) => (&cmd.file, &cmd.output),
            ConvertCommand::Docx(cmd) => (&cmd.file, &cmd.output),
            ConvertCommand::Pptx(cmd) => (&cmd.file, &cmd.output),
        };

        if let Some(out) = output {
            out.clone()
        } else {
            let parent = file.parent().unwrap_or(Path::new("."));
            let stem = file.file_stem().unwrap_or_default();
            parent.join(stem).with_extension("pdf")
        }
    }

    /// Get the detected format from the selected command.
    pub fn detect_format(&self) -> Format {
        match &self.command {
            ConvertCommand::Md(_) => Format::Md,
            ConvertCommand::Docx(_) => Format::Docx,
            ConvertCommand::Pptx(_) => Format::Pptx,
        }
    }

    /// Check if verbose mode is enabled.
    pub fn is_verbose(&self) -> bool {
        match &self.command {
            ConvertCommand::Md(cmd) => cmd.verbose,
            ConvertCommand::Docx(cmd) => cmd.verbose,
            ConvertCommand::Pptx(cmd) => cmd.verbose,
        }
    }

    /// Check if output should be suppressed.
    pub fn is_quiet(&self) -> bool {
        match &self.command {
            ConvertCommand::Md(cmd) => cmd.quiet,
            ConvertCommand::Docx(cmd) => cmd.quiet,
            ConvertCommand::Pptx(cmd) => cmd.quiet,
        }
    }

    /// Check if the PDF should be opened after conversion.
    pub fn should_open(&self) -> bool {
        match &self.command {
            ConvertCommand::Md(cmd) => cmd.open,
            ConvertCommand::Docx(cmd) => cmd.open,
            ConvertCommand::Pptx(cmd) => cmd.open,
        }
    }
}
