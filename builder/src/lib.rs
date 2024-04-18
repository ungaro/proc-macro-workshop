use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    //eprintln!("Debuging_here {:?}", ast);

    let name = &ast.ident;

    let builder_name = format!("{}Builder", name);

    //get the original name's span to use in the new Ident
    let builder_ident = Ident::new(&builder_name, name.span());

    let expanded = quote! {
        struct #builder_ident {
         executable: Option<String>,
         args: Option<Vec<String>>,
         env: Option<Vec<String>>,
         current_dir: Option<String>,
        }

        impl #name {
            pub fn builder() -> #builder_ident {
                #builder_ident{
                executable: None,
                 args: None,
                 env: None,
                 current_dir: None,
                }
            }
        }
    };
    expanded.into()

    //TokenStream::new()
}
