use parser;

use proc_macro2::{TokenStream};
use syn::{Ident, Type};

fn get_parser_component(ident: &Ident, ty: &Type, option: &String) -> TokenStream {
    let span = ident.span();
    quote_spanned!{span=>
        {
            let parse_result = <#ty as jockey::Parsable>::parse_arg(&mut iter, Some(#option.to_string()));
            match parse_result.blacklist {
                Some(val) => {
                    blacklist.insert(val.to_string());
                },
                None => {}
            }
            match parse_result.parsed {
                Some(Ok(val)) => {
                    result.#ident = <#ty as jockey::Parsable>::assign(result.#ident, val);
                    continue;
                },
                Some(Err(err)) => return Err(err),
                None => {},
            }
        }
    }
}

pub fn derive_parse_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    match parser::parse_data(input) {
        parser::Data::Struct(data) => {
            let mut parser_components = quote! {};
            let mut unknown_args_field: Option<parser::UnknownField> = None;
            for field in data.fields { match field {
                parser::Field::Ordinary(field) => {
                    match field.long {
                        Some(option) => {
                            parser_components.extend(get_parser_component(&field.ident, &field.ty, &option));
                        },
                        None => {},
                    }
                    match field.short {
                        Some(option) => {
                            parser_components.extend(get_parser_component(&field.ident, &field.ty, &option));
                        },
                        None => {},
                    }
                },
                parser::Field::Unknown(field) => {
                    if unknown_args_field.is_some() {
                        panic!("Only one unknown_args field may be defined");
                    }
                    unknown_args_field = Some(field);
                },
            }}

            let unknown_args_component = match unknown_args_field {
                Some(field) => {
                    let ident = &field.ident;
                    let ty = &field.ty;
                    let span = ident.span();
                    quote_spanned! { span =>
                        match iter.next() {
                            Some((_, value)) => <#ty as std::iter::Extend<String>>::extend(&mut result.#ident, std::iter::once(value)),
                            None => {},
                        }
                    }
                },
                None => quote! {
                    return Err(jockey::Error::UnknownOption(iter.peek().unwrap().1.to_string()));
                },
            };

            let struct_ident = &input.ident;
            quote! {
                fn parse_args<I> (args: I) -> jockey::Result<#struct_ident> where I : Iterator<Item = String> {
                    let mut result = <#struct_ident as Default>::default();
                    let mut blacklist: std::collections::HashSet<String> = std::collections::HashSet::new();
                    let mut iter = args.enumerate().peekable();

                    // Skip first argument which is the executable path.
                    iter.next();

                    loop {
                        match iter.peek() {
                            Some((_, arg)) => {
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
            }
        },
    }.into()
}

