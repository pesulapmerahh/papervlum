//! Ruler UI component

use egui::{Ui, Vec2, Pos2, Color32};

pub fn show(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 24.0), egui::Sense::hover());
    let painter = ui.painter();

    // Draw ruler background
    painter.rect_filled(rect, 0.0, Color32::from_gray(240));
    painter.rect_stroke(rect, 0.0, (1.0, Color32::from_gray(200)));

    // Draw tick marks
    let dpi = ui.ctx().pixels_per_point();
    let start = rect.left();
    let end = rect.right();
    let major_interval = 72.0 * dpi; // 1 inch in points
    let minor_interval = major_interval / 8.0;

    let mut x = start;
    while x < end {
        let is_major = ((x - start) / major_interval).fract() < 0.001;
        let tick_height = if is_major { 16.0 } else { 8.0 };
        painter.line_segment([
            Pos2::new(x, rect.bottom() - tick_height),
            Pos2::new(x, rect.bottom()),
        ], (1.0, Color32::from_gray(100)));
        x += minor_interval;
    }
}