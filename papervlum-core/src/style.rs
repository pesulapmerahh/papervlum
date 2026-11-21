//! Style system

use crate::node::NodeId;
use anyhow::Result;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Style identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StyleId(pub u64);

impl StyleId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for StyleId {
    fn default() -> Self {
        Self::new()
    }
}

/// Style property value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleProperty {
    // Font properties
    FontFamily(String),
    FontSize(f32),
    FontWeight(FontWeight),
    FontStyle(FontStyle),
    FontVariant(FontVariant),
    FontColor([f32; 4]), // RGBA
    FontUnderline(UnderlineStyle),
    FontStrikethrough(bool),
    FontSuperscript(bool),
    FontSubscript(bool),
    FontKerning(bool),
    FontLigatures(bool),
    FontLetterSpacing(f32),

    // Paragraph properties
    Alignment(Alignment),
    LineSpacing(LineSpacing),
    SpaceBefore(f32),
    SpaceAfter(f32),
    IndentationLeft(f32),
    IndentationRight(f32),
    IndentationFirstLine(f32),
    TabStops(Vec<TabStop>),
    KeepWithNext(bool),
    KeepLinesTogether(bool),
    PageBreakBefore(bool),
    WidowControl(bool),
    TextDirection(TextDirection),

    // Border properties
    BorderTop(Border),
    BorderBottom(Border),
    BorderLeft(Border),
    BorderRight(Border),
    BorderInsideH(Border),
    BorderInsideV(Border),

    // Fill properties
    FillColor([f32; 4]),
    FillPattern(FillPattern),

    // Table properties
    TableWidth(TableWidth),
    TableAlignment(Alignment),
    TableIndent(f32),
    CellMargin(CellMargin),
    CellVerticalAlignment(VerticalAlignment),

    // Numbering
    NumberingLevel(u8),
    NumberingFormat(NumberingFormat),

    // Layout
    Width(Length),
    Height(Length),
    Margin(Margin),
    Padding(Padding),
    Display(Display),
    Position(Position),
    ZIndex(i32),
    Transform(Transform),
    Opacity(f32),

    // Custom
    Custom(String, serde_json::Value),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FontVariant {
    Normal,
    SmallCaps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnderlineStyle {
    None,
    Single,
    Double,
    Thick,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Wave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
    Distributed,
    Start,
    End,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LineSpacing {
    Single,
    OneAndHalf,
    Double,
    AtLeast(f32),
    Exactly(f32),
    Multiple(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TabStop {
    pub position: f32,
    pub alignment: TabAlignment,
    pub leader: TabLeader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabAlignment {
    Left,
    Center,
    Right,
    Decimal,
    Bar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabLeader {
    None,
    Dots,
    Dashes,
    Line,
    HeavyLine,
    MiddleDot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextDirection {
    Ltr,
    Rtl,
    TbRl,
    BtLr,
    LrTb,
    RlTb,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub style: BorderStyle,
    pub width: f32,
    pub color: [f32; 4],
    pub spacing: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BorderStyle {
    None,
    Single,
    Double,
    Thick,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Wave,
    Inset,
    Outset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FillPattern {
    None,
    Solid,
    Gradient(Gradient),
    Pattern(PatternType),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Gradient {
    pub stops: Vec<GradientStop>,
    pub angle: f32,
    pub center: (f32, f32),
    pub radius: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GradientStop {
    pub position: f32,
    pub color: [f32; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
    Percent5,
    Percent10,
    Percent20,
    Percent25,
    Percent30,
    Percent40,
    Percent50,
    Percent60,
    Percent70,
    Percent75,
    Percent80,
    Percent90,
    DarkHorizontal,
    DarkVertical,
    DarkDownwardDiagonal,
    DarkUpwardDiagonal,
    SmallCheckerBoard,
    Trellis,
    LightHorizontal,
    LightVertical,
    LightDownwardDiagonal,
    LightUpwardDiagonal,
    SmallGrid,
    DottedDiamond,
    WideDownwardDiagonal,
    WideUpwardDiagonal,
    DashedUpwardDiagonal,
    DashedDownwardDiagonal,
    NarrowVertical,
    NarrowHorizontal,
    DarkVertical,
    DarkHorizontal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableWidth {
    Auto,
    Fixed(f32),
    Percent(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    Baseline,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellMargin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NumberingFormat {
    Decimal,
    UpperRoman,
    LowerRoman,
    UpperLetter,
    LowerLetter,
    Bullet,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Length {
    Auto,
    Points(f32),
    Percent(f32),
    Em(f32),
    Rem(f32),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Margin {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub top: Length,
    pub right: Length,
    pub bottom: Length,
    pub left: Length,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Display {
    Block,
    Inline,
    InlineBlock,
    Flex,
    Grid,
    Table,
    TableRow,
    TableCell,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub matrix: [f32; 6], // a, b, c, d, e, f
}

/// Style definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub id: StyleId,
    pub name: String,
    pub parent: Option<StyleId>,
    pub based_on: Option<StyleId>,
    pub next: Option<StyleId>,
    pub properties: IndexMap<String, StyleProperty>,
    pub style_type: StyleType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StyleType {
    Paragraph,
    Character,
    Table,
    List,
    Cell,
}

impl Style {
    pub fn new(name: String, style_type: StyleType) -> Self {
        Self {
            id: StyleId::new(),
            name,
            parent: None,
            based_on: None,
            next: None,
            properties: IndexMap::new(),
            style_type,
        }
    }

    pub fn set_property(&mut self, name: String, value: StyleProperty) {
        self.properties.insert(name, value);
    }

    pub fn get_property(&self, name: &str) -> Option<&StyleProperty> {
        self.properties.get(name)
    }

    /// Resolve property with inheritance
    pub fn resolve_property<'a>(&'a self, name: &str, sheet: &'a StyleSheet) -> Option<&'a StyleProperty> {
        if let Some(prop) = self.properties.get(name) {
            return Some(prop);
        }
        if let Some(parent_id) = self.parent {
            if let Some(parent) = sheet.get(parent_id) {
                return parent.resolve_property(name, sheet);
            }
        }
        None
    }
}

/// Style sheet - collection of styles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleSheet {
    styles: IndexMap<StyleId, Style>,
    default_paragraph: Option<StyleId>,
    default_character: Option<StyleId>,
    default_table: Option<StyleId>,
    default_list: Option<StyleId>,
}

impl StyleSheet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, style: Style) -> StyleId {
        let id = style.id;
        self.styles.insert(id, style);
        id
    }

    pub fn get(&self, id: StyleId) -> Option<&Style> {
        self.styles.get(&id)
    }

    pub fn get_mut(&mut self, id: StyleId) -> Option<&mut Style> {
        self.styles.get_mut(&id)
    }

    pub fn remove(&mut self, id: StyleId) -> Option<Style> {
        self.styles.swap_remove(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&StyleId, &Style)> {
        self.styles.iter()
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Style> {
        self.styles.values().find(|s| s.name == name)
    }

    pub fn set_default_paragraph(&mut self, id: StyleId) {
        self.default_paragraph = Some(id);
    }

    pub fn default_paragraph(&self) -> Option<StyleId> {
        self.default_paragraph
    }

    pub fn set_default_character(&mut self, id: StyleId) {
        self.default_character = Some(id);
    }

    pub fn default_character(&self) -> Option<StyleId> {
        self.default_character
    }

    pub fn set_default_table(&mut self, id: StyleId) {
        self.default_table = Some(id);
    }

    pub fn default_table(&self) -> Option<StyleId> {
        self.default_table
    }

    pub fn set_default_list(&mut self, id: StyleId) {
        self.default_list = Some(id);
    }

    pub fn default_list(&self) -> Option<StyleId> {
        self.default_list
    }

    /// Resolve a property for a style, following inheritance chain
    pub fn resolve(&self, style_id: StyleId, property: &str) -> Option<&StyleProperty> {
        let mut current = Some(style_id);
        while let Some(id) = current {
            if let Some(style) = self.styles.get(&id) {
                if let Some(prop) = style.properties.get(property) {
                    return Some(prop);
                }
                current = style.parent;
            } else {
                break;
            }
        }
        None
    }
}