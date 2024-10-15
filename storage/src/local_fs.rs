use std::{
    fmt::format,
    fs::{DirBuilder, File},
    io::Write,
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
        self.location.to_string()
    }

    fn create_path(&self, slug: String) -> Result<String, anyhow::Error> {
        let dir_path: String = format!("{}/{}", self.location, slug).into();
        let _ = DirBuilder::new().recursive(true).create(dir_path.clone())?;
        Ok(dir_path)
    }

    ///
    /// Method: create_asset
    /// Arguments: slug, filename, content
    /// Usage: This method allows the user to create a file with the content.
    /// If the file is not existing, a new file would created with the content and
    /// if the file already exists, the contents of the file would be cleared and
    /// the provided content would be replaced in its place.
    ///

    fn create_asset(
        &self,
        slug: String,
        file_name: String,
        content: &str,
    ) -> Result<(), anyhow::Error> {
        let dir_path = self.create_path(slug)?;
        let file_path = format!("{}/{}", dir_path, file_name);
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{HandleStorage, Storage};
    use std::fs;

    #[test]
    fn test_create_path() {
        // Just testing out the ArcStr
        let test_path = "test";

        let storage_stub = Storage::init();
        let content = r#"
            <html>
                <body>
                    <h1>Hello World !! </h1>
                </body>

            </html>
            "#;

        _ = storage_stub.create_asset(test_path.to_string(), "index.html".to_string(), content);

        assert!(
            fs::metadata(format!("{}/{}", storage_stub.meta(), test_path))
                .unwrap()
                .is_dir()
        );
    }
}
