use quote::quote;
use syn::{
    Expr, Path, Token,
    parse::{ParseStream, Parser},
};

pub fn shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_fn = |input: ParseStream| {
        let ir = input.parse::<Expr>()?;
        let _ = input.parse::<Token![=>]>()?;
        let fmt_fn = input.parse::<Path>()?;

        Ok((ir, fmt_fn))
    };

    let (ir, fmt_fn) = match parse_fn.parse2(input.into()) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

    quote! {{
        const RSSHADER_STR: &str = {
            const RSSHADER_IR: rsshader::ir::Shader = #ir;

            const RSSHADER_LEN: usize = {
                let mut f = rsshader::lang::Formatter::without_output();
                rsshader::lang::fmt_wgsl(&mut f, &RSSHADER_IR);
                f.output_len()
            };

            const RSSHADER_BUF: [u8; RSSHADER_LEN] = {
                let mut buf = [0; RSSHADER_LEN];
                let mut f = rsshader::lang::Formatter::with_output(&mut buf);
                #fmt_fn(&mut f, &RSSHADER_IR);
                buf
            };

            unsafe { core::str::from_utf8_unchecked(&RSSHADER_BUF) }
        };

        RSSHADER_STR
    }}
    .into()
}
