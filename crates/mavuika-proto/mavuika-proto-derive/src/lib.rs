use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Meta, MetaList};

#[proc_macro_derive(CmdID, attributes(cmdid))]
pub fn cmd_id_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let id = match input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("cmdid"))
    {
        Some(attr) => match attr.meta {
            Meta::List(MetaList { ref tokens, .. }) => tokens.into_token_stream(),
            _ => panic!("Invalid cmdid attribute value"),
        },
        _ => 0u16.into_token_stream(),
    };

    TokenStream::from(quote! {
        impl crate::CmdID for #struct_name {
            const CMD_ID: u16 = #id;
        }
    })
}
