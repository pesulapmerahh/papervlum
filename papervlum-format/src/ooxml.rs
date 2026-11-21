//! OOXML (.docx, .xlsx, .pptx) parser

use crate::{FormatParser, FormatWriter};
use papervlum_core::{Document, DocumentFormat, Result};
use std::path::Path;
use zip::ZipArchive;

/// OOXML parser - handles .docx, .xlsx, .pptx
pub struct OoxmlParser;

impl OoxmlParser {
    /// Detect document type from [Content_Types].xml
    fn detect_type(archive: &mut ZipArchive<std::fs::File>) -> Result<DocumentFormat> {
        let mut content_types = archive.by_name("[Content_Types].xml")?;
        let mut buf = String::new();
        std::io::Read::read_to_string(&mut content_types, &mut buf)?;

        if buf.contains("wordprocessingml") {
            Ok(DocumentFormat::Docx)
        } else if buf.contains("spreadsheetml") {
            Ok(DocumentFormat::Xlsx)
        } else if buf.contains("presentationml") {
            Ok(DocumentFormat::Pptx)
        } else {
            anyhow::bail!("Unknown OOXML document type");
        }
    }

    /// Parse Word document (.docx)
    fn parse_docx(archive: ZipArchive<std::fs::File>) -> Result<Box<dyn Document>> {
        // TODO: Implement full .docx parsing
        // For now, return a stub
        use papervlum_writer::WriterDocument;
        Ok(Box::new(WriterDocument::new()))
    }

    /// Parse Spreadsheet (.xlsx)
    fn parse_xlsx(archive: ZipArchive<std::fs::File>) -> Result<Box<dyn Document>> {
        // TODO: Implement full .xlsx parsing
        use papervlum_calc::CalcDocument;
        Ok(Box::new(CalcDocument::new()))
    }

    /// Parse Presentation (.pptx)
    fn parse_pptx(archive: ZipArchive<std::fs::File>) -> Result<Box<dyn Document>> {
        // TODO: Implement full .pptx parsing
        use papervlum_impress::ImpressDocument;
        Ok(Box::new(ImpressDocument::new()))
    }
}

impl FormatParser for OoxmlParser {
    fn can_parse(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        matches!(ext.to_lowercase().as_str(), "docx" | "xlsx" | "pptx")
    }

    fn parse(&self, path: &Path) -> Result<Box<dyn Document>> {
        let file = std::fs::File::open(path)?;
        let mut archive = ZipArchive::new(file)?;
        let format = Self::detect_type(&mut archive)?;

        match format {
            DocumentFormat::Docx => Self::parse_docx(archive),
            DocumentFormat::Xlsx => Self::parse_xlsx(archive),
            DocumentFormat::Pptx => Self::parse_pptx(archive),
            _ => anyhow::bail!("Unsupported OOXML format: {:?}", format),
        }
    }
}

impl FormatWriter for OoxmlParser {
    fn can_write(&self, format: DocumentFormat) -> bool {
        matches!(format, DocumentFormat::Docx | DocumentFormat::Xlsx | DocumentFormat::Pptx)
    }

    fn write(&self, document: &dyn Document, path: &Path) -> Result<()> {
        // TODO: Implement writing
        match document.format() {
            DocumentFormat::Docx => {
                // Serialize to .docx
                todo!("Write .docx")
            }
            DocumentFormat::Xlsx => {
                // Serialize to .xlsx
                todo!("Write .xlsx")
            }
            DocumentFormat::Pptx => {
                // Serialize to .pptx
                todo!("Write .pptx")
            }
            _ => anyhow::bail!("Format not writable by OOXML parser"),
        }
    }
}

