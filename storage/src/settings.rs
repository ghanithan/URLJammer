use common::{ArcStr, LoadSettings};
use lazy_static::lazy_static;
use serde::Deserialize;
use strum_macros::EnumString;

use super::create_enum_and_type;

use super::{AwsS3, LocalFs};

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Settings {
    pub storage: StorageSetting,
}

create_enum_and_type!(
    #[derive(Debug)]
    pub enum Storage {
        LocalFs(LocalFs),
        AwsS3(AwsS3),
    }
);

#[derive(Debug, Deserialize, Clone)]
pub struct StorageSetting {
    pub storage_type: StorageType,
    pub storage_location: ArcStr,
}

impl LoadSettings for Settings {}

lazy_static! {
    pub(crate) static ref SETTINGS: Settings = Settings::load();
}
