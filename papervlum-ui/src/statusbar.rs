//! Status bar UI component

use egui::{Ui};

pub fn show(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Ready");
        ui.separator();
        ui.label("Page 1 of 1");
        ui.separator();
        ui.label("Words: 0");
        ui.separator();
        ui.label("Characters: 0");
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label("100%");
            ui.separator();
            ui.add(egui::Slider::new(&mut 100.0, 10.0..=500.0).text("Zoom"));
        });
    });
}