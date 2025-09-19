use crate::{global_logic, global_util, slint_generatedAppWindow::AppWindow};
use slint::ComponentHandle;

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
