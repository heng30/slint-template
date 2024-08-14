use crate::slint_generatedAppWindow::AppWindow;

mod about;
mod clipboard;
mod confirm_dialog;
mod setting;
mod toast;
mod tr;
mod util;

pub fn init(ui: &AppWindow) {
    util::init(ui);
    clipboard::init(ui);
    toast::init(ui);
    confirm_dialog::init(ui);
    about::init(ui);
    setting::init(ui);
}
