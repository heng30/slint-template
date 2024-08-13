#![windows_subsystem = "windows"]

slint::include_modules!();

#[macro_use]
extern crate derivative;

mod config;
mod db;
mod logic;
mod version;

#[cfg(not(target_os = "android"))]
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

async fn ui_before() {
    init_logger();
    config::init();
    db::init(config::db_path().to_str().expect("invalid db path")).await;
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
    ui_after(&ui);
    ui.run().unwrap();

    log::debug!("exit...");
}

#[cfg(not(target_os = "android"))]
pub async fn desktop_main() {
    log::debug!("start...");

    ui_before().await;
    let ui = AppWindow::new().unwrap();
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}
