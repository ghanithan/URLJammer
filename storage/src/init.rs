use common::ArcStr;

use crate::{AwsS3, HandleStorage, LocalFs, Storage, StorageSetting, StorageType, SETTINGS};

impl Storage {
    pub fn init() -> Self {
        Self::new(SETTINGS.storage.clone())
    }
}

macro_rules! access_storage_variant {
    ( $self: ident, $fn: ident, $($args:tt),*) => {
        match $self {
            Storage::LocalFs(stub) => stub.$fn($($args),*),
            Storage::AwsS3(stub) => stub.$fn($($args),*),
        }
    };
}

impl HandleStorage for Storage {
    fn new(storage_setting: StorageSetting) -> Self {
        match storage_setting.storage_type {
            StorageType::LocalFs => Self::LocalFs(LocalFs::new(storage_setting)),
            StorageType::AwsS3 => Self::AwsS3(AwsS3::new(storage_setting)),
        }
    }

    fn meta(&self) -> String {
        access_storage_variant!(self, meta,)
    }

    fn create_path(&self, slug: String) -> Result<String, anyhow::Error> {
        access_storage_variant!(self, create_path, slug)
    }

    fn create_asset(
        &self,
        slug: String,
        file_name: String,
        content: &str,
    ) -> Result<(), anyhow::Error> {
        access_storage_variant!(self, create_asset, slug, file_name, content)
    }
}
