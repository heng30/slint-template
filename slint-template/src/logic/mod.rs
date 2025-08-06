use crate::slint_generatedAppWindow::AppWindow;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod about;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod clipboard;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod util;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod setting;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod tr;

mod confirm_dialog;
mod popup_action;
mod toast;

#[cfg(feature = "desktop")]
mod examples_desktop;

#[cfg(feature = "android")]
mod examples_mobile;

#[cfg(feature = "web")]
mod examples_web;

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
