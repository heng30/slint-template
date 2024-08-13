use crate::slint_generatedAppWindow::{AppWindow, Logic};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Logic>()
        .on_handle_confirm_dialog(move |handle_type, _user_data| {
            let _ui = ui_handle.unwrap();

            match handle_type.as_str() {
                // "remove-all-cache" => {
                //     ui.global::<Logic>().invoke_remove_all_cache();
                // }
                _ => (),
            }
        });
}
