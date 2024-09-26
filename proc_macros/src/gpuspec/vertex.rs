use super::*;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    match item {
        GPUItem::Struct(item) => {
            if let Some(_) = item.vertex_info {
                errs.push(quote_spanned! {
                    spec.span() =>
                    compile_error!("duplicate spec");
                });
            } else {
                item.vertex_info = Some(());
            }
        }
        _ => errs.push(quote_spanned! {
            spec.span() =>
            compile_error!("this item type can't be used as a gpu(vertex) item");
        }),
    }
}
