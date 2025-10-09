//! Toast notification module
//! 
//! Provides toast notification functionality with different status types
//! and automatic timeout management.

use crate::{
    global_util,
    slint_generatedAppWindow::{AppWindow, ToastSetting, ToastStatus},
};
use slint::{ComponentHandle, Timer, TimerMode, Weak};

/// Macro to show warning toast notification
/// 
/// # Parameters
/// - `$ui`: AppWindow instance
/// - `$msg`: Warning message
#[macro_export]
macro_rules! toast_warn {
    ($ui:expr, $msg:expr) => {
        $ui.global::<$crate::slint_generatedAppWindow::Util>()
            .invoke_show_toast(
                slint::format!("{}", $msg),
                $crate::slint_generatedAppWindow::ToastStatus::Warning,
            )
    };
}

/// Macro to show success toast notification
/// 
/// # Parameters
/// - `$ui`: AppWindow instance
/// - `$msg`: Success message
#[macro_export]
macro_rules! toast_success {
    ($ui:expr, $msg:expr) => {
        $ui.global::<$crate::slint_generatedAppWindow::Util>()
            .invoke_show_toast(
                slint::format!("{}", $msg),
                $crate::slint_generatedAppWindow::ToastStatus::Success,
            )
    };
}

/// Macro to show info toast notification
/// 
/// # Parameters
/// - `$ui`: AppWindow instance
/// - `$msg`: Info message
#[allow(dead_code)]
#[macro_export]
macro_rules! toast_info {
    ($ui:expr, $msg:expr) => {
        $ui.global::<$crate::slint_generatedAppWindow::Util>()
            .invoke_show_toast(
                slint::format!("{}", $msg),
                $crate::slint_generatedAppWindow::ToastStatus::Info,
            )
    };
}

/// Shows warning toast notification asynchronously
/// 
/// # Parameters
/// - `ui`: Weak reference to the application window
/// - `msg`: Warning message
#[allow(dead_code)]
pub fn async_toast_warn(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        global_util!(ui.unwrap())
            .invoke_show_toast(slint::format!("{}", msg), ToastStatus::Warning);
    });
}

/// Shows success toast notification asynchronously
/// 
/// # Parameters
/// - `ui`: Weak reference to the application window
/// - `msg`: Success message
#[allow(dead_code)]
pub fn async_toast_success(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        global_util!(ui.unwrap())
            .invoke_show_toast(slint::format!("{}", msg), ToastStatus::Success);
    });
}

/// Shows info toast notification asynchronously
/// 
/// # Parameters
/// - `ui`: Weak reference to the application window
/// - `msg`: Info message
#[allow(dead_code)]
pub fn async_toast_info(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        global_util!(ui.unwrap()).invoke_show_toast(slint::format!("{}", msg), ToastStatus::Info);
    });
}

/// Initializes toast notification functionality
/// 
/// Sets up the toast callback with automatic timeout management.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
pub fn init(ui: &AppWindow) {
    let timer = Timer::default();
    let ui_weak = ui.as_weak();
    global_util!(ui).on_show_toast(move |msg, status| {
        let ui = ui_weak.unwrap();

        if timer.running() {
            timer.stop();
        }

        let interval = if msg.chars().collect::<Vec<_>>().len() > 20 {
            5
        } else {
            2
        };

        ui.global::<ToastSetting>().set_is_timeout(false);
        ui.global::<ToastSetting>().invoke_set(msg, status);

        timer.start(
            TimerMode::SingleShot,
            std::time::Duration::from_secs(interval),
            move || {
                ui.global::<ToastSetting>().set_is_timeout(true);
            },
        );
    });
}
