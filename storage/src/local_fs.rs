use std::fs::{DirBuilder, File};

use common::ArcStr;

use super::SETTINGS;

pub fn create_path(slug: String) -> Result<String, anyhow::Error> {
    let dir_path: String = format!("{}/{}", SETTINGS.storage.storage_location, slug).into();
    let _ = DirBuilder::new().recursive(true).create(dir_path.clone())?;
    Ok(dir_path)
}

pub fn create_asset(dir_path: String, content: String) -> Result<(), anyhow::Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use common::ArcStr;

    use super::*;

    #[test]
    fn test_create_path() {
        // Just testing out the ArcStr
        let test_path = ArcStr::from("test_path");

        create_path(test_path.to_string());

        assert!(fs::metadata(format!(
            "{}/{}",
            SETTINGS.storage.storage_location, test_path
        ))
        .unwrap()
        .is_dir());
    }
}
