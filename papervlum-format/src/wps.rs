//! WPS legacy format parser (.wps, .et, .dps)

use crate::{FormatParser, FormatWriter};
use papervlum_core::{Document, DocumentFormat, Result};
use std::path::Path;

/// WPS legacy format parser
pub struct WpsParser;

impl FormatParser for WpsParser {
    fn can_parse(&self, path: &Path) -> bool {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        matches!(ext.to_lowercase().as_str(), "wps" | "et" | "dps" | "wpt" | "ett" | "dpt")
    }

    fn parse(&self, path: &Path) -> Result<Box<dyn Document>> {
        // TODO: Implement WPS binary format parsing
        // This requires reverse-engineering the OLE2 compound document format
        anyhow::bail!("WPS format parsing not yet implemented")
    }
}

impl FormatWriter for WpsParser {
    fn can_write(&self, format: DocumentFormat) -> bool {
        matches!(format, DocumentFormat::Wps | DocumentFormat::Et | DocumentFormat::Dps)
    }

    fn write(&self, document: &dyn Document, path: &Path) -> Result<()> {
        anyhow::bail!("WPS format writing not yet implemented")
    }
}