//! UI logic and callback management module
//!
//! Contains macros and initialization functions for connecting Slint UI callbacks
//! to Rust functions. Provides global access to UI components and utilities.

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

/// Macro to access the global Store component
///
/// # Parameters
/// - `$ui`: AppWindow instance
///
/// # Returns
/// - Reference to the global Store component
#[macro_export]
macro_rules! global_store {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Store>()
    };
}

/// Macro to access the global Logic component
///
/// # Parameters
/// - `$ui`: AppWindow instance
///
/// # Returns
/// - Reference to the global Logic component
#[macro_export]
macro_rules! global_logic {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Logic>()
    };
}

/// Macro to access the global Util component
///
/// # Parameters
/// - `$ui`: AppWindow instance
///
/// # Returns
/// - Reference to the global Util component
#[macro_export]
macro_rules! global_util {
    ($ui:expr) => {
        $ui.global::<crate::slint_generatedAppWindow::Util>()
    };
}

/// Macro to connect Slint callbacks to Rust functions
///
/// Creates a callback connection with proper weak reference handling
/// to prevent memory leaks.
///
/// # Parameters
/// - `$callback_name`: Name of the callback function
/// - `$ui`: AppWindow instance
/// - `$($arg:ident),*`: Callback arguments
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

/// Macro to implement serde Serialize and Deserialize for Slint enums
///
/// Automatically generates serde implementations that convert between
/// enum variants and their string representations.
///
/// # Parameters
/// - `$ty`: Enum type name
/// - `$($arg:ident),+`: Enum variant names
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

// Example: impl_c_like_enum_convert!(Foo, Bar, A, B, C);
#[allow(unused_macros)]
macro_rules! impl_c_like_enum_convert {
    ($enum1:ident, $enum2:ident, $($variant:ident),*) => {
        impl From<$enum1> for $enum2 {
            fn from(value: $enum1) -> Self {
                match value {
                    $(
                        $enum1::$variant => $enum2::$variant,
                    )*
                }
            }
        }

        impl From<$enum2> for $enum1 {
            fn from(value: $enum2) -> Self {
                match value {
                    $(
                        $enum2::$variant => $enum1::$variant,
                    )*
                }
            }
        }
    };
}

/// Initializes all UI logic modules
///
/// Sets up callbacks and initializes platform-specific logic modules.
///
/// # Parameters
/// - `ui`: Reference to the application window
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
