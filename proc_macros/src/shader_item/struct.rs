use proc_macro2::TokenStream;
use quote::quote;
use syn::{ImplGenerics, ItemStruct, Type, TypeGenerics, TypeParam};

use crate::shader_item::labels::Labels;

pub fn shader_item(item: ItemStruct, labels: &mut Labels) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = &item;

    let (impl_generics, ty_params, where_clause) = generics.split_for_impl();

    let where_clause = {
        let new_predicates = generics
            .type_params()
            .map(|param| {
                let param_name = &param.ident;
                quote! { #param_name: rsshader::reflection::ShaderType }
            })
            .collect::<Vec<_>>();

        if !new_predicates.is_empty() || where_clause.is_some() {
            let where_predicates = match where_clause {
                Some(where_clause) => where_clause
                    .predicates
                    .iter()
                    .map(|predicate| {
                        quote! { #predicate }
                    })
                    .collect::<Vec<_>>(),
                None => Vec::new(),
            };

            quote! { where #(#new_predicates,)* #(#where_predicates,)* }
        } else {
            quote! {}
        }
    };

    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

    let field_members = fields.members().collect::<Vec<_>>();

    let vertex_output = handle_vertex(
        &item,
        labels,
        &impl_generics,
        &ty_params,
        &where_clause,
        &field_types,
    );
    let fragment_output = handle_fragment(
        &item,
        labels,
        &impl_generics,
        &ty_params,
        &where_clause,
        &field_types,
    );

    quote! {
        #item

        impl #impl_generics rsshader::reflection::ShaderType for #ident #ty_params #where_clause {
            const IR: rsshader::ir::Type = rsshader::ir::Type::Struct(rsshader::ir::Struct {
                fields: &[#(
                    rsshader::ir::Field {
                        ty: &<#field_types as rsshader::reflection::ShaderType>::IR,
                        id: core::mem::offset_of!(Self, #field_members),
                    },
                )*],
            });
        }

        #vertex_output
        #fragment_output
    }
}

fn handle_vertex(
    item: &ItemStruct,
    labels: &mut Labels,
    impl_generics: &ImplGenerics,
    ty_params: &TypeGenerics,
    where_clause: &TokenStream,
    field_types: &[&Type],
) -> TokenStream {
    if labels.find("vertex").is_none() {
        return quote! {};
    }

    let ident = &item.ident;

    let where_clause = if where_clause.is_empty() {
        quote! {}
    } else {
        let new_predicates = item.generics.type_params().map(|TypeParam { ident, .. }| {
            quote! { #ident: rsshader::reflection::VertexType }
        });

        quote! { #where_clause #(#new_predicates,)* }
    };

    quote! {
        impl #impl_generics rsshader::reflection::VertexType for #ident #ty_params #where_clause {
            const _ASSERT: () = {#(
                <#field_types as rsshader::reflection::VertexType>::_ASSERT;
            )*};
        }
    }
}

fn handle_fragment(
    item: &ItemStruct,
    labels: &mut Labels,
    impl_generics: &ImplGenerics,
    ty_params: &TypeGenerics,
    where_clause: &TokenStream,
    field_types: &[&Type],
) -> TokenStream {
    if labels.find("fragment").is_none() {
        return quote! {};
    }

    let ident = &item.ident;

    let where_clause = if where_clause.is_empty() {
        quote! {}
    } else {
        let new_predicates = item.generics.type_params().map(|TypeParam { ident, .. }| {
            quote! { #ident: rsshader::reflection::FragmentType }
        });

        quote! { #where_clause #(#new_predicates,)* }
    };

    quote! {
        impl #impl_generics rsshader::reflection::FragmentType for #ident #ty_params #where_clause {
            const _ASSERT: () = {#(
                <#field_types as rsshader::reflection::FragmentType>::_ASSERT;
            )*};
        }
    }
}
