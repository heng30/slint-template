use crate::slint_generatedAppWindow::{AboutSetting, AppWindow};
use crate::{config, version::VERSION};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<AboutSetting>()
        .set_app_name(config::all().app_name.into());

    ui.global::<AboutSetting>().set_version(
        if VERSION.is_empty() {
            "v0.0.1"
        } else {
            VERSION
        }
        .into(),
    );
}
