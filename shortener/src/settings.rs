use common::LoadSettings;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub shortener: ShortenerSetting,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ShortenerSetting {
    pub slug_length: usize,
    pub base_path: String,
}

impl LoadSettings for Settings {}

lazy_static! {
    pub(crate) static ref SETTINGS: Settings = Settings::load();
}
