use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DerivedMacro)]
pub fn print_attributes_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_print_attributes(&ast)
}

fn impl_print_attributes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields.named.iter().map(|f| f.ident.clone().unwrap()),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let gen = quote! {

    impl DerivedMacro for #name {
            fn print_attributes(&self) {
                println!("Attributes of {}: ", stringify!(#name));
                #(
                    println!("{}: {:?}", stringify!(#fields), self.#fields);
                )*
            }
        }
    };

    gen.into()
}
#[proc_macro_derive(GetAttributesMacro)]
pub fn get_attributes_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_get_attributes(&ast)
}

fn impl_get_attributes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match ast.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => fields.named.iter().map(|f| f.ident.clone().unwrap()),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    let gen = quote! {

    impl GetAttributesMacro for #name {
            fn get_attributes(&self) -> Vec<&str> {
             let mut vec = Vec::new();

                #(
                    vec.push(stringify!(#fields));
                )*
            return vec;
            }
        }
    };

    gen.into()
}
