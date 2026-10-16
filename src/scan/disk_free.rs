use std::ffi::CString;

/// Free bytes on the data volume.
pub fn disk_free() -> Option<u64> {
    let path = CString::new("/System/Volumes/Data").ok()?;
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return None;
    }
    Some(stat.f_bavail * u64::from(stat.f_bsize))
}
