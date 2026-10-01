use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const MAX_DOCUMENT_BYTES: u64 = 1_048_576;

/// Keeps CLI file handling aligned with the library's bounded parser contract.
pub(crate) fn read_bounded_utf8(path: &str) -> Result<String, String> {
    let path = Path::new(path);
    let metadata =
        fs::symlink_metadata(path).map_err(|_| "document could not be inspected".to_owned())?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err("document must be a regular non-symlink file".to_owned());
    }
    if metadata.len() > MAX_DOCUMENT_BYTES {
        return Err("document exceeds 1 MiB".to_owned());
    }
    let file = File::open(path).map_err(|_| "document could not be opened".to_owned())?;
    let mut bytes = Vec::new();
    file.take(MAX_DOCUMENT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "document could not be read".to_owned())?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_DOCUMENT_BYTES {
        return Err("document exceeds 1 MiB".to_owned());
    }
    String::from_utf8(bytes).map_err(|_| "document must be UTF-8".to_owned())
}
