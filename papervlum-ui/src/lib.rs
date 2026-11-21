//! Papervlum UI - Shared UI components

pub mod toolbar;
pub mod ruler;
pub mod statusbar;
pub mod filepicker;
pub mod colorpicker;
pub mod fontpicker;
pub mod a11y;

use egui::{Context, Ui};

/// Shared UI components for all apps
pub struct PapervlumUI {
    ctx: Context,
}

impl PapervlumUI {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    pub fn toolbar(&mut self, ui: &mut Ui) {
        toolbar::show(ui);
    }

    pub fn ruler(&mut self, ui: &mut Ui) {
        ruler::show(ui);
    }

    pub fn statusbar(&mut self, ui: &mut Ui) {
        statusbar::show(ui);
    }
}