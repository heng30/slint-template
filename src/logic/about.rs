use crate::slint_generatedAppWindow::{AboutSetting, AppWindow};
use crate::version::VERSION;
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<AboutSetting>().set_title(slint::format!(
        "sollaw {}",
        if VERSION.is_empty() {
            "v0.0.1"
        } else {
            VERSION
        }
    ));
}
