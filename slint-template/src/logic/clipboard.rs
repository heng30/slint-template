use super::tr::tr;
use crate::{global_logic, slint_generatedAppWindow::AppWindow, toast_success, toast_warn};
use anyhow::Result;
use slint::ComponentHandle;

fn copy_to_clipboard(msg: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        if super::util::is_wayland() && copy_to_wayland_clipboard(msg).is_ok() {
            return Ok(());
        }
    }

    let mut ctx = arboard::Clipboard::new()?;
    ctx.set_text(msg)?;
    Ok(())
}

fn paste_from_clipboard() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        if super::util::is_wayland() {
            if let Ok(text) = paste_from_wayland_clipboard() {
                return Ok(text);
            }
        }
    }

    let mut ctx = arboard::Clipboard::new()?;
    Ok(ctx.get_text()?)
}

#[cfg(feature = "android")]
fn copy_to_clipboard(msg: &str) -> Result<()> {
    match android_clipboard::set_text(msg.to_string()) {
        Err(e) => bail!("{e:?}"),
        _ => Ok(()),
    }
}

#[cfg(feature = "android")]
fn paste_from_clipboard() -> Result<String> {
    match android_clipboard::get_text() {
        Err(e) => bail!("{e:?}"),
        Ok(msg) => Ok(msg),
    }
}

#[cfg(target_os = "linux")]
fn copy_to_wayland_clipboard(text: &str) -> Result<()> {
    duct::cmd!("wl-copy", text).run()?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn paste_from_wayland_clipboard() -> Result<String> {
    Ok(duct::cmd!("wl-paste").read()?)
}

pub fn init(ui: &AppWindow) {
    let ui_weak = ui.as_weak();
    global_logic!(ui).on_copy_to_clipboard(move |msg| {
        let ui = ui_weak.unwrap();
        match copy_to_clipboard(&msg) {
            Err(e) => toast_warn!(
                ui,
                format!("{}. {}: {e:?}", tr("Copy failed"), tr("Reason"))
            ),
            _ => toast_success!(ui, tr("Copy success")),
        }
    });

    let ui_weak = ui.as_weak();
    global_logic!(ui).on_paste_from_clipboard(move || {
        let ui = ui_weak.unwrap();
        match paste_from_clipboard() {
            Err(e) => {
                toast_warn!(
                    ui,
                    format!("{}. {}: {e:?}", tr("Paste failed"), tr("Reason"))
                );
                slint::SharedString::default()
            }
            Ok(msg) => msg.into(),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard() -> Result<()> {
        let msg = "hello world";
        copy_to_clipboard(msg)?;
        let res = paste_from_clipboard()?;

        assert_eq!(msg, res);
        Ok(())
    }
}
