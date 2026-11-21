//! Writer document implementation

use papervlum_core::{DocumentId, DocumentMetadata, DocumentFormat, Node, NodeKind, StyleSheet, Selection, UndoStack, Result};
use std::path::Path;
use chrono::Utc;

/// Writer document
pub struct WriterDocument {
    pub metadata: DocumentMetadata,
    pub root: Node,
    pub styles: StyleSheet,
    pub selection: Selection,
    pub undo_stack: UndoStack,
}

impl WriterDocument {
    pub fn new() -> Self {
        let mut root = Node::new(NodeKind::Group { children: Vec::new() });
        let mut styles = StyleSheet::new();

        // Add default styles
        let default_para = styles.add(papervlum_core::Style::new("Normal".into(), papervlum_core::StyleType::Paragraph));
        let default_char = styles.add(papervlum_core::Style::new("Default Paragraph Font".into(), papervlum_core::StyleType::Character));
        let default_table = styles.add(papervlum_core::Style::new("Table Normal".into(), papervlum_core::StyleType::Table));
        let default_list = styles.add(papervlum_core::Style::new("List Paragraph".into(), papervlum_core::StyleType::List));

        styles.set_default_paragraph(default_para);
        styles.set_default_character(default_char);
        styles.set_default_table(default_table);
        styles.set_default_list(default_list);

        Self {
            metadata: DocumentMetadata {
                id: DocumentId::new(),
                title: "Untitled".into(),
                author: None,
                created: Utc::now(),
                modified: Utc::now(),
                format: DocumentFormat::Docx,
            },
            root,
            styles,
            selection: Selection::new(),
            undo_stack: UndoStack::new(),
        }
    }

    pub fn load(_path: &Path) -> Result<Self> {
        // TODO: Implement actual loading from .docx
        Ok(Self::new())
    }

    pub fn save(&self, _path: &Path) -> Result<()> {
        // TODO: Implement actual saving to .docx
        Ok(())
    }
}

impl Default for WriterDocument {
    fn default() -> Self {
        Self::new()
    }
}