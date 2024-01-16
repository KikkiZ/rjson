use parser::ParserContext;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod parser;

#[proc_macro_derive(Parser)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", input);
    let context = ParserContext::new(input);
    context.generate().into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
