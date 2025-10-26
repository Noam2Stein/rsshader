use quote::quote;
use syn::{
    Ident, Path, Token,
    parse::{ParseStream, Parser},
};

pub fn shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parse_fn = |input: ParseStream| {
        let mut entry_points = Vec::new();

        while input.peek(Ident) || input.peek(Token![::]) {
            entry_points.push(input.parse::<Path>()?);

            if let Some(_) = input.parse::<Option<Token![,]>>()? {
                continue;
            } else {
                break;
            }
        }

        let _ = input.parse::<Token![=>]>()?;
        let fmt_fn = input.parse::<Path>()?;

        Ok((entry_points, fmt_fn))
    };

    let (entry_points, fmt_fn) = match parse_fn.parse2(input.into()) {
        Ok(result) => result,
        Err(e) => return e.to_compile_error().into(),
    };

    quote! {{
        const RSSHADER_STR: &str = {
            const RSSHADER_IR: rsshader::ir::Shader = rsshader::ir::Shader {
                entry_points: &[#(&<#entry_points as rsshader::reflection::ShaderFn>::IR),*],
            };

            const RSSHADER_LINKED_IR_BUF: rsshader::ir::LinkedShaderBuffer<128, 128> = rsshader::ir::LinkedShaderBuffer::link(&RSSHADER_IR);

            const RSSHADER_LINKED_IR: rsshader::ir::LinkedShader = RSSHADER_LINKED_IR_BUF.as_ref();

            const RSSHADER_LEN: usize = {
                let mut f = rsshader::lang::Formatter::without_output();
                rsshader::lang::fmt_wgsl(&mut f, &RSSHADER_LINKED_IR);
                f.output_len()
            };

            const RSSHADER_BUF: [u8; RSSHADER_LEN] = {
                let mut buf = [0; RSSHADER_LEN];
                let mut f = rsshader::lang::Formatter::with_output(&mut buf);
                #fmt_fn(&mut f, &RSSHADER_LINKED_IR);
                buf
            };

            unsafe { core::str::from_utf8_unchecked(&RSSHADER_BUF) }
        };

        RSSHADER_STR
    }}
    .into()
}
