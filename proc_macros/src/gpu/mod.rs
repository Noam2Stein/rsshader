mod blend;
mod r#fn;
mod r#struct;

use std::time::{SystemTime, UNIX_EPOCH};

use bitflags::bitflags;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use syn::{parse, parse_macro_input, Attribute, Error, Item};

pub fn gpu(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    let input_attribs = parse_macro_input!(input_attrib with Attribute::parse_inner);
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

    let (_attribs, attrib_errors) = {
        let mut attribs = GPUAttribs::empty();
        let mut attrib_errs = Vec::new();

        for input_attrib in input_attribs {
            match input_attrib.path().require_ident() {
                Ok(attrib_ident) => match attrib_ident.to_string().as_str() {
                    "blend" => attribs |= GPUAttribs::BLEND,
                    _ => attrib_errs.push(
                        Error::new(attrib_ident.span(), "unknown gpu attribute").to_compile_error(),
                    ),
                },
                Err(err) => attrib_errs.push(err.into_compile_error()),
            }
        }

        (attribs, attrib_errs)
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

        #(#attrib_errors)*
    }
    .into()
}

bitflags! {
    struct GPUAttribs: u8 {
        const BLEND = 1;
    }
}

fn gen_item_id() -> u128 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    let random_part: u64 = rand::thread_rng().gen();

    ((timestamp as u128) << 64) | random_part as u128
}
