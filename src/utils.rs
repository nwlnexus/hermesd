use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;

pub fn path_readable_file(value: &OsStr) -> Result<(), String> {
    let path = PathBuf::from(value); //.as_ref();

    if path.is_dir() {
        return Err(format!(
            "{}: Input path must be a file, not a directory",
            path.display()
        ));
    }

    File::open(&path)
        .map(|_| ())
        .map_err(|e| format!("{}: {}", path.display(), e))
}
