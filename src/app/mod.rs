mod format_size;
mod groups;
mod palette;
mod size_bar;
mod state;
mod view_confirm;
mod view_disk_bar;
mod view_done;
mod view_log;
mod view_select;
mod view_snapshots;

use std::time::Duration;

use eframe::egui;

use crate::worker::{Msg, Worker};
use state::Screen;

pub use state::App;

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let worker = Worker::new(cc.egui_ctx.clone());
        worker.scan(Duration::ZERO);
        Self {
            worker,
            screen: Screen::Scanning,
            items: Vec::new(),
            disk: None,
            free_before_clean: None,
            plan: Vec::new(),
            log: Vec::new(),
            snapshots: None,
            delete_reference: false,
            deleting_snapshots: false,
        }
    }

    pub fn free(&self) -> Option<u64> {
        self.disk.map(|d| d.free)
    }

    fn handle(&mut self, msg: Msg) {
        match msg {
            Msg::Scanned(result) => {
                self.items = result.items;
                self.disk = result.disk;
                self.screen = if self.screen == Screen::Rescanning {
                    Screen::Done
                } else {
                    Screen::Select
                };
            }
            Msg::Log(line) => self.log.push(line),
            Msg::CleanDone => self.start_snapshots(),
            Msg::Snapshots(result) => self.snapshots = Some(result),
            Msg::SnapshotsDeleted => {
                self.deleting_snapshots = false;
                self.rescan_after_clean();
            }
        }
    }

    fn show_top(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("top").show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() - 80.0);
                    self.show_disk_bar(ui);
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(self.screen == Screen::Select, egui::Button::new("Re-scan"))
                        .clicked()
                    {
                        self.screen = Screen::Scanning;
                        self.worker.scan(Duration::ZERO);
                    }
                });
            });
            ui.add_space(6.0);
        });
    }

    fn show_busy(&self, ui: &mut egui::Ui, text: &str) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spinner();
                ui.label(text);
            });
            if !self.log.is_empty() {
                ui.separator();
                self.show_log(ui);
            }
        });
    }
}

impl eframe::App for App {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.worker.rx.try_recv() {
            self.handle(msg);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show_top(ui);
        match self.screen {
            Screen::Scanning => self.show_busy(ui, "Scanning…"),
            Screen::Select => self.show_select(ui),
            Screen::Confirm => self.show_confirm(ui),
            Screen::Cleaning => self.show_busy(ui, "Cleaning…"),
            Screen::Snapshots => self.show_snapshots(ui),
            Screen::Rescanning => self.show_busy(ui, "Re-scanning…"),
            Screen::Done => self.show_done(ui),
        }
    }
}