/// Parsed OOXML document structure (generic, format-agnostic)
#[derive(Debug, Default)]
pub struct ParsedOoxmlDocument {
    pub format: DocumentFormat,
    pub content_types: String,
    pub main_part: String, // word/document.xml, xl/workbook.xml, ppt/presentation.xml
    pub styles_part: Option<String>, // word/styles.xml, xl/styles.xml, ppt/theme/theme1.xml
    pub numbering_part: Option<String>, // word/numbering.xml
    pub relationships: Vec<String>, // .rels files
    pub media_files: Vec<(String, Vec<u8>)>, // (path, data)
}

impl OoxmlParser {
    /// Parse OOXML archive into a generic structure
    pub fn parse_generic(archive: &mut ZipArchive<std::fs::File>) -> Result<ParsedOoxmlDocument> {
        // Read content types first
        let mut ct_buf = String::new();
        {
            let mut content_types = archive.by_name("[Content_Types].xml")?;
            std::io::Read::read_to_string(&mut content_types, &mut ct_buf)?;
        } // content_types dropped here

        let format = if ct_buf.contains("wordprocessingml") {
            DocumentFormat::Docx
        } else if ct_buf.contains("spreadsheetml") {
            DocumentFormat::Xlsx
        } else if ct_buf.contains("presentationml") {
            DocumentFormat::Pptx
        } else {
            anyhow::bail!("Unknown OOXML document type");
        };

        let main_part_name = match format {
            DocumentFormat::Docx => "word/document.xml",
            DocumentFormat::Xlsx => "xl/workbook.xml",
            DocumentFormat::Pptx => "ppt/presentation.xml",
            _ => anyhow::bail!("Unknown format"),
        };

        let mut main_buf = String::new();
        {
            let mut main_part = archive.by_name(main_part_name)?;
            std::io::Read::read_to_string(&mut main_part, &mut main_buf)?;
        }

        let styles_part = match format {
            DocumentFormat::Docx => "word/styles.xml",
            DocumentFormat::Xlsx => "xl/styles.xml",
            DocumentFormat::Pptx => "ppt/theme/theme1.xml",
            _ => "",
        };

        let styles = if !styles_part.is_empty() {
            archive.by_name(styles_part).ok().and_then(|mut f| {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut f, &mut buf).ok()?;
                Some(buf)
            })
        } else {
            None
        };

        let numbering_part = if format == DocumentFormat::Docx {
            "word/numbering.xml"
        } else {
            ""
        };

        let numbering = if !numbering_part.is_empty() {
            archive.by_name(numbering_part).ok().and_then(|mut f| {
                let mut buf = String::new();
                std::io::Read::read_to_string(&mut f, &mut buf).ok()?;
                Some(buf)
            })
        } else {
            None
        };

        // Collect relationships - need to get names first, then read each
        let rel_names: Vec<String> = (0..archive.len())
            .filter_map(|i| archive.by_index(i).ok().map(|f| f.name().to_string()))
            .filter(|name| name.ends_with(".rels"))
            .collect();

        let mut relationships = Vec::new();
        for name in rel_names {
            if let Ok(mut f) = archive.by_name(&name) {
                let mut buf = String::new();
                if std::io::Read::read_to_string(&mut f, &mut buf).is_ok() {
                    relationships.push(buf);
                }
            }
        }

        // Collect media files
        let media_names: Vec<String> = (0..archive.len())
            .filter_map(|i| archive.by_index(i).ok().map(|f| f.name().to_string()))
            .filter(|name| name.starts_with("word/media/") || name.starts_with("xl/media/") || name.starts_with("ppt/media/"))
            .collect();

        let mut media_files = Vec::new();
        for name in media_names {
            if let Ok(mut f) = archive.by_name(&name) {
                let mut buf = Vec::new();
                if std::io::Read::read_to_end(&mut f, &mut buf).is_ok() {
                    media_files.push((name, buf));
                }
            }
        }

        Ok(ParsedOoxmlDocument {
            format,
            content_types: ct_buf,
            main_part: main_buf,
            styles_part: styles,
            numbering_part: numbering,
            relationships,
            media_files,
        })
    }
}