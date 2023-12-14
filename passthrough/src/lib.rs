use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn passthrough(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn passthrough_wrapped(input: TokenStream) -> TokenStream {
    let input_string: String = input.to_string();
    let input_2 = proc_macro2::TokenStream::from(input);
    quote! {
        {
            let _ = #input_string;

            {
                #input_2
            }
        }
    }.into()
}
