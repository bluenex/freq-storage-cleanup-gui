use std::time::Duration;

use eframe::egui;

use super::state::{App, Screen};

impl App {
    pub fn start_snapshots(&mut self) {
        self.free_before_clean.get_or_insert(self.free.unwrap_or(0));
        self.snapshots = None;
        self.delete_dataless = false;
        self.worker.list_snapshots();
        self.screen = Screen::Snapshots;
    }

    pub fn rescan_after_clean(&mut self) {
        self.screen = Screen::Rescanning;
        self.worker.scan(Duration::from_secs(2));
    }

    fn snapshots_to_delete(&self) -> Vec<String> {
        let Some(Ok(list)) = &self.snapshots else {
            return Vec::new();
        };
        list.iter()
            .filter(|s| !s.dataless || self.delete_dataless)
            .map(|s| s.date.clone())
            .collect()
    }

    pub fn show_snapshots(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Time Machine local snapshots");
            ui.label("Snapshots taken before the cleanup still hold the deleted files. Space comes back only after they are deleted.");
            ui.separator();

            match &self.snapshots {
                None => {
                    ui.spinner();
                }
                Some(Err(e)) => {
                    ui.colored_label(egui::Color32::RED, format!("Can't list snapshots: {e}"));
                }
                Some(Ok(list)) => {
                    ui.label(format!("{} snapshot(s), size unknown without root.", list.len()));
                    for s in list {
                        ui.monospace(if s.dataless { format!("{} (dataless)", s.date) } else { s.date.clone() });
                    }
                    if list.iter().any(|s| s.dataless) {
                        ui.checkbox(&mut self.delete_dataless, "Also delete the (dataless) snapshot")
                            .on_hover_text("It marks the last external backup. Deleting it makes the next backup rescan the whole disk.");
                    }
                }
            }

            ui.separator();
            ui.horizontal(|ui| {
                let dates = self.snapshots_to_delete();
                let ready = !self.deleting_snapshots && self.snapshots.is_some();
                if ui.add_enabled(ready && !dates.is_empty(), egui::Button::new(format!("Delete {} snapshot(s)", dates.len()))).clicked() {
                    self.deleting_snapshots = true;
                    self.worker.delete_snapshots(dates);
                }
                if ui.add_enabled(ready, egui::Button::new("Skip")).clicked() {
                    self.rescan_after_clean();
                }
            });
            ui.separator();
            self.show_log(ui);
        });
    }
}
