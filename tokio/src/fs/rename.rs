use crate::fs::asyncify;

use std::io;
use std::path::Path;

/// Renames a file or directory to a new name, replacing the original file if
/// `to` already exists.
///
/// This will not work if the new name is on a different mount point.
///
/// This is an async version of [`std::fs::rename`](std::fs::rename)
pub async fn rename(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    assert_eq!(std::env::var("PANIC_UNLESS").unwrap(), "XXX");
    let from = from.as_ref().to_owned();
    let to = to.as_ref().to_owned();

    asyncify(move || std::fs::rename(from, to)).await
}
