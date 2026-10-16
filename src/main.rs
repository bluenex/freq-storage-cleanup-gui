mod app;
mod clean;
mod rules;
mod scan;
mod snapshots;
mod worker;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("Storage Cleanup")
            .with_inner_size([820.0, 640.0]),
        ..Default::default()
    };
    eframe::run_native(
        "freq-storage-cleanup",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
