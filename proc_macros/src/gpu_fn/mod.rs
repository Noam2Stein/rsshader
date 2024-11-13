use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse, spanned::Spanned, Error, FnArg, Ident, ItemFn, Pat, ReturnType};

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
    let associated_const_ident = get_associated_const_ident(&sig.ident);

    let input_idents = sig.inputs.iter().map(|input| match input {
        FnArg::Receiver(_) => None,
        FnArg::Typed(input) => match &*input.pat {
            Pat::Ident(input) => Some(&input.ident),
            _ => None,
        },
    });
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
            quote_spanned! { output.span() => Some(&<#output as rsshader::GPUType>::DESC) }
        }
    };

    let block_error = if block.stmts.len() > 0 {
        Some(Error::new(block.span(), "stmts are not supported yet").to_compile_error())
    } else {
        None
    };

    quote! {
        #vis #sig #block

        #[allow(non_upper_case_globals)]
        #vis const #associated_const_ident: rsshader::GPUFnDesc<'static> = rsshader::GPUFnDesc {
            ident: stringify!(#fn_ident),
            inputs: &[#(
                rsshader::GPUFnInputDesc {
                    ident: stringify!(#input_idents),
                    ty: &<#input_types as rsshader::GPUType>::DESC,
                },
            )*],
            output: #output,
        };
        #block_error
    }
    .into()
}

fn get_associated_const_ident(fn_ident: &Ident) -> Ident {
    Ident::new(&format!("GPUFn_{fn_ident}"), fn_ident.span())
}
