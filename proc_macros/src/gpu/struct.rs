use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

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

    let desc = {
        let field_idents = fields.iter().map(|field| field.ident.as_ref());
        let field_types = fields.iter().map(|field| &field.ty);

        quote! {
            rsshader::GPUTypeDesc::Struct(rsshader::GPUStructDesc {
                ident: stringify!(#ident),
                fields: &[#(
                    rsshader::GPUFieldDesc {
                        ident: stringify!(#field_idents),
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
