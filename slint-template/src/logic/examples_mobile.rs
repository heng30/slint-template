//! Mobile-specific example logic module
//! 
//! Contains mobile-specific UI logic and dialog examples.

use crate::{
    global_logic,
    slint_generatedAppWindow::{AppWindow, TestDialog},
};
use slint::ComponentHandle;

/// Initializes mobile-specific example logic
/// 
/// Sets up callbacks for mobile-specific UI components.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
pub fn init(ui: &AppWindow) {
    global_logic!(ui).on_show_mobile_test_dialog(move || {
        let dialog = TestDialog::new().unwrap();
        dialog.show().unwrap();
    });
}
