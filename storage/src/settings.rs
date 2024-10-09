use common::{ArcStr, LoadSettings};
use lazy_static::lazy_static;
use serde::Deserialize;
use strum_macros::EnumString;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub storage: StorageSetting,
}

#[derive(Debug, Deserialize)]
pub(crate) struct StorageSetting {
    pub storage_type: StorageType,
    pub storage_location: ArcStr,
}

#[derive(Debug, Deserialize, EnumString)]
pub enum StorageType {
    LocalFs,
    AwsS3,
}

impl LoadSettings for Settings {}

lazy_static! {
    pub(crate) static ref SETTINGS: Settings = Settings::load();
}
