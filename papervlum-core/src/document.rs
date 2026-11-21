//! Document trait and metadata

use crate::node::{Node, NodeId};
use crate::style::StyleSheet;
use crate::selection::Selection;
use crate::undo::UndoStack;
use crate::command::Command;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

/// Unique identifier for a document
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(pub u64);

impl DocumentId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for DocumentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Document metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: DocumentId,
    pub title: String,
    pub author: Option<String>,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub format: DocumentFormat,
}

/// Supported document formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentFormat {
    Docx,
    Xlsx,
    Pptx,
    Pdf,
    Wps,
    Et,
    Dps,
}

/// Core document trait - implemented by each app (Writer, Calc, Impress, PDF)
pub trait Document: Send + Sync {
    /// Load document from file
    fn load(path: &Path) -> Result<Self> where Self: Sized;

    /// Save document to file
    fn save(&self, path: &Path) -> Result<()>;

    /// Get document metadata
    fn metadata(&self) -> &DocumentMetadata;

    /// Get mutable metadata
    fn metadata_mut(&mut self) -> &mut DocumentMetadata;

    /// Get the root node of the document tree
    fn root(&self) -> &crate::node::Node;

    /// Get mutable root node
    fn root_mut(&mut self) -> &mut crate::node::Node;

    /// Get the style sheet
    fn styles(&self) -> &crate::style::StyleSheet;

    /// Get mutable style sheet
    fn styles_mut(&mut self) -> &mut crate::style::StyleSheet;

    /// Get current selection
    fn selection(&self) -> &crate::selection::Selection;

    /// Set selection
    fn set_selection(&mut self, selection: crate::selection::Selection);

    /// Get undo stack
    fn undo_stack(&self) -> &crate::undo::UndoStack;

    /// Get mutable undo stack
    fn undo_stack_mut(&mut self) -> &mut crate::undo::UndoStack;

    /// Execute a command (records to undo stack)
    fn execute(&mut self, command: Box<dyn Command>) -> Result<()>;

    /// Render document to GPU commands for a given viewport
    fn render(&self, viewport: RenderViewport) -> RenderCommands;

    /// Get document format
    fn format(&self) -> DocumentFormat;
}

/// Viewport for rendering
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RenderViewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

/// GPU render commands (opaque, submitted to wgpu queue)
#[derive(Debug, Clone, Default)]
pub struct RenderCommands {
    pub commands: Vec<RenderCommand>,
}

#[derive(Debug, Clone)]
pub enum RenderCommand {
    DrawText {
        glyphs: Vec<GlyphInstance>,
        color: [f32; 4],
    },
    DrawRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
    },
    DrawImage {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        texture_id: u64,
    },
    SetClipRect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    ClearClipRect,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphInstance {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}