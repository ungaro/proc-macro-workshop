use proc_macro::TokenStream;
use syn::{parse_macro_input,  DeriveInput};


#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    //let name= &ast.ident;
    eprintln!("Debuging_here {:?}", ast);
    TokenStream::new()
}
