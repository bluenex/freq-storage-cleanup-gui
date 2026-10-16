use eframe::egui::{self, ProgressBar, RichText};

use super::format_size::format_size;
use super::groups::group_items;
use super::state::{App, Screen};
use crate::clean::build_plan;
use crate::rules::home;

impl App {
    pub fn selected_size(&self) -> u64 {
        self.items
            .iter()
            .filter(|i| i.selected && i.keep_reason.is_none())
            .map(|i| i.size)
            .sum()
    }

    pub fn show_select(&mut self, ui: &mut egui::Ui) {
        egui::Panel::bottom("select_actions").show(ui, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Selected: {}", format_size(self.selected_size()))).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let any = self.items.iter().any(|i| i.selected && i.keep_reason.is_none());
                    if ui.add_enabled(any, egui::Button::new("Review…")).clicked() {
                        self.plan = build_plan(&self.items);
                        self.screen = Screen::Confirm;
                    }
                    if ui.button("Snapshots only").clicked() {
                        self.start_snapshots();
                    }
                });
            });
            ui.add_space(6.0);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            if self.items.is_empty() {
                ui.label("Nothing to clean.");
                return;
            }
            ui.horizontal(|ui| {
                if ui.button("Select all").clicked() {
                    self.items
                        .iter_mut()
                        .filter(|i| i.keep_reason.is_none())
                        .for_each(|i| i.selected = true);
                }
                if ui.button("Select none").clicked() {
                    self.items.iter_mut().for_each(|i| i.selected = false);
                }
            });
            ui.separator();

            let max = self.items.iter().map(|i| i.size).max().unwrap_or(1).max(1);
            let home = home();
            egui::ScrollArea::vertical().show(ui, |ui| {
                for group in group_items(&self.items) {
                    let name = match &group.worktree {
                        Some(path) => path.strip_prefix(&home).unwrap_or(path).display().to_string(),
                        None => "Caches and Xcode".to_owned(),
                    };
                    egui::CollapsingHeader::new(format!("{name}   {}", format_size(group.size)))
                        .id_salt(&name)
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new(&name).num_columns(3).striped(true).show(ui, |ui| {
                                for &i in &group.indices {
                                    let item = &mut self.items[i];
                                    let enabled = item.keep_reason.is_none();
                                    let response = ui.add_enabled(
                                        enabled,
                                        egui::Checkbox::new(&mut item.selected, item.label.as_str()),
                                    );
                                    let paths: Vec<String> =
                                        item.paths.iter().map(|p| p.display().to_string()).collect();
                                    let tip = match item.keep_reason {
                                        Some(reason) => format!("Kept: {reason}\n{}", paths.join("\n")),
                                        None => paths.join("\n"),
                                    };
                                    response.on_hover_text(tip.clone()).on_disabled_hover_text(tip);
                                    ui.label(format_size(item.size));
                                    ui.add(ProgressBar::new(item.size as f32 / max as f32).desired_width(220.0));
                                    ui.end_row();
                                }
                            });
                        });
                }
            });
        });
    }
}
