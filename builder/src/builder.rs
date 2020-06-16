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
                fn #ident<'a>(&'a mut self, #ident: #arg) -> &'a mut Self {
                    self.#ident = #ident.clone();
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
            Some(quote! {#ident: #arg})
        } else {
            None
        }
    });
    let builder_name = Ident::new(&format!("{}Builder", &input.ident), Span::call_site());
    let build_fields = data.fields.iter().filter_map(|field| {
        if let Some(ident) = &field.ident {
            Some(quote! {#ident: self.#ident.clone()})
        } else {
            None
        }
    });
    let struct_name = &input.ident;
    Ok(quote! {
         #[derive(Default, Debug, Clone)]
         pub struct #builder_name {
             #(#fields),*
         }
        impl #builder_name {
            fn build(&mut self) -> Result<#struct_name, anyhow::Error>{
                Ok(#struct_name{
                    #(#build_fields),*
                })
            }
            #(#setters)*
        }
        impl #struct_name {
            fn builder() -> #builder_name {
                #builder_name::default()
            }
        }
    })
}
