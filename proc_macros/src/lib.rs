use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Item, Meta};

mod gpuitem;
use gpuitem::*;

mod gpuspec;
use gpuspec::*;

#[proc_macro_attribute]
pub fn gpu(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = parse_macro_input!(attr with Punctuated::<Meta, syn::Token![,]>::parse_terminated);

    let mut item = GPUItem::from(parse_macro_input!(item as Item));

    let mut errs = Vec::new();
    
    for spec in &attr {
        apply_gpuspec(spec, &mut item, &mut errs);
    }

    quote! {
        #item
        #(
            #errs
        )*
    }.into()
}