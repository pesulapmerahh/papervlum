//! Papervlum Writer - Word processor

pub mod document;
pub mod model;
pub mod layout;
pub mod shaping;
pub mod render;
pub mod edit;
pub mod commands;
pub mod save;
pub mod streaming;
pub mod autosave;
pub mod scripts;

use crate::document::WriterDocument;
use papervlum_core::{Document, DocumentFormat, Result};
use std::path::Path;

impl Document for WriterDocument {
    fn load(path: &Path) -> Result<Self> where Self: Sized {
        Self::load(path)
    }

    fn save(&self, path: &Path) -> Result<()> {
        Self::save(self, path)
    }

    fn metadata(&self) -> &papervlum_core::DocumentMetadata {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut papervlum_core::DocumentMetadata {
        &mut self.metadata
    }

    fn root(&self) -> &papervlum_core::Node {
        &self.root
    }

    fn root_mut(&mut self) -> &mut papervlum_core::Node {
        &mut self.root
    }

    fn styles(&self) -> &papervlum_core::StyleSheet {
        &self.styles
    }

    fn styles_mut(&mut self) -> &mut papervlum_core::StyleSheet {
        &mut self.styles
    }

    fn selection(&self) -> &papervlum_core::Selection {
        &self.selection
    }

    fn set_selection(&mut self, selection: papervlum_core::Selection) {
        self.selection = selection;
    }

    fn undo_stack(&self) -> &papervlum_core::UndoStack {
        &self.undo_stack
    }

    fn undo_stack_mut(&mut self) -> &mut papervlum_core::UndoStack {
        &mut self.undo_stack
    }

    fn execute(&mut self, command: Box<dyn papervlum_core::Command>) -> Result<()> {
        self.undo_stack.push(command)?;
        Ok(())
    }

    fn render(&self, _viewport: papervlum_core::RenderViewport) -> papervlum_core::RenderCommands {
        Default::default()
    }

    fn format(&self) -> DocumentFormat {
        DocumentFormat::Docx
    }
}