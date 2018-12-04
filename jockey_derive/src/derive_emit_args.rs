use util;

pub fn derive_emit_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let struct_def = util::derive_input_to_struct_def(input);

    let mut pushes = quote!{};

    for ref field in struct_def.fields {
        let field_ident = &field.ident;
        let field_type = &field.ty;
        let long_option = String::from("--") + &field.long_option.clone().unwrap();
        let span = field.ident.span();

        pushes.extend(quote_spanned!{span=>
            result.extend(<#field_type as jockey::Emittable>::emit_args(&self.#field_ident, #long_option.to_string()));
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
