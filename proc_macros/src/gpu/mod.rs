mod r#fn;
mod r#struct;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse, Error, Item};

pub fn gpu(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    let input_item = match parse::<Item>(input_item.clone()) {
        Ok(input_item) => input_item,
        Err(err) => {
            let input_item = TokenStream::from(input_item);
            let err = err.into_compile_error();

            return quote! {
                #input_item
                #err
            }
            .into();
        }
    };

    let attrib_error = if !input_attrib.is_empty() {
        Error::new(Span::call_site(), "unexpected tokens").to_compile_error()
    } else {
        TokenStream::new()
    };

    let item_output = match input_item {
        Item::Struct(input_item) => r#struct::gpu(input_item),
        Item::Fn(input_item) => r#fn::gpu(input_item),
        _ => quote! {
            #input_item

            compiler_error!("unsupported item type");
        },
    };

    quote! {
        #item_output

        #attrib_error
    }
    .into()
}
