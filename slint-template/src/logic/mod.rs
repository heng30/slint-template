use crate::slint_generatedAppWindow::AppWindow;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod about;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod util;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod setting;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod clipboard;

mod confirm_dialog;
mod popup_action;
mod toast;
mod tr;

#[cfg(feature = "desktop")]
mod examples_desktop;

#[cfg(feature = "android")]
mod examples_mobile;

#[cfg(feature = "web")]
mod examples_web;

#[macro_export]
macro_rules! global_store {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Store>()
    };
}

#[macro_export]
macro_rules! global_logic {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Logic>()
    };
}

#[macro_export]
macro_rules! global_util {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Util>()
    };
}

#[macro_export]
macro_rules! logic_cb {
    ($callback_name:ident, $ui:expr, $($arg:ident),*) => {
        {{
            let ui_weak = $ui.as_weak();
            crate::global_logic!($ui)
                .$callback_name(move |$($arg),*| {
                    $callback_name(&ui_weak.unwrap(), $($arg),*)
                });
        }}
    };
}

pub fn init(ui: &AppWindow) {
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        util::init(ui);
        clipboard::init(ui);
        about::init(ui);
        setting::init(ui);
    }

    toast::init(ui);
    confirm_dialog::init(ui);
    popup_action::init(ui);

    {
        #[cfg(feature = "desktop")]
        examples_desktop::init(ui);

        #[cfg(any(feature = "android"))]
        examples_mobile::init(ui);

        #[cfg(feature = "web")]
        examples_web::init(ui);
    }
}
