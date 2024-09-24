use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

pub fn process(item: TokenStream) -> TokenStream {
    let item = match parse2::<DeriveInput>(item) {
        Ok(ok) => ok,
        Err(err) => return err.into_compile_error().into()
    };

    let ident = item.ident;
    quote! {
        impl rsshader::constructs::Type for #ident {

        }
    }
}