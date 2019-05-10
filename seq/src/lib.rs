extern crate proc_macro;

use proc_macro::TokenStream;

use proc_quote::quote;
use syn::{parse_macro_input, Token};

struct Seq {
    name: syn::Ident,
    start: syn::LitInt,
    end: syn::LitInt,
    body: syn::Expr,
}

impl syn::parse::Parse for Seq {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        let name: syn::Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start: syn::LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let end: syn::LitInt = input.parse()?;
        let body = input.parse::<syn::Expr>()?;

        Ok(Self {
            name,
            start,
            end,
            body,
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let seq: Seq = parse_macro_input!(input as Seq);

    let tokens = quote! {};

    tokens.into()
}
