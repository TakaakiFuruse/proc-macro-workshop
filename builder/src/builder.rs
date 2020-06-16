use anyhow;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput};

pub fn build(input: &DeriveInput) -> Result<TokenStream, anyhow::Error> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Ok(TokenStream::new()),
    }
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream, anyhow::Error> {
    let setters = data.fields.iter().filter_map(|field| {
        if let (Some(ident), syn::Type::Path(t)) = (&field.ident, &field.ty) {
            let arg = &t.path.segments;

            Some(quote! {
                fn #ident(&mut self, #ident: #arg) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            })
        } else {
            None
        }
    });
    let fields = data.fields.iter().filter_map(|field| {
        if let (Some(ident), syn::Type::Path(t)) = (&field.ident, &field.ty) {
            let arg = &t.path.segments;
            Some(quote! {#ident: Option<#arg>})
        } else {
            None
        }
    });
    let struct_name = &input.ident;
    let builder_name = Ident::new(&format!("{}Builder", &input.ident), Span::call_site());
    Ok(quote! {
         #[derive(Default, Debug)]
         pub struct #builder_name {
             #(#fields),*
         }
        impl #builder_name {
            #(#setters)*
        }
        impl #struct_name {
            fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
    })
}
