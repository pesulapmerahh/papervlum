//! Papervlum Core - Shared document model and editing primitives

pub mod document;
pub mod node;
pub mod style;
pub mod selection;
pub mod undo;
pub mod command;

pub use document::{Document, DocumentId, DocumentMetadata, DocumentFormat, RenderViewport, RenderCommands, RenderCommand, GlyphInstance};
pub use node::{Node, NodeId, NodeKind, SectionType};
pub use style::{
    Style, StyleId, StyleSheet, StyleProperty, StyleType,
    Alignment, TextDirection, UnderlineStyle, LineSpacing,
    TabAlignment, TabLeader, TableWidth, VerticalAlignment,
    Border, BorderStyle, FillPattern, Gradient, GradientStop,
    PatternType, CellMargin, NumberingFormat, Length, Margin, Padding,
    Display, Position, Transform,
};
pub use selection::{Selection, SelectionRange, Cursor, TableSelection, CellAddress, ShapeSelection};
pub use undo::UndoStack;
pub use command::Command;
pub use anyhow::Result;

/// Initialize the core library (called from JNI_OnLoad)
pub fn init() {
    // Initialize logging for Android
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("papervlum"),
    );
    log::info!("Papervlum core initialized");
}