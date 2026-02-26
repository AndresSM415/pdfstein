use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, arg_required_else_help(true))]
pub struct CliArgs {
    #[arg(
        short,
        long,
        value_name = "FILE",
        num_args(1..),
        help = "Input files to convert"
    )]
    files: Vec<String>,

    #[arg(short, long, value_name = "FORMAT", help = "Output format (pdf)")]
    output_format: String,

    #[arg(
        long,
        value_name = "OUTPUT_DIR",
        help = "Output directory for converted files"
    )]
    output_dir: Option<String>,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Overwrite existing files"
    )]
    overwrite: bool,

    #[arg(
        long,
        action = ArgAction::SetTrue,
        help = "Enable verbose output"
    )]
    verbose: bool,
}
