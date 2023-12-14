extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[proc_macro]
pub fn passthrough(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn passthrough_wrapped(input: TokenStream) -> TokenStream {
    let input_parsed = syn::parse_macro_input!(input as syn::Expr);
    let input_without_spans: syn::Expr = syn::parse_str(
        &input_parsed.to_token_stream().to_string()
    ).unwrap();
    quote! {
        (|| {
            if false {
                let _ = #input_without_spans;
            }

            #[allow(unreachable_code)]
            {
                #input_parsed
            }
        })()
    }.into()
}
