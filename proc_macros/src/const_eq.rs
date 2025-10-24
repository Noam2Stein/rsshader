use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, GenericArgument, Member, PathArguments, Type,
    TypeArray, TypePath, TypeReference, TypeSlice,
};

pub fn const_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let DeriveInput { ident, data, .. } = &input;

    let const_eq_body = match &data {
        Data::Struct(DataStruct { fields, .. }) => {
            let field_ifs = fields.iter().zip(fields.members()).map(|(field, member)| {
                let field_const_eq = field_const_eq(
                    &field.ty,
                    &quote! { self.#member },
                    &quote! { other.#member },
                );

                quote! {
                    if !(#field_const_eq) {
                        return false;
                    }
                }
            });

            quote! {
                #(#field_ifs)*

                true
            }
        }

        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms = variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let members = variant.fields.members().collect::<Vec<_>>();

                let self_members = variant.fields.members().map(|member| {
                    match member {
                        Member::Named(name) => format_ident!("self_{name}"),
                        Member::Unnamed(index) => format_ident!("self_{}", index.index),
                    }
                }).collect::<Vec<_>>();

                let other_members = variant.fields.members().map(|member| {
                    match member {
                        Member::Named(name) => format_ident!("other_{name}"),
                        Member::Unnamed(index) => format_ident!("other_{}", index.index),
                    }
                }).collect::<Vec<_>>();

                let field_ifs = variant.fields.iter().zip(self_members.iter().zip(other_members.iter())).map(|(field, (self_member, other_member))| {
                    let field_const_eq = field_const_eq(&field.ty, &quote! { *#self_member }, &quote! { *#other_member });

                    quote! {
                        if !(#field_const_eq) {
                            return false;
                        }
                    }
                });

                quote! {
                    (Self::#variant_name { #(#members: #self_members),* }, Self::#variant_name { #(#members: #other_members),* }) => {
                        #(#field_ifs)*

                        true
                    },
                }
            });

            quote! {
                match (self, other) {
                    #(#match_arms)*
                    _ => false,
                }
            }
        }

        Data::Union(_) => quote! { compile_error!("const_eq is not supported for unions") },
    };

    quote! {
        impl #ident {
            #[allow(unused_parens)]
            #[allow(unused_braces)]
            pub const fn eq(&self, other: &Self) -> bool {
                #const_eq_body
            }
        }
    }
    .into()
}

fn field_const_eq(
    field_ty: &Type,
    self_value: &TokenStream,
    other_value: &TokenStream,
) -> TokenStream {
    match field_ty {
        Type::Reference(TypeReference { elem, .. }) => {
            return field_const_eq(
                elem,
                &quote! { *(#self_value) },
                &quote! { *(#other_value) },
            );
        }

        Type::Path(TypePath { path, .. }) => {
            if path.is_ident("usize") {
                return quote! { (#self_value) == (#other_value) };
            }

            if path.segments.first().unwrap().ident == "Option" {
                let elem = if let PathArguments::AngleBracketed(args) =
                    &path.segments.first().unwrap().arguments
                {
                    if let GenericArgument::Type(ty) = args.args.first().unwrap() {
                        ty
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                };

                let elem_const_eq =
                    field_const_eq(elem, &quote! { *self_value }, &quote! { *other_value });

                return quote! {
                    match (&(#self_value), &(#other_value)) {
                        (Some(self_value), Some(other_value)) => {
                            #elem_const_eq
                        }
                        (None, None) => true,
                        _ => false,
                    }
                };
            }
        }

        Type::Array(TypeArray { elem, .. }) | Type::Slice(TypeSlice { elem, .. }) => {
            let elem_const_eq = field_const_eq(
                elem,
                &quote! { (#self_value)[i] },
                &quote! { (#other_value)[i] },
            );

            return quote! {{
                if (#self_value).len() != (#other_value).len() {
                    return false;
                }

                let mut i = 0;
                while i < (#self_value).len() {
                    if !#elem_const_eq {
                        return false;
                    }

                    i += 1;
                }

                true
            }};
        }

        _ => {}
    };

    quote! { (#self_value).eq(&(#other_value)) }
}
