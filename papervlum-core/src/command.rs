//! Command pattern for undo/redo

use anyhow::Result;
use std::any::Any;

/// Command trait - all editing operations implement this
pub trait Command: Send + Sync {
    /// Execute the command
    fn execute(&mut self, document: &mut dyn crate::document::Document) -> Result<()>;

    /// Undo the command
    fn undo(&mut self, document: &mut dyn crate::document::Document) -> Result<()>;

    /// Human-readable description for UI
    fn description(&self) -> &str;

    /// Whether this command can be coalesced with the previous one
    fn can_coalesce(&self, _other: &dyn Command) -> bool {
        false
    }

    /// Coalesce this command with another (for rapid typing, etc.)
    fn coalesce(&mut self, _other: &dyn Command) -> Option<Box<dyn Command>> {
        None
    }

    /// Get command ID for grouping
    fn group_id(&self) -> Option<u64> {
        None
    }

    /// Downcast support
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// No-op command (for testing)
pub struct NoopCommand;

impl Command for NoopCommand {
    fn execute(&mut self, _document: &mut dyn crate::document::Document) -> Result<()> {
        Ok(())
    }

    fn undo(&mut self, _document: &mut dyn crate::document::Document) -> Result<()> {
        Ok(())
    }

    fn description(&self) -> &str {
        "No operation"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Macro for implementing common command boilerplate
#[macro_export]
macro_rules! impl_command {
    ($name:ident) => {
        impl $crate::command::Command for $name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
}