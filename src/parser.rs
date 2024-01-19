use std::{fs::File, io::Read};

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde_json::Value;
use syn::{parse_str, ItemStruct};

#[derive(Debug)]
pub struct ParserContext {
    token: ItemStruct,
    file: File,
}

impl ParserContext {
    pub fn new(input: ItemStruct, path: &String) -> Self {
        let file = File::open(path).unwrap();

        Self {
            token: input,
            file: file,
        }
    }

    pub fn generate(&mut self) -> TokenStream {
        let visiable = &self.token.vis;

        let name = &self.token.ident;
        let struct_name = Ident::new(&format!("{}", name), name.span());
        let fields = gen_fields(&mut self.file);

        println!("{:#?}", fields);

        quote!(
            #[derive(Debug)]
            #visiable struct #struct_name {
                #(pub #fields),*
            }
        )
    }
}

fn gen_fields(file: &mut File) -> Vec<TokenStream> {
    let mut buffer = [0; 1024 * 64];
    let len = file.read(&mut buffer).unwrap();
    let json = std::str::from_utf8(&buffer[0..len]).unwrap();

    let data: Value = serde_json::from_str(json).unwrap();
    let obj = data.as_object().unwrap();

    let mut fields: Vec<TokenStream> = Vec::new();
    for key in obj.keys() {
        let ident_key: Ident = parse_str(&key).unwrap();
        let ident_type = type_match(&obj.get(key).unwrap());
        fields.push(quote!(#ident_key: #ident_type));
    }

    fields
}

fn type_match(v: &Value) -> TokenStream {
    let ident;
    if v.is_boolean() {
        ident = quote!(bool);
    } else if v.is_number() {
        ident = quote!(usize);
    } else if v.is_string() {
        ident = quote!(String);
    } else if v.is_array() {
        let arr = v.as_array().unwrap();
        if arr.len() > 0 {
            // ident = parse_str(&format!("Vec<{}>", type_match(&arr[0]).to_string())).unwrap();
            // ident = type_match(&arr[0]);
            // println!("Vec<{}>", ident.to_string());
            let ty = type_match(&arr[0]);
            ident = quote!(Vec<#ty>);
        } else {
            panic!()
        }
    } else {
        panic!()
    }

    ident
    // match v {
    //     Bool => parse_str("bool").unwrap(),
    //     Number => parse_str("usize").unwrap(),
    //     String => parse_str("String").unwrap(),
    //     Array => {
    //         let arr = v.as_array().unwrap();
    //         if arr.len() > 0 {
    //             
    //         } else {
    //             panic!()
    //         }
    //     },
    //     _ => panic!()
    // }
    // todo!()
}
