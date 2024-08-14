use crate::slint_generatedAppWindow::{AppWindow, Util, Logic};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_handle_confirm_dialog(move |handle_type, _user_data| {
            let ui = ui_handle.unwrap();

            match handle_type.as_str() {
                "remove-all-cache" => {
                    ui.global::<Logic>().invoke_remove_all_cache();
                }
                _ => (),
            }
        });
}
