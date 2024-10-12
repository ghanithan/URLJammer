use common::ArcStr;
use serde::Deserialize;

use crate::{HandleStorage, StorageSetting};

#[derive(Debug, Deserialize)]
pub(crate) struct AwsS3 {
    location: ArcStr,
}

impl HandleStorage for AwsS3 {
    fn new(storage_setting: StorageSetting) -> Self {
        self::AwsS3 {
            location: storage_setting.storage_location,
        }
    }

    fn meta(&self) -> String {
        format!("AWS S3")
    }

    fn create_path(&self, slug: String) -> Result<String, anyhow::Error> {
        Ok("Under development".to_string())
    }

    fn create_asset(
        &self,
        slug: String,
        file_name: String,
        content: &str,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
