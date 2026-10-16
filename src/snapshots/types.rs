pub struct Snapshot {
    /// `YYYY-MM-DD-HHMMSS`, the id `tmutil deletelocalsnapshots` takes.
    pub date: String,
    /// The snapshot that marks the last external backup. Kept by default.
    pub dataless: bool,
}
