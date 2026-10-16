use eframe::egui::{self, Color32, Sense, vec2};

/// A thin bar filled to `fraction` in `color`.
pub fn size_bar(ui: &mut egui::Ui, fraction: f32, color: Color32) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(220.0, 8.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 4.0, ui.visuals().faint_bg_color);
    let mut filled = rect;
    filled.set_width((rect.width() * fraction).max(3.0));
    painter.rect_filled(filled, 4.0, color);
    response
}
