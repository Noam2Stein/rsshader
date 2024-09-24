use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Meta};

mod struct_;

#[proc_macro_attribute]
pub fn gpu(attrib: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item = TokenStream::from(item);
    let attrib = parse_macro_input!(attrib as Meta);
    
    let addition = match attrib.path().get_ident() {
        Some(item_ident) => match item_ident.to_string().as_str() {
            "Struct" => struct_::process(item.clone()),
            _ => {
                let message = format!("'{item_ident}' is not a gpu item type");
                quote_spanned! {
                    item_ident.span() =>
                    compile_error!(#message);
                }
            }
        },
        None => quote_spanned! {
            attrib.span() =>
            compile_error!("expected an ident");
        }
    };

    quote! {
        #item
        #addition
    }.into()
}