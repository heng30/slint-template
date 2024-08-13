use chrono::Local;

pub fn local_now(format: &str) -> String {
    return Local::now().format(format).to_string();
}
