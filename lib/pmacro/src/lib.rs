//! A procedural macro library for Slint UI framework integration.
//!
//! This library provides the `SlintFromConvert` derive macro that automatically generates
//! bidirectional conversion implementations between Rust structs and Slint UI types.
//!
//! # Features
//!
//! - Automatic `From` trait implementations between Rust structs and Slint UI types
//! - Support for vector field mapping between `Vec<T>` and Slint's `ModelRc<T>`
//! - Customizable field mappings using attributes
//! - Default value handling for UI types
//!
//! # Usage
//!
//! ```ignore
//! use pmacro::SlintFromConvert;
//!
//! // Define your UI type (should use slint::ModelRc in real usage)
//! #[derive(Default)]
//! struct UIType {
//!     name: String,
//!     age: u32,
//!     ui_field_name: std::sync::Arc<Vec<String>>,
//! }
//!
//! // Define your Rust struct with the macro
//! #[derive(SlintFromConvert)]
//! #[from("UIType")]
//! pub struct MyStruct {
//!     pub name: String,
//!     pub age: u32,
//!     #[vec(from = "ui_field_name")]
//!     pub items: Vec<String>,
//! }
//! ```
//!
//! This will generate `From<MyStruct> for UIType` and `From<UIType> for MyStruct` implementations.

// cargo expand --bin pmacro

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Derive macro for bidirectional conversion between Rust structs and Slint UI types.
///
/// This macro generates `From` trait implementations for converting between a Rust struct
/// and a Slint UI type, handling field mappings and vector conversions automatically.
///
/// # Attributes
///
/// - `#[from("UIType")]`: Specifies the target Slint UI type for conversion
/// - `#[vec_ui("field_name")]`: Creates an empty vector field in the UI type
/// - `#[vec(from = "ui_field_name")]`: Maps a Rust vector field to a UI field
///
/// # Example
///
/// ```ignore
/// use pmacro::SlintFromConvert;
///
/// // Define UI type (should use slint::ModelRc in real usage)
/// #[derive(Default)]
/// struct UIUser {
///     name: String,
///     items: std::sync::Arc<Vec<String>>,
///     empty_items: std::sync::Arc<Vec<u32>>,
/// }
///
/// // Define Rust struct with macro
/// #[derive(SlintFromConvert)]
/// #[from("UIUser")]
/// #[vec_ui("empty_items")]
/// struct User {
///     name: String,
///     #[vec(from = "items")]
///     user_items: Vec<String>,
/// }
/// ```
#[proc_macro_derive(SlintFromConvert, attributes(from, vec, vec_ui))]
pub fn from_convert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut target_type = None;
    let mut vec_names_ui = vec![];
    let mut vec_field_mappings = std::collections::HashMap::new();

    for attr in &input.attrs {
        // find `#[from("Type")]`
        if attr.path().is_ident("from") {
            match attr.parse_args::<LitStr>() {
                Ok(lit) => {
                    target_type = Some(syn::parse_str::<syn::Path>(&lit.value()).unwrap());
                }
                Err(e) => {
                    eprintln!("{e:?}");
                    panic!("parse args failed");
                }
            }
        }

        // find `#[vec_ui("vec_name")]`
        if attr.path().is_ident("vec_ui") {
            match attr.parse_args::<LitStr>() {
                Ok(lit) => {
                    vec_names_ui.push(syn::parse_str::<syn::Path>(&lit.value()).unwrap());
                }
                Err(e) => {
                    eprintln!("{e:?}");
                    panic!("parse args failed");
                }
            }
        }
    }

    let target_type = target_type.expect("Must specify target type with #[from(\"Type\")]");

    let fields = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            fields_named.named
        } else {
            panic!("SlintFromConvert only works on structs with named fields");
        }
    } else {
        panic!("SlintFromConvert only works on structs");
    };

    // Process field-level vec attributes
    for field in &fields {
        let field_name = field.ident.as_ref().unwrap();

        for attr in &field.attrs {
            if attr.path().is_ident("vec") {
                match attr.parse_args::<syn::Meta>() {
                    Ok(syn::Meta::NameValue(meta_name_value))
                        if meta_name_value.path.is_ident("from") =>
                    {
                        if let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }) = &meta_name_value.value
                        {
                            let ui_field_name =
                                syn::parse_str::<syn::Path>(&lit_str.value()).unwrap();
                            vec_field_mappings.insert(field_name.to_string(), ui_field_name);
                        }
                    }
                    _ => {
                        panic!(
                            "Invalid #[vec] attribute format. Expected #[vec(name = \"field_name\")]"
                        );
                    }
                }
            }
        }
    }

    let field_conversions = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();

        // Check if this field is mapped to a UI field
        let is_vec_field = vec_field_mappings.contains_key(&field_name_str);

        if is_vec_field {
            None
        } else {
            Some(quote! {
                #field_name: entry.#field_name.into()
            })
        }
    });

    let field_conversions_duplicta = field_conversions.clone();

    // Handle field-level vec mappings
    let field_vec_conversions = vec_field_mappings.iter().map(|(field_name, ui_field_name)| {
        let field_ident = syn::parse_str::<syn::Ident>(field_name).unwrap();
        quote! {
            #field_ident: entry.#ui_field_name.iter().map(|item| item.clone().into()).collect::<Vec<_>>()
        }
    });

    let field_vec_conversions_slint =
        vec_field_mappings
            .iter()
            .map(|(field_name, ui_field_name)| {
                let field_ident = syn::parse_str::<syn::Ident>(field_name).unwrap();
                quote! {
                 #ui_field_name: slint::ModelRc::new(
                        entry
                            .#field_ident
                            .into_iter()
                            .map(|item| item.into())
                            .collect::<slint::VecModel<_>>()
                    )
                }
            });

    let vec_name_ui_conversions_slint = vec_names_ui.iter().map(|name| {
        quote! {
         #name: slint::ModelRc::new(slint::VecModel::default())
        }
    });

    let expanded = quote! {
        impl From<#name> for #target_type {
            fn from(entry: #name) -> Self {
                Self {
                    #(#field_conversions,)*
                    #(#field_vec_conversions_slint,)*
                    #(#vec_name_ui_conversions_slint,)*
                    ..Default::default()
                }
            }
        }

        impl From<#target_type> for #name {
            fn from(entry: #target_type) -> Self {
                Self {
                    #(#field_conversions_duplicta,)*
                    #(#field_vec_conversions,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
