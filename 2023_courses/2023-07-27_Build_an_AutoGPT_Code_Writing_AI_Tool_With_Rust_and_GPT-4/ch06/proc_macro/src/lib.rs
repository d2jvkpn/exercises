use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloWorld)]
pub fn helloworld_object_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as the name of the type to implement
    let name = input.ident;

    // Generate the code to paste into the user's program
    let expanded = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, world!");
            }
        }
    };

    TokenStream::from(expanded)
}
