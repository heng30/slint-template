// cargo expand --bin pmacro

use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

#[proc_macro_derive(SlintFromConvert, attributes(from, vec, vec_ui))]
pub fn from_convert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut target_type = None;
    let mut vec_names = vec![];
    let mut vec_names_ui = vec![];
    let mut vec_names_set = HashSet::new();

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

        // find `#[vec("vec_name")]`
        if attr.path().is_ident("vec") {
            match attr.parse_args::<LitStr>() {
                Ok(lit) => {
                    vec_names.push(syn::parse_str::<syn::Path>(&lit.value()).unwrap());
                    vec_names_set.insert(lit.value());
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

    let field_conversions = fields.iter().filter_map(|field| {
        let field_name = &field.ident;

        if vec_names_set.contains(&field_name.as_ref().unwrap().to_string()) {
            None
        } else {
            Some(quote! {
                #field_name: entry.#field_name.into()
            })
        }
    });

    let field_conversions_duplicta = field_conversions.clone();

    let vec_name_conversions = vec_names.iter().map(|name| {
        quote! {
            #name: entry.#name.iter().map(|item| item.clone().into()).collect::<Vec<_>>()
        }
    });

    let vec_name_conversions_slint = vec_names.iter().map(|name| {
        quote! {
         #name: slint::ModelRc::new(
                entry
                    .#name
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
                    #(#vec_name_conversions_slint,)*
                    #(#vec_name_ui_conversions_slint,)*
                    ..Default::default()
                }
            }
        }

        impl From<#target_type> for #name {
            fn from(entry: #target_type) -> Self {
                Self {
                    #(#field_conversions_duplicta,)*
                    #(#vec_name_conversions,)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
