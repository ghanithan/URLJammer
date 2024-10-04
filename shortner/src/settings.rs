use common::LoadSettings;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub shortner: Shortener,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Shortener {
    pub slug_length: usize,
}

impl LoadSettings for Settings {}

lazy_static! {
    pub(crate) static ref SETTINGS: Settings = Settings::load();
}
