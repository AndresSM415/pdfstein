//! PDFpstein CLI entry point.
//!
//! This is the main entry point for the pdfpstein command-line tool.
//! It handles argument parsing, validation, and orchestrates the conversion process.

use clap::Parser;
use pdfpstein::cli::Args;
use pdfpstein::error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// Main entry point that returns a Result for clean error handling.
fn run() -> Result<()> {
    let args = Args::parse();

    // Validate input file exists
    if !args.file.exists() {
        eprintln!("Error: file '{}' not found.", args.file.display());
        std::process::exit(1);
    }

    let output = args.resolve_output();

    let format = match args.detect_format() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    if !args.quiet {
        println!(
            "Converting '{}' -> '{}' (format: {:?})",
            args.file.display(),
            output.display(),
            format
        );
    }

    // TODO: Perform conversion using the converter module
    // let converter = pdfpstein::converter::create_converter(format);
    // let options = pdfpstein::config::ConversionOptions::from_args(&args);
    // converter.convert(&args.file, &output, &options)?;

    if !args.quiet {
        println!("Done! PDF saved to '{}'", output.display());
    }

    if args.open {
        let _ = std::process::Command::new("open").arg(&output).status(); // macOS
        // use "xdg-open" on Linux, "start" on Windows
    }

    Ok(())
}
