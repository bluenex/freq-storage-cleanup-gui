use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

/// The per-user dir under `/var/folders` that holds `T` (temp) and `X` (code-sign clones), canonicalized.
pub fn user_var() -> Option<PathBuf> {
    let mut buf = [0u8; 1024];
    let n = unsafe { libc::confstr(libc::_CS_DARWIN_USER_TEMP_DIR, buf.as_mut_ptr().cast(), buf.len()) };
    if n == 0 || n > buf.len() {
        return None;
    }
    let temp = Path::new(OsStr::from_bytes(&buf[..n - 1])).canonicalize().ok()?;
    temp.parent().map(Path::to_path_buf)
}
