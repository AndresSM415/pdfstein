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

    // ==================== FORMATTING OPTIONS ====================
    /// Paper size (a4, letter, legal, tabloid)
    #[arg(short = 'p', long, default_value = "a4")]
    pub paper_size: String,

    /// Page margins in millimeters (default: 20mm all sides)
    #[arg(long)]
    pub margin: Option<f32>,

    /// Top margin override in mm
    #[arg(long)]
    pub margin_top: Option<f32>,

    /// Bottom margin override in mm
    #[arg(long)]
    pub margin_bottom: Option<f32>,

    /// Left margin override in mm
    #[arg(long)]
    pub margin_left: Option<f32>,

    /// Right margin override in mm
    #[arg(long)]
    pub margin_right: Option<f32>,

    /// Font family for fallback text (default: sans-serif)
    #[arg(short = 'F', long, default_value = "sans-serif")]
    pub font_family: String,

    /// Base font size in points (default: 12pt)
    #[arg(short = 's', long, default_value = "12")]
    pub font_size: f32,

    /// Line height multiplier (default: 1.5)
    #[arg(long, default_value = "1.5")]
    pub line_height: f32,

    // ==================== LAYOUT OPTIONS ====================
    /// Enable landscape orientation
    #[arg(short = 'l', long)]
    pub landscape: bool,

    /// Show page numbers
    #[arg(long)]
    pub show_page_numbers: bool,

    /// Page number position (top-left, top-center, top-right, bottom-left, etc.)
    #[arg(long, default_value = "bottom-center")]
    pub page_number_position: String,

    /// Include table of contents if available in source
    #[arg(short, long)]
    pub toc: bool,

    /// TOC depth (1-5, default: 3)
    #[arg(long, default_value = "3")]
    pub toc_depth: u8,

    // ==================== QUALITY OPTIONS ====================
    /// DPI for image rendering (default: 150)
    #[arg(long, default_value = "150")]
    pub dpi: u32,

    /// Enable image compression
    #[arg(short, long)]
    pub compress_images: bool,

    /// Compression level (1-9, default: 5)
    #[arg(long, default_value = "5")]
    pub image_compression_level: u8,

    // ==================== OUTPUT OPTIONS ====================
    /// Enable PDF security/password protection
    #[arg(long)]
    pub password: Option<String>,

    /// Set PDF metadata title
    #[arg(long)]
    pub title: Option<String>,

    /// Set PDF metadata author
    #[arg(long)]
    pub author: Option<String>,

    /// Set PDF metadata subject
    #[arg(long)]
    pub subject: Option<String>,

    /// Set PDF metadata keywords
    #[arg(long)]
    pub keywords: Option<String>,

    // ==================== PERFORMANCE OPTIONS ====================
    /// Number of parallel conversion workers (default: auto)
    #[arg(short = 'w', long)]
    pub workers: Option<usize>,

    /// Maximum memory usage in MB (0 = unlimited, default: 512)
    #[arg(long, default_value = "512")]
    pub max_memory: u64,
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

    /// Get effective page margins based on override values.
    pub fn get_margins(&self) -> (f32, f32, f32, f32) {
        let base = self.margin.unwrap_or(20.0);
        (
            self.margin_top.unwrap_or(base),
            self.margin_bottom.unwrap_or(base),
            self.margin_left.unwrap_or(base),
            self.margin_right.unwrap_or(base),
        )
    }

    /// Validate configuration before processing.
    pub fn validate_config(&self) -> Result<(), String> {
        if let Some(ref password) = self.password {
            if password.is_empty() {
                return Err("Password cannot be empty".to_string());
            }
        }

        if self.dpi < 72 || self.dpi > 600 {
            return Err(format!("DPI must be between 72 and 600, got: {}", self.dpi));
        }

        if self.image_compression_level < 1 || self.image_compression_level > 9 {
            return Err(format!(
                "Compression level must be between 1 and 9, got: {}",
                self.image_compression_level
            ));
        }

        if self.toc_depth < 1 || self.toc_depth > 5 {
            return Err(format!(
                "TOC depth must be between 1 and 5, got: {}",
                self.toc_depth
            ));
        }

        Ok(())
    }
}
