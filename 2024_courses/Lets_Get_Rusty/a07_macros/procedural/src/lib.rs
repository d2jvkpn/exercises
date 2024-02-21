// procedural macros

use chrono::{Local, SecondsFormat};
use darling::{FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn;

/// proc_macro
#[proc_macro]
pub fn do_nothing(input: TokenStream) -> TokenStream {
    println!("==> print this message while compiling");

    input
}

#[proc_macro]
pub fn log_info(input: TokenStream) -> TokenStream {
    let mut output = "[INFO] ".to_owned();

    dbg!(&input.to_string());

    // input: "[TIME] start program ..."
    // 6 tokens: ["[TIME]", "start", "program", ".", ".", "."]
    for token in input {
        // dbg!(&token);

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

#[proc_macro]
pub fn info(input: TokenStream) -> TokenStream {
    let msg = input.to_string();
    let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);

    // let output = String::with_capacity(??);
    let output = format!("{} INFO - {}", now, msg.trim());

    TokenStream::from(quote! {
        println!("{}", #output);
    })
}

/// proc_macro_derive
#[proc_macro_derive(Log)]
pub fn derive_log(input: TokenStream) -> TokenStream {
    // TokenStream::new()

    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    /*
    pub struct DeriveInput {
        pub attrs: Vec<Attribue>,
        pub vis: Visibility,
        pub idnet: Ident,
        pub generics: Generics,
        pub data: Data,
    }
     */
    let name: &syn::Ident = &ast.ident;

    let trait_impl = quote! {
        use chrono::{Local, SecondsFormat};

        impl Log for #name {
            fn info(&self, msg: &str) {
                let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);
                println!("{} INFO - {}: {}", now, stringify!(#name), msg.trim());
            }

            fn warn(&self, msg: &str) {
                let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);
                eprintln!("{} WARN - {}: {}", now, stringify!(#name), msg.trim());
            }

            fn error(&self, msg: &str) {
                let now = Local::now().to_rfc3339_opts(SecondsFormat::Millis, true);
                eprintln!("{} ERROR - {}: {}", now, stringify!(#name), msg.trim());
            }
        }
    };

    trait_impl.into()
}

/// proc_macro_attribute
#[derive(darling::FromMeta)]
struct MacroArgs {
    #[darling(default)]
    verbose: bool,
}

#[proc_macro_attribute]
pub fn log_call(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match darling::ast::NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };

    let mut input = syn::parse_macro_input!(input as syn::ItemFn);

    let attr_args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    impl_log_call(&attr_args, &mut input)
}

fn impl_log_call(attr_args: &MacroArgs, input: &mut syn::ItemFn) -> TokenStream {
    // TokenStream::new()

    let fn_name = &input.sig.ident;

    if attr_args.verbose {
        // 1. Extract argument names
        let fn_args = extract_arg_names(&input);

        // 2. Generate verbose log message
        let statements = generate_verbos_log(fn_name, fn_args);

        // 3. Prepend verbose log message to function block
        input.block.stmts.splice(0..0, statements);
    } else {
        input.block.stmts.insert(
            0,
            syn::parse_quote! {
                println!("INFO - calling {}", stringify!(#fn_name));
            },
        );
    }

    input.to_token_stream().into()
}

fn extract_arg_names(func: &syn::ItemFn) -> Vec<&syn::Ident> {
    func.sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(pat) = &(*pat_type.pat) {
                    return Some(&pat.ident);
                }
            };

            None
        })
        .collect()
}

fn generate_verbos_log(fn_name: &syn::Ident, fn_args: Vec<&syn::Ident>) -> Vec<syn::Stmt> {
    let mut statements = Vec::with_capacity(fn_args.len() + 1);

    statements.push(syn::parse_quote! {
        println!("INFO - calling {}", stringify!(#fn_name));
    });

    for arg in fn_args {
        statements.push(syn::parse_quote! {
            println!("  argument: {} = {:?}", stringify!(#arg), #arg);
        });
    }

    statements
}
