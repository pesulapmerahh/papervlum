//! Papervlum Format - Document format parsers and writers

pub mod ooxml;
pub mod pdf;
pub mod wps;

use crate::ooxml::OoxmlParser;
use crate::pdf::PdfParser;
use crate::wps::WpsParser;
use papervlum_core::{Document, DocumentFormat, Result};
use std::path::Path;

/// Format parser trait
pub trait FormatParser: Send + Sync {
    fn can_parse(&self, path: &Path) -> bool;
    fn parse(&self, path: &Path) -> Result<Box<dyn Document>>;
}

/// Format writer trait
pub trait FormatWriter: Send + Sync {
    fn can_write(&self, format: DocumentFormat) -> bool;
    fn write(&self, document: &dyn Document, path: &Path) -> Result<()>;
}

/// Main parser registry
pub struct FormatRegistry {
    parsers: Vec<Box<dyn FormatParser>>,
    writers: Vec<Box<dyn FormatWriter>>,
}

impl FormatRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: Vec::new(),
            writers: Vec::new(),
        };
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        // OOXML parser handles .docx, .xlsx, .pptx
        self.parsers.push(Box::new(OoxmlParser));
        self.writers.push(Box::new(OoxmlParser)); // OoxmlParser implements both

        // PDF parser
        self.parsers.push(Box::new(PdfParser));
        self.writers.push(Box::new(PdfParser));

        // WPS legacy formats
        self.parsers.push(Box::new(WpsParser));
    }

    pub fn parse(&self, path: &Path) -> Result<Box<dyn Document>> {
        for parser in &self.parsers {
            if parser.can_parse(path) {
                return parser.parse(path);
            }
        }
        anyhow::bail!("No parser found for file: {}", path.display());
    }

    pub fn write(&self, document: &dyn Document, path: &Path) -> Result<()> {
        let format = document.format();
        for writer in &self.writers {
            if writer.can_write(format) {
                return writer.write(document, path);
            }
        }
        anyhow::bail!("No writer found for format: {:?}", format);
    }

    pub fn register_parser(&mut self, parser: Box<dyn FormatParser>) {
        self.parsers.push(parser);
    }

    pub fn register_writer(&mut self, writer: Box<dyn FormatWriter>) {
        self.writers.push(writer);
    }
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}