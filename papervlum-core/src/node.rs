//! Document node tree

use crate::style::StyleId;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Unique node identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

/// Node kind - the type of content this node represents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeKind {
    // Writer nodes
    Paragraph { runs: Vec<TextRun> },
    TextRun { text: String, style: StyleId },
    Table { rows: Vec<TableRow> },
    TableRow { cells: Vec<TableCell> },
    TableCell { content: Vec<NodeId> },
    Image { data: ImageData, layout: ImageLayout },
    PageBreak,
    SectionBreak { section_type: SectionType },
    Bookmark { name: String },
    Comment { author: String, text: String, date: chrono::DateTime<chrono::Utc> },

    // Calc nodes
    Sheet { name: String, rows: u32, cols: u32 },
    Cell { address: CellAddress, value: CellValue, style: StyleId },
    DefinedName { name: String, refers_to: String },

    // Impress nodes
    Slide { layout: SlideLayout },
    Shape { shape_type: ShapeType, geometry: ShapeGeometry },
    Placeholder { placeholder_type: PlaceholderType },
    Animation { timeline: AnimationTimeline },

    // PDF nodes
    PdfPage { page_index: u32 },
    Annotation { annot_type: AnnotationType, rect: Rect },
    FormField { field_type: FormFieldType, value: String },

    // Generic container
    Group { children: Vec<NodeId> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub style: StyleId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub height: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableCell {
    pub content: Vec<NodeId>,
    pub colspan: u16,
    pub rowspan: u16,
    pub width: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    pub mime_type: String,
    pub data: Vec<u8>, // In practice, this would be a reference to external storage
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionType {
    NextPage,
    Continuous,
    EvenPage,
    OddPage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellAddress {
    pub row: u32,
    pub col: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Boolean(bool),
    Error(CellError),
    Formula(String), // Stored as formula string, evaluated separately
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CellError {
    Div0,
    NA,
    Name,
    Null,
    Num,
    Ref,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlideLayout {
    Title,
    TitleAndContent,
    TwoContent,
    Comparison,
    Blank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeType {
    Rectangle,
    RoundedRectangle,
    Ellipse,
    Line,
    Arrow,
    TextBox,
    Picture,
    Chart,
    Table,
    Connector,
    Freeform,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeGeometry {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub flip_h: bool,
    pub flip_v: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaceholderType {
    Title,
    Body,
    CenteredTitle,
    Subtitle,
    DateTime,
    SlideNumber,
    Footer,
    Header,
    Picture,
    Chart,
    Table,
    Media,
    ClipArt,
    Diagram,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationTimeline {
    pub steps: Vec<AnimationStep>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationStep {
    pub target: NodeId,
    pub effect: AnimationEffect,
    pub trigger: AnimationTrigger,
    pub duration: f32,
    pub delay: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationEffect {
    FadeIn,
    FadeOut,
    FlyIn { direction: FlyDirection },
    FlyOut { direction: FlyDirection },
    ZoomIn,
    ZoomOut,
    Spin,
    Pulse,
    Custom { property: String, from: f32, to: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlyDirection {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationTrigger {
    OnClick,
    WithPrevious,
    AfterPrevious { delay: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationType {
    Highlight,
    Underline,
    StrikeOut,
    Squiggly,
    FreeText,
    Stamp,
    Ink,
    Popup,
    FileAttachment,
    Sound,
    Movie,
    Widget,
    Screen,
    PrinterMark,
    TrapNet,
    Watermark,
    ThreeD,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormFieldType {
    Text,
    Button,
    Choice,
    Signature,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Document node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub style: Option<StyleId>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Node {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            id: NodeId::new(),
            kind,
            parent: None,
            children: Vec::new(),
            style: None,
            properties: HashMap::new(),
        }
    }

    pub fn with_style(mut self, style: StyleId) -> Self {
        self.style = Some(style);
        self
    }
}