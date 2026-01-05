use crate::{
    global_logic, global_popup_action,
    slint_generatedAppWindow::AppWindow,
};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    global_popup_action!(ui).on_action(move |action, _user_data| {
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
