use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn shader_item(item: ItemStruct) -> TokenStream {
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

    let shader_type_where_predicates = generics
        .type_params()
        .map(|param| {
            let param_name = &param.ident;
            quote! { #param_name: rsshader::reflection::ShaderType }
        })
        .collect::<Vec<_>>();

    let where_clause = if !shader_type_where_predicates.is_empty() || where_clause.is_some() {
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

        quote! { where #(#shader_type_where_predicates,)* #(#where_predicates),* }
    } else {
        quote! {}
    };

    let field_types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

    let field_members = fields.members().collect::<Vec<_>>();

    quote! {
        #item

        impl #impl_generics rsshader::reflection::ShaderType for #ident #ty_params #where_clause {
            const IR: rsshader::ir::Type = rsshader::ir::Type::Struct(rsshader::ir::Struct {
                fields: &[#(
                    rsshader::ir::Field {
                        ty: &<#field_types as rsshader::reflection::ShaderType>::IR,
                        id: core::mem::offset_of!(Self, #field_members),
                        kind: rsshader::ir::FieldKind::Normal,
                    },
                )*],
            });
        }
    }
}
