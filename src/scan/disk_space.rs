use std::ffi::CString;

use super::types::DiskSpace;

/// Total and free bytes on the data volume.
pub fn disk_space() -> Option<DiskSpace> {
    let path = CString::new("/System/Volumes/Data").ok()?;
    let mut stat: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(path.as_ptr(), &mut stat) } != 0 {
        return None;
    }
    let block = u64::from(stat.f_bsize);
    Some(DiskSpace {
        total: stat.f_blocks * block,
        free: stat.f_bavail * block,
    })
}
