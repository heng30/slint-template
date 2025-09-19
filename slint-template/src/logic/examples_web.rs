use crate::{slint_generatedAppWindow::{AppWindow, Logic}, global_logic}

use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    global_logic!(ui).on_web_debug(move |text| {
        log::debug!("{}", text);
    });
}
