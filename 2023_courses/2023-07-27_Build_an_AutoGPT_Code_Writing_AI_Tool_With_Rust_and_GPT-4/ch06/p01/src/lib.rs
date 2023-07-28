#![allow(unused_macros)]

mod m01_decl_macro;
mod m03_proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloWorld)]
pub fn helloworld_object_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world!() {
                println!("Hello, world!");
            }
        }
    };

    TokenStream::from(expanded)
}
