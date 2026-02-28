use clap::Parser;

mod cli;

fn main() {
    let args = cli::Args::parse();

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

    // TODO: conversion logic

    if !args.quiet {
        println!("Done! PDF saved to '{}'", output.display());
    }

    if args.open {
        let _ = std::process::Command::new("open").arg(&output).status(); // macOS
        // use "xdg-open" on Linux, "start" on Windows
    }
}
