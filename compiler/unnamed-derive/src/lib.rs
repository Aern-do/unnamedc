use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

mod spanned;

#[proc_macro_derive(Spanned, attributes(span))]
pub fn spanned(token_stream: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(token_stream as DeriveInput);
    spanned::expand(derive_input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
