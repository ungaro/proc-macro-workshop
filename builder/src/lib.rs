use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // you can debug the ast to see what it looks like by uncommenting the line below
    // need to have extra-traits feature on cargo.toml
    // eprintln!("Debuging_here {:?}", ast);

    let name = &ast.ident;

    let builder_name = format!("{}Builder", name);

    // get the original name's span to use in the new Ident
    let builder_ident = Ident::new(&builder_name, name.span());

    let expanded = quote! {
        pub struct #builder_ident {
             executable: Option<String>,
             args: Option<Vec<String>>,
             env: Option<Vec<String>>,
             current_dir: Option<String>,
        }


        impl #builder_ident{

            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
   
                Ok(
                    #name{
                        executable: self.executable.clone().ok_or("executable is required")?,
                        args: self.args.clone().ok_or("args is required")?,
                        env: self.env.clone().ok_or("env is required")?,
                        current_dir: self.current_dir.clone().ok_or("current_dir is required")?,
                    }
                )

            }


        }


            impl #name {
                fn builder() -> #builder_ident {
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

}


/*
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
//     }
*/