use util;

pub fn derive_parse_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_ident: &syn::Ident = &input.ident;

    let struct_data: &syn::DataStruct = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Can only derive JockeyArguments from struct"),
    };

    let struct_fields: &syn::FieldsNamed = match struct_data.fields {
        syn::Fields::Named(ref fields) => fields,
        _ => panic!("Can only derive JockeyArguments from struct with named fields"),
    };

    let mut decl_mandatories = quote! {};
    let mut check_mandatories = quote! {};
    let mut parser_components = quote! {};

    for ref field in struct_fields.named.iter() {
        let field_ident = field.ident.clone().unwrap();
        let argument_key = "--".to_string() + &field_ident.to_string();

        match util::supported_type_to_string(&field.ty) {
            util::SupportedType::MandatoryString => {
                let mandatory_ident = syn::Ident::new(&format!("got_{}", field_ident), proc_macro2::Span::call_site());
                decl_mandatories.extend(quote! {
                    let mut #mandatory_ident: bool = false;
                });
                check_mandatories.extend(quote! {
                    if !#mandatory_ident {
                        panic!("Did not get mandatory argument {}", #argument_key);
                    }
                });
                parser_components.extend(quote! {
                    if key == #argument_key {
                        match iter.next() {
                            Some(val) => result.#field_ident = val.clone(),
                            None => panic!("Unexpected end of arguments vector"),
                        }
                        #mandatory_ident = true;
                        continue;
                    }
                });
            },

            util::SupportedType::OptionalString => parser_components.extend(quote! {
                if key == #argument_key {
                    match iter.next() {
                        Some(val) => result.#field_ident = Some(val.clone()),
                        None => panic!("Unexpected end of arguments vector"),
                    }
                    continue;
                }
            }),

            util::SupportedType::Flag => parser_components.extend(quote! {
                if (key == #argument_key) {
                    result.#field_ident = true;
                    continue;
                }
            }),

            _ => panic!("Unsupported type for JockeyArguments derivation"),
        }
    }

    let result = quote!{
        fn parse_args(args: Vec<String>) -> #struct_ident {
            let mut result = #struct_ident::new();
            let mut iter = args.iter();

            #decl_mandatories

            loop {
                match iter.next() {
                    Some(key) => {
                        #parser_components
                        panic!("Unknown flag: {}", key);
                    },
                    None => { break; },
                }
            }

            #check_mandatories

            result
        }
    };
    result.into()
}

