//! Document conversion module.
//!
//! This module provides the core conversion functionality for transforming
//! DOCX, PPTX, and Markdown files into PDF format.
//!
//! ## Architecture
//!
//! The conversion system uses a trait-based approach where each format
//! implements the `Converter` trait. A factory function selects the
//! appropriate converter based on the input format.

mod docx;
mod md;
mod pptx;
