use crate::slint_generatedAppWindow::{AppWindow, Logic};
use slint::{ComponentHandle, Model, ModelRc, VecModel};

pub fn init(ui: &AppWindow) {
    ui.global::<Logic>()
        .on_generate_search_values(move |entries| {
            let values = entries.iter().map(|entry| entry.title).collect::<Vec<_>>();
            ModelRc::new(VecModel::from_slice(&values[..]))
        });

    ui.global::<Logic>()
        .on_get_component_index_from_search_values(move |entries, text| {
            let entries = entries
                .iter()
                .filter(|item| item.title.to_lowercase().contains(text.to_lowercase().as_str()))
                .collect::<Vec<_>>();

            if entries.is_empty() {
                Default::default()
            } else {
                entries[0].clone()
            }
        });
}
