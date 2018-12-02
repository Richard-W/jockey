#![recursion_limit="128"] 

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod derive_parse_args;

#[proc_macro_derive(JockeyArguments)]
pub fn derive_arguments(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::DeriveInput);

    let parse_args = derive_parse_args::derive_parse_args(&input);

    let struct_ident: &syn::Ident = &input.ident;
    let result = quote!{
        impl jockey::Arguments for #struct_ident {
            fn to_args(self) -> Vec<String> {
                vec![]
            }

            #parse_args
        }
    };
    result.into()
}
