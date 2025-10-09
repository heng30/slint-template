//! Confirmation dialog logic module
//! 
//! Handles confirmation dialog callbacks for various application actions.

use crate::{global_logic, global_util, slint_generatedAppWindow::AppWindow};
use slint::ComponentHandle;

/// Initializes confirmation dialog callbacks
/// 
/// Sets up handlers for different confirmation dialog actions.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
pub fn init(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    global_util!(ui).on_handle_confirm_dialog(move |handle_type, _user_data| {
        let ui = ui_weak.unwrap();

        match handle_type.as_str() {
            "remove-caches" => {
                global_logic!(ui).invoke_remove_caches();
            }
            "uninstall" => {
                global_logic!(ui).invoke_uninstall();
            }
            "close-window" => {
                global_util!(ui).invoke_close_window();
            }
            _ => (),
        }
    });
}
