use eframe::egui::{self, Align2, Color32, FontId, Rect, Sense, Stroke, StrokeKind, vec2};

use super::format_size::format_size;
use super::groups::{Group, group_name};
use super::palette::{group_color, item_fill};
use super::squarify::squarify;
use super::state::App;
use crate::rules::home;

const HEADER: f32 = 16.0;

fn hatch(painter: &egui::Painter, rect: Rect, color: Color32) {
    let painter = painter.with_clip_rect(rect);
    let mut x = rect.left() - rect.height();
    while x < rect.right() {
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(x + rect.height(), rect.top())],
            Stroke::new(1.0, color),
        );
        x += 6.0;
    }
}

impl App {
    /// Items as nested tiles, one block per group. Clicking a tile toggles it.
    pub fn show_treemap(&mut self, ui: &mut egui::Ui, groups: &[Group], height: f32) {
        let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
        let painter = ui.painter_at(rect);
        let visuals = ui.visuals().clone();
        let home = home();

        let mut tiles: Vec<(usize, Rect, Color32)> = Vec::new();
        let group_rects = squarify(&groups.iter().map(|g| g.size).collect::<Vec<_>>(), rect);
        for (gi, (group, grect)) in groups.iter().zip(group_rects).enumerate() {
            let color = group_color(gi);
            let mut inner = grect.shrink(2.0);
            if inner.height() > HEADER * 2.5 && inner.width() > 80.0 {
                let header = painter.with_clip_rect(inner);
                header.text(
                    inner.left_top() + vec2(2.0, 1.0),
                    Align2::LEFT_TOP,
                    format!("{}  {}", group_name(group, &home), format_size(group.size)),
                    FontId::proportional(11.0),
                    color,
                );
                inner.min.y += HEADER;
            }
            let sizes: Vec<u64> = group.indices.iter().map(|&i| self.items[i].size).collect();
            for (&i, tile) in group.indices.iter().zip(squarify(&sizes, inner)) {
                tiles.push((i, tile.shrink(1.0), color));
            }
        }

        let pointer = response.hover_pos();
        let hovered_tile = pointer.and_then(|p| tiles.iter().find(|(_, r, _)| r.contains(p)).map(|t| t.0));
        if hovered_tile.is_some() {
            self.hover_next = hovered_tile;
        }

        for &(i, tile, color) in &tiles {
            if !tile.is_positive() {
                continue;
            }
            let item = &self.items[i];
            let id = egui::Id::new(("tile", i, &item.label));
            let t = ui.ctx().animate_bool_with_time(id, item.selected, 0.15);

            if item.keep_reason.is_some() {
                painter.rect_filled(tile, 3.0, visuals.faint_bg_color);
                hatch(&painter, tile, visuals.widgets.noninteractive.bg_stroke.color);
            } else {
                painter.rect_filled(tile, 3.0, item_fill(color, t));
            }
            if self.hovered == Some(i) {
                painter.rect_stroke(
                    tile,
                    3.0,
                    Stroke::new(1.5, visuals.strong_text_color()),
                    StrokeKind::Inside,
                );
            }

            let text_color = if t > 0.5 { Color32::WHITE } else { visuals.text_color() };
            let clip = painter.with_clip_rect(tile.shrink(2.0));
            if tile.width() > 60.0 && tile.height() > 30.0 {
                clip.text(
                    tile.left_top() + vec2(5.0, 4.0),
                    Align2::LEFT_TOP,
                    &item.label,
                    FontId::proportional(11.0),
                    text_color,
                );
                clip.text(
                    tile.left_top() + vec2(5.0, 17.0),
                    Align2::LEFT_TOP,
                    format_size(item.size),
                    FontId::proportional(10.0),
                    text_color.gamma_multiply(0.8),
                );
            } else if tile.width() > 40.0 && tile.height() > 14.0 {
                clip.text(
                    tile.left_top() + vec2(4.0, 2.0),
                    Align2::LEFT_TOP,
                    format_size(item.size),
                    FontId::proportional(10.0),
                    text_color,
                );
            }
        }

        if let Some(i) = hovered_tile {
            if response.clicked() && self.items[i].keep_reason.is_none() {
                self.items[i].selected ^= true;
            }
            let item = &self.items[i];
            response.on_hover_ui_at_pointer(|ui| {
                ui.strong(format!("{}  {}", item.label, format_size(item.size)));
                if let Some(reason) = item.keep_reason {
                    ui.label(format!("Kept: {reason}"));
                }
                for path in &item.paths {
                    ui.monospace(path.display().to_string());
                }
            });
        }
    }
}
