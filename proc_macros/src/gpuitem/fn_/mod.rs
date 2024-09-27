use std::iter::once;

use proc_macro2::Span;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use syn::{parse2, spanned::Spanned, FnArg, Ident, ItemFn, Pat, ReturnType, Stmt};

use crate::span_fallback;

mod local;

#[derive(Clone)]
pub enum PipelineFn {
    Vertex { spec_span: Span },
    Fragment { spec_span: Span },
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

        let vis = &self.input.vis;

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
            #vis struct #ty_ident {

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
                PipelineFn::Vertex { spec_span } => {
                    let input = match self.input.sig.inputs.first() {
                        Some(input) => match input {
                            FnArg::Typed(input) => &*input.ty,
                            FnArg::Receiver(input) => &*input.ty,
                        },
                        None => &parse2(
                            quote_spanned! { *spec_span => rsshader::constructs::FallbackVertex },
                        )
                        .unwrap(),
                    };
                    tokens.append_all(quote_spanned! {
                        span_fallback(output.span(), *spec_span) =>
                        unsafe impl rsshader::constructs::VertexFn for #ty_ident
                    });
                    tokens.append_all(quote_spanned! {
                        input.span() =>
                        {
                            type Input = #input;
                        }
                    });
                }
                PipelineFn::Fragment { spec_span } => {
                    let input = match self.input.sig.inputs.first() {
                        Some(input) => match input {
                            FnArg::Typed(input) => &*input.ty,
                            FnArg::Receiver(input) => &*input.ty,
                        },
                        None => &parse2(
                            quote_spanned! { *spec_span => rsshader::constructs::FallbackFragment },
                        )
                        .unwrap(),
                    };
                    tokens.append_all(quote_spanned! {
                        span_fallback(output.span(), *spec_span) =>
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
