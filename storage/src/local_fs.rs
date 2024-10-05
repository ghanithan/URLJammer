use std::fs::DirBuilder;

use super::SETTINGS;

pub fn create_path(slug: String) {
    let mut builder = DirBuilder::new()
        .recursive(true)
        .create(format!("{}/{}", SETTINGS.storage.storage_location, slug));
}
