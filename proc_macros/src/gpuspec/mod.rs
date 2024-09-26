use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn::{spanned::Spanned, Meta};

mod vertex;
mod fragment;
mod vertex_fn;
mod fragment_fn;

use crate::GPUItem;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    match spec {
        Meta::Path(spec_path) => match spec_path.get_ident() {
            Some(spec_ident) => match spec_ident.to_string().as_str() {
                "vertex" => vertex::apply_gpuspec(spec, item, errs),
                "fragment" => fragment::apply_gpuspec(spec, item, errs),
                "vertex_fn" => vertex_fn::apply_gpuspec(spec, item, errs),
                "fragment_fn" => fragment_fn::apply_gpuspec(spec, item, errs),
                _ => {
                    let message = format!("'{spec_ident}' is not a gpu specification");
                    errs.push(
                        quote_spanned! {
                            spec_ident.span() =>
                            compile_error!(#message);
                        }
                    );
                }
            },
            None => errs.push(
                quote_spanned! {
                    spec_path.span() =>
                    compile_error!("expected an ident");
                }
            ),
        },
        Meta::List(spec) => match spec.path.get_ident() {
            Some(spec) => match spec.to_string().as_str() {
                _ => errs.push(
                    {
                        let message = format!("'{spec}' is not a gpu specification");
                        quote_spanned! {
                            spec.span() =>
                            compile_error!(#message);
                        }
                    }
                ),
            },
            None => errs.push(
                quote_spanned! {
                    spec.span() =>
                    compile_error!("expected an ident");
                }
            ),
        },
        Meta::NameValue(spec) => match spec.path.get_ident() {
            Some(spec) => match spec.to_string().as_str() {
                _ => errs.push(
                    {
                        let message = format!("'{spec}' is not a gpu specification");
                        quote_spanned! {
                            spec.span() =>
                            compile_error!(#message);
                        }
                    }
                ),
            },
            None => errs.push(
                quote_spanned! {
                    spec.span() =>
                    compile_error!("expected an ident");
                }
            ),
        }
    }
}