use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Format {
    Docx,
    Pptx,
    Md,
}

#[derive(Parser, Debug)]
#[command(name = "pdfpstein")]
#[command(about = "Convert docx/pptx/md files to PDF")]
#[command(version = "0.1.0")]
pub struct Args {
    /// Input file to convert (.docx, .pptx, or .md)
    pub file: PathBuf,

    /// Output PDF file path (defaults to same name/dir as input)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Override format detection (auto-detected from extension by default)
    #[arg(short, long, value_enum)]
    pub format: Option<Format>,

    /// Open the PDF after conversion
    #[arg(short = 'O', long)]
    pub open: bool,

    /// Suppress output messages
    #[arg(short, long)]
    pub quiet: bool,
}

impl Args {
    /// Resolve the output path: use provided output, or derive from input file
    pub fn resolve_output(&self) -> PathBuf {
        if let Some(ref out) = self.output {
            out.clone()
        } else {
            let parent = self.file.parent().unwrap_or(Path::new("."));
            let stem = self.file.file_stem().unwrap_or_default();
            parent.join(stem).with_extension("pdf")
        }
    }

    /// Detect format from file extension if not explicitly set
    pub fn detect_format(&self) -> Result<Format, String> {
        if let Some(ref fmt) = self.format {
            return Ok(fmt.clone());
        }

        match self.file.extension().and_then(|e| e.to_str()) {
            Some("docx") => Ok(Format::Docx),
            Some("pptx") => Ok(Format::Pptx),
            Some("md") => Ok(Format::Md),
            Some(ext) => Err(format!("Unsupported extension: .{ext}")),
            None => Err("File has no extension. Use --format to specify.".to_string()),
        }
    }
}
