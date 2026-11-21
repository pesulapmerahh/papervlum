//! Undo/Redo stack

use crate::command::Command;
use anyhow::Result;
use std::collections::VecDeque;

/// Maximum undo stack size
const MAX_UNDO_STACK: usize = 1000;

/// Undo stack with redo support
#[derive(Default)]
pub struct UndoStack {
    undo_stack: VecDeque<Box<dyn Command>>,
    redo_stack: VecDeque<Box<dyn Command>>,
    /// Commands being coalesced (for rapid typing)
    coalescing: Option<Box<dyn Command>>,
    /// Whether we're currently undoing/redoing (to avoid re-recording)
    in_replay: bool,
}

impl UndoStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a command onto the undo stack
    pub fn push(&mut self, command: Box<dyn Command>) -> Result<()> {
        if self.in_replay {
            return Ok(());
        }

        // Try to coalesce with previous command
        if let Some(mut prev) = self.coalescing.take() {
            if prev.can_coalesce(command.as_ref()) {
                if let Some(coalesced) = prev.coalesce(command) {
                    self.coalescing = Some(coalesced);
                    return Ok(());
                }
            }
            // Couldn't coalesce, push the previous command
            self.commit_coalescing();
        }

        // Start new coalescing group if this command can be coalesced
        if command.group_id().is_some() {
            self.coalescing = Some(command);
        } else {
            self.commit_command(command);
        }
        Ok(())
    }

    /// Commit the current coalescing command
    fn commit_coalescing(&mut self) {
        if let Some(cmd) = self.coalescing.take() {
            self.commit_command(cmd);
        }
    }

    /// Actually commit a command to the undo stack
    fn commit_command(&mut self, command: Box<dyn Command>) {
        self.undo_stack.push_back(command);
        if self.undo_stack.len() > MAX_UNDO_STACK {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear();
    }

    /// Undo the last command
    pub fn undo(&mut self, document: &mut dyn crate::document::Document) -> Result<bool> {
        self.commit_coalescing();

        if let Some(mut command) = self.undo_stack.pop_back() {
            self.in_replay = true;
            command.undo(document)?;
            self.in_replay = false;
            self.redo_stack.push_back(command);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Redo the last undone command
    pub fn redo(&mut self, document: &mut dyn crate::document::Document) -> Result<bool> {
        if let Some(mut command) = self.redo_stack.pop_back() {
            self.in_replay = true;
            command.execute(document)?;
            self.in_replay = false;
            self.undo_stack.push_back(command);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Check if undo is available
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty() || self.coalescing.is_some()
    }

    /// Check if redo is available
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Get undo description
    pub fn undo_description(&self) -> Option<&str> {
        self.undo_stack.back().map(|c| c.description())
    }

    /// Get redo description
    pub fn redo_description(&self) -> Option<&str> {
        self.redo_stack.back().map(|c| c.description())
    }

    /// Clear the stacks
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.coalescing = None;
    }

    /// Get undo count
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len() + if self.coalescing.is_some() { 1 } else { 0 }
    }

    /// Get redo count
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Flush any pending coalescing command
    pub fn flush(&mut self) {
        self.commit_coalescing();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::NoopCommand;
    use crate::document::Document;

    struct MockDocument;

    impl Document for MockDocument {
        fn load(_path: &std::path::Path) -> anyhow::Result<Self> where Self: Sized {
            Ok(Self)
        }
        fn save(&self, _path: &std::path::Path) -> anyhow::Result<()> { Ok(()) }
        fn metadata(&self) -> &crate::document::DocumentMetadata { unimplemented!() }
        fn metadata_mut(&mut self) -> &mut crate::document::DocumentMetadata { unimplemented!() }
        fn root(&self) -> &crate::node::Node { unimplemented!() }
        fn root_mut(&mut self) -> &mut crate::node::Node { unimplemented!() }
        fn styles(&self) -> &crate::style::StyleSheet { unimplemented!() }
        fn styles_mut(&mut self) -> &mut crate::style::StyleSheet { unimplemented!() }
        fn selection(&self) -> &crate::selection::Selection { unimplemented!() }
        fn set_selection(&mut self, _selection: crate::selection::Selection) { unimplemented!() }
        fn undo_stack(&self) -> &UndoStack { unimplemented!() }
        fn undo_stack_mut(&mut self) -> &mut UndoStack { unimplemented!() }
        fn execute(&mut self, _command: Box<dyn Command>) -> anyhow::Result<()> { Ok(()) }
        fn render(&self, _viewport: crate::document::RenderViewport) -> crate::document::RenderCommands { Default::default() }
        fn format(&self) -> crate::document::DocumentFormat { crate::document::DocumentFormat::Docx }
    }

    #[test]
    fn test_undo_redo() {
        let mut stack = UndoStack::new();
        let mut doc = MockDocument;

        // Push a command
        stack.push(Box::new(NoopCommand)).unwrap();
        assert_eq!(stack.undo_count(), 1);

        // Undo
        assert!(stack.undo(&mut doc).unwrap());
        assert_eq!(stack.undo_count(), 0);
        assert_eq!(stack.redo_count(), 1);

        // Redo
        assert!(stack.redo(&mut doc).unwrap());
        assert_eq!(stack.undo_count(), 1);
        assert_eq!(stack.redo_count(), 0);
    }

    #[test]
    fn test_coalesce() {
        let mut stack = UndoStack::new();
        let mut doc = MockDocument;

        // This would require a real command with group_id
        // For now just test the flush mechanism
        stack.flush();
        assert_eq!(stack.undo_count(), 0);
    }
}