extern crate proc_macro;

mod builder;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let i = parse_macro_input!(input as DeriveInput);
    builder::build(&i).unwrap().into()
}
