use syn::Ident;

use crate::struct_::LerpInfo;

use super::*;

pub fn apply_gpuspec(spec: &Meta, item: &mut GPUItem, errs: &mut Vec<TokenStream>) {
    match item {
        GPUItem::Struct(item) => {
            if let Some(_) = item.lerp_info {
                errs.push(quote_spanned! {
                    spec.span() =>
                    compile_error!("duplicate spec");
                });
            } else {
                let mut flats = Vec::new();
                for field in item.input.fields.iter_mut() {
                    if let Some(field_ident) = &field.ident {
                        field.attrs.retain(|attr| {
                            attr.path().require_ident().map_or(true, |attr_ident| {
                                if attr_ident.to_string() == "flat" {
                                    if flats.contains(field_ident) {
                                        errs.push(quote_spanned! {
                                            attr.span() =>
                                            compile_error!("duplicate flat");
                                        });
                                    } else {
                                        flats.push(Ident::new(
                                            &field_ident.to_string(),
                                            attr_ident.span(),
                                        ));
                                    }
                                    false
                                } else {
                                    true
                                }
                            })
                        })
                    }
                }

                item.lerp_info = Some(LerpInfo { flats })
            }
        }
        _ => errs.push(quote_spanned! {
            spec.span() =>
            compile_error!("this item type can't be used as a gpu(lerp) item");
        }),
    }
}
