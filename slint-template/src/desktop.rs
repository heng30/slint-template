#![windows_subsystem = "windows"]

#[tokio::main]
async fn main() {
    extern crate slint_template;
    slint_template::desktop_main().await;
}
