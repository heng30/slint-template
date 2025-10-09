//! Popup action menu logic module
//! 
//! Handles actions triggered from popup context menus.

use crate::{
    global_logic,
    slint_generatedAppWindow::{AppWindow, PopupActionSetting},
};
use slint::ComponentHandle;

/// Initializes popup action menu callbacks
/// 
/// Sets up handlers for different popup menu actions.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
pub fn init(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    ui.global::<PopupActionSetting>()
        .on_action(move |action, _user_data| {
            let ui = ui_weak.unwrap();

            #[allow(clippy::single_match)]
            match action.as_str() {
                "remove-caches" => {
                    global_logic!(ui).invoke_remove_caches();
                }
                _ => (),
            }
        });
}
