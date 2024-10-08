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

    #[serde(skip)]
    pub app_name: String,

    #[serde(default = "appid_default")]
    pub appid: String,

    pub preference: Preference,

    pub proxy: Proxy,
}

#[derive(Serialize, Deserialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Preference {
    #[derivative(Default(value = "1500"))]
    pub win_width: u32,

    #[derivative(Default(value = "800"))]
    pub win_height: u32,

    #[derivative(Default(value = "16"))]
    pub font_size: u32,

    #[derivative(Default(value = "\"Default\".to_string()"))]
    pub font_family: String,

    #[derivative(Default(value = "\"cn\".to_string()"))]
    pub language: String,

    #[derivative(Default(value = "false"))]
    pub always_on_top: bool,

    #[derivative(Default(value = "false"))]
    pub no_frame: bool,

    pub is_dark: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct Proxy {
    #[derivative(Default(value = "\"127.0.0.1\".to_string()"))]
    pub http_url: String,

    #[derivative(Default(value = "3128"))]
    pub http_port: u16,

    #[derivative(Default(value = "\"127.0.0.1\".to_string()"))]
    pub socks5_url: String,

    #[derivative(Default(value = "1080"))]
    pub socks5_port: u16,
}

pub fn appid_default() -> String {
    Uuid::new_v4().to_string()
}
