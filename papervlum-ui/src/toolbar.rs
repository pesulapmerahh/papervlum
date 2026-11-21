//! Toolbar UI component

use egui::{Ui, Button, ComboBox, Id};

pub fn show(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Papervlum");
        ui.separator();
        if ui.button("New").clicked() {}
        if ui.button("Open").clicked() {}
        if ui.button("Save").clicked() {}
        ui.separator();
        if ui.button("Undo").clicked() {}
        if ui.button("Redo").clicked() {}
        ui.separator();
        ui.label("Font:");
        ComboBox::new(Id::new("font"), "Default").show_ui(ui, |ui| {
            ui.selectable_value(&mut "Default", "Default", "Default");
        });
        ComboBox::new(Id::new("size"), "12").show_ui(ui, |ui| {
            ui.selectable_value(&mut "12", "12", "12");
        });
        if ui.button("B").clicked() {}
        if ui.button("I").clicked() {}
        if ui.button("U").clicked() {}
    });
}