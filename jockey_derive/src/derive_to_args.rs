use util;
pub fn derive_to_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut pushes = quote!{};

    for ref field in struct_def.fields {
        let field_ident = &field.ident;
        let long_option = &field.long_option;

        pushes.extend(match field.ty {
            util::Type::MandatoryString => quote!{
                result.push(#long_option.into());
                result.push(self.#field_ident.clone());
            },
            util::Type::OptionalString => quote! {
                match self.#field_ident {
                    Some(ref val) => {
                        result.push(#long_option.into());
                        result.push(val.clone());
                    },
                    None => {},
                }
            },
            util::Type::Flag => quote! {
                if self.#field_ident {
                    result.push(#long_option.into());
                }
            },
        });
    }

    let result = quote! {
        fn to_args(&self) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();

            #pushes

            result
        }
    };
    result.into()
}
