use util;

use proc_macro2::{TokenStream};
use syn::{Ident, Type};

fn get_parser_component(ident: &Ident, ty: &Type, option: &String) -> TokenStream {
    let span = ident.span();
    quote_spanned!{span=>
        {
            let parse_result = <#ty as jockey::Parsable>::parse_arg(&mut iter, &#option.to_string());
            match parse_result.blacklist {
                Some(val) => {
                    blacklist.insert(val.to_string());
                },
                None => {}
            }
            match parse_result.parsed {
                Some(Ok(val)) => { result.#ident = val; continue; }
                Some(Err(err)) => return Err(err),
                None => {},
            }
        }
    }
}

pub fn derive_parse_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut parser_components = quote! {};

    let mut unknown_args_field: Option<util::StructField> = None;

    for ref field in struct_def.fields {
        if field.unknown_args {
            if unknown_args_field.is_some() {
                panic!("Only one unknown_args field may be defined");
            }
            unknown_args_field = Some(field.clone());
        }

        match &field.long_option {
            Some(ref long_option) => {
                let long_option = String::from("--") + long_option;
                parser_components.extend(get_parser_component(&field.ident, &field.ty, &long_option));
            },
            None => {},
        }

        match &field.short_option {
            Some(ref short_option) => {
                let short_option = String::from("-") + short_option;
                parser_components.extend(get_parser_component(&field.ident, &field.ty, &short_option));
            }
            None => {},
        }
    }

    let unknown_args_component = match unknown_args_field {
        Some(field) => {
            let field_ident = &field.ident;
            let field_type = &field.ty;
            let span = field_ident.span();

            quote_spanned!{span=>
                match iter.next() {
                    Some(value) => <#field_type as std::iter::Extend<String>>::extend(&mut result.#field_ident, std::iter::once(value)),
                    None => {},
                }
            }
        }
        None => quote! {
            return Err(jockey::Error::UnknownOption(iter.peek().unwrap().to_string()));
        },
    };

    let struct_ident = &struct_def.ident;
    let result = quote!{
        fn parse_args<I> (args: I) -> jockey::Result<#struct_ident> where I : Iterator<Item = String> {
            let mut result = <#struct_ident as Default>::default();
            let mut blacklist: std::collections::HashSet<String> = std::collections::HashSet::new();

            let mut iter = args.peekable();

            // Skip first argument which is the executable path.
            iter.next();

            loop {
                match iter.peek() {
                    Some(arg) => {
                        if blacklist.contains(arg) {
                            return Err(jockey::Error::DuplicateOption(arg.to_string()));
                        }
                    },
                    None => {
                        break;
                    },
                }
                if iter.peek().is_none() { break; }

                #parser_components

                #unknown_args_component
            }

            Ok(result)
        }
    };
    result.into()
}

