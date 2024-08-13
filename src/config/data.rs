use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    #[serde(skip)]
    pub config_path: PathBuf,

    #[serde(skip)]
    pub db_path: PathBuf,

    #[serde(skip)]
    pub cache_dir: PathBuf,

    #[serde(skip)]
    pub is_first_run: bool,

    #[serde(default = "appid_default")]
    pub appid: String,

    pub ui: UI,
}

#[derive(Serialize, Deserialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct UI {
    #[derivative(Default(value = "16"))]
    pub font_size: u32,

    #[derivative(Default(value = "\"Default\".to_string()"))]
    pub font_family: String,

    #[derivative(Default(value = "\"cn\".to_string()"))]
    pub language: String,

    pub is_dark: bool,
}

pub fn appid_default() -> String {
    Uuid::new_v4().to_string()
}
