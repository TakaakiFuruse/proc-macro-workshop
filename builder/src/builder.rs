use anyhow;
use proc_macro2::TokenStream;
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
        if let Some(ident) = &field.ident {
            if let syn::Type::Path(t) = &field.ty {
                let arg = &t.path.segments;

                Some(quote! {
                    fn #ident(&self, arg: #arg) {
                    }
                })
            } else {
                Some(quote! {
                    fn #ident(&self) {
                    }
                })
            }
        } else {
            None
        }
    });
    let struct_name = &input.ident;
    Ok(quote! {
        impl #struct_name {
            fn builder() -> #struct_name {
                return #struct_name {
                };
            }
            #(#setters)*,
        }
        impl Default for #struct_name {
            fn default() -> #struct_name{
                #struct_name { }
            }
        }
    })
}
