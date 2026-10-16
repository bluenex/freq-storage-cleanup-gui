use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;

use eframe::egui;

use crate::clean::{Step, execute};
use crate::scan::{ScanResult, scan_all};
use crate::snapshots::{Snapshot, delete_snapshots, list_snapshots};

pub enum Msg {
    Scanned(ScanResult),
    Log(String),
    CleanDone,
    Snapshots(Result<Vec<Snapshot>, String>),
    SnapshotsDeleted,
}

/// Runs slow work off the UI thread and wakes the UI on every message.
pub struct Worker {
    tx: Sender<Msg>,
    pub rx: Receiver<Msg>,
    ctx: egui::Context,
}

impl Worker {
    pub fn new(ctx: egui::Context) -> Self {
        let (tx, rx) = channel();
        Self { tx, rx, ctx }
    }

    fn spawn(&self, job: impl FnOnce(&dyn Fn(Msg)) + Send + 'static) {
        let tx = self.tx.clone();
        let ctx = self.ctx.clone();
        thread::spawn(move || {
            let send = |msg| {
                let _ = tx.send(msg);
                ctx.request_repaint();
            };
            job(&send);
        });
    }

    /// `delay` gives APFS time to report freed space after a cleanup.
    pub fn scan(&self, delay: Duration) {
        self.spawn(move |send| {
            thread::sleep(delay);
            send(Msg::Scanned(scan_all()));
        });
    }

    pub fn clean(&self, steps: Vec<Step>) {
        self.spawn(move |send| {
            execute(&steps, |line| send(Msg::Log(line)));
            send(Msg::CleanDone);
        });
    }

    pub fn list_snapshots(&self) {
        self.spawn(|send| send(Msg::Snapshots(list_snapshots().map_err(|e| format!("{e:#}")))));
    }

    pub fn delete_snapshots(&self, dates: Vec<String>) {
        self.spawn(move |send| {
            delete_snapshots(&dates, |line| send(Msg::Log(line)));
            send(Msg::SnapshotsDeleted);
        });
    }
}
