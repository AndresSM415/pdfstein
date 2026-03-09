//! PDFpstein CLI entry point.
//!
//! This is the main entry point for the pdfpstein command-line tool.
//! It handles argument parsing, validation, and orchestrates the conversion process.

use std::env::consts;

use clap::Parser;
use pdfpstein::cli::{Args, ConvertCommand};
use pdfpstein::error::Result;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    let input_file = args.input_file();
    let output = args.resolve_output();
    let format = args.detect_format();

    if !args.is_quiet() {
        println!(
            "Converting '{}' -> '{}' (format: {:?})",
            input_file.display(),
            output.display(),
            format
        );
    }

    // TODO: Perform conversion using the converter module
    match &args.command {
        ConvertCommand::Md(cmd) => {
            println!("{:?}", cmd)
        }
        ConvertCommand::Pptx(cmd) => {
            println!("{:?}", cmd)
        }
        ConvertCommand::Docx(cmd) => {
            println!("{:?}", cmd)
        }
    }

    if !args.is_quiet() {
        println!("Done! PDF saved to '{}'", output.display());
    }

    if args.should_open() {
        if !args.is_quiet() {
            println!("Opening PDF...");
        }

        let open_result = match consts::OS {
            "linux" => {
                // Try common Linux open commands in order of preference
                std::process::Command::new("xdg-open")
                    .arg(&output)
                    .status()
                    .or_else(|_| {
                        std::process::Command::new("gnome-open")
                            .arg(&output)
                            .status()
                    })
                    .or_else(|_| std::process::Command::new("kde-open").arg(&output).status())
            }
            "windows" => std::process::Command::new("cmd")
                .arg("/c")
                .arg("start")
                .arg(&output)
                .status(),
            "macos" => std::process::Command::new("open").arg(&output).status(),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Opening files is not supported on this platform",
            )),
        };

        if let Err(e) = open_result {
            eprintln!("Warning: Failed to open PDF: {}", e);
        }
    }

    Ok(())
}
