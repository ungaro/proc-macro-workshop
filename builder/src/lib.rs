use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

fn ty_inner_type(ty: &syn::Type) -> Option<&syn::Type> {
    //explain what we are doing here
    if let syn::Type::Path(ref p) = ty {
        if p.path.segments.len() != 1 || p.path.segments[0].ident != "Option" {
            return None;
        }

        //explain what we are doing here

        if let syn::PathArguments::AngleBracketed(ref inner_ty) =
            p.path.segments[0].arguments
        {
            if inner_ty.args.len() != 1 {
                return None;
            }



            let inner_ty = inner_ty.args.first().unwrap();
            if let syn::GenericArgument::Type(ref t) = inner_ty {
                return Some(t);
            }

           
        }
    }
    None
}

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

    // make the fields optionized
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if ty_inner_type(ty).is_some() {
            return quote! { #name: #ty };
        } else {
            return quote! { #name: std::option::Option<#ty> };
        }

    });

    // create the builder methods from AST
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

    // create the builder args from AST
    let build_args = fields.iter().map(|f| {
        let name = &f.ident;
        //let t = &f.ty;

        if let syn::Type::Path(ref p) = &f.ty {
            if let Some(segment) = p.path.segments.first() {
                if segment.ident == "Option" {
                    return quote! {
                        #name: self.#name.clone()
                    };
                }
            }
        }

        quote! {
            #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is required"))?
        }
    });

    // create the builder empty fields from AST for instatiating the builder
    let build_empty = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name: None }
    });

    //create the expanded code
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
