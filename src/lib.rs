pub mod models;
pub mod traits;

#[cfg(test)]
mod test;

pub(crate) const DOWNLOAD_BASE_URL: &str = "https://www.bungie.net";
pub(crate) const API_BASE_URL: &str = "https://www.bungie.net/Platform";

use std::path::PathBuf;
use directories::ProjectDirs;

pub fn get_recommended_manifest_path() -> Option<PathBuf> {
    match ProjectDirs::from(
        "com.github",
        "",
        "destiny-rs"
    ) {
        Some(dirs) => Some(dirs.data_dir().to_path_buf()),
        None => None
    }
}