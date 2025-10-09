//! About panel logic module
//! 
//! Handles initialization of the about panel with application information.

use crate::slint_generatedAppWindow::{AboutSetting, AppWindow};
use crate::{config, version::VERSION};
use slint::ComponentHandle;

/// Initializes the about panel with application information
/// 
/// Sets the application name and version in the about panel.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
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
