use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

pub fn gpu_type(input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    let DeriveInput {
        attrs: _,
        vis: _,
        ident,
        generics,
        data,
    } = {
        let input_item_clone = input_item.clone();
        parse_macro_input!(input_item_clone)
    };

    let input_item = TokenStream::from(input_item);

    let attrib_error = if !input_attrib.is_empty() {
        Error::new(Span::call_site(), "unexpected tokens").to_compile_error()
    } else {
        TokenStream::new()
    };

    let desc = match data {
        Data::Enum(_) => {
            return Error::new(Span::call_site(), "enums are not supported as gpu types")
                .to_compile_error()
                .into()
        }
        Data::Union(_) => {
            return Error::new(Span::call_site(), "unions are not supported as gpu types")
                .to_compile_error()
                .into()
        }
        Data::Struct(data) => {
            let field_idents = data.fields.iter().map(|field| field.ident.as_ref());
            let field_types = data.fields.iter().map(|field| &field.ty);

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
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[repr(C)]
        #input_item

        impl #impl_generics rsshader::GPUType for #ident #ty_generics #where_clause {
            const TYPE_DESC: rsshader::GPUTypeDesc<'static> = #desc;
        }

        #attrib_error
    }
    .into()
}
