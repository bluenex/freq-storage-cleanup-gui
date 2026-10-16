use eframe::egui::{self, Color32, RichText, Sense, vec2};

use super::format_size::format_size;
use super::state::App;

impl App {
    /// The whole disk as one strip: other used space, cleanable space, the
    /// part the selection frees, and free space.
    pub fn show_disk_bar(&mut self, ui: &mut egui::Ui) {
        let Some(disk) = self.disk else {
            ui.label(RichText::new("Free space: …").heading());
            return;
        };
        let cleanable: u64 = self
            .items
            .iter()
            .filter(|i| i.keep_reason.is_none())
            .map(|i| i.size)
            .sum();
        let selected = self.selected_size() as f32;
        let selected = ui
            .ctx()
            .animate_value_with_time(egui::Id::new("disk_bar_selected"), selected, 0.25);
        let used_other = disk.total.saturating_sub(disk.free + cleanable);

        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("Free {}", format_size(disk.free))).heading());
            if selected >= 1.0 {
                let after = disk.free + selected as u64;
                ui.label(RichText::new(format!("→ {}", format_size(after))).heading().strong());
            }
        });

        let accent = ui.visuals().selection.bg_fill;
        let segments = [
            (used_other as f32, ui.visuals().widgets.inactive.bg_fill),
            (cleanable as f32 - selected, accent.gamma_multiply(0.35)),
            (selected, accent),
            (disk.free as f32, Color32::TRANSPARENT),
        ];
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 12.0), Sense::hover());
        let painter = ui.painter_at(rect);
        let scale = rect.width() / disk.total.max(1) as f32;
        let mut x = rect.left();
        for (bytes, color) in segments {
            let width = bytes.max(0.0) * scale;
            let segment = egui::Rect::from_min_size(egui::pos2(x, rect.top()), vec2(width, rect.height()));
            painter.rect_filled(segment, 0.0, color);
            x += width;
        }
        painter.rect_stroke(
            rect,
            6.0,
            ui.visuals().widgets.noninteractive.bg_stroke,
            egui::StrokeKind::Inside,
        );

        ui.horizontal(|ui| {
            let small = |text: String| RichText::new(text).small().weak();
            ui.label(small(format!("Other {}", format_size(used_other))));
            ui.label(small(format!("Cleanable {}", format_size(cleanable))));
            ui.label(small(format!("Selected {}", format_size(selected as u64))));
        });
    }
}
