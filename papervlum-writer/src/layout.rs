//! Text layout engine for Writer

use crate::shaping::{ShapingEngine, FontId, ShapedText, FontWeight, FontStyle, shape_run, guess_script};
use papervlum_core::{Result, Alignment, LineSpacing};
use rustybuzz::{Direction, Script};

/// Layout result
#[derive(Debug, Default)]
pub struct LayoutResult {
    pub lines: Vec<Line>,
    pub pages: Vec<Page>,
    pub glyph_runs: Vec<GlyphRun>,
}

#[derive(Debug)]
pub struct Line {
    pub glyphs: Vec<GlyphInstance>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub range: TextRange,
}

#[derive(Debug)]
pub struct Page {
    pub lines: Vec<usize>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub margin: Margin,
}

#[derive(Debug, Clone, Copy)]
pub struct Margin {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self { top: 72.0, right: 72.0, bottom: 72.0, left: 72.0 }
    }
}

impl Clone for Margin {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for Margin {}

#[derive(Debug)]
pub struct GlyphRun {
    pub glyphs: Vec<rustybuzz::GlyphInfo>,
    pub font: FontRef,
    pub color: [f32; 4],
    pub range: TextRange,
}

#[derive(Debug, Clone, Copy)]
pub struct GlyphInstance {
    pub glyph_id: u32,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
    pub cluster: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontRef(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

/// Layout a paragraph (stub - to be implemented)
pub fn layout_paragraph(
    _engine: &mut ShapingEngine,
    _text: &str,
    _font_family: &str,
    _font_size: f32,
    _font_weight: FontWeight,
    _font_style: FontStyle,
    _color: [f32; 4],
    _max_width: f32,
    _alignment: Alignment,
    _direction: Direction,
    _indent_left: f32,
    _indent_right: f32,
    _first_line_indent: f32,
    _line_spacing: LineSpacing,
) -> Result<LayoutResult> {
    // TODO: Implement full paragraph layout
    Ok(LayoutResult::default())
}

/// Calculate line height from font size and line spacing
fn calculate_line_height(font_size: f32, spacing: &LineSpacing) -> f32 {
    match spacing {
        LineSpacing::Single => font_size * 1.2,
        LineSpacing::OneAndHalf => font_size * 1.5,
        LineSpacing::Double => font_size * 2.0,
        LineSpacing::AtLeast(pts) => font_size * 1.2.max(*pts),
        LineSpacing::Exactly(pts) => *pts,
        LineSpacing::Multiple(mult) => font_size * mult,
    }
}

/// Perform bidirectional reordering
pub fn apply_bidi_reordering(text: &str, base_dir: Direction) -> Vec<(String, Direction)> {
    // Simplified bidi - just return the whole text as one run
    vec![(text.to_string(), base_dir)]
}

/// Break shaped text into lines using UAX #14
pub fn break_lines(shaped: &ShapedText, _max_width: f32) -> Result<Vec<TextRange>> {
    // TODO: Implement proper line breaking with unicode-linebreak
    if shaped.glyphs.is_empty() {
        return Ok(Vec::new());
    }
    Ok(vec![TextRange { start: 0, end: shaped.glyphs.len() }])
}

/// Input for paragraph layout
pub struct ParagraphLayoutInput {
    pub text: String,
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub color: [f32; 4],
    pub alignment: Alignment,
    pub direction: Direction,
    pub indent_left: f32,
    pub indent_right: f32,
    pub first_line_indent: f32,
    pub line_spacing: LineSpacing,
}

/// Simple table layout (placeholder)
pub fn layout_table(
    _engine: &mut ShapingEngine,
    _rows: u32,
    _cols: u32,
    _cell_widths: &[f32],
    _cell_heights: &[f32],
) -> Result<LayoutResult> {
    Ok(LayoutResult::default())
}