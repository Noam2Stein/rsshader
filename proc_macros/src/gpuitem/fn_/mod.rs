use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemFn, ReturnType, Stmt, Type};

mod local;

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
        value.block.stmts = value.block.stmts.into_iter().map(|stmt| {
            let validation_stmt = match &stmt {
                Stmt::Expr(_, _) => None,
                Stmt::Item(_) => None,
                Stmt::Local(input) => local::validation_stmt(input),
                Stmt::Macro(_) => None,
            };

            once(stmt).chain(validation_stmt.into_iter())
        }).flatten().collect();

        value.block.stmts.insert(
            0,
            parse2(
                value.sig.inputs.iter().map(|arg| {
                        let ty = match arg {
                            FnArg::Typed(ty) => &ty.ty,
                            FnArg::Receiver(receiver) => &*receiver.ty,
                        };
                        quote_spanned! {
                            ty.span() =>
                            <#ty as rsshader::constructs::GPUType>::validate();
                        }
                    }).collect::<TokenStream>(),
            ).unwrap(),
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

        let ident = &self.input.sig.ident;
        let ty_ident = gpufn(ident);

        tokens.append_all(
            quote! {
                #[doc(hidden)]
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
                        writeln!(f, "() {{")?;

                        writeln!(f, "}}")
                    }
                }
            }
        );

        if let Some(pipeline_stage) = &self.pipeline_stage {
            match pipeline_stage {
                PipelineFn::Vertex { vertex_ty, fragment_ty } => tokens.append_all(
                    quote! {
                        unsafe impl rsshader::constructs::VertexFn for #ty_ident {
                            type I = #vertex_ty;
                            type O = #fragment_ty;

                            fn invoke(input: #vertex_ty) -> #fragment_ty {
                                #ident(input)
                            }
                        }
                    }
                ),
                PipelineFn::Fragment { fragment_ty } => tokens.append_all(
                    quote_spanned! {
                        fragment_ty.span() =>
                        unsafe impl rsshader::constructs::FragmentFn for #ty_ident {
                            type I = #fragment_ty;

                            fn invoke(input: #fragment_ty) -> rsshader::shader_core::Vec4 {
                                #ident(input)
                            }
                        }
                    }
                ),
            }
        }
    }
}

pub fn gpufn(ident: &Ident) -> Ident {
    Ident::new(&format!("{}_GPUFn", ident.to_string()), ident.span())
}