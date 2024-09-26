use quote::quote;
use syn::{parse2, FnArg, ReturnType, Type};

use crate::fn_::PipelineStage;

use super::*;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    match item {
        GPUItem::Fn(item) => {
            if let Some(_) = item.pipeline_stage {
                errs.push(
                    quote_spanned! {
                        spec.span() =>
                        compile_error!("fn can't be a fragment_fn because it is already mapped to a pipeline stage");
                    }
                )
            }
            else {
                item.pipeline_stage = Some(PipelineStage::Fragment);

                item.input.block.stmts.insert(0, parse2(
                    if item.input.sig.inputs.len() == 1 {
                        let arg = item.input.sig.inputs.first().unwrap();
                        match arg {
                            FnArg::Typed(ty) => {
                                let ty = &ty.ty;
                                quote_spanned! {
                                    ty.span() =>
                                    <#ty as rsshader::constructs::Fragment>::validate();
                                }
                            },
                            FnArg::Receiver(_) => {
                                quote_spanned! {
                                    arg.span() =>
                                    compile_error!("a fragment fn can't recieve a: self, &self, &mut self");
                                }
                            }
                        }
                    }
                    else {
                        quote_spanned! {
                            item.input.sig.inputs.span() =>
                            compile_error!("a fragment fn has to have 1 argument");
                        }
                    }
                ).unwrap());

                item.input.block.stmts.insert(0,parse2( {
                    let ty: Type = match &item.input.sig.output {
                        ReturnType::Type(_, ty) => *(*ty).clone(),
                        ReturnType::Default => parse2(quote! { () }).unwrap(),
                    };
                    quote_spanned! {
                        ty.span() =>
                        fn validate_correct_output(x: #ty) -> rsshader::shader_core::Vec4 {
                            x
                        }
                    }
                }).unwrap());
            }
        }
        _ => errs.push(
            quote_spanned! {
                spec.span() =>
                compile_error!("this item type can't be used as a gpu(fragment_fn) item");
            }
        ),
    }
}