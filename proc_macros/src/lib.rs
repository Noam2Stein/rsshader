use std::mem;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Item, Meta, Path};

mod gpuitem;
use gpuitem::*;

mod gpuspec;
use gpuspec::*;

#[proc_macro_attribute]
pub fn gpu(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
    }
    .into()
}

#[proc_macro]
pub fn gpufn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut path = parse_macro_input!(input as Path);
    if let Some(last) = path.segments.last_mut() {
        last.ident = gpuitem::fn_::gpufn(&last.ident)
    }

    path.to_token_stream().into()
}

#[doc = "replaces a span with a fallback if it is equal to the call site. may not always work."]
fn span_fallback(span: Span, fallback: Span) -> Span {
    if unsafe {
        mem::transmute::<Span, [u8; 4]>(span) == mem::transmute::<Span, [u8; 4]>(Span::call_site())
    } {
        fallback
    } else {
        span
    }
}
