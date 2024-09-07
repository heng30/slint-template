#![windows_subsystem = "windows"]

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
slint::slint! {
    export * from "ui/desktop-window.slint";
}

#[cfg(target_os = "android")]
slint::slint! {
    export * from "ui/android-window.slint";
}

#[cfg(target_arch = "wasm32")]
slint::slint! {
    export * from "ui/web-window.slint";
}

#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "android"
))]
#[macro_use]
extern crate derivative;

#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "android"
))]
mod config;

#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "android"
))]
mod version;

#[cfg(feature = "database")]
mod db;

mod logic;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub fn init_logger() {
    use cutil::chrono::Local;
    use env_logger::fmt::Color;
    use std::io::Write;

    env_logger::builder()
        .format(|buf, record| {
            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
            let mut level_style = buf.style();
            match record.level() {
                log::Level::Warn | log::Level::Error => {
                    level_style.set_color(Color::Red).set_bold(true)
                }
                _ => level_style.set_color(Color::Blue).set_bold(true),
            };

            writeln!(
                buf,
                "[{} {} {} {}] {}",
                ts,
                level_style.value(record.level()),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .last()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

#[cfg(target_os = "android")]
fn init_logger() {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Trace)
            .with_filter(
                android_logger::FilterBuilder::new()
                    .filter_level(log::LevelFilter::Debug)
                    .build(),
            ),
    );
}

#[cfg(target_arch = "wasm32")]
fn init_logger() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "macos",
    target_os = "android"
))]
async fn ui_before() {
    init_logger();
    config::init();

    cfg_if::cfg_if! {
        if #[cfg(feature = "database")] {
            db::init(config::db_path().to_str().expect("invalid db path")).await;
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn ui_before() {
    init_logger();
}

fn ui_after(ui: &AppWindow) {
    logic::init(ui);
}

#[cfg(target_os = "android")]
#[no_mangle]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) {
    log::debug!("start...");

    slint::android::init(app).unwrap();

    ui_before().await;
    let ui = AppWindow::new().unwrap();
    ui.global::<Store>().set_device_type(DeviceType::Mobile);
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub async fn desktop_main() {
    log::debug!("start...");

    ui_before().await;
    let ui = AppWindow::new().unwrap();
    ui.global::<Store>().set_device_type(DeviceType::Desktop);
    ui.global::<Util>().invoke_update_window_size();
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}

#[cfg(target_arch = "wasm32")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    log::debug!("start...");

    ui_before();
    let ui = AppWindow::new().unwrap();
    ui.global::<Store>().set_device_type(DeviceType::Web);
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}
