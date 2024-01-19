#![allow(dead_code)]
use std::iter::Map;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    punctuated::{Iter, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
};

type TokenStreamIter<'a> = Map<Iter<'a, Field>, fn(&'a Field) -> TokenStream>;

#[derive(Debug)]
pub struct ParseContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}

impl ParseContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;

        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("unsupported data type")
        };

        Self {
            name: name,
            fields: fields,
        }
    }

    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let parser_name = Ident::new(&format!("{}Parser", name), name.span());
        let optionized_fileds = self.gen_optionnized_fields();
        let methods = self.gen_methods();
        let assigns = self.gen_assigns();

        quote!(
            #[derive(Debug, Default)]
            struct #parser_name {
                #(#optionized_fileds,)*
            }

            impl #parser_name {
                #(#methods)*

                pub fn finish(mut self) -> Result<#name, &'static str> {
                    Ok(#name {
                        #(#assigns,)*
                    })
                }
            }

            impl #name {
                fn parser() -> #parser_name {
                    Default::default()
                }
            }
        )
    }

    fn gen_optionnized_fields(&self) -> TokenStreamIter {
        self.fields.iter().map(|field| {
            let ty = &field.ty;
            let name = &field.ident;
            quote!(#name: std::option::Option<#ty>)
        })
    }

    fn gen_methods(&self) -> TokenStreamIter {
        self.fields.iter().map(|field| {
            let ty = &field.ty;
            let name = &field.ident;
            quote!(
                pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                    self.#name = Some(v.into());
                    self
                }
            )
        })
    }

    fn gen_assigns(&self) -> TokenStreamIter {
        self.fields.iter().map(|field| {
            // let ty = &field.ty;
            let name = &field.ident;
            quote!(
                #name: self.#name.take().ok_or(concat!(stringify!(#name), " need to be set"))?
            )
        })
    }
}
