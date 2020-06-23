use anyhow;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, GenericArgument, Path,
    PathArguments, Type, TypePath,
};

pub fn build(input: &DeriveInput) -> Result<TokenStream, anyhow::Error> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        _ => Ok(TokenStream::new()),
    }
}

macro_rules! extract {
    ($member:ident, $body:expr) => {
        fn $member<'a>(data: &'a syn::DataStruct) -> Vec<TokenStream> {
            data.fields
                .iter()
                .filter_map($body)
                .collect::<Vec<TokenStream>>()
        }
    };
}

macro_rules! handle_option {
    ($fn_name:ident, $stmt1: expr, $stmt2: expr) => {
        fn $fn_name<'a>(field: &'a syn::Field) -> Option<TokenStream> {
            if let (Some(ident), syn::Type::Path(t)) = (&field.ident, &field.ty) {
                let arg = &t.path.segments;
                if &arg[0].ident.to_string() == "Option" {
                    if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        args,
                        ..
                    }) = &arg[0].arguments
                    {
                        if let GenericArgument::Type(Type::Path(TypePath { path, .. })) = &args[0] {
                            $stmt1(path, ident)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    $stmt2(ident, arg)
                }
            } else {
                None
            }
        }
    };
}

extract!(extract_setter, |field| {
    handle_option!(
        handle_for_setters,
        |path: &Path, ident| {
            let a = &path.segments[0].ident;
            Some(quote! {
                fn #ident<'a>(&'a mut self, #ident: #a) -> &'a mut Self {
                    self.#ident = Some(#ident.clone());
                    self
                }
            })
        },
        |ident, arg| {
            Some(quote! {
                fn #ident<'a>(&'a mut self, #ident: #arg) -> &'a mut Self {
                    self.#ident = Some(#ident.clone());
                    self
                }
            })
        }
    );
    handle_for_setters(field)
});

extract!(extract_fields, |field| {
    handle_option!(
        handle_for_fields,
        |path: &Path, ident| {
            let a = &path.segments[0].ident;
            Some(quote! {#ident: Option<#a>})
        },
        |ident, arg| { Some(quote! {#ident: Option<#arg>}) }
    );
    handle_for_fields(field)
});

extract!(extract_builder_fields, |field| {
    handle_option!(
        handle_for_build,
        |_: &Path, ident| { Some(quote! {#ident: self.#ident.clone()}) },
        |ident, _| { Some(quote! {#ident: self.#ident.clone().unwrap()}) }
    );
    handle_for_build(field)
});
fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream, anyhow::Error> {
    let setters = extract_setter(&data);

    let fields = extract_fields(&data);

    let build_fields = extract_builder_fields(&data);

    let builder_name = Ident::new(&format!("{}Builder", &input.ident), Span::call_site());
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
