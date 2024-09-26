use quote::{quote, quote_spanned, ToTokens};
use syn::{spanned::Spanned, Ident, ItemStruct, Type};

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

        let field_idents = self
            .input
            .fields
            .iter()
            .map(|field| &field.ident)
            .collect::<Box<[&Option<Ident>]>>();
        let field_types = self
            .input
            .fields
            .iter()
            .map(|field| &field.ty)
            .collect::<Box<[&Type]>>();
        let field_fragment_pos_attrs = self.input.fields.iter().map(|field| {
            if let Some(fragment_info) = &self.fragment_info {
                if fragment_info.pos_field == field.ident {
                    Some(quote! { @builtin(position) })
                } else {
                    None
                }
            }
            else {
                None
            }
        });

        tokens.extend(quote! {
            unsafe impl #impl_generics rsshader::constructs::GPUType for #ident #ty_generics #where_clause {
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
                        write!(f, "\t{}", stringify!(#field_fragment_pos_attrs #field_idents: ))?;
                        <#field_types as rsshader::constructs::GPUType>::wgsl_ident(f)?;
                        writeln!(f, ",")?;
                    )*
    
                    write!(f, "}}")
                }
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
                |pos_field| quote_spanned! { pos_field.span() => self.#pos_field }
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
