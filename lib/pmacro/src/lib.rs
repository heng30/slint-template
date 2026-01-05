use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

#[proc_macro_derive(SlintFromConvert, attributes(from, vec, vec_ui))]
pub fn from_convert_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut target_type = None;
    let mut vec_names_ui = vec![];
    let mut vec_field_mappings = std::collections::HashMap::new();

    for attr in &input.attrs {
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
