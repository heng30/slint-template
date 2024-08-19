use crate::slint_generatedAppWindow::{AboutSetting, AppWindow};
use crate::{config::app_name, version::VERSION};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<AboutSetting>().set_title(slint::format!(
        "{} {}",
        app_name(),
        if VERSION.is_empty() {
            "v0.0.1"
        } else {
            VERSION
        }
    ));
}
