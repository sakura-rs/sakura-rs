use from_binary_derive::impl_from_binary;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod from_binary_derive;

#[proc_macro_derive(FromBinary)]
pub fn from_binary_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_from_binary(input).into()
}
