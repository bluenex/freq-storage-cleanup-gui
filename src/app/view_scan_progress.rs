use eframe::egui;

use super::state::App;

/// How many unfinished rows to name under the bar.
const SHOWN_PENDING: usize = 3;

impl App {
    /// A bar of measured rows and the names of the ones still running. Shows nothing outside a scan.
    pub fn show_scan_progress(&self, ui: &mut egui::Ui) {
        let total = self.scan_progress.len();
        if total == 0 {
            return;
        }
        let done = self.scan_progress.iter().filter(|(_, done)| *done).count();
        ui.add_space(6.0);
        ui.add(
            egui::ProgressBar::new(done as f32 / total as f32)
                .text(format!("Measured {done} of {total}"))
                .animate(done < total),
        );

        let pending: Vec<&str> = self
            .scan_progress
            .iter()
            .filter(|(_, done)| !*done)
            .map(|(label, _)| label.as_str())
            .collect();
        if pending.is_empty() {
            return;
        }
        let mut text = pending[..pending.len().min(SHOWN_PENDING)].join(", ");
        if pending.len() > SHOWN_PENDING {
            text += &format!(" and {} more", pending.len() - SHOWN_PENDING);
        }
        ui.label(egui::RichText::new(format!("Still measuring: {text}")).small().weak());
    }
}
