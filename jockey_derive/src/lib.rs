#![recursion_limit="128"] 

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

mod derive_parse_args;
mod derive_emit_args;
mod util;

/// Implementation of `#[derive(JockeyArguments)]` (don't use this directly).
#[proc_macro_derive(JockeyArguments, attributes(jockey))]
pub fn derive_arguments(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);

    let parse_args = derive_parse_args::derive_parse_args(&input);
    let to_args = derive_emit_args::derive_emit_args(&input);

    let struct_ident: &syn::Ident = &input.ident;
    let result = quote!{
        impl jockey::Arguments for #struct_ident {
            #to_args

            #parse_args
        }
    };
    result.into()
}
