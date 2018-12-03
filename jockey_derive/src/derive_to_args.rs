pub fn derive_to_args(input: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let result = quote! {
        fn to_args(self) -> Vec<String> {
            vec![]
        }
    };
    result.into()
}
