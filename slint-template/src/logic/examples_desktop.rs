use crate::{
    logic_cb,
    slint_generatedAppWindow::{AppWindow, SideBarEntry as UISideBarEntry},
};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

pub fn init(ui: &AppWindow) {
    logic_cb!(generate_search_values, ui, entries);
    logic_cb!(get_sidebar_key_from_search_values, ui, entries, text);
    logic_cb!(generate_sound_data, ui, counts);
}

fn generate_search_values(
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

fn get_sidebar_key_from_search_values(
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

fn generate_sound_data(_ui: &AppWindow, counts: i32) -> ModelRc<f32> {
    use rand::RngExt;

    let mut rng = rand::rng();
    let data: Vec<f32> = (0..counts).map(|_| rng.random_range(-1.0..=1.0)).collect();
    ModelRc::new(VecModel::from_slice(&data))
}
