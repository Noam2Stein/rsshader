use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, ItemStruct};

use crate::gpu::wgsl_ident;

pub fn gpu(input: ItemStruct, id: u128) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = &input;

    let generics_error = if generics.params.len() > 0 {
        Error::new(
            Span::call_site(),
            "generics in gpu items are not allowed yet",
        )
        .into_compile_error()
    } else {
        TokenStream::new()
    };

    let wgsl_ident = wgsl_ident(id, ident);

    let field_idents = fields.iter().map(|field| field.ident.as_ref());
    let field_types = fields.iter().map(|field| &field.ty).collect::<Box<[_]>>();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[repr(C)]
        #input

        unsafe impl #impl_generics rsshader::GPUType for #ident #ty_generics #where_clause {
            const GPU_TYPE_INFO: rsshader::GPUTypeInfo = rsshader::GPUTypeInfo {
                item_info: rsshader::GPUItemInfo {
                    id: #id,
                    dependencies: &[#(
                        &<#field_types as rsshader::GPUType>::GPU_TYPE_INFO.item_info
                    ), *],
                    wgsl_declaration: {
                        const STRS: &[&str] = &[
                            "struct ", #wgsl_ident, " { ", #(
                                "field_", stringify!(#field_idents), ": ",
                                <#field_types as rsshader::GPUType>::GPU_TYPE_INFO.wgsl_reference,
                                ", ",
                            )* " }",
                        ];
                        rsshader::__concat_strs_into__(&rsshader::__concat_strs_init__::<{ rsshader::__concat_strs_len__(STRS) }>(STRS))
                    },
                },
                wgsl_reference: #wgsl_ident,
            };
        }

        #generics_error
    }
}
