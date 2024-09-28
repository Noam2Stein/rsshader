use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Ident, ItemStruct};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LerpInfo {
    pub flats: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentInfo {
    pub pos_field: Option<Ident>,
}

#[derive(Clone)]
pub struct GPUStruct {
    pub input: ItemStruct,
    pub lerp_info: Option<LerpInfo>,
    pub vertex_info: Option<()>,
    pub fragment_info: Option<FragmentInfo>,
}
impl From<ItemStruct> for GPUStruct {
    fn from(value: ItemStruct) -> Self {
        Self {
            input: value,
            lerp_info: None,
            vertex_info: None,
            fragment_info: None,
        }
    }
}
impl ToTokens for GPUStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.input.to_tokens(tokens);

        let ident = &self.input.ident;
        let (impl_generics, ty_generics, where_clause) = self.input.generics.split_for_impl();

        let field_idents = self
            .input
            .fields
            .iter()
            .map(|field| field.ident.clone())
            .collect::<Box<[Option<Ident>]>>();

        let fields = self.input.fields.iter().map(|field| {
            let ident = field.ident.clone().unwrap();
            let ty = &field.ty;
            quote! {
                rsshader::constructs::GPUField::new::<#ty>(stringify!(#ident))
            }
        });

        tokens.extend(quote! {
            unsafe impl #impl_generics rsshader::constructs::GPUType for #ident #ty_generics #where_clause {}

            unsafe impl #impl_generics rsshader::constructs::GPUStruct for #ident #ty_generics #where_clause {
                const FIELDS: &'static [rsshader::constructs::GPUField] = &[
                    #(
                        #fields,
                    )*
                ];
            }
        });

        if let Some(lerp_info) = &self.lerp_info {
            let field_values = self.input.fields.iter().map(|field| {
                if let Some(field_ident) = &field.ident {
                    if lerp_info.flats.contains(&field_ident) {
                        quote! {
                            self.#field_ident.clone()
                        }
                    } else {
                        let field_ty = &field.ty;
                        quote! {
                            <#field_ty as rsshader::constructs::GPULerp>::lerp(&self.#field_ident, &other.#field_ident, t)
                        }
                    }
                } else {
                    quote! {
                        
                    }
                }
            });
            tokens.extend(
                quote! {
                    unsafe impl #impl_generics rsshader::constructs::GPULerp for #ident #ty_generics #where_clause {
                        fn lerp(&self, other: &Self, t: f32) -> Self {
                            Self {
                                #(
                                    #field_idents: #field_values,
                                )*
                            }
                        }
                    }
                }
            );
        }

        if let Some(_) = &self.vertex_info {
            tokens.extend(
                quote! {
                    unsafe impl #impl_generics rsshader::constructs::GPUVertex for #ident #ty_generics #where_clause {

                    }
                }
            );
        }

        if let Some(fragment_info) = &self.fragment_info {
            let pos = fragment_info.pos_field.clone().map_or_else(
                || quote! { unreachable!() },
                |pos_field| quote_spanned! { pos_field.span() => self.#pos_field },
            );
            tokens.extend(
                quote_spanned! {
                    pos.span() =>
                    unsafe impl #impl_generics rsshader::constructs::GPUFragment for #ident #ty_generics #where_clause {
                        fn pos(&self) -> rsshader::shader_core::Vec4 {
                            #pos
                        }
                    }
                }
            );
        }
    }
}
