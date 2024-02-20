// procedural macros
extern crate proc_macro;

use chrono::{Local, SecondsFormat};
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn do_nothing(input: TokenStream) -> TokenStream {
    println!("==> Print This Message While Compiling");

    input
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[Info] ".to_owned();

    for token in input {
        let token_string = token.to_string();

        match token_string.as_str() {
            "[TIME]" => {
                // let now = Utc::now().time().to_string();
                let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);
                output.push_str(&format!("{}", now));
            }
            _ => output.push_str(&format!(" {}", token_string)),
        }
    }

    TokenStream::from(quote! {
        println!("{}", #output);
    })
}
