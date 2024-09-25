use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{parse2, spanned::Spanned, Data, DeriveInput, Type};

pub fn process(item: TokenStream) -> TokenStream {
    let item = match parse2::<DeriveInput>(item) {
        Ok(ok) => ok,
        Err(err) => return err.into_compile_error().into()
    };
    let data = match item.data {
        Data::Struct(data) => data,
        Data::Enum(data) => return quote_spanned! { data.variants.span() => compile_error!("expected a struct") },
        Data::Union(data) => return quote_spanned! { data.fields.span() => compile_error!("expected a struct") },
    };

    let ident = item.ident;
    let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
    let field_idents = data.fields.iter().map(|field| &field.ident).collect::<Box<[&Option<Ident>]>>();
    let field_types = data.fields.iter().map(|field| &field.ty).collect::<Box<[&Type]>>();

    quote! {
        impl #impl_generics rsshader::constructs::GPUType for #ident #ty_generics #where_clause {
            fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut hasher = std::hash::DefaultHasher::new();
                <std::any::TypeId as std::hash::Hash>::hash(&std::any::TypeId::of::<Self>(), &mut hasher);
                write!(f, "Type___{}", <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher))
            }
            fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "struct ")?;
                Self::wgsl_ident(f)?;
                writeln!(f, " {{")?;
                #(
                    write!(f, "\t{}: ", stringify!(#field_idents))?;
                    <#field_types as rsshader::constructs::GPUType>::wgsl_ident(f)?;
                    writeln!(f, ",")?;
                )*

                write!(f, "}}")
            }
        }
        impl #impl_generics rsshader::constructs::GPUStruct for #ident #ty_generics #where_clause {

        }
    }
}