use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::gpu::gen_item_id;

pub fn gpu(input: ItemStruct) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = &input;

    let id = gen_item_id();

    let desc = {
        let field_idents = fields.iter().map(|field| field.ident.as_ref());
        let field_types = fields.iter().map(|field| &field.ty);
        let field_ids = (0..fields.len()).map(|_| gen_item_id());

        quote! {
            rsshader::GPUTypeDesc::Struct(rsshader::GPUStructDesc {
                id: rsshader::GPUItemID(#id),
                name: stringify!(#ident),
                fields: &[#(
                    rsshader::GPUFieldDesc {
                        id: rsshader::GPUItemID(#field_ids),
                        name: stringify!(#field_idents),
                        ty: &<#field_types as rsshader::GPUType>::TYPE_DESC,
                    },
                )*],
            })
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[repr(C)]
        #input

        impl #impl_generics rsshader::GPUType for #ident #ty_generics #where_clause {
            const TYPE_DESC: rsshader::GPUTypeDesc<'static> = #desc;
        }
    }
}
