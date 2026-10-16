use eframe::egui::{self, RichText};

use super::format_size::format_size;
use super::state::{App, Screen};

impl App {
    pub fn show_confirm(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("confirm_actions").show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    self.screen = Screen::Select;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let button =
                        egui::Button::new(RichText::new("Delete").strong()).fill(egui::Color32::from_rgb(170, 40, 40));
                    if ui.add(button).clicked() {
                        self.free_before_clean = self.free;
                        self.log.clear();
                        self.worker.clean(std::mem::take(&mut self.plan));
                        self.screen = Screen::Cleaning;
                    }
                });
            });
            ui.add_space(6.0);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading(format!("About to free {}", format_size(self.selected_size())));
            ui.label("These steps will run. Files are deleted, not moved to the Trash.");
            ui.separator();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for step in &self.plan {
                    ui.monospace(step.to_string());
                }
            });
        });
    }
}
