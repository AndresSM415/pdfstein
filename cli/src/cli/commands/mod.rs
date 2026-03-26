//! CLI subcommand definitions.
//!
//! Each subcommand is defined in its own module for better organization.

mod docx;
mod md;
mod pptx;

pub use docx::DocxCommand;
pub use md::MdCommand;
pub use pptx::PptxCommand;
