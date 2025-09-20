use crate::{
    logic_cb,
    slint_generatedAppWindow::{AppWindow, SideBarEntry as UISideBarEntry},
};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

pub fn init(ui: &AppWindow) {
    logic_cb!(on_generate_search_values, ui, entries);
    logic_cb!(on_get_sidebar_key_from_search_values, ui, entries, text);
}

fn on_generate_search_values(
    _ui: &AppWindow,
    entries: ModelRc<UISideBarEntry>,
) -> ModelRc<SharedString> {
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
}

fn on_get_sidebar_key_from_search_values(
    _ui: &AppWindow,
    entries: ModelRc<UISideBarEntry>,
    text: SharedString,
) -> SharedString {
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
}
