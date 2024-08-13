use super::data::{self, Config};
use anyhow::{Context, Result};
use log::debug;
use once_cell::sync::Lazy;
use std::{fs, path::PathBuf, sync::Mutex};

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));

#[cfg(not(target_os = "android"))]
use platform_dirs::AppDirs;

#[cfg(target_os = "android")]
pub struct AppDirs {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

#[cfg(target_os = "android")]
impl AppDirs {
    pub fn new(name: Option<&str>, _: bool) -> Option<Self> {
        let root_dir = "/data/data";
        let name = name.unwrap();

        Some(Self {
            config_dir: PathBuf::from(&format!("{root_dir}/{name}/config")),
            data_dir: PathBuf::from(&format!("{root_dir}/{name}/data")),
        })
    }
}

pub fn init() {
    if let Err(e) = CONFIG.lock().unwrap().init() {
        log::error!("{e:?}");
        panic!("{:?}", e);
    }
}

#[allow(dead_code)]
pub fn appid() -> String {
    CONFIG.lock().unwrap().appid.clone()
}

pub fn is_first_run() -> bool {
    CONFIG.lock().unwrap().is_first_run
}

pub fn all() -> data::Config {
    CONFIG.lock().unwrap().clone()
}

#[allow(dead_code)]
pub fn reset(mut conf: Config) {
    let mut c = CONFIG.lock().unwrap();

    conf.config_path = c.config_path.clone();
    conf.db_path = c.db_path.clone();
    conf.cache_dir = c.cache_dir.clone();
    conf.is_first_run = c.is_first_run;

    *c = conf;
    _ = c.save();
}

pub fn ui() -> data::UI {
    CONFIG.lock().unwrap().ui.clone()
}

pub fn db_path() -> PathBuf {
    CONFIG.lock().unwrap().db_path.clone()
}

#[allow(dead_code)]
pub fn cache_dir() -> PathBuf {
    CONFIG.lock().unwrap().cache_dir.clone()
}

pub fn save(conf: data::Config) -> Result<()> {
    let mut config = CONFIG.lock().unwrap();
    *config = conf;
    config.save()
}

impl Config {
    pub fn init(&mut self) -> Result<()> {
        let app_name = if cfg!(not(target_os = "android")) {
            "slint"
        } else {
            if cfg!(debug_assertions) {
                "xyz.heng30.slint"
            } else {
                "xyz.heng30.slint"
            }
        };

        let app_dirs = AppDirs::new(Some(app_name), true).unwrap();
        self.init_config(&app_dirs)?;
        self.load().with_context(|| "load config file failed")?;
        debug!("{:?}", self);
        Ok(())
    }

    fn init_config(&mut self, app_dirs: &AppDirs) -> Result<()> {
        self.db_path = app_dirs.data_dir.join("slint.db");
        self.config_path = app_dirs.config_dir.join("slint.toml");
        self.cache_dir = app_dirs.data_dir.join("cache");

        if self.appid.is_empty() {
            self.appid = super::data::appid_default();
        }

        fs::create_dir_all(&app_dirs.data_dir)?;
        fs::create_dir_all(&app_dirs.config_dir)?;
        fs::create_dir_all(&self.cache_dir)?;

        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        match fs::read_to_string(&self.config_path) {
            Ok(text) => match toml::from_str::<Config>(&text) {
                Ok(c) => {
                    self.appid = c.appid;
                    self.ui = c.ui;
                    Ok(())
                }
                Err(_) => {
                    self.is_first_run = true;
                    match toml::to_string_pretty(self) {
                        Ok(text) => Ok(fs::write(&self.config_path, text)?),
                        Err(e) => Err(e.into()),
                    }
                }
            },
            Err(_) => {
                self.is_first_run = true;
                match toml::to_string_pretty(self) {
                    Ok(text) => Ok(fs::write(&self.config_path, text)?),
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    pub fn save(&self) -> Result<()> {
        match toml::to_string_pretty(self) {
            Ok(text) => Ok(fs::write(&self.config_path, text)
                .with_context(|| format!("save config failed"))?),
            Err(e) => anyhow::bail!(format!("convert config from toml format failed. {e:?}")),
        }
    }
}
