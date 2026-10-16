use eframe::egui;

use super::state::App;

impl App {
    pub fn show_log(&self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .auto_shrink(false)
            .show(ui, |ui| {
                for line in &self.log {
                    ui.monospace(line);
                }
            });
    }
}
