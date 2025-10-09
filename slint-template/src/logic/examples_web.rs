//! Web-specific example logic module
//! 
//! Contains web-specific UI logic and debugging functionality.

use crate::{global_logic, slint_generatedAppWindow::AppWindow};

use slint::ComponentHandle;

/// Initializes web-specific example logic
/// 
/// Sets up callbacks for web-specific debugging functionality.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
pub fn init(ui: &AppWindow) {
    global_logic!(ui).on_web_debug(move |text| {
        log::debug!("{}", text);
    });
}
