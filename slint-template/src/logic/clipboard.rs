//! Clipboard operations module
//! 
//! Provides cross-platform clipboard functionality including copy and paste operations.
//! Supports different clipboard backends for various platforms (Wayland, X11, Android).

use super::tr::tr;
use crate::{global_logic, slint_generatedAppWindow::AppWindow, toast_success, toast_warn};
use anyhow::{bail, Result};
use slint::ComponentHandle;

/// Copies text to clipboard on desktop platforms
/// 
/// Supports both X11 and Wayland clipboard backends on Linux.
/// 
/// # Parameters
/// - `msg`: Text to copy to clipboard
/// 
/// # Returns
/// - `Result<()>` indicating success or failure
#[cfg(feature = "desktop")]
fn copy_to_clipboard(msg: &str) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        if super::util::is_wayland() && copy_to_wayland_clipboard(msg).is_ok() {
            return Ok(());
        }
    }

    use clipboard::{ClipboardContext, ClipboardProvider};
    let ctx: Result<ClipboardContext, _> = ClipboardProvider::new();

    match ctx {
        Ok(mut ctx) => match ctx.set_contents(msg.to_string()) {
            Err(e) => bail!("{e:?}"),
            _ => Ok(()),
        },
        Err(e) => bail!("{e:?}"),
    }
}

/// Pastes text from clipboard on desktop platforms
/// 
/// Supports both X11 and Wayland clipboard backends on Linux.
/// 
/// # Returns
/// - `Result<String>` containing the clipboard text
#[cfg(feature = "desktop")]
fn paste_from_clipboard() -> Result<String> {
    #[cfg(target_os = "linux")]
    {
        if super::util::is_wayland() {
            if let Ok(text) = paste_from_wayland_clipboard() {
                return Ok(text);
            }
        }
    }

    use clipboard::{ClipboardContext, ClipboardProvider};
    let ctx: Result<ClipboardContext, _> = ClipboardProvider::new();

    match ctx {
        Ok(mut ctx) => match ctx.get_contents() {
            Err(e) => bail!("{e:?}"),
            Ok(msg) => Ok(msg),
        },
        Err(e) => bail!("{e:?}"),
    }
}

/// Copies text to clipboard on Android platforms
/// 
/// # Parameters
/// - `msg`: Text to copy to clipboard
/// 
/// # Returns
/// - `Result<()>` indicating success or failure
#[cfg(feature = "android")]
fn copy_to_clipboard(msg: &str) -> Result<()> {
    match android_clipboard::set_text(msg.to_string()) {
        Err(e) => bail!("{e:?}"),
        _ => Ok(()),
    }
}

/// Pastes text from clipboard on Android platforms
/// 
/// # Returns
/// - `Result<String>` containing the clipboard text
#[cfg(feature = "android")]
fn paste_from_clipboard() -> Result<String> {
    match android_clipboard::get_text() {
        Err(e) => bail!("{e:?}"),
        Ok(msg) => Ok(msg),
    }
}

/// Copies text to Wayland clipboard using wl-copy command
/// 
/// # Parameters
/// - `text`: Text to copy to clipboard
/// 
/// # Returns
/// - `Result<()>` indicating success or failure
#[cfg(target_os = "linux")]
fn copy_to_wayland_clipboard(text: &str) -> Result<()> {
    duct::cmd!("wl-copy", text).run()?;

    Ok(())
}

/// Pastes text from Wayland clipboard using wl-paste command
/// 
/// # Returns
/// - `Result<String>` containing the clipboard text
#[cfg(target_os = "linux")]
fn paste_from_wayland_clipboard() -> Result<String> {
    Ok(duct::cmd!("wl-paste").read()?)
}

/// Initializes clipboard functionality
/// 
/// Sets up callbacks for copy and paste operations with proper error handling.
/// 
/// # Parameters
/// - `ui`: Reference to the application window
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
