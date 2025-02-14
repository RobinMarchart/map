use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);
    let expanded = quote! {
    };
    TokenStream::from(expanded)
}
