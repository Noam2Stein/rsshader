use quote::quote;
use syn::{parse2, Ident};

use crate::struct_::FragmentInfo;

use super::*;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    lerp::apply_gpuspec(spec, item, errs);
    match item {
        GPUItem::Struct(item) => {
            for field in &mut item.input.fields {
                field.attrs.retain_mut(|attr| {
                    if attr
                        .path()
                        .require_ident()
                        .map_or(false, |ident| ident.to_string() == "fragment_pos")
                    {
                        if item.fragment_info == None {
                            item.fragment_info = Some(FragmentInfo {
                                pos_field: field
                                    .ident
                                    .clone()
                                    .map(|ident| Ident::new(&ident.to_string(), field.ty.span())),
                            });

                            attr.meta = parse2(quote! { allow(dead_code) }).unwrap();

                            true
                        } else {
                            errs.push(quote_spanned! {
                                spec.span() =>
                                compile_error!("expected 1 field to be marked #[fragment_pos]");
                            });

                            false
                        }
                    } else {
                        true
                    }
                });
            }

            if item.fragment_info == None {
                errs.push(quote_spanned! {
                    spec.span() =>
                    compile_error!("expected 1 field to be marked #[fragment_pos]");
                });

                item.fragment_info = Some(FragmentInfo { pos_field: None });
            }
        }
        _ => errs.push(quote_spanned! {
            spec.span() =>
            compile_error!("this item type can't be used as a gpu(vertex) item");
        }),
    }
}
