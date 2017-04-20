
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

// `quote` relies on macro recursion, so it is likely to hit the normal cap.
#![recursion_limit = "192"]

use proc_macro::TokenStream;

mod component;

#[proc_macro_derive(ComponentGroup), attributes(group)]
pub fn component_group(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    
}
