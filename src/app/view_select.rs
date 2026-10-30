use eframe::egui::{self, RichText, text::LayoutJob};

use super::format_size::format_size;
use super::groups::group_items;
use super::palette::{group_color, item_fill};
use super::size_bar::size_bar;
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
                    let hide_empty = self.hide_empty;
                    self.items
                        .iter_mut()
                        .filter(|i| i.keep_reason.is_none() && !(hide_empty && i.size == 0))
                        .for_each(|i| i.selected = true);
                }
                if ui.button("Select none").clicked() {
                    self.items.iter_mut().for_each(|i| i.selected = false);
                }
                if ui.checkbox(&mut self.hide_empty, "Hide 0 B").changed() && self.hide_empty {
                    // Hidden rows must not stay selected, or they'd be cleaned unseen.
                    self.items
                        .iter_mut()
                        .filter(|i| i.size == 0)
                        .for_each(|i| i.selected = false);
                }
            });
            ui.add_space(4.0);

            let groups = group_items(&self.items, &home(), self.hide_empty);
            if groups.is_empty() {
                ui.label("Nothing to clean.");
                return;
            }

            let max = self.items.iter().map(|i| i.size).max().unwrap_or(1).max(1);
            egui::ScrollArea::vertical().show(ui, |ui| {
                for group in &groups {
                    let name = &group.name;
                    let color = group_color(name);
                    let mut title = LayoutJob::default();
                    let style = ui.style();
                    title.append(
                        "● ",
                        0.0,
                        egui::TextFormat::simple(egui::TextStyle::Body.resolve(style), color),
                    );
                    let body =
                        egui::TextFormat::simple(egui::TextStyle::Body.resolve(style), ui.visuals().text_color());
                    title.append(&format!("{name}   {}", format_size(group.size)), 0.0, body);

                    egui::CollapsingHeader::new(title)
                        .id_salt(name)
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new(name).num_columns(3).show(ui, |ui| {
                                for &i in &group.indices {
                                    let item = &mut self.items[i];
                                    let checkbox = egui::Checkbox::new(&mut item.selected, item.label.as_str());
                                    let response = ui.add_enabled(item.keep_reason.is_none(), checkbox);
                                    let paths: Vec<String> =
                                        item.paths.iter().map(|p| p.display().to_string()).collect();
                                    let tip = match item.keep_reason {
                                        Some(reason) => format!("Kept: {reason}\n{}", paths.join("\n")),
                                        None => paths.join("\n"),
                                    };
                                    response.on_hover_text(tip.clone()).on_disabled_hover_text(tip);

                                    ui.label(format_size(item.size));
                                    let t = ui.ctx().animate_bool_with_time(
                                        egui::Id::new(("item", i, &item.label)),
                                        item.selected,
                                        0.15,
                                    );
                                    let fill = if item.keep_reason.is_some() {
                                        ui.visuals().weak_text_color()
                                    } else {
                                        item_fill(color, t)
                                    };
                                    size_bar(ui, item.size as f32 / max as f32, fill);
                                    ui.end_row();
                                }
                            });
                        });
                }
            });
        });
    }
}
