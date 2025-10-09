//! Desktop entry point module
//! 
//! Contains the main function for desktop applications.
//! Sets Windows subsystem to "windows" to hide console window.

#![windows_subsystem = "windows"]

/// Main entry point for desktop applications
/// 
/// Initializes the async runtime and delegates to the library's desktop_main function.
#[tokio::main]
async fn main() {
    extern crate slint_template;
    slint_template::desktop_main().await;
}
