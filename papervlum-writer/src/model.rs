//! OOXML document model for Writer

use papervlum_core::Result;

/// Document model parsed from OOXML
#[derive(Debug, Default)]
pub struct DocumentModel {
    pub body: Body,
    pub styles: Styles,
    pub numbering: Numbering,
}

#[derive(Debug, Default)]
pub struct Body {
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub enum Block {
    Paragraph(Paragraph),
    Table(Table),
    PageBreak,
    SectionBreak(SectionBreak),
}

#[derive(Debug)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub style: Option<String>,
    pub properties: ParagraphProperties,
}

#[derive(Debug)]
pub struct Run {
    pub text: String,
    pub style: Option<String>,
    pub properties: RunProperties,
}

#[derive(Debug, Default)]
pub struct ParagraphProperties {
    pub alignment: Option<papervlum_core::Alignment>,
    pub indentation: Option<Indentation>,
    pub spacing: Option<Spacing>,
    pub tabs: Vec<TabStop>,
    pub keep_with_next: bool,
    pub keep_lines: bool,
    pub page_break_before: bool,
    pub widow_control: bool,
    pub text_direction: Option<papervlum_core::TextDirection>,
}

#[derive(Debug, Default)]
pub struct RunProperties {
    pub bold: bool,
    pub italic: bool,
    pub underline: Option<papervlum_core::UnderlineStyle>,
    pub strikethrough: bool,
    pub font_size: Option<f32>,
    pub font_family: Option<String>,
    pub color: Option<[f32; 4]>,
    pub superscript: bool,
    pub subscript: bool,
    pub kerning: bool,
    pub ligatures: bool,
}

#[derive(Debug, Default)]
pub struct Indentation {
    pub left: Option<f32>,
    pub right: Option<f32>,
    pub first_line: Option<f32>,
    pub hanging: Option<f32>,
}

#[derive(Debug, Default)]
pub struct Spacing {
    pub before: Option<f32>,
    pub after: Option<f32>,
    pub line: Option<papervlum_core::LineSpacing>,
}

#[derive(Debug)]
pub struct TabStop {
    pub position: f32,
    pub alignment: papervlum_core::TabAlignment,
    pub leader: papervlum_core::TabLeader,
}

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<TableRow>,
    pub properties: TableProperties,
}

#[derive(Debug)]
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub height: Option<f32>,
}

#[derive(Debug)]
pub struct TableCell {
    pub content: Vec<Block>,
    pub colspan: u16,
    pub rowspan: u16,
    pub width: Option<f32>,
    pub properties: CellProperties,
}

#[derive(Debug, Default)]
pub struct TableProperties {
    pub width: Option<papervlum_core::TableWidth>,
    pub alignment: Option<papervlum_core::Alignment>,
    pub indent: Option<f32>,
    pub borders: TableBorders,
    pub cell_margin: Option<papervlum_core::CellMargin>,
}

#[derive(Debug, Default)]
pub struct TableBorders {
    pub top: Option<papervlum_core::Border>,
    pub bottom: Option<papervlum_core::Border>,
    pub left: Option<papervlum_core::Border>,
    pub right: Option<papervlum_core::Border>,
    pub inside_h: Option<papervlum_core::Border>,
    pub inside_v: Option<papervlum_core::Border>,
}

#[derive(Debug, Default)]
pub struct CellProperties {
    pub vertical_align: Option<papervlum_core::VerticalAlignment>,
    pub margins: Option<papervlum_core::CellMargin>,
    pub borders: TableBorders,
    pub shading: Option<papervlum_core::FillPattern>,
}

#[derive(Debug)]
pub struct SectionBreak {
    pub section_type: papervlum_core::SectionType,
}

#[derive(Debug, Default)]
pub struct Styles {
    pub paragraph_styles: std::collections::HashMap<String, ParagraphStyle>,
    pub character_styles: std::collections::HashMap<String, CharacterStyle>,
    pub table_styles: std::collections::HashMap<String, TableStyle>,
    pub numbering_styles: std::collections::HashMap<String, NumberingStyle>,
}

#[derive(Debug)]
pub struct ParagraphStyle {
    pub name: String,
    pub based_on: Option<String>,
    pub next: Option<String>,
    pub properties: ParagraphProperties,
    pub run_properties: RunProperties,
}

#[derive(Debug)]
pub struct CharacterStyle {
    pub name: String,
    pub based_on: Option<String>,
    pub properties: RunProperties,
}

#[derive(Debug)]
pub struct TableStyle {
    pub name: String,
    pub based_on: Option<String>,
    pub properties: TableProperties,
}

#[derive(Debug)]
pub struct NumberingStyle {
    pub name: String,
    pub levels: Vec<NumberingLevel>,
}

#[derive(Debug)]
pub struct NumberingLevel {
    pub level: u8,
    pub format: papervlum_core::NumberingFormat,
    pub text: String,
    pub alignment: papervlum_core::Alignment,
    pub properties: ParagraphProperties,
    pub run_properties: RunProperties,
}

#[derive(Debug, Default)]
pub struct Numbering {
    pub abstract_num: std::collections::HashMap<String, AbstractNum>,
    pub num_instances: std::collections::HashMap<u32, NumInstance>,
}

#[derive(Debug)]
pub struct AbstractNum {
    pub id: String,
    pub name: String,
    pub levels: Vec<NumberingLevel>,
}

#[derive(Debug)]
pub struct NumInstance {
    pub abstract_num_id: String,
    pub overrides: Vec<LevelOverride>,
}

#[derive(Debug)]
pub struct LevelOverride {
    pub level: u8,
    pub format: Option<papervlum_core::NumberingFormat>,
    pub text: Option<String>,
}

/// Parse OOXML document.xml into DocumentModel
pub fn parse_document_xml(xml: &str) -> Result<DocumentModel> {
    // TODO: Implement actual XML parsing with quick-xml
    Ok(DocumentModel::default())
}