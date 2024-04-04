extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Message)]
pub fn message_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_message(&ast)
}

fn impl_message(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Message for #name {
            fn ser(&self) -> Vec<u8> {
                bincode::serialize(&self).unwrap()
            }
            fn deser(encoded: &Vec<u8>) -> Self {
                bincode::deserialize(&encoded).unwrap()
            }
        }
    };
    gen.into()
}
