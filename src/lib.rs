use darling::{ast::NestedMeta, Error, FromMeta};
use parse::ParseContext;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

use crate::parser::ParserContext;

// mod attribute;
mod parse;
mod parser;

#[proc_macro_derive(Parse)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let context = ParseContext::new(input);
    println!("{:#?}", context);
    context.generate().into()
}

#[derive(Debug, FromMeta)]
struct Args {
    path: String,
}

#[proc_macro_attribute]
pub fn parser(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_attribute(attr);

    let input = parse_macro_input!(input as ItemStruct);
    // println!("{:#?}", input);
    let mut context = ParserContext::new(input, &args.path);
    
    context.generate().into()
}

fn parse_attribute(attr: TokenStream) -> Args {
    let args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => panic!("{}", Error::from(e).write_errors()),
    };

    match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => panic!("{}", e.write_errors()),
    }
}
