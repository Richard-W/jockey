use util;

pub fn derive_parse_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut decl_mandatories = quote! {};
    let mut check_mandatories = quote! {};
    let mut parser_components = quote! {};

    for ref field in struct_def.fields {
        let field_ident = &field.ident;
        let long_option = &field.long_option;

        match field.ty {
            util::Type::MandatoryString => {
                let mandatory_ident = syn::Ident::new(&format!("got_{}", field_ident), proc_macro2::Span::call_site());
                decl_mandatories.extend(quote! {
                    let mut #mandatory_ident: bool = false;
                });
                check_mandatories.extend(quote! {
                    if !#mandatory_ident {
                        panic!("Did not get mandatory argument {}", #long_option);
                    }
                });
                parser_components.extend(quote! {
                    if key == #long_option {
                        match iter.next() {
                            Some(val) => result.#field_ident = val.clone(),
                            None => panic!("Unexpected end of arguments vector"),
                        }
                        #mandatory_ident = true;
                        continue;
                    }
                });
            },

            util::Type::OptionalString => parser_components.extend(quote! {
                if key == #long_option {
                    match iter.next() {
                        Some(val) => result.#field_ident = Some(val.clone()),
                        None => panic!("Unexpected end of arguments vector"),
                    }
                    continue;
                }
            }),

            util::Type::Flag => parser_components.extend(quote! {
                if (key == #long_option) {
                    result.#field_ident = true;
                    continue;
                }
            }),
        }
    }

    let struct_ident = &struct_def.ident;
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

