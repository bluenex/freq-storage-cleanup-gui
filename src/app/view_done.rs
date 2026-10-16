use eframe::egui::{self, RichText};

use super::format_size::format_size;
use super::state::{App, Screen};

impl App {
    pub fn show_done(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            let freed = match (self.free_before_clean, self.free()) {
                (Some(before), Some(after)) => format_size(after.saturating_sub(before)),
                _ => "unknown".to_owned(),
            };
            ui.heading(RichText::new(format!("Freed {freed}")).strong());
            ui.label("APFS can take a moment to report freed space. Re-scan if the number looks low.");
            ui.horizontal(|ui| {
                if ui.button("Back to list").clicked() {
                    self.free_before_clean = None;
                    self.screen = Screen::Select;
                }
                if ui.button("Re-scan").clicked() {
                    self.rescan_after_clean();
                }
            });
            ui.separator();
            self.show_log(ui);
        });
    }
}
