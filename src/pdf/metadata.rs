//! PDF metadata handling.
//!
//! This module provides structures and functions for managing
//! PDF document metadata (title, author, subject, keywords).

/// PDF document metadata.
#[derive(Debug, Clone, Default)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
}

impl PdfMetadata {
    /// Create a new PdfMetadata with the given values.
    pub fn new(
        title: Option<String>,
        author: Option<String>,
        subject: Option<String>,
        keywords: Option<String>,
    ) -> Self {
        Self {
            title,
            author,
            subject,
            keywords,
        }
    }

    /// Check if any metadata is present.
    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.author.is_none()
            && self.subject.is_none()
            && self.keywords.is_none()
    }
}
