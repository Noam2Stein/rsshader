use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemFn, ReturnType, Type};

#[derive(Clone)]
pub enum PipelineFn {
    Vertex { vertex_ty: Box<Type>, fragment_ty: Box<Type> },
    Fragment { fragment_ty: Box<Type> },
}

#[derive(Clone)]
pub struct GPUFn {
    pub input: ItemFn,
    pub pipeline_stage: Option<PipelineFn>,
}
impl From<ItemFn> for GPUFn {
    fn from(mut value: ItemFn) -> Self {
        value.block.stmts.insert(
            0,
            parse2(
                value
                    .sig
                    .inputs
                    .iter()
                    .map(|arg| match arg {
                        FnArg::Typed(ty) => {
                            let ty = &ty.ty;
                            quote_spanned! {
                                ty.span() =>
                                <#ty as rsshader::constructs::GPUType>::validate();
                            }
                        }
                        FnArg::Receiver(receiver) => {
                            let ty = &*receiver.ty;
                            quote_spanned! {
                                ty.span() =>
                                <#ty as rsshader::constructs::GPUType>::validate();
                            }
                        }
                    })
                    .collect::<TokenStream>(),
            )
            .unwrap(),
        );

        value.block.stmts.insert(
            0,
            parse2({
                let ty: Type = match &value.sig.output {
                    ReturnType::Type(_, ty) => *(*ty).clone(),
                    ReturnType::Default => parse2(quote! { () }).unwrap(),
                };
                quote_spanned! {
                    ty.span() =>
                    <#ty as rsshader::constructs::GPUType>::validate();
                }
            })
            .unwrap(),
        );

        Self {
            input: value,
            pipeline_stage: None,
        }
    }
}
impl ToTokens for GPUFn {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append_all(
            quote! {
                #[allow(unused)]
            }
        );

        self.input.to_tokens(tokens);

        let ty_ident = Ident::new(
            &format!("{}_as_gpu_fn", self.input.sig.ident),
            self.input.sig.ident.span(),
        );

        tokens.append_all(
            quote! {
                #[allow(non_camel_case_types)]
                pub struct #ty_ident {
                    
                }
                unsafe impl rsshader::constructs::GPUFn for #ty_ident {
                    fn wgsl_ident(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        let mut hasher = std::hash::DefaultHasher::new();
                        <std::any::TypeId as std::hash::Hash>::hash(&std::any::TypeId::of::<Self>(), &mut hasher);
                        write!(f, "fn___{}", <std::hash::DefaultHasher as std::hash::Hasher>::finish(&hasher))
                    }
                    fn wgsl_declaration(f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(f, "fn ")?;
                        Self::wgsl_ident(f)?;
                        writeln!(f, "() {{ }}")
                    }
                }
            }
        );

        if let Some(pipeline_stage) = &self.pipeline_stage {
            match pipeline_stage {
                PipelineFn::Vertex { vertex_ty, fragment_ty } => tokens.append_all(
                    quote! {
                        unsafe impl rsshader::constructs::VertexFn<#vertex_ty, #fragment_ty> for #ty_ident {

                        }
                    }
                ),
                PipelineFn::Fragment { fragment_ty } => tokens.append_all(
                    quote! {
                        unsafe impl rsshader::constructs::FragmentFn<#fragment_ty> for #ty_ident {

                        }
                    }
                ),
            }
        }
    }
}
