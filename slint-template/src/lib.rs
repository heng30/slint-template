//! Slint Template Application
//!
//! A cross-platform GUI application built with the Slint framework.
//! Supports Desktop (Windows, Linux, macOS), Android, and Web platforms.
//!
//! # Features
//! - Desktop: Full desktop application with native window management
//! - Android: Mobile application with touch interface
//! - Web: WebAssembly compilation for browser deployment
//! - Database: SQL database support for data persistence
//! - QR Code: QR code generation functionality
//! - Center Window: Window centering utilities
//!
//! # Architecture
//! - Platform-specific entry points: `desktop_main`, `android_main`, `main` (web)
//! - Global configuration management
//! - UI logic initialization and callback handling
//! - Cross-platform logging setup

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

/// Initializes the logger for desktop platforms.
///
/// Sets up a custom logger format with timestamp, log level, file name, line number,
/// and log message. Uses local time format for timestamps.
/// Initializes the logger for desktop platforms.
///
/// Sets up a custom logger format with timestamp, log level, file name, line number,
/// and log message. Uses local time format for timestamps.
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

/// Initializes the logger for Android platforms.
///
/// Uses Android-specific logging with debug level filtering.
/// Initializes the logger for Android platforms.
///
/// Uses Android-specific logging with debug level filtering.
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

/// Initializes the logger for web platforms.
///
/// Uses console logging for web applications with trace level.
/// Initializes the logger for web platforms.
///
/// Uses console logging for web applications with trace level.
#[cfg(feature = "web")]
fn init_logger() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

/// Performs initialization tasks before UI creation for desktop and mobile platforms.
///
/// # Tasks
/// - Initializes logger
/// - Loads configuration
/// - Initializes database (if enabled)
/// - Sets XDG app ID on Linux
/// Performs initialization tasks before UI creation for desktop and mobile platforms.
///
/// # Tasks
/// - Initializes logger
/// - Loads configuration
/// - Initializes database (if enabled)
/// - Sets XDG app ID on Linux
#[cfg(any(feature = "desktop", feature = "mobile"))]
async fn ui_before() {
    init_logger();
    config::init();

    #[cfg(feature = "database")]
    db::init(config::all().db_path.to_str().expect("invalid db path")).await;

    #[cfg(target_os = "linux")]
    {
        _ = slint::set_xdg_app_id("slint-template".to_string());
    }
}

/// Performs initialization tasks before UI creation for web platforms.
///
/// # Tasks
/// - Initializes logger
/// Performs initialization tasks before UI creation for web platforms.
///
/// # Tasks
/// - Initializes logger
#[cfg(feature = "web")]
fn ui_before() {
    init_logger();
}

/// Performs initialization tasks after UI creation.
///
/// # Parameters
/// - `ui`: Reference to the application window
///
/// # Tasks
/// - Initializes UI logic and callbacks
/// Performs initialization tasks after UI creation.
///
/// # Parameters
/// - `ui`: Reference to the application window
///
/// # Tasks
/// - Initializes UI logic and callbacks
fn ui_after(ui: &AppWindow) {
    logic::init(ui);
}

/// Main entry point for Android applications.
///
/// # Parameters
/// - `app`: Android application context
///
/// # Tasks
/// - Initializes Slint Android runtime
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to mobile
/// - Initializes UI logic
/// - Runs the application
/// Main entry point for Android applications.
///
/// # Parameters
/// - `app`: Android application context
///
/// # Tasks
/// - Initializes Slint Android runtime
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to mobile
/// - Initializes UI logic
/// - Runs the application
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

/// Main entry point for desktop applications.
///
/// # Tasks
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to desktop
/// - Initializes UI logic
/// - Centers the window
/// - Runs the application
/// Main entry point for desktop applications.
///
/// # Tasks
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to desktop
/// - Initializes UI logic
/// - Centers the window
/// - Runs the application
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

/// Main entry point for web applications.
///
/// # Tasks
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to web
/// - Initializes UI logic
/// - Runs the application
/// Main entry point for web applications.
///
/// # Tasks
/// - Performs pre-UI initialization
/// - Creates and configures application window
/// - Sets device type to web
/// - Initializes UI logic
/// - Runs the application
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
