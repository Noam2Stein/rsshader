use std::iter::once;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemFn, Pat, ReturnType, Stmt, Type};

mod local;

#[derive(Clone)]
pub enum PipelineFn {
    Vertex,
    Fragment,
}

#[derive(Clone)]
pub struct GPUFn {
    pub input: ItemFn,
    pub pipeline_stage: Option<PipelineFn>,
}
impl From<ItemFn> for GPUFn {
    fn from(mut value: ItemFn) -> Self {
        value.block.stmts = value
            .block
            .stmts
            .into_iter()
            .map(|stmt| {
                let validation_stmt = match &stmt {
                    Stmt::Expr(_, _) => None,
                    Stmt::Item(_) => None,
                    Stmt::Local(input) => local::validation_stmt(input),
                    Stmt::Macro(_) => None,
                };

                once(stmt).chain(validation_stmt.into_iter())
            })
            .flatten()
            .collect();

        value.block.stmts.insert(
            0,
            parse2(
                value
                    .sig
                    .inputs
                    .iter()
                    .map(|arg| {
                        let ty = match arg {
                            FnArg::Typed(ty) => &ty.ty,
                            FnArg::Receiver(receiver) => &*receiver.ty,
                        };
                        quote_spanned! {
                            ty.span() =>
                            <#ty as rsshader::constructs::GPUType>::validate();
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
        tokens.append_all(quote! {
            #[allow(unused)]
        });

        self.input.to_tokens(tokens);

        let ident = &self.input.sig.ident;
        let ty_ident = gpufn(ident);

        let inputs = self.input.sig.inputs.iter().map(|arg| {
            let ident = match arg {
                FnArg::Typed(arg) => match &*arg.pat {
                    Pat::Ident(pat) => {
                        let ident = &pat.ident;
                        quote! { stringify!(#ident) }
                    }
                    pat => quote_spanned! {
                        pat.span() =>
                        compile_error!("expected an ident")
                    },
                },
                FnArg::Receiver(_) => quote! { "self" },
            };
            let ty = match arg {
                FnArg::Typed(arg) => &*arg.ty,
                FnArg::Receiver(arg) => &*arg.ty,
            };
            quote_spanned! {
                ty.span() =>
                rsshader::constructs::GPUArgument::new::<#ty>(stringify!(#ident))
            }
        });

        let output = match &self.input.sig.output {
            ReturnType::Default => quote! { () },
            ReturnType::Type(_, ty) => ty.to_token_stream(),
        };

        tokens.append_all(quote! {
            #[allow(non_camel_case_types)]
            pub struct #ty_ident {

            }
            unsafe impl rsshader::constructs::GPUFn for #ty_ident {
                const INPUTS: &'static [rsshader::constructs::GPUArgument] = &[
                    #(
                        #inputs,
                    )*
                ];

                type Output = #output;
            }
        });

        if let Some(pipeline_stage) = &self.pipeline_stage {
            match pipeline_stage {
                PipelineFn::Vertex => {
                    let input = match self.input.sig.inputs.first() {
                        Some(input) => match input {
                            FnArg::Typed(input) => &*input.ty,
                            FnArg::Receiver(input) => &*input.ty,
                        },
                        None => &parse2(quote! { () }).unwrap(),
                    };
                    tokens.append_all(quote_spanned! {
                        output.span() =>
                        unsafe impl rsshader::constructs::VertexFn for #ty_ident
                    });
                    tokens.append_all(quote_spanned! {
                        input.span() =>
                        {
                            type Input = #input;
                        }
                    });
                }
                PipelineFn::Fragment => {
                    let input = match self.input.sig.inputs.first() {
                        Some(input) => match input {
                            FnArg::Typed(input) => &*input.ty,
                            FnArg::Receiver(input) => &*input.ty,
                        },
                        None => &parse2(quote! { () }).unwrap(),
                    };
                    tokens.append_all(quote_spanned! {
                        output.span() =>
                        unsafe impl rsshader::constructs::FragmentFn for #ty_ident
                    });
                    tokens.append_all(quote_spanned! {
                        input.span() =>
                        {
                            type Input = #input;
                        }
                    });
                }
            }
        }
    }
}

pub fn gpufn(ident: &Ident) -> Ident {
    Ident::new(&format!("{}_GPUFn", ident.to_string()), ident.span())
}