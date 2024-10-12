use std::{
    fs::{DirBuilder, File},
    prelude::v1,
};

use common::ArcStr;
use serde::Deserialize;

use crate::{HandleStorage, StorageSetting};

#[derive(Debug, Deserialize)]
pub(crate) struct LocalFs {
    location: ArcStr,
}

impl HandleStorage for LocalFs {
    fn new(storage_setting: StorageSetting) -> Self {
        self::LocalFs {
            location: storage_setting.storage_location,
        }
    }

    fn meta(&self) -> String {
        format!("Local File System")
    }

    fn create_path(&self, slug: String) -> Result<String, anyhow::Error> {
        let dir_path: String = format!("{}/{}", self.location, slug).into();
        let _ = DirBuilder::new().recursive(true).create(dir_path.clone())?;
        Ok(dir_path)
    }

    fn create_asset(&self, dir_path: String, content: String) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{HandleStorage, Storage};

    use super::*;

    #[test]
    fn test_create_path() {
        // Just testing out the ArcStr
        let test_path = "test_path";

        let test_storage = Storage::init();

        _ = test_storage.create_path(test_path.to_string());

        assert!(fs::metadata(format!(
            "{}/{}",
            SETTINGS.storage.storage_location, test_path
        ))
        .unwrap()
        .is_dir());
    }
}
