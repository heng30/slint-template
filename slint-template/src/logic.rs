use crate::slint_generatedAppWindow::AppWindow;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod about;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod util;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod setting;

#[cfg(any(feature = "desktop", feature = "mobile"))]
mod clipboard;

mod confirm_dialog;
mod popup_action;
mod toast;
mod tr;

#[cfg(feature = "desktop")]
mod examples_desktop;

#[cfg(feature = "android")]
mod examples_mobile;

#[cfg(feature = "web")]
mod examples_web;

#[macro_export]
macro_rules! global_store {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Store>()
    };
}

#[macro_export]
macro_rules! global_logic {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Logic>()
    };
}

#[macro_export]
macro_rules! global_util {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Util>()
    };
}

#[macro_export]
macro_rules! logic_cb {
    ($callback_name:ident, $ui:expr, $($arg:ident),*) => {
        {{
            let ui_weak = $ui.as_weak();
            paste::paste! {
                crate::global_logic!($ui)
                    .[<on_ $callback_name>](move |$($arg),*| {
                        $callback_name(&ui_weak.unwrap(), $($arg),*)
                    });
            }
        }}
    };
}

#[macro_export]
macro_rules! impl_slint_enum_serde {
    ($ty:ident, $($arg:ident),+) => {
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                match self {
                    $(
                        $ty::$arg => serializer.serialize_str(stringify!($arg)),
                    )+
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $ty;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(&format!("a string representing {}", stringify!($ty)))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$ty, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            $(
                                stringify!($arg) => Ok($ty::$arg),
                            )+
                            _ => Err(E::custom(format!(
                                "unknown {} variant: {}",
                                stringify!($ty),
                                value
                            ))),
                        }
                    }
                }

                deserializer.deserialize_str(EnumVisitor)
            }
        }
    };
}

pub fn init(ui: &AppWindow) {
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        util::init(ui);
        clipboard::init(ui);
        about::init(ui);
        setting::init(ui);
    }

    toast::init(ui);
    confirm_dialog::init(ui);
    popup_action::init(ui);

    {
        #[cfg(feature = "desktop")]
        examples_desktop::init(ui);

        #[cfg(any(feature = "android"))]
        examples_mobile::init(ui);

        #[cfg(feature = "web")]
        examples_web::init(ui);
    }
}

#[cfg(test)]
mod test {
    use crate::impl_slint_enum_serde;

    #[derive(Debug, Clone)]
    enum MyEnum {
        VariantA,
        VariantB,
    }

    impl_slint_enum_serde!(MyEnum, VariantA, VariantB);

    // cargo test test_slint_enum_serde -- --no-capture
    #[test]
    fn test_impl_slint_enum_serde() {
        let va = serde_json::to_string(&MyEnum::VariantA).unwrap();
        let vb = serde_json::to_string(&MyEnum::VariantB).unwrap();
        println!("{}", va);
        println!("{}", vb);

        let va = serde_json::from_str::<MyEnum>(&va).unwrap();
        println!("{:?}", va);
    }
}
