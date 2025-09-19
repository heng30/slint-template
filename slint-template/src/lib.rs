slint::include_modules!();

#[cfg(any(feature = "desktop", feature = "mobile"))]
#[macro_use]
extern crate derivative;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod config;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod version;

#[cfg(feature = "database")]
mod db;

mod logic;

#[cfg(feature = "desktop")]
pub fn init_logger() {
    use std::io::Write;

    env_logger::builder()
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            let ts = cutil::time::local_now("%H:%M:%S");

            writeln!(
                buf,
                "[{} {style}{}{style:#} {} {}] {}",
                ts,
                record.level(),
                record
                    .file()
                    .unwrap_or("None")
                    .split('/')
                    .next_back()
                    .unwrap_or("None"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

#[cfg(feature = "android")]
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

#[cfg(feature = "web")]
fn init_logger() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
async fn ui_before() {
    init_logger();
    config::init();

    #[cfg(feature = "database")]
    db::init(config::db_path().to_str().expect("invalid db path")).await;

    #[cfg(target_os = "linux")]
    {
        _ = slint::set_xdg_app_id("slint-template".to_string());
    }
}

#[cfg(feature = "web")]
fn ui_before() {
    init_logger();
}

fn ui_after(ui: &AppWindow) {
    logic::init(ui);
}

#[cfg(feature = "android")]
#[unsafe(no_mangle)]
#[tokio::main]
async fn android_main(app: slint::android::AndroidApp) {
    log::debug!("start...");

    slint::android::init(app).unwrap();

    ui_before().await;
    let ui = AppWindow::new().unwrap();
    global_store!(ui).set_device_type(DeviceType::Mobile);
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}

#[cfg(feature = "desktop")]
pub async fn desktop_main() {
    log::debug!("start...");

    ui_before().await;
    let ui = AppWindow::new().unwrap();
    global_store!(ui).set_device_type(DeviceType::Desktop);
    ui_after(&ui);

    global_util!(ui).invoke_set_window_center();

    ui.run().unwrap();

    log::debug!("exit...");
}

#[cfg(feature = "web")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    log::debug!("start...");

    ui_before();
    let ui = AppWindow::new().unwrap();
    global_store!(ui).set_device_type(DeviceType::Web);
    ui_after(&ui);

    ui.run().unwrap();

    log::debug!("exit...");
}
