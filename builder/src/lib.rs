use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // you can debug the ast to see what it looks like by uncommenting the line below
    // need to have extra-traits feature on cargo.toml
    //eprintln!("Debuging_here {:#?}", ast);

    let name = &ast.ident;

    let builder_name = format!("{}Builder", name);

    // get the original name's span to use in the new Ident
    let builder_ident = Ident::new(&builder_name, name.span());

    // get fields from the struct
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!()
    };

    //eprintln!("Fields {:#?}", fields);

    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: std::option::Option<#ty> }
    });

    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_args = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is required"))?
        }
    });

    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: None }
    });

    let expanded = quote! {

    pub struct #builder_ident {
         #(#optionized,)*
    }

    impl #builder_ident{
        #(#methods)*

        pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
            Ok(#name{
                    #(#build_args,)*
                }
            )
        }
    }

    impl #name {
            fn builder() -> #builder_ident {
                #builder_ident {
                  #(#build_empty,)*
                }
            }
        }
    };

    expanded.into()
}

