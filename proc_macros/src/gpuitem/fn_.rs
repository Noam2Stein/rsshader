use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemFn, Pat, ReturnType, Stmt, Type};

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

        let ident = &self.input.sig.ident;
        let ty_ident = gpufn(ident);

        let write_stmts = self.input.block.stmts.iter().map(|stmt|
            match stmt {
                Stmt::Local(stmt) => {
                    match &stmt.pat {
                        Pat::Type(pat) => {
                            if let Pat::Ident(ident) = &*pat.pat {
                                let ty = &pat.ty;
                                quote! {
                                    write!(f, "var {}: ", stringify!(#ident))?;
                                    <#ty as rsshader::constructs::GPUType>::wgsl_ident(f)?;
                                    writeln!(f, ";")?;
                                }
                            }
                            else {
                                quote_spanned! {
                                    pat.pat.span() =>
                                    compile_error!("expected an ident");
                                }
                            }
                        },
                        Pat::Ident(ident) => {
                            quote_spanned! {
                                ident.span() =>
                                compile_error!("expected the type of variable");
                            }
                        }
                        _ => {
                            quote_spanned! {
                                stmt.pat.span() =>
                                compile_error!("unsupported pat type");
                            }
                        }
                    }
                }
                _ => {
                    TokenStream::new()
                }
            }
        ).collect::<TokenStream>();

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
                        
                        #write_stmts

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