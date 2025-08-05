use crate::slint_generatedAppWindow::{AppWindow, Logic, TestDialog};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    ui.global::<Logic>().on_show_mobile_test_dialog(move || {
        let dialog = TestDialog::new().unwrap();
        dialog.show().unwrap();
    });
}
