use std::mem::take;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{Error, Ident, Index, ItemStruct, Member, Type};

use crate::shader_item::util::Labels;

pub fn shader_item(item: ItemStruct, errors: &mut Vec<Error>, labels: &mut Labels) -> TokenStream {
    let mut syn_item = item;

    let mut item = {
        let (impl_generics, ty_params, where_clause) = syn_item.generics.split_for_impl();

        Struct {
            ident: syn_item.ident.clone(),

            ty_params: syn_item
                .generics
                .type_params()
                .map(|param| param.ident.clone())
                .collect(),

            impl_generics: impl_generics.into_token_stream(),
            impl_ty_params: ty_params.into_token_stream(),

            where_predicates: where_clause
                .map(|w| w.predicates.iter().map(|p| p.into_token_stream()))
                .into_iter()
                .flatten()
                .collect(),

            fields: syn_item
                .fields
                .iter_mut()
                .enumerate()
                .map(|(field_idx, field)| Field {
                    ident: match field.ident.clone() {
                        Some(ident) => Member::Named(ident),
                        None => Member::Unnamed(Index {
                            index: field_idx as u32,
                            span: Span::call_site(),
                        }),
                    },
                    ty: field.ty.clone(),
                    labels: Labels::from_attributes(&mut field.attrs, errors),
                })
                .collect(),
        }
    };

    let fragment_label = fragment_label(&mut item, errors, labels);

    let where_clause = {
        let original_predicates = &item.where_predicates;

        let new_predicates = item
            .ty_params
            .iter()
            .map(|param| {
                quote! { #param: rsshader::reflection::ShaderType }
            })
            .collect::<Vec<_>>();

        if !new_predicates.is_empty() || !original_predicates.is_empty() {
            quote! { where #(#new_predicates,)* #(#original_predicates,)* }
        } else {
            quote! {}
        }
    };

    let field_irs = item
        .fields
        .iter_mut()
        .map(
            |Field {
                 ident,
                 ty,
                 labels: field_labels,
             }| {
                let metadata = if labels.find("fragment").is_some()
                    && field_labels.find("position").is_some()
                {
                    quote! { Some(rsshader::ir::FieldMetadataIr::Position) }
                } else {
                    quote! { None }
                };

                quote! {
                    rsshader::ir::FieldIr {
                        ty: &<#ty as rsshader::reflection::ShaderType>::IR,
                        rust_offset: core::mem::offset_of!(Self, #ident),
                        metadata: #metadata,
                    }
                }
            },
        )
        .collect::<Vec<_>>();

    for field in take(&mut item.fields) {
        field.labels.finish(errors);
    }

    let Struct {
        ident,
        impl_generics,
        impl_ty_params,
        ..
    } = &item;

    quote! {
        #syn_item

        impl #impl_generics rsshader::reflection::ShaderType for #ident #impl_ty_params #where_clause {
            const IR: rsshader::ir::TypeIr = rsshader::ir::TypeIr::Struct(rsshader::ir::StructIr {
                fields: &[#(#field_irs),*],
            });
        }

        #fragment_label
    }
}

struct Struct {
    ident: Ident,
    ty_params: Vec<Ident>,
    impl_generics: TokenStream,
    impl_ty_params: TokenStream,
    where_predicates: Vec<TokenStream>,
    fields: Vec<Field>,
}

struct Field {
    ident: Member,
    ty: Type,
    labels: Labels,
}

fn fragment_label(item: &mut Struct, errors: &mut Vec<Error>, labels: &mut Labels) -> TokenStream {
    let Some(label_span) = labels.find("fragment") else {
        return quote! {};
    };

    let Struct {
        ident,
        impl_generics,
        impl_ty_params,
        ..
    } = item;

    let where_clause = {
        let original_predicates = &item.where_predicates;

        let new_predicates = item
            .ty_params
            .iter()
            .map(|param| {
                quote! { #param: rsshader::reflection::ShaderType }
            })
            .collect::<Vec<_>>();

        if !new_predicates.is_empty() || !original_predicates.is_empty() {
            quote! { where #(#new_predicates,)* #(#original_predicates,)* }
        } else {
            quote! {}
        }
    };

    let mut position_type = None;
    for field in &mut item.fields {
        if let Some(position_label) = field.labels.find("position") {
            if position_type.is_some() {
                errors.push(Error::new(
                    position_label,
                    "found multiple fields marked as #[position]",
                ));
            } else {
                position_type = Some(field.ty.clone());
            }
        }
    }

    let assert_position_type = if let Some(position_type) = position_type {
        quote! {
            const fn _assert_position<T: rsshader::reflection::VectorType<4, f32>>() {}

            _assert_position::<#position_type>();
        }
    } else {
        errors.push(Error::new(
            label_span,
            "expected a single field marked as #[position]",
        ));

        quote! {}
    };

    quote! {
        impl #impl_generics rsshader::reflection::FragmentType for #ident #impl_ty_params #where_clause {
            const __: () = {
                #assert_position_type
            };
        }
    }
}
