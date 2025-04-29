use crate::slint_generatedAppWindow::{AppWindow, Logic};

use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<Logic>().on_web_debug(move |text| {
        log::debug!("{}", text);
    });
}
