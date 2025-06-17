use crate::slint_generatedAppWindow::{AppWindow, Logic};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

#[cfg(target_arch = "wasm32")]
pub fn init(ui: &AppWindow) {
    ui.global::<Logic>().on_web_debug(move |text| {
        log::debug!(text);
    });
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub fn init(ui: &AppWindow) {
    ui.global::<Logic>()
        .on_generate_search_values(move |entries| {
            let values = entries
                .iter()
                .flat_map(|entry| {
                    if entry.children.row_count() > 0 {
                        entry
                            .children
                            .iter()
                            .map(|item| item.title)
                            .collect::<Vec<_>>()
                    } else {
                        vec![entry.category]
                    }
                })
                .collect::<Vec<_>>();
            ModelRc::new(VecModel::from_slice(&values[..]))
        });

    ui.global::<Logic>()
        .on_get_sidebar_key_from_search_values(move |entries, text| {
            if text.is_empty() {
                return Default::default();
            }

            let entries = entries
                .iter()
                .flat_map(|entry| {
                    if entry.children.row_count() > 0 {
                        entry
                            .children
                            .iter()
                            .map(|item| (item.title, item.key))
                            .collect::<Vec<_>>()
                    } else {
                        vec![(entry.category, entry.key)]
                    }
                })
                .filter_map(|item| {
                    if item.0.to_lowercase().contains(text.to_lowercase().as_str()) {
                        Some(item.1)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if entries.is_empty() {
                Default::default()
            } else {
                entries[0].clone()
            }
        });
}
