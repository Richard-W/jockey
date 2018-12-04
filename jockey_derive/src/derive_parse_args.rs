use util;

pub fn derive_parse_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut parser_components = quote! {};

    for ref field in struct_def.fields {
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let long_option = &field.long_option;
        let span = field_ident.span();

        parser_components.extend(quote_spanned!{span=>
            match <#field_type as jockey::Parsable>::parse_arg(&mut iter, &#long_option.to_string()) {
                Some(Ok(val)) => { result.#field_ident = val; continue; }
                Some(Err(err)) => return Err(err),
                None => {},
            }
        });

        match &field.short_option {
            Some(ref short_option_char) => {
                let short_option = "-".to_string() + short_option_char;
                parser_components.extend(quote_spanned!{span=>
                    match <#field_type as jockey::Parsable>::parse_arg(&mut iter, &#short_option.to_string()) {
                        Some(Ok(val)) => { result.#field_ident = val; continue; }
                        Some(Err(err)) => return Err(err),
                        None => {},
                    }
                });
            }
            None => {},
        }
    }

    let struct_ident = &struct_def.ident;
    let result = quote!{
        fn parse_args(args: Vec<String>) -> jockey::Result<#struct_ident> {
            let mut result = <#struct_ident as Default>::default();
            let mut iter = args.iter().peekable();

            // Skip first argument which is the executable path.
            iter.next();

            loop {
                if iter.peek().is_none() { break; }

                #parser_components

                return Err(jockey::Error::UnknownOption(iter.peek().unwrap().to_string()));
            }

            Ok(result)
        }
    };
    result.into()
}

