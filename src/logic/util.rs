use super::tr::tr;
use crate::{
    slint_generatedAppWindow::{AppWindow, Util},
    toast_warn,
};
use cutil::{
    self, number,
    rand::{self, Rng},
    time,
};
use slint::ComponentHandle;
use std::str::FromStr;
use webbrowser::{self, Browser};

pub fn init(ui: &AppWindow) {
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
            toast_warn!(ui, format!("{}{}: {:?}", tr("Open link failed"), tr("Reason"), e));
        }
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
