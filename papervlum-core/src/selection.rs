//! Selection and cursor handling

use crate::node::NodeId;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Text cursor position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    pub node: NodeId,
    pub offset: u32, // Character offset within the node
    pub affinity: CursorAffinity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CursorAffinity {
    Upstream,  // Prefer position before the character
    Downstream, // Prefer position after the character
}

impl Cursor {
    pub fn new(node: NodeId, offset: u32) -> Self {
        Self {
            node,
            offset,
            affinity: CursorAffinity::Downstream,
        }
    }

    pub fn with_affinity(mut self, affinity: CursorAffinity) -> Self {
        self.affinity = affinity;
        self
    }
}

/// Selection range - from anchor to focus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionRange {
    pub anchor: Cursor,
    pub focus: Cursor,
}

impl SelectionRange {
    pub fn new(anchor: Cursor, focus: Cursor) -> Self {
        Self { anchor, focus }
    }

    pub fn collapsed(&self) -> bool {
        self.anchor == self.focus
    }

    pub fn is_forward(&self) -> bool {
        // For simplicity, compare node IDs - in reality this would need tree position
        self.anchor.node.0 <= self.focus.node.0 &&
        (self.anchor.node.0 != self.focus.node.0 || self.anchor.offset <= self.focus.offset)
    }

    pub fn normalize(&self) -> Self {
        if self.is_forward() {
            self.clone()
        } else {
            Self {
                anchor: self.focus,
                focus: self.anchor,
            }
        }
    }
}

/// Selection - can be multiple ranges (for multi-selection)
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Selection {
    pub ranges: Vec<SelectionRange>,
    pub primary: usize, // Index of primary range
}

impl Selection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn collapsed(&self) -> bool {
        self.ranges.len() == 1 && self.ranges[0].collapsed()
    }

    pub fn single_range(&self) -> Option<&SelectionRange> {
        if self.ranges.len() == 1 {
            Some(&self.ranges[0])
        } else {
            None
        }
    }

    pub fn set_single(&mut self, range: SelectionRange) {
        self.ranges = vec![range];
        self.primary = 0;
    }

    pub fn add_range(&mut self, range: SelectionRange) {
        self.ranges.push(range);
        self.primary = self.ranges.len() - 1;
    }

    pub fn clear(&mut self) {
        self.ranges.clear();
        self.primary = 0;
    }

    pub fn primary_range(&self) -> Option<&SelectionRange> {
        self.ranges.get(self.primary)
    }

    pub fn primary_range_mut(&mut self) -> Option<&mut SelectionRange> {
        self.ranges.get_mut(self.primary)
    }
}

/// Table selection (for spreadsheets)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableSelection {
    pub anchor: CellAddress,
    pub focus: CellAddress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellAddress {
    pub sheet: u32,
    pub row: u32,
    pub col: u32,
}

impl TableSelection {
    pub fn new(anchor: CellAddress, focus: CellAddress) -> Self {
        Self { anchor, focus }
    }

    pub fn rows(&self) -> (u32, u32) {
        let start = self.anchor.row.min(self.focus.row);
        let end = self.anchor.row.max(self.focus.row);
        (start, end)
    }

    pub fn cols(&self) -> (u32, u32) {
        let start = self.anchor.col.min(self.focus.col);
        let end = self.anchor.col.max(self.focus.col);
        (start, end)
    }

    pub fn contains(&self, cell: CellAddress) -> bool {
        let (r1, r2) = self.rows();
        let (c1, c2) = self.cols();
        cell.sheet == self.anchor.sheet && cell.row >= r1 && cell.row <= r2 && cell.col >= c1 && cell.col <= c2
    }

    pub fn normalize(&self) -> Self {
        let (r1, r2) = self.rows();
        let (c1, c2) = self.cols();
        Self {
            anchor: CellAddress { sheet: self.anchor.sheet, row: r1, col: c1 },
            focus: CellAddress { sheet: self.anchor.sheet, row: r2, col: c2 },
        }
    }
}

/// Shape selection (for presentations)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeSelection {
    pub shapes: Vec<NodeId>,
    pub primary: Option<NodeId>,
}

impl ShapeSelection {
    pub fn new() -> Self {
        Self { shapes: Vec::new(), primary: None }
    }

    pub fn single(shape: NodeId) -> Self {
        Self { shapes: vec![shape], primary: Some(shape) }
    }

    pub fn add(&mut self, shape: NodeId) {
        if !self.shapes.contains(&shape) {
            self.shapes.push(shape);
            if self.primary.is_none() {
                self.primary = Some(shape);
            }
        }
    }

    pub fn remove(&mut self, shape: NodeId) {
        self.shapes.retain(|s| *s != shape);
        if self.primary == Some(shape) {
            self.primary = self.shapes.first().copied();
        }
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
        self.primary = None;
    }
}