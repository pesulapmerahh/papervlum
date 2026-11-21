//! PDF parser

use crate::{FormatParser, FormatWriter};
use papervlum_core::{Document, DocumentFormat, Result};
use std::path::Path;

/// PDF parser
pub struct PdfParser;

impl FormatParser for PdfParser {
    fn can_parse(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        ext.to_lowercase() == "pdf"
    }

    fn parse(&self, path: &Path) -> Result<Box<dyn Document>> {
        // TODO: Implement PDF parsing with lopdf or pdfium
        anyhow::bail!("PDF parsing not yet implemented")
    }
}

impl FormatWriter for PdfParser {
    fn can_write(&self, format: DocumentFormat) -> bool {
        format == DocumentFormat::Pdf
    }

    fn write(&self, document: &dyn Document, path: &Path) -> Result<()> {
        // TODO: Implement PDF writing
        anyhow::bail!("Write PDF not implemented")
    }
}