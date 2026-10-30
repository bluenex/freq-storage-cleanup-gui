use eframe::egui;

/// The width a checkbox with this label takes, so rows in different groups can share columns.
pub fn checkbox_width(ui: &egui::Ui, label: &str) -> f32 {
    let galley = egui::WidgetText::from(label).into_galley(
        ui,
        Some(egui::TextWrapMode::Extend),
        f32::INFINITY,
        egui::TextStyle::Button,
    );
    let spacing = ui.spacing();
    galley.size().x + spacing.icon_width + spacing.icon_spacing + 2.0 * spacing.button_padding.x
}
