use util;

pub fn derive_emit_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut pushes = quote!{};

    for ref field in struct_def.fields {
        let field_ident = &field.ident;
        let long_option = &field.long_option;

        pushes.extend(quote!{
            result.extend(self.#field_ident.emit_args(#long_option.to_string()));
        });
    }

    let result = quote! {
        fn emit_args(&self) -> Vec<String> {
            let mut result: Vec<String> = Vec::new();
            use jockey::Emittable;

            #pushes

            result
        }
    };
    result.into()
}
