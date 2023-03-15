use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn succeeds_if(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn invariant(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Arbitrary)]
pub fn derive_arbitrary(item: TokenStream) -> TokenStream {
    item
}
