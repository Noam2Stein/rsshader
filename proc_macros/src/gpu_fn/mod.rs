use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse, spanned::Spanned, Error, Expr, FnArg, Ident, ItemFn, Lit, Pat, ReturnType, Stmt};

use crate::{get_expr_desc_item_ident, get_fn_desc_item_ident};

pub fn gpu_fn(_input_attrib: TokenStream1, input_item: TokenStream1) -> TokenStream1 {
    let ItemFn {
        attrs: _,
        vis,
        sig,
        block,
    } = match parse(input_item.clone()) {
        Ok(ok) => ok,
        Err(err) => {
            let input_item = TokenStream::from(input_item);
            let err = err.to_compile_error();

            return quote! {
                #input_item
                #err
            }
            .into();
        }
    };

    let fn_ident = &sig.ident;

    let fn_desc_item_ident = get_fn_desc_item_ident(&sig.ident);
    let expr_desc_item_ident = get_expr_desc_item_ident(&sig.ident);

    let input_idents = sig.inputs.iter().filter_map(|input| match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(input) => match &*input.pat {
            Pat::Ident(input) => Some(&input.ident),
            _ => None,
        },
    }).collect::<Box<[&Ident]>>();
    let input_types = sig.inputs.iter().map(|input| match input {
        FnArg::Receiver(_) => quote_spanned! { input.span() => <compile_error!("receivers are not supported in gpu fns")> },
        FnArg::Typed(input) => match &*input.pat {
            Pat::Ident(_) => input.ty.to_token_stream(),
            _ => quote_spanned! { input.span() => <compile_error!("only ident inputs are supported in gpu fns")> },
        },
    });

    let output = match &sig.output {
        ReturnType::Default => quote_spanned! { sig.span() => None },
        ReturnType::Type(_, output) => {
            quote_spanned! { output.span() => Some(&<#output as rsshader::GPUType>::TYPE_DESC) }
        }
    };

    let input_expr_item_idents = input_idents.iter().map(|input_ident| get_expr_desc_item_ident(input_ident));
    let stmts = stmts_desc(block.stmts.iter());

    quote! {
        #vis #sig #block

        #[allow(non_upper_case_globals)]
        #vis const #fn_desc_item_ident: rsshader::GPUFnDesc<'static> = rsshader::GPUFnDesc {
            ident: stringify!(#fn_ident),
            inputs: &[#(
                rsshader::GPUFnInputDesc {
                    ident: stringify!(#input_idents),
                    ty: &<#input_types as rsshader::GPUType>::TYPE_DESC,
                },
            )*],
            output: #output,
            stmts: {
                #(
                    #[allow(non_snake_case)]
                    let #input_expr_item_idents = rsshader::GPUExprDesc::Local(stringify!(#input_idents));
                )*

                #stmts
            },
        };

        #[allow(non_upper_case_globals)]
        #vis const #expr_desc_item_ident: rsshader::GPUUnsupportedType = rsshader::GPUUnsupportedType;
    }
    .into()
}

fn stmts_desc<'a>(stmts: impl Iterator<Item = &'a Stmt>) -> TokenStream {
    let stmt_vars = stmts
        .zip(0..)
        .map(|(stmt, i)| {
            let stmt_var_ident = Ident::new(&format!("stmt_{i}"), stmt.span());

            match stmt {
                Stmt::Expr(expr, _) => {
                    let expr_desc = expr_desc(expr);
    
                    quote_spanned! { stmt.span() => let #stmt_var_ident = rsshader::GPUStmtDesc::Expr(#expr_desc); }
                }
                Stmt::Local(local) => {
                    let ident = match &local.pat {
                        Pat::Ident(input) => &input.ident,
                        _ => return Error::new(local.pat.span(), "expected an ident").into_compile_error(),
                    };

                    let expr_desc_item_ident = get_expr_desc_item_ident(ident);
    
                    let value_desc = local.init.as_ref().map_or_else(
                        || {
                            Error::new(local.semi_token.span(), "expected an initializer")
                                .into_compile_error()
                        },
                        |init| expr_desc(&init.expr),
                    );
    
                    quote_spanned! {
                        stmt.span() =>
    
                        #[allow(non_snake_case)]
                        let #expr_desc_item_ident = rsshader::GPUExprDesc::Local(stringify!(#ident));

                        let #stmt_var_ident = rsshader::GPUStmtDesc::Let(rsshader::GPULetDesc {
                            ident: stringify!(#ident),
                            value: #value_desc,
                        });
                    }
                }
                _ => Error::new(stmt.span(), "unsupported stmt type").into_compile_error(),
            }
        })
        .collect::<Box<[TokenStream]>>();

    let stmt_idents = (0..stmt_vars.len())
        .map(|i| Ident::new(&format!("stmt_{i}"), stmt_vars[i].span()))
        .collect::<Box<[Ident]>>();

    quote_spanned! {
        stmt_idents.last().span() => {
            #(#stmt_vars)*

            &[#(#stmt_idents), *]
        }
    }
}

fn expr_desc(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Lit(expr) => match &expr.lit {
            Lit::Bool(lit) => {
                quote_spanned! { lit.span() => rsshader::GPUExprDesc::BoolLiteral(#lit) }
            }
            Lit::Int(lit) => {
                let lit: u128 = lit.base10_parse().unwrap();

                quote_spanned! { lit.span() => rsshader::GPUExprDesc::IntLiteral(#lit) }
            }
            _ => Error::new(expr.span(), "unsupported expr type").into_compile_error(),
        },
        Expr::Path(expr) => {
            let mut path = expr.path.clone();
            if let Some(last_segment) = path.segments.last_mut() {
                last_segment.ident = get_expr_desc_item_ident(&last_segment.ident);
            };

            path.to_token_stream()
        }
        _ => Error::new(expr.span(), "unsupported expr type").into_compile_error(),
    }
}
