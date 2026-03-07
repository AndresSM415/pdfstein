//! Main CLI arguments and subcommand orchestration.
//!
//! This module defines the top-level Args struct that holds the selected
//! subcommand, along with shared helper methods.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::{Path, PathBuf};

use crate::cli::commands::{DocxCommand, MdCommand, PptxCommand};

/// Supported input file formats.
#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Docx,
    Pptx,
    Md,
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
  pdfpstein md readme.md --gfm --highlight
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
