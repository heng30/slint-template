use crate::{config, global_about, version::VERSION};
use crate::slint_generatedAppWindow::AppWindow;
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    global_about!(ui).set_app_name(config::all().app_name.into());

    global_about!(ui).set_version(
        if VERSION.is_empty() {
            "v0.0.1"
        } else {
            VERSION
        }
        .into(),
    );
}
