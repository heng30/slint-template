use super::tr::tr;
use crate::{
    config,
    slint_generatedAppWindow::{AppPosType, AppWindow, Util},
    toast_warn,
};
use cutil::{
    self, number,
    rand::{self, Rng},
    time,
};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};
use std::str::FromStr;
use webbrowser::{self, Browser};

pub fn init(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_hide_window(move || {
        _ = ui_handle.unwrap().hide();
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_show_window(move || {
        _ = ui_handle.unwrap().show();
    });

    ui.global::<Util>().on_close_window(move || {
        std::process::exit(0);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_min_window(move |minimized| {
        ui_handle.unwrap().window().set_minimized(minimized);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_min_window(move || ui_handle.unwrap().window().is_minimized());

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_max_window(move |maximized| {
        ui_handle.unwrap().window().set_maximized(maximized);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_max_window(move || ui_handle.unwrap().window().is_maximized());

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_fullscreen(move |fullscreen| {
        ui_handle.unwrap().window().set_fullscreen(fullscreen);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_is_fullscreen(move || ui_handle.unwrap().window().is_fullscreen());

    let ui_handle = ui.as_weak();
    ui.global::<Util>()
        .on_get_scale_factor(move || ui_handle.unwrap().window().scale_factor());

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_get_current_pos(move || {
        let ui = ui_handle.unwrap();
        let scale = ui.window().scale_factor();
        let pos = slint::LogicalPosition::from_physical(ui.window().position(), scale);

        AppPosType { x: pos.x, y: pos.y }
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_set_current_pos(move |pos| {
        let ui = ui_handle.unwrap();
        let scale = ui.window().scale_factor();
        let pos = slint::PhysicalPosition::from_logical(
            slint::LogicalPosition { x: pos.x, y: pos.y },
            scale,
        );

        ui.window().set_position(pos);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_update_window_size(move || {
        let ui = ui_handle.unwrap();
        let preference = config::preference();

        let scale = ui.window().scale_factor();
        let psize = slint::PhysicalSize::from_logical(
            slint::LogicalSize {
                width: preference.win_width as f32,
                height: preference.win_height as f32,
            },
            scale,
        );
        ui.window().set_size(psize);
    });

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_set_window_center(move || {
        let ui = ui_handle.unwrap();
        let preference = config::preference();

        let scale = ui.window().scale_factor();
        let _psize = slint::PhysicalSize::from_logical(
            slint::LogicalSize {
                width: preference.win_width as f32,
                height: preference.win_height as f32,
            },
            scale,
        );

        todo!("Get the monitor size to calculate the position");
        // ui.window().set_position(pos)
    });

    ui.global::<Util>().on_string_fixed2(move |n| {
        let n = n.to_string().parse::<f32>().unwrap_or(0.0f32);
        slint::format!("{:2}", (n * 100.0).round() / 100.0)
    });

    ui.global::<Util>()
        .on_float_fixed2(move |n| slint::format!("{:2}", (n * 100.0).round() / 100.0));

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_open_url(move |browser, url| {
        let ui = ui_handle.unwrap();

        let browser = Browser::from_str(&browser.to_lowercase()).unwrap_or_default();

        let browser = if browser.exists() {
            browser
        } else {
            Browser::Default
        };

        if let Err(e) = webbrowser::open_browser(browser, url.as_str()) {
            toast_warn!(
                ui,
                format!("{}{}: {:?}", tr("Open link failed"), tr("Reason"), e)
            );
        }
    });

    ui.global::<Util>()
        .on_remove_str_items_after(move |items, index| {
            let index = i32::max(0, index) as usize;

            let items = items
                .as_any()
                .downcast_ref::<VecModel<SharedString>>()
                .expect("We know we set a VecModel earlier");

            if index >= items.row_count() - 1 {
                return;
            }

            let count = items.row_count() - 1 - index;
            for _ in 0..count {
                items.remove(index + 1);
            }
        });

    ui.global::<Util>()
        .on_append_str_to_items(move |items, text| {
            let items = items
                .as_any()
                .downcast_ref::<VecModel<SharedString>>()
                .expect("We know we set a VecModel earlier");

            items.push(text);
        });

    ui.global::<Util>()
        .on_search_str_items_by(move |items, text| {
            if text.is_empty() {
                return ModelRc::default();
            }

            let items = items
                .iter()
                .filter(|item| item.to_lowercase().contains(text.to_lowercase().as_str()))
                .collect::<Vec<_>>();

            ModelRc::new(VecModel::from_slice(&items[..]))
        });

    ui.global::<Util>()
        .on_find_tree_children_nodes(move |items, target_node| {
            if target_node.is_empty() {
                return ModelRc::default();
            }

            let items = items
                .iter()
                .filter(|item| item.parent_node == target_node)
                .collect::<Vec<_>>();

            ModelRc::new(VecModel::from_slice(&items[..]))
        });

    ui.global::<Util>()
        .on_format_number_with_commas(move |number_str| {
            number::format_number_with_commas(number_str.as_str()).into()
        });

    ui.global::<Util>()
        .on_local_now(move |format| time::local_now(format.as_str()).into());

    ui.global::<Util>().on_text_len(move |text| {
        let chars_text = text.chars().collect::<Vec<_>>();
        chars_text.len() as i32
    });

    ui.global::<Util>()
        .on_rand_int(move |low, up| rand::thread_rng().gen_range(low..up) as i32);

    ui.global::<Util>()
        .on_split_and_join_string(move |input, length, sep| {
            cutil::str::split_string_to_fixed_length_parts(input.as_str(), length as usize)
                .join(sep.as_str())
                .into()
        });

    cfg_if::cfg_if! {
        if #[cfg(feature = "qrcode")] {
            init_qrcode(ui);
        }
    }
}

#[cfg(feature = "qrcode")]
pub fn init_qrcode(ui: &AppWindow) {
    use crate::slint_generatedAppWindow::Icons;
    use image::Rgb;
    use qrcode::QrCode;
    use slint::{Image, Rgb8Pixel, SharedPixelBuffer};

    let ui_handle = ui.as_weak();
    ui.global::<Util>().on_qr_code(move |text| {
        let ui = ui_handle.unwrap();
        match QrCode::new(text) {
            Ok(code) => {
                let qrc = code.render::<Rgb<u8>>().build();

                let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    qrc.as_raw(),
                    qrc.width(),
                    qrc.height(),
                );
                Image::from_rgb8(buffer)
            }
            _ => ui.global::<Icons>().get_no_data(),
        }
    });
}
