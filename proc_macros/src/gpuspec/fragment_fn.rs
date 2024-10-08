use crate::fn_::PipelineFn;

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
            } else {
                item.pipeline_stage = Some(PipelineFn::Fragment {
                    spec_span: spec.span(),
                });

                if item.input.sig.inputs.len() != 1 {
                    errs.push(quote_spanned! {
                        item.input.sig.paren_token.span.span() =>
                        compile_error!("a fragment fn has to have 1 arg");
                    });
                }
            }
        }
        _ => errs.push(quote_spanned! {
            spec.span() =>
            compile_error!("this item type can't be used as a gpu(fragment_fn) item");
        }),
    }
}
