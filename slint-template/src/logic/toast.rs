use crate::slint_generatedAppWindow::{AppWindow, ToastSetting, ToastStatus, Util};
use slint::{ComponentHandle, Timer, TimerMode, Weak};

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

#[allow(dead_code)]
pub fn async_toast_warn(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        ui.unwrap()
            .global::<Util>()
            .invoke_show_toast(slint::format!("{}", msg), ToastStatus::Warning);
    });
}

#[allow(dead_code)]
pub fn async_toast_success(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        ui.unwrap()
            .global::<Util>()
            .invoke_show_toast(slint::format!("{}", msg), ToastStatus::Success);
    });
}

#[allow(dead_code)]
pub fn async_toast_info(ui: Weak<AppWindow>, msg: String) {
    let _ = slint::invoke_from_event_loop(move || {
        ui.unwrap()
            .global::<Util>()
            .invoke_show_toast(slint::format!("{}", msg), ToastStatus::Info);
    });
}

pub fn init(ui: &AppWindow) {
    let timer = Timer::default();
    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_show_toast(move |msg, status| {
        let ui = ui_handle.unwrap();

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
