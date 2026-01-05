use crate::{
    global_logic,
    slint_generatedAppWindow::{AppWindow, TestDialog},
};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    global_logic!(ui).on_show_mobile_test_dialog(move || {
        let dialog = TestDialog::new().unwrap();
        dialog.show().unwrap();
    });
}
