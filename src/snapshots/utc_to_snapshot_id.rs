/// Parses `YYYY-MM-DD HH:MM:SS +0000` into its six numbers.
fn parse_utc(text: &str) -> Option<[i32; 6]> {
    let digits: Vec<i32> = text
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .take(6)
        .map(|s| s.parse().ok())
        .collect::<Option<_>>()?;
    digits.try_into().ok()
}

/// Converts a UTC date from the Time Machine prefs into a snapshot id, which uses local time.
pub fn utc_to_snapshot_id(text: &str) -> Option<String> {
    let [year, month, day, hour, min, sec] = parse_utc(text)?;
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    tm.tm_year = year - 1900;
    tm.tm_mon = month - 1;
    tm.tm_mday = day;
    tm.tm_hour = hour;
    tm.tm_min = min;
    tm.tm_sec = sec;
    let epoch = unsafe { libc::timegm(&mut tm) };
    let mut local: libc::tm = unsafe { std::mem::zeroed() };
    if unsafe { libc::localtime_r(&epoch, &mut local) }.is_null() {
        return None;
    }
    Some(format!(
        "{:04}-{:02}-{:02}-{:02}{:02}{:02}",
        local.tm_year + 1900,
        local.tm_mon + 1,
        local.tm_mday,
        local.tm_hour,
        local.tm_min,
        local.tm_sec
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_prefs_date() {
        assert_eq!(parse_utc("2026-10-30 08:42:30 +0000"), Some([2026, 10, 30, 8, 42, 30]));
        assert_eq!(parse_utc("2026-10-30"), None);
    }
}
