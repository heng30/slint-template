pub use chrono;
use chrono::Local;

// "%Y-%m-%d %H:%M:%S" -> 2023-11-15 14:30:45
pub fn local_now(format: &str) -> String {
    Local::now().format(format).to_string()
}
