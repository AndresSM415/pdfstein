//! CLI argument parsing module.
//!
//! This module handles all command-line interface functionality,
//! including argument definition, parsing, and validation.
//!
//! ## Structure
//!
//! - `args` - Main Args struct and subcommand enum
//! - `commands` - Individual subcommand definitions (md, docx, pptx)
//! - `shared` - Shared utilities like validators

pub mod args;
pub mod commands;
pub mod shared;

pub use args::{Args, ConvertCommand, Format};
pub use commands::{DocxCommand, MdCommand, PptxCommand};
