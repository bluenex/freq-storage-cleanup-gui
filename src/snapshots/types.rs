pub struct Snapshot {
    /// `YYYY-MM-DD-HHMMSS`, the id `tmutil deletelocalsnapshots` takes.
    pub date: String,
    /// `tmutil` marks some reference snapshots this way once their data is purged.
    pub dataless: bool,
    /// Time Machine's reference for the next external backup, from its prefs.
    pub last_backup: bool,
}

impl Snapshot {
    /// Kept by default: deleting it makes the next external backup scan the whole disk.
    pub fn is_reference(&self) -> bool {
        self.dataless || self.last_backup
    }
}
