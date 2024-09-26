use crate::struct_::FragmentInfo;

use super::*;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    match item {
        GPUItem::Struct(item) => {
            let mut field_index = 0;
            for field in &mut item.input.fields {
                field.attrs.retain(|attr| {
                    if attr
                        .path()
                        .require_ident()
                        .map_or(false, |ident| ident.to_string() == "fragment_pos")
                    {
                        if item.fragment_info == None {
                            item.fragment_info = Some(FragmentInfo {
                                pos_field_index: field_index,
                            });
                        } else {
                            errs.push(quote_spanned! {
                                spec.span() =>
                                compile_error!("expected 1 field to be marked #[fragment_pos]");
                            });
                        }

                        false
                    } else {
                        true
                    }
                });

                field_index += 1;
            }

            if item.fragment_info == None {
                errs.push(quote_spanned! {
                    spec.span() =>
                    compile_error!("expected 1 field to be marked #[fragment_pos]");
                });

                item.fragment_info = Some(FragmentInfo { pos_field_index: 0 });
            }
        }
        _ => errs.push(quote_spanned! {
            spec.span() =>
            compile_error!("this item type can't be used as a gpu(vertex) item");
        }),
    }
}
