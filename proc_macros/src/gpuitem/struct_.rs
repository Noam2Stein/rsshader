use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Ident, ItemStruct};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentInfo {
    pub pos_field: Option<Ident>,
}

#[derive(Clone)]
pub struct GPUStruct {
    pub input: ItemStruct,
    pub vertex_info: Option<()>,
    pub fragment_info: Option<FragmentInfo>,
}
impl From<ItemStruct> for GPUStruct {
    fn from(value: ItemStruct) -> Self {
        Self {
            input: value,
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

        let fields = self.input.fields.iter().map(|field| {
            let ident = field.ident.clone().unwrap();
            let ty = &field.ty;
            quote! {
                rsshader::constructs::GPUFieldInfo::new::<#ty>(stringify!(#ident))
            }
        });

        tokens.extend(quote! {
            unsafe impl #impl_generics rsshader::constructs::GPUType for #ident #ty_generics #where_clause {}

            unsafe impl #impl_generics rsshader::constructs::GPUStruct for #ident #ty_generics #where_clause {
                const FIELDS: &'static [rsshader::constructs::GPUFieldInfo] = &[
                    #(
                        #fields,
                    )*
                ];
            }
        });

        if let Some(_) = &self.vertex_info {
            tokens.extend(
                quote! {
                    unsafe impl #impl_generics rsshader::constructs::Vertex for #ident #ty_generics #where_clause {

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
                    unsafe impl #impl_generics rsshader::constructs::Fragment for #ident #ty_generics #where_clause {
                        fn pos(&self) -> rsshader::shader_core::Vec4 {
                            #pos
                        }
                    }
                }
            );
        }
    }
}
