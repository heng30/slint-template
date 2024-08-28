mod conf;
mod data;

pub use conf::{all, app_name, init, is_first_run, preference, proxy, save};

#[cfg(feature = "database")]
pub use conf::db_path;
